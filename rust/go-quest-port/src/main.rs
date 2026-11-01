//! Standalone offline CLI for the safe, portable parts of the Go project.
use chrono::Utc;
use go_quest_port::{manager::QuestManager, parse_quests};
use serde::Serialize;
use std::{env, error::Error, fs, process};

#[derive(Serialize)]
struct Report {
    analyzed: usize,
    go_filter_valid: usize,
    go_filter_redeemable: usize,
    enrollment_blocked_until: Option<String>,
    assessments: Vec<go_quest_port::manager::QuestAssessment>,
}

fn usage() {
    eprintln!(
        "Go Quest -> Rust (offline port)\n\
         Usage:\n\
         go-quest-port demo [--json] [--workers N]\n\
         go-quest-port inspect <local-quests.json> [--json] [--workers N]\n\
         \n\
         No account token, network connection, game spoofing, or reward claims."
    );
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "--help".to_owned());
    if command == "--help" || command == "-h" {
        usage();
        return Ok(());
    }

    let input = match command.as_str() {
        "demo" => include_str!("../examples/sample-quests.json").to_owned(),
        "inspect" => {
            let path = args.next().ok_or("Missing local JSON file path")?;
            fs::read_to_string(&path)?
        }
        _ => {
            usage();
            return Err(format!("Unsupported command: {command}").into());
        }
    };

    let mut workers: usize = 4;
    let mut output_json = false;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--json" => output_json = true,
            "--workers" => {
                let amount = args.next().ok_or("Missing value for --workers")?;
                workers = amount.parse()?;
                if !(1..=32).contains(&workers) {
                    return Err("--workers must be an integer between 1 and 32".into());
                }
            }
            _ => return Err(format!("Unknown argument: {flag}").into()),
        }
    }

    let (quests, enrollment_blocked_until) = parse_quests(&input)?;
    let manager = QuestManager::new(quests, enrollment_blocked_until.clone());
    let now = Utc::now();
    let report = Report {
        analyzed: manager.quests().len(),
        go_filter_valid: manager.filter_valid(now).len(),
        go_filter_redeemable: manager.filter_redeemable().len(),
        enrollment_blocked_until,
        assessments: manager.assess_all(now, workers),
    };
    if output_json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!(
        "Analyzed {} Quest record(s); Go-style eligible: {}; claimable: {}.",
        report.analyzed, report.go_filter_valid, report.go_filter_redeemable
    );
    if let Some(blocked) = report.enrollment_blocked_until {
        println!("Enrollment blocked until (source data): {blocked}");
    }
    for quest in report.assessments {
        println!("\n{} ({})", quest.name, quest.quest_id);
        println!("  status: {:?}; operator: {}", quest.verdict, quest.join_operator);
        for task in quest.tasks {
            let value = task.reported_progress.map_or_else(|| "?".to_owned(), |v| v.to_string());
            let goal = task.goal.map_or_else(|| "?".to_owned(), |v| v.to_string());
            println!(
                "  {:26} {:>6}/{:<6} confirmed={} family={}",
                task.task_type, value, goal, task.server_task_completed, task.family
            );
        }
        for warning in quest.warnings {
            println!("  note: {warning}");
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

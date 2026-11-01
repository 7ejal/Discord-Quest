//! Standalone offline CLI for the safe, portable parts of the Go project.
use chrono::{DateTime, Utc};
use go_quest_port::{
    manager::QuestManager,
    output::{escape_terminal, LimitedWriter, MAX_OUTPUT_BYTES},
    parse_document,
};
use serde::Serialize;
use std::{
    env,
    error::Error,
    fs,
    io::{self, Read, Write},
    process,
};

#[derive(Serialize)]
struct ExcludedQuestSummary {
    id: String,
    preview: bool,
    has_config: bool,
    has_status: bool,
}

#[derive(Serialize)]
struct Report {
    schema_version: u32,
    input_trust: &'static str,
    evaluated_at: String,
    analyzed: usize,
    go_filter_valid: usize,
    go_filter_redeemable: usize,
    enrollment_blocked_until: Option<String>,
    excluded_count: usize,
    excluded_quests: Vec<ExcludedQuestSummary>,
    assessments: Vec<go_quest_port::manager::QuestAssessment>,
}

fn usage() {
    eprintln!(
        "Go Quest -> Rust (offline port)\n\
         Usage:\n\
         go-quest-port demo [--json] [--workers N] [--at RFC3339]\n\
         go-quest-port inspect <local-quests.json|-> [--json] [--workers N] [--at RFC3339]\n\
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
    if command == "--version" || command == "-V" {
        println!("go-quest-port {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let input = match command.as_str() {
        "demo" => include_str!("../examples/sample-quests.json").to_owned(),
        "inspect" => {
            let path = args.next().ok_or("Missing local JSON file path")?;
            let mut input = String::new();
            if path == "-" {
                io::stdin()
                    .lock()
                    .take((go_quest_port::MAX_JSON_BYTES + 1) as u64)
                    .read_to_string(&mut input)?;
            } else {
                fs::File::open(&path)?
                    .take((go_quest_port::MAX_JSON_BYTES + 1) as u64)
                    .read_to_string(&mut input)?;
            }
            if input.len() > go_quest_port::MAX_JSON_BYTES {
                return Err("Local JSON exceeds the 8 MiB input limit".into());
            }
            input
        }
        _ => {
            usage();
            return Err("Unsupported command (run --help for usage)".into());
        }
    };

    let mut workers: usize = 4;
    let mut output_json = false;
    let mut evaluated_at: Option<DateTime<Utc>> = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--json" => output_json = true,
            "--at" => {
                let text = args.next().ok_or("Missing RFC3339 value for --at")?;
                let parsed = DateTime::parse_from_rfc3339(&text)
                    .map_err(|_| "Invalid --at timestamp; expected RFC3339 with timezone")?;
                evaluated_at = Some(parsed.with_timezone(&Utc));
            }
            "--workers" => {
                let amount = args.next().ok_or("Missing value for --workers")?;
                workers = amount.parse()?;
                if !(1..=32).contains(&workers) {
                    return Err("--workers must be an integer between 1 and 32".into());
                }
            }
            _ => return Err("Unknown argument (run --help for usage)".into()),
        }
    }

    let parsed = parse_document(&input)?;
    let excluded: Vec<_> = parsed
        .excluded_quests
        .into_iter()
        .map(|partial| ExcludedQuestSummary {
            id: partial.id,
            preview: partial.preview,
            has_config: partial.config.is_some(),
            has_status: partial.user_status.is_some(),
        })
        .collect();
    let manager = QuestManager::new(parsed.quests, parsed.enrollment_blocked_until.clone());
    let now = evaluated_at.unwrap_or_else(Utc::now);
    let report = Report {
        schema_version: 1,
        input_trust: "unverified_local_json",
        evaluated_at: now.to_rfc3339(),
        analyzed: manager.quests().len(),
        go_filter_valid: manager.filter_valid(now).len(),
        go_filter_redeemable: manager.filter_redeemable().len(),
        enrollment_blocked_until: parsed.enrollment_blocked_until,
        excluded_count: excluded.len(),
        excluded_quests: excluded,
        assessments: manager.assess_all(now, workers),
    };
    if output_json {
        // Serialize to a size-limited buffer before touching stdout so an
        // oversized report never corrupts a JSON pipeline with partial data.
        let mut output = LimitedWriter::new(MAX_OUTPUT_BYTES);
        serde_json::to_writer_pretty(&mut output, &report)?;
        let bytes = output.into_bytes();
        let mut stdout = io::stdout().lock();
        stdout.write_all(&bytes)?;
        stdout.write_all(b"\n")?;
        return Ok(());
    }

    println!("Input data: unverified local JSON (not authenticated server status).");
    println!("Evaluated at (UTC): {}", report.evaluated_at);
    println!(
        "Analyzed {} Quest record(s); Go-style eligible: {}; claimable: {}.",
        report.analyzed, report.go_filter_valid, report.go_filter_redeemable
    );
    if let Some(blocked) = report.enrollment_blocked_until {
        println!(
            "Enrollment blocked until (source data): {}",
            escape_terminal(&blocked)
        );
    }
    if report.excluded_count > 0 {
        println!(
            "Excluded partial Quest records (not evaluated): {}",
            report.excluded_count
        );
        for partial in &report.excluded_quests {
            println!(
                "  excluded: {} (preview={}, config_present={})",
                escape_terminal(&partial.id),
                partial.preview,
                partial.has_config
            );
        }
    }
    for quest in report.assessments {
        println!(
            "\n{} ({})",
            escape_terminal(&quest.name),
            escape_terminal(&quest.quest_id)
        );
        println!(
            "  status: {:?}; operator: {}",
            quest.verdict, quest.join_operator
        );
        for task in quest.tasks {
            let value = task
                .reported_progress
                .map_or_else(|| "?".to_owned(), |v| v.to_string());
            let goal = task.goal.map_or_else(|| "?".to_owned(), |v| v.to_string());
            println!(
                "  {:26} {:>6}/{:<6} confirmed={} family={}",
                escape_terminal(&task.task_type),
                value,
                goal,
                task.server_task_completed,
                task.family
            );
        }
        for warning in quest.warnings {
            println!("  note: {}", escape_terminal(&warning));
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", escape_terminal(&error.to_string()));
        process::exit(1);
    }
}

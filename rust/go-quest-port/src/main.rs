//! Linux-only offline Quest analysis. No account access or network operations.
use chrono::{DateTime, Utc};
use go_quest_port::{
    manager::{try_analyze_parallel, QuestManager},
    output::{escape_terminal, LimitedWriter, MAX_OUTPUT_BYTES},
    parse_document,
};
use serde::Serialize;
use std::{
    env, fmt, fs,
    io::{self, Read, Write},
    process,
    sync::atomic::AtomicBool,
};

#[derive(Debug)]
enum CliError {
    Usage(&'static str),
    Input(String),
    Io(io::Error),
    Analysis,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => f.write_str(message),
            Self::Input(message) => f.write_str(message),
            Self::Io(error) => write!(f, "I/O failure ({:?})", error.kind()),
            Self::Analysis => f.write_str("Analysis failed; no partial report emitted"),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self::Input(message)
    }
}

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
    println!(
        "Go Quest -> Rust (offline port)\n\
         Usage:\n\
         go-quest-port demo [--json] [--workers N] [--at RFC3339] [--verbose]\n\
         go-quest-port inspect <local-quests.json|-> [same options]\n\
         --max-input-bytes N   Lower the default 8388608-byte input cap\n\
         --max-output-bytes N  Lower the default 16777216-byte report cap\n\
         --version | --help\n\
         No account token, network connection, game spoofing, or reward claims."
    );
}

fn run() -> Result<(), CliError> {
    let mut args = env::args_os().skip(1);
    let command = args.next().unwrap_or_else(|| "--help".into());
    if command == "--help" || command == "-h" || command == "--version" || command == "-V" {
        if args.next().is_some() {
            return Err(CliError::Usage("Unexpected argument"));
        }
        if command == "--help" || command == "-h" {
            usage();
        } else {
            println!("go-quest-port {}", env!("CARGO_PKG_VERSION"));
        }
        return Ok(());
    }
    let path = if command == "inspect" {
        Some(
            args.next()
                .ok_or(CliError::Usage("Missing local JSON file path"))?,
        )
    } else if command == "demo" {
        None
    } else {
        return Err(CliError::Usage(
            "Unsupported command (run --help for usage)",
        ));
    };

    let mut workers: usize = 4;
    let mut output_json = false;
    let mut verbose = false;
    let mut max_input = go_quest_port::MAX_JSON_BYTES;
    let mut max_output = MAX_OUTPUT_BYTES;
    let mut evaluated_at: Option<DateTime<Utc>> = None;
    while let Some(flag) = args.next() {
        match flag.to_str() {
            Some("--json") => output_json = true,
            Some("--verbose") => verbose = true,
            Some("--at") => {
                let text = args
                    .next()
                    .ok_or(CliError::Usage("Missing RFC3339 value for --at"))?;
                let parsed =
                    DateTime::parse_from_rfc3339(text.to_str().unwrap_or("")).map_err(|_| {
                        CliError::Usage("Invalid --at timestamp; expected RFC3339 with timezone")
                    })?;
                evaluated_at = Some(parsed.with_timezone(&Utc));
            }
            Some("--workers" | "--max-input-bytes" | "--max-output-bytes") => {
                let amount = args
                    .next()
                    .ok_or(CliError::Usage("Missing numeric flag value"))?;
                let amount: usize = amount
                    .to_str()
                    .and_then(|text| text.parse().ok())
                    .ok_or(CliError::Usage("Expected an unsigned integer flag value"))?;
                let limit = match flag.to_str() {
                    Some("--workers") => 32,
                    Some("--max-input-bytes") => go_quest_port::MAX_JSON_BYTES,
                    _ => MAX_OUTPUT_BYTES,
                };
                if !(1..=limit).contains(&amount) {
                    return Err(CliError::Usage(
                        "Numeric flag value outside supported range",
                    ));
                }
                match flag.to_str() {
                    Some("--workers") => workers = amount,
                    Some("--max-input-bytes") => max_input = amount,
                    _ => max_output = amount,
                }
            }
            _ => return Err(CliError::Usage("Unknown argument (run --help for usage)")),
        }
    }

    // Validate flags before opening a file or waiting for stdin.
    let input = if let Some(path) = path {
        let mut input = String::new();
        if path == "-" {
            io::stdin()
                .lock()
                .take((max_input + 1) as u64)
                .read_to_string(&mut input)?;
        } else {
            fs::File::open(path)?
                .take((max_input + 1) as u64)
                .read_to_string(&mut input)?;
        }
        input
    } else {
        include_str!("../examples/sample-quests.json").to_owned()
    };
    if input.len() > max_input {
        return Err(CliError::Input("Local JSON exceeds input limit".to_owned()));
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
        enrollment_blocked_until: parsed
            .enrollment_blocked_until
            .filter(|raw| DateTime::parse_from_rfc3339(raw).is_ok()),
        excluded_count: excluded.len(),
        excluded_quests: excluded,
        assessments: try_analyze_parallel(manager.quests(), now, workers, &AtomicBool::new(false))
            .map_err(|_| CliError::Analysis)?,
    };
    if verbose {
        eprintln!("Offline analysis: {} records, {} excluded, workers capped at {}. Source: unverified_local_json.",
            report.analyzed, report.excluded_count, workers);
    }

    // Buffer both formats: errors leave stdout empty, never a truncated report.
    let mut output = LimitedWriter::new(max_output.saturating_sub(1));
    if output_json {
        serde_json::to_writer_pretty(&mut output, &report).map_err(|_| {
            CliError::Input("Cannot serialize report within output limit".to_owned())
        })?;
    } else {
        render_text(&mut output, &report)?;
    }
    let mut bytes = output.into_bytes();
    if output_json {
        bytes.push(b'\n');
    }
    io::stdout().lock().write_all(&bytes)?;
    Ok(())
}

fn render_text(output: &mut impl Write, report: &Report) -> io::Result<()> {
    writeln!(
        output,
        "Input data: unverified local JSON (not authenticated server status)."
    )?;
    writeln!(output, "Evaluated at (UTC): {}", report.evaluated_at)?;
    writeln!(
        output,
        "Analyzed {} Quest record(s); Go-style eligible: {}; reported completed/unclaimed: {}.",
        report.analyzed, report.go_filter_valid, report.go_filter_redeemable
    )?;
    if let Some(blocked) = &report.enrollment_blocked_until {
        writeln!(
            output,
            "Enrollment blocked until (source data): {}",
            escape_terminal(blocked)
        )?;
    }
    if report.excluded_count > 0 {
        writeln!(
            output,
            "Excluded partial Quest records (not evaluated): {}",
            report.excluded_count
        )?;
        for partial in &report.excluded_quests {
            writeln!(
                output,
                "  excluded: {} (preview={}, config_present={})",
                escape_terminal(&partial.id),
                partial.preview,
                partial.has_config
            )?;
        }
    }
    for quest in &report.assessments {
        writeln!(
            output,
            "\n{} ({})",
            escape_terminal(&quest.name),
            escape_terminal(&quest.quest_id)
        )?;
        writeln!(
            output,
            "  reported status: {:?}; operator: {}; external redemption: unknown",
            quest.verdict,
            escape_terminal(&quest.join_operator)
        )?;
        for task in &quest.tasks {
            let value = task
                .reported_progress
                .map_or_else(|| "?".to_owned(), |v| v.to_string());
            let goal = task.goal.map_or_else(|| "?".to_owned(), |v| v.to_string());
            writeln!(
                output,
                "  {:26} {:>6}/{:<6} reported_complete={} family={}",
                escape_terminal(&task.task_type),
                value,
                goal,
                task.server_task_completed,
                task.family
            )?;
        }
        for warning in &quest.warnings {
            writeln!(output, "  note: {}", escape_terminal(warning))?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", escape_terminal(&error.to_string()));
        process::exit(match error {
            CliError::Usage(_) => 64,
            CliError::Input(_) => 65,
            CliError::Io(_) => 74,
            CliError::Analysis => 70,
        });
    }
}

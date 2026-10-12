use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{env, fs, process};

#[derive(Default)]
struct Usage {
    calls: u64,
    errors: u64,
    millis: i64,
    chars: u64,
}

impl Usage {
    fn record(&mut self, failed: bool, millis: i64, chars: u64) {
        self.errors += u64::from(failed);
        self.millis += millis;
        self.chars += chars;
    }

    fn cell(&self) -> String {
        format!(
            "{} calls, {} errors, {}, {} chars",
            self.calls,
            self.errors,
            duration(self.millis),
            count(self.chars)
        )
    }
}

struct Call {
    name: String,
    command: Option<String>,
    at: Option<i64>,
}

struct TestRun {
    at: i64,
    passed: u64,
    failed: u64,
    filtered: u64,
}

#[derive(Default)]
struct Run {
    name: String,
    model: String,
    version: String,
    cwd: String,
    result: Value,
    first: Option<i64>,
    invalid: u64,
    messages: HashSet<String>,
    subagent_requests: u64,
    contexts: Vec<u64>,
    text_blocks: u64,
    text_chars: u64,
    thinking_blocks: u64,
    tool_input_chars: u64,
    calls: HashMap<String, Call>,
    tools: BTreeMap<String, Usage>,
    commands: BTreeMap<String, Usage>,
    files: BTreeMap<String, (u64, u64)>,
    tasks: BTreeMap<String, u64>,
    tests: Vec<TestRun>,
}

impl Run {
    fn read(path: &Path) -> Run {
        let (file, name) = if path.is_dir() {
            (path.join("run.jsonl"), path)
        } else {
            (path.to_path_buf(), path.parent().unwrap_or(path))
        };
        let text = fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", file.display()));
        let mut run = Run {
            name: name.file_name().map_or_else(
                || file.display().to_string(),
                |n| n.to_string_lossy().into(),
            ),
            ..Run::default()
        };
        for line in text.lines().filter(|line| !line.trim().is_empty()) {
            match serde_json::from_str::<Value>(line) {
                Ok(event) => run.event(&event),
                Err(_) => run.invalid += 1,
            }
        }
        run
    }

    fn event(&mut self, event: &Value) {
        let at = event["timestamp"].as_str().and_then(millis);
        if let Some(at) = at {
            self.first.get_or_insert(at);
        }
        match (str(&event["type"]), str(&event["subtype"])) {
            ("system", "init") => {
                self.model = str(&event["model"]).to_string();
                self.version = str(&event["claude_code_version"]).to_string();
                self.cwd = str(&event["cwd"]).to_string();
            }
            ("system", "task_started") => {
                *self
                    .tasks
                    .entry(str(&event["task_type"]).to_string())
                    .or_default() += 1;
            }
            ("assistant", _) => self.assistant(event, at),
            ("user", _) => self.user(event, at),
            ("result", _) => self.result = event.clone(),
            _ => {}
        }
    }

    fn assistant(&mut self, event: &Value, at: Option<i64>) {
        let message = &event["message"];
        if self.messages.insert(str(&message["id"]).to_string()) {
            if event["parent_tool_use_id"].is_null() {
                let usage = &message["usage"];
                self.contexts.push(
                    [
                        "input_tokens",
                        "cache_creation_input_tokens",
                        "cache_read_input_tokens",
                    ]
                    .iter()
                    .map(|key| num(&usage[*key]))
                    .sum(),
                );
            } else {
                self.subagent_requests += 1;
            }
        }
        for block in message["content"].as_array().into_iter().flatten() {
            match str(&block["type"]) {
                "text" => {
                    self.text_blocks += 1;
                    self.text_chars += str(&block["text"]).chars().count() as u64;
                }
                "thinking" | "redacted_thinking" => self.thinking_blocks += 1,
                "tool_use" => self.tool_use(block, at),
                _ => {}
            }
        }
    }

    fn tool_use(&mut self, block: &Value, at: Option<i64>) {
        let name = str(&block["name"]).to_string();
        let input = &block["input"];
        self.tool_input_chars += input.to_string().chars().count() as u64;
        self.tools.entry(name.clone()).or_default().calls += 1;
        let command = (name == "Bash").then(|| category(str(&input["command"])));
        if let Some(command) = &command {
            self.commands.entry(command.clone()).or_default().calls += 1;
        }
        match name.as_str() {
            "Write" => self.file(input).0 += 1,
            "Edit" | "MultiEdit" | "NotebookEdit" => self.file(input).1 += 1,
            _ => {}
        }
        let id = str(&block["id"]).to_string();
        self.calls.insert(id, Call { name, command, at });
    }

    fn file(&mut self, input: &Value) -> &mut (u64, u64) {
        let path = str(&input["file_path"]);
        let path = path
            .strip_prefix(self.cwd.as_str())
            .map_or(path, |relative| relative.trim_start_matches('/'))
            .to_string();
        self.files.entry(path).or_default()
    }

    fn user(&mut self, event: &Value, at: Option<i64>) {
        for block in event["message"]["content"].as_array().into_iter().flatten() {
            if str(&block["type"]) != "tool_result" {
                continue;
            }
            let Some(call) = self.calls.remove(str(&block["tool_use_id"])) else {
                continue;
            };
            let output = result_text(&block["content"]);
            let failed = block["is_error"].as_bool().unwrap_or(false);
            let millis = at.zip(call.at).map_or(0, |(end, start)| end - start);
            let chars = output.chars().count() as u64;
            self.tools
                .entry(call.name)
                .or_default()
                .record(failed, millis, chars);
            let Some(command) = call.command else {
                continue;
            };
            self.commands
                .entry(command)
                .or_default()
                .record(failed, millis, chars);
            if let (Some(test), Some(at), Some(first)) = (test_run(&output), at, self.first) {
                self.tests.push(TestRun {
                    at: at - first,
                    ..test
                });
            }
        }
    }

    fn report(&self) -> Vec<Section> {
        let result = &self.result;
        let usage = &result["usage"];
        let input = num(&usage["input_tokens"]);
        let written = num(&usage["cache_creation_input_tokens"]);
        let read = num(&usage["cache_read_input_tokens"]);
        let output = num(&usage["output_tokens"]);
        let thinking = num(&usage["output_tokens_details"]["thinking_tokens"]);
        let wall = num(&result["duration_ms"]);
        let api = num(&result["duration_api_ms"]);
        let window = num(&result["modelUsage"][&self.model]["contextWindow"]);
        let peak = self.contexts.iter().copied().max().unwrap_or(0);
        let tool_calls: u64 = self.tools.values().map(|u| u.calls).sum();
        let tool_errors: u64 = self.tools.values().map(|u| u.errors).sum();

        let mut run = vec![
            row("model", &self.model),
            row("claude code", &self.version),
            row(
                "outcome",
                format!(
                    "{} ({})",
                    str(&result["subtype"]),
                    str(&result["terminal_reason"])
                ),
            ),
            row("wall time", duration(wall as i64)),
            row(
                "api time",
                format!("{} ({})", duration(api as i64), percent(api, wall)),
            ),
            row(
                "cost",
                format!("${:.2}", result["total_cost_usd"].as_f64().unwrap_or(0.0)),
            ),
        ];
        if self.invalid > 0 {
            run.push(row("invalid lines", count(self.invalid)));
        }

        let mut iterations = vec![
            row("turns", count(num(&result["num_turns"]))),
            row("api requests", count(self.contexts.len() as u64)),
        ];
        if self.subagent_requests > 0 {
            iterations.push(row("subagent requests", count(self.subagent_requests)));
        }
        iterations.extend([
            row("tool calls", count(tool_calls)),
            row("tool errors", count(tool_errors)),
            row("text blocks", count(self.text_blocks)),
            row("thinking blocks", count(self.thinking_blocks)),
            row("test runs", count(self.tests.len() as u64)),
        ]);

        let tokens = vec![
            row("input", count(input)),
            row("cache write", count(written)),
            row("cache read", count(read)),
            row("output", count(output)),
            row("  thinking", count(thinking)),
            row(
                "  text and tool input",
                count(output.saturating_sub(thinking)),
            ),
            row("cache hit rate", percent(read, input + written + read)),
        ];

        let mean = self.contexts.iter().sum::<u64>() / self.contexts.len().max(1) as u64;
        let context = vec![
            row(
                "peak",
                format!("{} ({})", count(peak), percent(peak, window)),
            ),
            row("final", count(self.contexts.last().copied().unwrap_or(0))),
            row("mean", count(mean)),
            row("window", count(window)),
        ];

        let output_size = vec![
            row("text chars", count(self.text_chars)),
            row("tool input chars", count(self.tool_input_chars)),
        ];

        let models = result["modelUsage"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(model, usage)| {
                row(
                    model,
                    format!(
                        "${:.2}, {} out",
                        usage["costUSD"].as_f64().unwrap_or(0.0),
                        count(num(&usage["outputTokens"]))
                    ),
                )
            })
            .collect();

        let mut files: Vec<_> = self.files.iter().collect();
        files.sort_by_key(|(_, (writes, edits))| std::cmp::Reverse(writes + edits));
        let files = files
            .into_iter()
            .take(10)
            .map(|(path, (writes, edits))| row(path, format!("{writes} writes, {edits} edits")))
            .collect();

        let mut tasks: Vec<_> = self
            .tasks
            .iter()
            .map(|(kind, n)| row(kind, count(*n)))
            .collect();
        tasks.push(row(
            "subagents spawned",
            count(num(&result["subagent_stats"]["spawned"])),
        ));

        vec![
            Section::new("Run", run),
            Section::new("Iterations", iterations),
            Section::new("Tokens", tokens),
            Section::new("Context", context),
            Section::new("Output size", output_size),
            Section::new("Models", models),
            Section::new("Tools", usage_rows(&self.tools)),
            Section::new("Commands", usage_rows(&self.commands)),
            Section::new("Files", files),
            Section::new("Tasks", tasks),
        ]
    }
}

struct Section {
    title: &'static str,
    rows: Vec<(String, String)>,
}

impl Section {
    fn new(title: &'static str, rows: Vec<(String, String)>) -> Section {
        Section { title, rows }
    }

    fn value(&self, label: &str) -> &str {
        self.rows
            .iter()
            .find(|(l, _)| l == label)
            .map_or("", |(_, value)| value)
    }
}

fn row(label: &str, value: impl Into<String>) -> (String, String) {
    (label.to_string(), value.into())
}

fn usage_rows(usages: &BTreeMap<String, Usage>) -> Vec<(String, String)> {
    let mut usages: Vec<_> = usages.iter().collect();
    usages.sort_by_key(|(_, usage)| std::cmp::Reverse(usage.calls));
    usages
        .into_iter()
        .map(|(name, usage)| row(name, usage.cell()))
        .collect()
}

fn print(runs: &[Run]) {
    let reports: Vec<Vec<Section>> = runs.iter().map(Run::report).collect();
    let mut table: Vec<(String, Vec<&str>)> = vec![(
        String::new(),
        runs.iter().map(|run| run.name.as_str()).collect(),
    )];
    for (index, section) in reports[0].iter().enumerate() {
        table.push((section.title.to_string(), Vec::new()));
        let mut labels: Vec<&str> = Vec::new();
        for report in &reports {
            for (label, _) in &report[index].rows {
                if !labels.contains(&label.as_str()) {
                    labels.push(label);
                }
            }
        }
        for label in labels {
            let values = reports.iter().map(|r| r[index].value(label)).collect();
            table.push((format!("  {label}"), values));
        }
    }

    let label_width = table
        .iter()
        .map(|(label, _)| label.len())
        .max()
        .unwrap_or(0);
    let widths: Vec<usize> = (0..runs.len())
        .map(|column| {
            table
                .iter()
                .filter_map(|(_, values)| values.get(column))
                .map(|value| value.chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    for (label, values) in &table {
        let mut line = format!("{label:<label_width$}");
        for (value, width) in values.iter().zip(&widths) {
            line.push_str(&format!("   {value:<width$}"));
        }
        println!("{}", line.trim_end());
    }

    for run in runs.iter().filter(|run| !run.tests.is_empty()) {
        println!("\nTest runs: {}", run.name);
        println!(
            "  {:>10}  {:>7}  {:>7}  {:>9}",
            "elapsed", "passed", "failed", "filtered"
        );
        for test in &run.tests {
            println!(
                "  {:>10}  {:>7}  {:>7}  {:>9}",
                duration(test.at),
                test.passed,
                test.failed,
                test.filtered
            );
        }
    }
}

fn str(value: &Value) -> &str {
    value.as_str().unwrap_or("")
}

fn num(value: &Value) -> u64 {
    value.as_u64().unwrap_or(0)
}

fn result_text(content: &Value) -> String {
    match content {
        Value::String(text) => text.clone(),
        Value::Array(blocks) => blocks
            .iter()
            .filter_map(|block| block["text"].as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

fn test_run(output: &str) -> Option<TestRun> {
    let lines: Vec<&str> = output
        .lines()
        .filter_map(|line| line.split_once("test result: ").map(|(_, rest)| rest))
        .collect();
    if lines.is_empty() {
        return None;
    }
    let total = |label: &str| -> u64 {
        lines
            .iter()
            .flat_map(|line| line.split("; "))
            .filter_map(|part| part.trim().strip_suffix(label)?.split_whitespace().last())
            .filter_map(|n| n.parse::<u64>().ok())
            .sum()
    };
    Some(TestRun {
        at: 0,
        passed: total(" passed"),
        failed: total(" failed"),
        filtered: total(" filtered out"),
    })
}

fn category(command: &str) -> String {
    command
        .split(['\n', ';', '|', '&'])
        .find_map(program)
        .unwrap_or_else(|| "other".to_string())
}

fn program(segment: &str) -> Option<String> {
    let mut words = segment
        .split_whitespace()
        .map(|word| word.trim_start_matches(['(', '{']))
        .filter(|word| !word.is_empty() && !is_assignment(word));
    let mut word = words.next()?;
    while matches!(word, "timeout" | "time" | "env" | "sudo" | "nohup" | "exec")
        || word.starts_with('-')
        || word.starts_with(|c: char| c.is_ascii_digit())
    {
        word = words.next()?;
    }
    if word.starts_with('#') || matches!(word, "cd" | "pushd" | "export" | "set" | "source" | ".") {
        return None;
    }
    let word = word.rsplit('/').next().unwrap_or(word);
    if word != "cargo" {
        return Some(word.to_string());
    }
    Some(match words.find(|w| !w.starts_with(['-', '+'])) {
        Some(subcommand) => format!("cargo {subcommand}"),
        None => "cargo".to_string(),
    })
}

fn is_assignment(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    })
}

fn millis(timestamp: &str) -> Option<i64> {
    let (date, time) = timestamp.trim_end_matches('Z').split_once('T')?;
    let mut date = date.split('-').map(|part| part.parse::<i64>().ok());
    let (year, month, day) = (date.next()??, date.next()??, date.next()??);
    let mut time = time.split(':');
    let hour: i64 = time.next()?.parse().ok()?;
    let minute: i64 = time.next()?.parse().ok()?;
    let second: f64 = time.next()?.parse().ok()?;
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    Some(((days * 24 + hour) * 60 + minute) * 60_000 + (second * 1000.0).round() as i64)
}

fn duration(millis: i64) -> String {
    let seconds = millis / 1000;
    match seconds {
        3600.. => format!(
            "{}h {:02}m {:02}s",
            seconds / 3600,
            seconds % 3600 / 60,
            seconds % 60
        ),
        60.. => format!("{}m {:02}s", seconds / 60, seconds % 60),
        _ => format!("{seconds}s"),
    }
}

fn count(n: u64) -> String {
    let digits = n.to_string();
    let mut grouped = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

fn percent(part: u64, whole: u64) -> String {
    if whole == 0 {
        return "-".to_string();
    }
    format!("{:.1}%", part as f64 * 100.0 / whole as f64)
}

fn main() {
    let paths: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("usage: cargo run -p runstats -- <run dir or run.jsonl>...");
        process::exit(2);
    }
    let runs: Vec<Run> = paths.iter().map(|path| Run::read(path)).collect();
    print(&runs);
}

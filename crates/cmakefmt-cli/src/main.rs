use std::collections::HashMap;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use clap::{ArgAction, Parser};
use cmakefmt::{
    CaseStyle, ConfigDiagnostic, ConfigLoadResult, Configuration, IndentStyle, NewLineKind,
    SortArguments, SpaceBeforeParen, format_text, load_from_toml_path,
};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

mod trace_summary;
#[derive(Debug, Parser)]
#[command(name = "cmakefmt", about = "Format CMake files", version)]
struct Cli {
    /// File paths or glob patterns to format.
    /// If omitted, reads from stdin.
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    /// Check if files are formatted (exit 1 if not). No files are modified.
    #[arg(long)]
    check: bool,

    /// Print a unified diff of formatting changes. No files are modified.
    #[arg(long)]
    diff: bool,

    /// Write formatted output back to files in-place.
    #[arg(short, long, alias = "inplace")]
    write: bool,

    /// Read source from stdin.
    #[arg(long)]
    stdin: bool,

    /// Explicit path to a .cmakefmt.toml configuration file.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Pretend stdin input comes from this file path.
    #[arg(long, value_name = "PATH")]
    assume_filename: Option<PathBuf>,

    /// Always emit ANSI color in diagnostics and diff output.
    #[arg(long, conflicts_with = "no_color")]
    color: bool,

    /// Suppress ANSI color in diagnostics and diff output.
    #[arg(long, conflicts_with = "color")]
    no_color: bool,

    /// Increase diagnostic output.
    #[arg(long)]
    verbose: bool,

    /// Suppress non-error output.
    #[arg(long)]
    quiet: bool,

    /// Print resolved configuration as TOML.
    #[arg(long)]
    print_config: bool,

    /// Maximum line width.
    #[arg(long)]
    line_width: Option<u32>,

    /// Number of spaces per indentation level.
    #[arg(long)]
    indent_width: Option<u8>,

    /// Use tabs instead of spaces for indentation.
    #[arg(long, action = ArgAction::SetTrue)]
    use_tabs: bool,

    /// Newline style.
    #[arg(long)]
    new_line_kind: Option<NewLineKind>,

    /// Case style for commands.
    #[arg(long)]
    command_case: Option<CaseStyle>,

    /// Case style for keywords.
    #[arg(long)]
    keyword_case: Option<CaseStyle>,

    /// Place closing paren on a new line in multi-line commands.
    #[arg(long, action = ArgAction::Set)]
    closing_paren_newline: Option<bool>,

    /// Sort argument lists alphabetically.
    #[arg(long, action = ArgAction::SetTrue)]
    sort_lists: bool,

    /// Maximum consecutive blank lines to preserve.
    #[arg(long)]
    max_blank_lines: Option<u8>,

    /// Insert space before '(' for these commands (comma-separated, e.g. if,while,foreach).
    #[arg(long, value_delimiter = ',')]
    space_before_paren: Vec<String>,

    /// Write Chrome trace JSON output to this path.
    #[arg(long, value_name = "PATH")]
    trace_output: Option<PathBuf>,

    /// Write normalized trace summary JSON to this path.
    #[arg(long, value_name = "PATH")]
    trace_summary_output: Option<PathBuf>,

    /// tracing-subscriber EnvFilter directive string for trace capture.
    #[arg(long, value_name = "DIRECTIVE")]
    trace_filter: Option<String>,
}

#[derive(Debug, Clone)]
struct CliOverrides {
    line_width: Option<u32>,
    indent_width: Option<u8>,
    use_tabs: bool,
    new_line_kind: Option<NewLineKind>,
    command_case: Option<CaseStyle>,
    keyword_case: Option<CaseStyle>,
    closing_paren_newline: Option<bool>,
    sort_lists: bool,
    max_blank_lines: Option<u8>,
    space_before_paren: Option<Vec<String>>,
}

impl Cli {
    fn overrides(&self) -> CliOverrides {
        let space_before_paren = if self.space_before_paren.is_empty() {
            None
        } else {
            Some(
                self.space_before_paren
                    .iter()
                    .map(|value| value.to_ascii_lowercase())
                    .collect(),
            )
        };

        CliOverrides {
            line_width: self.line_width,
            indent_width: self.indent_width,
            use_tabs: self.use_tabs,
            new_line_kind: self.new_line_kind,
            command_case: self.command_case,
            keyword_case: self.keyword_case,
            closing_paren_newline: self.closing_paren_newline,
            sort_lists: self.sort_lists,
            max_blank_lines: self.max_blank_lines,
            space_before_paren,
        }
    }

    fn color_enabled(&self) -> bool {
        match (self.color, self.no_color) {
            (true, _) => true,
            (_, true) => false,
            (false, false) => io::stdout().is_terminal(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OutputControl {
    quiet: bool,
    verbose: bool,
    color: bool,
}

impl OutputControl {
    fn warning(&self, message: impl AsRef<str>) {
        if !self.quiet {
            eprintln!(
                "{}: {}",
                stylize("warning", "33", self.color),
                message.as_ref()
            );
        }
    }

    fn status(&self, message: impl AsRef<str>) {
        if !self.quiet {
            eprintln!("{}", message.as_ref());
        }
    }

    fn info(&self, message: impl AsRef<str>) {
        if self.verbose && !self.quiet {
            eprintln!(
                "{}: {}",
                stylize("info", "36", self.color),
                message.as_ref()
            );
        }
    }

    fn error(&self, message: impl AsRef<str>) {
        eprintln!(
            "{}: {}",
            stylize("error", "31", self.color),
            message.as_ref()
        );
    }
}

fn stylize(text: &str, color_code: &str, enabled: bool) -> String {
    if enabled {
        format!("\u{1b}[{color_code}m{text}\u{1b}[0m")
    } else {
        text.to_string()
    }
}

fn validate_flag_interactions(cli: &Cli, output: &OutputControl) -> Result<(), ExitCode> {
    if cli.check && cli.write {
        output.error("--check and --write are mutually exclusive");
        return Err(ExitCode::from(2));
    }
    if cli.diff && cli.write {
        output.error("--diff and --write are mutually exclusive");
        return Err(ExitCode::from(2));
    }
    if cli.stdin && !cli.files.is_empty() {
        output.error("--stdin cannot be used with file arguments");
        return Err(ExitCode::from(2));
    }
    if !cli.stdin && cli.assume_filename.is_some() {
        output.warning("--assume-filename is ignored without --stdin");
    }
    if cli.trace_summary_output.is_some() && cli.trace_output.is_none() {
        output.error("--trace-summary-output requires --trace-output");
        return Err(ExitCode::from(2));
    }
    Ok(())
}

fn apply_cli_overrides(config: &mut Configuration, overrides: &CliOverrides) {
    if let Some(line_width) = overrides.line_width {
        config.line_width = line_width;
    }
    if let Some(indent_width) = overrides.indent_width {
        config.indent_width = indent_width;
    }
    if overrides.use_tabs {
        config.use_tabs = true;
        config.indent_style = IndentStyle::Tab;
    }
    if let Some(new_line_kind) = overrides.new_line_kind {
        config.new_line_kind = new_line_kind;
    }
    if let Some(command_case) = overrides.command_case {
        config.command_case = command_case;
    }
    if let Some(keyword_case) = overrides.keyword_case {
        config.keyword_case = keyword_case;
    }
    if let Some(closing_paren_newline) = overrides.closing_paren_newline {
        config.closing_paren_newline = closing_paren_newline;
    }
    if overrides.sort_lists {
        config.sort_lists = true;
        config.sort_arguments = SortArguments::Enabled;
    }
    if let Some(max_blank_lines) = overrides.max_blank_lines {
        config.max_blank_lines = max_blank_lines;
    }
    if let Some(space_before_paren) = &overrides.space_before_paren {
        config.space_before_paren = SpaceBeforeParen::CommandList(space_before_paren.clone());
    }
}

fn load_effective_config(
    cli: &Cli,
    overrides: &CliOverrides,
    source_path: &Path,
) -> ConfigLoadResult {
    let mut load_result = match &cli.config {
        Some(config_path) => load_from_toml_path(config_path),
        None => load_from_toml_path(source_path),
    };
    apply_cli_overrides(&mut load_result.config, overrides);
    load_result
}

fn emit_config_diagnostics(output: &OutputControl, path: &Path, diagnostics: &[ConfigDiagnostic]) {
    for diagnostic in diagnostics {
        output.warning(format!(
            "{}: {} ({})",
            path.display(),
            diagnostic.message,
            diagnostic.key
        ));
    }
}

fn current_dir_fallback() -> PathBuf {
    match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::from("."),
    }
}

fn resolve_stdin_probe_path(cli: &Cli) -> PathBuf {
    match (&cli.stdin, &cli.assume_filename) {
        (true, Some(path)) => path.clone(),
        _ => current_dir_fallback(),
    }
}

/// Expand a list of file arguments (which may be globs) into concrete paths.
fn expand_globs(patterns: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    for pattern in patterns {
        let entries: Vec<_> = glob::glob(pattern)
            .map_err(|e| format!("invalid glob pattern '{}': {}", pattern, e))?
            .collect();
        if entries.is_empty() {
            // No glob match — treat as literal path.
            let p = PathBuf::from(pattern);
            if !p.exists() {
                return Err(format!("no such file: {}", pattern));
            }
            paths.push(p);
        } else {
            for entry in entries {
                let p = entry.map_err(|e| format!("glob error: {}", e))?;
                if p.is_file() {
                    paths.push(p);
                }
            }
        }
    }
    Ok(paths)
}

fn render_unified_diff(path: &Path, original: &str, formatted: &str, color: bool) -> String {
    if original == formatted {
        return String::new();
    }

    let original_count = if original.is_empty() {
        0
    } else {
        original.lines().count()
    };
    let formatted_count = if formatted.is_empty() {
        0
    } else {
        formatted.lines().count()
    };

    let mut lines = Vec::new();
    lines.push(format!("--- {}", path.display()));
    lines.push(format!("+++ {}", path.display()));
    lines.push(format!(
        "@@ -1,{} +1,{} @@",
        original_count, formatted_count
    ));
    lines.extend(original.lines().map(|line| format!("-{line}")));
    lines.extend(formatted.lines().map(|line| format!("+{line}")));

    let mut output = String::new();
    for line in lines {
        let rendered = if color {
            if line.starts_with("---") || line.starts_with("+++") || line.starts_with("@@") {
                stylize(&line, "36", true)
            } else if line.starts_with('-') {
                stylize(&line, "31", true)
            } else if line.starts_with('+') {
                stylize(&line, "32", true)
            } else {
                line
            }
        } else {
            line
        };
        output.push_str(&rendered);
        output.push('\n');
    }
    output
}

#[derive(Debug, Clone, Copy)]
struct FormatMode {
    check: bool,
    diff: bool,
    write: bool,
    color: bool,
}

#[derive(Debug, Default)]
struct RunState {
    any_unformatted: bool,
    had_error: bool,
    files_processed: usize,
    input_bytes_total: u64,
    changed_files: usize,
    error_count: usize,
    file_records: Vec<trace_summary::TraceFileRecord>,
}

struct TraceSession {
    trace_output: PathBuf,
    trace_summary_output: Option<PathBuf>,
    started_at: Instant,
    flush_guard: tracing_chrome::FlushGuard,
}

fn record_trace_file_result(
    state: &mut RunState,
    path: &Path,
    input: &str,
    changed: bool,
    status: &str,
) {
    state.file_records.push(trace_summary::TraceFileRecord {
        path: path.display().to_string(),
        input_bytes: input.len() as u64,
        changed,
        status: status.to_string(),
    });
}

fn format_single_input(
    path: &Path,
    input: &str,
    config: &Configuration,
    output: &OutputControl,
    mode: FormatMode,
    state: &mut RunState,
    write_back_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    state.files_processed += 1;
    state.input_bytes_total += input.len() as u64;

    match format_text(path, input, config) {
        Ok(result) => {
            let changed = result.is_some();
            let formatted = result.as_deref().unwrap_or(input);
            if changed {
                state.changed_files += 1;
            }

            if mode.check {
                if changed {
                    output.status(format!("would reformat: {}", path.display()));
                    state.any_unformatted = true;
                }
                if mode.diff && changed {
                    io::stdout().write_all(
                        render_unified_diff(path, input, formatted, mode.color).as_bytes(),
                    )?;
                }
                record_trace_file_result(state, path, input, changed, "ok");
                return Ok(());
            }

            if mode.diff {
                if changed {
                    io::stdout().write_all(
                        render_unified_diff(path, input, formatted, mode.color).as_bytes(),
                    )?;
                }
                record_trace_file_result(state, path, input, changed, "ok");
                return Ok(());
            }

            if mode.write {
                if let Some(destination) = write_back_path {
                    if changed {
                        std::fs::write(destination, formatted)?;
                        output.status(format!("formatted: {}", destination.display()));
                    }
                    record_trace_file_result(state, path, input, changed, "ok");
                    return Ok(());
                }
                output.error("--write cannot be used with stdin");
                state.had_error = true;
                state.error_count += 1;
                record_trace_file_result(state, path, input, false, "error");
                return Ok(());
            }

            io::stdout().write_all(formatted.as_bytes())?;
            record_trace_file_result(state, path, input, changed, "ok");
            Ok(())
        }
        Err(error) => {
            output.error(format!("{}: {error}", path.display()));
            state.had_error = true;
            state.error_count += 1;
            record_trace_file_result(state, path, input, false, "error");

            if !mode.check && !mode.diff && !mode.write {
                io::stdout().write_all(input.as_bytes())?;
            }

            Ok(())
        }
    }
}

fn print_config(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = toml::to_string_pretty(config)?;
    io::stdout().write_all(rendered.as_bytes())?;
    Ok(())
}

fn start_trace_session(cli: &Cli) -> Result<Option<TraceSession>, Box<dyn std::error::Error>> {
    let Some(trace_output) = cli.trace_output.clone() else {
        return Ok(None);
    };

    let (chrome_layer, flush_guard) = tracing_chrome::ChromeLayerBuilder::new()
        .file(trace_output.clone())
        .build();
    let filter = match &cli.trace_filter {
        Some(filter) => EnvFilter::try_new(filter.clone())?,
        None => EnvFilter::new("cmakefmt=info,cmakefmt_cli=info"),
    };

    Registry::default()
        .with(filter)
        .with(chrome_layer)
        .try_init()?;

    Ok(Some(TraceSession {
        trace_output,
        trace_summary_output: cli.trace_summary_output.clone(),
        started_at: Instant::now(),
        flush_guard,
    }))
}

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let output = OutputControl {
        quiet: cli.quiet,
        verbose: cli.verbose,
        color: cli.color_enabled(),
    };

    if let Err(code) = validate_flag_interactions(&cli, &output) {
        return Ok(code);
    }

    let mut trace_session = start_trace_session(&cli)?;
    let overrides = cli.overrides();
    let mode = FormatMode {
        check: cli.check,
        diff: cli.diff,
        write: cli.write,
        color: output.color,
    };
    let use_stdin = cli.stdin || cli.files.is_empty();

    let mut state = RunState::default();
    let mut invocation_file_count = 0usize;

    let mut run_result = (|| -> Result<ExitCode, Box<dyn std::error::Error>> {
        if use_stdin {
            invocation_file_count = 1;
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;

            let probe_path = resolve_stdin_probe_path(&cli);
            output.info(format!(
                "resolving stdin configuration from {}",
                probe_path.display()
            ));
            let config_result = load_effective_config(&cli, &overrides, &probe_path);
            emit_config_diagnostics(&output, &probe_path, &config_result.diagnostics);

            if cli.print_config {
                print_config(&config_result.config)?;
                return Ok(ExitCode::SUCCESS);
            }

            let path = cli
                .assume_filename
                .as_deref()
                .unwrap_or_else(|| Path::new("<stdin>"));
            format_single_input(
                path,
                &input,
                &config_result.config,
                &output,
                mode,
                &mut state,
                None,
            )?;

            if state.had_error {
                return Ok(ExitCode::from(2));
            }
            if cli.check && state.any_unformatted {
                return Ok(ExitCode::from(1));
            }
            return Ok(ExitCode::SUCCESS);
        }

        let paths = expand_globs(&cli.files)?;
        if paths.is_empty() {
            output.error("no files matched");
            return Ok(ExitCode::from(2));
        }
        invocation_file_count = paths.len();

        if cli.print_config {
            let first_path = &paths[0];
            let config_result = load_effective_config(&cli, &overrides, first_path);
            emit_config_diagnostics(&output, first_path, &config_result.diagnostics);
            print_config(&config_result.config)?;
            return Ok(ExitCode::SUCCESS);
        }

        // Cache config by parent directory to avoid re-discovering and re-parsing
        // the same .cmakefmt.toml for every file in the same directory tree.
        let mut config_cache: HashMap<PathBuf, ConfigLoadResult> = HashMap::new();

        for path in &paths {
            output.info(format!("formatting {}", path.display()));

            let input = std::fs::read_to_string(path)?;

            // Use explicit config path as cache key, or the source file's parent dir.
            let cache_key = cli.config.clone().unwrap_or_else(|| {
                path.parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("."))
            });
            let config_result = config_cache
                .entry(cache_key)
                .or_insert_with(|| load_effective_config(&cli, &overrides, path));
            emit_config_diagnostics(&output, path, &config_result.diagnostics);

            format_single_input(
                path,
                &input,
                &config_result.config,
                &output,
                mode,
                &mut state,
                Some(path.as_path()),
            )?;
        }

        if state.had_error {
            Ok(ExitCode::from(2))
        } else if cli.check && state.any_unformatted {
            Ok(ExitCode::from(1))
        } else {
            Ok(ExitCode::SUCCESS)
        }
    })();

    if let Some(session) = trace_session.take() {
        drop(session.flush_guard);

        if let Some(summary_path) = session.trace_summary_output.as_deref() {
            let file_count = if invocation_file_count > 0 {
                invocation_file_count
            } else {
                state.files_processed
            };
            let summary_input = trace_summary::TraceSummaryInput {
                tool_version: env!("CARGO_PKG_VERSION"),
                mode: trace_summary::TraceModeFlags {
                    check: mode.check,
                    diff: mode.diff,
                    write: mode.write,
                    stdin: use_stdin,
                },
                file_count,
                input_bytes_total: state.input_bytes_total,
                changed_files: state.changed_files,
                error_count: state.error_count,
                total_wall_ms: session.started_at.elapsed().as_secs_f64() * 1000.0,
                file_records: state.file_records.clone(),
            };

            if let Err(error) = trace_summary::write_summary_from_trace(
                &session.trace_output,
                summary_path,
                &summary_input,
            ) {
                output.error(format!(
                    "failed to write trace summary {}: {error}",
                    summary_path.display()
                ));
                if matches!(run_result, Ok(code) if code == ExitCode::SUCCESS) {
                    run_result = Ok(ExitCode::from(2));
                }
            }
        }
    }

    run_result
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

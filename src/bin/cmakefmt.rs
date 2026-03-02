use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use cmakefmt::{
    CaseStyle, ConfigLoadResult, Configuration, NewLineKind, format_text,
    load_from_cli,
};

#[derive(Parser)]
#[command(name = "cmakefmt", about = "Format CMake files", version)]
struct Cli {
    /// File paths or glob patterns to format.
    /// If omitted, reads from stdin.
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    /// Check if files are formatted (exit 1 if not). No files are modified.
    #[arg(long, conflicts_with = "write")]
    check: bool,

    /// Write formatted output back to files in-place.
    #[arg(short, long)]
    write: bool,

    /// Maximum line width.
    #[arg(long, default_value_t = Configuration::default().line_width)]
    line_width: u32,

    /// Number of spaces per indentation level.
    #[arg(long, default_value_t = Configuration::default().indent_width)]
    indent_width: u8,

    /// Use tabs instead of spaces for indentation.
    #[arg(long, default_value_t = Configuration::default().use_tabs)]
    use_tabs: bool,

    /// Newline style.
    #[arg(long, default_value_t = Configuration::default().new_line_kind)]
    new_line_kind: NewLineKind,

    /// Case style for commands.
    #[arg(long, default_value_t = Configuration::default().command_case)]
    command_case: CaseStyle,

    /// Case style for keywords.
    #[arg(long, default_value_t = Configuration::default().keyword_case)]
    keyword_case: CaseStyle,

    /// Place closing paren on a new line in multi-line commands.
    #[arg(long, default_value_t = Configuration::default().closing_paren_newline, action = clap::ArgAction::Set)]
    closing_paren_newline: bool,

    /// Sort argument lists alphabetically.
    #[arg(long, default_value_t = Configuration::default().sort_lists)]
    sort_lists: bool,

    /// Maximum consecutive blank lines to preserve.
    #[arg(long, default_value_t = Configuration::default().max_blank_lines)]
    max_blank_lines: u8,

    /// Insert a space before opening parenthesis.
    #[arg(long, default_value_t = Configuration::default().space_before_paren)]
    space_before_paren: bool,
}

impl Cli {
    fn to_config(&self) -> ConfigLoadResult {
        load_from_cli(Configuration {
            line_width: self.line_width,
            indent_width: self.indent_width,
            use_tabs: self.use_tabs,
            new_line_kind: self.new_line_kind,
            command_case: self.command_case,
            keyword_case: self.keyword_case,
            closing_paren_newline: self.closing_paren_newline,
            sort_lists: self.sort_lists,
            max_blank_lines: self.max_blank_lines,
            space_before_paren: self.space_before_paren,
        })
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

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config_result = cli.to_config();
    for diagnostic in &config_result.diagnostics {
        eprintln!("warning: {} ({})", diagnostic.message, diagnostic.key);
    }
    let config = config_result.config;

    if cli.files.is_empty() {
        return run_stdin(&cli, &config);
    }

    let paths = expand_globs(&cli.files)?;
    if paths.is_empty() {
        eprintln!("error: no files matched");
        return Ok(ExitCode::from(2));
    }

    let mut any_unformatted = false;

    for path in &paths {
        let input = std::fs::read_to_string(path)?;
        let result = format_text(path, &input, &config)?;

        match (cli.check, cli.write) {
            (true, _) => {
                if result.is_some() {
                    eprintln!("would reformat: {}", path.display());
                    any_unformatted = true;
                }
            }
            (false, true) => {
                if let Some(formatted) = result {
                    std::fs::write(path, formatted)?;
                    eprintln!("formatted: {}", path.display());
                }
            }
            (false, false) => {
                let output = result.as_deref().unwrap_or(&input);
                io::stdout().write_all(output.as_bytes())?;
            }
        }
    }

    if cli.check && any_unformatted {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn run_stdin(cli: &Cli, config: &Configuration) -> Result<ExitCode, Box<dyn std::error::Error>> {
    if cli.write {
        eprintln!("error: --write cannot be used with stdin");
        return Ok(ExitCode::from(2));
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let path = Path::new("<stdin>");
    let result = format_text(path, &input, config)?;

    if cli.check {
        if result.is_some() {
            Ok(ExitCode::from(1))
        } else {
            Ok(ExitCode::SUCCESS)
        }
    } else {
        let output = result.as_deref().unwrap_or(&input);
        io::stdout().write_all(output.as_bytes())?;
        Ok(ExitCode::SUCCESS)
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

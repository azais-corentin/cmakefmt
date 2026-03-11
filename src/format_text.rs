use std::path::Path;

use anyhow::Result;
use dprint_core::formatting::{self, PrintOptions};
use glob::Pattern;

use crate::configuration::{Configuration, NewLineKind, apply_inline_overrides};
use crate::generation::gen_file;
use crate::parser;

pub fn format_text(path: &Path, input: &str, config: &Configuration) -> Result<Option<String>> {
    if should_bypass_formatting(path, config) {
        return Ok(None);
    }

    let text = strip_bom(input);
    let result = format_inner(text, config)?;
    if result == input {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn should_bypass_formatting(path: &Path, config: &Configuration) -> bool {
    config.disable_formatting || should_ignore_path(path, &config.ignore_patterns)
}

fn should_ignore_path(path: &Path, ignore_patterns: &[String]) -> bool {
    if ignore_patterns.is_empty() {
        return false;
    }

    let canonical_path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    ignore_patterns.iter().any(|pattern| {
        Pattern::new(pattern).is_ok_and(|compiled| {
            compiled.matches_path(path) || compiled.matches_path(&canonical_path)
        })
    })
}

const PUSH_PREFIX: &str = "# cmakefmt: push";
const POP_MARKER: &str = "# cmakefmt: pop";

fn parse_push_directive(comment: &str) -> Option<&str> {
    let trimmed = comment.trim();
    if !trimmed.starts_with(PUSH_PREFIX) {
        return None;
    }
    let rest = trimmed[PUSH_PREFIX.len()..].trim();
    if rest.starts_with('{') {
        Some(rest)
    } else {
        None
    }
}

fn resolve_print_options_config(text: &str, base: &Configuration) -> Configuration {
    let mut current = base.clone();
    let mut stack: Vec<Configuration> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !trimmed.starts_with('#') {
            break;
        }

        if let Some(body) = parse_push_directive(trimmed) {
            stack.push(current.clone());
            current = apply_inline_overrides(&current, body);
            continue;
        }

        if trimmed == POP_MARKER
            && let Some(previous) = stack.pop()
        {
            current = previous;
        }
    }

    current
}

fn format_inner(text: &str, config: &Configuration) -> Result<String> {
    let file = parser::parse(text)?;
    let print_config = resolve_print_options_config(text, config);
    let print_options = build_print_options(text, &print_config);
    let result = formatting::format(|| gen_file(&file, text, config), print_options);
    Ok(result)
}

fn build_print_options(text: &str, config: &Configuration) -> PrintOptions {
    PrintOptions {
        max_width: config.line_width,
        indent_width: config.indent_width,
        use_tabs: config.use_tabs,
        new_line_text: resolve_new_line_kind(text, config.new_line_kind),
    }
}

fn resolve_new_line_kind(text: &str, kind: NewLineKind) -> &'static str {
    match kind {
        NewLineKind::Lf => "\n",
        NewLineKind::CrLf => "\r\n",
        NewLineKind::Auto => {
            // Detect from file content
            if let Some(idx) = text.find('\n') {
                if idx > 0 && text.as_bytes()[idx - 1] == b'\r' {
                    "\r\n"
                } else {
                    "\n"
                }
            } else {
                "\n"
            }
        }
    }
}

fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{feff}').unwrap_or(text)
}

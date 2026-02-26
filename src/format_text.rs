use std::path::Path;

use anyhow::Result;
use dprint_core::formatting::{self, PrintOptions};

use crate::configuration::{Configuration, NewLineKind};
use crate::generation::gen_file;
use crate::parser;

pub fn format_text(_path: &Path, input: &str, config: &Configuration) -> Result<Option<String>> {
    let text = strip_bom(input);
    let result = format_inner(text, config)?;
    if result == text {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn format_inner(text: &str, config: &Configuration) -> Result<String> {
    let file = parser::parse(text)?;
    let print_options = build_print_options(text, config);
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

use dprint_core::formatting::ir_helpers;
use dprint_core::formatting::{PrintItems, Signal};

use crate::configuration::Configuration;
use crate::parser::ast::{File, FileElement};

use super::gen_command::gen_command;

/// Flow-control commands that increase indentation for the block following them.
const BLOCK_OPENERS: &[&str] = &["if", "foreach", "while", "function", "macro", "block"];

/// Commands that sit at the same indentation level as their opener.
const BLOCK_MIDDLES: &[&str] = &["elseif", "else"];

/// Commands that decrease indentation (closing the block).
const BLOCK_CLOSERS: &[&str] = &[
    "endif",
    "endforeach",
    "endwhile",
    "endfunction",
    "endmacro",
    "endblock",
];

fn is_block_opener(name: &str) -> bool {
    BLOCK_OPENERS.iter().any(|&s| s.eq_ignore_ascii_case(name))
}

fn is_block_middle(name: &str) -> bool {
    BLOCK_MIDDLES.iter().any(|&s| s.eq_ignore_ascii_case(name))
}

fn is_block_closer(name: &str) -> bool {
    BLOCK_CLOSERS.iter().any(|&s| s.eq_ignore_ascii_case(name))
}

pub fn gen_file(file: &File, source: &str, config: &Configuration) -> PrintItems {
    let mut items = PrintItems::new();
    let mut consecutive_blanks: u8 = 0;
    let mut indent_level: u32 = 0;
    let mut first = true;

    // Strip trailing blank lines — the formatter adds its own trailing newline
    let elements: &[FileElement] = &file.elements;
    let last_content = elements
        .iter()
        .rposition(|e| !matches!(e, FileElement::BlankLine))
        .map(|i| i + 1)
        .unwrap_or(0);
    let elements = &elements[..last_content];

    for element in elements {
        match element {
            FileElement::BlankLine => {
                // Only emit blank lines between content, not before first content
                if !first {
                    consecutive_blanks += 1;
                    if consecutive_blanks <= config.max_blank_lines {
                        items.push_signal(Signal::NewLine);
                    }
                }
                continue;
            }
            _ => {
                consecutive_blanks = 0;
            }
        }

        match element {
            FileElement::Command(cmd) => {
                let cmd_name = cmd.name.text(source);

                // Adjust indent BEFORE emitting the command for middles/closers
                if (is_block_closer(cmd_name) || is_block_middle(cmd_name)) && indent_level > 0 {
                    indent_level -= 1;
                }

                if !first {
                    items.push_signal(Signal::NewLine);
                }
                first = false;

                // Emit indentation
                let cmd_items = gen_command(cmd, source, config);
                if indent_level > 0 {
                    items.extend(ir_helpers::with_indent_times(cmd_items, indent_level));
                } else {
                    items.extend(cmd_items);
                }

                // Adjust indent AFTER emitting for openers/middles
                if is_block_opener(cmd_name) || is_block_middle(cmd_name) {
                    indent_level += 1;
                }
            }
            FileElement::LineComment(span) => {
                if !first {
                    items.push_signal(Signal::NewLine);
                }
                first = false;
                let comment_text = span.text(source);
                let comment_items = ir_helpers::gen_from_raw_string(comment_text);
                if indent_level > 0 {
                    items.extend(ir_helpers::with_indent_times(comment_items, indent_level));
                } else {
                    items.extend(comment_items);
                }
            }
            FileElement::BracketComment(span) => {
                if !first {
                    items.push_signal(Signal::NewLine);
                }
                first = false;
                let comment_text = span.text(source);
                let comment_items = ir_helpers::gen_from_raw_string(comment_text);
                if indent_level > 0 {
                    items.extend(ir_helpers::with_indent_times(comment_items, indent_level));
                } else {
                    items.extend(comment_items);
                }
            }
            FileElement::BlankLine => unreachable!(),
        }
    }

    // Ensure trailing newline
    if !first {
        items.push_signal(Signal::NewLine);
    }

    items
}

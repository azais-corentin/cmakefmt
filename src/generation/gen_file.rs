use dprint_core::formatting::ir_helpers;
use dprint_core::formatting::{PrintItems, Signal};

use crate::configuration::{Configuration, EndCommandArgs, apply_inline_overrides};
use crate::parser::ast::{Argument, CommandInvocation, File, FileElement};

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

const PRAGMA_PREFIX: &str = "cmakefmt:";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PragmaDirective<'a> {
    Off,
    On,
    Skip,
    Pop,
    Push(&'a str),
}

fn parse_pragma_directive(comment: &str) -> Option<PragmaDirective<'_>> {
    let trimmed = comment.trim();
    let rest = trimmed.strip_prefix('#')?.trim_start();
    let rest = rest.strip_prefix(PRAGMA_PREFIX)?.trim_start();
    if rest.is_empty() {
        return None;
    }

    let action_end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let action = &rest[..action_end];
    let remainder = &rest[action_end..];

    match action {
        "off" => Some(PragmaDirective::Off),
        "on" => Some(PragmaDirective::On),
        "skip" => Some(PragmaDirective::Skip),
        "pop" => Some(PragmaDirective::Pop),
        "push" => {
            let body = remainder.trim_start();
            if body.starts_with('{') {
                Some(PragmaDirective::Push(body))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_ignored_command(command_name: &str, config: &Configuration) -> bool {
    config
        .ignore_commands
        .iter()
        .any(|configured_name| configured_name.eq_ignore_ascii_case(command_name))
}

fn command_source_slice<'a>(cmd: &CommandInvocation, source: &'a str) -> &'a str {
    let line_start = source[..cmd.name.start]
        .rfind('\n')
        .map_or(0, |newline_index| newline_index + 1);

    let mut line_end = cmd.close_paren.end;
    while line_end < source.len() {
        let byte = source.as_bytes()[line_end];
        if byte == b'\n' || byte == b'\r' {
            break;
        }
        line_end += 1;
    }

    &source[line_start..line_end]
}

/// Returns the corresponding opener name for a block closer.
fn closer_to_opener(name: &str) -> Option<&'static str> {
    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        "endif" => Some("if"),
        "endforeach" => Some("foreach"),
        "endwhile" => Some("while"),
        "endfunction" => Some("function"),
        "endmacro" => Some("macro"),
        "endblock" => Some("block"),
        _ => None,
    }
}

/// Tracks block openers for endCommandArgs="match" during file traversal.
/// Stores the opener's AST arguments so they can be copied to the closer.
struct BlockStack {
    /// Stack of (opener_name_lower, opener_arguments)
    entries: Vec<(String, Vec<Argument>)>,
}

impl BlockStack {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Push a new block opener onto the stack.
    fn push_opener(&mut self, name: &str, args: Vec<Argument>) {
        self.entries.push((name.to_ascii_lowercase(), args));
    }

    /// Pop the most recent block opener.
    fn pop_opener(&mut self) {
        self.entries.pop();
    }

    /// Get the opener args for a given closer command.
    /// For block closers (endif, endforeach, etc.), returns the nearest matching opener's args.
    /// For else(), returns the nearest unmatched if()'s args.
    fn get_opener_args(&self, closer_name: &str) -> Option<&[Argument]> {
        let lower = closer_name.to_ascii_lowercase();

        if lower == "else" {
            // else() matches the nearest unmatched if()
            for entry in self.entries.iter().rev() {
                if entry.0 == "if" {
                    return Some(&entry.1);
                }
            }
            None
        } else if let Some(opener) = closer_to_opener(&lower) {
            for entry in self.entries.iter().rev() {
                if entry.0 == opener {
                    return Some(&entry.1);
                }
            }
            None
        } else {
            None
        }
    }
}

/// Apply endCommandArgs policy to a command invocation.
/// Returns replacement arguments if the command should be modified, or None to keep as-is.
fn apply_end_command_args(
    cmd: &CommandInvocation,
    source: &str,
    config: &Configuration,
    block_stack: &BlockStack,
) -> Option<Vec<Argument>> {
    let cmd_name = cmd.name.text(source);
    let lower = cmd_name.to_ascii_lowercase();

    // elseif is never affected by endCommandArgs — its condition is definitional
    if lower == "elseif" {
        return None;
    }

    // Only closers and else() are affected
    if !is_block_closer(cmd_name) && lower != "else" {
        return None;
    }

    match config.end_command_args {
        EndCommandArgs::Remove => {
            // Strip all arguments from closing commands and else()
            if cmd.arguments.is_empty() {
                None // Already empty
            } else {
                Some(Vec::new())
            }
        }
        EndCommandArgs::Preserve => None,
        EndCommandArgs::Match => {
            // block()/endblock(): block() has keyword clauses, not a positional name.
            // endblock() is always empty per §14.2.
            if lower == "endblock" {
                if cmd.arguments.is_empty() {
                    return None;
                }
                return Some(Vec::new());
            }

            if let Some(opener_args) = block_stack.get_opener_args(cmd_name) {
                if opener_args.is_empty() {
                    // Opener has no args → closer should be empty too
                    if cmd.arguments.is_empty() {
                        None
                    } else {
                        Some(Vec::new())
                    }
                } else {
                    // Replace closer's args with opener's args (cloned).
                    // The Span values in the cloned arguments still point to valid positions
                    // in the original source text.
                    Some(opener_args.to_vec())
                }
            } else {
                None
            }
        }
    }
}

/// Filter out comments from arguments (keep only real arguments for matching).
fn non_comment_args(args: &[Argument]) -> Vec<Argument> {
    args.iter()
        .filter(|a| !matches!(a, Argument::LineComment(_) | Argument::BracketComment(_)))
        .cloned()
        .collect()
}

pub fn gen_file(file: &File, source: &str, config: &Configuration) -> PrintItems {
    let mut items = PrintItems::new();
    let mut pending_blanks: u8 = 0;
    let mut indent_level: u32 = 0;
    let mut first = true;
    let mut just_opened_block = false;

    // Config stack for push/pop directives
    let mut config_stack: Vec<Configuration> = Vec::new();
    let mut current_config = config.clone();

    // Block stack for endCommandArgs="match"
    let mut block_stack = BlockStack::new();

    let mut skip_next_command = false;
    let mut formatting_disabled = false;

    // Strip trailing blank lines — the formatter adds its own trailing newline
    let elements: &[FileElement] = &file.elements;
    let last_content = elements
        .iter()
        .rposition(|e| !matches!(e, FileElement::BlankLine))
        .map(|i| i + 1)
        .unwrap_or(0);
    let elements = &elements[..last_content];

    for element in elements {
        if let FileElement::BlankLine = element {
            if formatting_disabled {
                if !first {
                    items.push_signal(Signal::NewLine);
                }
                first = false;
            } else if !first && !just_opened_block {
                pending_blanks = (pending_blanks + 1).min(current_config.max_blank_lines);
            }
            continue;
        }

        if formatting_disabled {
            match element {
                FileElement::Command(cmd) => {
                    if !first {
                        items.push_signal(Signal::NewLine);
                    }
                    first = false;
                    items.extend(ir_helpers::gen_from_raw_string(command_source_slice(
                        cmd, source,
                    )));
                }
                FileElement::LineComment(span) => {
                    let comment_text = span.text(source);
                    if !first {
                        items.push_signal(Signal::NewLine);
                    }
                    first = false;
                    items.extend(ir_helpers::gen_from_raw_string(comment_text));

                    if matches!(
                        parse_pragma_directive(comment_text),
                        Some(PragmaDirective::On)
                    ) {
                        formatting_disabled = false;
                    }
                }
                FileElement::BracketComment(span) => {
                    if !first {
                        items.push_signal(Signal::NewLine);
                    }
                    first = false;
                    items.extend(ir_helpers::gen_from_raw_string(span.text(source)));
                }
                FileElement::BlankLine => unreachable!(),
            }
            continue;
        }

        // Suppress blank lines before block closers/middles
        let is_closer_or_middle = matches!(element, FileElement::Command(cmd) if {
            let name = cmd.name.text(source);
            is_block_closer(name) || is_block_middle(name)
        });
        if is_closer_or_middle {
            pending_blanks = 0;
        }

        // Emit pending blank lines
        for _ in 0..pending_blanks {
            items.push_signal(Signal::NewLine);
        }
        pending_blanks = 0;
        just_opened_block = false;
        match element {
            FileElement::Command(cmd) => {
                let cmd_name = cmd.name.text(source);

                // Adjust indent BEFORE emitting the command for middles/closers
                let was_in_block = indent_level > 0;
                if (is_block_closer(cmd_name) || is_block_middle(cmd_name)) && indent_level > 0 {
                    indent_level -= 1;
                }

                // Track block opener args for endCommandArgs="match"
                if is_block_opener(cmd_name) {
                    block_stack.push_opener(cmd_name, non_comment_args(&cmd.arguments));
                }

                let preserve_verbatim =
                    skip_next_command || is_ignored_command(cmd_name, &current_config);
                if skip_next_command {
                    skip_next_command = false;
                }

                if !first {
                    items.push_signal(Signal::NewLine);
                }
                first = false;

                if preserve_verbatim {
                    // Suppressed commands must remain byte-for-byte identical on their own line.
                    let raw_command = command_source_slice(cmd, source);
                    items.extend(ir_helpers::gen_from_raw_string(raw_command));
                } else {
                    // Apply endCommandArgs policy — may replace the command's arguments
                    let modified_cmd;
                    let effective_cmd = if let Some(new_args) =
                        apply_end_command_args(cmd, source, &current_config, &block_stack)
                    {
                        modified_cmd = CommandInvocation {
                            name: cmd.name,
                            open_paren: cmd.open_paren,
                            close_paren: cmd.close_paren,
                            arguments: new_args,
                            trailing_comment: cmd.trailing_comment,
                        };
                        &modified_cmd
                    } else {
                        cmd
                    };

                    let cmd_items =
                        gen_command(effective_cmd, source, &current_config, indent_level);
                    if indent_level > 0 {
                        items.extend(ir_helpers::with_indent_times(cmd_items, indent_level));
                    } else {
                        items.extend(cmd_items);
                    }
                }

                // Adjust indent AFTER emitting for openers/middles
                if is_block_opener(cmd_name) {
                    if current_config.indent_block_body {
                        indent_level += 1;
                    }
                    just_opened_block = true;
                } else if is_block_middle(cmd_name) && was_in_block {
                    // Only re-indent after a middle if we were actually inside a block
                    if current_config.indent_block_body {
                        indent_level += 1;
                    }
                    just_opened_block = true;
                }

                // Pop block stack for closers
                if is_block_closer(cmd_name) {
                    block_stack.pop_opener();
                }
            }
            FileElement::LineComment(span) => {
                let comment_text = span.text(source).trim_end();
                let directive = parse_pragma_directive(comment_text);

                if let Some(PragmaDirective::Push(body)) = directive {
                    config_stack.push(current_config.clone());
                    current_config = apply_inline_overrides(&current_config, body);
                }
                let is_pop = matches!(directive, Some(PragmaDirective::Pop));
                let is_skip = matches!(directive, Some(PragmaDirective::Skip));
                let is_off = matches!(directive, Some(PragmaDirective::Off));
                if !first {
                    items.push_signal(Signal::NewLine);
                }
                first = false;
                let comment_items = ir_helpers::gen_from_raw_string(comment_text);
                if indent_level > 0 {
                    items.extend(ir_helpers::with_indent_times(comment_items, indent_level));
                } else {
                    items.extend(comment_items);
                }

                if is_skip {
                    skip_next_command = true;
                }
                if is_off {
                    // `off` starts a byte-preserving region and consumes any pending `skip`.
                    formatting_disabled = true;
                    skip_next_command = false;
                }

                // Pop config after emitting the pop comment
                if is_pop && let Some(prev) = config_stack.pop() {
                    current_config = prev;
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
    items.push_signal(Signal::NewLine);

    items
}

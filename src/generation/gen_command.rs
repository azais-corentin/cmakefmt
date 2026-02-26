use dprint_core::formatting::ir_helpers;
use dprint_core::formatting::{PrintItems, Signal};

use crate::configuration::{CaseStyle, Configuration};
use crate::parser::ast::{Argument, CommandInvocation};

/// Keywords recognized for casing normalization.
const KNOWN_KEYWORDS: &[&str] = &[
    "PUBLIC",
    "PRIVATE",
    "INTERFACE",
    "IMPORTED",
    "ALIAS",
    "TARGETS",
    "FILES",
    "DESTINATION",
    "PERMISSIONS",
    "CONFIGURATIONS",
    "COMPONENT",
    "NAMESPACE",
    "EXPORT",
    "COMMAND",
    "DEPENDS",
    "OUTPUT",
    "WORKING_DIRECTORY",
    "COMMENT",
    "VERBATIM",
    "APPEND",
    "SOURCES",
    "COMPILE_FLAGS",
    "COMPILE_DEFINITIONS",
    "COMPILE_OPTIONS",
    "INCLUDE_DIRECTORIES",
    "LINK_LIBRARIES",
    "LINK_DIRECTORIES",
    "LINK_OPTIONS",
    "PROPERTIES",
    "REQUIRED",
    "COMPONENTS",
    "OPTIONAL_COMPONENTS",
    "CONFIG",
    "MODULE",
    "NO_MODULE",
    "QUIET",
    "NAMES",
    "PATHS",
    "HINTS",
    "PATH_SUFFIXES",
    "DOC",
    "TYPE",
    "RUNTIME",
    "LIBRARY",
    "ARCHIVE",
    "FRAMEWORK",
    "BUNDLE",
    "OBJECTS",
    "RESULT_VARIABLE",
    "VERSION",
    "DESCRIPTION",
    "HOMEPAGE_URL",
    "LANGUAGES",
    "FATAL_ERROR",
    "SEND_ERROR",
    "WARNING",
    "AUTHOR_WARNING",
    "DEPRECATION",
    "STATUS",
    "VERBOSE",
    "DEBUG",
    "TRACE",
    "CHECK_START",
    "CHECK_PASS",
    "CHECK_FAIL",
    "BOOL",
    "FILEPATH",
    "PATH",
    "STRING",
    "INTERNAL",
    "FORCE",
    "PARENT_SCOPE",
    "CACHE",
    "STREQUAL",
    "MATCHES",
    "LESS",
    "GREATER",
    "EQUAL",
    "LESS_EQUAL",
    "GREATER_EQUAL",
    "VERSION_LESS",
    "VERSION_GREATER",
    "VERSION_EQUAL",
    "VERSION_LESS_EQUAL",
    "VERSION_GREATER_EQUAL",
    "AND",
    "OR",
    "NOT",
    "DEFINED",
    "IN",
    "LISTS",
    "ITEMS",
    "RANGE",
    "ZIP_LISTS",
    "DIRECTORY",
    "TARGET",
    "TEST",
    "PROPERTY",
    "GLOBAL",
    "VARIABLE",
    "BRIEF_DOCS",
    "FULL_DOCS",
    "PROPAGATE",
    "CONDITION",
    "ON",
    "OFF",
    "TRUE",
    "FALSE",
    "YES",
    "NO",
];

/// Commands where arguments after keywords can be sorted.
const SORTABLE_COMMANDS: &[&str] = &[
    "target_link_libraries",
    "target_include_directories",
    "target_compile_options",
    "target_compile_definitions",
    "target_sources",
    "find_package",
];

/// Keywords that start a sortable group.
const SORT_GROUP_KEYWORDS: &[&str] = &[
    "PUBLIC",
    "PRIVATE",
    "INTERFACE",
    "COMPONENTS",
    "OPTIONAL_COMPONENTS",
    "SOURCES",
];

fn is_known_keyword(text: &str) -> bool {
    KNOWN_KEYWORDS.iter().any(|&k| k.eq_ignore_ascii_case(text))
}

fn is_sortable_command(name: &str) -> bool {
    SORTABLE_COMMANDS
        .iter()
        .any(|&c| c.eq_ignore_ascii_case(name))
}

fn is_sort_group_keyword(text: &str) -> bool {
    SORT_GROUP_KEYWORDS
        .iter()
        .any(|&k| k.eq_ignore_ascii_case(text))
}

fn apply_command_case(name: &str, style: CaseStyle) -> String {
    match style {
        CaseStyle::Lower => name.to_ascii_lowercase(),
        CaseStyle::Upper => name.to_ascii_uppercase(),
        CaseStyle::Preserve => name.to_string(),
    }
}

fn apply_keyword_case(text: &str, style: CaseStyle) -> String {
    match style {
        CaseStyle::Lower => text.to_ascii_lowercase(),
        CaseStyle::Upper => text.to_ascii_uppercase(),
        CaseStyle::Preserve => text.to_string(),
    }
}

pub fn gen_command(cmd: &CommandInvocation, source: &str, config: &Configuration) -> PrintItems {
    let mut items = PrintItems::new();

    // Command name with casing
    let raw_name = cmd.name.text(source);
    let formatted_name = apply_command_case(raw_name, config.command_case);
    items.push_string(formatted_name.clone());

    // Space before paren
    if config.space_before_paren {
        items.push_space();
    }
    items.push_str_runtime_width_computed("(");

    // Determine if arguments need sorting
    let mut arguments = build_argument_list(cmd, source, config);

    // Sort if applicable
    if config.sort_lists && is_sortable_command(raw_name) {
        sort_argument_groups(&mut arguments);
    }

    // Format arguments
    if !arguments.is_empty() {
        let single_line = try_single_line(&formatted_name, &arguments, config);
        if let Some(single) = single_line {
            items.extend(single);
        } else {
            items.extend(gen_multi_line_args(&arguments, config));
        }
    }

    items.push_str_runtime_width_computed(")");

    // Trailing comment
    if let Some(comment_span) = &cmd.trailing_comment {
        items.push_space();
        items.push_string(comment_span.text(source).to_string());
    }

    items
}

/// Represents a processed argument ready for formatting.
#[derive(Debug, Clone)]
struct FormattedArg {
    text: String,
    is_keyword: bool,
    trailing_comment: Option<String>,
    is_paren_group: bool,
    paren_inner: Vec<FormattedArg>,
}

fn build_argument_list(
    cmd: &CommandInvocation,
    source: &str,
    config: &Configuration,
) -> Vec<FormattedArg> {
    let mut result = Vec::new();
    let mut i = 0;
    let args = &cmd.arguments;

    while i < args.len() {
        match &args[i] {
            Argument::Bracket(span) => {
                result.push(FormattedArg {
                    text: span.text(source).to_string(),
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
            Argument::Quoted(span) => {
                result.push(FormattedArg {
                    text: span.text(source).to_string(),
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
            Argument::Unquoted(span) => {
                let text = span.text(source);
                let is_kw = is_known_keyword(text);
                let formatted_text = if is_kw {
                    apply_keyword_case(text, config.keyword_case)
                } else {
                    text.to_string()
                };
                result.push(FormattedArg {
                    text: formatted_text,
                    is_keyword: is_kw,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
            Argument::ParenGroup { arguments } => {
                let inner = build_argument_list_from_args(arguments, source, config);
                result.push(FormattedArg {
                    text: String::new(),
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: true,
                    paren_inner: inner,
                });
            }
            Argument::LineComment(span) => {
                // Attach comment to previous argument if possible
                let comment_text = span.text(source).to_string();
                if let Some(last) = result.last_mut() {
                    if last.trailing_comment.is_none() {
                        last.trailing_comment = Some(comment_text);
                        i += 1;
                        continue;
                    }
                }
                // Standalone comment — emit as a pseudo-argument
                result.push(FormattedArg {
                    text: comment_text,
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
        }
        i += 1;
    }

    result
}

fn build_argument_list_from_args(
    args: &[Argument],
    source: &str,
    config: &Configuration,
) -> Vec<FormattedArg> {
    // Reuse the same logic but with a slice of arguments
    let mut result = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match &args[i] {
            Argument::Bracket(span) => {
                result.push(FormattedArg {
                    text: span.text(source).to_string(),
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
            Argument::Quoted(span) => {
                result.push(FormattedArg {
                    text: span.text(source).to_string(),
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
            Argument::Unquoted(span) => {
                let text = span.text(source);
                let is_kw = is_known_keyword(text);
                let formatted_text = if is_kw {
                    apply_keyword_case(text, config.keyword_case)
                } else {
                    text.to_string()
                };
                result.push(FormattedArg {
                    text: formatted_text,
                    is_keyword: is_kw,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
            Argument::ParenGroup { arguments } => {
                let inner = build_argument_list_from_args(arguments, source, config);
                result.push(FormattedArg {
                    text: String::new(),
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: true,
                    paren_inner: inner,
                });
            }
            Argument::LineComment(span) => {
                let comment_text = span.text(source).to_string();
                if let Some(last) = result.last_mut() {
                    if last.trailing_comment.is_none() {
                        last.trailing_comment = Some(comment_text);
                        i += 1;
                        continue;
                    }
                }
                result.push(FormattedArg {
                    text: comment_text,
                    is_keyword: false,
                    trailing_comment: None,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                });
            }
        }
        i += 1;
    }

    result
}

fn format_paren_group_inline(args: &[FormattedArg]) -> String {
    let mut s = String::from("(");
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        if arg.is_paren_group {
            s.push_str(&format_paren_group_inline(&arg.paren_inner));
        } else {
            s.push_str(&arg.text);
        }
    }
    s.push(')');
    s
}

fn arg_inline_text(arg: &FormattedArg) -> String {
    if arg.is_paren_group {
        format_paren_group_inline(&arg.paren_inner)
    } else {
        arg.text.clone()
    }
}

/// Try to format all arguments on a single line. Returns None if it would
/// exceed line_width or if any argument has a trailing comment.
fn try_single_line(
    cmd_name: &str,
    args: &[FormattedArg],
    config: &Configuration,
) -> Option<PrintItems> {
    // If any argument has a trailing comment, force multi-line
    if args.iter().any(|a| a.trailing_comment.is_some()) {
        return None;
    }

    // If any argument text contains a newline (e.g., bracket arg), force multi-line
    if args.iter().any(|a| a.text.contains('\n')) {
        return None;
    }

    // Calculate total width: cmd_name + "(" + args joined by spaces + ")"
    let close_overhead = 1; // ")"
    let args_text: Vec<String> = args.iter().map(arg_inline_text).collect();
    let args_width: usize = args_text.iter().map(|s| s.len()).sum::<usize>()
        + if args_text.len() > 1 {
            args_text.len() - 1
        } else {
            0
        }; // spaces between args
    let total = cmd_name.len() + 1 + args_width + close_overhead; // +1 for "("

    if total > config.line_width as usize {
        return None;
    }

    let mut items = PrintItems::new();
    for (i, text) in args_text.iter().enumerate() {
        if i > 0 {
            items.push_space();
        }
        items.push_string(text.clone());
    }
    Some(items)
}

/// Generate multi-line argument formatting.
fn gen_multi_line_args(args: &[FormattedArg], config: &Configuration) -> PrintItems {
    let mut items = PrintItems::new();
    let mut inner = PrintItems::new();

    for arg in args {
        // Each argument on its own line
        inner.push_signal(Signal::NewLine);

        if arg.is_paren_group {
            inner.push_str_runtime_width_computed("(");
            if !arg.paren_inner.is_empty() {
                let paren_items = gen_multi_line_args(&arg.paren_inner, config);
                inner.extend(paren_items);
            }
            inner.push_str_runtime_width_computed(")");
        } else {
            inner.push_string(arg.text.clone());
        }

        if let Some(comment) = &arg.trailing_comment {
            inner.push_space();
            inner.push_string(comment.clone());
        }
    }

    if config.closing_paren_newline {
        inner.push_signal(Signal::NewLine);
    }

    items.extend(ir_helpers::with_indent(inner));

    items
}

/// Sort arguments within keyword groups for sortable commands.
fn sort_argument_groups(args: &mut [FormattedArg]) {
    // Find keyword group boundaries and sort within each group
    let mut i = 0;
    while i < args.len() {
        if args[i].is_keyword && is_sort_group_keyword(&args[i].text) {
            // Found a keyword — sort everything after it until next keyword or end
            let start = i + 1;
            let mut end = start;
            while end < args.len() && !args[end].is_keyword {
                end += 1;
            }
            if end > start {
                args[start..end].sort_by(|a, b| {
                    a.text
                        .to_ascii_lowercase()
                        .cmp(&b.text.to_ascii_lowercase())
                });
            }
            i = end;
        } else {
            i += 1;
        }
    }
}

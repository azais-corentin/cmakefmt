use dprint_core::formatting::ir_helpers;
use dprint_core::formatting::{PrintItems, Signal};

use crate::configuration::{CaseStyle, Configuration, SortArguments};
use crate::parser::ast::{Argument, CommandInvocation};

use super::signatures::{CommandKind, CommandSpec, EMPTY_SPEC, KwType, lookup_command};
// ---------------------------------------------------------------------------
// Literal tokens: boolean values and comparison operators subject to literalCase.
// Keyword-vs-literal precedence: if a token is a keyword for the current
// command (per CommandSpec) or in customKeywords, keywordCase wins; otherwise
// literalCase applies when the token matches this list (§4.4).
// ---------------------------------------------------------------------------
const LITERAL_TOKENS: &[&str] = &[
    // Boolean values
    "ON",
    "OFF",
    "TRUE",
    "FALSE",
    "YES",
    "NO",
    // Logical/condition operators
    "AND",
    "OR",
    "NOT",
    // Comparison operators
    "STREQUAL",
    "STRLESS",
    "STRGREATER",
    "STRLESS_EQUAL",
    "STRGREATER_EQUAL",
    "VERSION_EQUAL",
    "VERSION_LESS",
    "VERSION_GREATER",
    "VERSION_LESS_EQUAL",
    "VERSION_GREATER_EQUAL",
    "EQUAL",
    "LESS",
    "GREATER",
    "LESS_EQUAL",
    "GREATER_EQUAL",
    "MATCHES",
    "IN_LIST",
    // Unary test keywords used as literals when not in keyword position
    "DEFINED",
    "COMMAND",
    "POLICY",
    "TARGET",
    "TEST",
    "EXISTS",
    "IS_DIRECTORY",
    "IS_SYMLINK",
    "IS_ABSOLUTE",
    "IS_NEWER_THAN",
    "PATH_EQUAL",
];

/// Check if a token is in the literal list (case-insensitive).
fn is_literal_token(text: &str) -> bool {
    LITERAL_TOKENS
        .iter()
        .any(|&lit| lit.eq_ignore_ascii_case(text))
}

/// Apply literal casing per §4.4. Only applies to unquoted arguments that match
/// the literal token list and are NOT classified as keywords for the current command.
fn apply_literal_case(text: &str, style: CaseStyle) -> String {
    match style {
        CaseStyle::Lower => text.to_ascii_lowercase(),
        CaseStyle::Upper => text.to_ascii_uppercase(),
        CaseStyle::Preserve => text.to_string(),
    }
}

/// Check if a token is a keyword in the context of a specific command.
/// Uses the command's spec keywords + sections + customKeywords.
/// For condition-syntax commands, condition operators count as keywords.
fn is_keyword_for_command(
    text: &str,
    cmd_kind: Option<&CommandKind>,
    config: &Configuration,
) -> bool {
    // customKeywords always take keyword precedence
    if config
        .custom_keywords
        .iter()
        .any(|k| k.eq_ignore_ascii_case(text))
    {
        return true;
    }

    match cmd_kind {
        Some(CommandKind::Known(spec)) => is_in_command_spec(text, spec),
        Some(CommandKind::ConditionSyntax) => {
            // In condition-syntax commands, unary test ops, logical ops, and NOT
            // are keywords. Comparison operators and booleans are NOT keywords
            // (they get literalCase instead).
            is_condition_keyword(text)
        }
        None => is_sort_group_keyword(text),
    }
}

/// Check if a token appears as a keyword or section keyword in a CommandSpec.
fn is_in_command_spec(text: &str, spec: &CommandSpec) -> bool {
    // Check top-level keywords
    for &(kw, _) in spec.keywords {
        if kw.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    // Check section keywords
    for &(sec_kw, _, sub_kw) in spec.sections {
        if sec_kw.eq_ignore_ascii_case(text) {
            return true;
        }
        for &(sub, kw_type) in sub_kw {
            if sub.eq_ignore_ascii_case(text) {
                return true;
            }
            // Check group sub-keywords
            if let KwType::Group(_, group_kw) = kw_type {
                for &(gk, _) in group_kw {
                    if gk.eq_ignore_ascii_case(text) {
                        return true;
                    }
                }
            }
        }
    }
    // Check command_line_keywords, pair_keywords, property_keywords,
    // flow_keywords, compound_keywords, once_keywords
    for &kw in spec.command_line_keywords {
        if kw.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    for &kw in spec.pair_keywords {
        if kw.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    for &kw in spec.property_keywords {
        if kw.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    for &kw in spec.flow_keywords {
        if kw.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    for &(kw1, kw2) in spec.compound_keywords {
        if kw1.eq_ignore_ascii_case(text) || kw2.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    for &kw in spec.once_keywords {
        if kw.eq_ignore_ascii_case(text) {
            return true;
        }
    }
    false
}

/// In condition-syntax commands (if, elseif, while), these tokens are treated
/// as keywords and get keywordCase. Others (comparison operators, booleans)
/// get literalCase instead.
fn is_condition_keyword(text: &str) -> bool {
    // Unary test operators: DEFINED, TARGET, COMMAND, POLICY, TEST, EXISTS, etc.
    is_condition_unary_test(text)
        // Logical operators: AND, OR
        || is_logical_op(text)
        // NOT operator
        || is_not_op(text)
}

/// Commands where arguments after keywords can be sorted.
const SORTABLE_COMMANDS: &[&str] = &[
    "target_link_libraries",
    "target_include_directories",
    "target_compile_options",
    "target_compile_definitions",
    "target_compile_features",
    "target_link_options",
    "target_sources",
    "find_package",
    "add_custom_command",
];

/// Default keywords that start a sortable group (used when sortArguments=true).
const SORT_GROUP_KEYWORDS: &[&str] = &[
    "PUBLIC",
    "PRIVATE",
    "INTERFACE",
    "COMPONENTS",
    "OPTIONAL_COMPONENTS",
    "SOURCES",
    "DEPENDS",
    "BYPRODUCTS",
];

/// Canonical section ordering for sortKeywordSections per Appendix F.
/// Commands not listed here get no reordering even when sortKeywordSections=true.
fn canonical_section_order(cmd_name: &str) -> Option<&'static [&'static str]> {
    let lower = cmd_name.to_ascii_lowercase();
    match lower.as_str() {
        "target_link_libraries" => Some(&[
            "PUBLIC",
            "INTERFACE",
            "PRIVATE",
            "LINK_PUBLIC",
            "LINK_PRIVATE",
            "LINK_INTERFACE_LIBRARIES",
        ]),
        "target_sources"
        | "target_compile_definitions"
        | "target_compile_options"
        | "target_compile_features"
        | "target_link_options"
        | "target_include_directories" => Some(&["PUBLIC", "INTERFACE", "PRIVATE"]),
        "install" => Some(&[
            "ARCHIVE",
            "LIBRARY",
            "RUNTIME",
            "OBJECTS",
            "FRAMEWORK",
            "BUNDLE",
            "PUBLIC_HEADER",
            "PRIVATE_HEADER",
            "RESOURCE",
            "FILE_SET",
        ]),
        "export" => Some(&["PACKAGE_DEPENDENCY", "TARGET", "VERSION"]),
        _ => None,
    }
}

fn is_known_keyword(text: &str, cmd_kind: Option<&CommandKind>, config: &Configuration) -> bool {
    is_keyword_for_command(text, cmd_kind, config)
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

/// Apply keyword casing. Unlike the old version, this does NOT normalize booleans —
/// boolean/literal casing is now handled separately via literalCase.
fn apply_keyword_case(text: &str, style: CaseStyle) -> String {
    match style {
        CaseStyle::Lower => text.to_ascii_lowercase(),
        CaseStyle::Upper => text.to_ascii_uppercase(),
        CaseStyle::Preserve => text.to_string(),
    }
}

// ===========================================================================
// Public entry point
// ===========================================================================

pub fn gen_command(
    cmd: &CommandInvocation,
    source: &str,
    config: &Configuration,
    indent_depth: u32,
) -> PrintItems {
    // Resolve per-command overrides before any formatting decisions
    let raw_name = cmd.name.text(source);
    let effective = config.effective_config_for_command(raw_name);
    let config = &effective;

    let mut items = PrintItems::new();

    // Command name with casing
    let formatted_name = apply_command_case(raw_name, config.command_case);

    // Check if this is an unknown command — preserve original formatting
    // But if customKeywords is set, unknown commands may still have keyword recognition
    let cmd_kind = lookup_command(raw_name);
    if cmd_kind.is_none() && config.custom_keywords.is_empty() {
        return gen_unknown_command(cmd, source, config, &formatted_name);
    }

    items.push_string(formatted_name.clone());

    // Space before paren
    if config.has_space_before_paren(raw_name) {
        items.push_space();
    }
    items.push_str_runtime_width_computed("(");

    // Build and optionally sort argument list
    let mut arguments = build_argument_list(cmd, source, config, cmd_kind.as_ref());
    if should_sort_for_command(raw_name, config, cmd_kind.as_ref()) {
        sort_argument_groups(&mut arguments, config, cmd_kind.as_ref());
    }
    // Sort keyword sections if enabled
    if config.sort_keyword_sections
        && let Some(order) = canonical_section_order(raw_name)
    {
        sort_keyword_sections_by_order(&mut arguments, order);
    }

    // Format arguments
    let prefer_multiline = source[cmd.open_paren.end..cmd.close_paren.start].contains('\n');
    if !arguments.is_empty() {
        let single_line = try_single_line(
            &formatted_name,
            &arguments,
            config,
            indent_depth,
            prefer_multiline,
        );
        if let Some(single) = single_line {
            items.extend(single);
        } else {
            match cmd_kind {
                Some(CommandKind::ConditionSyntax) => {
                    let is_condition_closer = matches!(
                        formatted_name.to_ascii_lowercase().as_str(),
                        "endif" | "endwhile" | "else"
                    );
                    if is_condition_closer {
                        items.extend(gen_condition_closer_multi_line(
                            &arguments,
                            config,
                            indent_depth,
                        ));
                    } else {
                        items.extend(gen_condition_multi_line(
                            &formatted_name,
                            &arguments,
                            config,
                            indent_depth,
                        ));
                    }
                }
                Some(CommandKind::Known(spec)) => {
                    items.extend(gen_known_multi_line(
                        &formatted_name,
                        &arguments,
                        spec,
                        config,
                        indent_depth,
                    ));
                }
                None => {
                    // Unknown command with customKeywords — use empty spec
                    items.extend(gen_known_multi_line(
                        &formatted_name,
                        &arguments,
                        &EMPTY_SPEC,
                        config,
                        indent_depth,
                    ));
                }
            }
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

/// Generate formatting for an unknown command, preserving the original source text.
/// Only the command name gets case-normalized. All content between parens is preserved as-is,
/// except that block-level indentation is adjusted.
fn gen_unknown_command(
    cmd: &CommandInvocation,
    source: &str,
    config: &Configuration,
    formatted_name: &str,
) -> PrintItems {
    let mut items = PrintItems::new();

    items.push_string(formatted_name.to_string());
    if config.has_space_before_paren(cmd.name.text(source)) {
        items.push_space();
    }
    items.push_str_runtime_width_computed("(");

    let content_start = cmd.open_paren.end;
    let content_end = cmd.close_paren.start;
    let raw_content = &source[content_start..content_end];

    if !raw_content.contains('\n') {
        // Single-line: emit content as-is
        if !raw_content.is_empty() {
            items.extend(ir_helpers::gen_from_raw_string(raw_content));
        }
    } else {
        // Multi-line: process per-argument to preserve bracket/quoted content verbatim.
        let cmd_line_start = source[..cmd.name.start]
            .rfind('\n')
            .map(|p| p + 1)
            .unwrap_or(0);
        let base_indent_len = cmd.name.start - cmd_line_start;

        emit_unknown_args_raw(
            &mut items,
            &cmd.arguments,
            source,
            content_start,
            content_end,
            base_indent_len,
        );
    }

    items.push_str_runtime_width_computed(")");

    if let Some(comment_span) = &cmd.trailing_comment {
        items.push_space();
        items.push_string(comment_span.text(source).to_string());
    }

    items
}

fn emit_unknown_args_raw(
    items: &mut PrintItems,
    args: &[Argument],
    source: &str,
    content_start: usize,
    content_end: usize,
    base_indent_len: usize,
) {
    let mut pos = content_start;

    for arg in args {
        let (arg_start, arg_end) = arg_source_range(arg);
        let gap = &source[pos..arg_start];
        if gap.contains('\n') {
            for line in gap.split('\n').skip(1) {
                items.push_signal(Signal::NewLine);
                let stripped = strip_base_indent(line, base_indent_len);
                if !stripped.is_empty() {
                    items.extend(ir_helpers::gen_from_raw_string(stripped));
                }
            }
        } else if !gap.is_empty() {
            items.extend(ir_helpers::gen_from_raw_string(gap));
        }

        let arg_text = &source[arg_start..arg_end];
        items.extend(ir_helpers::gen_from_raw_string(arg_text));
        pos = arg_end;
    }

    // If there are newlines before the closing paren, emit a single newline.
    // The closing paren will be at block indent level (handled by gen_file).
    let trailing = &source[pos..content_end];
    if trailing.contains('\n') {
        items.push_signal(Signal::NewLine);
    }
}

fn arg_source_range(arg: &Argument) -> (usize, usize) {
    match arg {
        Argument::Bracket(span)
        | Argument::Quoted(span)
        | Argument::Unquoted(span)
        | Argument::LineComment(span)
        | Argument::BracketComment(span) => (span.start, span.end),
        Argument::ParenGroup { arguments } => {
            if arguments.is_empty() {
                (0, 0)
            } else {
                let first = arg_source_range(&arguments[0]);
                let last = arg_source_range(arguments.last().unwrap());
                (first.0.saturating_sub(1), last.1 + 1)
            }
        }
    }
}

fn strip_base_indent(line: &str, base_indent_len: usize) -> &str {
    if line.len() >= base_indent_len
        && line.as_bytes()[..base_indent_len]
            .iter()
            .all(|&c| c == b' ' || c == b'\t')
    {
        &line[base_indent_len..]
    } else {
        line
    }
}
// ===========================================================================
// FormattedArg: processed argument ready for formatting
// ===========================================================================

#[derive(Debug, Clone)]
struct FormattedArg {
    text: String,
    is_keyword: bool,
    is_bracket: bool,
    trailing_comment: Option<String>,
    trailing_is_bracket: bool,
    is_paren_group: bool,
    paren_inner: Vec<FormattedArg>,
    blank_line_before: bool,
}

fn build_argument_list(
    cmd: &CommandInvocation,
    source: &str,
    config: &Configuration,
    cmd_kind: Option<&CommandKind>,
) -> Vec<FormattedArg> {
    build_argument_list_from_args(&cmd.arguments, source, config, cmd_kind)
}

fn build_argument_list_from_args(
    args: &[Argument],
    source: &str,
    config: &Configuration,
    cmd_kind: Option<&CommandKind>,
) -> Vec<FormattedArg> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let blank_line_before = if i > 0 {
            let prev_end = arg_source_range(&args[i - 1]).1;
            let (current_start, _) = arg_source_range(&args[i]);
            has_blank_line_between(source, prev_end, current_start)
        } else {
            false
        };

        match &args[i] {
            Argument::Bracket(span) => {
                result.push(FormattedArg {
                    text: span.text(source).to_string(),
                    is_keyword: false,
                    is_bracket: true,
                    trailing_comment: None,
                    trailing_is_bracket: false,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                    blank_line_before,
                });
            }
            Argument::Quoted(span) => {
                result.push(FormattedArg {
                    text: span.text(source).to_string(),
                    is_keyword: false,
                    is_bracket: false,
                    trailing_comment: None,
                    trailing_is_bracket: false,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                    blank_line_before,
                });
            }
            Argument::Unquoted(span) => {
                let text = span.text(source);
                let is_kw = is_known_keyword(text, cmd_kind, config);
                // Store original text — keyword casing is applied during formatting
                // based on command context, not globally
                result.push(FormattedArg {
                    text: text.to_string(),
                    is_keyword: is_kw,
                    is_bracket: false,
                    trailing_comment: None,
                    trailing_is_bracket: false,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                    blank_line_before,
                });
            }
            Argument::ParenGroup { arguments } => {
                let inner = build_argument_list_from_args(arguments, source, config, cmd_kind);
                result.push(FormattedArg {
                    text: String::new(),
                    is_keyword: false,
                    is_bracket: false,
                    trailing_comment: None,
                    trailing_is_bracket: false,
                    is_paren_group: true,
                    paren_inner: inner,
                    blank_line_before,
                });
            }
            Argument::LineComment(span) => {
                let comment_text = span.text(source).to_string();
                // Only attach to previous arg if on the same source line
                let same_line = if i > 0 {
                    let prev_end = arg_source_range(&args[i - 1]).1;
                    !source[prev_end..span.start].contains('\n')
                } else {
                    false
                };
                if same_line
                    && let Some(last) = result.last_mut()
                    && last.trailing_comment.is_none()
                {
                    last.trailing_comment = Some(comment_text);
                    i += 1;
                    continue;
                }
                result.push(FormattedArg {
                    text: comment_text,
                    is_keyword: false,
                    is_bracket: false,
                    trailing_comment: None,
                    trailing_is_bracket: false,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                    blank_line_before,
                });
            }
            Argument::BracketComment(span) => {
                let comment_text = span.text(source).to_string();
                // Only attach to previous arg if on the same source line
                let same_line = if i > 0 {
                    let prev_end = arg_source_range(&args[i - 1]).1;
                    !source[prev_end..span.start].contains('\n')
                } else {
                    false
                };
                if same_line
                    && let Some(last) = result.last_mut()
                    && last.trailing_comment.is_none()
                {
                    last.trailing_comment = Some(comment_text);
                    last.trailing_is_bracket = true;
                    i += 1;
                    continue;
                }
                result.push(FormattedArg {
                    text: comment_text,
                    is_keyword: false,
                    is_bracket: false,
                    trailing_comment: None,
                    trailing_is_bracket: false,
                    is_paren_group: false,
                    paren_inner: Vec::new(),
                    blank_line_before,
                });
            }
        }
        i += 1;
    }

    result
}

// ===========================================================================
// Inline text helpers
// ===========================================================================

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

/// Width of an argument's text (handling paren groups).
fn arg_width(arg: &FormattedArg) -> usize {
    if arg.is_paren_group {
        format_paren_group_inline(&arg.paren_inner).len()
    } else {
        arg.text.len()
    }
}

// ===========================================================================
// Single-line attempt
// ===========================================================================

fn try_single_line(
    cmd_name: &str,
    args: &[FormattedArg],
    config: &Configuration,
    indent_depth: u32,
    prefer_multiline: bool,
) -> Option<PrintItems> {
    // If any argument has a trailing line comment, force multi-line
    if args
        .iter()
        .any(|a| a.trailing_comment.is_some() && !a.trailing_is_bracket)
    {
        return None;
    }

    // If any argument text contains a newline, force multi-line
    if args.iter().any(|a| a.text.contains('\n')) {
        return None;
    }

    // If any argument is a standalone comment (line or bracket), force multi-line.
    // Standalone comments came from separate source lines (not attached as trailing
    // comments) and must not be inlined — line comments would comment out following
    // content, and standalone bracket comments need their own line.
    if args.iter().any(|a| a.text.starts_with('#')) {
        return None;
    }

    if prefer_multiline {
        return None;
    }

    if config.blank_line_between_sections
        && args.iter().filter(|arg| arg.is_keyword).take(2).count() >= 2
    {
        return None;
    }

    // Calculate total width including file-level indentation
    let base_indent = indent_depth as usize * config.indent_width as usize;
    let close_overhead = 1; // ")"
    // Apply keyword/literal casing per §4.4:
    // 1. Keyword tokens get keywordCase
    // 2. Literal tokens (not keywords) get literalCase
    // 3. Other tokens preserved as-is
    let cmd_kind = lookup_command(cmd_name);
    let args_text: Vec<String> = if let Some(ref ck) = cmd_kind {
        match ck {
            CommandKind::Known(spec) => {
                let keyword_positions = compute_keyword_positions(args, spec);
                args.iter()
                    .enumerate()
                    .map(|(i, a)| {
                        let t = arg_inline_text(a);
                        if keyword_positions.contains(&i) {
                            apply_keyword_case(&t, config.keyword_case)
                        } else if !a.is_bracket
                            && !t.starts_with('"')
                            && !t.starts_with('#')
                            && is_literal_token(&t)
                        {
                            apply_literal_case(&t, config.literal_case)
                        } else {
                            t
                        }
                    })
                    .collect()
            }
            CommandKind::ConditionSyntax => args
                .iter()
                .map(|a| {
                    let t = arg_inline_text(a);
                    if a.is_keyword {
                        apply_keyword_case(&t, config.keyword_case)
                    } else if !a.is_bracket
                        && !t.starts_with('"')
                        && !t.starts_with('#')
                        && is_literal_token(&t)
                    {
                        apply_literal_case(&t, config.literal_case)
                    } else {
                        t
                    }
                })
                .collect(),
        }
    } else if !config.custom_keywords.is_empty() {
        // Unknown command with customKeywords: apply keyword/literal casing
        args.iter()
            .map(|a| {
                let t = arg_inline_text(a);
                if a.is_keyword {
                    apply_keyword_case(&t, config.keyword_case)
                } else if !a.is_bracket
                    && !t.starts_with('"')
                    && !t.starts_with('#')
                    && is_literal_token(&t)
                {
                    apply_literal_case(&t, config.literal_case)
                } else {
                    t
                }
            })
            .collect()
    } else {
        // Unknown command: preserve original casing
        args.iter().map(arg_inline_text).collect()
    };
    let args_width: usize = args_text.iter().map(|s| s.len()).sum::<usize>()
        + if args_text.len() > 1 {
            args_text.len() - 1
        } else {
            0
        };
    let total = base_indent + cmd_name.len() + 1 + args_width + close_overhead;

    let keep_condition_header_inline = matches!(
        cmd_name.to_ascii_lowercase().as_str(),
        "if" | "while" | "elseif"
    );
    if !keep_condition_header_inline && total >= config.line_width as usize {
        return None;
    }

    let mut items = PrintItems::new();
    for (i, text) in args_text.iter().enumerate() {
        if i > 0 {
            items.push_space();
        }
        items.extend(ir_helpers::gen_from_raw_string(text));
    }
    Some(items)
}

// ===========================================================================
// Keyword splitting for known commands
// ===========================================================================

/// A group of arguments produced by keyword splitting.
#[derive(Debug)]
enum ArgGroup<'a> {
    /// Positional arguments (not associated with a keyword).
    Positional(Vec<&'a FormattedArg>),
    /// A keyword and its associated values.
    Keyword {
        keyword: &'a FormattedArg,
        values: Vec<&'a FormattedArg>,
    },
    /// A command-line keyword (e.g., `debug`, `optimized`, `general`) with its single value.
    /// These are NOT subject to keyword casing and always keep keyword + value inline.
    CmdLineKeyword {
        keyword: &'a FormattedArg,
        value: Option<&'a FormattedArg>,
    },
}

/// Compute the indices of args that are at keyword positions in a command spec.
/// This is used to apply keyword casing only to actual keywords, not values.
fn compute_keyword_positions(args: &[FormattedArg], spec: &CommandSpec) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut once_keyword_seen = false;
    let mut i = 0;

    // Skip front positional (but mark keyword args for casing)
    let mut pos_count = 0;
    while i < args.len() && pos_count < spec.front_positional {
        if get_keyword_type(&args[i], spec).is_some() || is_section_keyword(&args[i], spec) {
            positions.push(i);
        }
        pos_count += 1;
        i += 1;
    }

    // Scan remaining args for keywords
    while i < args.len() {
        // Skip command-line keywords (not keyword-cased)
        if is_cmd_line_keyword(&args[i], spec) {
            i += if i + 1 < args.len() { 2 } else { 1 };
            continue;
        }

        let kw_type = get_keyword_type(&args[i], spec);
        let is_section = is_section_keyword(&args[i], spec);
        // Skip once_keywords that have already been consumed
        let arg_upper = args[i].text.to_ascii_uppercase();
        let is_once = spec
            .once_keywords
            .iter()
            .any(|&ok| ok.eq_ignore_ascii_case(&arg_upper));
        if kw_type.is_some() && is_once && once_keyword_seen {
            i += 1;
            continue;
        }
        if kw_type.is_some() || is_section {
            if is_once {
                once_keyword_seen = true;
            }
            positions.push(i);
            let kw_type = kw_type.unwrap_or(KwType::MultiValue);
            match kw_type {
                KwType::Option => {
                    i += 1;
                }
                KwType::OneValue => {
                    // Skip intervening Option keywords (record their positions)
                    // then skip the actual value.
                    let kw_idx = i;
                    i += 1;
                    // Skip compound keyword continuation (not a keyword position)
                    if i < args.len()
                        && is_compound_keyword(&args[kw_idx].text, &args[i].text, spec)
                    {
                        i += 1;
                    }
                    while i < args.len() {
                        if let Some(KwType::Option) = get_keyword_type(&args[i], spec) {
                            positions.push(i);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    // Unconditionally skip the actual value
                    if i < args.len() {
                        i += 1;
                    }
                }
                KwType::MultiValue => {
                    let kw_text = &args[i].text;
                    let section_sub_kws = get_section_keywords(kw_text, spec);
                    i += 1;
                    let mut val_count = 0usize;
                    // Skip until next keyword
                    while i < args.len() {
                        let inner_kw = get_keyword_type(&args[i], spec);
                        let inner_upper = args[i].text.to_ascii_uppercase();
                        let inner_is_blocked_once = inner_kw.is_some()
                            && spec
                                .once_keywords
                                .iter()
                                .any(|&ok| ok.eq_ignore_ascii_case(&inner_upper))
                            && once_keyword_seen;
                        if !inner_is_blocked_once
                            && (inner_kw.is_some() || is_section_keyword(&args[i], spec))
                        {
                            // Inside a section: sub-keywords are not top-level keywords
                            if let Some(sub_kws) = section_sub_kws
                                && let Some((_, sub_kw_type)) = sub_kws
                                    .iter()
                                    .find(|(name, _)| name.eq_ignore_ascii_case(&args[i].text))
                            {
                                i += 1;
                                val_count += 1;
                                if matches!(sub_kw_type, KwType::OneValue) && i < args.len() {
                                    i += 1;
                                    val_count += 1;
                                }
                                continue;
                            }
                            // Compound keyword: skip continuation word
                            if val_count == 0 && is_compound_keyword(kw_text, &args[i].text, spec) {
                                i += 1;
                                val_count += 1;
                                continue;
                            }
                            break;
                        }
                        i += 1;
                        val_count += 1;
                    }
                }
                KwType::Group(..) => {
                    // Group keywords only appear in section sub-keywords, never in top-level kw.
                    unreachable!("Group keyword in top-level argument classification");
                }
            }
        } else {
            i += 1;
        }
    }

    positions
}
/// Look up keyword type in a command spec (case-insensitive).
fn get_keyword_type(arg: &FormattedArg, spec: &CommandSpec) -> Option<KwType> {
    if arg.is_paren_group {
        return None;
    }
    // Comments are never keywords
    if arg.text.starts_with('#') {
        return None;
    }
    spec.keywords
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(&arg.text))
        .map(|(_, kt)| *kt)
}

fn has_blank_line_between(source: &str, start: usize, end: usize) -> bool {
    if end <= start {
        return false;
    }

    source[start..end]
        .chars()
        .filter(|&ch| ch == '\n')
        .take(2)
        .count()
        >= 2
}

fn is_custom_keyword(arg: &FormattedArg, config: &Configuration) -> bool {
    if arg.is_paren_group || arg.text.starts_with('#') {
        return false;
    }

    config
        .custom_keywords
        .iter()
        .any(|kw| kw.eq_ignore_ascii_case(&arg.text))
}

fn is_section_keyword(arg: &FormattedArg, spec: &CommandSpec) -> bool {
    if arg.is_paren_group || arg.text.starts_with('#') {
        return false;
    }
    spec.sections
        .iter()
        .any(|(name, _, _)| name.eq_ignore_ascii_case(&arg.text))
}

/// Check if an argument is a command-line keyword (e.g., `debug`, `optimized`, `general`).
fn is_cmd_line_keyword(arg: &FormattedArg, spec: &CommandSpec) -> bool {
    if arg.is_paren_group || arg.text.starts_with('#') {
        return false;
    }
    spec.command_line_keywords
        .iter()
        .any(|&name| name.eq_ignore_ascii_case(&arg.text))
}

/// Get section sub-keywords for a section keyword.
fn get_section_keywords(
    keyword_text: &str,
    spec: &CommandSpec,
) -> Option<&'static [(&'static str, KwType)]> {
    spec.sections
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case(keyword_text))
        .map(|(_, _, kws)| *kws)
}

/// Get the number of front positional arguments for a section keyword.
fn get_section_front_positional(keyword_text: &str, spec: &CommandSpec) -> usize {
    spec.sections
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case(keyword_text))
        .map(|(_, fp, _)| *fp)
        .unwrap_or(0)
}

/// Split arguments into groups based on command spec keywords.
/// Returns (front_positionals, keyword_groups, back_positionals).
fn split_arguments<'a>(
    args: &'a [FormattedArg],
    spec: &CommandSpec,
    config: &Configuration,
) -> (
    Vec<&'a FormattedArg>,
    Vec<ArgGroup<'a>>,
    Vec<&'a FormattedArg>,
) {
    // 0. Extract back positional args from the end of the list
    let back_count = spec.back_positional.min(args.len());
    let main_args = &args[..args.len() - back_count];
    let back_pos: Vec<&'a FormattedArg> = args[args.len() - back_count..].iter().collect();

    let mut front_pos = Vec::new();
    let mut groups: Vec<ArgGroup<'a>> = Vec::new();
    let mut i = 0;

    // 1. Consume front positional args (args before first keyword)
    let mut pos_count = 0;
    while i < main_args.len() && pos_count < spec.front_positional {
        front_pos.push(&main_args[i]);
        pos_count += 1;
        i += 1;
    }

    if i < main_args.len() && spec.keywords.is_empty() && spec.sections.is_empty() {
        let has_keyword_sections = main_args[i..]
            .iter()
            .any(|arg| is_custom_keyword(arg, config) || arg.is_keyword);
        if has_keyword_sections && !main_args[i].is_keyword {
            front_pos.push(&main_args[i]);
            i += 1;
        }
    }

    // 2. Split remaining args by keywords
    let mut current_positional: Vec<&'a FormattedArg> = Vec::new();
    let mut once_keyword_seen = false;
    while i < main_args.len() {
        // Check command-line keywords first (e.g., debug, optimized, general)
        if is_cmd_line_keyword(&main_args[i], spec) {
            if !current_positional.is_empty() {
                groups.push(ArgGroup::Positional(std::mem::take(
                    &mut current_positional,
                )));
            }
            let value = if i + 1 < main_args.len() {
                Some(&main_args[i + 1])
            } else {
                None
            };
            let consumed = 1 + if value.is_some() { 1 } else { 0 };
            groups.push(ArgGroup::CmdLineKeyword {
                keyword: &main_args[i],
                value,
            });
            i += consumed;
            continue;
        }

        let kw_type = get_keyword_type(&main_args[i], spec);
        let is_section = is_section_keyword(&main_args[i], spec);
        let is_custom = is_custom_keyword(&main_args[i], config);

        let is_generic_keyword = main_args[i].is_keyword
            && kw_type.is_none()
            && !is_section
            && !is_custom
            && spec.keywords.is_empty()
            && spec.sections.is_empty();
        // Skip once_keywords that have already been consumed
        let arg_upper = main_args[i].text.to_ascii_uppercase();
        let is_once = spec
            .once_keywords
            .iter()
            .any(|&ok| ok.eq_ignore_ascii_case(&arg_upper));
        if kw_type.is_some() && is_once && once_keyword_seen {
            current_positional.push(&main_args[i]);
            i += 1;
            continue;
        }

        if kw_type.is_some() || is_section || is_custom || is_generic_keyword {
            if is_once {
                once_keyword_seen = true;
            }

            // Flush current positional group
            if !current_positional.is_empty() {
                groups.push(ArgGroup::Positional(std::mem::take(
                    &mut current_positional,
                )));
            }

            let kw_type = kw_type.unwrap_or(KwType::MultiValue);

            match kw_type {
                KwType::Option => {
                    groups.push(ArgGroup::Keyword {
                        keyword: &main_args[i],
                        values: Vec::new(),
                    });
                    i += 1;
                }
                KwType::OneValue => {
                    // OneValue keywords consume the next non-Option arg as their value.
                    // Any intervening Option keywords are absorbed into the values
                    // (e.g., EXTENSION LAST_ONLY <out-var> in cmake_path).
                    // If a compound keyword continuation follows, consume it first.
                    let mut values: Vec<&'a FormattedArg> = Vec::new();
                    let mut j = i + 1;
                    // Consume compound keyword continuation
                    if j < main_args.len()
                        && is_compound_keyword(&main_args[i].text, &main_args[j].text, spec)
                    {
                        values.push(&main_args[j]);
                        j += 1;
                    }
                    // Absorb consecutive Option keywords
                    while j < main_args.len() {
                        if let Some(KwType::Option) = get_keyword_type(&main_args[j], spec) {
                            values.push(&main_args[j]);
                            j += 1;
                        } else {
                            break;
                        }
                    }
                    // Then unconditionally consume the next arg as the value,
                    // even if it looks like a keyword name (e.g., `HELP help`).
                    if j < main_args.len() {
                        values.push(&main_args[j]);
                        j += 1;
                    }
                    groups.push(ArgGroup::Keyword {
                        keyword: &main_args[i],
                        values,
                    });
                    i = j;
                }
                KwType::MultiValue => {
                    let kw = &main_args[i];
                    i += 1;
                    let mut values: Vec<&'a FormattedArg> = Vec::new();
                    // If this keyword is a section, its sub-keywords should be
                    // consumed as values (not treated as top-level keywords).
                    let section_sub_kws = get_section_keywords(&kw.text, spec);
                    while i < main_args.len() {
                        if main_args[i].text.starts_with('#') {
                            let next_index = i + 1;
                            if next_index < main_args.len() {
                                let next_kw = get_keyword_type(&main_args[next_index], spec)
                                    .is_some()
                                    || is_section_keyword(&main_args[next_index], spec)
                                    || is_cmd_line_keyword(&main_args[next_index], spec)
                                    || is_custom_keyword(&main_args[next_index], config)
                                    || (main_args[next_index].is_keyword
                                        && spec.keywords.is_empty()
                                        && spec.sections.is_empty());
                                if next_kw {
                                    break;
                                }
                            }
                        }

                        let inner_kw = get_keyword_type(&main_args[i], spec);
                        let inner_is_section = is_section_keyword(&main_args[i], spec);
                        let inner_is_cmd_line = is_cmd_line_keyword(&main_args[i], spec);
                        let inner_is_custom = is_custom_keyword(&main_args[i], config);
                        let inner_is_generic_keyword = main_args[i].is_keyword
                            && inner_kw.is_none()
                            && !inner_is_section
                            && !inner_is_custom
                            && spec.keywords.is_empty()
                            && spec.sections.is_empty();

                        // Already-seen once_keywords are not treated as keywords
                        let inner_upper = main_args[i].text.to_ascii_uppercase();
                        let inner_is_blocked_once = inner_kw.is_some()
                            && spec
                                .once_keywords
                                .iter()
                                .any(|&ok| ok.eq_ignore_ascii_case(&inner_upper))
                            && once_keyword_seen;

                        if !inner_is_blocked_once
                            && (inner_kw.is_some()
                                || inner_is_section
                                || inner_is_cmd_line
                                || inner_is_custom
                                || inner_is_generic_keyword)
                        {
                            // Inside a section: sub-keywords are consumed as values
                            if let Some(sub_kws) = section_sub_kws
                                && is_text_in_keyword_list(&main_args[i].text, sub_kws)
                            {
                                values.push(&main_args[i]);
                                i += 1;
                                continue;
                            }
                            // Compound keyword: consume continuation word as value
                            if values.is_empty()
                                && is_compound_keyword(&kw.text, &main_args[i].text, spec)
                            {
                                values.push(&main_args[i]);
                                i += 1;
                                continue;
                            }
                            break;
                        }
                        values.push(&main_args[i]);
                        i += 1;
                    }
                    groups.push(ArgGroup::Keyword {
                        keyword: kw,
                        values,
                    });
                }
                KwType::Group(..) => {
                    unreachable!("Group keyword in top-level argument classification");
                }
            }
        } else {
            current_positional.push(&main_args[i]);
            i += 1;
        }
    }

    // Flush remaining positional
    if !current_positional.is_empty() {
        groups.push(ArgGroup::Positional(current_positional));
    }

    (front_pos, groups, back_pos)
}

// ===========================================================================
// Known command multi-line formatting
// ===========================================================================

fn gen_known_multi_line(
    cmd_name: &str,
    arguments: &[FormattedArg],
    spec: &CommandSpec,
    config: &Configuration,
    indent_depth: u32,
) -> PrintItems {
    let (mut front_pos, mut groups, back_pos) = split_arguments(arguments, spec, config);

    // Keep option(<NAME> ... ) style in multiline layout for consistency with fixtures.
    if cmd_name.eq_ignore_ascii_case("option")
        && front_pos.is_empty()
        && let Some(ArgGroup::Positional(args)) = groups.first_mut()
        && let Some(first) = args.first().copied()
    {
        front_pos.push(first);
        args.remove(0);
        if args.is_empty() {
            groups.remove(0);
        }
    }

    // Keep install(TARGETS <name> ...) on the opening line for canonical install layout.
    if cmd_name.eq_ignore_ascii_case("install")
        && front_pos.is_empty()
        && let Some(ArgGroup::Keyword { keyword, values }) = groups.first()
        && keyword.text.eq_ignore_ascii_case("TARGETS")
        && values.len() == 1
        && let Some(first) = values.first().copied()
    {
        front_pos.push(keyword);
        front_pos.push(first);
        groups.remove(0);
    }

    let base_indent = (indent_depth as usize + 1) * config.indent_width as usize;

    let mut inner = PrintItems::new();
    let mut last_on_opening_line = false;

    let mut emitted_keyword_groups = 0usize;
    let opening_line_count = if cmd_name.eq_ignore_ascii_case("install")
        && front_pos.len() >= 2
        && front_pos[0].text.eq_ignore_ascii_case("TARGETS")
    {
        2
    } else {
        1
    };
    // Emit front positional args
    for (idx, arg) in front_pos.iter().enumerate() {
        if idx == 0 {
            // First positional arg always on same line as cmd(
            last_on_opening_line = true;
        } else if idx < opening_line_count {
            inner.push_space();
            last_on_opening_line = true;
        } else {
            inner.push_signal(Signal::NewLine);
            last_on_opening_line = false;
        }
        if arg.is_keyword {
            emit_kw_arg(&mut inner, arg, config);
        } else {
            emit_arg(&mut inner, arg, config);
        }
    }

    // Emit keyword groups
    for group in &groups {
        last_on_opening_line = false;
        match group {
            ArgGroup::Positional(args) => {
                if spec.flow_positional {
                    emit_flow_values(&mut inner, args, config, base_indent, false);
                } else {
                    for arg in args {
                        inner.push_signal(Signal::NewLine);
                        emit_arg(&mut inner, arg, config);
                    }
                }
            }
            ArgGroup::Keyword { keyword, values } => {
                if config.blank_line_between_sections && emitted_keyword_groups > 0 {
                    inner.push_signal(Signal::NewLine);
                }
                emit_keyword_group(
                    &mut inner,
                    keyword,
                    values,
                    spec,
                    config,
                    base_indent,
                    cmd_name.eq_ignore_ascii_case("export"),
                );
                emitted_keyword_groups += 1;
            }
            ArgGroup::CmdLineKeyword { keyword, value } => {
                emit_cmd_line_keyword(&mut inner, keyword, *value, config, base_indent);
            }
        }
    }

    // Emit back positional args
    for arg in &back_pos {
        last_on_opening_line = false;
        inner.push_signal(Signal::NewLine);
        if arg.is_keyword || get_keyword_type(arg, spec).is_some() {
            emit_kw_arg(&mut inner, arg, config);
        } else {
            emit_arg(&mut inner, arg, config);
        }
    }
    // Only add closing paren newline if the last item wasn't on the opening line
    if config.closing_paren_newline && !last_on_opening_line {
        inner.push_signal(Signal::NewLine);
    }

    ir_helpers::with_indent(inner)
}

/// Emit a keyword + values group. Tries inline first, expands if needed.
fn emit_keyword_group(
    items: &mut PrintItems,
    keyword: &FormattedArg,
    values: &[&FormattedArg],
    spec: &CommandSpec,
    config: &Configuration,
    base_indent: usize,
    allow_plain_section_inline: bool,
) {
    // Check for compound keyword (e.g., QUERY WINDOWS_REGISTRY)
    if let Some(first_val) = values.first()
        && is_compound_keyword(&keyword.text, &first_val.text, spec)
    {
        let compound_text = format!(
            "{} {}",
            if keyword.is_keyword {
                apply_keyword_case(&keyword.text, config.keyword_case)
            } else {
                keyword.text.clone()
            },
            if first_val.is_keyword || get_keyword_type(first_val, spec).is_some() {
                apply_keyword_case(&first_val.text, config.keyword_case)
            } else {
                first_val.text.clone()
            },
        );
        let compound_arg = FormattedArg {
            text: compound_text,
            is_keyword: false, // already cased
            is_bracket: false,
            trailing_comment: None,
            trailing_is_bracket: false,
            is_paren_group: false,
            paren_inner: Vec::new(),
            blank_line_before: false,
        };
        emit_keyword_group(
            items,
            &compound_arg,
            &values[1..],
            spec,
            config,
            base_indent,
            allow_plain_section_inline,
        );
        return;
    }

    if values.is_empty() {
        // Option keyword: alone on its line
        items.push_signal(Signal::NewLine);
        emit_kw_arg(items, keyword, config);
        return;
    }

    // Check if this is a section keyword with sub-keywords.
    let section_kws = get_section_keywords(&keyword.text, spec);
    let section_front = get_section_front_positional(&keyword.text, spec);
    let is_section_kw = section_kws.is_some();
    let has_section_tail_values =
        is_section_kw && section_front > 0 && values.len() > section_front;
    // Try inline: keyword + all values on one line.
    let inline_width = compute_keyword_inline_width(keyword, values);
    let can_inline_content = !values.iter().any(|v| v.text.contains('\n'))
        && values
            .iter()
            .all(|v| v.trailing_comment.is_none() || v.trailing_is_bracket)
        && (keyword.trailing_comment.is_none() || keyword.trailing_is_bracket)
        && !values.iter().any(|v| v.text.starts_with('#'));

    let inline_margin = if values.len() > 2 {
        config.indent_width as usize
    } else {
        0
    };

    let force_expanded_section_layout = keyword.is_keyword
        && (config.blank_line_between_sections
            || !matches!(config.sort_arguments, SortArguments::Disabled));
    if !is_section_kw
        && !force_expanded_section_layout
        && base_indent + inline_width + inline_margin <= config.line_width as usize
        && can_inline_content
        && !has_section_tail_values
    {
        items.push_signal(Signal::NewLine);
        let kw_text = if keyword.is_keyword {
            apply_keyword_case(&keyword.text, config.keyword_case)
        } else {
            keyword.text.clone()
        };
        items.push_string(kw_text);
        for val in values {
            items.push_space();
            let val_text = arg_inline_text(val);
            items.extend(ir_helpers::gen_from_raw_string(&val_text));
        }
        if let Some(comment) = &keyword.trailing_comment {
            items.push_space();
            items.extend(ir_helpers::gen_from_raw_string(comment));
        }
        return;
    }

    // Step 2: keyword on its own line, all values on one line at deeper indent.
    let sub_indent = base_indent + config.indent_width as usize;
    let values_only_width: usize =
        values.iter().map(|v| arg_width(v)).sum::<usize>() + values.len().saturating_sub(1);
    if !is_section_kw
        && !force_expanded_section_layout
        && sub_indent + values_only_width <= config.line_width as usize
        && can_inline_content
        && values.len() >= 2
        && values.len() <= 2
        && !has_section_tail_values
    {
        items.push_signal(Signal::NewLine);
        emit_kw_arg(items, keyword, config);
        let mut val_items = PrintItems::new();
        val_items.push_signal(Signal::NewLine);
        for (vi, val) in values.iter().enumerate() {
            if vi > 0 {
                val_items.push_space();
            }
            let val_text = arg_inline_text(val);
            val_items.extend(ir_helpers::gen_from_raw_string(&val_text));
        }
        items.extend(ir_helpers::with_indent(val_items));
        return;
    }

    // Expanded: keyword on its own line, values indented below
    items.push_signal(Signal::NewLine);

    if let Some(sub_kws) = section_kws {
        let has_subkeyword_values = values
            .iter()
            .any(|v| is_text_in_keyword_list(&v.text, sub_kws));

        // Export-style sections with plain values (no sub-keyword usage) can stay inline.
        if allow_plain_section_inline
            && !has_subkeyword_values
            && base_indent + inline_width + inline_margin <= config.line_width as usize
            && can_inline_content
        {
            let kw_text = if keyword.is_keyword {
                apply_keyword_case(&keyword.text, config.keyword_case)
            } else {
                keyword.text.clone()
            };
            items.push_string(kw_text);
            for val in values {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
            }
            if let Some(comment) = &keyword.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
            return;
        }

        // Section keyword with possible front positionals to keep inline.
        let sec_front = section_front;
        let leading_count = sec_front.min(values.len());
        if leading_count > 0 {
            let leading_width = keyword.text.len()
                + values[..leading_count]
                    .iter()
                    .map(|v| 1 + arg_width(v))
                    .sum::<usize>();
            let can_inline_leading = base_indent + leading_width <= config.line_width as usize
                && !values[..leading_count]
                    .iter()
                    .any(|v| v.text.contains('\n'));
            if can_inline_leading {
                let kw_text = if keyword.is_keyword {
                    apply_keyword_case(&keyword.text, config.keyword_case)
                } else {
                    keyword.text.clone()
                };
                items.push_string(kw_text);
                for val in &values[..leading_count] {
                    items.push_space();
                    items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
                }
                let wrap_section_tail = values[leading_count..]
                    .iter()
                    .any(|v| v.text.contains('\n'));
                emit_section_values_inner(
                    items,
                    &values[leading_count..],
                    sub_kws,
                    config,
                    base_indent,
                    wrap_section_tail,
                );
                return;
            }
        }
        emit_kw_arg(items, keyword, config);
        emit_section_values(items, values, sub_kws, config, base_indent);
    } else {
        emit_kw_arg(items, keyword, config);
        if is_property_keyword(&keyword.text, spec) {
            // Property keyword: first value is the property name, rest are its values
            emit_property_values(items, values, config, base_indent);
        } else if is_pair_keyword(&keyword.text, spec) {
            // Pair keyword: values are alternating key-value pairs
            emit_pair_values(items, values, config, base_indent);
        } else if is_flow_keyword(&keyword.text, spec) {
            // Flow keyword: values flow-wrap at line width
            emit_flow_values(items, values, config, base_indent, true);
        } else {
            // Regular keyword: values at next indent level.
            // Trailing standalone comments (after the last non-comment value)
            // are emitted at keyword indent level, not value indent level.
            let trailing_comment_start = values
                .iter()
                .rposition(|v| !v.text.starts_with('#'))
                .map(|i| i + 1)
                .unwrap_or(0);
            let regular_values = &values[..trailing_comment_start];
            let trailing_comments = &values[trailing_comment_start..];

            // Detect and format genex groups among the values.
            emit_values_with_genex(items, regular_values, config);

            for comment in trailing_comments {
                items.push_signal(Signal::NewLine);
                emit_arg(items, comment, config);
            }
        }
    }
}

/// Check if a keyword name is in the command spec's pair_keywords list.
fn is_pair_keyword(keyword_text: &str, spec: &CommandSpec) -> bool {
    spec.pair_keywords
        .iter()
        .any(|&pk| pk.eq_ignore_ascii_case(keyword_text))
}

/// Check if a keyword name is in the command spec's property_keywords list.
fn is_property_keyword(keyword_text: &str, spec: &CommandSpec) -> bool {
    spec.property_keywords
        .iter()
        .any(|&pk| pk.eq_ignore_ascii_case(keyword_text))
}

/// Emit values for a property keyword.
///
/// Layout: first value is the property name (L2, one indent from keyword),
/// remaining values are property values (L3, two indents from keyword).
/// When there is exactly one value, tries to inline: `PROP_NAME VALUE` at L2.
fn emit_property_values(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    config: &Configuration,
    base_indent: usize,
) {
    if values.is_empty() {
        return;
    }

    let prop_indent = base_indent + config.indent_width as usize;
    let prop_name = values[0];
    let prop_values = &values[1..];

    // Filter out standalone comments from prop_values to find real values
    let real_value_count = prop_values
        .iter()
        .filter(|v| !v.text.starts_with('#'))
        .count();

    if real_value_count <= 1 {
        // Try inline: PROP_NAME VALUE at L2 (only if single real value)
        let single_val = prop_values.iter().find(|v| !v.text.starts_with('#'));
        if let Some(&val) = single_val {
            let inline_width = arg_width(prop_name) + 1 + arg_width(val);
            let name_has_line_comment =
                prop_name.trailing_comment.is_some() && !prop_name.trailing_is_bracket;
            let val_has_line_comment = val.trailing_comment.is_some() && !val.trailing_is_bracket;
            let can_inline = prop_indent + inline_width <= config.line_width as usize
                && !prop_name.text.contains('\n')
                && !val.text.contains('\n')
                && !name_has_line_comment
                && !val_has_line_comment;

            if can_inline {
                let mut val_items = PrintItems::new();
                val_items.push_signal(Signal::NewLine);
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(prop_name)));
                val_items.push_space();
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
                if let Some(comment) = &prop_name.trailing_comment {
                    val_items.push_space();
                    val_items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                if let Some(comment) = &val.trailing_comment {
                    val_items.push_signal(Signal::NewLine);
                    val_items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                // Emit any standalone comments in prop_values
                for pv in prop_values {
                    if pv.text.starts_with('#') {
                        val_items.push_signal(Signal::NewLine);
                        val_items.extend(ir_helpers::gen_from_raw_string(&pv.text));
                    }
                }
                items.extend(ir_helpers::with_indent(val_items));
                return;
            }
        } else if real_value_count == 0 {
            // Only property name, no values
            let mut val_items = PrintItems::new();
            val_items.push_signal(Signal::NewLine);
            emit_arg(&mut val_items, prop_name, config);
            // Emit trailing comments at L2
            for pv in prop_values {
                if pv.text.starts_with('#') {
                    val_items.push_signal(Signal::NewLine);
                    val_items.extend(ir_helpers::gen_from_raw_string(&pv.text));
                }
            }
            items.extend(ir_helpers::with_indent(val_items));
            return;
        }
    }

    // Expanded: property name at L2, all values at L3
    let mut val_items = PrintItems::new();
    val_items.push_signal(Signal::NewLine);
    emit_arg(&mut val_items, prop_name, config);

    let mut sub = PrintItems::new();
    for pv in prop_values {
        sub.push_signal(Signal::NewLine);
        emit_arg(&mut sub, pv, config);
    }
    val_items.extend(ir_helpers::with_indent(sub));

    items.extend(ir_helpers::with_indent(val_items));
}

/// Check if a keyword followed by a value forms a compound keyword.
fn is_compound_keyword(keyword_text: &str, value_text: &str, spec: &CommandSpec) -> bool {
    spec.compound_keywords.iter().any(|&(kw, continuation)| {
        kw.eq_ignore_ascii_case(keyword_text) && continuation.eq_ignore_ascii_case(value_text)
    })
}

/// Check if a keyword name is in the command spec's flow_keywords list.
fn is_flow_keyword(keyword_text: &str, spec: &CommandSpec) -> bool {
    spec.flow_keywords
        .iter()
        .any(|&fk| fk.eq_ignore_ascii_case(keyword_text))
}

/// Emit values using flow layout: pack values onto lines, wrapping at line width.
/// Multiline strings and line comments force line breaks.
fn emit_flow_values(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    config: &Configuration,
    base_indent: usize,
    indent: bool,
) {
    let flow_indent = base_indent + config.indent_width as usize;
    let mut val_items = PrintItems::new();
    let mut current_line_width = flow_indent;
    let mut line_started = false;

    for val in values {
        let is_comment = val.text.starts_with('#');
        let has_newline = val.text.contains('\n');
        let val_text = arg_inline_text(val);
        let val_width = val_text.len();

        // Comments always go on their own line
        if is_comment {
            val_items.push_signal(Signal::NewLine);
            val_items.extend(ir_helpers::gen_from_raw_string(&val_text));
            current_line_width = flow_indent;
            line_started = false;
            continue;
        }

        // Multiline values always start on a new line
        if has_newline && line_started {
            current_line_width = flow_indent;
            line_started = false;
        }

        // Check if this value fits on the current line
        let needed = if line_started {
            1 + val_width
        } else {
            val_width
        };
        if line_started && current_line_width + needed > config.line_width as usize {
            // Wrap to next line
            current_line_width = flow_indent;
            line_started = false;
        }

        if !line_started {
            val_items.push_signal(Signal::NewLine);
            line_started = true;
        } else {
            val_items.push_space();
        }

        if has_newline {
            // Multiline strings: emit first line normally (gets indented),
            // then remaining lines with StartIgnoringIndent (verbatim).
            let raw = &val.text;
            if let Some(first_nl) = raw.find('\n') {
                let first_line = &raw[..first_nl];
                let rest = &raw[first_nl + 1..];
                val_items.push_string(first_line.to_string());
                val_items.push_signal(Signal::StartIgnoringIndent);
                for line in rest.lines() {
                    val_items.push_signal(Signal::NewLine);
                    if !line.is_empty() {
                        val_items.push_string(line.to_string());
                    }
                }
                if rest.ends_with('\n') {
                    val_items.push_signal(Signal::NewLine);
                }
                val_items.push_signal(Signal::FinishIgnoringIndent);
            } else {
                val_items.extend(ir_helpers::gen_from_raw_string(raw));
            }
            current_line_width = flow_indent;
            line_started = false;
        } else {
            val_items.extend(ir_helpers::gen_from_raw_string(&val_text));
            current_line_width += needed;
            // Emit trailing comment if present
            if let Some(comment) = &val.trailing_comment {
                if val.trailing_is_bracket {
                    // Bracket comments stay inline
                    val_items.push_space();
                    val_items.extend(ir_helpers::gen_from_raw_string(comment));
                } else {
                    // Line comments go on their own line
                    val_items.push_signal(Signal::NewLine);
                    val_items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                current_line_width = flow_indent;
                line_started = false;
            }
        }
    }

    if indent {
        items.extend(ir_helpers::with_indent(val_items));
    } else {
        items.extend(val_items);
    }
}

/// Emit values as alternating key-value pairs for pair keywords (e.g., PROPERTIES).
///
/// Layout at indent levels relative to the keyword (already emitted):
/// - L2: pair keys (one `with_indent` from keyword)
/// - L3: pair values when expanded (two `with_indent`s from keyword)
fn emit_pair_values(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    config: &Configuration,
    base_indent: usize,
) {
    let pair_indent = base_indent + config.indent_width as usize;

    let mut val_items = PrintItems::new();
    let mut i = 0;

    while i < values.len() {
        let key = values[i];

        // Standalone comment (previous arg already had a trailing comment): emit at L2
        if key.text.starts_with('#') {
            val_items.push_signal(Signal::NewLine);
            emit_arg(&mut val_items, key, config);
            i += 1;
            continue;
        }

        // Look ahead for the value of this pair, skipping intervening comments
        let mut val_idx = i + 1;
        while val_idx < values.len() && values[val_idx].text.starts_with('#') {
            val_idx += 1;
        }
        let intervening_comments = if val_idx > i + 1 {
            &values[i + 1..val_idx]
        } else {
            &[][..]
        };
        let value = if val_idx < values.len() {
            Some(values[val_idx])
        } else {
            None
        };

        if let Some(val) = value {
            let has_intervening_comments = !intervening_comments.is_empty();
            let key_has_line_comment = key.trailing_comment.is_some() && !key.trailing_is_bracket;
            let val_has_line_comment = val.trailing_comment.is_some() && !val.trailing_is_bracket;

            // Try inline: KEY VALUE at L2 (only if no intervening comments)
            let inline_width = arg_width(key) + 1 + arg_width(val);
            let can_inline = pair_indent + inline_width <= config.line_width as usize
                && !key.text.contains('\n')
                && !val.text.contains('\n')
                && !key_has_line_comment
                && !val_has_line_comment
                && !has_intervening_comments;

            if can_inline {
                val_items.push_signal(Signal::NewLine);
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(key)));
                val_items.push_space();
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
                // Key's bracket comment (if any) goes inline
                if let Some(comment) = &key.trailing_comment {
                    val_items.push_space();
                    val_items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                // Value's trailing comment goes on its own line at L2
                if let Some(comment) = &val.trailing_comment {
                    val_items.push_signal(Signal::NewLine);
                    val_items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                i = val_idx + 1;
                continue;
            }

            // Expanded: KEY at L2, intervening comments + VALUE at L3
            if key_has_line_comment && val_has_line_comment && !has_intervening_comments {
                // Both have line comments: special layout
                // KEY alone at L2, key-comment at L3, value at L3, val-comment at L2
                val_items.push_signal(Signal::NewLine);
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(key)));
                let mut sub = PrintItems::new();
                sub.push_signal(Signal::NewLine);
                sub.extend(ir_helpers::gen_from_raw_string(
                    key.trailing_comment.as_ref().unwrap(),
                ));
                sub.push_signal(Signal::NewLine);
                sub.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
                val_items.extend(ir_helpers::with_indent(sub));
                // Value's trailing comment at L2
                val_items.push_signal(Signal::NewLine);
                val_items.extend(ir_helpers::gen_from_raw_string(
                    val.trailing_comment.as_ref().unwrap(),
                ));
            } else {
                // Key at L2 (with trailing comment inline if present)
                val_items.push_signal(Signal::NewLine);
                emit_arg(&mut val_items, key, config);
                // Intervening comments + value at L3
                let mut sub = PrintItems::new();
                for comment in intervening_comments {
                    sub.push_signal(Signal::NewLine);
                    emit_arg(&mut sub, comment, config);
                }
                sub.push_signal(Signal::NewLine);
                emit_arg(&mut sub, val, config);
                val_items.extend(ir_helpers::with_indent(sub));
            }
            i = val_idx + 1;
        } else {
            // Odd value (no pair partner): emit key alone at L2,
            // then intervening comments at L2
            val_items.push_signal(Signal::NewLine);
            emit_arg(&mut val_items, key, config);
            for comment in intervening_comments {
                val_items.push_signal(Signal::NewLine);
                emit_arg(&mut val_items, comment, config);
            }
            i = val_idx;
        }
    }

    items.extend(ir_helpers::with_indent(val_items));
}

/// Emit a command-line keyword (e.g., `debug`, `optimized`, `general`) with its value.
/// No keyword casing is applied. Tries inline (keyword + value on one line),
/// falls back to keyword on its own line with value indented below.
fn emit_cmd_line_keyword(
    items: &mut PrintItems,
    keyword: &FormattedArg,
    value: Option<&FormattedArg>,
    config: &Configuration,
    base_indent: usize,
) {
    if let Some(val) = value {
        let inline_width = keyword.text.len() + 1 + arg_width(val);
        if base_indent + inline_width <= config.line_width as usize && !val.text.contains('\n') {
            items.push_signal(Signal::NewLine);
            items.extend(ir_helpers::gen_from_raw_string(&keyword.text));
            items.push_space();
            items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
            if let Some(comment) = &val.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
            return;
        }
        // Expanded: keyword on its own line, value indented below
        items.push_signal(Signal::NewLine);
        items.extend(ir_helpers::gen_from_raw_string(&keyword.text));
        let mut val_items = PrintItems::new();
        val_items.push_signal(Signal::NewLine);
        emit_arg(&mut val_items, val, config);
        items.extend(ir_helpers::with_indent(val_items));
    } else {
        items.push_signal(Signal::NewLine);
        items.extend(ir_helpers::gen_from_raw_string(&keyword.text));
    }
}

/// Emit values within a section that has its own sub-keyword definitions.
fn emit_section_values(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    sub_keywords: &[(&str, KwType)],
    config: &Configuration,
    base_indent: usize,
) {
    emit_section_values_inner(items, values, sub_keywords, config, base_indent, true);
}

fn emit_section_values_inner(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    sub_keywords: &[(&str, KwType)],
    config: &Configuration,
    base_indent: usize,
    wrap_indent: bool,
) {
    let sub_indent = base_indent + config.indent_width as usize;
    let mut val_items = PrintItems::new();
    let mut i = 0;

    while i < values.len() {
        if values[i].blank_line_before {
            val_items.push_signal(Signal::NewLine);
        }

        // Check if this value is a sub-keyword
        let sub_kw_type = sub_keywords
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(&values[i].text))
            .map(|(_, kt)| *kt);

        if let Some(kt) = sub_kw_type {
            match kt {
                KwType::Option => {
                    val_items.push_signal(Signal::NewLine);
                    emit_sub_kw_arg(&mut val_items, values[i], config);
                    i += 1;
                }
                KwType::OneValue => {
                    let sub_val = if i + 1 < values.len() {
                        Some(values[i + 1])
                    } else {
                        None
                    };
                    // Try inline
                    if let Some(sv) = sub_val {
                        let iw = values[i].text.len() + 1 + arg_width(sv);
                        if sub_indent + iw <= config.line_width as usize && !sv.text.contains('\n')
                        {
                            val_items.push_signal(Signal::NewLine);
                            val_items.push_string(apply_keyword_case(
                                &values[i].text,
                                config.keyword_case,
                            ));
                            val_items.push_space();
                            let sv_text = arg_inline_text(sv);
                            val_items.extend(ir_helpers::gen_from_raw_string(&sv_text));
                            if let Some(comment) = &values[i].trailing_comment {
                                val_items.push_space();
                                val_items.extend(ir_helpers::gen_from_raw_string(comment));
                            }
                            i += 2;
                            continue;
                        }
                    }
                    // Expanded
                    val_items.push_signal(Signal::NewLine);
                    emit_sub_kw_arg(&mut val_items, values[i], config);
                    if let Some(sv) = sub_val {
                        let mut sub_items = PrintItems::new();
                        sub_items.push_signal(Signal::NewLine);
                        emit_arg(&mut sub_items, sv, config);
                        val_items.extend(ir_helpers::with_indent(sub_items));
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                KwType::MultiValue => {
                    let kw = values[i];
                    i += 1;
                    let mut sub_values: Vec<&FormattedArg> = Vec::new();
                    while i < values.len() {
                        let is_sub_kw = sub_keywords
                            .iter()
                            .any(|(name, _)| name.eq_ignore_ascii_case(&values[i].text));
                        if is_sub_kw {
                            break;
                        }
                        sub_values.push(values[i]);
                        i += 1;
                    }
                    // Try inline
                    let iw =
                        kw.text.len() + sub_values.iter().map(|v| 1 + arg_width(v)).sum::<usize>();
                    if sub_indent + iw <= config.line_width as usize
                        && !sub_values.iter().any(|v| v.text.contains('\n'))
                        && sub_values.len() == 1
                        && sub_values
                            .iter()
                            .all(|v| v.trailing_comment.is_none() || v.trailing_is_bracket)
                        && (kw.trailing_comment.is_none() || kw.trailing_is_bracket)
                        && !sub_values.iter().any(|v| v.text.starts_with('#'))
                    {
                        val_items.push_signal(Signal::NewLine);
                        let sub_kw_text = apply_keyword_case(&kw.text, config.keyword_case);
                        val_items.push_string(sub_kw_text);
                        for sv in &sub_values {
                            val_items.push_space();
                            val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(sv)));
                        }
                        if let Some(comment) = &kw.trailing_comment {
                            val_items.push_space();
                            val_items.extend(ir_helpers::gen_from_raw_string(comment));
                        }
                    } else if sub_values.is_empty() {
                        val_items.push_signal(Signal::NewLine);
                        emit_sub_kw_arg(&mut val_items, kw, config);
                    } else {
                        val_items.push_signal(Signal::NewLine);
                        emit_sub_kw_arg(&mut val_items, kw, config);
                        let mut sub_items = PrintItems::new();
                        for sv in &sub_values {
                            sub_items.push_signal(Signal::NewLine);
                            emit_arg(&mut sub_items, sv, config);
                        }
                        val_items.extend(ir_helpers::with_indent(sub_items));
                    }
                }
                KwType::Group(front_count, group_sub_kws) => {
                    let kw = values[i];
                    i += 1;

                    // Collect all values belonging to this group
                    // (until next keyword at the current sub_keywords level).
                    let group_start = i;
                    while i < values.len() {
                        let is_outer_kw = sub_keywords
                            .iter()
                            .any(|(name, _)| name.eq_ignore_ascii_case(&values[i].text));
                        if is_outer_kw {
                            break;
                        }
                        i += 1;
                    }
                    let all_group_values = &values[group_start..i];
                    let leading_count = front_count.min(all_group_values.len());
                    let front_positionals = &all_group_values[..leading_count];
                    let rest_values = &all_group_values[leading_count..];

                    // Try full inline: keyword + all values on one line
                    let total_iw = kw.text.len()
                        + all_group_values
                            .iter()
                            .map(|v| 1 + arg_width(v))
                            .sum::<usize>();
                    let all_inlineable = !all_group_values.iter().any(|v| v.text.contains('\n'))
                        && all_group_values
                            .iter()
                            .all(|v| v.trailing_comment.is_none() || v.trailing_is_bracket)
                        && (kw.trailing_comment.is_none() || kw.trailing_is_bracket)
                        && !all_group_values.iter().any(|v| v.text.starts_with('#'));

                    if sub_indent + total_iw <= config.line_width as usize && all_inlineable {
                        // Full inline
                        val_items.push_signal(Signal::NewLine);
                        val_items.push_string(apply_keyword_case(&kw.text, config.keyword_case));
                        for v in all_group_values {
                            val_items.push_space();
                            let vt = if v.is_keyword {
                                apply_keyword_case(&arg_inline_text(v), config.keyword_case)
                            } else {
                                arg_inline_text(v)
                            };
                            val_items.extend(ir_helpers::gen_from_raw_string(&vt));
                        }
                        if let Some(comment) = &kw.trailing_comment {
                            val_items.push_space();
                            val_items.extend(ir_helpers::gen_from_raw_string(comment));
                        }
                    } else {
                        // Try to inline keyword + front positionals only
                        let front_iw = kw.text.len()
                            + front_positionals
                                .iter()
                                .map(|v| 1 + arg_width(v))
                                .sum::<usize>();
                        let can_inline_front = !front_positionals.is_empty()
                            && sub_indent + front_iw <= config.line_width as usize
                            && !front_positionals.iter().any(|v| v.text.contains('\n'))
                            && front_positionals
                                .iter()
                                .all(|v| v.trailing_comment.is_none() || v.trailing_is_bracket)
                            && (kw.trailing_comment.is_none() || kw.trailing_is_bracket)
                            && !front_positionals.iter().any(|v| v.text.starts_with('#'));

                        if can_inline_front {
                            // Keyword + front positionals inline, rest indented below
                            val_items.push_signal(Signal::NewLine);
                            val_items
                                .push_string(apply_keyword_case(&kw.text, config.keyword_case));
                            for v in front_positionals {
                                val_items.push_space();
                                val_items
                                    .extend(ir_helpers::gen_from_raw_string(&arg_inline_text(v)));
                            }
                            if let Some(comment) = &kw.trailing_comment {
                                val_items.push_space();
                                val_items.extend(ir_helpers::gen_from_raw_string(comment));
                            }
                            emit_section_values_inner(
                                &mut val_items,
                                rest_values,
                                group_sub_kws,
                                config,
                                sub_indent,
                                true,
                            );
                        } else {
                            // Keyword alone on its line, everything else indented
                            val_items.push_signal(Signal::NewLine);
                            emit_sub_kw_arg(&mut val_items, kw, config);
                            emit_section_values_inner(
                                &mut val_items,
                                all_group_values,
                                group_sub_kws,
                                config,
                                sub_indent,
                                true,
                            );
                        }
                    }
                }
            }
        } else {
            // Regular value (not a sub-keyword)
            val_items.push_signal(Signal::NewLine);
            emit_arg(&mut val_items, values[i], config);
            i += 1;
        }
    }

    if wrap_indent {
        items.extend(ir_helpers::with_indent(val_items));
    } else {
        items.extend(val_items);
    }
}

fn is_text_in_keyword_list(text: &str, kws: &[(&str, KwType)]) -> bool {
    kws.iter().any(|(name, kt)| {
        if name.eq_ignore_ascii_case(text) {
            return true;
        }
        // Also check inside Group sub-keywords
        if let KwType::Group(_, sub_kws) = kt {
            return is_text_in_keyword_list(text, sub_kws);
        }
        false
    })
}

fn compute_keyword_inline_width(keyword: &FormattedArg, values: &[&FormattedArg]) -> usize {
    let mut w = keyword.text.len();
    for val in values {
        w += 1 + arg_width(val); // space + value
    }
    if let Some(comment) = &keyword.trailing_comment {
        w += 1 + comment.len();
    }
    w
}
// ===========================================================================
fn gen_condition_closer_multi_line(
    args: &[FormattedArg],
    config: &Configuration,
    indent_depth: u32,
) -> PrintItems {
    let base_indent = (indent_depth as usize + 1) * config.indent_width as usize;
    let mut inner = PrintItems::new();

    let mut tokens: Vec<String> = Vec::with_capacity(args.len());
    for arg in args {
        let raw = arg_inline_text(arg);
        let token = if arg.is_keyword {
            apply_keyword_case(&raw, config.keyword_case)
        } else if !arg.is_bracket
            && !raw.starts_with('"')
            && !raw.starts_with('#')
            && is_literal_token(&raw)
        {
            apply_literal_case(&raw, config.literal_case)
        } else {
            raw
        };
        tokens.push(token);
    }

    if let Some(first) = tokens.first() {
        inner.extend(ir_helpers::gen_from_raw_string(first));
        let mut current_width = base_indent + first.len();
        let mut i = 1;

        while i < tokens.len() {
            let token = &tokens[i];

            if is_logical_op(token) && i + 1 < tokens.len() {
                let next = &tokens[i + 1];
                let needed = 1 + token.len() + 1 + next.len();
                if current_width + needed > config.line_width as usize {
                    inner.push_signal(Signal::NewLine);
                    inner.extend(ir_helpers::gen_from_raw_string(token));
                    inner.push_space();
                    inner.extend(ir_helpers::gen_from_raw_string(next));
                    current_width = base_indent + token.len() + 1 + next.len();
                    i += 2;
                    continue;
                }
            }

            let needed = 1 + token.len();
            if current_width + needed > config.line_width as usize {
                inner.push_signal(Signal::NewLine);
                inner.extend(ir_helpers::gen_from_raw_string(token));
                current_width = base_indent + token.len();
            } else {
                inner.push_space();
                inner.extend(ir_helpers::gen_from_raw_string(token));
                current_width += needed;
            }
            i += 1;
        }
    }

    if config.closing_paren_newline {
        inner.push_signal(Signal::NewLine);
    }

    ir_helpers::with_indent(inner)
}

// ===========================================================================
// Condition syntax multi-line formatting (for if/while/elseif)
// ===========================================================================
// Condition syntax multi-line formatting (for if/while/elseif)
// ===========================================================================

/// Condition-specific unary test operators (high priority).
const CONDITION_UNARY_TEST_OPS: &[&str] = &[
    "COMMAND",
    "POLICY",
    "TARGET",
    "TEST",
    "EXISTS",
    "IS_DIRECTORY",
    "IS_SYMLINK",
    "IS_ABSOLUTE",
    "DEFINED",
    "IS_READABLE",
    "IS_WRITABLE",
    "IS_EXECUTABLE",
];

/// Condition binary operators.
const CONDITION_BINARY_OPS: &[&str] = &[
    "IS_NEWER_THAN",
    "MATCHES",
    "LESS",
    "GREATER",
    "EQUAL",
    "LESS_EQUAL",
    "GREATER_EQUAL",
    "STRLESS",
    "STRGREATER",
    "STREQUAL",
    "STRLESS_EQUAL",
    "STRGREATER_EQUAL",
    "VERSION_LESS",
    "VERSION_GREATER",
    "VERSION_EQUAL",
    "VERSION_LESS_EQUAL",
    "VERSION_GREATER_EQUAL",
    "IN_LIST",
    "PATH_EQUAL",
];

fn is_condition_unary_test(text: &str) -> bool {
    CONDITION_UNARY_TEST_OPS
        .iter()
        .any(|&op| op.eq_ignore_ascii_case(text))
}

fn is_condition_binary_op(text: &str) -> bool {
    CONDITION_BINARY_OPS
        .iter()
        .any(|&op| op.eq_ignore_ascii_case(text))
}

fn is_logical_op(text: &str) -> bool {
    text.eq_ignore_ascii_case("AND") || text.eq_ignore_ascii_case("OR")
}

fn is_not_op(text: &str) -> bool {
    text.eq_ignore_ascii_case("NOT")
}

/// Condition expression tree node.
#[derive(Debug)]
enum CondExpr<'a> {
    /// Simple atom (unquoted, quoted, bracket, etc.)
    Atom(&'a FormattedArg),
    /// Parenthesized group: (expr...)
    ParenGroup(&'a FormattedArg),
    /// Unary operation: OP expr
    Unary {
        op: &'a FormattedArg,
        operand: Box<CondExpr<'a>>,
    },
    /// Binary operation: lhs OP rhs
    Binary {
        lhs: Box<CondExpr<'a>>,
        op: &'a FormattedArg,
        rhs: Box<CondExpr<'a>>,
    },
}

/// A top-level item in a condition expression.
/// After isolating AND/OR as unary operators, the top-level is a list of items
/// where the first is a standalone expression and subsequent ones are AND/OR-prefixed.
#[derive(Debug)]
enum CondItem<'a> {
    /// Standalone expression (first item or after grouping).
    Expr(CondExpr<'a>),
    /// Logical operator + expression: AND expr, OR expr.
    /// Comments between the operator and expression are stored in `comments`.
    LogicalOp {
        op: &'a FormattedArg,
        comments: Vec<&'a FormattedArg>,
        expr: CondExpr<'a>,
    },
    /// Standalone comment (line comment not attached to any expression).
    Comment(&'a FormattedArg),
}

/// Parse a flat argument list into condition items.
fn parse_condition_items<'a>(args: &'a [FormattedArg]) -> Vec<CondItem<'a>> {
    let mut items: Vec<CondItem<'a>> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        // Standalone line comments (text starts with '#', not bracket '#[')
        if args[i].text.starts_with('#') && !args[i].text.starts_with("#[") {
            items.push(CondItem::Comment(&args[i]));
            i += 1;
            continue;
        }

        if i > 0 && is_logical_op(&args[i].text) {
            // AND/OR: consume operator, collect standalone comments, then expression
            let op = &args[i];
            i += 1;
            let mut comments: Vec<&'a FormattedArg> = Vec::new();
            while i < args.len() && args[i].text.starts_with('#') && !args[i].text.starts_with("#[")
            {
                comments.push(&args[i]);
                i += 1;
            }
            if i < args.len() {
                let (expr, consumed) = parse_one_expression(&args[i..]);
                items.push(CondItem::LogicalOp { op, comments, expr });
                i += consumed;
            } else {
                // Operator at end with no expression — emit as atom
                for c in &comments {
                    items.push(CondItem::Comment(c));
                }
                items.push(CondItem::Expr(CondExpr::Atom(op)));
            }
        } else {
            // First item or standalone expression
            let (expr, consumed) = parse_one_expression(&args[i..]);
            items.push(CondItem::Expr(expr));
            i += consumed;
        }
    }

    items
}

/// Parse one expression from the front of the argument list.
/// Returns (expression, number of args consumed).
fn parse_one_expression<'a>(args: &'a [FormattedArg]) -> (CondExpr<'a>, usize) {
    // NOT operator
    if args.is_empty() {
        // Should not happen in well-formed input; panic is appropriate
        panic!("parse_one_expression called with empty args");
    }

    // NOT operator
    if is_not_op(&args[0].text) && args.len() > 1 {
        let (operand, consumed) = parse_one_expression(&args[1..]);
        return (
            CondExpr::Unary {
                op: &args[0],
                operand: Box::new(operand),
            },
            1 + consumed,
        );
    }

    // Unary test operators (COMMAND, POLICY, etc.)
    if is_condition_unary_test(&args[0].text) && args.len() > 1 && !args[1].is_paren_group {
        return (
            CondExpr::Unary {
                op: &args[0],
                operand: Box::new(CondExpr::Atom(&args[1])),
            },
            2,
        );
    }

    // Parenthesized group
    if args[0].is_paren_group {
        return (CondExpr::ParenGroup(&args[0]), 1);
    }

    // Check for binary operator: atom BINOP atom
    if args.len() >= 3
        && !args[0].is_paren_group
        && is_condition_binary_op(&args[1].text)
        && !args[2].is_paren_group
    {
        return (
            CondExpr::Binary {
                lhs: Box::new(CondExpr::Atom(&args[0])),
                op: &args[1],
                rhs: Box::new(CondExpr::Atom(&args[2])),
            },
            3,
        );
    }

    // Simple atom
    (CondExpr::Atom(&args[0]), 1)
}

/// Compute the inline text width of a condition expression.
fn cond_expr_inline_width(expr: &CondExpr<'_>) -> usize {
    match expr {
        CondExpr::Atom(arg) => {
            let mut w = arg_width(arg);
            if let Some(c) = &arg.trailing_comment {
                w += 1 + c.len();
            }
            w
        }
        CondExpr::ParenGroup(arg) => {
            let mut w = arg_width(arg);
            if let Some(c) = &arg.trailing_comment {
                w += 1 + c.len();
            }
            w
        }
        CondExpr::Unary { op, operand } => {
            let mut w = op.text.len() + 1 + cond_expr_inline_width(operand);
            if let Some(c) = &op.trailing_comment {
                w += 1 + c.len();
            }
            w
        }
        CondExpr::Binary { lhs, op, rhs } => {
            let mut w =
                cond_expr_inline_width(lhs) + 1 + op.text.len() + 1 + cond_expr_inline_width(rhs);
            if let Some(c) = &op.trailing_comment {
                w += 1 + c.len();
            }
            w
        }
    }
}

/// Check if expression has line comments that force multi-line.
fn cond_expr_has_line_comment(expr: &CondExpr<'_>) -> bool {
    match expr {
        CondExpr::Atom(arg) => arg.trailing_comment.is_some() && !arg.trailing_is_bracket,
        CondExpr::ParenGroup(arg) => arg.trailing_comment.is_some() && !arg.trailing_is_bracket,
        CondExpr::Unary { op, operand } => {
            (op.trailing_comment.is_some() && !op.trailing_is_bracket)
                || cond_expr_has_line_comment(operand)
        }
        CondExpr::Binary { lhs, op, rhs } => {
            cond_expr_has_line_comment(lhs)
                || (op.trailing_comment.is_some() && !op.trailing_is_bracket)
                || cond_expr_has_line_comment(rhs)
        }
    }
}

fn gen_condition_multi_line(
    _cmd_name: &str,
    args: &[FormattedArg],
    config: &Configuration,
    indent_depth: u32,
) -> PrintItems {
    let cond_items = parse_condition_items(args);
    let base_indent = (indent_depth as usize + 1) * config.indent_width as usize;

    let mut inner = PrintItems::new();

    for item in &cond_items {
        inner.push_signal(Signal::NewLine);
        match item {
            CondItem::Expr(expr) => {
                emit_cond_expr(&mut inner, expr, config, base_indent);
            }
            CondItem::LogicalOp { op, comments, expr } => {
                emit_cond_logical_op(&mut inner, op, comments, expr, config, base_indent);
            }
            CondItem::Comment(arg) => {
                inner.extend(ir_helpers::gen_from_raw_string(&arg.text));
            }
        }
    }

    if config.closing_paren_newline {
        inner.push_signal(Signal::NewLine);
    }

    ir_helpers::with_indent(inner)
}

/// Emit a logical operator + expression (AND expr, OR expr).
fn emit_cond_logical_op(
    items: &mut PrintItems,
    op: &FormattedArg,
    comments: &[&FormattedArg],
    expr: &CondExpr<'_>,
    config: &Configuration,
    base_indent: usize,
) {
    let op_has_comment = op.trailing_comment.is_some() && !op.trailing_is_bracket;
    let has_interleaved_comments = !comments.is_empty();

    // Try inline: "AND expr" on one line (not possible with interleaved comments)
    let inline_width = op.text.len() + 1 + cond_expr_inline_width(expr);
    let can_inline = base_indent + inline_width <= config.line_width as usize
        && !cond_expr_has_line_comment(expr)
        && !op_has_comment
        && !has_interleaved_comments;

    if can_inline {
        let op_text = if op.is_keyword {
            apply_keyword_case(&op.text, config.keyword_case)
        } else {
            op.text.clone()
        };
        items.push_string(op_text);
        if let Some(comment) = &op.trailing_comment {
            items.push_space();
            items.extend(ir_helpers::gen_from_raw_string(comment));
        }
        items.push_space();
        emit_cond_expr_inline(items, expr, config);
        return;
    }

    // Expanded: operator on line, comments, then expression
    let op_text = if op.is_keyword {
        apply_keyword_case(&op.text, config.keyword_case)
    } else {
        op.text.clone()
    };
    items.push_string(op_text);
    if let Some(comment) = &op.trailing_comment {
        items.push_space();
        items.extend(ir_helpers::gen_from_raw_string(comment));
    }

    // Emit interleaved comments on their own lines
    for c in comments {
        items.push_signal(Signal::NewLine);
        items.extend(ir_helpers::gen_from_raw_string(&c.text));
    }

    if op_has_comment || has_interleaved_comments {
        // Comment(s) force line break: expression on next line indented
        let mut sub = PrintItems::new();
        sub.push_signal(Signal::NewLine);
        emit_cond_expr(
            &mut sub,
            expr,
            config,
            base_indent + config.indent_width as usize,
        );
        items.extend(ir_helpers::with_indent(sub));
    } else {
        // Try to put expression on same line (for short operators like AND/OR)
        items.push_space();
        emit_cond_expr(items, expr, config, base_indent);
    }
}

/// Emit a condition expression, handling inline vs expanded based on width.
fn emit_cond_expr(
    items: &mut PrintItems,
    expr: &CondExpr<'_>,
    config: &Configuration,
    base_indent: usize,
) {
    match expr {
        CondExpr::Atom(arg) => {
            emit_kw_arg(items, arg, config);
        }
        CondExpr::ParenGroup(arg) => {
            // Try inline paren group
            let inline = arg_inline_text(arg);
            if base_indent + inline.len() <= config.line_width as usize
                && !inline.contains('\n')
                && !has_line_comment_in_paren(arg)
            {
                items.extend(ir_helpers::gen_from_raw_string(&inline));
                if let Some(comment) = &arg.trailing_comment {
                    items.push_space();
                    items.extend(ir_helpers::gen_from_raw_string(comment));
                }
            } else {
                // Expanded paren group
                items.push_str_runtime_width_computed("(");
                let sub_indent = base_indent + config.indent_width as usize;
                let inner_items = parse_condition_items(&arg.paren_inner);
                let mut paren_inner = PrintItems::new();
                for sub_item in &inner_items {
                    paren_inner.push_signal(Signal::NewLine);
                    match sub_item {
                        CondItem::Expr(e) => {
                            emit_cond_expr(&mut paren_inner, e, config, sub_indent);
                        }
                        CondItem::LogicalOp {
                            op,
                            comments,
                            expr: e,
                        } => {
                            emit_cond_logical_op(
                                &mut paren_inner,
                                op,
                                comments,
                                e,
                                config,
                                sub_indent,
                            );
                        }
                        CondItem::Comment(arg) => {
                            paren_inner.extend(ir_helpers::gen_from_raw_string(&arg.text));
                        }
                    }
                }
                paren_inner.push_signal(Signal::NewLine);
                items.extend(ir_helpers::with_indent(paren_inner));
                items.push_str_runtime_width_computed(")");
                if let Some(comment) = &arg.trailing_comment {
                    items.push_space();
                    items.extend(ir_helpers::gen_from_raw_string(comment));
                }
            }
        }
        CondExpr::Unary { op, operand } => {
            let op_has_comment = op.trailing_comment.is_some() && !op.trailing_is_bracket;
            let inline_width = op.text.len() + 1 + cond_expr_inline_width(operand);
            let can_inline = base_indent + inline_width <= config.line_width as usize
                && !cond_expr_has_line_comment(operand)
                && !op_has_comment;

            if can_inline {
                items.push_string(if op.is_keyword {
                    apply_keyword_case(&op.text, config.keyword_case)
                } else {
                    op.text.clone()
                });
                if let Some(comment) = &op.trailing_comment {
                    items.push_space();
                    items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                items.push_space();
                emit_cond_expr_inline(items, operand, config);
                return;
            }

            // Expanded
            items.push_string(if op.is_keyword {
                apply_keyword_case(&op.text, config.keyword_case)
            } else {
                op.text.clone()
            });
            if let Some(comment) = &op.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }

            // For short operators (< indent_width): operand on same line
            // For long operators (>= indent_width): operand on next line indented
            if !op_has_comment && op.text.len() < config.indent_width as usize {
                items.push_space();
                emit_cond_expr(items, operand, config, base_indent);
            } else {
                let mut sub = PrintItems::new();
                sub.push_signal(Signal::NewLine);
                emit_cond_expr(
                    &mut sub,
                    operand,
                    config,
                    base_indent + config.indent_width as usize,
                );
                items.extend(ir_helpers::with_indent(sub));
            }
        }
        CondExpr::Binary { lhs, op, rhs } => {
            let op_has_comment = op.trailing_comment.is_some() && !op.trailing_is_bracket;
            let inline_width =
                cond_expr_inline_width(lhs) + 1 + op.text.len() + 1 + cond_expr_inline_width(rhs);
            let can_inline = base_indent + inline_width <= config.line_width as usize
                && !cond_expr_has_line_comment(lhs)
                && !cond_expr_has_line_comment(rhs)
                && !op_has_comment;

            if can_inline {
                emit_cond_expr_inline(items, lhs, config);
                items.push_space();
                items.push_string(if op.is_keyword {
                    apply_keyword_case(&op.text, config.keyword_case)
                } else {
                    op.text.clone()
                });
                if let Some(comment) = &op.trailing_comment {
                    items.push_space();
                    items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                items.push_space();
                emit_cond_expr_inline(items, rhs, config);
                return;
            }

            // Expanded: lhs on this line, operator+rhs indented
            emit_cond_expr(items, lhs, config, base_indent);
            let mut sub = PrintItems::new();
            sub.push_signal(Signal::NewLine);
            sub.push_string(if op.is_keyword {
                apply_keyword_case(&op.text, config.keyword_case)
            } else {
                op.text.clone()
            });
            if let Some(comment) = &op.trailing_comment {
                sub.push_space();
                sub.extend(ir_helpers::gen_from_raw_string(comment));
            }
            sub.push_signal(Signal::NewLine);
            emit_cond_expr(
                &mut sub,
                rhs,
                config,
                base_indent + config.indent_width as usize,
            );
            items.extend(ir_helpers::with_indent(sub));
        }
    }
}

/// Emit a condition expression inline (no width checks — caller verified it fits).
fn emit_cond_expr_inline(items: &mut PrintItems, expr: &CondExpr<'_>, config: &Configuration) {
    match expr {
        CondExpr::Atom(arg) => {
            let t = arg_inline_text(arg);
            let t = if arg.is_keyword {
                apply_keyword_case(&t, config.keyword_case)
            } else {
                t
            };
            items.extend(ir_helpers::gen_from_raw_string(&t));
            if let Some(comment) = &arg.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
        }
        CondExpr::ParenGroup(arg) => {
            items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(arg)));
            if let Some(comment) = &arg.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
        }
        CondExpr::Unary { op, operand } => {
            let op_text = if op.is_keyword {
                apply_keyword_case(&op.text, config.keyword_case)
            } else {
                op.text.clone()
            };
            items.push_string(op_text);
            if let Some(comment) = &op.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
            items.push_space();
            emit_cond_expr_inline(items, operand, config);
        }
        CondExpr::Binary { lhs, op, rhs } => {
            emit_cond_expr_inline(items, lhs, config);
            items.push_space();
            let op_text = if op.is_keyword {
                apply_keyword_case(&op.text, config.keyword_case)
            } else {
                op.text.clone()
            };
            items.push_string(op_text);
            if let Some(comment) = &op.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
            items.push_space();
            emit_cond_expr_inline(items, rhs, config);
        }
    }
}

fn has_line_comment_in_paren(arg: &FormattedArg) -> bool {
    for inner in &arg.paren_inner {
        if inner.trailing_comment.is_some() && !inner.trailing_is_bracket {
            return true;
        }
        if inner.text.starts_with('#') {
            return true;
        }
    }
    false
}

// ===========================================================================
// Shared emit helpers
// ===========================================================================

/// Emit a single argument with its trailing comment.
/// Applies literalCase if the token is in the literal list and not a keyword.
fn emit_arg(items: &mut PrintItems, arg: &FormattedArg, config: &Configuration) {
    emit_arg_with_case(items, arg, None, config.literal_case);
}

/// Emit a keyword argument with keyword casing applied.
fn emit_kw_arg(items: &mut PrintItems, arg: &FormattedArg, config: &Configuration) {
    emit_arg_with_case(items, arg, Some(config.keyword_case), config.literal_case);
}

/// Emit a sub-keyword with keyword casing applied unconditionally.
/// Used for sub-keywords identified by position in section groups,
/// which lack the `is_keyword` flag.
fn emit_sub_kw_arg(items: &mut PrintItems, arg: &FormattedArg, config: &Configuration) {
    let text = apply_keyword_case(&arg.text, config.keyword_case);
    items.extend(ir_helpers::gen_from_raw_string(&text));
    if let Some(comment) = &arg.trailing_comment {
        items.push_space();
        if arg.trailing_is_bracket {
            emit_bracket_verbatim(items, comment);
        } else {
            items.extend(ir_helpers::gen_from_raw_string(comment));
        }
    }
}

fn emit_arg_with_case(
    items: &mut PrintItems,
    arg: &FormattedArg,
    kw_case: Option<CaseStyle>,
    literal_case: CaseStyle,
) {
    if arg.is_paren_group {
        items.push_str_runtime_width_computed("(");
        if !arg.paren_inner.is_empty() {
            let paren_items = gen_flat_paren_inner(&arg.paren_inner, literal_case);
            items.extend(paren_items);
        }
        items.push_str_runtime_width_computed(")");
    } else if arg.is_bracket || arg.text.starts_with("#[") {
        emit_bracket_verbatim(items, &arg.text);
    } else {
        let text = if let Some(case) = kw_case {
            if arg.is_keyword {
                apply_keyword_case(&arg.text, case)
            } else if !arg.text.starts_with('"') && is_literal_token(&arg.text) {
                apply_literal_case(&arg.text, literal_case)
            } else {
                arg.text.clone()
            }
        } else if !arg.text.starts_with('"')
            && !arg.text.starts_with('#')
            && is_literal_token(&arg.text)
        {
            apply_literal_case(&arg.text, literal_case)
        } else {
            arg.text.clone()
        };
        if text.contains('\n') {
            // Multi-line quoted strings: emit first line normally (gets indent
            // from context), then continuation lines verbatim.
            let first_nl = text.find('\n').unwrap();
            let first_line = &text[..first_nl];
            let rest = &text[first_nl + 1..];
            items.push_string(first_line.to_string());
            items.push_signal(Signal::StartIgnoringIndent);
            for line in rest.lines() {
                items.push_signal(Signal::NewLine);
                if !line.is_empty() {
                    items.push_string(line.to_string());
                }
            }
            if rest.ends_with('\n') {
                items.push_signal(Signal::NewLine);
            }
            items.push_signal(Signal::FinishIgnoringIndent);
        } else {
            items.extend(ir_helpers::gen_from_raw_string(&text));
        }
    }

    if let Some(comment) = &arg.trailing_comment {
        items.push_space();
        if arg.trailing_is_bracket {
            emit_bracket_verbatim(items, comment);
        } else {
            items.extend(ir_helpers::gen_from_raw_string(comment));
        }
    }
}

fn emit_bracket_verbatim(items: &mut PrintItems, text: &str) {
    if let Some(first_nl) = text.find('\n') {
        let first_line = &text[..first_nl];
        items.push_string(first_line.to_string());
        let rest = &text[first_nl + 1..];
        items.push_signal(Signal::StartIgnoringIndent);
        for line in rest.lines() {
            items.push_signal(Signal::NewLine);
            if !line.is_empty() {
                items.push_string(line.to_string());
            }
        }
        if rest.ends_with('\n') {
            items.push_signal(Signal::NewLine);
        }
        items.push_signal(Signal::FinishIgnoringIndent);
    } else {
        items.push_string(text.to_string());
    }
}

fn gen_flat_paren_inner(args: &[FormattedArg], literal_case: CaseStyle) -> PrintItems {
    let mut inner = PrintItems::new();
    for arg in args {
        inner.push_signal(Signal::NewLine);
        emit_arg_with_case(&mut inner, arg, None, literal_case);
    }
    inner.push_signal(Signal::NewLine);
    ir_helpers::with_indent(inner)
}

// ===========================================================================
// Sorting
// ===========================================================================

/// Determine if sorting should be applied for a given command.
fn should_sort_for_command(
    raw_name: &str,
    config: &Configuration,
    _cmd_kind: Option<&CommandKind>,
) -> bool {
    match &config.sort_arguments {
        SortArguments::Disabled => false,
        SortArguments::Enabled => {
            // Sort if the command is sortable OR has customKeywords
            is_sortable_command(raw_name) || !config.custom_keywords.is_empty()
        }
        SortArguments::CommandList(_keywords) => {
            // Only sort sections whose keyword name is in the list
            // Check that the command has at least one sortable keyword
            is_sortable_command(raw_name) || !config.custom_keywords.is_empty()
        }
    }
}

fn sort_argument_groups(
    args: &mut [FormattedArg],
    config: &Configuration,
    _cmd_kind: Option<&CommandKind>,
) {
    let selective_keywords: Option<&[String]> = match &config.sort_arguments {
        SortArguments::CommandList(kws) => Some(kws),
        _ => None,
    };

    let mut i = 0;
    while i < args.len() {
        if args[i].is_keyword {
            let kw_text = &args[i].text;
            // Determine if this keyword section is sortable
            let sortable = if let Some(sel_kws) = selective_keywords {
                // Only sort sections whose keyword name matches the list
                sel_kws.iter().any(|k| k.eq_ignore_ascii_case(kw_text))
            } else {
                // sortArguments=true: sort if keyword is a known sort group keyword
                // or a customKeyword
                is_sort_group_keyword(kw_text)
                    || config
                        .custom_keywords
                        .iter()
                        .any(|k| k.eq_ignore_ascii_case(kw_text))
            };

            if sortable {
                let start = i + 1;
                let mut end = start;
                // Find end of this section (next keyword or end of args)
                // Respect sub-keyword structures: if a sub-keyword is encountered,
                // don't sort past it
                while end < args.len() && !args[end].is_keyword {
                    end += 1;
                }
                if end > start {
                    sort_section_with_groups(&mut args[start..end]);
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}

/// Sort a section of arguments respecting group boundaries.
/// A blank-line-preceded comment acts as a group boundary;
/// sorting happens independently within each sub-group.
fn sort_section_with_groups(args: &mut [FormattedArg]) {
    #[derive(Clone)]
    struct SortUnit {
        key: String,
        items: Vec<FormattedArg>,
    }

    fn is_standalone_comment(arg: &FormattedArg) -> bool {
        arg.text.starts_with('#')
    }

    fn sort_segment(segment: &[FormattedArg]) -> Vec<FormattedArg> {
        let mut units: Vec<SortUnit> = Vec::new();
        let mut pending_comments: Vec<FormattedArg> = Vec::new();

        for item in segment {
            if is_standalone_comment(item) {
                pending_comments.push(item.clone());
                continue;
            }

            let mut grouped_items = Vec::new();
            grouped_items.append(&mut pending_comments);
            grouped_items.push(item.clone());

            units.push(SortUnit {
                key: item.text.to_ascii_lowercase(),
                items: grouped_items,
            });
        }

        if units.is_empty() {
            return pending_comments;
        }

        let trailing_comments = pending_comments;
        units.sort_by(|a, b| a.key.cmp(&b.key));

        let mut sorted = Vec::new();
        for unit in units {
            sorted.extend(unit.items);
        }
        sorted.extend(trailing_comments);
        sorted
    }

    let mut rebuilt: Vec<FormattedArg> = Vec::with_capacity(args.len());
    let mut segment_start = 0;

    for (index, arg) in args.iter().enumerate() {
        let is_boundary = arg.text.starts_with('#') && arg.blank_line_before;
        if !is_boundary {
            continue;
        }

        if segment_start < index {
            rebuilt.extend(sort_segment(&args[segment_start..index]));
        }
        rebuilt.push(arg.clone());
        segment_start = index + 1;
    }

    if segment_start < args.len() {
        rebuilt.extend(sort_segment(&args[segment_start..]));
    }

    debug_assert_eq!(rebuilt.len(), args.len());
    args.clone_from_slice(&rebuilt);
}

/// Reorder entire keyword sections to match a canonical order.
/// Positional arguments before the first keyword stay in place.
fn sort_keyword_sections_by_order(args: &mut Vec<FormattedArg>, order: &[&str]) {
    fn is_standalone_comment(arg: &FormattedArg) -> bool {
        arg.text.starts_with('#')
    }

    fn is_order_keyword(arg: &FormattedArg, order: &[&str]) -> bool {
        arg.is_keyword
            && order
                .iter()
                .any(|&section| section.eq_ignore_ascii_case(&arg.text))
    }

    fn take_trailing_attached_comments(items: &mut Vec<FormattedArg>) -> Vec<FormattedArg> {
        let mut trailing: Vec<FormattedArg> = Vec::new();
        while let Some(last) = items.last() {
            if !is_standalone_comment(last) || last.blank_line_before {
                break;
            }
            trailing.push(items.pop().expect("last item must exist"));
        }
        trailing.reverse();
        trailing
    }

    let first_kw = match args.iter().position(|a| is_order_keyword(a, order)) {
        Some(pos) => pos,
        None => return,
    };

    let mut positional_prefix: Vec<FormattedArg> = args[..first_kw].to_vec();
    let mut pre_comments = take_trailing_attached_comments(&mut positional_prefix);

    let mut sections: Vec<(String, Vec<FormattedArg>)> = Vec::new();
    let mut index = first_kw;

    while index < args.len() {
        let mut section_pre_comments = std::mem::take(&mut pre_comments);
        while index < args.len() && !is_order_keyword(&args[index], order) {
            section_pre_comments.push(args[index].clone());
            index += 1;
        }

        if index >= args.len() {
            if let Some(last) = sections.last_mut() {
                last.1.extend(section_pre_comments);
            } else {
                positional_prefix.extend(section_pre_comments);
            }
            break;
        }

        let keyword_name = args[index].text.to_ascii_uppercase();
        let mut section_items = section_pre_comments;
        section_items.push(args[index].clone());
        index += 1;

        // Keep nested keywords (for example DESTINATION inside install sections)
        // in their current section; only canonical section keywords split sections.
        while index < args.len() && !is_order_keyword(&args[index], order) {
            section_items.push(args[index].clone());
            index += 1;
        }

        pre_comments = take_trailing_attached_comments(&mut section_items);
        sections.push((keyword_name, section_items));
    }

    if !pre_comments.is_empty() {
        if let Some(last) = sections.last_mut() {
            last.1.extend(pre_comments);
        } else {
            positional_prefix.extend(pre_comments);
        }
    }

    sections.sort_by(|a, b| {
        let pos_a = order
            .iter()
            .position(|&o| o.eq_ignore_ascii_case(&a.0))
            .unwrap_or(usize::MAX);
        let pos_b = order
            .iter()
            .position(|&o| o.eq_ignore_ascii_case(&b.0))
            .unwrap_or(usize::MAX);
        pos_a.cmp(&pos_b)
    });

    args.clear();
    args.extend(positional_prefix);
    for (_, section_args) in sections {
        args.extend(section_args);
    }
}

// ---------------------------------------------------------------------------
// Generator expression (genex) formatting
// ---------------------------------------------------------------------------

/// Compute the net depth change of `$<` / `>` in a string.
/// Each `$<` increments by 1, each `>` decrements by 1.
fn genex_depth_delta(text: &str) -> i32 {
    let mut depth: i32 = 0;
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'$' && bytes[i + 1] == b'<' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'>' {
            depth -= 1;
            i += 1;
        } else {
            i += 1;
        }
    }
    depth
}

/// Split `text` at occurrences of `sep` that are at genex depth 0.
fn split_at_depth0(text: &str, sep: char) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth: i32 = 0;
    let mut start = 0;
    let bytes = text.as_bytes();
    let sep_byte = sep as u8;
    let mut i = 0;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'$' && bytes[i + 1] == b'<' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'>' {
            depth -= 1;
            i += 1;
        } else if bytes[i] == sep_byte && depth == 0 {
            result.push(&text[start..i]);
            start = i + 1;
            i += 1;
        } else {
            i += 1;
        }
    }
    if start <= text.len() {
        result.push(&text[start..]);
    }
    result
}

/// Find the byte offset of the first `:` at genex depth 0, or `None`.
fn find_depth0_colon(text: &str) -> Option<usize> {
    let mut depth: i32 = 0;
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'$' && bytes[i + 1] == b'<' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'>' {
            depth -= 1;
            i += 1;
        } else if bytes[i] == b':' && depth == 0 {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

/// A parsed generator expression tree node.
enum GenexNode {
    /// Plain text (no genex).
    Text(String),
    /// `$<NAME>` — a genex with no colon (simple variable reference).
    SimpleGenex { prefix: String, name: String },
    /// `$<NAME:child1,child2,...>` — a named genex with comma-separated children.
    NamedGenex {
        prefix: String,
        name: String,
        children: Vec<GenexNode>,
    },
    /// `$<$<COND>:val1 val2 ...>` — condition genex wrapping space-separated values.
    ConditionGenex {
        condition: Box<GenexNode>,
        values: Vec<GenexNode>,
    },
}

/// Parse a text fragment into a `GenexNode`.
/// The text may start with a non-genex prefix (e.g. `LOG_LEVEL=$<IF:...>`).
fn parse_genex(text: &str) -> GenexNode {
    let text = text.trim();
    // Find the first `$<` in the text.
    let genex_start = text.find("$<");
    match genex_start {
        None => GenexNode::Text(text.to_owned()),
        Some(0) => parse_genex_inner(text),
        Some(pos) => {
            // There is a prefix before the genex (e.g. "LOG_LEVEL=").
            let prefix = &text[..pos];
            let rest = &text[pos..];
            let mut node = parse_genex_inner(rest);
            // Attach prefix to the parsed node.
            match &mut node {
                GenexNode::SimpleGenex { prefix: p, .. }
                | GenexNode::NamedGenex { prefix: p, .. } => {
                    *p = prefix.to_owned();
                }
                GenexNode::ConditionGenex { .. } | GenexNode::Text(_) => {
                    // Condition genex or text shouldn't normally have a prefix,
                    // but if they do, wrap in text.
                    return GenexNode::Text(text.to_owned());
                }
            }
            node
        }
    }
}

/// Parse text that starts with `$<` as a genex expression.
fn parse_genex_inner(text: &str) -> GenexNode {
    let text = text.trim();
    // Must start with `$<` and end with `>`.
    if !text.starts_with("$<") || !text.ends_with('>') {
        return GenexNode::Text(text.to_owned());
    }
    // Strip outer `$<` and `>`.
    let inner = &text[2..text.len() - 1];

    // Find first depth-0 `:` within the stripped content.
    match find_depth0_colon(inner) {
        None => {
            // No colon \u2192 SimpleGenex like `$<CXX_COMPILER_VERSION>`.
            GenexNode::SimpleGenex {
                prefix: String::new(),
                name: inner.trim().to_owned(),
            }
        }
        Some(colon_pos) => {
            let before_colon = inner[..colon_pos].trim();
            let after_colon = inner[colon_pos + 1..].trim();

            if before_colon.starts_with("$<") {
                // Condition genex: `$<$<COND>:values>`.
                let condition = parse_genex(before_colon);
                // Values are space-separated at depth 0.
                let value_parts = split_at_depth0(after_colon, ' ');
                let values = value_parts
                    .into_iter()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(parse_genex)
                    .collect();
                GenexNode::ConditionGenex {
                    condition: Box::new(condition),
                    values,
                }
            } else {
                // Named genex: `$<NAME:children>`.
                let name = before_colon.to_owned();
                let child_parts = split_at_depth0(after_colon, ',');
                let children = child_parts
                    .into_iter()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(parse_genex)
                    .collect();
                GenexNode::NamedGenex {
                    prefix: String::new(),
                    name,
                    children,
                }
            }
        }
    }
}

/// Reconstruct the flat (inline) text of a genex node.
fn genex_inline_text(node: &GenexNode) -> String {
    match node {
        GenexNode::Text(s) => s.clone(),
        GenexNode::SimpleGenex { prefix, name } => {
            format!("{prefix}$<{name}>")
        }
        GenexNode::NamedGenex {
            prefix,
            name,
            children,
        } => {
            let children_text: Vec<String> = children.iter().map(genex_inline_text).collect();
            format!("{prefix}$<{name}:{}>", children_text.join(","))
        }
        GenexNode::ConditionGenex { condition, values } => {
            let cond_text = genex_inline_text(condition);
            let values_text: Vec<String> = values.iter().map(genex_inline_text).collect();
            format!("$<{cond_text}:{}>", values_text.join(" "))
        }
    }
}

/// Returns true if a genex node should be formatted inline (single line).
fn genex_is_inline(node: &GenexNode) -> bool {
    match node {
        GenexNode::Text(_) | GenexNode::SimpleGenex { .. } => true,
        GenexNode::NamedGenex { children, .. } => {
            // Inline only if exactly 1 child that is itself inline-simple.
            children.len() == 1
                && matches!(
                    &children[0],
                    GenexNode::Text(_) | GenexNode::SimpleGenex { .. }
                )
        }
        GenexNode::ConditionGenex { .. } => false,
    }
}

fn condition_genex_should_inline(
    condition: &GenexNode,
    values: &[GenexNode],
    depth: usize,
    close_suffix: &str,
) -> bool {
    depth == 0
        && close_suffix.is_empty()
        && genex_is_inline(condition)
        && !values.is_empty()
        && values.iter().all(genex_is_inline)
}

fn condition_genex_prefers_compact_close(condition: &GenexNode) -> bool {
    match condition {
        GenexNode::NamedGenex { name, children, .. } => {
            matches!(name.to_ascii_uppercase().as_str(), "OR" | "AND" | "NOT")
                && !children.is_empty()
                && children.iter().all(genex_is_inline)
        }
        _ => false,
    }
}

/// Format a genex node tree into PrintItems.
fn format_genex_node(items: &mut PrintItems, node: &GenexNode) {
    format_genex_impl(items, node, "", 0);
}

/// Internal: format a genex node, appending `close_suffix` after the closing `>`.
/// This is used by ConditionGenex to merge condition's `>` with `:`.
fn format_genex_impl(items: &mut PrintItems, node: &GenexNode, close_suffix: &str, depth: usize) {
    match node {
        GenexNode::Text(s) => {
            items.extend(ir_helpers::gen_from_raw_string(s));
            if !close_suffix.is_empty() {
                items.extend(ir_helpers::gen_from_raw_string(close_suffix));
            }
        }
        GenexNode::SimpleGenex { prefix, name } => {
            let text = format!("{prefix}$<{name}>{close_suffix}");
            items.extend(ir_helpers::gen_from_raw_string(&text));
        }
        GenexNode::NamedGenex {
            prefix,
            name,
            children,
        } => {
            if genex_is_inline(node) {
                let text = format!("{}{close_suffix}", genex_inline_text(node));
                items.extend(ir_helpers::gen_from_raw_string(&text));
            } else {
                // Multi-line: `$<NAME:` then children indented, then `>`.
                let opener = format!("{prefix}$<{name}:");
                items.extend(ir_helpers::gen_from_raw_string(&opener));
                let mut inner = PrintItems::new();
                for (i, child) in children.iter().enumerate() {
                    inner.push_signal(Signal::NewLine);
                    format_genex_impl(&mut inner, child, "", depth + 1);
                    if i + 1 < children.len() {
                        inner.push_str_runtime_width_computed(",");
                    }
                }
                items.extend(ir_helpers::with_indent(inner));
                let compact_close =
                    close_suffix == ":" && condition_genex_prefers_compact_close(node);
                if compact_close {
                    let closer = format!(">{close_suffix}");
                    items.extend(ir_helpers::gen_from_raw_string(&closer));
                } else {
                    items.push_signal(Signal::NewLine);
                    let closer = format!(">{close_suffix}");
                    items.extend(ir_helpers::gen_from_raw_string(&closer));
                }
            }
        }
        GenexNode::ConditionGenex { condition, values } => {
            if condition_genex_should_inline(condition, values, depth, close_suffix) {
                let text = format!("{}{close_suffix}", genex_inline_text(node));
                items.extend(ir_helpers::gen_from_raw_string(&text));
                return;
            } else if genex_is_inline(condition) {
                // Condition fits inline: `$<inline_cond:`
                let cond_text = genex_inline_text(condition);
                let opener = format!("$<{cond_text}:");
                items.extend(ir_helpers::gen_from_raw_string(&opener));
            } else {
                // Condition is multi-line: `$<` then condition with >: suffix.
                items.extend(ir_helpers::gen_from_raw_string("$<"));
                format_genex_impl(items, condition, ":", depth + 1);
            }
            // Values indented.
            let mut val_items = PrintItems::new();
            for val in values {
                val_items.push_signal(Signal::NewLine);
                format_genex_impl(&mut val_items, val, "", depth + 1);
            }
            items.extend(ir_helpers::with_indent(val_items));
            items.push_signal(Signal::NewLine);
            let closer = format!(">{close_suffix}");
            items.extend(ir_helpers::gen_from_raw_string(&closer));
        }
    }
}

/// Group a slice of `FormattedArg` values into genex groups and standalone args.
/// A genex group is a sequence of consecutive args whose cumulative `$<`/`>`
/// depth goes above 0 and returns to 0.
enum GenexArgGroup<'a> {
    /// A standalone argument (not part of a genex spanning multiple tokens).
    Single(&'a FormattedArg),
    /// A group of consecutive args forming one complete genex.
    Genex(Vec<&'a FormattedArg>),
}

fn group_args_by_genex<'a>(args: &[&'a FormattedArg]) -> Vec<GenexArgGroup<'a>> {
    let mut groups: Vec<GenexArgGroup<'a>> = Vec::new();
    let mut depth: i32 = 0;
    let mut genex_buf: Vec<&'a FormattedArg> = Vec::new();

    for &arg in args {
        let delta = genex_depth_delta(&arg.text);
        if depth == 0 && delta == 0 {
            // Standalone arg, not genex.
            // But it might contain a self-contained genex (like LOG_LEVEL=$<IF:...>).
            if arg.text.contains("$<") {
                groups.push(GenexArgGroup::Genex(vec![arg]));
            } else {
                groups.push(GenexArgGroup::Single(arg));
            }
        } else {
            genex_buf.push(arg);
            depth += delta;
            if depth <= 0 {
                // Genex group complete.
                groups.push(GenexArgGroup::Genex(std::mem::take(&mut genex_buf)));
                depth = 0;
            }
        }
    }
    // If there are leftover tokens (unclosed genex), emit them individually.
    for arg in genex_buf {
        groups.push(GenexArgGroup::Single(arg));
    }
    groups
}

/// Emit values under a regular keyword, detecting and formatting genex groups.
fn emit_values_with_genex(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    config: &Configuration,
) {
    let groups = group_args_by_genex(values);
    let mut val_items = PrintItems::new();
    let mut first_group_is_inline_condition = false;

    for (group_idx, group) in groups.iter().enumerate() {
        let group_has_leading_blank = match group {
            GenexArgGroup::Single(arg) => arg.blank_line_before,
            GenexArgGroup::Genex(args) => args
                .first()
                .map(|first| first.blank_line_before)
                .unwrap_or(false),
        };
        if group_has_leading_blank {
            val_items.push_signal(Signal::NewLine);
        }

        match group {
            GenexArgGroup::Single(arg) => {
                val_items.push_signal(Signal::NewLine);
                emit_arg(&mut val_items, arg, config);
            }
            GenexArgGroup::Genex(args) => {
                // Join token texts with spaces to reconstruct the full genex.
                let joined: String = args
                    .iter()
                    .map(|a| a.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                let node = parse_genex(&joined);
                if group_idx == 0
                    && let GenexNode::ConditionGenex { condition, values } = &node
                    && condition_genex_should_inline(condition, values, 0, "")
                {
                    first_group_is_inline_condition = true;
                }
                val_items.push_signal(Signal::NewLine);
                format_genex_node(&mut val_items, &node);
            }
        }
    }
    if first_group_is_inline_condition {
        items.push_str_runtime_width_computed(" ");
    }
    items.extend(ir_helpers::with_indent(val_items));
}

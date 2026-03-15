use std::borrow::Cow;

use tracing::info_span;

use crate::configuration::{
    CaseStyle, Configuration, IndentStyle, SortArguments, SpaceInsideParen, WrapStyle,
};
use crate::instrumentation::EVENT_GEN_COMMAND;
use crate::parser::ast::{Argument, CommandInvocation};
use crate::printer::ir_helpers;
use crate::printer::{PrintItems, Signal};

use super::signatures::{CommandKind, CommandSpec, EMPTY_SPEC, KwType, lookup_command};
// ---------------------------------------------------------------------------
// Literal tokens: boolean values and comparison operators subject to literalCase.
// Keyword-vs-literal precedence: if a token is a keyword for the current
// command (per CommandSpec) or in customKeywords, keywordCase wins; otherwise
// literalCase applies when the token matches this list (§4.4).
// ---------------------------------------------------------------------------
const LITERAL_TOKENS: &[&str] = &[
    "AND",
    "COMMAND",
    "DEFINED",
    "EQUAL",
    "EXISTS",
    "FALSE",
    "GREATER_EQUAL",
    "GREATER",
    "IN_LIST",
    "IS_ABSOLUTE",
    "IS_DIRECTORY",
    "IS_NEWER_THAN",
    "IS_SYMLINK",
    "LESS_EQUAL",
    "LESS",
    "MATCHES",
    "NO",
    "NOT",
    "OFF",
    "ON",
    "OR",
    "PATH_EQUAL",
    "POLICY",
    "STREQUAL",
    "STRGREATER_EQUAL",
    "STRGREATER",
    "STRLESS_EQUAL",
    "STRLESS",
    "TARGET",
    "TEST",
    "TRUE",
    "VERSION_EQUAL",
    "VERSION_GREATER_EQUAL",
    "VERSION_GREATER",
    "VERSION_LESS_EQUAL",
    "VERSION_LESS",
    "YES",
];

/// Check if a token is in the literal list (case-insensitive binary search).
fn is_literal_token(text: &str) -> bool {
    LITERAL_TOKENS
        .binary_search_by(|probe| {
            let mut probe_bytes = probe.as_bytes().iter();
            let mut text_bytes = text.as_bytes().iter();
            loop {
                match (probe_bytes.next(), text_bytes.next()) {
                    (Some(&a), Some(&b)) => {
                        let ord = a.cmp(&b.to_ascii_uppercase());
                        if ord != std::cmp::Ordering::Equal {
                            return ord;
                        }
                    }
                    (None, None) => return std::cmp::Ordering::Equal,
                    (None, Some(_)) => return std::cmp::Ordering::Less,
                    (Some(_), None) => return std::cmp::Ordering::Greater,
                }
            }
        })
        .is_ok()
}

/// Apply case style in-place on an owned string to avoid extra allocations when
/// the caller already materialized token text.
fn apply_case_owned(mut text: String, style: CaseStyle) -> String {
    match style {
        CaseStyle::Lower => text.make_ascii_lowercase(),
        CaseStyle::Upper => text.make_ascii_uppercase(),
        CaseStyle::Preserve => {}
    }
    text
}

/// Apply literal casing per §4.4. Only applies to unquoted arguments that match
/// the literal token list and are NOT classified as keywords for the current command.
fn apply_literal_case<'a>(text: &'a str, style: CaseStyle) -> Cow<'a, str> {
    match style {
        CaseStyle::Preserve => Cow::Borrowed(text),
        CaseStyle::Lower => {
            let mut s = text.to_string();
            s.make_ascii_lowercase();
            Cow::Owned(s)
        }
        CaseStyle::Upper => {
            let mut s = text.to_string();
            s.make_ascii_uppercase();
            Cow::Owned(s)
        }
    }
}

fn apply_literal_case_owned(text: String, style: CaseStyle) -> String {
    apply_case_owned(text, style)
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
        Some(CommandKind::Known(spec)) => {
            is_in_command_spec(text, spec) || is_sort_group_keyword(text)
        }
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

/// Commands where arguments after keywords (or trailing positional args) can be sorted.
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
    "add_library",
    "add_executable",
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
    let mut buf = [0u8; 64];
    let len = cmd_name.len();
    if len > buf.len() {
        return None;
    }
    buf[..len].copy_from_slice(cmd_name.as_bytes());
    buf[..len].make_ascii_lowercase();
    let lower = std::str::from_utf8(&buf[..len]).unwrap();
    match lower {
        "target_link_libraries" => Some(&[
            "PUBLIC",
            "INTERFACE",
            "PRIVATE",
            "LINK_PUBLIC",
            "LINK_PRIVATE",
            "LINK_INTERFACE_LIBRARIES",
        ]),
        "target_sources" => Some(&["PUBLIC", "INTERFACE", "PRIVATE"]),
        "export" => Some(&["PACKAGE_DEPENDENCY", "TARGET", "VERSION"]),
        _ => None,
    }
}

/// Commands with section-like keywords but no canonical order in Appendix F.
fn non_canonical_section_keywords(cmd_name: &str) -> Option<&'static [&'static str]> {
    let mut buf = [0u8; 64];
    let len = cmd_name.len();
    if len > buf.len() {
        return None;
    }
    buf[..len].copy_from_slice(cmd_name.as_bytes());
    buf[..len].make_ascii_lowercase();
    let lower = std::str::from_utf8(&buf[..len]).unwrap();
    match lower {
        "target_compile_definitions"
        | "target_compile_options"
        | "target_compile_features"
        | "target_link_options"
        | "target_include_directories" => Some(&["PUBLIC", "INTERFACE", "PRIVATE"]),
        _ => None,
    }
}

/// Keep keyword sections in their first-seen order while preserving section boundaries.
fn sort_keyword_sections_in_source_order(args: &mut Vec<FormattedArg>, candidates: &[&str]) {
    let mut seen: Vec<String> = Vec::new();
    for arg in args.iter() {
        if !arg.is_keyword
            || !candidates
                .iter()
                .any(|keyword| keyword.eq_ignore_ascii_case(&arg.text))
            || seen
                .iter()
                .any(|keyword| keyword.eq_ignore_ascii_case(&arg.text))
        {
            continue;
        }
        seen.push(arg.text.clone());
    }

    if seen.is_empty() {
        return;
    }

    let order: Vec<&str> = seen.iter().map(String::as_str).collect();
    sort_keyword_sections_by_order(args, &order);
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

fn apply_command_case<'a>(name: &'a str, style: CaseStyle) -> Cow<'a, str> {
    match style {
        CaseStyle::Preserve => Cow::Borrowed(name),
        CaseStyle::Lower => {
            let mut s = name.to_string();
            s.make_ascii_lowercase();
            Cow::Owned(s)
        }
        CaseStyle::Upper => {
            let mut s = name.to_string();
            s.make_ascii_uppercase();
            Cow::Owned(s)
        }
    }
}

/// Apply keyword casing. Unlike the old version, this does NOT normalize booleans —
/// boolean/literal casing is now handled separately via literalCase.
fn apply_keyword_case<'a>(text: &'a str, style: CaseStyle) -> Cow<'a, str> {
    match style {
        CaseStyle::Preserve => Cow::Borrowed(text),
        CaseStyle::Lower => {
            let mut s = text.to_string();
            s.make_ascii_lowercase();
            Cow::Owned(s)
        }
        CaseStyle::Upper => {
            let mut s = text.to_string();
            s.make_ascii_uppercase();
            Cow::Owned(s)
        }
    }
}

fn apply_keyword_case_owned(text: String, style: CaseStyle) -> String {
    apply_case_owned(text, style)
}

#[derive(Debug, Clone, Copy, Default)]
struct SingleLineParenSpacing {
    after_open: bool,
    before_close: bool,
}

fn resolve_single_line_paren_spacing(
    cmd: &CommandInvocation,
    source: &str,
    config: &Configuration,
    has_arguments: bool,
) -> SingleLineParenSpacing {
    if !has_arguments {
        return SingleLineParenSpacing::default();
    }

    match config.space_inside_paren {
        SpaceInsideParen::Insert => SingleLineParenSpacing {
            after_open: true,
            before_close: true,
        },
        SpaceInsideParen::Remove => SingleLineParenSpacing::default(),
        SpaceInsideParen::Preserve => {
            let between_parens = &source[cmd.open_paren.end..cmd.close_paren.start];
            if between_parens.contains('\n') {
                return SingleLineParenSpacing::default();
            }

            let after_open = between_parens
                .as_bytes()
                .first()
                .is_some_and(|byte| matches!(*byte, b' ' | b'\t'));
            let before_close = between_parens
                .as_bytes()
                .last()
                .is_some_and(|byte| matches!(*byte, b' ' | b'\t'));

            SingleLineParenSpacing {
                after_open,
                before_close,
            }
        }
    }
}

fn extract_deferred_closing_comment(arguments: &mut [FormattedArg]) -> Option<String> {
    arguments.iter_mut().rev().find_map(|arg| {
        if arg.text.starts_with('#') || arg.trailing_is_bracket {
            return None;
        }

        arg.trailing_comment.take()
    })
}

fn push_comment_gap(items: &mut PrintItems, gap: u8) {
    for _ in 0..usize::from(gap) {
        items.push_space();
    }
}

fn visual_indent_prefix(width: usize, config: &Configuration) -> String {
    if width == 0 {
        return String::new();
    }

    match config.indent_style {
        IndentStyle::Space => " ".repeat(width),
        IndentStyle::Tab => {
            let tab_width = config.indent_width as usize;
            let tabs = width / tab_width;
            let spaces = width % tab_width;
            let mut s = String::with_capacity(tabs + spaces);
            for _ in 0..tabs {
                s.push('\t');
            }
            for _ in 0..spaces {
                s.push(' ');
            }
            s
        }
    }
}

fn push_visual_indent(items: &mut PrintItems, width: usize, config: &Configuration) {
    let prefix = visual_indent_prefix(width, config);
    if !prefix.is_empty() {
        items.extend(ir_helpers::gen_from_raw_string(&prefix));
    }
}

fn push_newline_with_visual_indent(items: &mut PrintItems, width: usize, config: &Configuration) {
    items.push_signal(Signal::NewLine);
    push_visual_indent(items, width, config);
}

fn push_wrapped_newline(
    items: &mut PrintItems,
    wrap_indent: bool,
    visual_indent: usize,
    config: &Configuration,
) {
    if wrap_indent {
        push_newline_with_visual_indent(items, visual_indent, config);
    } else {
        items.push_signal(Signal::NewLine);
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
    let command_stage = info_span!(
        EVENT_GEN_COMMAND,
        command = raw_name,
        indent_depth,
        source_is_multiline = tracing::field::Empty,
        argument_count = cmd.arguments.len()
    );
    let _command_entered = command_stage.enter();

    let effective = config.effective_config_for_command(raw_name);
    let config = effective.as_ref();

    let mut items = PrintItems::with_capacity(cmd.arguments.len() * 4 + 10);

    // Command name with casing
    let formatted_name = apply_command_case(raw_name, config.command_case);

    // Multi-line quoted/bracket arguments are emitted via the raw command path so
    // their internal content and line endings remain byte-preserved.
    if has_multiline_verbatim_argument(&cmd.arguments, source) {
        return gen_unknown_command(cmd, source, config, &formatted_name);
    }

    // Check if this is an unknown command — preserve original formatting
    // But if customKeywords is set, unknown commands may still have keyword recognition
    let cmd_kind = lookup_command(raw_name);
    if cmd_kind.is_none() && config.custom_keywords.is_empty() {
        return gen_unknown_command(cmd, source, config, &formatted_name);
    }

    items.push_string(formatted_name.clone().into_owned());

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
    let has_keyword_args = arguments.iter().any(|arg| arg.is_keyword);
    // Sort keyword sections if enabled
    if config.sort_keyword_sections && has_keyword_args {
        if let Some(order) = canonical_section_order(raw_name) {
            sort_keyword_sections_by_order(&mut arguments, order);
        } else if let Some(candidates) = non_canonical_section_keywords(raw_name) {
            sort_keyword_sections_in_source_order(&mut arguments, candidates);
        }
    }

    let single_line_paren_spacing =
        resolve_single_line_paren_spacing(cmd, source, config, !arguments.is_empty());
    let source_is_multiline = source[cmd.open_paren.end..cmd.close_paren.start].contains('\n');
    command_stage.record("source_is_multiline", source_is_multiline);

    let mut deferred_closing_comment = None;
    if !config.closing_paren_newline {
        deferred_closing_comment = extract_deferred_closing_comment(&mut arguments);
    }
    let has_deferred_closing_comment = deferred_closing_comment.is_some();
    let avoid_inline_compaction = has_deferred_closing_comment
        || (!config.closing_paren_newline && source_is_multiline && cmd.trailing_comment.is_some());

    // Format arguments using wrapping cascade controls (threshold, magic newline, style).
    let force_one_per_line = config.wrap_arg_threshold > 0
        && count_wrap_arguments(&arguments) > config.wrap_arg_threshold as usize;
    let magic_trailing_newline =
        config.magic_trailing_newline && has_magic_trailing_newline_signal(cmd, source, &arguments);
    let allow_single_line_by_style = match config.wrap_style {
        WrapStyle::Cascade => true,
        WrapStyle::Vertical => count_wrap_arguments(&arguments) <= 2,
    };
    let allow_single_line = allow_single_line_by_style
        && !(force_one_per_line || magic_trailing_newline || avoid_inline_compaction)
        && !(source_is_multiline && has_keyword_args);
    if !arguments.is_empty() {
        let single_line = try_single_line(
            &formatted_name,
            &arguments,
            config,
            indent_depth,
            allow_single_line,
            cmd_kind.as_ref(),
        );
        if let Some(single) = single_line {
            if single_line_paren_spacing.after_open {
                items.push_space();
            }
            items.extend(single);
            if single_line_paren_spacing.before_close {
                items.push_space();
            }
        } else {
            match cmd_kind {
                Some(CommandKind::ConditionSyntax) => {
                    let is_condition_closer = formatted_name.eq_ignore_ascii_case("endif")
                        || formatted_name.eq_ignore_ascii_case("endwhile")
                        || formatted_name.eq_ignore_ascii_case("else");
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
                    let suppress_keyword_inline = force_one_per_line
                        || avoid_inline_compaction
                        || (config.wrap_arg_threshold > 0 && magic_trailing_newline);
                    let allow_keyword_inline =
                        matches!(config.wrap_style, WrapStyle::Cascade) && !suppress_keyword_inline;
                    let allow_opening_arg_packing = allow_keyword_inline;
                    items.extend(gen_known_multi_line(
                        &formatted_name,
                        &arguments,
                        spec,
                        config,
                        indent_depth,
                        allow_keyword_inline,
                        allow_opening_arg_packing,
                        config.first_arg_same_line,
                    ));
                }
                None => {
                    // Unknown command with customKeywords — use empty spec.
                    let suppress_keyword_inline = force_one_per_line
                        || avoid_inline_compaction
                        || (config.wrap_arg_threshold > 0 && magic_trailing_newline);
                    let allow_keyword_inline =
                        matches!(config.wrap_style, WrapStyle::Cascade) && !suppress_keyword_inline;
                    let allow_opening_arg_packing = allow_keyword_inline;
                    items.extend(gen_known_multi_line(
                        &formatted_name,
                        &arguments,
                        &EMPTY_SPEC,
                        config,
                        indent_depth,
                        allow_keyword_inline,
                        allow_opening_arg_packing,
                        config.first_arg_same_line,
                    ));
                }
            }
        }
    }

    items.push_str_runtime_width_computed(")");
    if let Some(comment) = deferred_closing_comment {
        push_comment_gap(&mut items, config.comment_gap);
        items.extend(ir_helpers::gen_from_raw_string(&comment));
    }

    // Trailing comment
    if let Some(comment_span) = &cmd.trailing_comment {
        push_comment_gap(&mut items, config.comment_gap);
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
        push_comment_gap(&mut items, config.comment_gap);
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
        if matches!(arg, Argument::Bracket(_) | Argument::Quoted(_)) && arg_text.contains('\n') {
            emit_text_preserving_line_endings(items, arg_text);
        } else {
            items.extend(ir_helpers::gen_from_raw_string(arg_text));
        }
        pos = arg_end;
    }

    // If there are newlines before the closing paren, emit a single newline.
    // The closing paren will be at block indent level (handled by gen_file).
    let trailing = &source[pos..content_end];
    if trailing.contains('\n') {
        items.push_signal(Signal::NewLine);
    }
}

fn emit_text_preserving_line_endings(items: &mut PrintItems, text: &str) {
    if !text.contains('\n') {
        items.extend(ir_helpers::gen_from_raw_string(text));
        return;
    }

    items.push_signal(Signal::StartIgnoringIndent);
    let bytes = text.as_bytes();
    let mut segment_start = 0;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            let mut segment_end = i;
            let had_cr = segment_end > segment_start && bytes[segment_end - 1] == b'\r';
            if had_cr {
                segment_end -= 1;
            }

            if segment_end > segment_start {
                items.push_string(text[segment_start..segment_end].to_string());
            }
            if had_cr {
                items.push_string("\r".to_string());
            }
            items.push_signal(Signal::NewLine);
            segment_start = i + 1;
        }
        i += 1;
    }

    if segment_start < text.len() {
        items.push_string(text[segment_start..].to_string());
    }

    items.push_signal(Signal::FinishIgnoringIndent);
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

fn has_multiline_verbatim_argument(arguments: &[Argument], source: &str) -> bool {
    arguments.iter().any(|arg| match arg {
        Argument::Bracket(span) | Argument::Quoted(span) => {
            let text = span.text(source);
            text.contains('\n') || text.contains('\r')
        }
        Argument::ParenGroup { arguments } => has_multiline_verbatim_argument(arguments, source),
        Argument::Unquoted(_) | Argument::LineComment(_) | Argument::BracketComment(_) => false,
    })
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
    /// Whether a source newline separates this arg from the previous one.
    /// Used by `alignArgGroups` to preserve source-level token grouping.
    new_line_before: bool,
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
    let mut result = Vec::with_capacity(args.len());
    let arg_ranges: Vec<(usize, usize)> = args.iter().map(arg_source_range).collect();
    let mut i = 0;

    while i < args.len() {
        let preserve_source_blank_lines =
            config.align_arg_groups || !matches!(config.sort_arguments, SortArguments::Disabled);
        let blank_line_before = if preserve_source_blank_lines && i > 0 {
            let prev_end = arg_ranges[i - 1].1;
            let current_start = arg_ranges[i].0;
            has_blank_line_between(source, prev_end, current_start)
        } else {
            false
        };
        let new_line_before = if i > 0 {
            let prev_end = arg_ranges[i - 1].1;
            let current_start = arg_ranges[i].0;
            source[prev_end..current_start].contains('\n')
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
                    new_line_before,
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
                    new_line_before,
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
                    new_line_before,
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
                    new_line_before,
                });
            }
            Argument::LineComment(span) => {
                let comment_text = span.text(source).trim_end().to_string();
                // Only attach to previous arg if on the same source line
                let same_line = if i > 0 {
                    let prev_end = arg_ranges[i - 1].1;
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
                    new_line_before,
                });
            }
            Argument::BracketComment(span) => {
                let comment_text = span.text(source).to_string();
                // Only attach to previous arg if on the same source line
                let same_line = if i > 0 {
                    let prev_end = arg_ranges[i - 1].1;
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
                    new_line_before,
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

fn arg_inline_text(arg: &FormattedArg) -> Cow<'_, str> {
    if arg.is_paren_group {
        Cow::Owned(format_paren_group_inline(&arg.paren_inner))
    } else {
        Cow::Borrowed(&arg.text)
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

fn count_wrap_arguments(args: &[FormattedArg]) -> usize {
    args.iter().filter(|arg| !arg.text.starts_with('#')).count()
}

fn has_magic_trailing_newline_signal(
    cmd: &CommandInvocation,
    source: &str,
    args: &[FormattedArg],
) -> bool {
    if args.is_empty() {
        return false;
    }
    let before_close = &source[..cmd.close_paren.start];
    let trimmed_before_close = before_close.trim_end_matches([' ', '\t']);
    trimmed_before_close.ends_with('\n')
}

fn can_pack_args_on_line(
    prefix_width: usize,
    args: &[&FormattedArg],
    config: &Configuration,
) -> bool {
    if args.is_empty() {
        return true;
    }

    if args.iter().any(|arg| {
        arg.text.starts_with('#')
            || arg.text.contains('\n')
            || (arg.trailing_comment.is_some() && !arg.trailing_is_bracket)
    }) {
        return false;
    }

    let args_width: usize =
        args.iter().map(|arg| arg_width(arg)).sum::<usize>() + args.len().saturating_sub(1);
    prefix_width + args_width <= config.line_width as usize
}

// ===========================================================================
// Single-line attempt
// ===========================================================================

fn try_single_line(
    cmd_name: &str,
    args: &[FormattedArg],
    config: &Configuration,
    indent_depth: u32,
    allow_single_line: bool,
    cmd_kind: Option<&CommandKind>,
) -> Option<PrintItems> {
    if !allow_single_line {
        return None;
    }

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

    // Only reject single-line if the command actually has section-bearing keywords
    // that would produce blank lines in multi-line layout. Commands like
    // cmake_minimum_required(VERSION 3.28 FATAL_ERROR) have multiple keywords
    // but no section structure, so they should still collapse to a single line.
    if config.blank_line_between_sections
        && args.iter().filter(|arg| arg.is_keyword).take(2).count() >= 2
    {
        let has_sections = match cmd_kind {
            Some(CommandKind::Known(spec)) => {
                // Command has explicit sections, or has custom keywords
                // that act as section dividers.
                !spec.sections.is_empty()
                    || args.iter().any(|arg| {
                        arg.is_keyword
                            && (is_section_keyword(arg, spec) || is_custom_keyword(arg, config))
                    })
            }
            Some(CommandKind::ConditionSyntax) | None => {
                // Condition syntax (if/while/elseif) and unknown commands:
                // only reject if custom keywords create sections.
                !config.custom_keywords.is_empty()
                    && args
                        .iter()
                        .any(|arg| arg.is_keyword && is_custom_keyword(arg, config))
            }
        };
        if has_sections {
            return None;
        }
    }

    // Calculate total width including file-level indentation.
    // Width is computed BEFORE building cased strings — ASCII casing preserves length,
    // so arg_inline_text().len() is the final width regardless of casing.
    let base_indent = indent_depth as usize * config.indent_width as usize;
    let close_overhead = 1; // ")"
    let line_width = config.line_width as usize;

    // Pre-compute per-arg widths to enable early rejection without allocating cased strings.
    let arg_widths: Vec<usize> = args.iter().map(|a| arg_inline_text(a).len()).collect();
    let args_width: usize =
        arg_widths.iter().sum::<usize>() + if args.len() > 1 { args.len() - 1 } else { 0 };
    let total = base_indent + cmd_name.len() + 1 + args_width + close_overhead;

    let keep_condition_header_inline = cmd_name.eq_ignore_ascii_case("if")
        || cmd_name.eq_ignore_ascii_case("while")
        || cmd_name.eq_ignore_ascii_case("elseif");
    let has_keyword_args = args.iter().any(|arg| arg.is_keyword);
    let has_unbreakable_long_token = cmd_name.len() > line_width
        || arg_widths.iter().zip(args.iter()).any(|(&w, a)| {
            w > line_width && {
                let t = arg_inline_text(a);
                memchr::memmem::find(t.as_bytes(), b"$<").is_none()
            }
        });
    let indentation_overflow = base_indent > line_width;
    let deep_indentation = base_indent.saturating_mul(2) >= line_width;
    if !keep_condition_header_inline
        && ((total > line_width && !has_unbreakable_long_token)
            || (!has_keyword_args && total == line_width))
        && !indentation_overflow
        && !deep_indentation
    {
        return None;
    }

    // Width check passed — build cased strings for the output.
    // Apply keyword/literal casing per §4.4:
    // 1. Keyword tokens get keywordCase
    // 2. Literal tokens (not keywords) get literalCase
    // 3. Other tokens preserved as-is
    let args_text: Vec<String> = if let Some(ck) = cmd_kind {
        match ck {
            CommandKind::Known(spec) => {
                let mut is_keyword_position = vec![false; args.len()];
                for idx in compute_keyword_positions(args, spec) {
                    is_keyword_position[idx] = true;
                }
                args.iter()
                    .enumerate()
                    .map(|(i, a)| {
                        let t = arg_inline_text(a);
                        if is_keyword_position[i] {
                            apply_keyword_case_owned(t.into_owned(), config.keyword_case)
                        } else if !a.is_bracket
                            && !t.starts_with('"')
                            && !t.starts_with('#')
                            && is_literal_token(&t)
                        {
                            apply_literal_case_owned(t.into_owned(), config.literal_case)
                        } else {
                            t.into_owned()
                        }
                    })
                    .collect()
            }
            CommandKind::ConditionSyntax => args
                .iter()
                .map(|a| {
                    let t = arg_inline_text(a);
                    if a.is_keyword {
                        apply_keyword_case_owned(t.into_owned(), config.keyword_case)
                    } else if !a.is_bracket
                        && !t.starts_with('"')
                        && !t.starts_with('#')
                        && is_literal_token(&t)
                    {
                        apply_literal_case_owned(t.into_owned(), config.literal_case)
                    } else {
                        t.into_owned()
                    }
                })
                .collect(),
        }
    } else if !config.custom_keywords.is_empty() {
        args.iter()
            .map(|a| {
                let t = arg_inline_text(a);
                if a.is_keyword {
                    apply_keyword_case_owned(t.into_owned(), config.keyword_case)
                } else if !a.is_bracket
                    && !t.starts_with('"')
                    && !t.starts_with('#')
                    && is_literal_token(&t)
                {
                    apply_literal_case_owned(t.into_owned(), config.literal_case)
                } else {
                    t.into_owned()
                }
            })
            .collect()
    } else {
        args.iter()
            .map(|a| arg_inline_text(a).into_owned())
            .collect()
    };

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
#[derive(Debug, Clone)]
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
        let is_once = kw_type.is_some() && is_once_keyword_text(&args[i].text, spec);
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
                        let inner_is_blocked_once = inner_kw.is_some()
                            && once_keyword_seen
                            && is_once_keyword_text(&args[i].text, spec);
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
                KwType::Group(front_count, _group_sub_kws) => {
                    // Group keyword (e.g., FILE_SET <name> TYPE ... BASE_DIRS ... FILES ...)
                    // Consume the keyword's front positional args, then consume all
                    // remaining args until the next outer keyword or section keyword.
                    i += 1;
                    let mut consumed = 0usize;
                    while consumed < front_count && i < args.len() {
                        i += 1;
                        consumed += 1;
                    }
                    while i < args.len() {
                        let inner_kw = get_keyword_type(&args[i], spec);
                        let inner_is_section = is_section_keyword(&args[i], spec);
                        // Another top-level keyword (including another Group) breaks the group.
                        // Sub-keywords (TYPE, BASE_DIRS, etc.) are NOT in the outer spec,
                        // so they are consumed as values.
                        if inner_kw.is_some() || inner_is_section {
                            break;
                        }
                        i += 1;
                    }
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

#[inline]
fn is_once_keyword_text(text: &str, spec: &CommandSpec) -> bool {
    spec.once_keywords
        .iter()
        .any(|&keyword| keyword.eq_ignore_ascii_case(text))
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
            && spec.sections.is_empty();
        // Skip once_keywords that have already been consumed
        let is_once = kw_type.is_some() && is_once_keyword_text(&main_args[i].text, spec);
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
                            && spec.sections.is_empty();

                        // Already-seen once_keywords are not treated as keywords
                        let inner_is_blocked_once = inner_kw.is_some()
                            && once_keyword_seen
                            && is_once_keyword_text(&main_args[i].text, spec);

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
                KwType::Group(front_count, _group_sub_kws) => {
                    // Group keyword: consume front positional args + all remaining
                    // args until the next outer keyword/section.
                    let kw = &main_args[i];
                    let mut values: Vec<&'a FormattedArg> = Vec::new();
                    i += 1;
                    let mut consumed = 0usize;
                    while consumed < front_count && i < main_args.len() {
                        values.push(&main_args[i]);
                        i += 1;
                        consumed += 1;
                    }
                    while i < main_args.len() {
                        let inner_kw = get_keyword_type(&main_args[i], spec);
                        let inner_is_section = is_section_keyword(&main_args[i], spec);
                        let inner_is_cmd_line = is_cmd_line_keyword(&main_args[i], spec);
                        let inner_is_custom = is_custom_keyword(&main_args[i], config);
                        let inner_is_generic_keyword = main_args[i].is_keyword
                            && inner_kw.is_none()
                            && !inner_is_section
                            && !inner_is_custom
                            && spec.sections.is_empty();
                        if inner_kw.is_some()
                            || inner_is_section
                            || inner_is_cmd_line
                            || inner_is_custom
                            || inner_is_generic_keyword
                        {
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

/// Flatten non-section keyword groups into positional streams for column alignment.
///
/// When `alignArgGroups` is enabled, each non-section, non-group
/// `Keyword { kw, values }` group is converted to a `Positional` group containing
/// `[kw, ...values]`. If the immediately preceding group is an original `Positional`
/// (not one created by keyword flattening), its args are prepended to the keyword's
/// new positional group. Section keywords, group keywords, and `CmdLineKeyword`
/// groups are preserved as-is.
fn flatten_keyword_groups_for_alignment<'a>(
    groups: &[ArgGroup<'a>],
    spec: &CommandSpec,
    config: &Configuration,
) -> Vec<ArgGroup<'a>> {
    let mut result: Vec<ArgGroup<'a>> = Vec::new();
    // Track whether the last entry in result was an original Positional
    // (from the input) vs one synthesized from keyword flattening.
    let mut last_is_original_positional = false;

    for group in groups {
        match group {
            ArgGroup::Keyword { keyword, values } => {
                let is_section = is_section_keyword(keyword, spec)
                    || is_custom_keyword(keyword, config)
                    || (config.blank_line_between_sections
                        && keyword.is_keyword
                        && spec.sections.is_empty());
                let is_group_keyword =
                    matches!(get_keyword_type(keyword, spec), Some(KwType::Group(..)));
                if is_section || is_group_keyword {
                    result.push(ArgGroup::Keyword {
                        keyword,
                        values: values.clone(),
                    });
                    last_is_original_positional = false;
                } else {
                    // Convert keyword + values into a positional line.
                    let mut merged = vec![*keyword];
                    merged.extend(values.iter().copied());
                    // If the preceding entry is an original positional group,
                    // absorb it: prepend its args so the positional + keyword
                    // tokens form one line (e.g., `MyLib RUNTIME DEST bin`).
                    if last_is_original_positional {
                        if let Some(ArgGroup::Positional(prev)) = result.last_mut() {
                            let mut combined = std::mem::take(prev);
                            combined.extend(merged);
                            *prev = combined;
                        }
                    } else {
                        result.push(ArgGroup::Positional(merged));
                    }
                    last_is_original_positional = false;
                }
            }
            ArgGroup::Positional(args) => {
                result.push(ArgGroup::Positional(args.clone()));
                last_is_original_positional = true;
            }
            ArgGroup::CmdLineKeyword { keyword, value } => {
                result.push(ArgGroup::CmdLineKeyword {
                    keyword,
                    value: *value,
                });
                last_is_original_positional = false;
            }
        }
    }

    result
}

fn flatten_target_sources_sections_for_alignment<'a>(
    cmd_name: &str,
    groups: &[ArgGroup<'a>],
    spec: &CommandSpec,
) -> Option<Vec<ArgGroup<'a>>> {
    if !cmd_name.eq_ignore_ascii_case("target_sources") {
        return None;
    }

    let mut merged: Vec<&FormattedArg> = Vec::new();
    for group in groups {
        let ArgGroup::Keyword { keyword, values } = group else {
            return None;
        };
        if !is_section_keyword(keyword, spec) {
            return None;
        }
        if let Some(sub_kws) = get_section_keywords(&keyword.text, spec)
            && values
                .iter()
                .any(|value| is_text_in_keyword_list(&value.text, sub_kws))
        {
            return None;
        }

        merged.push(*keyword);
        merged.extend(values.iter().copied());
    }

    if merged.is_empty() {
        None
    } else {
        Some(vec![ArgGroup::Positional(merged)])
    }
}

fn reshape_install_target_groups_for_alignment<'a>(
    cmd_name: &str,
    groups: &[ArgGroup<'a>],
) -> Option<Vec<ArgGroup<'a>>> {
    if !cmd_name.eq_ignore_ascii_case("install") || groups.len() < 2 {
        return None;
    }

    let ArgGroup::Positional(targets_intro) = groups.first()? else {
        return None;
    };
    if targets_intro.len() != 2 || !targets_intro[0].text.eq_ignore_ascii_case("TARGETS") {
        return None;
    }

    let mut current_target = targets_intro[1];
    let mut row_stream: Vec<&FormattedArg> = Vec::new();

    for (index, group) in groups.iter().enumerate().skip(1) {
        let ArgGroup::Keyword { keyword, values } = group else {
            return None;
        };
        if values.is_empty() || !values[0].text.eq_ignore_ascii_case("DESTINATION") {
            return None;
        }

        let mut row_values = values.clone();
        let has_following_section = groups[index + 1..]
            .iter()
            .any(|entry| matches!(entry, ArgGroup::Keyword { .. }));
        let next_target = if has_following_section {
            row_values.pop()
        } else {
            None
        };
        if has_following_section && next_target.is_none() {
            return None;
        }

        row_stream.push(current_target);
        row_stream.push(*keyword);
        row_stream.extend(row_values);

        if let Some(next_target) = next_target {
            current_target = next_target;
        }
    }

    if row_stream.is_empty() {
        return None;
    }

    Some(vec![
        ArgGroup::Positional(vec![targets_intro[0]]),
        ArgGroup::Positional(row_stream),
    ])
}

#[derive(Default)]
struct PositionalAlignmentProfile {
    keyword_first_col_width: Option<usize>,
    column_widths_by_count: std::collections::BTreeMap<usize, Vec<usize>>,
}

fn is_one_line_alignment_candidate(cmd_name: &str, args: &[&FormattedArg]) -> bool {
    !args.is_empty()
        && args
            .iter()
            .all(|arg| !arg.text.starts_with('#') && !arg.text.contains('\n'))
        && (cmd_name.eq_ignore_ascii_case("install") || is_keyword_like_value(args[0]))
}

fn build_positional_alignment_profile(
    cmd_name: &str,
    groups: &[ArgGroup<'_>],
) -> Option<PositionalAlignmentProfile> {
    let mut profile = PositionalAlignmentProfile::default();
    let mut candidate_count = 0usize;
    let mut keyword_line_count = 0usize;
    let mut max_keyword_width = 0usize;

    for group in groups {
        let ArgGroup::Positional(args) = group else {
            continue;
        };
        if !is_one_line_alignment_candidate(cmd_name, args) {
            continue;
        }

        candidate_count += 1;
        if is_keyword_like_value(args[0]) {
            keyword_line_count += 1;
            max_keyword_width = max_keyword_width.max(token_visual_width(args[0]));
        }
        if args.len() >= 2 {
            let widths = profile
                .column_widths_by_count
                .entry(args.len())
                .or_insert_with(|| vec![0; args.len()]);
            for (index, arg) in args.iter().enumerate() {
                widths[index] = widths[index].max(token_visual_width(arg));
            }
        }
    }

    if candidate_count < 2 {
        return None;
    }
    if keyword_line_count >= 2 {
        // Keep an explicit visual gap between keyword columns and values.
        profile.keyword_first_col_width = Some(max_keyword_width + 1);
    }

    Some(profile)
}

fn emit_aligned_single_positional_line(
    items: &mut PrintItems,
    args: &[&FormattedArg],
    config: &Configuration,
    profile: &PositionalAlignmentProfile,
    cmd_name: &str,
) {
    items.push_signal(Signal::NewLine);
    for (index, arg) in args.iter().enumerate() {
        emit_arg(items, arg, config);
        if index + 1 >= args.len() {
            continue;
        }

        let token_width = token_visual_width(arg);
        let mut spaces = 1usize;
        if index == 0
            && is_keyword_like_value(args[0])
            && let Some(width) = profile.keyword_first_col_width
        {
            spaces = spaces.max(width.saturating_sub(token_width) + 1);
        }
        if let Some(column_widths) = profile.column_widths_by_count.get(&args.len()) {
            spaces = spaces.max(column_widths[index].saturating_sub(token_width) + 1);
        }
        if index == 0 && cmd_name.eq_ignore_ascii_case("install") {
            spaces += 1;
        }
        for _ in 0..spaces {
            items.push_space();
        }
    }
}

fn emit_install_target_rows(
    items: &mut PrintItems,
    args: &[&FormattedArg],
    config: &Configuration,
) -> bool {
    if args.len() < 4 || !args.len().is_multiple_of(4) {
        return false;
    }
    let mut rows: Vec<&[&FormattedArg]> = Vec::new();
    for chunk in args.chunks(4) {
        if !chunk[2].text.eq_ignore_ascii_case("DESTINATION") {
            return false;
        }
        rows.push(chunk);
    }

    let mut column_widths = [0usize; 4];
    for row in &rows {
        for (index, token) in row.iter().enumerate() {
            column_widths[index] = column_widths[index].max(token_visual_width(token));
        }
    }

    for row in rows {
        items.push_signal(Signal::NewLine);
        for (index, token) in row.iter().enumerate() {
            emit_arg(items, token, config);
            if index + 1 >= row.len() {
                continue;
            }

            let mut spaces = column_widths[index].saturating_sub(token_visual_width(token)) + 1;
            if index == 0 {
                // Keep an extra gap between TARGET name and artifact section keyword.
                spaces += 1;
            }
            for _ in 0..spaces {
                items.push_space();
            }
        }
    }

    true
}

fn emit_flattened_target_sources_sections(
    items: &mut PrintItems,
    args: &[&FormattedArg],
    config: &Configuration,
    spec: &CommandSpec,
) {
    let mut lines: Vec<Vec<&FormattedArg>> = Vec::new();
    let mut current: Vec<&FormattedArg> = Vec::new();
    for arg in args {
        if arg.new_line_before && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        current.push(*arg);
    }
    if !current.is_empty() {
        lines.push(current);
    }

    let value_column_width = lines
        .iter()
        .filter(|line| line.len() >= 2 && !is_section_keyword(line[0], spec))
        .map(|line| token_visual_width(line[0]))
        .max()
        .unwrap_or(0);
    let continuation_indent = config.effective_continuation_indent_width() as usize;

    for line in lines {
        items.push_signal(Signal::NewLine);
        let is_section_header = line.len() == 1 && is_section_keyword(line[0], spec);
        if !is_section_header {
            for _ in 0..continuation_indent {
                items.push_space();
            }
        }

        for (index, arg) in line.iter().enumerate() {
            emit_arg(items, arg, config);
            if index + 1 >= line.len() {
                continue;
            }

            let mut spaces = 1usize;
            if !is_section_header && index == 0 && value_column_width > 0 {
                spaces = spaces.max(value_column_width.saturating_sub(token_visual_width(arg)) + 1);
            }
            for _ in 0..spaces {
                items.push_space();
            }
        }
    }
}

// ===========================================================================
// Known command multi-line formatting
// ===========================================================================

#[allow(clippy::too_many_arguments)]
fn gen_known_multi_line(
    cmd_name: &str,
    arguments: &[FormattedArg],
    spec: &CommandSpec,
    config: &Configuration,
    indent_depth: u32,
    allow_keyword_inline: bool,
    allow_opening_arg_packing: bool,
    first_arg_same_line: bool,
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
        && !config.align_arg_groups
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

    let mut opening_pair_values: Option<Vec<&FormattedArg>> = None;
    if cmd_name.eq_ignore_ascii_case("set_target_properties")
        && config.align_property_values
        && let Some(ArgGroup::Keyword { keyword, values }) = groups.first()
        && keyword.text.eq_ignore_ascii_case("PROPERTIES")
    {
        front_pos.push(keyword);
        opening_pair_values = Some(values.clone());
        groups.remove(0);
    }

    if config.align_arg_groups {
        groups = flatten_keyword_groups_for_alignment(&groups, spec, config);
        if let Some(section_flattened) =
            flatten_target_sources_sections_for_alignment(cmd_name, &groups, spec)
        {
            groups = section_flattened;
        }
        if let Some(install_rows) = reshape_install_target_groups_for_alignment(cmd_name, &groups) {
            groups = install_rows;
        }
        if cmd_name.eq_ignore_ascii_case("install")
            && front_pos.is_empty()
            && let Some(ArgGroup::Positional(targets_keyword)) = groups.first()
            && targets_keyword.len() == 1
            && targets_keyword[0].text.eq_ignore_ascii_case("TARGETS")
        {
            front_pos.push(targets_keyword[0]);
            groups.remove(0);
        }
    }

    let has_keyword_groups = groups
        .iter()
        .any(|group| matches!(group, ArgGroup::Keyword { .. }));
    let mut opening_args = front_pos;
    if !config.align_arg_groups
        && has_keyword_groups
        && let Some(ArgGroup::Positional(args)) = groups.first()
    {
        opening_args.extend(args.iter().copied());
        groups.remove(0);
    }

    let command_indent = indent_depth as usize * config.indent_width as usize;
    let base_indent = (indent_depth as usize + 1) * config.indent_width as usize;

    let mut inner = PrintItems::new();
    let mut last_on_opening_line = false;

    let force_two_install_opening = cmd_name.eq_ignore_ascii_case("install")
        && opening_args.len() >= 2
        && opening_args[0].text.eq_ignore_ascii_case("TARGETS");
    let force_properties_opening =
        cmd_name.eq_ignore_ascii_case("set_target_properties") && opening_pair_values.is_some();
    let force_one_per_line = config.wrap_arg_threshold > 0
        && count_wrap_arguments(arguments) > config.wrap_arg_threshold as usize;
    let opening_line_overflow = first_arg_same_line
        && opening_args
            .first()
            .map(|first| {
                command_indent + cmd_name.len() + 1 + arg_width(first) > config.line_width as usize
            })
            .unwrap_or(false);
    if first_arg_same_line {
        if let Some((first, rest)) = opening_args.split_first() {
            if first.is_keyword {
                emit_kw_arg(&mut inner, first, config);
            } else {
                emit_arg(&mut inner, first, config);
            }
            last_on_opening_line = true;

            let can_pack_rest = !force_one_per_line
                && (force_two_install_opening
                    || force_properties_opening
                    || allow_opening_arg_packing
                    || (allow_keyword_inline && has_keyword_groups))
                && can_pack_args_on_line(
                    command_indent + cmd_name.len() + 1,
                    &opening_args,
                    config,
                );

            for arg in rest {
                let arg = *arg;
                if can_pack_rest {
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
        }
    } else if !opening_args.is_empty() {
        let can_pack_all = !force_one_per_line
            && (force_two_install_opening
                || force_properties_opening
                || allow_opening_arg_packing
                || (allow_keyword_inline && has_keyword_groups))
            && can_pack_args_on_line(base_indent, &opening_args, config);
        if can_pack_all {
            inner.push_signal(Signal::NewLine);
            for (idx, arg) in opening_args.iter().enumerate() {
                let arg = *arg;
                if idx > 0 {
                    inner.push_space();
                }
                if arg.is_keyword {
                    emit_kw_arg(&mut inner, arg, config);
                } else {
                    emit_arg(&mut inner, arg, config);
                }
            }
        } else {
            for arg in &opening_args {
                inner.push_signal(Signal::NewLine);
                if arg.is_keyword {
                    emit_kw_arg(&mut inner, arg, config);
                } else {
                    emit_arg(&mut inner, arg, config);
                }
            }
        }
        last_on_opening_line = false;
    }

    if let Some(values) = opening_pair_values.as_ref() {
        emit_pair_values(&mut inner, values.as_slice(), config, base_indent, false);
        last_on_opening_line = false;
    }

    let mut emitted_section_groups = 0usize;
    let mut expanded_keyword_group_seen = opening_line_overflow || opening_pair_values.is_some();
    let disable_inline_after_expansion = config.line_width <= 40;
    let force_expanded_keyword_layout = config.blank_line_between_sections
        && (!spec.sections.is_empty()
            || !spec.keywords.is_empty()
            || !config.custom_keywords.is_empty());

    let positional_alignment_profile = if config.align_arg_groups {
        build_positional_alignment_profile(cmd_name, &groups)
    } else {
        None
    };

    // Emit keyword groups
    for group in groups.iter() {
        last_on_opening_line = false;
        match group {
            ArgGroup::Positional(args) => {
                if force_one_per_line {
                    emit_values_with_genex_with_indent(
                        &mut inner,
                        args.as_slice(),
                        config,
                        false,
                        config.effective_continuation_indent_width() as usize,
                        false,
                        false,
                        0, // pack_tokens=false, column_start irrelevant
                    );
                } else if config.align_arg_groups
                    && cmd_name.eq_ignore_ascii_case("install")
                    && emit_install_target_rows(&mut inner, args.as_slice(), config)
                {
                    // install(TARGETS ...) artifact rows emitted above.
                } else if config.align_arg_groups
                    && cmd_name.eq_ignore_ascii_case("target_sources")
                    && args
                        .first()
                        .is_some_and(|arg| is_section_keyword(arg, spec))
                {
                    emit_flattened_target_sources_sections(
                        &mut inner,
                        args.as_slice(),
                        config,
                        spec,
                    );
                } else if is_one_line_alignment_candidate(cmd_name, args.as_slice()) {
                    if let Some(profile) = positional_alignment_profile.as_ref() {
                        emit_aligned_single_positional_line(
                            &mut inner,
                            args.as_slice(),
                            config,
                            profile,
                            cmd_name,
                        );
                    } else if spec.flow_positional {
                        emit_flow_values(&mut inner, args.as_slice(), config, base_indent, false);
                    } else {
                        emit_values_with_genex_with_indent(
                            &mut inner,
                            args.as_slice(),
                            config,
                            false,
                            config.effective_continuation_indent_width() as usize,
                            false,
                            config.align_arg_groups
                                || args
                                    .iter()
                                    .filter(|arg| arg.new_line_before)
                                    .take(2)
                                    .count()
                                    >= 2,
                            base_indent,
                        );
                    }
                } else if spec.flow_positional {
                    emit_flow_values(&mut inner, args.as_slice(), config, base_indent, false);
                } else {
                    emit_values_with_genex_with_indent(
                        &mut inner,
                        args.as_slice(),
                        config,
                        false,
                        config.effective_continuation_indent_width() as usize,
                        false,
                        config.align_arg_groups
                            || args
                                .iter()
                                .filter(|arg| arg.new_line_before)
                                .take(2)
                                .count()
                                >= 2,
                        base_indent,
                    );
                }
            }
            ArgGroup::Keyword { keyword, values } => {
                let section_keyword = is_section_keyword(keyword, spec)
                    || is_custom_keyword(keyword, config)
                    || (keyword.is_keyword && spec.sections.is_empty());
                if config.blank_line_between_sections
                    && section_keyword
                    && emitted_section_groups > 0
                {
                    inner.push_signal(Signal::NewLine);
                }
                let keyword_inline_allowed = allow_keyword_inline
                    && !force_expanded_keyword_layout
                    && !(disable_inline_after_expansion && expanded_keyword_group_seen);
                let expanded = emit_keyword_group(
                    &mut inner,
                    keyword,
                    values.as_slice(),
                    spec,
                    config,
                    base_indent,
                    cmd_name.eq_ignore_ascii_case("export"),
                    keyword_inline_allowed,
                    force_one_per_line,
                );
                expanded_keyword_group_seen |= expanded;
                if section_keyword {
                    emitted_section_groups += 1;
                }
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
#[allow(clippy::too_many_arguments)]
fn emit_keyword_group(
    items: &mut PrintItems,
    keyword: &FormattedArg,
    values: &[&FormattedArg],
    spec: &CommandSpec,
    config: &Configuration,
    base_indent: usize,
    allow_plain_section_inline: bool,
    allow_keyword_inline: bool,
    force_one_per_line: bool,
) -> bool {
    // Check for compound keyword (e.g., QUERY WINDOWS_REGISTRY)
    if let Some(first_val) = values.first()
        && is_compound_keyword(&keyword.text, &first_val.text, spec)
    {
        let compound_text = format!(
            "{} {}",
            if keyword.is_keyword {
                apply_keyword_case(&keyword.text, config.keyword_case)
            } else {
                Cow::Borrowed(keyword.text.as_str())
            },
            if first_val.is_keyword || get_keyword_type(first_val, spec).is_some() {
                apply_keyword_case(&first_val.text, config.keyword_case)
            } else {
                Cow::Borrowed(first_val.text.as_str())
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
            new_line_before: false,
        };
        return emit_keyword_group(
            items,
            &compound_arg,
            &values[1..],
            spec,
            config,
            base_indent,
            allow_plain_section_inline,
            allow_keyword_inline,
            force_one_per_line,
        );
    }

    if values.is_empty() {
        // Option keyword: alone on its line
        items.push_signal(Signal::NewLine);
        emit_kw_arg(items, keyword, config);
        return true;
    }

    // Check if this is a section keyword with sub-keywords.
    let section_kws = get_section_keywords(&keyword.text, spec);
    let section_front = get_section_front_positional(&keyword.text, spec);
    let is_section_kw = section_kws.is_some();
    let plain_section_keyword = match section_kws {
        Some(kws) => kws.is_empty(),
        None => false,
    };
    let treat_as_regular_keyword = !is_section_kw || plain_section_keyword;
    let has_section_tail_values = is_section_kw
        && !plain_section_keyword
        && section_front > 0
        && values.len() > section_front;

    let continuation_is_explicit = config.continuation_indent_width.is_some();
    let preserve_inline_keyword_layout = (config.sort_arguments_explicit
        || config.sort_keyword_sections_explicit)
        && !allow_plain_section_inline
        && (is_section_kw
            || (keyword.is_keyword && spec.sections.is_empty())
            || is_custom_keyword(keyword, config));
    // Try inline: keyword + all values on one line.
    let inline_width = compute_keyword_inline_width(keyword, values);
    let can_inline_content = !values.iter().any(|v| v.text.contains('\n'))
        && values
            .iter()
            .all(|v| v.trailing_comment.is_none() || v.trailing_is_bracket)
        && (keyword.trailing_comment.is_none() || keyword.trailing_is_bracket)
        && !values.iter().any(|v| v.text.starts_with('#'))
        && !values
            .iter()
            .any(|v| memchr::memmem::find(v.text.as_bytes(), b"$<").is_some());
    if !force_one_per_line
        && allow_keyword_inline
        && !preserve_inline_keyword_layout
        && treat_as_regular_keyword
        && base_indent + inline_width <= config.line_width as usize
        && can_inline_content
        && !has_section_tail_values
    {
        items.push_signal(Signal::NewLine);
        let kw_text = if keyword.is_keyword {
            apply_keyword_case(&keyword.text, config.keyword_case)
        } else {
            Cow::Borrowed(keyword.text.as_str())
        };
        items.push_string(kw_text.into_owned());
        for val in values {
            items.push_space();
            let val_text = arg_inline_text(val);
            items.extend(ir_helpers::gen_from_raw_string(&val_text));
        }
        if let Some(comment) = &keyword.trailing_comment {
            items.push_space();
            items.extend(ir_helpers::gen_from_raw_string(comment));
        }
        return false;
    }

    // Expanded: keyword on its own line, values indented below.
    items.push_signal(Signal::NewLine);

    if let Some(sub_kws) = section_kws
        && !plain_section_keyword
    {
        let has_subkeyword_values = values
            .iter()
            .any(|v| is_text_in_keyword_list(&v.text, sub_kws));

        let has_group_subkeywords = sub_kws
            .iter()
            .any(|(_, kw_type)| matches!(kw_type, KwType::Group(..)));
        let can_inline_section_values = !force_one_per_line
            && allow_keyword_inline
            && !preserve_inline_keyword_layout
            && !has_subkeyword_values
            && !has_group_subkeywords
            && !continuation_is_explicit
            && (section_front == 0 || allow_plain_section_inline);
        if can_inline_section_values
            && base_indent + inline_width <= config.line_width as usize
            && can_inline_content
        {
            let kw_text = if keyword.is_keyword {
                apply_keyword_case(&keyword.text, config.keyword_case)
            } else {
                Cow::Borrowed(keyword.text.as_str())
            };
            items.push_string(kw_text.into_owned());
            for val in values {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(val)));
            }
            if let Some(comment) = &keyword.trailing_comment {
                items.push_space();
                items.extend(ir_helpers::gen_from_raw_string(comment));
            }
            return false;
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
            let can_inline_leading = !force_one_per_line
                && allow_keyword_inline
                && !preserve_inline_keyword_layout
                && !continuation_is_explicit
                && base_indent + leading_width <= config.line_width as usize
                && !values[..leading_count]
                    .iter()
                    .any(|v| v.text.contains('\n'));
            if can_inline_leading {
                let kw_text = if keyword.is_keyword {
                    apply_keyword_case(&keyword.text, config.keyword_case)
                } else {
                    Cow::Borrowed(keyword.text.as_str())
                };
                items.push_string(kw_text.into_owned());
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
                return true;
            }
        }
        emit_kw_arg(items, keyword, config);
        emit_section_values(items, values, sub_kws, config, base_indent);
    } else {
        emit_kw_arg(items, keyword, config);
        if let Some(KwType::Group(front_count, group_sub_keywords)) =
            get_keyword_type(keyword, spec)
        {
            let leading_count = front_count.min(values.len());
            for value in &values[..leading_count] {
                items.push_space();
                emit_arg(items, value, config);
            }
            if leading_count < values.len() {
                emit_section_values_inner(
                    items,
                    &values[leading_count..],
                    group_sub_keywords,
                    config,
                    base_indent,
                    true,
                );
            }
        } else if is_property_keyword(&keyword.text, spec) {
            // Property keyword: first value is the property name, rest are its values
            emit_property_values(items, values, config, base_indent);
        } else if is_pair_keyword(&keyword.text, spec) {
            // Pair keyword: values are alternating key-value pairs
            emit_pair_values(items, values, config, base_indent, true);
        } else if is_flow_keyword(&keyword.text, spec)
            && !force_one_per_line
            && !(config.blank_line_between_sections && is_section_kw)
        {
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
            // When alignArgGroups is enabled, pack tokens onto lines so that
            // same-token-count runs can be column-aligned.
            let continuation = config.effective_continuation_indent_width() as usize;
            emit_values_with_genex_with_indent(
                items,
                regular_values,
                config,
                true,
                continuation,
                false,
                config.align_arg_groups
                    && !force_one_per_line
                    && !(config.blank_line_between_sections && is_section_kw),
                base_indent,
            );

            for comment in trailing_comments {
                items.push_signal(Signal::NewLine);
                emit_arg(items, comment, config);
            }
        }
    }
    true
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
                items.push_indented(val_items);
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
            items.push_indented(val_items);
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
    val_items.push_indented(sub);

    items.push_indented(val_items);
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
        items.push_indented(val_items);
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
    nest_under_keyword: bool,
) {
    let pair_indent = if nest_under_keyword {
        base_indent + config.indent_width as usize
    } else {
        base_indent
    };

    if config.align_property_values {
        emit_aligned_property_pairs(items, values, config, nest_under_keyword);
        return;
    }

    let aligned_key_width = None::<usize>;
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
                let key_text = arg_inline_text(key);
                val_items.push_signal(Signal::NewLine);
                val_items.extend(ir_helpers::gen_from_raw_string(&key_text));

                if let Some(width) = aligned_key_width {
                    let padding = width.saturating_sub(key_text.len()) + 1;
                    for _ in 0..padding {
                        val_items.push_space();
                    }
                } else {
                    val_items.push_space();
                }

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
                val_items.push_indented(sub);
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
                val_items.push_indented(sub);
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

    if nest_under_keyword {
        items.push_indented(val_items);
    } else {
        items.extend(val_items);
    }
}

fn emit_aligned_property_pairs(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    config: &Configuration,
    nest_under_keyword: bool,
) {
    let key_width = compute_pair_alignment_width(values).unwrap_or_else(|| {
        values
            .iter()
            .filter(|arg| looks_like_property_key(arg))
            .map(|arg| arg_width(arg))
            .max()
            .unwrap_or(0)
    });

    if key_width == 0 {
        return;
    }

    let mut val_items = PrintItems::new();
    let mut index = 0usize;

    while index < values.len() {
        let key = values[index];
        if key.text.starts_with('#') {
            val_items.push_signal(Signal::NewLine);
            emit_arg(&mut val_items, key, config);
            index += 1;
            continue;
        }

        let key_text = arg_inline_text(key);
        val_items.push_signal(Signal::NewLine);
        val_items.extend(ir_helpers::gen_from_raw_string(&key_text));
        index += 1;

        let mut emitted_first_value = false;
        while index < values.len() {
            let value = values[index];
            if value.text.starts_with('#') {
                break;
            }
            if emitted_first_value && looks_like_property_key(value) {
                break;
            }

            if !emitted_first_value {
                let padding = key_width.saturating_sub(key_text.len()) + 1;
                for _ in 0..padding {
                    val_items.push_space();
                }
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(value)));
                emitted_first_value = true;
            } else {
                val_items.push_signal(Signal::NewLine);
                for _ in 0..(key_width + 1) {
                    val_items.push_space();
                }
                val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(value)));
            }
            index += 1;
        }
    }

    if nest_under_keyword {
        items.push_indented(val_items);
    } else {
        items.extend(val_items);
    }
}

fn looks_like_property_key(arg: &FormattedArg) -> bool {
    let text = arg_inline_text(arg);
    if text.starts_with('"') || text.starts_with("$<") || text.starts_with('#') {
        return false;
    }

    let upper = text.to_ascii_uppercase();
    if matches!(
        upper.as_str(),
        "ON" | "OFF" | "TRUE" | "FALSE" | "YES" | "NO" | "Y" | "N"
    ) {
        return false;
    }

    !text.is_empty()
        && text
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
}

fn compute_pair_alignment_width(values: &[&FormattedArg]) -> Option<usize> {
    let mut max_width = 0usize;
    let mut i = 0usize;

    while i < values.len() {
        let key = values[i];
        if key.text.starts_with('#') {
            i += 1;
            continue;
        }

        let mut val_idx = i + 1;
        while val_idx < values.len() && values[val_idx].text.starts_with('#') {
            val_idx += 1;
        }

        if val_idx < values.len() {
            max_width = max_width.max(arg_width(key));
            i = val_idx + 1;
        } else {
            i = val_idx;
        }
    }

    if max_width == 0 {
        None
    } else {
        Some(max_width)
    }
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
        items.push_indented(val_items);
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
    let continuation_indent = config.effective_continuation_indent_width() as usize;
    let sub_indent = base_indent + continuation_indent;
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
                    push_wrapped_newline(&mut val_items, wrap_indent, continuation_indent, config);
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
                            push_wrapped_newline(
                                &mut val_items,
                                wrap_indent,
                                continuation_indent,
                                config,
                            );
                            val_items.push_string(
                                apply_keyword_case(&values[i].text, config.keyword_case)
                                    .into_owned(),
                            );
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
                    push_wrapped_newline(&mut val_items, wrap_indent, continuation_indent, config);
                    emit_sub_kw_arg(&mut val_items, values[i], config);
                    if let Some(sv) = sub_val {
                        let mut sub_items = PrintItems::new();
                        sub_items.push_signal(Signal::NewLine);
                        emit_arg(&mut sub_items, sv, config);
                        val_items.push_indented(sub_items);
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
                        push_wrapped_newline(
                            &mut val_items,
                            wrap_indent,
                            continuation_indent,
                            config,
                        );
                        let sub_kw_text = apply_keyword_case(&kw.text, config.keyword_case);
                        val_items.push_string(sub_kw_text.into_owned());
                        for sv in &sub_values {
                            val_items.push_space();
                            val_items.extend(ir_helpers::gen_from_raw_string(&arg_inline_text(sv)));
                        }
                        if let Some(comment) = &kw.trailing_comment {
                            val_items.push_space();
                            val_items.extend(ir_helpers::gen_from_raw_string(comment));
                        }
                    } else if sub_values.is_empty() {
                        push_wrapped_newline(
                            &mut val_items,
                            wrap_indent,
                            continuation_indent,
                            config,
                        );
                        emit_sub_kw_arg(&mut val_items, kw, config);
                    } else {
                        push_wrapped_newline(
                            &mut val_items,
                            wrap_indent,
                            continuation_indent,
                            config,
                        );
                        emit_sub_kw_arg(&mut val_items, kw, config);
                        let mut sub_items = PrintItems::new();
                        for sv in &sub_values {
                            sub_items.push_signal(Signal::NewLine);
                            emit_arg(&mut sub_items, sv, config);
                        }
                        val_items.push_indented_times(sub_items, 2);
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
                        push_wrapped_newline(
                            &mut val_items,
                            wrap_indent,
                            continuation_indent,
                            config,
                        );
                        val_items.push_string(
                            apply_keyword_case(&kw.text, config.keyword_case).into_owned(),
                        );
                        for v in all_group_values {
                            val_items.push_space();
                            let raw = arg_inline_text(v);
                            let vt = if v.is_keyword {
                                apply_keyword_case_owned(raw.into_owned(), config.keyword_case)
                            } else {
                                raw.into_owned()
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
                            push_wrapped_newline(
                                &mut val_items,
                                wrap_indent,
                                continuation_indent,
                                config,
                            );
                            val_items.push_string(
                                apply_keyword_case(&kw.text, config.keyword_case).into_owned(),
                            );
                            for v in front_positionals {
                                val_items.push_space();
                                val_items
                                    .extend(ir_helpers::gen_from_raw_string(&arg_inline_text(v)));
                            }
                            if let Some(comment) = &kw.trailing_comment {
                                val_items.push_space();
                                val_items.extend(ir_helpers::gen_from_raw_string(comment));
                            }
                            let mut nested_items = PrintItems::new();
                            emit_section_values_inner(
                                &mut nested_items,
                                rest_values,
                                group_sub_kws,
                                config,
                                sub_indent,
                                true,
                            );
                            val_items.push_indented(nested_items);
                        } else {
                            // Keyword alone on its line, everything else indented
                            push_wrapped_newline(
                                &mut val_items,
                                wrap_indent,
                                continuation_indent,
                                config,
                            );
                            emit_sub_kw_arg(&mut val_items, kw, config);
                            let mut nested_items = PrintItems::new();
                            emit_section_values_inner(
                                &mut nested_items,
                                all_group_values,
                                group_sub_kws,
                                config,
                                sub_indent,
                                true,
                            );
                            val_items.push_indented(nested_items);
                        }
                    }
                }
            }
        } else {
            // Consecutive regular values can contain one or more genex groups.
            let run_start = i;
            while i < values.len()
                && !sub_keywords
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case(&values[i].text))
            {
                i += 1;
            }

            emit_values_with_genex_with_indent(
                &mut val_items,
                &values[run_start..i],
                config,
                wrap_indent,
                continuation_indent,
                true,
                config.align_arg_groups && !config.blank_line_between_sections,
                base_indent,
            );
        }
    }

    items.extend(val_items);
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
            apply_keyword_case_owned(raw.into_owned(), config.keyword_case)
        } else if !arg.is_bracket
            && !raw.starts_with('"')
            && !raw.starts_with('#')
            && is_literal_token(&raw)
        {
            apply_literal_case_owned(raw.into_owned(), config.literal_case)
        } else {
            raw.into_owned()
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
            Cow::Borrowed(op.text.as_str())
        };
        items.push_string(op_text.into_owned());
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
        Cow::Borrowed(op.text.as_str())
    };
    items.push_string(op_text.into_owned());
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
        items.push_indented(sub);
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
                items.push_indented(paren_inner);
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
                items.push_string(
                    if op.is_keyword {
                        apply_keyword_case(&op.text, config.keyword_case)
                    } else {
                        Cow::Borrowed(op.text.as_str())
                    }
                    .into_owned(),
                );
                if let Some(comment) = &op.trailing_comment {
                    items.push_space();
                    items.extend(ir_helpers::gen_from_raw_string(comment));
                }
                items.push_space();
                emit_cond_expr_inline(items, operand, config);
                return;
            }

            // Expanded
            items.push_string(
                if op.is_keyword {
                    apply_keyword_case(&op.text, config.keyword_case)
                } else {
                    Cow::Borrowed(op.text.as_str())
                }
                .into_owned(),
            );
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
                items.push_indented(sub);
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
                items.push_string(
                    if op.is_keyword {
                        apply_keyword_case(&op.text, config.keyword_case)
                    } else {
                        Cow::Borrowed(op.text.as_str())
                    }
                    .into_owned(),
                );
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
            sub.push_string(
                if op.is_keyword {
                    apply_keyword_case(&op.text, config.keyword_case)
                } else {
                    Cow::Borrowed(op.text.as_str())
                }
                .into_owned(),
            );
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
            items.push_indented(sub);
        }
    }
}

/// Emit a condition expression inline (no width checks — caller verified it fits).
fn emit_cond_expr_inline(items: &mut PrintItems, expr: &CondExpr<'_>, config: &Configuration) {
    match expr {
        CondExpr::Atom(arg) => {
            let t = arg_inline_text(arg);
            let t = if arg.is_keyword {
                apply_keyword_case_owned(t.into_owned(), config.keyword_case)
            } else {
                t.into_owned()
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
                Cow::Borrowed(op.text.as_str())
            };
            items.push_string(op_text.into_owned());
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
                Cow::Borrowed(op.text.as_str())
            };
            items.push_string(op_text.into_owned());
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
                Cow::Borrowed(arg.text.as_str())
            }
        } else if !arg.text.starts_with('"')
            && !arg.text.starts_with('#')
            && is_literal_token(&arg.text)
        {
            apply_literal_case(&arg.text, literal_case)
        } else {
            Cow::Borrowed(arg.text.as_str())
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

/// Look up the `KwType` for a keyword text from the command spec.
/// Returns `None` when the keyword is unrecognized or no spec is available.
fn lookup_kw_type(kw_text: &str, cmd_kind: Option<&CommandKind>) -> Option<KwType> {
    let spec = match cmd_kind {
        Some(CommandKind::Known(spec)) => spec,
        _ => return None,
    };
    spec.keywords
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(kw_text))
        .map(|(_, kt)| *kt)
}

fn sort_argument_groups(
    args: &mut [FormattedArg],
    config: &Configuration,
    cmd_kind: Option<&CommandKind>,
) {
    let selective_keywords: Option<&[String]> = match &config.sort_arguments {
        SortArguments::CommandList(kws) => Some(kws),
        _ => None,
    };

    // Number of leading non-keyword positional args to skip (e.g. target name).
    let front = match cmd_kind {
        Some(CommandKind::Known(spec)) => spec.front_positional,
        _ => 0,
    };

    let mut i = 0;
    let mut positional_seen = 0usize;

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
                while end < args.len() && !args[end].is_keyword {
                    end += 1;
                }
                if end > start {
                    sort_section_with_groups(&mut args[start..end]);
                }
                i = end;
            } else {
                // Not sortable: skip past the keyword and the args it consumes.
                // Option keywords (flags) consume no args — remaining args may still
                // be sortable as trailing positionals. Multi-value / section keywords
                // consume all following non-keyword args (those belong to the section).
                let kw_consumes = lookup_kw_type(kw_text, cmd_kind);
                match kw_consumes {
                    Some(KwType::Option) => {
                        // Flag keyword: skip only the keyword itself
                        i += 1;
                    }
                    Some(KwType::OneValue) => {
                        // Skip keyword + 1 value arg
                        i += 1;
                        if i < args.len() && !args[i].is_keyword {
                            i += 1;
                        }
                    }
                    _ => {
                        // MultiValue, Group, or unknown: skip entire section
                        let mut end = i + 1;
                        while end < args.len() && !args[end].is_keyword {
                            end += 1;
                        }
                        i = end;
                    }
                }
            }
        } else {
            // Non-keyword arg: track front positional count
            positional_seen += 1;
            if positional_seen <= front {
                // Still in the front positional region (e.g. target name) — skip
                i += 1;
            } else {
                // Past the front positionals: collect contiguous non-keyword run
                // and sort it (these are trailing source/value args).
                let start = i;
                let mut end = i + 1;
                while end < args.len() && !args[end].is_keyword {
                    end += 1;
                }
                if end - start > 1 {
                    sort_section_with_groups(&mut args[start..end]);
                }
                // Advance positional_seen for all args in the sorted run
                // (the first was already counted above)
                positional_seen += end - start - 1;
                i = end;
            }
        }
    }
}

/// Sort a section of arguments respecting group boundaries.
///
/// A source blank line starts a new independent sort segment. Within each
/// segment, comments remain attached to the following sortable row.
fn sort_section_with_groups(args: &mut [FormattedArg]) {
    #[derive(Clone)]
    struct SortUnit {
        key: String,
        items: Vec<FormattedArg>,
    }

    fn is_standalone_comment(arg: &FormattedArg) -> bool {
        arg.text.starts_with('#')
    }

    fn flush_row(
        units: &mut Vec<SortUnit>,
        row_key: &mut Option<String>,
        row_items: &mut Vec<FormattedArg>,
    ) {
        if let Some(key) = row_key.take() {
            units.push(SortUnit {
                key,
                items: std::mem::take(row_items),
            });
        }
    }

    fn sort_segment(segment: &[FormattedArg]) -> Vec<FormattedArg> {
        let mut units: Vec<SortUnit> = Vec::new();
        let mut pending_comments: Vec<FormattedArg> = Vec::new();
        let mut row_items: Vec<FormattedArg> = Vec::new();
        let mut row_key: Option<String> = None;

        let mut leading_comments: Vec<FormattedArg> = Vec::new();
        let mut start_index = 0usize;
        while start_index < segment.len()
            && is_standalone_comment(&segment[start_index])
            && segment[start_index].blank_line_before
        {
            leading_comments.push(segment[start_index].clone());
            start_index += 1;
        }
        let has_explicit_rows = segment[start_index..]
            .iter()
            .any(|item| !is_standalone_comment(item) && item.new_line_before);

        for item in &segment[start_index..] {
            if is_standalone_comment(item) {
                pending_comments.push(item.clone());
                continue;
            }

            let starts_new_row = if has_explicit_rows {
                item.new_line_before && row_key.is_some()
            } else {
                row_key.is_some()
            };
            if starts_new_row {
                flush_row(&mut units, &mut row_key, &mut row_items);
            }

            if row_key.is_none() {
                row_key = Some(item.text.to_ascii_lowercase());
                row_items.append(&mut pending_comments);
            }
            row_items.push(item.clone());
        }

        flush_row(&mut units, &mut row_key, &mut row_items);
        if units.is_empty() {
            leading_comments.extend(pending_comments);
            return leading_comments;
        }

        let trailing_comments = pending_comments;
        units.sort_by(|a, b| a.key.cmp(&b.key));

        let mut sorted = leading_comments;
        for unit in units {
            sorted.extend(unit.items);
        }
        sorted.extend(trailing_comments);
        let segment_starts_with_blank = segment
            .first()
            .map(|item| item.blank_line_before)
            .unwrap_or(false);
        for item in &mut sorted {
            item.blank_line_before = false;
        }
        if let Some(first) = sorted.first_mut() {
            first.blank_line_before = segment_starts_with_blank;
        }
        sorted
    }

    let mut rebuilt: Vec<FormattedArg> = Vec::with_capacity(args.len());
    let mut segment_start = 0usize;
    for index in 1..args.len() {
        if !args[index].blank_line_before {
            continue;
        }
        rebuilt.extend(sort_segment(&args[segment_start..index]));
        segment_start = index;
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

fn split_condition_values(text: &str) -> Vec<&str> {
    let mut values = Vec::new();
    for part in split_at_depth0(text, ' ') {
        for candidate in split_at_depth0(part, ';') {
            let trimmed = candidate.trim();
            if !trimmed.is_empty() {
                values.push(trimmed);
            }
        }
    }
    values
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
                // Values split on depth-0 spaces and semicolons.
                let values = split_condition_values(after_colon)
                    .into_iter()
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
            format!("$<{cond_text}:{}>", values_text.join(";"))
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
    node: &GenexNode,
    extra_indent: usize,
    close_suffix: &str,
    config: &Configuration,
) -> bool {
    let inline_text = genex_inline_text(node);
    let total_width = extra_indent + inline_text.len() + close_suffix.len();
    total_width <= config.line_width as usize
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
fn format_genex_node(
    items: &mut PrintItems,
    node: &GenexNode,
    config: &Configuration,
    extra_indent: usize,
) {
    format_genex_impl(items, node, "", 0, config, extra_indent);
}

/// Internal: format a genex node, appending `close_suffix` after the closing `>`.
/// This is used by ConditionGenex to merge condition's `>` with `:`.
fn format_genex_impl(
    items: &mut PrintItems,
    node: &GenexNode,
    close_suffix: &str,
    _depth: usize,
    config: &Configuration,
    extra_indent: usize,
) {
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
                let genex_indent = config.effective_genex_indent_width() as usize;
                let child_indent = extra_indent + genex_indent;
                for (i, child) in children.iter().enumerate() {
                    push_newline_with_visual_indent(items, child_indent, config);
                    format_genex_impl(items, child, "", _depth + 1, config, child_indent);
                    if i + 1 < children.len() {
                        items.push_str_runtime_width_computed(",");
                    }
                }
                let compact_close =
                    close_suffix == ":" && condition_genex_prefers_compact_close(node);
                if compact_close {
                    let closer = format!(">{close_suffix}");
                    items.extend(ir_helpers::gen_from_raw_string(&closer));
                } else {
                    if config.genex_closing_angle_newline {
                        push_newline_with_visual_indent(items, extra_indent, config);
                    }
                    let closer = format!(">{close_suffix}");
                    items.extend(ir_helpers::gen_from_raw_string(&closer));
                }
            }
        }
        GenexNode::ConditionGenex { condition, values } => {
            if condition_genex_should_inline(node, extra_indent, close_suffix, config) {
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
                format_genex_impl(items, condition, ":", _depth + 1, config, extra_indent);
            }
            // Values indented relative to the `$<` opener column.
            let genex_indent = config.effective_genex_indent_width() as usize;
            let value_indent = extra_indent + genex_indent;
            let packed_values = values
                .iter()
                .map(genex_inline_text)
                .collect::<Vec<_>>()
                .join(";");
            let can_pack_values = values.len() <= 2
                && !packed_values.is_empty()
                && packed_values.len() <= 12
                && values.iter().all(genex_is_inline)
                && value_indent + packed_values.len() <= config.line_width as usize;
            if can_pack_values {
                push_newline_with_visual_indent(items, value_indent, config);
                items.extend(ir_helpers::gen_from_raw_string(&packed_values));
            } else {
                for val in values {
                    push_newline_with_visual_indent(items, value_indent, config);
                    format_genex_impl(items, val, "", _depth + 1, config, value_indent);
                }
            }
            if config.genex_closing_angle_newline {
                push_newline_with_visual_indent(items, extra_indent, config);
            }
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
            if memchr::memmem::find(arg.text.as_bytes(), b"$<").is_some() {
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

/// Emits a generator-expression argument with wrapping behavior configured by `genexWrap`.
fn emit_genex_value(
    items: &mut PrintItems,
    text: &str,
    config: &Configuration,
    wrap_indent: bool,
    continuation_indent: usize,
) {
    push_wrapped_newline(items, wrap_indent, continuation_indent, config);
    if matches!(config.genex_wrap, crate::configuration::GenexWrap::Never) {
        items.extend(ir_helpers::gen_from_raw_string(text));
        return;
    }
    let node = parse_genex(text);
    format_genex_node(items, &node, config, continuation_indent);
}

#[derive(Clone)]
enum ValueLayoutLine<'a> {
    Blank,
    Comment(&'a FormattedArg),
    Genex(String),
    Tokens(Vec<&'a FormattedArg>),
}

#[derive(Clone, Default)]
struct TokenLineAlignment {
    first_col_width: Option<usize>,
    column_widths: Option<Vec<usize>>,
}

fn is_keyword_like_value(arg: &FormattedArg) -> bool {
    let text = arg_inline_text(arg);
    if text.starts_with('"') || text.starts_with("$<") || text.starts_with('#') {
        return false;
    }
    text.chars()
        .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
}

fn token_visual_width(arg: &FormattedArg) -> usize {
    arg_width(arg)
}

fn aligned_token_line_width(
    tokens: &[&FormattedArg],
    alignment: &TokenLineAlignment,
    base_indent: usize,
) -> usize {
    let mut width = base_indent;
    for (idx, token) in tokens.iter().enumerate() {
        let token_width = token_visual_width(token);
        width += token_width;
        if idx + 1 < tokens.len() {
            let mut spaces = 1usize;
            if idx == 0
                && let Some(first_col_width) = alignment.first_col_width
            {
                spaces = spaces.max(first_col_width.saturating_sub(token_width) + 1);
            }
            if let Some(column_widths) = &alignment.column_widths {
                spaces = spaces.max(column_widths[idx].saturating_sub(token_width) + 1);
            }
            width += spaces;
        }
    }
    width
}

fn apply_arg_group_alignment(
    lines: &[ValueLayoutLine<'_>],
    config: &Configuration,
    wrap_indent: bool,
    continuation_indent: usize,
) -> Vec<TokenLineAlignment> {
    let mut alignment = vec![TokenLineAlignment::default(); lines.len()];
    let indent_width = if wrap_indent { continuation_indent } else { 0 };
    let line_width_limit = config.line_width as usize;

    let mut segment_start = 0usize;
    while segment_start < lines.len() {
        while segment_start < lines.len() {
            if matches!(lines[segment_start], ValueLayoutLine::Tokens(_)) {
                break;
            }
            segment_start += 1;
        }
        if segment_start >= lines.len() {
            break;
        }

        let mut segment_end = segment_start;
        while segment_end < lines.len() {
            if !matches!(lines[segment_end], ValueLayoutLine::Tokens(_)) {
                break;
            }
            segment_end += 1;
        }

        let mut carried_column_widths: std::collections::BTreeMap<usize, Vec<usize>> =
            std::collections::BTreeMap::new();

        let mut keyword_line_indices: Vec<usize> = Vec::new();
        for (line_idx, line) in lines
            .iter()
            .enumerate()
            .take(segment_end)
            .skip(segment_start)
        {
            if let ValueLayoutLine::Tokens(tokens) = line
                && !tokens.is_empty()
                && is_keyword_like_value(tokens[0])
            {
                keyword_line_indices.push(line_idx);
            }
        }

        if keyword_line_indices.len() >= 2 {
            let keyword_width = keyword_line_indices
                .iter()
                .map(|line_idx| {
                    if let ValueLayoutLine::Tokens(tokens) = &lines[*line_idx] {
                        token_visual_width(tokens[0])
                    } else {
                        0
                    }
                })
                .max()
                .unwrap_or(0);
            // Keep an explicit gap after keyword-style first columns so aligned
            // keyword groups remain visually distinct from their values.
            let first_column_width = keyword_width + 1;

            for line_idx in keyword_line_indices {
                alignment[line_idx].first_col_width = Some(first_column_width);
            }
        }

        let mut run_start = segment_start;
        while run_start < segment_end {
            let token_count = if let ValueLayoutLine::Tokens(tokens) = &lines[run_start] {
                tokens.len()
            } else {
                0
            };
            let mut run_end = run_start + 1;
            while run_end < segment_end {
                let next_count = if let ValueLayoutLine::Tokens(tokens) = &lines[run_end] {
                    tokens.len()
                } else {
                    0
                };
                if next_count != token_count {
                    break;
                }
                run_end += 1;
            }

            if token_count >= 2 && run_end - run_start >= 2 {
                // Compute per-column max widths across all lines in the run.
                let mut column_widths = vec![0usize; token_count];
                for line in lines.iter().take(run_end).skip(run_start) {
                    if let ValueLayoutLine::Tokens(tokens) = line {
                        for (col, token) in tokens.iter().enumerate() {
                            column_widths[col] = column_widths[col].max(token_visual_width(token));
                        }
                    }
                }

                if let Some(previous_widths) = carried_column_widths.get(&token_count) {
                    for (col, width) in column_widths.iter_mut().enumerate() {
                        *width = (*width).max(previous_widths[col]);
                    }
                }

                // Overflow check: use the global max width (uniform) to determine
                // if alignment would cause any line to exceed lineWidth. This is
                // conservative — if worst-case uniform alignment overflows, the
                // entire run is excluded even though per-column widths might fit.
                let global_max = *column_widths.iter().max().unwrap_or(&0);
                let uniform_widths = vec![global_max; token_count];
                let mut fits = true;
                for line_idx in run_start..run_end {
                    let test_alignment = TokenLineAlignment {
                        column_widths: Some(uniform_widths.clone()),
                        ..alignment[line_idx].clone()
                    };
                    if let ValueLayoutLine::Tokens(tokens) = &lines[line_idx]
                        && aligned_token_line_width(tokens, &test_alignment, indent_width)
                            > line_width_limit
                    {
                        fits = false;
                        break;
                    }
                }

                if fits {
                    carried_column_widths.insert(token_count, column_widths.clone());
                    for alignment_entry in alignment.iter_mut().take(run_end).skip(run_start) {
                        alignment_entry.column_widths = Some(column_widths.clone());
                    }
                }
            }

            run_start = run_end;
        }

        segment_start = segment_end;
    }

    alignment
}

/// Emit values with genex detection using an explicit indentation strategy.
#[allow(clippy::too_many_arguments)]
fn emit_values_with_genex_with_indent(
    items: &mut PrintItems,
    values: &[&FormattedArg],
    config: &Configuration,
    wrap_indent: bool,
    continuation_indent: usize,
    skip_first_blank: bool,
    pack_tokens: bool,
    column_start: usize,
) {
    let grouped_values = group_args_by_genex(values);
    let indent_width = if wrap_indent { continuation_indent } else { 0 };
    let max_content_width = config.line_width as usize;

    let mut lines: Vec<ValueLayoutLine<'_>> = Vec::new();
    let mut current_tokens: Vec<&FormattedArg> = Vec::new();
    let mut current_width = 0usize;

    fn flush_current_line<'a>(
        lines: &mut Vec<ValueLayoutLine<'a>>,
        current_tokens: &mut Vec<&'a FormattedArg>,
        current_width: &mut usize,
    ) {
        if !current_tokens.is_empty() {
            lines.push(ValueLayoutLine::Tokens(std::mem::take(current_tokens)));
            *current_width = 0;
        }
    }

    for (group_index, group) in grouped_values.into_iter().enumerate() {
        let group_has_leading_blank = match &group {
            GenexArgGroup::Single(arg) => arg.blank_line_before,
            GenexArgGroup::Genex(args) => args
                .first()
                .map(|first| first.blank_line_before)
                .unwrap_or(false),
        };

        if group_has_leading_blank && !(skip_first_blank && group_index == 0) {
            flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
            lines.push(ValueLayoutLine::Blank);
        }

        match group {
            GenexArgGroup::Single(arg) => {
                if arg.text.starts_with('#') {
                    flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                    lines.push(ValueLayoutLine::Comment(arg));
                    continue;
                }

                let token_width = arg_width(arg);
                if !pack_tokens {
                    // One token per line (original behavior)
                    flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                    current_tokens.push(arg);
                    current_width = token_width;
                    if arg.trailing_comment.is_some() && !arg.trailing_is_bracket {
                        flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                    }
                } else {
                    // Pack tokens into lines, respecting source newlines
                    // and falling back to width-based packing.
                    let source_break = arg.new_line_before && !current_tokens.is_empty();
                    if current_tokens.is_empty() {
                        current_tokens.push(arg);
                        current_width = token_width;
                    } else if source_break {
                        // Source had a newline here — start a new line.
                        flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                        current_tokens.push(arg);
                        current_width = token_width;
                    } else {
                        let required = 1 + token_width;
                        let available_width =
                            max_content_width.saturating_sub(column_start + indent_width);
                        let last_has_line_comment = current_tokens.last().is_some_and(|token| {
                            token.trailing_comment.is_some() && !token.trailing_is_bracket
                        });

                        if last_has_line_comment || current_width + required > available_width {
                            flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                            current_tokens.push(arg);
                            current_width = token_width;
                        } else {
                            current_tokens.push(arg);
                            current_width += required;
                        }
                    }

                    if arg.trailing_comment.is_some() && !arg.trailing_is_bracket {
                        flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                    }
                }
            }
            GenexArgGroup::Genex(args) => {
                flush_current_line(&mut lines, &mut current_tokens, &mut current_width);
                let joined = args
                    .iter()
                    .map(|arg| arg.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                lines.push(ValueLayoutLine::Genex(joined));
            }
        }
    }

    flush_current_line(&mut lines, &mut current_tokens, &mut current_width);

    let alignments = if config.align_arg_groups {
        apply_arg_group_alignment(&lines, config, wrap_indent, continuation_indent)
    } else {
        vec![TokenLineAlignment::default(); lines.len()]
    };

    for (line_idx, line) in lines.into_iter().enumerate() {
        match line {
            ValueLayoutLine::Blank => items.push_signal(Signal::NewLine),
            ValueLayoutLine::Comment(comment) => {
                push_wrapped_newline(items, wrap_indent, continuation_indent, config);
                emit_arg(items, comment, config);
            }
            ValueLayoutLine::Genex(text) => {
                emit_genex_value(items, &text, config, wrap_indent, continuation_indent);
            }
            ValueLayoutLine::Tokens(tokens) => {
                let line_alignment = &alignments[line_idx];
                push_wrapped_newline(items, wrap_indent, continuation_indent, config);
                for (token_index, token) in tokens.iter().enumerate() {
                    emit_arg(items, token, config);
                    if token_index + 1 < tokens.len() {
                        let token_width = token_visual_width(token);
                        let mut spaces = 1usize;
                        if token_index == 0
                            && let Some(first_col_width) = line_alignment.first_col_width
                        {
                            spaces = spaces.max(first_col_width.saturating_sub(token_width) + 1);
                        }
                        if let Some(column_widths) = &line_alignment.column_widths {
                            spaces = spaces
                                .max(column_widths[token_index].saturating_sub(token_width) + 1);
                        }
                        for _ in 0..spaces {
                            items.push_space();
                        }
                    }
                }
            }
        }
    }
}

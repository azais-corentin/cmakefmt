//! Post-processing passes applied to the formatted text output.
//!
//! These passes handle alignment features that require cross-command context
//! (consecutive set alignment, trailing comment alignment at file scope) which
//! cannot be expressed within the per-command IR generation.

use crate::configuration::{Configuration, apply_inline_overrides};

/// Apply all post-processing alignment passes to the formatted output.
///
/// Currently handles:
/// - `alignConsecutiveSet`: column-align values in consecutive `set()` calls.
/// - `alignTrailingComments`: column-align trailing `#` comments on consecutive lines.
pub fn post_process_alignments(text: &str, base_config: &Configuration) -> String {
    let newline = detect_newline(text);
    let mut lines: Vec<String> = split_lines(text, newline);

    // Build per-line config snapshot for alignment-relevant settings.
    let configs = resolve_per_line_config(&lines, base_config);

    align_consecutive_set_lines(&mut lines, &configs);
    align_trailing_comment_lines(&mut lines, &configs, newline);

    join_lines(&lines, newline)
}

// ---------------------------------------------------------------------------
// Utility: line splitting that preserves the final newline convention
// ---------------------------------------------------------------------------

fn detect_newline(text: &str) -> &'static str {
    if text.contains("\r\n") { "\r\n" } else { "\n" }
}

fn split_lines(text: &str, newline: &str) -> Vec<String> {
    // Split but keep the final empty element if the text ends with a newline.
    text.split(newline).map(String::from).collect()
}

fn join_lines(lines: &[String], newline: &str) -> String {
    lines.join(newline)
}

// ---------------------------------------------------------------------------
// Per-line configuration tracking (for pragma push/pop)
// ---------------------------------------------------------------------------

/// Snapshot of alignment-relevant config at a given line.
#[derive(Clone)]
struct LineConfig {
    align_consecutive_set: bool,
    align_trailing_comments: bool,
    comment_gap: u8,
}

impl LineConfig {
    fn from_config(config: &Configuration) -> Self {
        Self {
            align_consecutive_set: config.align_consecutive_set,
            align_trailing_comments: config.align_trailing_comments,
            comment_gap: config.comment_gap,
        }
    }
}

const PRAGMA_PREFIX: &str = "cmakefmt:";

fn resolve_per_line_config(lines: &[String], base: &Configuration) -> Vec<LineConfig> {
    let mut current = base.clone();
    let mut stack: Vec<Configuration> = Vec::new();
    let mut result = Vec::with_capacity(lines.len());

    for line in lines {
        let trimmed = line.trim();

        // Check for push/pop pragma directives.
        if trimmed.starts_with('#') {
            if let Some(body) = parse_push_body(trimmed) {
                stack.push(current.clone());
                current = apply_inline_overrides(&current, body);
            } else if is_pop_directive(trimmed)
                && let Some(prev) = stack.pop()
            {
                current = prev;
            }
        }

        result.push(LineConfig::from_config(&current));
    }

    result
}

fn parse_push_body(comment_line: &str) -> Option<&str> {
    let rest = comment_line.trim().strip_prefix('#')?.trim_start();
    let rest = rest.strip_prefix(PRAGMA_PREFIX)?.trim_start();
    let action_end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let action = &rest[..action_end];
    if action != "push" {
        return None;
    }
    let body = rest[action_end..].trim_start();
    if body.starts_with('{') {
        Some(body)
    } else {
        None
    }
}

fn is_pop_directive(comment_line: &str) -> bool {
    let Some(rest) = comment_line.trim().strip_prefix('#') else {
        return false;
    };
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(PRAGMA_PREFIX) else {
        return false;
    };
    rest.trim() == "pop"
}

// ---------------------------------------------------------------------------
// alignConsecutiveSet
// ---------------------------------------------------------------------------

/// Information about a single-line `set()` call that participates in alignment.
struct SetCallInfo {
    line_index: usize,
    /// Byte offset of the variable name within the line.
    var_start: usize,
    /// Length of the variable name.
    var_len: usize,
    /// Byte offset where the first value starts (after the variable name + whitespace).
    /// `None` if the set() has no value (valueless or PARENT_SCOPE-only).
    value_start: Option<usize>,
}

/// Known keywords that, when appearing as the second token of `set()`, indicate
/// a valueless call (the keyword is not a value but a CMake keyword).
const SET_KEYWORDS: &[&str] = &["PARENT_SCOPE", "CACHE", "FORCE"];

fn align_consecutive_set_lines(lines: &mut [String], configs: &[LineConfig]) {
    // Collect groups of consecutive set() calls and align each group.
    let mut i = 0;
    while i < lines.len() {
        if !configs[i].align_consecutive_set {
            i += 1;
            continue;
        }

        // Try to start a group at line i.
        let info = parse_set_call(&lines[i]);
        if info.is_none() {
            i += 1;
            continue;
        }

        let mut group: Vec<SetCallInfo> = Vec::new();
        let group_start = i;

        while i < lines.len() && configs[i].align_consecutive_set {
            let line = &lines[i];
            let trimmed = line.trim();

            // Blank line breaks the group.
            if trimmed.is_empty() {
                break;
            }

            // Standalone comment line breaks the group.
            if trimmed.starts_with('#') {
                break;
            }

            // Non-set command breaks the group.
            if let Some(mut info) = parse_set_call(line) {
                info.line_index = i;
                group.push(info);
                i += 1;
            } else {
                break;
            }
        }

        if group.len() >= 2 {
            apply_set_alignment(lines, &group);
        }

        // If we stopped before moving past the group, advance past the breaker.
        if i == group_start && group.is_empty() {
            i += 1;
        } else if i < lines.len() && i > group_start && group.len() < 2 {
            // Single set in group — no alignment needed, move on.
        }
    }
}

fn parse_set_call(line: &str) -> Option<SetCallInfo> {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();

    // Match `set(` case-insensitively.
    if trimmed.len() < 4 {
        return None;
    }
    let cmd_prefix = &trimmed[..4];
    if !cmd_prefix.eq_ignore_ascii_case("set(") {
        return None;
    }

    // Must be a single-line set() call — the line must contain a closing `)`.
    if !trimmed.contains(')') {
        return None;
    }

    // Extract content between parens.
    let paren_start = indent + trimmed.find('(')? + 1;
    let paren_end = indent + trimmed.rfind(')')?;

    let inner = line[paren_start..paren_end].trim();
    if inner.is_empty() {
        return None;
    }

    // First token = variable name.
    let var_end_in_inner = inner
        .find(|ch: char| ch.is_ascii_whitespace())
        .unwrap_or(inner.len());
    let var_name = &inner[..var_end_in_inner];
    let var_start = paren_start + (inner.as_ptr() as usize - line[paren_start..].as_ptr() as usize);
    let var_len = var_name.len();

    // Check for value after variable name.
    let after_var = inner[var_end_in_inner..].trim_start();
    if after_var.is_empty() {
        // Valueless set, e.g., `set(EMPTY)`
        return Some(SetCallInfo {
            line_index: 0, // filled by caller
            var_start,
            var_len,
            value_start: None,
        });
    }

    // Check if the first token after the variable is a keyword (PARENT_SCOPE, etc.)
    let second_token_end = after_var
        .find(|ch: char| ch.is_ascii_whitespace())
        .unwrap_or(after_var.len());
    let second_token = &after_var[..second_token_end];

    if SET_KEYWORDS
        .iter()
        .any(|&kw| kw.eq_ignore_ascii_case(second_token))
        && second_token.eq_ignore_ascii_case("PARENT_SCOPE")
    {
        // PARENT_SCOPE-only: no value to align
        return Some(SetCallInfo {
            line_index: 0,
            var_start,
            var_len,
            value_start: None,
        });
    }

    // Has a value — find its start position in the line.
    let value_offset =
        paren_start + (after_var.as_ptr() as usize - line[paren_start..].as_ptr() as usize);

    Some(SetCallInfo {
        line_index: 0,
        var_start,
        var_len,
        value_start: Some(value_offset),
    })
}

fn apply_set_alignment(lines: &mut [String], group: &[SetCallInfo]) {
    // Find the maximum variable name length among entries that have a value.
    let max_var_len = group
        .iter()
        .filter(|info| info.value_start.is_some())
        .map(|info| info.var_len)
        .max()
        .unwrap_or(0);

    if max_var_len == 0 {
        return;
    }

    // Rebuild each line with aligned value column.
    for info in group {
        if info.value_start.is_none() {
            continue; // Skip valueless set() calls
        }

        let line = &lines[info.line_index];
        let var_end = info.var_start + info.var_len;
        let value_start = info.value_start.unwrap();

        // Build: prefix (up to end of var name) + padding + value portion (from value_start to end)
        let prefix = &line[..var_end];
        let padding = max_var_len.saturating_sub(info.var_len) + 1;
        let suffix = &line[value_start..];

        let mut new_line = String::with_capacity(prefix.len() + padding + suffix.len());
        new_line.push_str(prefix);
        for _ in 0..padding {
            new_line.push(' ');
        }
        new_line.push_str(suffix);

        lines[info.line_index] = new_line;
    }
}

// ---------------------------------------------------------------------------
// alignTrailingComments
// ---------------------------------------------------------------------------

fn align_trailing_comment_lines(lines: &mut [String], configs: &[LineConfig], _newline: &str) {
    let mut i = 0;
    while i < lines.len() {
        if !configs[i].align_trailing_comments {
            i += 1;
            continue;
        }

        // Try to start a group of consecutive lines with trailing comments
        // at the same indent level.
        let first = parse_trailing_comment(&lines[i]);
        if first.is_none() {
            i += 1;
            continue;
        }
        let first = first.unwrap();
        let group_indent = first.indent_level;
        let comment_gap = configs[i].comment_gap;

        let mut group: Vec<(usize, TrailingCommentInfo)> = vec![(i, first)];
        let mut j = i + 1;

        while j < lines.len() && configs[j].align_trailing_comments {
            let trimmed = lines[j].trim();
            // Blank line breaks group.
            if trimmed.is_empty() {
                break;
            }
            if let Some(info) = parse_trailing_comment(&lines[j]) {
                if info.indent_level != group_indent {
                    break;
                }
                group.push((j, info));
                j += 1;
            } else {
                // Line without trailing comment breaks the group.
                break;
            }
        }

        if group.len() >= 2 {
            apply_trailing_comment_alignment(lines, &group, comment_gap);
        }

        i = j;
    }
}

struct TrailingCommentInfo {
    /// Length of the code portion (trimmed of trailing whitespace).
    code_len: usize,
    /// The indent level (number of leading whitespace chars).
    indent_level: usize,
    /// The comment text (from `#` to end of line).
    comment: String,
}

/// Parse a line to find a trailing `# ...` comment that's not inside a string.
///
/// Returns None if the line has no trailing comment (e.g., it's a standalone comment,
/// blank line, or has no `#` outside of strings).
fn parse_trailing_comment(line: &str) -> Option<TrailingCommentInfo> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Standalone comment lines are not "trailing comments".
    if trimmed.starts_with('#') {
        return None;
    }

    let indent_level = line.len() - line.trim_start().len();

    // Find the trailing comment: scan for `#` that's not inside a quoted string
    // or bracket argument.
    let bytes = line.as_bytes();
    let mut in_quote = false;
    let mut comment_start = None;
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'"' if !in_quote => {
                in_quote = true;
                i += 1;
            }
            b'"' if in_quote => {
                in_quote = false;
                i += 1;
            }
            b'\\' if in_quote && i + 1 < bytes.len() => {
                i += 2; // Skip escaped char
            }
            b'#' if !in_quote => {
                comment_start = Some(i);
                break;
            }
            _ => {
                i += 1;
            }
        }
    }

    let comment_start = comment_start?;

    // The code portion is everything before the comment, trimmed of trailing whitespace.
    let code_part = &line[..comment_start];
    let code_trimmed = code_part.trim_end();

    // If there's no code before the comment, this is a standalone comment (shouldn't happen
    // since we checked trimmed.starts_with('#') above, but guard anyway).
    if code_trimmed.is_empty() {
        return None;
    }

    let comment = line[comment_start..].to_string();

    Some(TrailingCommentInfo {
        code_len: code_trimmed.len(),
        indent_level,
        comment,
    })
}

fn apply_trailing_comment_alignment(
    lines: &mut [String],
    group: &[(usize, TrailingCommentInfo)],
    comment_gap: u8,
) {
    let gap = comment_gap as usize;

    // Alignment column = max code length + commentGap.
    let max_code = group
        .iter()
        .map(|(_, info)| info.code_len)
        .max()
        .unwrap_or(0);
    let hash_col = max_code + gap;

    for (line_idx, info) in group {
        let code_part = &lines[*line_idx][..info.code_len];
        let padding = hash_col.saturating_sub(info.code_len);

        let mut new_line = String::with_capacity(code_part.len() + padding + info.comment.len());
        new_line.push_str(code_part);
        for _ in 0..padding {
            new_line.push(' ');
        }
        new_line.push_str(&info.comment);

        lines[*line_idx] = new_line;
    }
}

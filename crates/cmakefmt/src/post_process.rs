//! Post-processing passes applied to the formatted text output.
//!
//! These passes handle alignment features that require cross-command context
//! (consecutive set alignment, trailing comment alignment at file scope) which
//! cannot be expressed within the per-command IR generation.

use std::borrow::Cow;

use tracing::info_span;

use crate::configuration::{CommentPreservation, Configuration, apply_inline_overrides};
use crate::instrumentation::{
    EVENT_POST_PROCESS, EVENT_POST_PROCESS_ALIGN_BLOCK, EVENT_POST_PROCESS_REFLOW_COMMENT,
};

/// Apply all post-processing alignment passes to the formatted output.
///
/// Currently handles:
/// - `commentPreservation = reflow`: reflow standalone `# ...` comment blocks.
/// - `alignConsecutiveSet`: column-align values in consecutive `set()` calls.
/// - `alignTrailingComments`: column-align trailing `#` comments on consecutive lines.
pub fn post_process_alignments<'a>(text: &'a str, base_config: &Configuration) -> Cow<'a, str> {
    let _stage = info_span!(EVENT_POST_PROCESS, input_bytes = text.len()).entered();

    // Fast path: when no post-processing features are enabled and no pragmas
    // could change that, return the input unchanged.
    let needs_reflow = base_config.comment_preservation == CommentPreservation::Reflow;
    let needs_set_align = base_config.align_consecutive_set;
    let needs_comment_align = base_config.align_trailing_comments;
    let has_pragma_marker =
        memchr::memmem::find(text.as_bytes(), PRAGMA_PREFIX.as_bytes()).is_some();
    if !needs_reflow && !needs_set_align && !needs_comment_align && !has_pragma_marker {
        return Cow::Borrowed(text);
    }

    let newline = crate::util::detect_dominant_line_ending(text);
    let mut lines: Vec<String> = split_lines(text, newline);

    // Build per-line config snapshot used by the reflow pass.
    let (initial_configs, has_pragmas) = resolve_per_line_config(&lines, base_config);
    lines = if initial_configs
        .iter()
        .any(|c| c.comment_preservation == CommentPreservation::Reflow)
    {
        let _reflow_stage = info_span!(EVENT_POST_PROCESS_REFLOW_COMMENT).entered();
        reflow_standalone_comment_blocks(&lines, &initial_configs)
    } else {
        lines
    };

    // Re-resolve config after reflow because line counts may have changed.
    // Skip if no pragmas were found — all lines share the base config.
    let configs = if has_pragmas {
        resolve_per_line_config(&lines, base_config).0
    } else {
        vec![LineConfig::from_config(base_config); lines.len()]
    };

    {
        let _set_stage = info_span!(EVENT_POST_PROCESS_ALIGN_BLOCK, block = "set").entered();
        align_consecutive_set_lines(&mut lines, &configs);
    }
    {
        let _comment_stage =
            info_span!(EVENT_POST_PROCESS_ALIGN_BLOCK, block = "trailing_comment").entered();
        align_trailing_comment_lines(&mut lines, &configs, newline);
    }

    Cow::Owned(join_lines(&lines, newline))
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

/// Snapshot of post-processing-relevant config at a given line.
#[derive(Clone)]
struct LineConfig {
    align_consecutive_set: bool,
    align_trailing_comments: bool,
    comment_gap: u8,
    comment_preservation: CommentPreservation,
    comment_width: u32,
}

impl LineConfig {
    fn from_config(config: &Configuration) -> Self {
        Self {
            align_consecutive_set: config.align_consecutive_set,
            align_trailing_comments: config.align_trailing_comments,
            comment_gap: config.comment_gap,
            comment_preservation: config.comment_preservation,
            comment_width: config.effective_comment_width(),
        }
    }
}

const PRAGMA_PREFIX: &str = "cmakefmt:";

fn resolve_per_line_config(lines: &[String], base: &Configuration) -> (Vec<LineConfig>, bool) {
    let mut current = base.clone();
    let mut stack: Vec<Configuration> = Vec::new();
    let mut result = Vec::with_capacity(lines.len());
    let mut has_pragmas = false;

    for line in lines {
        let trimmed = line.trim();

        // Check for push/pop pragma directives.
        if trimmed.starts_with('#') {
            if let Some(body) = parse_push_body(trimmed) {
                has_pragmas = true;
                stack.push(current.clone());
                current = apply_inline_overrides(&current, body);
            } else if is_pop_directive(trimmed)
                && let Some(prev) = stack.pop()
            {
                has_pragmas = true;
                current = prev;
            }
        }

        result.push(LineConfig::from_config(&current));
    }

    (result, has_pragmas)
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
// commentPreservation = reflow
// ---------------------------------------------------------------------------

fn reflow_standalone_comment_blocks(lines: &[String], configs: &[LineConfig]) -> Vec<String> {
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if !should_reflow_comment_line(&lines[i], &configs[i]) {
            result.push(lines[i].clone());
            i += 1;
            continue;
        }

        let start = i;
        let target_width = configs[i].comment_width as usize;
        i += 1;

        while i < lines.len()
            && should_reflow_comment_line(&lines[i], &configs[i])
            && configs[i].comment_width as usize == target_width
        {
            i += 1;
        }

        result.extend(reflow_comment_block(&lines[start..i], target_width));
    }

    result
}

fn should_reflow_comment_line(line: &str, config: &LineConfig) -> bool {
    config.comment_preservation == CommentPreservation::Reflow
        && is_standalone_comment_line(line)
        && !is_pragma_comment_line(line)
}

fn is_standalone_comment_line(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

fn is_pragma_comment_line(line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix('#') else {
        return false;
    };
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(PRAGMA_PREFIX) else {
        return false;
    };

    matches!(
        rest.split_whitespace().next(),
        Some("push" | "pop" | "off" | "on" | "skip")
    )
}

#[derive(Clone)]
struct ParsedCommentLine<'a> {
    outer_prefix: &'a str,
    leading_ws: &'a str,
    content: &'a str,
    raw: &'a str,
}

impl<'a> ParsedCommentLine<'a> {
    fn parse(line: &'a str) -> Option<Self> {
        let hash_idx = line.find('#')?;
        if !line[..hash_idx].chars().all(char::is_whitespace) {
            return None;
        }

        let outer_prefix = &line[..hash_idx];
        let after_hash = line[hash_idx + 1..].trim_end_matches([' ', '\t']);
        let ws_end = after_hash
            .char_indices()
            .find(|(_, ch)| !ch.is_whitespace())
            .map_or(after_hash.len(), |(idx, _)| idx);

        Some(Self {
            outer_prefix,
            leading_ws: &after_hash[..ws_end],
            content: &after_hash[ws_end..],
            raw: line,
        })
    }

    fn is_blank(&self) -> bool {
        self.content.is_empty()
    }

    fn is_fence_marker(&self) -> bool {
        self.content.trim() == "```"
    }

    fn leading_ws_len(&self) -> usize {
        self.leading_ws.len()
    }
}

struct ListMarker {
    prefix: String,
    text: String,
}

fn parse_list_marker(content: &str) -> Option<ListMarker> {
    let bytes = content.as_bytes();
    if bytes.is_empty() {
        return None;
    }

    let mut token_end = 0;
    if matches!(bytes[0], b'-' | b'*' | b'+') {
        token_end = 1;
    } else if bytes[0].is_ascii_digit() {
        while token_end < bytes.len() && bytes[token_end].is_ascii_digit() {
            token_end += 1;
        }
        if token_end == 0 || token_end >= bytes.len() {
            return None;
        }
        if !matches!(bytes[token_end], b'.' | b')') {
            return None;
        }
        token_end += 1;
    } else {
        return None;
    }

    let mut prefix_end = token_end;
    while prefix_end < bytes.len() && bytes[prefix_end].is_ascii_whitespace() {
        prefix_end += 1;
    }

    if prefix_end == token_end && token_end < bytes.len() {
        return None;
    }

    Some(ListMarker {
        prefix: content[..prefix_end].to_string(),
        text: content[prefix_end..].to_string(),
    })
}

fn wrap_comment_words(
    words: &[&str],
    width: usize,
    first_prefix: &str,
    continuation_prefix: &str,
) -> Vec<String> {
    if words.is_empty() {
        return vec![first_prefix.trim_end().to_string()];
    }

    let mut lines = Vec::new();
    let mut current_prefix = first_prefix;
    let mut current = current_prefix.to_string();
    let mut current_len = current_prefix.chars().count();

    for word in words {
        let word_len = word.chars().count();
        let prefix_len = current_prefix.chars().count();
        let needs_space = current_len > prefix_len;
        let candidate_len = current_len + usize::from(needs_space) + word_len;

        if candidate_len >= width && current_len > prefix_len {
            lines.push(current);
            current_prefix = continuation_prefix;
            current = current_prefix.to_string();
            current_len = current_prefix.chars().count();
        }

        let current_prefix_len = current_prefix.chars().count();
        if current_len > current_prefix_len {
            current.push(' ');
            current_len += 1;
        }

        current.push_str(word);
        current_len += word_len;
    }

    lines.push(current);
    lines
}

fn reflow_comment_block(lines: &[String], width: usize) -> Vec<String> {
    let parsed: Vec<ParsedCommentLine> = lines
        .iter()
        .filter_map(|line| ParsedCommentLine::parse(line))
        .collect();

    if parsed.is_empty() {
        return lines.to_vec();
    }

    let baseline = parsed
        .iter()
        .filter(|line| !line.is_blank())
        .map(ParsedCommentLine::leading_ws_len)
        .min()
        .unwrap_or(0);

    let mut result = Vec::new();
    let mut i = 0;
    let mut in_fence = false;

    while i < parsed.len() {
        let line = &parsed[i];

        if in_fence {
            result.push(line.raw.to_string());
            if line.is_fence_marker() {
                in_fence = false;
            }
            i += 1;
            continue;
        }

        if line.is_fence_marker() {
            result.push(line.raw.to_string());
            in_fence = true;
            i += 1;
            continue;
        }

        if line.is_blank() {
            result.push(format!("{}#", line.outer_prefix));
            i += 1;
            continue;
        }

        if line.leading_ws_len() >= baseline + 4 {
            result.push(line.raw.to_string());
            i += 1;
            continue;
        }

        if let Some(marker) = parse_list_marker(line.content) {
            let base_outer = line.outer_prefix;
            let base_indent = line.leading_ws_len();
            let continuation_indent = base_indent + marker.prefix.chars().count();
            let first_prefix = format!("{}#{}{}", base_outer, line.leading_ws, marker.prefix);
            let continuation_prefix = format!("{}#{}", base_outer, " ".repeat(continuation_indent));

            let words: Vec<&str> = marker.text.split_whitespace().collect();
            let mut continuation_lines: Vec<String> = Vec::new();

            i += 1;
            while i < parsed.len() {
                let next = &parsed[i];
                if next.outer_prefix != base_outer
                    || next.is_blank()
                    || next.is_fence_marker()
                    || next.leading_ws_len() >= baseline + 4
                {
                    break;
                }

                if parse_list_marker(next.content).is_some() {
                    if next.leading_ws_len() <= base_indent {
                        break;
                    }
                    continuation_lines.push(next.raw.to_string());
                    i += 1;
                    continue;
                }

                if next.leading_ws_len() > base_indent {
                    continuation_lines.push(next.raw.to_string());
                    i += 1;
                    continue;
                }

                break;
            }

            result.extend(wrap_comment_words(
                &words,
                width,
                &first_prefix,
                &continuation_prefix,
            ));
            result.extend(continuation_lines);
            continue;
        }

        let base_outer = line.outer_prefix;
        let base_ws = line.leading_ws;
        let prefix = format!("{}#{}", base_outer, base_ws);
        let mut words: Vec<&str> = line.content.split_whitespace().collect();

        i += 1;
        while i < parsed.len() {
            let next = &parsed[i];
            if next.outer_prefix != base_outer
                || next.is_blank()
                || next.is_fence_marker()
                || next.leading_ws_len() >= baseline + 4
            {
                break;
            }
            if parse_list_marker(next.content).is_some() {
                break;
            }
            if next.leading_ws != base_ws {
                break;
            }

            words.extend(next.content.split_whitespace());
            i += 1;
        }

        result.extend(wrap_comment_words(&words, width, &prefix, &prefix));
    }

    result
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::Configuration;

    fn default_config() -> Configuration {
        Configuration::default()
    }

    fn config_with_reflow() -> Configuration {
        let mut c = Configuration::default();
        c.comment_preservation = CommentPreservation::Reflow;
        c
    }

    fn config_with_set_align() -> Configuration {
        let mut c = Configuration::default();
        c.align_consecutive_set = true;
        c
    }

    fn config_with_trailing_comment_align() -> Configuration {
        let mut c = Configuration::default();
        c.align_trailing_comments = true;
        c.comment_gap = 2;
        c
    }

    // -----------------------------------------------------------------------
    // Fast path
    // -----------------------------------------------------------------------

    #[test]
    fn fast_path_no_features_returns_borrowed() {
        let input = "set(VAR val)\nset(B value)\n";
        let config = default_config();
        let result = post_process_alignments(input, &config);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, input);
    }

    // -----------------------------------------------------------------------
    // is_standalone_comment_line
    // -----------------------------------------------------------------------

    #[test]
    fn standalone_comment_line_detection() {
        assert!(is_standalone_comment_line("# hello"));
        assert!(is_standalone_comment_line("  # indented"));
        assert!(!is_standalone_comment_line("set(X) # trailing"));
        assert!(!is_standalone_comment_line(""));
        assert!(!is_standalone_comment_line("  "));
    }

    // -----------------------------------------------------------------------
    // is_pragma_comment_line
    // -----------------------------------------------------------------------

    #[test]
    fn pragma_comment_detection() {
        assert!(is_pragma_comment_line("# cmakefmt: off"));
        assert!(is_pragma_comment_line("# cmakefmt: on"));
        assert!(is_pragma_comment_line("# cmakefmt: skip"));
        assert!(is_pragma_comment_line(
            "# cmakefmt: push { lineWidth = 40 }"
        ));
        assert!(is_pragma_comment_line("# cmakefmt: pop"));
        assert!(!is_pragma_comment_line("# normal comment"));
        assert!(!is_pragma_comment_line("# cmakefmt: unknown"));
    }

    // -----------------------------------------------------------------------
    // parse_list_marker
    // -----------------------------------------------------------------------

    #[test]
    fn list_marker_dash() {
        let m = parse_list_marker("- item").unwrap();
        assert_eq!(m.prefix, "- ");
        assert_eq!(m.text, "item");
    }

    #[test]
    fn list_marker_star() {
        let m = parse_list_marker("* item").unwrap();
        assert_eq!(m.prefix, "* ");
        assert_eq!(m.text, "item");
    }

    #[test]
    fn list_marker_plus() {
        let m = parse_list_marker("+ item").unwrap();
        assert_eq!(m.prefix, "+ ");
        assert_eq!(m.text, "item");
    }

    #[test]
    fn list_marker_numbered() {
        let m = parse_list_marker("1. item").unwrap();
        assert_eq!(m.prefix, "1. ");
        assert_eq!(m.text, "item");
    }

    #[test]
    fn list_marker_numbered_paren() {
        let m = parse_list_marker("2) item").unwrap();
        assert_eq!(m.prefix, "2) ");
        assert_eq!(m.text, "item");
    }

    #[test]
    fn list_marker_no_space_after_dash_is_not_list() {
        assert!(parse_list_marker("-nospc").is_none());
    }

    #[test]
    fn list_marker_empty_content() {
        assert!(parse_list_marker("").is_none());
    }

    #[test]
    fn list_marker_non_list() {
        assert!(parse_list_marker("regular text").is_none());
    }

    // -----------------------------------------------------------------------
    // parse_push_body / is_pop_directive
    // -----------------------------------------------------------------------

    #[test]
    fn parse_push_body_valid() {
        let body = parse_push_body("# cmakefmt: push { lineWidth = 40 }").unwrap();
        assert!(body.starts_with('{'));
    }

    #[test]
    fn parse_push_body_not_push() {
        assert!(parse_push_body("# cmakefmt: pop").is_none());
    }

    #[test]
    fn is_pop_directive_valid() {
        assert!(is_pop_directive("# cmakefmt: pop"));
        assert!(is_pop_directive("  # cmakefmt: pop"));
    }

    #[test]
    fn is_pop_directive_invalid() {
        assert!(!is_pop_directive("# cmakefmt: push { }"));
        assert!(!is_pop_directive("regular line"));
    }

    // -----------------------------------------------------------------------
    // ParsedCommentLine
    // -----------------------------------------------------------------------

    #[test]
    fn parsed_comment_line_basic() {
        let p = ParsedCommentLine::parse("# hello world").unwrap();
        assert_eq!(p.outer_prefix, "");
        assert_eq!(p.content, "hello world");
        assert!(!p.is_blank());
    }

    #[test]
    fn parsed_comment_line_indented() {
        let p = ParsedCommentLine::parse("  # indented").unwrap();
        assert_eq!(p.outer_prefix, "  ");
        assert_eq!(p.content, "indented");
    }

    #[test]
    fn parsed_comment_line_blank() {
        let p = ParsedCommentLine::parse("#").unwrap();
        assert!(p.is_blank());
    }

    #[test]
    fn parsed_comment_line_fence() {
        let p = ParsedCommentLine::parse("# ```").unwrap();
        assert!(p.is_fence_marker());
    }

    #[test]
    fn parsed_comment_line_not_comment() {
        assert!(ParsedCommentLine::parse("not a comment").is_none());
    }

    // -----------------------------------------------------------------------
    // wrap_comment_words
    // -----------------------------------------------------------------------

    #[test]
    fn wrap_words_single_line() {
        let words = vec!["hello", "world"];
        let result = wrap_comment_words(&words, 80, "# ", "# ");
        assert_eq!(result, vec!["# hello world"]);
    }

    #[test]
    fn wrap_words_forces_break() {
        let words = vec!["hello", "world"];
        let result = wrap_comment_words(&words, 12, "# ", "# ");
        assert_eq!(result, vec!["# hello", "# world"]);
    }

    #[test]
    fn wrap_words_empty() {
        let words: Vec<&str> = vec![];
        let result = wrap_comment_words(&words, 80, "# ", "# ");
        assert_eq!(result, vec!["#"]);
    }

    // -----------------------------------------------------------------------
    // parse_set_call
    // -----------------------------------------------------------------------

    #[test]
    fn parse_set_call_basic() {
        let info = parse_set_call("  set(VAR value)").unwrap();
        assert_eq!(info.var_len, 3);
        assert!(info.value_start.is_some());
    }

    #[test]
    fn parse_set_call_valueless() {
        let info = parse_set_call("set(EMPTY)").unwrap();
        assert!(info.value_start.is_none());
    }

    #[test]
    fn parse_set_call_parent_scope_only() {
        let info = parse_set_call("set(VAR PARENT_SCOPE)").unwrap();
        assert!(info.value_start.is_none());
    }

    #[test]
    fn parse_set_call_with_value_and_parent_scope() {
        // The second token is the value, not PARENT_SCOPE
        let info = parse_set_call("set(VAR some_value PARENT_SCOPE)").unwrap();
        assert!(info.value_start.is_some());
    }

    #[test]
    fn parse_set_call_not_set_command() {
        assert!(parse_set_call("message(STATUS \"hi\")").is_none());
    }

    #[test]
    fn parse_set_call_multiline_rejected() {
        // No closing paren on the line
        assert!(parse_set_call("set(VAR").is_none());
    }

    #[test]
    fn parse_set_call_case_insensitive() {
        assert!(parse_set_call("SET(VAR val)").is_some());
        assert!(parse_set_call("Set(VAR val)").is_some());
    }

    // -----------------------------------------------------------------------
    // parse_trailing_comment
    // -----------------------------------------------------------------------

    #[test]
    fn trailing_comment_basic() {
        let info = parse_trailing_comment("set(VAR val) # comment").unwrap();
        assert_eq!(info.comment, "# comment");
        assert_eq!(info.code_len, "set(VAR val)".len());
    }

    #[test]
    fn trailing_comment_standalone_is_none() {
        assert!(parse_trailing_comment("# standalone comment").is_none());
    }

    #[test]
    fn trailing_comment_blank_line_is_none() {
        assert!(parse_trailing_comment("").is_none());
        assert!(parse_trailing_comment("   ").is_none());
    }

    #[test]
    fn trailing_comment_inside_quoted_string_ignored() {
        // The `#` inside the quoted string should not be treated as a comment
        let result = parse_trailing_comment("set(VAR \"value # not comment\")");
        assert!(result.is_none());
    }

    #[test]
    fn trailing_comment_after_quoted_string() {
        let info = parse_trailing_comment("set(VAR \"value\") # comment").unwrap();
        assert_eq!(info.comment, "# comment");
    }

    #[test]
    fn trailing_comment_escaped_quote_in_string() {
        // Escaped quote inside string shouldn't confuse the parser
        let info = parse_trailing_comment(r#"set(VAR "val\"ue") # comment"#).unwrap();
        assert_eq!(info.comment, "# comment");
    }

    // -----------------------------------------------------------------------
    // align_consecutive_set_lines (integration-style)
    // -----------------------------------------------------------------------

    #[test]
    fn set_alignment_two_consecutive() {
        let config = config_with_set_align();
        let input = "set(A value)\nset(LONGVAR value)\n";
        let result = post_process_alignments(input, &config);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        // Both values should be at the same column
        let v1_pos = lines[0].find("value").unwrap();
        let v2_pos = lines[1].find("value").unwrap();
        assert_eq!(v1_pos, v2_pos);
    }

    #[test]
    fn set_alignment_broken_by_blank_line() {
        let config = config_with_set_align();
        let input = "set(A value)\n\nset(LONGVAR value)\n";
        let result = post_process_alignments(input, &config);
        let lines: Vec<&str> = result.lines().collect();
        // Groups should NOT be aligned together (blank line breaks the group)
        let v1_pos = lines[0].find("value").unwrap();
        let v2_pos = lines[2].find("value").unwrap();
        // They should have different positions because LONGVAR is longer
        assert_ne!(v1_pos, v2_pos);
    }

    // -----------------------------------------------------------------------
    // align_trailing_comment_lines (integration-style)
    // -----------------------------------------------------------------------

    #[test]
    fn trailing_comment_alignment_two_lines() {
        let config = config_with_trailing_comment_align();
        let input = "set(A val) # comment1\nset(LONGVAR val) # comment2\n";
        let result = post_process_alignments(input, &config);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        // Comments should be aligned
        let c1_pos = lines[0].find("# comment1").unwrap();
        let c2_pos = lines[1].find("# comment2").unwrap();
        assert_eq!(c1_pos, c2_pos);
    }

    // -----------------------------------------------------------------------
    // reflow (integration-style)
    // -----------------------------------------------------------------------

    #[test]
    fn reflow_long_comment_wraps() {
        let mut config = config_with_reflow();
        config.line_width = 30;
        config.comment_width = Some(30);
        let input = "# This is a very long comment that should be reflowed to a shorter width\n";
        let result = post_process_alignments(input, &config);
        // All lines should be <= 30 chars
        for line in result.lines() {
            assert!(
                line.len() <= 30,
                "line too long ({} chars): {line}",
                line.len()
            );
        }
    }

    #[test]
    fn reflow_preserves_fence_blocks() {
        let config = config_with_reflow();
        let input = "# ```\n# code inside fence block that should not be reflowed at all\n# ```\n";
        let result = post_process_alignments(input, &config);
        // Fence blocks should be preserved verbatim
        assert!(result.contains("code inside fence block"));
    }

    #[test]
    fn reflow_preserves_blank_comment_lines() {
        let config = config_with_reflow();
        let input = "# paragraph one\n#\n# paragraph two\n";
        let result = post_process_alignments(input, &config);
        // Blank comment line should be preserved as paragraph separator
        assert!(result.contains("#\n"));
    }
}

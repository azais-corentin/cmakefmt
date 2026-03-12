use std::path::Path;

use crate::printer::{PrintOptions, format as printer_format};
use anyhow::Result;
use glob::Pattern;
use tracing::info_span;

use crate::configuration::{Configuration, NewLineKind, apply_inline_overrides};
use crate::generation::gen_file;
use crate::instrumentation::{
    EVENT_FORMAT_BOM_STRIP, EVENT_FORMAT_BYPASS_CHECK, EVENT_FORMAT_FINAL_NEWLINE,
    EVENT_FORMAT_FINALIZE_WHITESPACE, EVENT_FORMAT_GENERATE_IR, EVENT_FORMAT_NORMALIZE_BARE_CR,
    EVENT_FORMAT_PARSE, EVENT_FORMAT_PIPELINE, EVENT_FORMAT_POST_PROCESS, EVENT_FORMAT_PRINT,
    EVENT_FORMAT_RESOLVE_OPTIONS, EVENT_FORMAT_RESTORE_BARE_CR, span_format_invocation,
};
use crate::parser;

pub fn format_text(path: &Path, input: &str, config: &Configuration) -> Result<Option<String>> {
    let invocation_span = span_format_invocation(path, input.len());
    let _invocation_entered = invocation_span.enter();

    let bypassed = {
        let _stage = info_span!(
            EVENT_FORMAT_BYPASS_CHECK,
            ignore_pattern_count = config.ignore_patterns.len()
        )
        .entered();
        should_bypass_formatting(path, config)
    };
    invocation_span.record("bypassed", bypassed);
    if bypassed {
        return Ok(None);
    }

    let text = {
        let _stage = info_span!(EVENT_FORMAT_BOM_STRIP).entered();
        strip_bom(input)
    };
    let result = {
        let _stage = info_span!(EVENT_FORMAT_PIPELINE).entered();
        format_inner(text, config)?
    };
    let changed = result != input;
    invocation_span.record("changed", changed);
    if changed { Ok(Some(result)) } else { Ok(None) }
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

const PRAGMA_PREFIX: &str = "cmakefmt:";

enum LeadingPragmaDirective<'a> {
    Push(&'a str),
    Pop,
}

fn parse_leading_pragma(comment: &str) -> Option<LeadingPragmaDirective<'_>> {
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
        "push" => {
            let body = remainder.trim_start();
            if body.starts_with('{') {
                Some(LeadingPragmaDirective::Push(body))
            } else {
                None
            }
        }
        "pop" => Some(LeadingPragmaDirective::Pop),
        _ => None,
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

        match parse_leading_pragma(trimmed) {
            Some(LeadingPragmaDirective::Push(body)) => {
                stack.push(current.clone());
                current = apply_inline_overrides(&current, body);
            }
            Some(LeadingPragmaDirective::Pop) => {
                if let Some(previous) = stack.pop() {
                    current = previous;
                }
            }
            None => {}
        }
    }

    current
}

fn format_inner(text: &str, config: &Configuration) -> Result<String> {
    // Normalize bare CRs (\r not followed by \n) to \n for parsing.
    // The spec says bare CRs are ordinary characters, not line endings.
    // We normalize for the parser, then restore in the output.
    let bare_cr = has_bare_cr(text);
    let normalized;
    let parse_text = if bare_cr {
        let _stage = info_span!(EVENT_FORMAT_NORMALIZE_BARE_CR, input_bytes = text.len()).entered();
        normalized = normalize_bare_crs(text);
        normalized.as_str()
    } else {
        text
    };

    let file = {
        let _stage = info_span!(EVENT_FORMAT_PARSE, input_bytes = parse_text.len()).entered();
        parser::parse(parse_text)?
    };

    // Detect line ending from the ORIGINAL text (bare CRs excluded from counting).
    let newline = resolve_new_line_kind(text, config.new_line_kind);
    let print_config = {
        let _stage = info_span!(EVENT_FORMAT_RESOLVE_OPTIONS).entered();
        resolve_print_options_config(parse_text, config)
    };
    let print_options = PrintOptions {
        max_width: print_config.line_width,
        indent_width: print_config.indent_width,
        use_tabs: print_config.use_tabs,
        new_line_text: newline,
    };

    let result = {
        let _stage = info_span!(EVENT_FORMAT_PRINT).entered();
        printer_format(
            || {
                let _gen_stage = info_span!(
                    EVENT_FORMAT_GENERATE_IR,
                    file_elements = file.elements.len()
                )
                .entered();
                gen_file(&file, parse_text, config)
            },
            print_options,
        )
    };
    let result = {
        let _stage = info_span!(EVENT_FORMAT_POST_PROCESS).entered();
        crate::post_process::post_process_alignments(&result, config)
    };

    // Restore whitespace where config says to preserve (trimTrailingWhitespace,
    // collapseSpaces) by comparing formatted output with original input.
    let result = {
        let _stage = info_span!(EVENT_FORMAT_FINALIZE_WHITESPACE).entered();
        finalize_whitespace(&result, parse_text, config, newline)
    };

    // Apply finalNewline semantics.
    let result = {
        let _stage = info_span!(EVENT_FORMAT_FINAL_NEWLINE).entered();
        apply_final_newline(&result, text, config, newline)
    };

    // Restore bare CRs at their original positions.
    let result = if bare_cr {
        let _stage = info_span!(EVENT_FORMAT_RESTORE_BARE_CR).entered();
        restore_bare_crs(&result, text)
    } else {
        result
    };

    Ok(result)
}

// ---------------------------------------------------------------------------
// Line-ending detection
// ---------------------------------------------------------------------------

/// Count dominant line-ending style. CRLF is counted as one unit; bare `\r`
/// (not followed by `\n`) is not counted per §7.1. On tie or no endings, LF wins.
fn detect_dominant_line_ending(text: &str) -> &'static str {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut lf: u32 = 0;
    let mut crlf: u32 = 0;
    let mut i = 0;
    while i < len {
        if bytes[i] == b'\r' {
            if i + 1 < len && bytes[i + 1] == b'\n' {
                crlf += 1;
                i += 2;
                continue;
            }
            // Bare CR — not counted as a line ending.
        } else if bytes[i] == b'\n' {
            lf += 1;
        }
        i += 1;
    }
    if crlf > lf { "\r\n" } else { "\n" }
}

fn resolve_new_line_kind(text: &str, kind: NewLineKind) -> &'static str {
    match kind {
        NewLineKind::Lf => "\n",
        NewLineKind::CrLf => "\r\n",
        NewLineKind::Auto => detect_dominant_line_ending(text),
    }
}

// ---------------------------------------------------------------------------
// BOM
// ---------------------------------------------------------------------------

fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{feff}').unwrap_or(text)
}

// ---------------------------------------------------------------------------
// Bare carriage-return handling (§7.1)
// ---------------------------------------------------------------------------

/// Returns `true` if `text` contains any bare `\r` (not part of `\r\n`).
fn has_bare_cr(text: &str) -> bool {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == b'\r' {
            if i + 1 < len && bytes[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            return true;
        }
        i += 1;
    }
    false
}

/// Replace bare `\r` with `\n` so the parser sees them as line separators.
fn normalize_bare_crs(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = Vec::with_capacity(len);
    let mut i = 0;
    while i < len {
        if bytes[i] == b'\r' {
            if i + 1 < len && bytes[i + 1] == b'\n' {
                out.push(b'\r');
                out.push(b'\n');
                i += 2;
                continue;
            }
            // Bare CR → LF for parsing.
            out.push(b'\n');
        } else {
            out.push(bytes[i]);
        }
        i += 1;
    }
    // Safety: we only replaced \r with \n; the rest is preserved byte-for-byte.
    // If the input was valid UTF-8, the output is too.
    String::from_utf8(out).expect("bare CR normalization preserved UTF-8")
}

/// Restore bare CRs in the formatted output at positions where the original
/// had them. We use a line-based approach: split the original by `\n` into
/// "super-lines" (each may contain bare `\r`), count how many bare CRs each
/// super-line contributes, then reconstruct the output using `\r` where
/// appropriate.
fn restore_bare_crs(formatted: &str, original: &str) -> String {
    // Each segment between \n in the original is a "super-line". If it contains
    // N bare CRs, that super-line was split into N+1 formatted lines during
    // normalization. We reverse that: consume N+1 formatted lines and rejoin
    // with \r, then join super-lines with \n.
    let orig_segments: Vec<&str> = original.split('\n').collect();
    let fmt_lines: Vec<&str> = formatted.split('\n').collect();

    let mut result = String::with_capacity(formatted.len());
    let mut fmt_idx = 0;

    for (seg_i, seg) in orig_segments.iter().enumerate() {
        if seg_i > 0 {
            result.push('\n');
        }
        let bare_count = count_bare_crs_in(seg);
        let needed = bare_count + 1;
        for j in 0..needed {
            if j > 0 {
                result.push('\r');
            }
            if fmt_idx < fmt_lines.len() {
                result.push_str(fmt_lines[fmt_idx]);
                fmt_idx += 1;
            }
        }
    }
    // Append any extra formatted lines not consumed (defensive).
    while fmt_idx < fmt_lines.len() {
        result.push('\n');
        result.push_str(fmt_lines[fmt_idx]);
        fmt_idx += 1;
    }

    result
}

fn count_bare_crs_in(segment: &str) -> usize {
    let bytes = segment.as_bytes();
    let len = bytes.len();
    let mut count = 0;
    let mut i = 0;
    while i < len {
        if bytes[i] == b'\r' {
            if i + 1 < len && bytes[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            count += 1;
        }
        i += 1;
    }
    count
}

// ---------------------------------------------------------------------------
// Final-newline handling (§7.2)
// ---------------------------------------------------------------------------

/// Apply `finalNewline` semantics to the formatted output.
///
/// When `true` (default): the formatter already produces exactly one trailing
/// newline from `gen_file`, so no extra work is needed.
///
/// When `false`: strip the forced trailing newline if the original input had
/// none. Preserve existing trailing newlines from the original up to
/// `maxBlankLines`.
fn apply_final_newline(
    formatted: &str,
    original: &str,
    config: &Configuration,
    newline: &str,
) -> String {
    if config.final_newline {
        // finalNewline=true: ensure exactly one trailing newline.
        // gen_file strips trailing blanks and adds one Signal::NewLine,
        // so this is already correct for non-empty content.
        // For empty/whitespace-only input, gen_file produces just "\n".
        return formatted.to_string();
    }

    // --- finalNewline = false ---

    // If the formatted content is entirely whitespace / blank lines with no
    // real content, return empty (per §7.2: "the file is still subject to
    // trimTrailingWhitespace and maxBlankLines, which may reduce it to empty").
    let has_content = formatted
        .bytes()
        .any(|b| !matches!(b, b' ' | b'\t' | b'\r' | b'\n'));
    if !has_content {
        return String::new();
    }

    // Strip the trailing newline that gen_file always appends.
    let content_end = formatted
        .rfind(|c: char| c != '\n' && c != '\r')
        .map(|i| i + formatted[i..].chars().next().map_or(1, |c| c.len_utf8()))
        .unwrap_or(0);
    let base = &formatted[..content_end];

    // Count trailing newlines in the original input.
    let orig_trailing = count_trailing_newlines(original, newline);
    if orig_trailing == 0 {
        // Original had no trailing newline → don't add one.
        return base.to_string();
    }

    // Preserve trailing newlines from the original, capped by maxBlankLines.
    // maxBlankLines limits blank lines (each blank line = one extra newline
    // beyond the content-terminating newline).
    let max_allowed = (config.max_blank_lines as usize) + 1;
    let trailing = orig_trailing.min(max_allowed);
    let mut result = base.to_string();
    for _ in 0..trailing {
        result.push_str(newline);
    }
    result
}

/// Count how many consecutive `newline` sequences appear at the end of `text`.
fn count_trailing_newlines(text: &str, newline: &str) -> usize {
    let mut count = 0;
    let mut rest = text;
    while let Some(stripped) = rest.strip_suffix(newline) {
        count += 1;
        rest = stripped;
    }
    count
}

// ---------------------------------------------------------------------------
// Whitespace finalization (§8.1, §8.2)
// ---------------------------------------------------------------------------

/// Restore original whitespace in regions where `trimTrailingWhitespace=false`
/// or `collapseSpaces=false` via pragma push/pop.
///
/// The dprint IR generation always produces clean output (single spaces, no
/// trailing whitespace). When these options are `false`, we compare the
/// formatted output line-by-line with the original input and restore the
/// original line's whitespace where the non-whitespace content matches.
fn finalize_whitespace(
    formatted: &str,
    original: &str,
    config: &Configuration,
    newline: &str,
) -> String {
    // Quick exit: if both options are true at the base level and no pragmas
    // could override them, skip the pass entirely.
    if config.trim_trailing_whitespace
        && config.collapse_spaces
        && !formatted.contains(PRAGMA_PREFIX)
    {
        return formatted.to_string();
    }

    let out_lines: Vec<&str> = formatted.split(newline).collect();
    let in_lines: Vec<&str> = original.split(newline).collect();

    let mut current = config.clone();
    let mut stack: Vec<Configuration> = Vec::new();

    let mut result_lines: Vec<String> = Vec::with_capacity(out_lines.len());

    for (i, out_line) in out_lines.iter().enumerate() {
        let trimmed = out_line.trim();

        // Track pragma push/pop to maintain per-line config.
        if trimmed.starts_with('#') {
            match parse_leading_pragma(trimmed) {
                Some(LeadingPragmaDirective::Push(body)) => {
                    stack.push(current.clone());
                    current = apply_inline_overrides(&current, body);
                }
                Some(LeadingPragmaDirective::Pop) => {
                    if let Some(prev) = stack.pop() {
                        current = prev;
                    }
                }
                None => {}
            }
        }

        let mut line = (*out_line).to_string();

        // Only attempt restoration when we have a corresponding input line.
        if i < in_lines.len() {
            let in_line = in_lines[i];

            // collapseSpaces=false: restore original inter-argument spacing.
            if !current.collapse_spaces {
                let out_collapsed = collapse_inline_spaces(out_line);
                let in_collapsed = collapse_inline_spaces(in_line);
                if out_collapsed == in_collapsed {
                    line = in_line.to_string();
                }
            }

            // trimTrailingWhitespace=false: restore original trailing whitespace.
            if !current.trim_trailing_whitespace {
                let out_trimmed = out_line.trim_end();
                let in_trimmed = in_line.trim_end();
                if out_trimmed == in_trimmed {
                    line = in_line.to_string();
                }
            }
        }

        result_lines.push(line);
    }

    result_lines.join(newline)
}

/// Collapse runs of multiple spaces to a single space, preserving leading
/// indentation and content inside quoted strings.
fn collapse_inline_spaces(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let len = bytes.len();

    // Preserve leading whitespace.
    let mut i = 0;
    while i < len && matches!(bytes[i], b' ' | b'\t') {
        result.push(bytes[i] as char);
        i += 1;
    }

    let mut in_quote = false;
    let mut prev_space = false;
    while i < len {
        let c = bytes[i];
        if c == b'"' {
            in_quote = !in_quote;
            prev_space = false;
            result.push('"');
        } else if c == b'\\' && in_quote && i + 1 < len {
            // Escaped char inside quotes — preserve both bytes.
            result.push(bytes[i] as char);
            result.push(bytes[i + 1] as char);
            i += 2;
            prev_space = false;
            continue;
        } else if c == b' ' && !in_quote {
            if !prev_space {
                result.push(' ');
            }
            prev_space = true;
        } else {
            prev_space = false;
            result.push(c as char);
        }
        i += 1;
    }

    result
}

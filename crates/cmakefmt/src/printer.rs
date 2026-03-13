//! Minimal print engine for cmakefmt.
//!
//! Replaces the subset of `dprint_core::formatting` that cmakefmt uses. The generation layer
//! builds a linear stream of [`PrintItem`]s (strings, spaces, signals); this module walks that
//! stream and renders it to a `String`, handling indentation, newlines, and ignore-indent regions.
//!
//! No backtracking, conditions, or line-width measurement — cmakefmt makes all layout decisions
//! in the generation layer.

use tracing::info_span;

use crate::instrumentation::EVENT_PRINTER_FORMAT;

// ── IR types ────────────────────────────────────────────────────────────────

/// A signal that controls printer state (indentation, newlines).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// Emit a line break.
    NewLine,
    /// Increase the indentation level by one.
    StartIndent,
    /// Decrease the indentation level by one.
    FinishIndent,
    /// Begin a region where indentation is suppressed (nesting-safe counter).
    StartIgnoringIndent,
    /// End a region where indentation is suppressed.
    FinishIgnoringIndent,
    /// Emit a literal tab character (inside raw strings split on `\t`).
    Tab,
}

/// A single item in the print stream.
#[derive(Debug, Clone)]
pub enum PrintItem {
    /// An owned text fragment. Never contains `\n` — newlines are always [`Signal::NewLine`].
    String(String),
    /// A static text fragment used for punctuation like `(`, `)`, and `,`.
    Static(&'static str),
    /// A single space character.
    Space,
    /// A printer signal (indent, newline, etc.).
    Signal(Signal),
}

/// A linear sequence of print items produced by the generation layer.
#[derive(Debug, Clone, Default)]
pub struct PrintItems {
    items: Vec<PrintItem>,
}

impl PrintItems {
    /// Create an empty item list.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Create an item list pre-allocated for `cap` items.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
        }
    }

    /// Append a signal.
    pub fn push_signal(&mut self, signal: Signal) {
        self.items.push(PrintItem::Signal(signal));
    }

    /// Append an owned string fragment.
    pub fn push_string(&mut self, text: String) {
        if !text.is_empty() {
            self.items.push(PrintItem::String(text));
        }
    }

    /// Append a static string fragment.
    ///
    /// In dprint-core this computes display width for CJK characters; cmakefmt only calls it
    /// for single ASCII characters like `(`, `)`, and `,`, so width computation is unnecessary.
    pub fn push_str_runtime_width_computed(&mut self, text: &'static str) {
        if !text.is_empty() {
            self.items.push(PrintItem::Static(text));
        }
    }

    /// Append a single space.
    pub fn push_space(&mut self) {
        self.items.push(PrintItem::Space);
    }

    /// Append all items from another `PrintItems`, consuming it.
    pub fn extend(&mut self, other: PrintItems) {
        self.items.extend(other.items);
    }

    /// Append `other`'s items wrapped in one indent level, consuming `other`.
    /// Avoids the intermediate Vec allocation of `extend(with_indent(other))`.
    pub fn push_indented(&mut self, other: PrintItems) {
        if other.is_empty() {
            return;
        }
        self.items.reserve(other.items.len() + 2);
        self.items.push(PrintItem::Signal(Signal::StartIndent));
        self.items.extend(other.items);
        self.items.push(PrintItem::Signal(Signal::FinishIndent));
    }

    /// Append `other`'s items wrapped in `times` indent levels, consuming `other`.
    pub fn push_indented_times(&mut self, other: PrintItems, times: u32) {
        if other.is_empty() || times == 0 {
            self.items.extend(other.items);
            return;
        }
        self.items.reserve(other.items.len() + (times as usize) * 2);
        for _ in 0..times {
            self.items.push(PrintItem::Signal(Signal::StartIndent));
        }
        self.items.extend(other.items);
        for _ in 0..times {
            self.items.push(PrintItem::Signal(Signal::FinishIndent));
        }
    }

    /// Returns `true` if no items have been pushed.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ── IR helpers ──────────────────────────────────────────────────────────────

/// Convenience functions that mirror `dprint_core::formatting::ir_helpers`.
pub mod ir_helpers {
    use super::{PrintItem, PrintItems, Signal};

    /// Wrap `items` in a single indent level.
    pub fn with_indent(items: PrintItems) -> PrintItems {
        with_indent_times(items, 1)
    }

    /// Wrap `items` in `times` indent levels.
    pub fn with_indent_times(items: PrintItems, times: u32) -> PrintItems {
        if items.is_empty() {
            return items;
        }

        let extra = (times as usize) * 2;
        let mut out = Vec::with_capacity(items.items.len() + extra);

        for _ in 0..times {
            out.push(PrintItem::Signal(Signal::StartIndent));
        }
        out.extend(items.items);
        for _ in 0..times {
            out.push(PrintItem::Signal(Signal::FinishIndent));
        }

        PrintItems { items: out }
    }

    /// Generate IR from a raw (potentially multi-line) string.
    ///
    /// If the string contains newlines, the entire output is wrapped in
    /// `StartIgnoringIndent`/`FinishIgnoringIndent` so the content is emitted verbatim.
    /// Each line is split on `\t` to emit `Signal::Tab` for literal tabs.
    pub fn gen_from_raw_string(text: &str) -> PrintItems {
        let has_newline = text.contains('\n');
        let mut items = PrintItems::new();

        if has_newline {
            items.push_signal(Signal::StartIgnoringIndent);
            gen_string_lines_into(&mut items, text);
            items.push_signal(Signal::FinishIgnoringIndent);
        } else {
            gen_line_into(&mut items, text);
        }

        items
    }

    /// Append IR for a multi-line string (lines separated by `Signal::NewLine`).
    fn gen_string_lines_into(items: &mut PrintItems, text: &str) {
        for (i, line) in text.lines().enumerate() {
            if i > 0 {
                items.push_signal(Signal::NewLine);
            }
            gen_line_into(items, line);
        }

        // `str::lines()` drops a trailing empty line; restore it.
        if text.ends_with('\n') {
            items.push_signal(Signal::NewLine);
        }
    }

    /// Append IR for a single line, splitting on `\t` to emit `Signal::Tab`.
    fn gen_line_into(items: &mut PrintItems, line: &str) {
        if !line.contains('\t') {
            if !line.is_empty() {
                items.items.push(PrintItem::String(line.to_string()));
            }
            return;
        }
        for (i, part) in line.split('\t').enumerate() {
            if i > 0 {
                items.push_signal(Signal::Tab);
            }
            if !part.is_empty() {
                items.items.push(PrintItem::String(part.to_string()));
            }
        }
    }
}

// ── Printer options ─────────────────────────────────────────────────────────

/// Options that control how the print stream is rendered to text.
#[derive(Debug, Clone)]
pub struct PrintOptions {
    /// Target line width (used by the generation layer, not the printer itself).
    pub max_width: u32,
    /// Number of spaces per indentation level (also used as tab display width).
    pub indent_width: u8,
    /// Use tabs instead of spaces for indentation.
    pub use_tabs: bool,
    /// The string to emit for each newline (`"\n"` or `"\r\n"`).
    pub new_line_text: &'static str,
    /// Initial output buffer capacity hint in bytes.
    pub initial_capacity: usize,
}

// ── Printer ─────────────────────────────────────────────────────────────────

/// Render `PrintItems` to a `String`.
///
/// Mirrors `dprint_core::formatting::format`: accepts a closure that produces the items and
/// options that control rendering. The closure form exists for API compatibility.
pub fn format(get_items: impl FnOnce() -> PrintItems, options: PrintOptions) -> String {
    let _stage = info_span!(EVENT_PRINTER_FORMAT).entered();
    let items = get_items();
    render(&items, &options)
}

/// Walk the item stream and produce the final formatted text.
fn render(items: &PrintItems, options: &PrintOptions) -> String {
    let mut out = String::with_capacity(options.initial_capacity);
    let mut indent_level: u8 = 0;
    let mut ignore_indent_count: u8 = 0;
    let mut at_line_start = true;
    // Cache expanded indent prefixes by level (index = indent level).
    // This avoids rebuilding the same N-space or N-tab prefixes for every line.
    let mut indent_cache = vec![String::new()];

    for item in &items.items {
        match item {
            PrintItem::Signal(signal) => match signal {
                Signal::NewLine => {
                    out.push_str(options.new_line_text);
                    at_line_start = true;
                }
                Signal::StartIndent => {
                    indent_level = indent_level.saturating_add(1);
                    // If still at line start, the indent hasn't been emitted yet,
                    // so the new level will take effect on the next text output.
                    // This matches dprint's `set_indent_level` behavior which updates
                    // `last_line_indent_level` when `current_line_column == 0`.
                }
                Signal::FinishIndent => {
                    indent_level = indent_level.saturating_sub(1);
                }
                Signal::StartIgnoringIndent => {
                    ignore_indent_count = ignore_indent_count.saturating_add(1);
                }
                Signal::FinishIgnoringIndent => {
                    ignore_indent_count = ignore_indent_count.saturating_sub(1);
                }
                Signal::Tab => {
                    emit_indent_if_needed(
                        &mut out,
                        &mut at_line_start,
                        indent_level,
                        ignore_indent_count,
                        options,
                        &mut indent_cache,
                    );
                    out.push('\t');
                }
            },
            PrintItem::String(text) => {
                emit_indent_if_needed(
                    &mut out,
                    &mut at_line_start,
                    indent_level,
                    ignore_indent_count,
                    options,
                    &mut indent_cache,
                );
                out.push_str(text);
            }
            PrintItem::Static(text) => {
                emit_indent_if_needed(
                    &mut out,
                    &mut at_line_start,
                    indent_level,
                    ignore_indent_count,
                    options,
                    &mut indent_cache,
                );
                out.push_str(text);
            }
            PrintItem::Space => {
                emit_indent_if_needed(
                    &mut out,
                    &mut at_line_start,
                    indent_level,
                    ignore_indent_count,
                    options,
                    &mut indent_cache,
                );
                out.push(' ');
            }
        }
    }

    out
}

/// If we are at the start of a line and indentation is not suppressed, emit the indent prefix.
#[inline]
fn emit_indent_if_needed(
    out: &mut String,
    at_line_start: &mut bool,
    indent_level: u8,
    ignore_indent_count: u8,
    options: &PrintOptions,
    indent_cache: &mut Vec<String>,
) {
    if *at_line_start {
        *at_line_start = false;
        if indent_level > 0 && ignore_indent_count == 0 {
            let indent_level = indent_level as usize;
            ensure_indent_prefix(indent_cache, indent_level, options);
            out.push_str(&indent_cache[indent_level]);
        }
    }
}

#[inline]
fn ensure_indent_prefix(
    indent_cache: &mut Vec<String>,
    indent_level: usize,
    options: &PrintOptions,
) {
    while indent_cache.len() <= indent_level {
        let mut prefix = indent_cache[indent_cache.len() - 1].clone();
        if options.use_tabs {
            prefix.push('\t');
        } else {
            for _ in 0..options.indent_width {
                prefix.push(' ');
            }
        }
        indent_cache.push(prefix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_items_produce_empty_string() {
        let result = format(
            PrintItems::new,
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "");
    }

    #[test]
    fn basic_indentation_with_spaces() {
        let result = format(
            || {
                let mut items = PrintItems::new();
                items.push_string("hello".into());
                items.push_signal(Signal::NewLine);
                items.push_signal(Signal::StartIndent);
                items.push_string("world".into());
                items.push_signal(Signal::FinishIndent);
                items.push_signal(Signal::NewLine);
                items
            },
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "hello\n  world\n");
    }

    #[test]
    fn basic_indentation_with_tabs() {
        let result = format(
            || {
                let mut items = PrintItems::new();
                items.push_signal(Signal::StartIndent);
                items.push_string("indented".into());
                items.push_signal(Signal::FinishIndent);
                items
            },
            PrintOptions {
                max_width: 80,
                indent_width: 4,
                use_tabs: true,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "\tindented");
    }

    #[test]
    fn ignore_indent_region() {
        let result = format(
            || {
                let mut items = PrintItems::new();
                items.push_signal(Signal::StartIndent);
                items.push_string("indented".into());
                items.push_signal(Signal::NewLine);
                items.push_signal(Signal::StartIgnoringIndent);
                items.push_string("raw".into());
                items.push_signal(Signal::NewLine);
                items.push_string("also raw".into());
                items.push_signal(Signal::FinishIgnoringIndent);
                items.push_signal(Signal::NewLine);
                items.push_string("back to indent".into());
                items.push_signal(Signal::FinishIndent);
                items
            },
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "  indented\nraw\nalso raw\n  back to indent");
    }

    #[test]
    fn gen_from_raw_string_multiline() {
        let items = ir_helpers::gen_from_raw_string("line1\nline2\nline3");
        let result = format(
            || items,
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "line1\nline2\nline3");
    }

    #[test]
    fn gen_from_raw_string_with_tabs() {
        let items = ir_helpers::gen_from_raw_string("col1\tcol2");
        let result = format(
            || items,
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "col1\tcol2");
    }

    #[test]
    fn with_indent_wraps_correctly() {
        let mut inner = PrintItems::new();
        inner.push_string("body".into());
        let wrapped = ir_helpers::with_indent(inner);
        let result = format(
            || wrapped,
            PrintOptions {
                max_width: 80,
                indent_width: 4,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        // Indent emitted because at_line_start is true at the beginning
        assert_eq!(result, "    body");
    }

    #[test]
    fn with_indent_times_wraps_correctly() {
        let mut inner = PrintItems::new();
        inner.push_string("deep".into());
        let wrapped = ir_helpers::with_indent_times(inner, 3);
        let result = format(
            || wrapped,
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "      deep");
    }

    #[test]
    fn crlf_newlines() {
        let result = format(
            || {
                let mut items = PrintItems::new();
                items.push_string("a".into());
                items.push_signal(Signal::NewLine);
                items.push_string("b".into());
                items
            },
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\r\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "a\r\nb");
    }

    #[test]
    fn gen_from_raw_string_trailing_newline() {
        let items = ir_helpers::gen_from_raw_string("line1\n");
        let result = format(
            || items,
            PrintOptions {
                max_width: 80,
                indent_width: 2,
                use_tabs: false,
                new_line_text: "\n",
                initial_capacity: 0,
            },
        );
        assert_eq!(result, "line1\n");
    }
}

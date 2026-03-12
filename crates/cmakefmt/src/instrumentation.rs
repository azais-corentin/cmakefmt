use std::path::Path;

use tracing::{Span, info_span};

pub(crate) const EVENT_FORMAT_INVOCATION: &str = "cmakefmt.format.invocation";
pub(crate) const EVENT_FORMAT_BYPASS_CHECK: &str = "cmakefmt.format.bypass_check";
pub(crate) const EVENT_FORMAT_BOM_STRIP: &str = "cmakefmt.format.strip_bom";
pub(crate) const EVENT_FORMAT_PIPELINE: &str = "cmakefmt.format.pipeline";
pub(crate) const EVENT_FORMAT_NORMALIZE_BARE_CR: &str = "cmakefmt.format.normalize_bare_cr";
pub(crate) const EVENT_FORMAT_PARSE: &str = "cmakefmt.format.parse";
pub(crate) const EVENT_FORMAT_RESOLVE_OPTIONS: &str = "cmakefmt.format.resolve_print_options";
pub(crate) const EVENT_FORMAT_GENERATE_IR: &str = "cmakefmt.format.generate_ir";
pub(crate) const EVENT_FORMAT_PRINT: &str = "cmakefmt.format.print";
pub(crate) const EVENT_FORMAT_POST_PROCESS: &str = "cmakefmt.format.post_process";
pub(crate) const EVENT_FORMAT_FINALIZE_WHITESPACE: &str = "cmakefmt.format.finalize_whitespace";
pub(crate) const EVENT_FORMAT_FINAL_NEWLINE: &str = "cmakefmt.format.final_newline";
pub(crate) const EVENT_FORMAT_RESTORE_BARE_CR: &str = "cmakefmt.format.restore_bare_cr";
pub(crate) const EVENT_GEN_FILE: &str = "cmakefmt.gen_file";
pub(crate) const EVENT_GEN_FILE_COMMAND: &str = "cmakefmt.gen_file.command";
pub(crate) const EVENT_GEN_COMMAND: &str = "cmakefmt.gen_command";
pub(crate) const EVENT_POST_PROCESS: &str = "cmakefmt.post_process";
pub(crate) const EVENT_POST_PROCESS_ALIGN_BLOCK: &str = "cmakefmt.post_process.align_block";
pub(crate) const EVENT_POST_PROCESS_REFLOW_COMMENT: &str = "cmakefmt.post_process.reflow_comment";
pub(crate) const EVENT_PARSER_FILE: &str = "cmakefmt.parser.file";
pub(crate) const EVENT_PARSER_COMMAND: &str = "cmakefmt.parser.command";
pub(crate) const EVENT_PRINTER_FORMAT: &str = "cmakefmt.printer.format";

pub(crate) fn span_format_invocation(path: &Path, input_bytes: usize) -> Span {
    info_span!(
        EVENT_FORMAT_INVOCATION,
        path = %path.display(),
        input_bytes,
        changed = tracing::field::Empty,
        bypassed = tracing::field::Empty
    )
}

mod configuration;
mod format_text;
mod generation;
mod parser;
mod post_process;
pub mod printer;

pub use configuration::{
    CaseStyle, CommandConfiguration, CommentPreservation, ConfigDiagnostic,
    ConfigDiagnosticSeverity, ConfigLoadResult, Configuration, EndCommandArgs, GenexWrap,
    IndentStyle, NewLineKind, SortArguments, SpaceBeforeParen, SpaceInsideParen, WrapStyle,
    apply_inline_overrides, load_from_cli, load_from_header, load_from_json_map, load_from_toml,
    load_from_toml_path,
};
pub use format_text::format_text;

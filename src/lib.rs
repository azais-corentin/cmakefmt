mod configuration;
mod format_text;
mod generation;
mod post_process;
mod parser;

#[cfg(target_arch = "wasm32")]
mod wasm_plugin;

pub use configuration::{
    CaseStyle, CommandConfiguration, CommentPreservation, ConfigDiagnostic,
    ConfigDiagnosticSeverity, ConfigLoadResult, Configuration, EndCommandArgs, GenexWrap,
    IndentStyle, NewLineKind, SortArguments, SpaceBeforeParen, SpaceInsideParen, WrapStyle,
    apply_inline_overrides, load_from_cli, load_from_dprint, load_from_header, load_from_toml,
    load_from_toml_path, resolve_config,
};
pub use format_text::format_text;

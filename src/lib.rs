mod configuration;
mod format_text;
mod generation;
mod parser;

#[cfg(target_arch = "wasm32")]
mod wasm_plugin;

pub use configuration::{
    CaseStyle, ConfigDiagnostic, ConfigDiagnosticSeverity, ConfigLoadResult, Configuration,
    NewLineKind, load_from_cli, load_from_dprint, load_from_header, resolve_config,
};
pub use format_text::format_text;

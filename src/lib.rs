mod configuration;
mod format_text;
mod generation;
mod parser;

#[cfg(target_arch = "wasm32")]
mod wasm_plugin;

pub use configuration::{resolve_config, CaseStyle, Configuration, NewLineKind};
pub use format_text::format_text;

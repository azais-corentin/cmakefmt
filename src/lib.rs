mod configuration;
mod format_text;
mod generation;
mod parser;

#[cfg(target_arch = "wasm32")]
mod wasm_plugin;

pub use configuration::{CaseStyle, Configuration, NewLineKind, resolve_config};
pub use format_text::format_text;

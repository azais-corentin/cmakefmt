//! dprint plugin bridge for cmakefmt.
//!
//! This crate adapts the core `cmakefmt` library to dprint's plugin protocol. It is the only
//! crate in the workspace that depends on `dprint-core`.
//!
//! The plugin handler and generated WASM glue are only compiled for `wasm32` targets.
//! The config bridge types are always available for testing on native platforms.

use dprint_core::configuration::{
    ConfigKeyMap, ConfigKeyValue, ConfigurationDiagnostic, GlobalConfiguration,
    NewLineKind as DprintNewLineKind,
};
use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};

use cmakefmt::{ConfigLoadResult, Configuration, IndentStyle, NewLineKind, load_from_json_map};

// ── Config bridge ───────────────────────────────────────────────────────────

/// Convert a `ConfigKeyValue` into a `serde_json::Value`.
///
/// The mapping is 1:1 since both are untyped JSON-like value enums.
fn config_key_value_to_json(value: ConfigKeyValue) -> serde_json::Value {
    match value {
        ConfigKeyValue::Bool(b) => serde_json::Value::Bool(b),
        ConfigKeyValue::Number(n) => serde_json::Value::Number(serde_json::Number::from(n)),
        ConfigKeyValue::String(s) => serde_json::Value::String(s),
        ConfigKeyValue::Null => serde_json::Value::Null,
        ConfigKeyValue::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(config_key_value_to_json).collect())
        }
        ConfigKeyValue::Object(map) => {
            let obj = map
                .into_iter()
                .map(|(k, v)| (k, config_key_value_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
    }
}

/// Build a cmakefmt `Configuration` base from dprint's global defaults.
fn base_from_global(global_config: &GlobalConfiguration) -> Configuration {
    let mut config = Configuration::default();

    if let Some(line_width) = global_config.line_width {
        config.line_width = line_width;
    }

    if let Some(indent_width) = global_config.indent_width {
        config.indent_width = indent_width;
    }

    if let Some(use_tabs) = global_config.use_tabs {
        config.use_tabs = use_tabs;
        config.indent_style = if use_tabs {
            IndentStyle::Tab
        } else {
            IndentStyle::Space
        };
    }

    if let Some(new_line_kind) = global_config.new_line_kind {
        config.new_line_kind = match new_line_kind {
            DprintNewLineKind::Auto => NewLineKind::Auto,
            DprintNewLineKind::LineFeed => NewLineKind::Lf,
            DprintNewLineKind::CarriageReturnLineFeed => NewLineKind::CrLf,
        };
    }

    config
}

/// Load plugin configuration from dprint's key/value map and global defaults.
pub fn load_dprint_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ConfigLoadResult {
    let base = base_from_global(global_config);
    let map: serde_json::Map<String, serde_json::Value> = config
        .drain(..)
        .map(|(k, v)| (k, config_key_value_to_json(v)))
        .collect();
    load_from_json_map(map, base)
}

/// Resolve configuration into the format dprint expects.
pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration> {
    let load_result = load_dprint_config(config, global_config);
    let diagnostics = load_result
        .diagnostics
        .into_iter()
        .map(|d| ConfigurationDiagnostic {
            property_name: d.key,
            message: d.message,
        })
        .collect();

    PluginResolveConfigurationResult {
        config: load_result.config,
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec!["cmake".into()],
            file_names: vec!["CMakeLists.txt".into()],
        },
    }
}

// ── Plugin handler (wasm32 only) ────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
mod wasm {
    use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
    use dprint_core::generate_plugin_code;
    use dprint_core::plugins::{
        CheckConfigUpdatesMessage, ConfigChange, FormatResult, PluginInfo,
        PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest,
        SyncPluginHandler,
    };

    use cmakefmt::{Configuration, format_text};

    use super::resolve_config;

    struct CmakePluginHandler;

    impl SyncPluginHandler<Configuration> for CmakePluginHandler {
        fn resolve_config(
            &mut self,
            config: ConfigKeyMap,
            global_config: &GlobalConfiguration,
        ) -> PluginResolveConfigurationResult<Configuration> {
            resolve_config(config, global_config)
        }

        fn plugin_info(&mut self) -> PluginInfo {
            PluginInfo {
                name: String::from("dprint-plugin-cmake"),
                version: env!("CARGO_PKG_VERSION").to_string(),
                config_key: String::from("cmake"),
                help_url: String::new(),
                config_schema_url: String::new(),
                update_url: None,
            }
        }

        fn license_text(&mut self) -> String {
            include_str!("../../../LICENSE").to_string()
        }

        fn check_config_updates(
            &self,
            _message: CheckConfigUpdatesMessage,
        ) -> anyhow::Result<Vec<ConfigChange>> {
            Ok(Vec::new())
        }

        fn format(
            &mut self,
            request: SyncFormatRequest<Configuration>,
            _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
        ) -> FormatResult {
            let file_path = std::path::PathBuf::from(request.file_path);
            let text = String::from_utf8(request.file_bytes)?;
            let result = format_text(&file_path, &text, request.config)?;
            Ok(result.map(|s| s.into_bytes()))
        }
    }

    generate_plugin_code!(CmakePluginHandler, CmakePluginHandler);
}

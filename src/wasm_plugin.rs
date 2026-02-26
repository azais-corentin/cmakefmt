use std::path::PathBuf;

use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
use dprint_core::generate_plugin_code;
use dprint_core::plugins::{
    CheckConfigUpdatesMessage, ConfigChange, FormatResult, PluginInfo,
    PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

use crate::configuration::{resolve_config, Configuration};
use crate::format_text::format_text;

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
        include_str!("../LICENSE").to_string()
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
        let file_path = PathBuf::from(request.file_path);
        let text = String::from_utf8(request.file_bytes)?;
        let result = format_text(&file_path, &text, request.config)?;
        Ok(result.map(|s| s.into_bytes()))
    }
}

generate_plugin_code!(CmakePluginHandler, CmakePluginHandler);

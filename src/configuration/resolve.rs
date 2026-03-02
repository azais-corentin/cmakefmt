use dprint_core::configuration::{ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration};
use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};

use super::load::load_from_dprint;
use super::types::Configuration;

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration> {
    let load_result = load_from_dprint(config, global_config);
    let diagnostics = load_result
        .diagnostics
        .into_iter()
        .map(|diagnostic| ConfigurationDiagnostic {
            property_name: diagnostic.key,
            message: diagnostic.message,
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

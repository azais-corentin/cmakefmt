use std::path::{Path, PathBuf};
use std::sync::Arc;

use dprint_development::{run_specs, ParseSpecOptions, RunSpecsOptions};

use dprint_plugin_cmake::{format_text, resolve_config};

#[test]
fn test_specs() {
    let specs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/specs");

    run_specs(
        &specs_dir,
        &ParseSpecOptions {
            default_file_name: "CMakeLists.txt",
        },
        &RunSpecsOptions {
            fix_failures: std::env::var("FIX").map(|v| v == "1").unwrap_or(false),
            format_twice: true,
        },
        Arc::new(
            |path: &Path,
             file_text: &str,
             spec_config: &serde_json::Map<String, serde_json::Value>| {
                let global_config = dprint_core::configuration::GlobalConfiguration::default();
                let config_map: dprint_core::configuration::ConfigKeyMap = spec_config
                    .iter()
                    .filter_map(|(k, v)| {
                        let ckv = match v {
                            serde_json::Value::String(s) => {
                                dprint_core::configuration::ConfigKeyValue::String(s.clone())
                            }
                            serde_json::Value::Number(n) => {
                                dprint_core::configuration::ConfigKeyValue::Number(
                                    n.as_i64().unwrap_or(0) as i32,
                                )
                            }
                            serde_json::Value::Bool(b) => {
                                dprint_core::configuration::ConfigKeyValue::Bool(*b)
                            }
                            _ => return None,
                        };
                        Some((k.clone(), ckv))
                    })
                    .collect();
                let result = resolve_config(config_map, &global_config);
                format_text(path, file_text, &result.config)
            },
        ),
        Arc::new(|_path, _file_text, _spec_config| {
            String::new() // no trace support
        }),
    );
}

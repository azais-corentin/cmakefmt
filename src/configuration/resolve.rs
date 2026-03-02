use dprint_core::configuration::{
    ConfigKeyMap, GlobalConfiguration, get_unknown_property_diagnostics, get_value,
};
use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};

use super::types::{CaseStyle, Configuration, NewLineKind};

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration> {
    let mut config = config;
    let mut diagnostics = Vec::new();
    let defaults = Configuration::default();

    let line_width = get_value(
        &mut config,
        "lineWidth",
        global_config.line_width.unwrap_or(defaults.line_width),
        &mut diagnostics,
    );
    let indent_width = get_value(
        &mut config,
        "indentWidth",
        global_config.indent_width.unwrap_or(defaults.indent_width),
        &mut diagnostics,
    );
    let use_tabs = get_value(
        &mut config,
        "useTabs",
        global_config.use_tabs.unwrap_or(defaults.use_tabs),
        &mut diagnostics,
    );

    let new_line_kind_str: String = get_value(
        &mut config,
        "newLineKind",
        defaults.new_line_kind.to_string(),
        &mut diagnostics,
    );
    let new_line_kind = match new_line_kind_str.to_lowercase().as_str() {
        "lf" => NewLineKind::Lf,
        "crlf" => NewLineKind::CrLf,
        _ => NewLineKind::Auto,
    };

    let command_case_str: String = get_value(
        &mut config,
        "commandCase",
        defaults.command_case.to_string(),
        &mut diagnostics,
    );
    let command_case = match command_case_str.to_lowercase().as_str() {
        "upper" => CaseStyle::Upper,
        "preserve" => CaseStyle::Preserve,
        _ => CaseStyle::Lower,
    };

    let keyword_case_str: String = get_value(
        &mut config,
        "keywordCase",
        defaults.keyword_case.to_string(),
        &mut diagnostics,
    );
    let keyword_case = match keyword_case_str.to_lowercase().as_str() {
        "lower" => CaseStyle::Lower,
        "preserve" => CaseStyle::Preserve,
        _ => CaseStyle::Upper,
    };

    let closing_paren_newline = get_value(
        &mut config,
        "closingParenNewline",
        defaults.closing_paren_newline,
        &mut diagnostics,
    );
    let sort_lists = get_value(
        &mut config,
        "sortLists",
        defaults.sort_lists,
        &mut diagnostics,
    );
    let max_blank_lines: u8 = get_value(
        &mut config,
        "maxBlankLines",
        defaults.max_blank_lines,
        &mut diagnostics,
    );
    let space_before_paren = get_value(
        &mut config,
        "spaceBeforeParen",
        defaults.space_before_paren,
        &mut diagnostics,
    );

    diagnostics.extend(get_unknown_property_diagnostics(config));

    PluginResolveConfigurationResult {
        config: Configuration {
            line_width,
            indent_width,
            use_tabs,
            new_line_kind,
            command_case,
            keyword_case,
            closing_paren_newline,
            sort_lists,
            max_blank_lines,
            space_before_paren,
        },
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec!["cmake".into()],
            file_names: vec!["CMakeLists.txt".into()],
        },
    }
}

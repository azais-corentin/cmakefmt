use dprint_core::configuration::{ConfigKeyMap, ConfigKeyValue, GlobalConfiguration};

use super::types::{CaseStyle, Configuration, NewLineKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigDiagnosticSeverity {
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDiagnostic {
    pub key: String,
    pub message: String,
    pub severity: ConfigDiagnosticSeverity,
}

#[derive(Debug, Clone)]
pub struct ConfigLoadResult {
    pub config: Configuration,
    pub diagnostics: Vec<ConfigDiagnostic>,
    pub overrides: Vec<(String, String)>,
}

pub fn load_from_cli(config: Configuration) -> ConfigLoadResult {
    ConfigLoadResult {
        config,
        diagnostics: Vec::new(),
        overrides: Vec::new(),
    }
}

pub fn load_from_dprint(mut config: ConfigKeyMap, global_config: &GlobalConfiguration) -> ConfigLoadResult {
    let mut loader = Loader::new(base_from_global(global_config));
    for (key, value) in config.drain(..) {
        loader.apply_raw_value(&key, RawConfigValue::Dprint(value));
    }
    loader.finish()
}

pub fn load_from_header(header: &str) -> ConfigLoadResult {
    let mut loader = Loader::new(Configuration::default());

    if let Ok(map) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(header) {
        for (key, value) in map {
            loader.apply_raw_value(&key, RawConfigValue::Json(value));
        }
        return loader.finish();
    }

    for (key, value) in parse_gersemi_pairs(header) {
        loader.apply_raw_value(&key, RawConfigValue::Text(value));
    }

    loader.finish()
}

#[derive(Debug, Clone, Copy)]
enum CanonicalKey {
    LineWidth,
    IndentWidth,
    UseTabs,
    NewLineKind,
    CommandCase,
    KeywordCase,
    ClosingParenNewline,
    SortLists,
    MaxBlankLines,
    SpaceBeforeParen,
    IndentMode,
}

fn canonical_key(key: &str) -> Option<CanonicalKey> {
    match key {
        "lineWidth" | "line_width" => Some(CanonicalKey::LineWidth),
        "indentWidth" | "indent_width" => Some(CanonicalKey::IndentWidth),
        "useTabs" | "use_tabs" => Some(CanonicalKey::UseTabs),
        "newLineKind" | "new_line_kind" => Some(CanonicalKey::NewLineKind),
        "commandCase" | "command_case" => Some(CanonicalKey::CommandCase),
        "keywordCase" | "keyword_case" => Some(CanonicalKey::KeywordCase),
        "closingParenNewline" | "closing_paren_newline" => Some(CanonicalKey::ClosingParenNewline),
        "sortLists" | "sort_lists" => Some(CanonicalKey::SortLists),
        "maxBlankLines" | "max_blank_lines" => Some(CanonicalKey::MaxBlankLines),
        "spaceBeforeParen" | "space_before_paren" => Some(CanonicalKey::SpaceBeforeParen),
        "indent" => Some(CanonicalKey::IndentMode),
        _ => None,
    }
}

struct Loader {
    config: Configuration,
    diagnostics: Vec<ConfigDiagnostic>,
    overrides: Vec<(String, String)>,
}

macro_rules! set_field {
    ($loader:expr, $key:expr, $value:expr, $parse_fn:ident, $field:ident) => {
        $loader.$parse_fn($key, $value, |loader, parsed| {
            loader.config.$field = parsed;
            loader.overrides.push(($key.to_string(), parsed.to_string()));
        })
    };
}
impl Loader {
    fn new(config: Configuration) -> Self {
        Self {
            config,
            diagnostics: Vec::new(),
            overrides: Vec::new(),
        }
    }

    fn finish(mut self) -> ConfigLoadResult {
        self.overrides.sort_by(|left, right| left.0.cmp(&right.0));
        ConfigLoadResult {
            config: self.config,
            diagnostics: self.diagnostics,
            overrides: self.overrides,
        }
    }

    fn apply_raw_value(&mut self, key: &str, value: RawConfigValue) {
        let Some(canonical_key) = canonical_key(key) else {
            self.push_unknown_key(key);
            return;
        };

        match canonical_key {
            CanonicalKey::LineWidth => set_field!(self, key, value, parse_u32, line_width),
            CanonicalKey::IndentWidth => set_field!(self, key, value, parse_u8, indent_width),
            CanonicalKey::UseTabs => set_field!(self, key, value, parse_bool, use_tabs),
            CanonicalKey::NewLineKind => self.parse_new_line_kind(key, value),
            CanonicalKey::CommandCase => self.parse_case_style(key, value, true),
            CanonicalKey::KeywordCase => self.parse_case_style(key, value, false),
            CanonicalKey::ClosingParenNewline => set_field!(self, key, value, parse_bool, closing_paren_newline),
            CanonicalKey::SortLists => set_field!(self, key, value, parse_bool, sort_lists),
            CanonicalKey::MaxBlankLines => set_field!(self, key, value, parse_u8, max_blank_lines),
            CanonicalKey::SpaceBeforeParen => self.parse_space_before_paren(key, value),
            CanonicalKey::IndentMode => self.parse_indent_mode(key, value),
        }
    }

    fn parse_case_style(&mut self, key: &str, value: RawConfigValue, is_command_case: bool) {
        self.parse_string(key, value, |loader, parsed| {
            let Ok(style) = parsed.parse::<CaseStyle>() else {
                let field_name = if is_command_case { "commandCase" } else { "keywordCase" };
                loader.push_invalid_value(
                    key,
                    format!("Invalid {field_name} value '{parsed}'. Expected one of: lower, upper, preserve"),
                );
                return;
            };

            if is_command_case {
                loader.config.command_case = style;
            } else {
                loader.config.keyword_case = style;
            }
            loader.overrides.push((key.to_string(), format!("{style:?}")));
        });
    }

    fn parse_new_line_kind(&mut self, key: &str, value: RawConfigValue) {
        self.parse_string(key, value, |loader, parsed| {
            let Ok(kind) = parsed.parse::<NewLineKind>() else {
                loader.push_invalid_value(
                    key,
                    format!("Invalid newLineKind value '{parsed}'. Expected one of: auto, lf, crlf"),
                );
                return;
            };

            loader.config.new_line_kind = kind;
            loader.overrides.push((key.to_string(), format!("{kind:?}")));
        });
    }

    fn parse_indent_mode(&mut self, key: &str, value: RawConfigValue) {
        if let Some(mode) = value.as_lower_string() {
            if mode == "tabs" {
                self.config.use_tabs = true;
                self.overrides.push((key.to_string(), "tabs".to_string()));
                return;
            }
            if mode == "spaces" {
                self.config.use_tabs = false;
                self.overrides.push((key.to_string(), "spaces".to_string()));
                return;
            }
        }

        if let Some(width) = value.as_u8_number() {
            self.config.indent_width = width;
            self.config.use_tabs = false;
            self.overrides.push((key.to_string(), width.to_string()));
            return;
        }

        self.push_invalid_type(key, "string ('tabs'/'spaces') or integer for indent width");
    }

    fn parse_bool(&mut self, key: &str, value: RawConfigValue, apply: impl FnOnce(&mut Loader, bool)) {
        let Some(parsed) = value.as_bool() else {
            self.push_invalid_type(key, "boolean");
            return;
        };
        apply(self, parsed);
    }

    fn parse_u8(&mut self, key: &str, value: RawConfigValue, apply: impl FnOnce(&mut Loader, u8)) {
        let Some(parsed) = value.as_u8_number() else {
            self.push_invalid_type(key, "integer in 0..=255");
            return;
        };
        apply(self, parsed);
    }

    fn parse_u32(&mut self, key: &str, value: RawConfigValue, apply: impl FnOnce(&mut Loader, u32)) {
        let Some(parsed) = value.as_u32_number() else {
            self.push_invalid_type(key, "non-negative integer");
            return;
        };
        apply(self, parsed);
    }

    fn parse_string(&mut self, key: &str, value: RawConfigValue, apply: impl FnOnce(&mut Loader, String)) {
        let Some(parsed) = value.into_string() else {
            self.push_invalid_type(key, "string");
            return;
        };
        apply(self, parsed);
    }

    fn parse_space_before_paren(&mut self, key: &str, value: RawConfigValue) {
        let parsed = match value {
            RawConfigValue::Dprint(ConfigKeyValue::Array(items)) => {
                let mut names = Vec::new();
                for item in items {
                    if let ConfigKeyValue::String(s) = item {
                        names.push(s.to_ascii_lowercase());
                    } else {
                        self.push_invalid_type(key, "array of command name strings");
                        return;
                    }
                }
                names
            }
            RawConfigValue::Json(serde_json::Value::Array(items)) => {
                let mut names = Vec::new();
                for item in items {
                    if let serde_json::Value::String(s) = item {
                        names.push(s.to_ascii_lowercase());
                    } else {
                        self.push_invalid_type(key, "array of command name strings");
                        return;
                    }
                }
                names
            }
            RawConfigValue::Text(ref text) => {
                let trimmed = text.trim();
                if let Some(inner) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                    inner
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_ascii_lowercase())
                        .filter(|s| !s.is_empty())
                        .collect()
                } else {
                    self.push_invalid_type(key, "array of command names (e.g. [if, while])");
                    return;
                }
            }
            _ => {
                self.push_invalid_type(key, "array of command name strings");
                return;
            }
        };
        let display: String = format!("[{}]", parsed.join(", "));
        self.config.space_before_paren = parsed;
        self.overrides.push((key.to_string(), display));
    }

    fn push_unknown_key(&mut self, key: &str) {
        self.diagnostics.push(ConfigDiagnostic {
            key: key.to_string(),
            message: "Unknown property in configuration".to_string(),
            severity: ConfigDiagnosticSeverity::Warning,
        });
    }

    fn push_invalid_type(&mut self, key: &str, expected: &str) {
        self.diagnostics.push(ConfigDiagnostic {
            key: key.to_string(),
            message: format!("Invalid type for '{key}', expected {expected}"),
            severity: ConfigDiagnosticSeverity::Warning,
        });
    }

    fn push_invalid_value(&mut self, key: &str, message: String) {
        self.diagnostics.push(ConfigDiagnostic {
            key: key.to_string(),
            message,
            severity: ConfigDiagnosticSeverity::Warning,
        });
    }
}


fn base_from_global(global_config: &GlobalConfiguration) -> Configuration {
    let defaults = Configuration::default();
    Configuration {
        line_width: global_config.line_width.unwrap_or(defaults.line_width),
        indent_width: global_config.indent_width.unwrap_or(defaults.indent_width),
        use_tabs: global_config.use_tabs.unwrap_or(defaults.use_tabs),
        ..defaults
    }
}

enum RawConfigValue {
    Dprint(ConfigKeyValue),
    Json(serde_json::Value),
    Text(String),
}

impl RawConfigValue {
    fn into_string(self) -> Option<String> {
        match self {
            RawConfigValue::Dprint(ConfigKeyValue::String(value)) => Some(value),
            RawConfigValue::Json(serde_json::Value::String(value)) => Some(value),
            RawConfigValue::Text(value) => Some(trim_quotes(&value)),
            _ => None,
        }
    }

    fn as_lower_string(&self) -> Option<String> {
        self.clone().into_string().map(|text| text.to_ascii_lowercase())
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            RawConfigValue::Dprint(ConfigKeyValue::Bool(value)) => Some(*value),
            RawConfigValue::Json(serde_json::Value::Bool(value)) => Some(*value),
            RawConfigValue::Text(value) => parse_bool_text(value),
            _ => None,
        }
    }

    fn as_u8_number(&self) -> Option<u8> {
        let parsed = self.as_i64_number()?;
        u8::try_from(parsed).ok()
    }

    fn as_u32_number(&self) -> Option<u32> {
        let parsed = self.as_i64_number()?;
        u32::try_from(parsed).ok()
    }

    fn as_i64_number(&self) -> Option<i64> {
        match self {
            RawConfigValue::Dprint(ConfigKeyValue::Number(value)) => Some((*value).into()),
            RawConfigValue::Json(serde_json::Value::Number(value)) => value.as_i64(),
            RawConfigValue::Text(value) => trim_quotes(value).parse::<i64>().ok(),
            _ => None,
        }
    }
}

impl Clone for RawConfigValue {
    fn clone(&self) -> Self {
        match self {
            RawConfigValue::Dprint(value) => RawConfigValue::Dprint(value.clone()),
            RawConfigValue::Json(value) => RawConfigValue::Json(value.clone()),
            RawConfigValue::Text(value) => RawConfigValue::Text(value.clone()),
        }
    }
}

fn trim_quotes(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn parse_bool_text(value: &str) -> Option<bool> {
    match trim_quotes(value).to_ascii_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_gersemi_pairs(header: &str) -> Vec<(String, String)> {
    let content = header.trim();
    let content = content.strip_prefix('{').unwrap_or(content);
    let content = content.strip_suffix('}').unwrap_or(content);
    let content = content.trim();

    if content.is_empty() {
        return Vec::new();
    }

    split_respecting_brackets(content)
        .into_iter()
        .filter_map(|pair| {
            let pair = pair.trim();
            let (key, value) = pair.split_once(':')?;
            Some((key.trim().trim_matches('"').to_string(), value.trim().to_string()))
        })
        .collect()
}

fn split_respecting_brackets(s: &str) -> Vec<&str> {
    let mut results = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (index, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => {
                results.push(&s[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }

    if start < s.len() {
        results.push(&s[start..]);
    }

    results
}

#[cfg(test)]
mod tests {
    use dprint_core::configuration::ConfigKeyMap;

    use super::*;

    #[test]
    fn parses_canonical_keys_from_json_header() {
        let result = load_from_header(r#"{"lineWidth": 100, "commandCase": "upper", "sortLists": true}"#);

        assert_eq!(result.config.line_width, 100);
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert!(result.config.sort_lists);
        assert!(result.diagnostics.is_empty());
        assert_eq!(
            result.overrides,
            vec![
                ("commandCase".to_string(), "Upper".to_string()),
                ("lineWidth".to_string(), "100".to_string()),
                ("sortLists".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn parses_alias_keys_from_gersemi_header() {
        let result = load_from_header(
            "{indent: tabs, line_width: 120, new_line_kind: lf, keyword_case: preserve, sort_lists: true}",
        );

        assert!(result.config.use_tabs);
        assert_eq!(result.config.line_width, 120);
        assert_eq!(result.config.new_line_kind, NewLineKind::Lf);
        assert_eq!(result.config.keyword_case, CaseStyle::Preserve);
        assert!(result.config.sort_lists);
        assert!(result.diagnostics.is_empty());
        assert_eq!(
            result.overrides,
            vec![
                ("indent".to_string(), "tabs".to_string()),
                ("keyword_case".to_string(), "Preserve".to_string()),
                ("line_width".to_string(), "120".to_string()),
                ("new_line_kind".to_string(), "Lf".to_string()),
                ("sort_lists".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn emits_diagnostics_for_removed_alias_keys() {
        let result = load_from_header("{line_length: 120, newline: lf}");

        assert_eq!(result.config.line_width, Configuration::default().line_width);
        assert_eq!(
            result.config.new_line_kind,
            Configuration::default().new_line_kind
        );
        assert!(result.overrides.is_empty());
        assert_eq!(result.diagnostics.len(), 2);
        assert_eq!(result.diagnostics[0].key, "line_length");
        assert_eq!(result.diagnostics[1].key, "newline");
        assert_eq!(
            result.diagnostics[0].message,
            "Unknown property in configuration"
        );
        assert_eq!(
            result.diagnostics[1].message,
            "Unknown property in configuration"
        );
    }

    #[test]
    fn emits_diagnostics_for_invalid_values() {
        let result = load_from_header(r#"{"commandCase": "weird", "sortLists": "nope"}"#);

        assert_eq!(result.config.command_case, Configuration::default().command_case);
        assert_eq!(result.config.sort_lists, Configuration::default().sort_lists);
        assert_eq!(result.diagnostics.len(), 2);
        assert_eq!(result.diagnostics[0].key, "commandCase");
        assert!(result.diagnostics[0].message.contains("Invalid commandCase value"));
        assert_eq!(result.diagnostics[1].key, "sortLists");
        assert!(result.diagnostics[1].message.contains("expected boolean"));
    }

    #[test]
    fn emits_diagnostics_for_unknown_keys() {
        let result = load_from_header(r#"{"unknownKey": 1}"#);
        assert_eq!(result.config.line_width, Configuration::default().line_width);
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].key, "unknownKey");
        assert_eq!(
            result.diagnostics[0].message,
            "Unknown property in configuration"
        );
    }

    #[test]
    fn applies_global_defaults_then_source_patch() {
        let mut map = ConfigKeyMap::new();
        map.insert("lineWidth".to_string(), ConfigKeyValue::Number(90));
        map.insert("sortLists".to_string(), ConfigKeyValue::Bool(true));

        let global = GlobalConfiguration {
            line_width: Some(120),
            indent_width: Some(6),
            use_tabs: Some(true),
            new_line_kind: None,
        };

        let result = load_from_dprint(map, &global);
        assert_eq!(result.config.line_width, 90);
        assert_eq!(result.config.indent_width, 6);
        assert!(result.config.use_tabs);
        assert!(result.config.sort_lists);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_space_before_paren_json_array() {
        let result = load_from_header(r#"{"spaceBeforeParen": ["if", "while", "foreach"]}"#);
        assert_eq!(result.config.space_before_paren, vec!["if", "while", "foreach"]);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_space_before_paren_json_array_lowercased() {
        let result = load_from_header(r#"{"spaceBeforeParen": ["IF", "While"]}"#);
        assert_eq!(result.config.space_before_paren, vec!["if", "while"]);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_space_before_paren_json_empty_array() {
        let result = load_from_header(r#"{"spaceBeforeParen": []}"#);
        assert!(result.config.space_before_paren.is_empty());
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_space_before_paren_gersemi_bracket_list() {
        let result = load_from_header("space_before_paren: [if, while]");
        assert_eq!(result.config.space_before_paren, vec!["if", "while"]);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn space_before_paren_invalid_value_emits_diagnostic() {
        let result = load_from_header(r#"{"spaceBeforeParen": 42}"#);
        assert!(result.config.space_before_paren.is_empty());
        assert_eq!(result.diagnostics.len(), 1);
        assert!(result.diagnostics[0].message.contains("expected array"));
    }

    #[test]
    fn space_before_paren_dprint_array() {
        let mut map = ConfigKeyMap::new();
        map.insert(
            "spaceBeforeParen".to_string(),
            ConfigKeyValue::Array(vec![
                ConfigKeyValue::String("set".to_string()),
                ConfigKeyValue::String("IF".to_string()),
            ]),
        );
        let global = GlobalConfiguration {
            line_width: None,
            indent_width: None,
            use_tabs: None,
            new_line_kind: None,
        };
        let result = load_from_dprint(map, &global);
        assert_eq!(result.config.space_before_paren, vec!["set", "if"]);
        assert!(result.diagnostics.is_empty());
    }
}

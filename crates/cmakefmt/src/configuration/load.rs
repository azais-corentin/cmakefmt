use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use super::types::{
    CaseStyle, CommandConfiguration, CommentPreservation, Configuration, EndCommandArgs,
    IndentStyle, NewLineKind, SortArguments, SpaceBeforeParen, SpaceInsideParen, WrapStyle,
};

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

/// Wraps a CLI-resolved configuration with empty diagnostics/override metadata.
pub fn load_from_cli(config: Configuration) -> ConfigLoadResult {
    ConfigLoadResult {
        config,
        diagnostics: Vec::new(),
        overrides: Vec::new(),
    }
}

/// Loads configuration from a JSON key/value map with an optional base configuration.
///
/// This is the bridge point for external integrations (e.g. dprint). Callers convert their
/// native config representation to `serde_json::Map` and pass it here. The `base` parameter
/// lets callers pre-apply global defaults (line_width, use_tabs, etc.) before plugin-specific
/// keys are layered on top.
pub fn load_from_json_map(
    map: serde_json::Map<String, serde_json::Value>,
    base: Configuration,
) -> ConfigLoadResult {
    let mut loader = Loader::new(base);
    for (key, value) in map {
        loader.apply_raw_value(&key, RawConfigValue::Json(value));
    }
    loader.finish()
}

const MAX_EXTENDS_DEPTH: usize = 32;
const DOTFILE_CONFIG_NAME: &str = ".cmakefmt.toml";
const PLAIN_CONFIG_NAME: &str = "cmakefmt.toml";

/// Parses a `.cmakefmt.toml` document into formatter configuration plus diagnostics.
pub fn load_from_toml(toml: &str) -> ConfigLoadResult {
    load_from_toml_with_base(toml, Configuration::default(), "<toml>")
}

/// Reads and parses configuration from disk into formatter configuration plus diagnostics.
///
/// When `path` points to a CMake source file instead of a config file, this performs config
/// discovery by walking parent directories and selecting the first `.cmakefmt.toml`/`cmakefmt.toml`
/// found (`.cmakefmt.toml` takes precedence when both are present).
pub fn load_from_toml_path(path: &Path) -> ConfigLoadResult {
    let config_path = if is_config_file_name(path) {
        Some(path.to_path_buf())
    } else {
        discover_toml_path(path)
    };

    match config_path {
        Some(config_path) => load_from_toml_path_with_context(&config_path, &mut Vec::new(), 0),
        None => ConfigLoadResult {
            config: Configuration::default(),
            diagnostics: Vec::new(),
            overrides: Vec::new(),
        },
    }
}

fn load_from_toml_path_with_context(
    path: &Path,
    resolution_stack: &mut Vec<PathBuf>,
    depth: usize,
) -> ConfigLoadResult {
    if depth >= MAX_EXTENDS_DEPTH {
        return ConfigLoadResult {
            config: Configuration::default(),
            diagnostics: vec![ConfigDiagnostic {
                key: path.display().to_string(),
                message: format!(
                    "Exceeded maximum extends depth ({MAX_EXTENDS_DEPTH}) while resolving configuration"
                ),
                severity: ConfigDiagnosticSeverity::Warning,
            }],
            overrides: Vec::new(),
        };
    }

    let canonical_path = canonicalize_for_resolution(path);
    if let Some(cycle_start) = resolution_stack
        .iter()
        .position(|entry| entry == &canonical_path)
    {
        let chain = resolution_stack[cycle_start..]
            .iter()
            .map(|entry| entry.display().to_string())
            .chain(std::iter::once(canonical_path.display().to_string()))
            .collect::<Vec<_>>()
            .join(" -> ");

        return ConfigLoadResult {
            config: Configuration::default(),
            diagnostics: vec![ConfigDiagnostic {
                key: path.display().to_string(),
                message: format!("Circular extends reference detected: {chain}"),
                severity: ConfigDiagnosticSeverity::Warning,
            }],
            overrides: Vec::new(),
        };
    }

    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => {
            return ConfigLoadResult {
                config: Configuration::default(),
                diagnostics: vec![ConfigDiagnostic {
                    key: path.display().to_string(),
                    message: format!("Failed to read TOML document: {error}"),
                    severity: ConfigDiagnosticSeverity::Warning,
                }],
                overrides: Vec::new(),
            };
        }
    };

    let mut parsed = match toml::from_str::<toml::Table>(&contents) {
        Ok(table) => table,
        Err(error) => {
            return ConfigLoadResult {
                config: Configuration::default(),
                diagnostics: vec![ConfigDiagnostic {
                    key: path.display().to_string(),
                    message: format!("Failed to parse TOML document: {error}"),
                    severity: ConfigDiagnosticSeverity::Warning,
                }],
                overrides: Vec::new(),
            };
        }
    };
    resolve_ignore_patterns_for_toml_path(&mut parsed, path);
    resolution_stack.push(canonical_path);
    let base_result = match resolve_extends_path(path, &parsed) {
        Some(base_path) => {
            load_from_toml_path_with_context(&base_path, resolution_stack, depth + 1)
        }
        None => ConfigLoadResult {
            config: Configuration::default(),
            diagnostics: Vec::new(),
            overrides: Vec::new(),
        },
    };
    resolution_stack.pop();

    let mut loader = Loader::new(base_result.config);
    apply_toml_table(&mut loader, parsed);

    let mut result = loader.finish();
    let mut diagnostics = base_result.diagnostics;
    diagnostics.append(&mut result.diagnostics);
    result.diagnostics = diagnostics;

    let mut overrides = base_result.overrides;
    overrides.append(&mut result.overrides);
    overrides.sort_by(|left, right| left.0.cmp(&right.0));
    result.overrides = overrides;

    result
}

fn load_from_toml_with_base(
    toml: &str,
    base: Configuration,
    diagnostic_key: &str,
) -> ConfigLoadResult {
    let mut loader = Loader::new(base);

    let parsed = match toml::from_str::<toml::Table>(toml) {
        Ok(table) => table,
        Err(error) => {
            loader.diagnostics.push(ConfigDiagnostic {
                key: diagnostic_key.to_string(),
                message: format!("Failed to parse TOML document: {error}"),
                severity: ConfigDiagnosticSeverity::Warning,
            });
            return loader.finish();
        }
    };

    apply_toml_table(&mut loader, parsed);
    loader.finish()
}

fn apply_toml_table(loader: &mut Loader, parsed: toml::Table) {
    for (key, value) in parsed {
        match serde_json::to_value(value) {
            Ok(value) => loader.apply_raw_value(&key, RawConfigValue::Json(value)),
            Err(error) => loader.diagnostics.push(ConfigDiagnostic {
                key,
                message: format!("Failed to convert TOML value to JSON: {error}"),
                severity: ConfigDiagnosticSeverity::Warning,
            }),
        }
    }
}

fn resolve_ignore_patterns_for_toml_path(parsed: &mut toml::Table, config_path: &Path) {
    let raw_patterns = if let Some(patterns) = parsed.get_mut("ignorePatterns") {
        patterns
    } else if let Some(patterns) = parsed.get_mut("ignore_patterns") {
        patterns
    } else {
        return;
    };

    let Some(patterns) = raw_patterns.as_array_mut() else {
        return;
    };

    let config_dir = canonicalize_for_resolution(config_path)
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    for pattern in patterns {
        let Some(raw_pattern) = pattern.as_str() else {
            continue;
        };

        if Path::new(raw_pattern).is_absolute() {
            continue;
        }

        let resolved_pattern = config_dir.join(raw_pattern);
        let normalized = resolved_pattern.to_string_lossy().replace('\\', "/");
        *pattern = toml::Value::String(normalized);
    }
}

fn resolve_extends_path(config_path: &Path, parsed: &toml::Table) -> Option<PathBuf> {
    let raw_extends = parsed.get("extends")?.as_str()?;
    let extends_path = PathBuf::from(raw_extends);
    if extends_path.is_absolute() {
        Some(extends_path)
    } else {
        let config_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
        Some(config_dir.join(extends_path))
    }
}

fn canonicalize_for_resolution(path: &Path) -> PathBuf {
    match std::fs::canonicalize(path) {
        Ok(path) => path,
        Err(_) => {
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                match std::env::current_dir() {
                    Ok(current_dir) => current_dir.join(path),
                    Err(_) => path.to_path_buf(),
                }
            }
        }
    }
}

fn is_config_file_name(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(DOTFILE_CONFIG_NAME) | Some(PLAIN_CONFIG_NAME)
    )
}

fn discover_toml_path(start_path: &Path) -> Option<PathBuf> {
    let mut current_dir = if start_path.is_dir() {
        start_path.to_path_buf()
    } else {
        match start_path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
            _ => PathBuf::from("."),
        }
    };

    loop {
        let dotfile_path = current_dir.join(DOTFILE_CONFIG_NAME);
        if dotfile_path.is_file() {
            return Some(dotfile_path);
        }

        let plain_path = current_dir.join(PLAIN_CONFIG_NAME);
        if plain_path.is_file() {
            return Some(plain_path);
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

/// Applies inline push-directive overrides on top of an existing configuration.
/// The `header` string is the content between braces in a `# cmakefmt: push { ... }` comment.
/// Supports JSON (`{"key": value}`) and TOML inline-table syntax (`{ key = value }`).
pub fn apply_inline_overrides(base: &Configuration, header: &str) -> Configuration {
    let mut loader = Loader::new(base.clone());
    let mut explicit_keys = HashSet::new();

    if let Ok(map) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(header) {
        apply_inline_override_entries(&mut loader, &mut explicit_keys, map);
        return finish_inline_overrides(loader, &explicit_keys);
    }

    if let Some(map) = parse_relaxed_inline_toml_map(header) {
        apply_inline_override_entries(&mut loader, &mut explicit_keys, map);
        return finish_inline_overrides(loader, &explicit_keys);
    }

    for (key, value) in parse_gersemi_pairs(header) {
        if let Some(canonical) = canonical_key(&key) {
            explicit_keys.insert(canonical);
        }

        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&value) {
            loader.apply_raw_value(&key, RawConfigValue::Json(json_val));
        } else {
            loader.apply_raw_value(&key, RawConfigValue::Text(value));
        }
    }

    finish_inline_overrides(loader, &explicit_keys)
}

fn apply_inline_override_entries(
    loader: &mut Loader,
    explicit_keys: &mut HashSet<CanonicalKey>,
    entries: serde_json::Map<String, serde_json::Value>,
) {
    for (key, value) in entries {
        if let Some(canonical) = canonical_key(&key) {
            explicit_keys.insert(canonical);
        }
        loader.apply_raw_value(&key, RawConfigValue::Json(value));
    }
}

fn finish_inline_overrides(loader: Loader, explicit_keys: &HashSet<CanonicalKey>) -> Configuration {
    let mut config = loader.finish().config;
    suppress_per_command_overrides_for_push_keys(&mut config, explicit_keys);
    config
}

fn parse_relaxed_inline_toml_map(
    header: &str,
) -> Option<serde_json::Map<String, serde_json::Value>> {
    let normalized = normalize_inline_table_literal(header)?;
    let relaxed = strip_relaxed_trailing_commas(&normalized);
    let toml_doc = format!("__cmakefmt_inline = {relaxed}");
    let parsed = toml::from_str::<toml::Table>(&toml_doc).ok()?;
    let inline_value = parsed.get("__cmakefmt_inline")?.clone();
    let json_value = serde_json::to_value(inline_value).ok()?;

    match json_value {
        serde_json::Value::Object(map) => Some(map),
        _ => None,
    }
}

fn normalize_inline_table_literal(header: &str) -> Option<String> {
    let trimmed = header.trim();
    if trimmed.is_empty() {
        return Some("{}".to_string());
    }

    if trimmed.starts_with('{') {
        if trimmed.ends_with('}') {
            return Some(trimmed.to_string());
        }
        return None;
    }

    Some(format!("{{ {trimmed} }}"))
}

fn strip_relaxed_trailing_commas(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut escaping = false;

    while let Some(ch) = chars.next() {
        if in_string {
            output.push(ch);
            if escaping {
                escaping = false;
                continue;
            }
            if ch == '\\' {
                escaping = true;
                continue;
            }
            if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            output.push(ch);
            continue;
        }

        if ch == ',' {
            let lookahead = chars.clone();
            let mut trailing = true;
            for next in lookahead {
                if next.is_whitespace() {
                    continue;
                }
                trailing = next == '}' || next == ']';
                break;
            }
            if trailing {
                continue;
            }
        }

        output.push(ch);
    }

    output
}

fn suppress_per_command_overrides_for_push_keys(
    config: &mut Configuration,
    explicit_keys: &HashSet<CanonicalKey>,
) {
    if explicit_keys.is_empty() || config.per_command_config.is_empty() {
        return;
    }

    for command_config in config.per_command_config.values_mut() {
        if explicit_keys.contains(&CanonicalKey::LineWidth) {
            command_config.line_width = None;
        }
        if explicit_keys.contains(&CanonicalKey::WrapStyle) {
            command_config.wrap_style = None;
        }
        if explicit_keys.contains(&CanonicalKey::FirstArgSameLine) {
            command_config.first_arg_same_line = None;
        }
        if explicit_keys.contains(&CanonicalKey::WrapArgThreshold) {
            command_config.wrap_arg_threshold = None;
        }

        if explicit_keys.contains(&CanonicalKey::IndentWidth)
            || explicit_keys.contains(&CanonicalKey::IndentMode)
        {
            command_config.indent_width = None;
        }
        if explicit_keys.contains(&CanonicalKey::IndentStyle)
            || explicit_keys.contains(&CanonicalKey::IndentMode)
        {
            command_config.indent_style = None;
        }
        if explicit_keys.contains(&CanonicalKey::ContinuationIndentWidth) {
            command_config.continuation_indent_width = None;
        }

        if explicit_keys.contains(&CanonicalKey::CommandCase) {
            command_config.command_case = None;
        }
        if explicit_keys.contains(&CanonicalKey::KeywordCase) {
            command_config.keyword_case = None;
        }
        if explicit_keys.contains(&CanonicalKey::CustomKeywords) {
            command_config.custom_keywords = None;
        }
        if explicit_keys.contains(&CanonicalKey::LiteralCase) {
            command_config.literal_case = None;
        }

        if explicit_keys.contains(&CanonicalKey::ClosingParenNewline) {
            command_config.closing_paren_newline = None;
        }
        if explicit_keys.contains(&CanonicalKey::SpaceBeforeParen) {
            command_config.space_before_paren = None;
        }
        if explicit_keys.contains(&CanonicalKey::SpaceInsideParen) {
            command_config.space_inside_paren = None;
        }

        if explicit_keys.contains(&CanonicalKey::CommentPreservation) {
            command_config.comment_preservation = None;
        }
        if explicit_keys.contains(&CanonicalKey::CommentWidth) {
            command_config.comment_width = None;
        }
        if explicit_keys.contains(&CanonicalKey::AlignTrailingComments) {
            command_config.align_trailing_comments = None;
        }
        if explicit_keys.contains(&CanonicalKey::CommentGap) {
            command_config.comment_gap = None;
        }

        if explicit_keys.contains(&CanonicalKey::AlignPropertyValues) {
            command_config.align_property_values = None;
        }
        if explicit_keys.contains(&CanonicalKey::AlignConsecutiveSet) {
            command_config.align_consecutive_set = None;
        }
        if explicit_keys.contains(&CanonicalKey::AlignArgGroups) {
            command_config.align_arg_groups = None;
        }

        if explicit_keys.contains(&CanonicalKey::SortArguments)
            || explicit_keys.contains(&CanonicalKey::SortKeywordSections)
        {
            command_config.sort_arguments = None;
        }
        if explicit_keys.contains(&CanonicalKey::SortKeywordSections) {
            command_config.sort_keyword_sections = None;
        }
    }
}

/// Parses a single-line fixture/header override payload (JSON or gersemi style).
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CanonicalKey {
    LineWidth,
    WrapStyle,
    FirstArgSameLine,
    WrapArgThreshold,
    IndentWidth,
    UseTabs,
    IndentStyle,
    ContinuationIndentWidth,
    NewLineKind,
    FinalNewline,
    MaxBlankLines,
    MinBlankLinesBetweenBlocks,
    BlankLineBetweenSections,
    CommandCase,
    KeywordCase,
    CustomKeywords,
    LiteralCase,
    ClosingParenNewline,
    SpaceBeforeParen,
    SpaceInsideParen,
    CommentPreservation,
    CommentWidth,
    AlignTrailingComments,
    CommentGap,
    TrimTrailingWhitespace,
    CollapseSpaces,
    AlignPropertyValues,
    AlignConsecutiveSet,
    AlignArgGroups,
    PerCommandConfig,
    SortArguments,
    SortKeywordSections,
    DisableFormatting,
    IgnorePatterns,
    IgnoreCommands,
    IndentBlockBody,
    EndCommandArgs,
    Extends,
    IndentMode,
}

fn canonical_key(key: &str) -> Option<CanonicalKey> {
    match key {
        "lineWidth" | "line_width" => Some(CanonicalKey::LineWidth),
        "wrapStyle" | "wrap_style" => Some(CanonicalKey::WrapStyle),
        "firstArgSameLine" | "first_arg_same_line" => Some(CanonicalKey::FirstArgSameLine),
        "wrapArgThreshold" | "wrap_arg_threshold" => Some(CanonicalKey::WrapArgThreshold),
        "indentWidth" | "indent_width" => Some(CanonicalKey::IndentWidth),
        "useTabs" | "use_tabs" => Some(CanonicalKey::UseTabs),
        "indentStyle" | "indent_style" => Some(CanonicalKey::IndentStyle),
        "continuationIndentWidth" | "continuation_indent_width" => {
            Some(CanonicalKey::ContinuationIndentWidth)
        }
        "lineEnding" | "line_ending" | "newLineKind" | "new_line_kind" => {
            Some(CanonicalKey::NewLineKind)
        }
        "finalNewline" | "final_newline" => Some(CanonicalKey::FinalNewline),
        "maxBlankLines" | "max_blank_lines" => Some(CanonicalKey::MaxBlankLines),
        "minBlankLinesBetweenBlocks" | "min_blank_lines_between_blocks" => {
            Some(CanonicalKey::MinBlankLinesBetweenBlocks)
        }
        "blankLineBetweenSections" | "blank_line_between_sections" => {
            Some(CanonicalKey::BlankLineBetweenSections)
        }
        "commandCase" | "command_case" => Some(CanonicalKey::CommandCase),
        "keywordCase" | "keyword_case" => Some(CanonicalKey::KeywordCase),
        "customKeywords" | "custom_keywords" => Some(CanonicalKey::CustomKeywords),
        "literalCase" | "literal_case" => Some(CanonicalKey::LiteralCase),
        "closingParenNewline" | "closing_paren_newline" => Some(CanonicalKey::ClosingParenNewline),
        "spaceBeforeParen" | "space_before_paren" => Some(CanonicalKey::SpaceBeforeParen),
        "spaceInsideParen" | "space_inside_paren" => Some(CanonicalKey::SpaceInsideParen),
        "commentPreservation" | "comment_preservation" => Some(CanonicalKey::CommentPreservation),
        "commentWidth" | "comment_width" => Some(CanonicalKey::CommentWidth),
        "alignTrailingComments" | "align_trailing_comments" => {
            Some(CanonicalKey::AlignTrailingComments)
        }
        "commentGap" | "comment_gap" => Some(CanonicalKey::CommentGap),
        "trimTrailingWhitespace" | "trim_trailing_whitespace" => {
            Some(CanonicalKey::TrimTrailingWhitespace)
        }
        "collapseSpaces" | "collapse_spaces" => Some(CanonicalKey::CollapseSpaces),
        "alignPropertyValues" | "align_property_values" => Some(CanonicalKey::AlignPropertyValues),
        "alignConsecutiveSet" | "align_consecutive_set" => Some(CanonicalKey::AlignConsecutiveSet),
        "alignArgGroups" | "align_arg_groups" => Some(CanonicalKey::AlignArgGroups),
        "perCommandConfig" | "per_command_config" => Some(CanonicalKey::PerCommandConfig),
        "sortArguments" | "sort_arguments" | "sortLists" | "sort_lists" => {
            Some(CanonicalKey::SortArguments)
        }
        "sortKeywordSections" | "sort_keyword_sections" => Some(CanonicalKey::SortKeywordSections),
        "disableFormatting" | "disable_formatting" => Some(CanonicalKey::DisableFormatting),
        "ignorePatterns" | "ignore_patterns" => Some(CanonicalKey::IgnorePatterns),
        "ignoreCommands" | "ignore_commands" => Some(CanonicalKey::IgnoreCommands),
        "indentBlockBody" | "indent_block_body" => Some(CanonicalKey::IndentBlockBody),
        "endCommandArgs" | "end_command_args" => Some(CanonicalKey::EndCommandArgs),
        "extends" => Some(CanonicalKey::Extends),
        "indent" => Some(CanonicalKey::IndentMode),
        _ => None,
    }
}

struct Loader {
    config: Configuration,
    diagnostics: Vec<ConfigDiagnostic>,
    overrides: Vec<(String, String)>,
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
            CanonicalKey::LineWidth => {
                if let Some(parsed) = self.parse_u32(key, value) {
                    self.config.line_width = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::WrapStyle => {
                if let Some(parsed) = self.parse_wrap_style(key, value) {
                    self.config.wrap_style = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::FirstArgSameLine => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.first_arg_same_line = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::WrapArgThreshold => {
                if let Some(parsed) = self.parse_u32(key, value) {
                    self.config.wrap_arg_threshold = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::IndentWidth => {
                if let Some(parsed) = self.parse_u8(key, value) {
                    self.config.indent_width = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::UseTabs => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.use_tabs = parsed;
                    self.config.indent_style = if parsed {
                        IndentStyle::Tab
                    } else {
                        IndentStyle::Space
                    };
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::IndentStyle => {
                if let Some(parsed) = self.parse_indent_style(key, value) {
                    self.config.indent_style = parsed;
                    self.config.use_tabs = matches!(parsed, IndentStyle::Tab);
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::ContinuationIndentWidth => {
                if let Some(parsed) = self.parse_nullable_u8(key, value) {
                    self.config.continuation_indent_width = parsed;
                    self.record_override_nullable_u8(key, parsed);
                }
            }
            CanonicalKey::NewLineKind => {
                if let Some(parsed) = self.parse_new_line_kind(key, value) {
                    self.config.new_line_kind = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::FinalNewline => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.final_newline = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::MaxBlankLines => {
                if let Some(parsed) = self.parse_u8(key, value) {
                    self.config.max_blank_lines = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::MinBlankLinesBetweenBlocks => {
                if let Some(parsed) = self.parse_u8(key, value) {
                    self.config.min_blank_lines_between_blocks = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::BlankLineBetweenSections => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.blank_line_between_sections = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::CommandCase => {
                if let Some(parsed) = self.parse_case_style(key, value, "commandCase") {
                    self.config.command_case = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::KeywordCase => {
                if let Some(parsed) = self.parse_case_style(key, value, "keywordCase") {
                    self.config.keyword_case = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::CustomKeywords => {
                if let Some(parsed) =
                    self.parse_string_array(key, value, false, "array of keyword strings")
                {
                    self.config.custom_keywords = parsed.clone();
                    self.record_override(key, format!("[{}]", parsed.join(", ")));
                }
            }
            CanonicalKey::LiteralCase => {
                if let Some(parsed) = self.parse_case_style(key, value, "literalCase") {
                    self.config.literal_case = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::ClosingParenNewline => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.closing_paren_newline = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::SpaceBeforeParen => {
                if let Some(parsed) = self.parse_space_before_paren(key, value) {
                    self.config.space_before_paren = parsed;
                    self.record_override(
                        key,
                        match &self.config.space_before_paren {
                            SpaceBeforeParen::Never => "false".to_string(),
                            SpaceBeforeParen::Always => "true".to_string(),
                            SpaceBeforeParen::CommandList(command_names) => {
                                format!("[{}]", command_names.join(", "))
                            }
                        },
                    );
                }
            }
            CanonicalKey::SpaceInsideParen => {
                if let Some(parsed) = self.parse_space_inside_paren(key, value) {
                    self.config.space_inside_paren = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::CommentPreservation => {
                if let Some(parsed) = self.parse_comment_preservation(key, value) {
                    self.config.comment_preservation = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::CommentWidth => {
                if let Some(parsed) = self.parse_nullable_u32(key, value) {
                    self.config.comment_width = parsed;
                    self.record_override_nullable_u32(key, parsed);
                }
            }
            CanonicalKey::AlignTrailingComments => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.align_trailing_comments = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::CommentGap => {
                if let Some(parsed) = self.parse_u8(key, value) {
                    self.config.comment_gap = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::TrimTrailingWhitespace => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.trim_trailing_whitespace = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::CollapseSpaces => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.collapse_spaces = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::AlignPropertyValues => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.align_property_values = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::AlignConsecutiveSet => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.align_consecutive_set = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::AlignArgGroups => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.align_arg_groups = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::PerCommandConfig => self.parse_per_command_config(key, value),
            CanonicalKey::SortArguments => {
                if let Some(parsed) = self.parse_sort_arguments(key, value) {
                    self.config.sort_lists = matches!(parsed, SortArguments::Enabled);
                    self.config.sort_arguments = parsed;
                    self.config.sort_arguments_explicit = true;
                    self.record_override(
                        key,
                        match &self.config.sort_arguments {
                            SortArguments::Disabled => "false".to_string(),
                            SortArguments::Enabled => "true".to_string(),
                            SortArguments::CommandList(names) => {
                                format!("[{}]", names.join(", "))
                            }
                        },
                    );
                }
            }
            CanonicalKey::SortKeywordSections => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.sort_keyword_sections = parsed;
                    self.config.sort_keyword_sections_explicit = true;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::DisableFormatting => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.disable_formatting = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::IgnorePatterns => {
                if let Some(parsed) =
                    self.parse_string_array(key, value, false, "array of glob pattern strings")
                {
                    self.config.ignore_patterns = parsed.clone();
                    self.record_override(key, format!("[{}]", parsed.join(", ")));
                }
            }
            CanonicalKey::IgnoreCommands => {
                if let Some(parsed) =
                    self.parse_string_array(key, value, false, "array of command name strings")
                {
                    self.config.ignore_commands = parsed.clone();
                    self.record_override(key, format!("[{}]", parsed.join(", ")));
                }
            }
            CanonicalKey::IndentBlockBody => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.indent_block_body = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::EndCommandArgs => {
                if let Some(parsed) = self.parse_end_command_args(key, value) {
                    self.config.end_command_args = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::Extends => {
                if let Some(parsed) = self.parse_string(key, value) {
                    self.config.extends = Some(parsed.clone());
                    self.record_override(key, parsed);
                }
            }
            CanonicalKey::IndentMode => self.parse_indent_mode(key, value),
        }
    }

    fn parse_case_style(
        &mut self,
        key: &str,
        value: RawConfigValue,
        field_name: &str,
    ) -> Option<CaseStyle> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<CaseStyle>() else {
            self.push_invalid_value(
                key,
                format!(
                    "Invalid {field_name} value '{parsed}'. Expected one of: lower, upper, unchanged"
                ),
            );
            return None;
        };

        Some(style)
    }

    fn parse_new_line_kind(&mut self, key: &str, value: RawConfigValue) -> Option<NewLineKind> {
        let parsed = self.parse_string(key, value)?;
        let Ok(kind) = parsed.parse::<NewLineKind>() else {
            self.push_invalid_value(
                key,
                format!(
                    "Invalid lineEnding/newLineKind value '{parsed}'. Expected one of: auto, lf, crlf"
                ),
            );
            return None;
        };
        Some(kind)
    }

    fn parse_wrap_style(&mut self, key: &str, value: RawConfigValue) -> Option<WrapStyle> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<WrapStyle>() else {
            self.push_invalid_value(
                key,
                format!("Invalid wrapStyle value '{parsed}'. Expected one of: cascade, vertical"),
            );
            return None;
        };

        Some(style)
    }

    fn parse_indent_style(&mut self, key: &str, value: RawConfigValue) -> Option<IndentStyle> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<IndentStyle>() else {
            self.push_invalid_value(
                key,
                format!("Invalid indentStyle value '{parsed}'. Expected one of: space, tab"),
            );
            return None;
        };

        Some(style)
    }

    fn parse_space_inside_paren(
        &mut self,
        key: &str,
        value: RawConfigValue,
    ) -> Option<SpaceInsideParen> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<SpaceInsideParen>() else {
            self.push_invalid_value(
                key,
                format!(
                    "Invalid spaceInsideParen value '{parsed}'. Expected one of: insert, remove, preserve"
                ),
            );
            return None;
        };

        Some(style)
    }

    fn parse_comment_preservation(
        &mut self,
        key: &str,
        value: RawConfigValue,
    ) -> Option<CommentPreservation> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<CommentPreservation>() else {
            self.push_invalid_value(
                key,
                format!(
                    "Invalid commentPreservation value '{parsed}'. Expected one of: preserve, reflow"
                ),
            );
            return None;
        };

        Some(style)
    }

    fn parse_end_command_args(
        &mut self,
        key: &str,
        value: RawConfigValue,
    ) -> Option<EndCommandArgs> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<EndCommandArgs>() else {
            self.push_invalid_value(
                key,
                format!(
                    "Invalid endCommandArgs value '{parsed}'. Expected one of: remove, preserve, match"
                ),
            );
            return None;
        };

        Some(style)
    }

    fn parse_space_before_paren(
        &mut self,
        key: &str,
        value: RawConfigValue,
    ) -> Option<SpaceBeforeParen> {
        if let Some(parsed) = value.as_bool() {
            return Some(if parsed {
                SpaceBeforeParen::Always
            } else {
                SpaceBeforeParen::Never
            });
        }

        let parsed =
            self.parse_string_array(key, value, true, "boolean or array of command name strings")?;
        Some(SpaceBeforeParen::CommandList(parsed))
    }

    fn parse_sort_arguments(&mut self, key: &str, value: RawConfigValue) -> Option<SortArguments> {
        if let Some(parsed) = value.as_bool() {
            return Some(if parsed {
                SortArguments::Enabled
            } else {
                SortArguments::Disabled
            });
        }

        let parsed = self.parse_string_array(
            key,
            value,
            true,
            "boolean or array of command/section name strings",
        )?;
        Some(SortArguments::CommandList(parsed))
    }

    fn parse_per_command_config(&mut self, key: &str, value: RawConfigValue) {
        let Some(entries) = value.into_object_entries() else {
            self.push_invalid_type(key, "table/object");
            return;
        };

        let mut parsed = BTreeMap::new();
        for (command_name, command_value) in entries {
            let scoped_key = format!("{key}.{command_name}");
            let Some(option_entries) = command_value.into_object_entries() else {
                self.push_invalid_type(&scoped_key, "table/object");
                continue;
            };

            let mut command_config = CommandConfiguration::default();
            for (option_key, option_value) in option_entries {
                self.apply_per_command_value(
                    key,
                    &command_name,
                    &option_key,
                    option_value,
                    &mut command_config,
                );
            }

            parsed.insert(command_name.to_ascii_lowercase(), command_config);
        }

        for (command_name, command_config) in parsed {
            self.config
                .per_command_config
                .insert(command_name, command_config);
        }
        self.record_override(
            key,
            format!(
                "{} command override(s)",
                self.config.per_command_config.len()
            ),
        );
    }

    fn apply_per_command_value(
        &mut self,
        parent_key: &str,
        command_name: &str,
        option_key: &str,
        option_value: RawConfigValue,
        target: &mut CommandConfiguration,
    ) {
        let scoped_key = format!("{parent_key}.{command_name}.{option_key}");
        let Some(canonical_key) = canonical_key(option_key) else {
            self.push_unknown_key(&scoped_key);
            return;
        };

        match canonical_key {
            CanonicalKey::LineWidth => {
                if let Some(parsed) = self.parse_u32(&scoped_key, option_value) {
                    target.line_width = Some(parsed);
                }
            }
            CanonicalKey::WrapStyle => {
                if let Some(parsed) = self.parse_wrap_style(&scoped_key, option_value) {
                    target.wrap_style = Some(parsed);
                }
            }
            CanonicalKey::FirstArgSameLine => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.first_arg_same_line = Some(parsed);
                }
            }
            CanonicalKey::WrapArgThreshold => {
                if let Some(parsed) = self.parse_u32(&scoped_key, option_value) {
                    target.wrap_arg_threshold = Some(parsed);
                }
            }
            CanonicalKey::IndentWidth => {
                if let Some(parsed) = self.parse_u8(&scoped_key, option_value) {
                    target.indent_width = Some(parsed);
                }
            }
            CanonicalKey::IndentStyle => {
                if let Some(parsed) = self.parse_indent_style(&scoped_key, option_value) {
                    target.indent_style = Some(parsed);
                }
            }
            CanonicalKey::ContinuationIndentWidth => {
                if let Some(parsed) = self.parse_nullable_u8(&scoped_key, option_value) {
                    target.continuation_indent_width = parsed;
                }
            }
            CanonicalKey::CommandCase => {
                if let Some(parsed) =
                    self.parse_case_style(&scoped_key, option_value, "commandCase")
                {
                    target.command_case = Some(parsed);
                }
            }
            CanonicalKey::KeywordCase => {
                if let Some(parsed) =
                    self.parse_case_style(&scoped_key, option_value, "keywordCase")
                {
                    target.keyword_case = Some(parsed);
                }
            }
            CanonicalKey::CustomKeywords => {
                if let Some(parsed) = self.parse_string_array(
                    &scoped_key,
                    option_value,
                    false,
                    "array of keyword strings",
                ) {
                    target.custom_keywords = Some(parsed);
                }
            }
            CanonicalKey::LiteralCase => {
                if let Some(parsed) =
                    self.parse_case_style(&scoped_key, option_value, "literalCase")
                {
                    target.literal_case = Some(parsed);
                }
            }
            CanonicalKey::ClosingParenNewline => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.closing_paren_newline = Some(parsed);
                }
            }
            CanonicalKey::SpaceBeforeParen => {
                if let Some(parsed) = self.parse_space_before_paren(&scoped_key, option_value) {
                    target.space_before_paren = Some(parsed);
                }
            }
            CanonicalKey::SpaceInsideParen => {
                if let Some(parsed) = self.parse_space_inside_paren(&scoped_key, option_value) {
                    target.space_inside_paren = Some(parsed);
                }
            }
            CanonicalKey::CommentPreservation => {
                if let Some(parsed) = self.parse_comment_preservation(&scoped_key, option_value) {
                    target.comment_preservation = Some(parsed);
                }
            }
            CanonicalKey::CommentWidth => {
                if let Some(parsed) = self.parse_nullable_u32(&scoped_key, option_value) {
                    target.comment_width = parsed;
                }
            }
            CanonicalKey::AlignTrailingComments => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.align_trailing_comments = Some(parsed);
                }
            }
            CanonicalKey::CommentGap => {
                if let Some(parsed) = self.parse_u8(&scoped_key, option_value) {
                    target.comment_gap = Some(parsed);
                }
            }
            CanonicalKey::AlignPropertyValues => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.align_property_values = Some(parsed);
                }
            }
            CanonicalKey::AlignConsecutiveSet => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.align_consecutive_set = Some(parsed);
                }
            }
            CanonicalKey::AlignArgGroups => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.align_arg_groups = Some(parsed);
                }
            }
            CanonicalKey::SortArguments => {
                if let Some(parsed) = self.parse_sort_arguments(&scoped_key, option_value) {
                    target.sort_arguments = Some(parsed);
                }
            }
            CanonicalKey::SortKeywordSections => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.sort_keyword_sections = Some(parsed);
                }
            }
            // These options are intentionally excluded from command-level overrides.
            CanonicalKey::NewLineKind
            | CanonicalKey::FinalNewline
            | CanonicalKey::MaxBlankLines
            | CanonicalKey::MinBlankLinesBetweenBlocks
            | CanonicalKey::BlankLineBetweenSections
            | CanonicalKey::TrimTrailingWhitespace
            | CanonicalKey::CollapseSpaces
            | CanonicalKey::PerCommandConfig
            | CanonicalKey::DisableFormatting
            | CanonicalKey::IgnorePatterns
            | CanonicalKey::IgnoreCommands
            | CanonicalKey::IndentBlockBody
            | CanonicalKey::EndCommandArgs
            | CanonicalKey::Extends
            | CanonicalKey::IndentMode
            | CanonicalKey::UseTabs => {
                self.push_unknown_key(&scoped_key);
            }
        }
    }

    fn parse_indent_mode(&mut self, key: &str, value: RawConfigValue) {
        if let Some(mode) = value.as_lower_string() {
            if mode == "tabs" {
                self.config.use_tabs = true;
                self.config.indent_style = IndentStyle::Tab;
                self.record_override(key, "tabs".to_string());
                return;
            }

            if mode == "spaces" {
                self.config.use_tabs = false;
                self.config.indent_style = IndentStyle::Space;
                self.record_override(key, "spaces".to_string());
                return;
            }
        }

        if let Some(width) = value.as_u8_number() {
            self.config.indent_width = width;
            self.config.use_tabs = false;
            self.config.indent_style = IndentStyle::Space;
            self.record_override(key, width.to_string());
            return;
        }

        self.push_invalid_type(key, "string ('tabs'/'spaces') or integer for indent width");
    }

    fn parse_bool(&mut self, key: &str, value: RawConfigValue) -> Option<bool> {
        let Some(parsed) = value.as_bool() else {
            self.push_invalid_type(key, "boolean");
            return None;
        };

        Some(parsed)
    }

    fn parse_u8(&mut self, key: &str, value: RawConfigValue) -> Option<u8> {
        let Some(parsed) = value.as_u8_number() else {
            self.push_invalid_type(key, "integer in 0..=255");
            return None;
        };

        Some(parsed)
    }

    fn parse_nullable_u8(&mut self, key: &str, value: RawConfigValue) -> Option<Option<u8>> {
        if value.is_null() {
            return Some(None);
        }

        self.parse_u8(key, value).map(Some)
    }

    fn parse_u32(&mut self, key: &str, value: RawConfigValue) -> Option<u32> {
        let Some(parsed) = value.as_u32_number() else {
            self.push_invalid_type(key, "non-negative integer");
            return None;
        };

        Some(parsed)
    }

    fn parse_nullable_u32(&mut self, key: &str, value: RawConfigValue) -> Option<Option<u32>> {
        if value.is_null() {
            return Some(None);
        }

        self.parse_u32(key, value).map(Some)
    }

    fn parse_string(&mut self, key: &str, value: RawConfigValue) -> Option<String> {
        let Some(parsed) = value.into_string() else {
            self.push_invalid_type(key, "string");
            return None;
        };

        Some(parsed)
    }

    fn parse_string_array(
        &mut self,
        key: &str,
        value: RawConfigValue,
        lowercase: bool,
        expected: &str,
    ) -> Option<Vec<String>> {
        let mut values = match value {
            RawConfigValue::Json(serde_json::Value::Array(items)) => {
                let mut values = Vec::new();
                for item in items {
                    match item {
                        serde_json::Value::String(text) => values.push(text),
                        _ => {
                            self.push_invalid_type(key, expected);
                            return None;
                        }
                    }
                }
                values
            }
            RawConfigValue::Text(text) => {
                let Some(values) = parse_text_list(&text) else {
                    self.push_invalid_type(key, expected);
                    return None;
                };
                values
            }
            _ => {
                self.push_invalid_type(key, expected);
                return None;
            }
        };

        if lowercase {
            values = values
                .into_iter()
                .map(|value| value.to_ascii_lowercase())
                .collect();
        }

        Some(values)
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

    fn record_override(&mut self, key: &str, value: String) {
        self.overrides.push((key.to_string(), value));
    }

    fn record_override_nullable_u8(&mut self, key: &str, value: Option<u8>) {
        self.record_override(
            key,
            value
                .map(|number| number.to_string())
                .unwrap_or_else(|| "null".to_string()),
        );
    }

    fn record_override_nullable_u32(&mut self, key: &str, value: Option<u32>) {
        self.record_override(
            key,
            value
                .map(|number| number.to_string())
                .unwrap_or_else(|| "null".to_string()),
        );
    }
}

enum RawConfigValue {
    Json(serde_json::Value),
    Text(String),
}

impl RawConfigValue {
    fn into_string(self) -> Option<String> {
        match self {
            RawConfigValue::Json(serde_json::Value::String(value)) => Some(value),
            RawConfigValue::Text(value) => Some(trim_quotes(&value)),
            _ => None,
        }
    }

    fn as_lower_string(&self) -> Option<String> {
        match self {
            RawConfigValue::Json(serde_json::Value::String(value)) => {
                Some(value.to_ascii_lowercase())
            }
            RawConfigValue::Text(value) => Some(trim_quotes(value).to_ascii_lowercase()),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
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
            RawConfigValue::Json(serde_json::Value::Number(value)) => value.as_i64(),
            RawConfigValue::Text(value) => trim_quotes(value).parse::<i64>().ok(),
            _ => None,
        }
    }

    fn is_null(&self) -> bool {
        match self {
            RawConfigValue::Json(serde_json::Value::Null) => true,
            RawConfigValue::Text(value) => trim_quotes(value).eq_ignore_ascii_case("null"),
            _ => false,
        }
    }

    fn into_object_entries(self) -> Option<Vec<(String, RawConfigValue)>> {
        match self {
            RawConfigValue::Json(serde_json::Value::Object(map)) => Some(
                map.into_iter()
                    .map(|(key, value)| (key, RawConfigValue::Json(value)))
                    .collect(),
            ),
            _ => None,
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

fn parse_text_list(value: &str) -> Option<Vec<String>> {
    let trimmed = value.trim();
    let inner = trimmed.strip_prefix('[')?.strip_suffix(']')?;

    let values = inner
        .split(',')
        .map(|entry| trim_quotes(entry.trim()))
        .filter(|entry| !entry.is_empty())
        .collect();

    Some(values)
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
            Some((
                key.trim().trim_matches('"').to_string(),
                value.trim().to_string(),
            ))
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
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    struct TempDirGuard {
        path: PathBuf,
    }

    impl TempDirGuard {
        fn new(prefix: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock before UNIX_EPOCH")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "cmakefmt_{prefix}_{}_{}",
                std::process::id(),
                unique
            ));
            std::fs::create_dir_all(&path).expect("failed to create temporary test directory");
            Self { path }
        }
    }

    impl Drop for TempDirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn parses_canonical_keys_from_json_header() {
        let result =
            load_from_header(r#"{"lineWidth": 100, "commandCase": "upper", "sortLists": true}"#);

        assert_eq!(result.config.line_width, 100);
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert!(result.config.sort_lists);
        assert!(matches!(
            result.config.sort_arguments,
            SortArguments::Enabled
        ));
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
        assert_eq!(result.config.indent_style, IndentStyle::Tab);
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

        assert_eq!(
            result.config.line_width,
            Configuration::default().line_width
        );
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

        assert_eq!(
            result.config.command_case,
            Configuration::default().command_case
        );
        assert_eq!(
            result.config.sort_lists,
            Configuration::default().sort_lists
        );
        assert_eq!(result.diagnostics.len(), 2);
        assert_eq!(result.diagnostics[0].key, "commandCase");
        assert!(
            result.diagnostics[0]
                .message
                .contains("Invalid commandCase value")
        );
        assert_eq!(result.diagnostics[1].key, "sortLists");
        assert!(
            result.diagnostics[1]
                .message
                .contains("expected boolean or array")
        );
    }

    #[test]
    fn emits_diagnostics_for_unknown_keys() {
        let result = load_from_header(r#"{"unknownKey": 1}"#);
        assert_eq!(
            result.config.line_width,
            Configuration::default().line_width
        );
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].key, "unknownKey");
        assert_eq!(
            result.diagnostics[0].message,
            "Unknown property in configuration"
        );
    }

    #[test]
    fn applies_base_defaults_then_json_map() {
        let mut map = serde_json::Map::new();
        map.insert(
            "lineWidth".to_string(),
            serde_json::Value::Number(90.into()),
        );
        map.insert("sortLists".to_string(), serde_json::Value::Bool(true));

        let base = Configuration {
            line_width: 120,
            indent_width: 6,
            use_tabs: true,
            indent_style: IndentStyle::Tab,
            ..Default::default()
        };

        let result = load_from_json_map(map, base);
        assert_eq!(result.config.line_width, 90);
        assert_eq!(result.config.indent_width, 6);
        assert!(result.config.use_tabs);
        assert_eq!(result.config.indent_style, IndentStyle::Tab);
        assert!(result.config.sort_lists);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_space_before_paren_json_array() {
        let result = load_from_header(r#"{"spaceBeforeParen": ["if", "while", "foreach"]}"#);
        assert_eq!(
            result.config.space_before_paren,
            SpaceBeforeParen::CommandList(vec![
                "if".to_string(),
                "while".to_string(),
                "foreach".to_string()
            ])
        );
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_space_before_paren_bool() {
        let result = load_from_header(r#"{"spaceBeforeParen": true}"#);
        assert_eq!(result.config.space_before_paren, SpaceBeforeParen::Always);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn space_before_paren_invalid_value_emits_diagnostic() {
        let result = load_from_header(r#"{"spaceBeforeParen": 42}"#);
        assert_eq!(result.config.space_before_paren, SpaceBeforeParen::Never);
        assert_eq!(result.diagnostics.len(), 1);
        assert!(
            result.diagnostics[0]
                .message
                .contains("expected boolean or array")
        );
    }

    #[test]
    fn parses_sort_arguments_array() {
        let result = load_from_header(r#"{"sortArguments": ["sources", "files"]}"#);

        assert_eq!(
            result.config.sort_arguments,
            SortArguments::CommandList(vec!["sources".to_string(), "files".to_string()])
        );
        assert!(!result.config.sort_lists);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_nullable_inherited_fields() {
        let result = load_from_header(r#"{"commentWidth": null, "continuationIndentWidth": 6}"#);

        assert_eq!(result.config.comment_width, None);
        assert_eq!(result.config.continuation_indent_width, Some(6));
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_per_command_config_table() {
        let result = load_from_header(
            r#"{"perCommandConfig": {"if": {"spaceBeforeParen": true, "lineWidth": 100}}}"#,
        );

        let per_command = result
            .config
            .per_command_config
            .get("if")
            .expect("expected if command override");
        assert_eq!(per_command.line_width, Some(100));
        assert_eq!(
            per_command.space_before_paren,
            Some(SpaceBeforeParen::Always)
        );
        assert!(result.diagnostics.is_empty());
    }
    #[test]
    fn parses_toml_arrays_and_per_command_tables() {
        let result = load_from_toml(
            r#"
sortArguments = ["SOURCES", "FILES"]

[perCommandConfig.if]
spaceBeforeParen = true
lineWidth = 100
"#,
        );

        assert_eq!(
            result.config.sort_arguments,
            SortArguments::CommandList(vec!["sources".to_string(), "files".to_string()])
        );
        let per_command = result
            .config
            .per_command_config
            .get("if")
            .expect("expected if command override");
        assert_eq!(per_command.line_width, Some(100));
        assert_eq!(
            per_command.space_before_paren,
            Some(SpaceBeforeParen::Always)
        );
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_appendix_b_example_config() {
        let result = load_from_toml(
            r#"
lineWidth = 120
indentWidth = 4
indentStyle = "space"
commandCase = "lower"
keywordCase = "upper"
closingParenNewline = true
lineEnding = "lf"
finalNewline = true
trimTrailingWhitespace = true
endCommandArgs = "remove"

sortArguments = ["SOURCES", "FILES"]
alignPropertyValues = true

ignorePatterns = ["build/**", "third_party/**"]
ignoreCommands = ["ExternalProject_Add"]

[perCommandConfig.if]
spaceBeforeParen = true

[perCommandConfig.elseif]
spaceBeforeParen = true
"#,
        );

        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.line_width, 120);
        assert_eq!(result.config.indent_width, 4);
        assert_eq!(result.config.indent_style, IndentStyle::Space);
        assert!(!result.config.use_tabs);
        assert_eq!(result.config.command_case, CaseStyle::Lower);
        assert_eq!(result.config.keyword_case, CaseStyle::Upper);
        assert!(result.config.closing_paren_newline);
        assert_eq!(result.config.new_line_kind, NewLineKind::Lf);
        assert!(result.config.final_newline);
        assert!(result.config.trim_trailing_whitespace);
        assert_eq!(result.config.end_command_args, EndCommandArgs::Remove);
        assert!(result.config.align_property_values);
        assert_eq!(
            result.config.sort_arguments,
            SortArguments::CommandList(vec!["sources".to_string(), "files".to_string()])
        );
        assert_eq!(
            result.config.ignore_patterns,
            vec!["build/**".to_string(), "third_party/**".to_string()]
        );
        assert_eq!(
            result.config.ignore_commands,
            vec!["ExternalProject_Add".to_string()]
        );

        let if_config = result
            .config
            .per_command_config
            .get("if")
            .expect("expected if command override");
        assert_eq!(if_config.space_before_paren, Some(SpaceBeforeParen::Always));

        let elseif_config = result
            .config
            .per_command_config
            .get("elseif")
            .expect("expected elseif command override");
        assert_eq!(
            elseif_config.space_before_paren,
            Some(SpaceBeforeParen::Always)
        );
    }

    #[test]
    fn discovers_nearest_config_with_dotfile_precedence() {
        let temp = TempDirGuard::new("config_discovery");
        let repo_root = temp.path.join("repo");
        let nested = repo_root.join("src/module");
        std::fs::create_dir_all(&nested).expect("failed to create nested fixture directory");

        let parent_plain = repo_root.join(PLAIN_CONFIG_NAME);
        std::fs::write(&parent_plain, "indentWidth = 4\n")
            .expect("failed to write parent plain config");

        let discovered_plain =
            discover_toml_path(&nested.join("CMakeLists.txt")).expect("expected discovered config");
        assert_eq!(discovered_plain, parent_plain);

        let parent_dotfile = repo_root.join(DOTFILE_CONFIG_NAME);
        std::fs::write(&parent_dotfile, "indentWidth = 6\n")
            .expect("failed to write parent dotfile config");

        let discovered_dotfile = discover_toml_path(&nested.join("CMakeLists.txt"))
            .expect("expected discovered dotfile");
        assert_eq!(discovered_dotfile, parent_dotfile);
    }

    #[test]
    fn resolves_extends_scalar_override_fixture() {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/formatter/15_config_meta/02_extends/scalar_override/.cmakefmt.toml");

        let result = load_from_toml_path(&config_path);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.command_case, CaseStyle::Lower);
        assert_eq!(result.config.indent_width, 4);
    }

    #[test]
    fn resolves_extends_array_replacement_fixture() {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/formatter/15_config_meta/02_extends/array_replaces/.cmakefmt.toml");

        let result = load_from_toml_path(&config_path);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert_eq!(
            result.config.space_before_paren,
            SpaceBeforeParen::CommandList(vec!["if".to_string()])
        );
    }

    #[test]
    fn resolves_extends_per_command_shallow_merge_fixture() {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "tests/formatter/15_config_meta/02_extends/per_command_shallow_merge/.cmakefmt.toml",
        );

        let result = load_from_toml_path(&config_path);
        assert!(result.diagnostics.is_empty());

        let set_override = result
            .config
            .per_command_config
            .get("set")
            .expect("expected set override");
        assert_eq!(set_override.wrap_style, Some(WrapStyle::Cascade));
        assert_eq!(set_override.closing_paren_newline, None);

        let message_override = result
            .config
            .per_command_config
            .get("message")
            .expect("expected message override");
        assert_eq!(message_override.command_case, Some(CaseStyle::Upper));
    }

    #[test]
    fn rebases_ignore_patterns_relative_to_declaring_config_file() {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/formatter/16_suppression/03_ignore_patterns/.cmakefmt.toml");

        let result = load_from_toml_path(&config_path);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.ignore_patterns.len(), 1);

        let normalized = result.config.ignore_patterns[0].replace('\\', "/");
        assert!(
            normalized
                .ends_with("/tests/formatter/16_suppression/03_ignore_patterns/ignored/*.cmake"),
            "unexpected rebased ignore pattern: {normalized}"
        );
    }

    #[test]
    fn keeps_inherited_ignore_patterns_relative_to_base_config() {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "tests/formatter/16_suppression/03_ignore_patterns/inherited_relative/child/.cmakefmt.toml",
        );

        let result = load_from_toml_path(&config_path);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.ignore_patterns.len(), 1);

        let normalized = result.config.ignore_patterns[0].replace('\\', "/");
        assert!(
            normalized.ends_with(
                "/tests/formatter/16_suppression/03_ignore_patterns/inherited_relative/base/vendor/*.cmake"
            ),
            "unexpected inherited ignore pattern: {normalized}"
        );
    }

    #[test]
    fn detects_circular_extends_references() {
        let temp = TempDirGuard::new("cycle_extends");
        let first_dir = temp.path.join("first");
        let second_dir = temp.path.join("second");
        std::fs::create_dir_all(&first_dir).expect("failed to create first directory");
        std::fs::create_dir_all(&second_dir).expect("failed to create second directory");

        let first_config = first_dir.join(DOTFILE_CONFIG_NAME);
        let second_config = second_dir.join(DOTFILE_CONFIG_NAME);

        std::fs::write(
            &first_config,
            "extends = \"../second/.cmakefmt.toml\"\ncommandCase = \"upper\"\n",
        )
        .expect("failed to write first config");
        std::fs::write(
            &second_config,
            "extends = \"../first/.cmakefmt.toml\"\nindentWidth = 4\n",
        )
        .expect("failed to write second config");

        let result = load_from_toml_path(&first_config);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("Circular extends reference detected")
        }));
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert_eq!(result.config.indent_width, 4);
    }

    #[test]
    fn detects_extends_depth_limit() {
        let temp = TempDirGuard::new("depth_extends");
        let chain_len = MAX_EXTENDS_DEPTH + 2;

        for index in 0..chain_len {
            let config_dir = temp.path.join(format!("level_{index}"));
            std::fs::create_dir_all(&config_dir).expect("failed to create level directory");
            let config_path = config_dir.join(DOTFILE_CONFIG_NAME);

            if index + 1 < chain_len {
                let next = format!("../level_{}/.cmakefmt.toml", index + 1);
                std::fs::write(&config_path, format!("extends = \"{next}\"\n"))
                    .expect("failed to write extends config");
            } else {
                std::fs::write(&config_path, "commandCase = \"upper\"\n")
                    .expect("failed to write tail config");
            }
        }

        let root_config = temp.path.join("level_0").join(DOTFILE_CONFIG_NAME);
        let result = load_from_toml_path(&root_config);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("Exceeded maximum extends depth")
        }));
    }

    #[test]
    fn reports_diagnostic_for_invalid_toml_assignment() {
        let result = load_from_toml(
            r#"
commandCase = "upper"
this_is_not_valid_toml
"#,
        );

        assert_eq!(
            result.config.command_case,
            Configuration::default().command_case
        );
        assert!(result.overrides.is_empty());
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].key, "<toml>");
        assert!(
            result.diagnostics[0]
                .message
                .contains("Failed to parse TOML document")
        );
        assert_eq!(
            result.diagnostics[0].severity,
            ConfigDiagnosticSeverity::Warning
        );
    }

    #[test]
    fn empty_toml_returns_defaults() {
        let result = load_from_toml("");
        assert!(result.diagnostics.is_empty());
        assert_eq!(
            result.config.line_width,
            Configuration::default().line_width
        );
        assert_eq!(
            result.config.indent_width,
            Configuration::default().indent_width
        );
        assert_eq!(
            result.config.command_case,
            Configuration::default().command_case
        );
    }

    #[test]
    fn load_from_header_empty_returns_defaults() {
        let result = load_from_header("{}");
        assert!(result.diagnostics.is_empty());
        assert_eq!(
            result.config.line_width,
            Configuration::default().line_width
        );
    }

    #[test]
    fn load_from_header_with_overrides() {
        let result = load_from_header(r#"{"lineWidth": 120, "indentWidth": 4}"#);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.line_width, 120);
        assert_eq!(result.config.indent_width, 4);
    }

    #[test]
    fn invalid_enum_value_produces_diagnostic() {
        let result = load_from_toml("commandCase = \"invalid_value\"\n");
        assert!(
            result.diagnostics.iter().any(|d| d.key == "commandCase"),
            "expected diagnostic for invalid commandCase, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn unknown_key_produces_diagnostic() {
        let result = load_from_toml("completelyUnknownKey = true\n");
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.key == "completelyUnknownKey"),
            "expected diagnostic for unknown key, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn load_from_json_map_basic() {
        let mut map = serde_json::Map::new();
        map.insert(
            "lineWidth".to_string(),
            serde_json::Value::Number(serde_json::Number::from(100)),
        );
        map.insert(
            "commandCase".to_string(),
            serde_json::Value::String("upper".to_string()),
        );

        let result = load_from_json_map(map, Configuration::default());
        assert_eq!(result.config.line_width, 100);
        assert_eq!(result.config.command_case, CaseStyle::Upper);
    }

    #[test]
    fn load_from_json_map_empty() {
        let map = serde_json::Map::new();
        let result = load_from_json_map(map, Configuration::default());
        assert_eq!(
            result.config.line_width,
            Configuration::default().line_width
        );
    }

    // -----------------------------------------------------------------------
    // Edge case: per-command config with multiple commands
    // -----------------------------------------------------------------------

    #[test]
    fn per_command_config_multiple_commands() {
        let result = load_from_toml(
            r#"
[perCommandConfig.set]
wrapStyle = "cascade"

[perCommandConfig.message]
commandCase = "upper"

[perCommandConfig.if]
spaceBeforeParen = true
lineWidth = 100
"#,
        );

        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.per_command_config.len(), 3);

        let set_cfg = result.config.per_command_config.get("set").unwrap();
        assert_eq!(set_cfg.wrap_style, Some(WrapStyle::Cascade));

        let msg_cfg = result.config.per_command_config.get("message").unwrap();
        assert_eq!(msg_cfg.command_case, Some(CaseStyle::Upper));

        let if_cfg = result.config.per_command_config.get("if").unwrap();
        assert_eq!(if_cfg.space_before_paren, Some(SpaceBeforeParen::Always));
        assert_eq!(if_cfg.line_width, Some(100));
    }

    // -----------------------------------------------------------------------
    // Edge case: all non-default values round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn all_non_default_values() {
        let result = load_from_toml(
            r#"
lineWidth = 200
indentWidth = 8
indentStyle = "tab"
commandCase = "upper"
keywordCase = "preserve"
closingParenNewline = false
lineEnding = "crlf"
finalNewline = false
trimTrailingWhitespace = false
maxBlankLines = 5
wrapStyle = "vertical"
firstArgSameLine = false
sortLists = true
endCommandArgs = "match"
indentBlockBody = false
"#,
        );

        assert!(
            result.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            result.diagnostics
        );
        assert_eq!(result.config.line_width, 200);
        assert_eq!(result.config.indent_width, 8);
        assert_eq!(result.config.indent_style, IndentStyle::Tab);
        assert!(result.config.use_tabs);
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert_eq!(result.config.keyword_case, CaseStyle::Preserve);
        assert!(!result.config.closing_paren_newline);
        assert_eq!(result.config.new_line_kind, NewLineKind::CrLf);
        assert!(!result.config.final_newline);
        assert!(!result.config.trim_trailing_whitespace);
        assert_eq!(result.config.max_blank_lines, 5);
        assert_eq!(result.config.wrap_style, WrapStyle::Vertical);
        assert!(!result.config.first_arg_same_line);
        assert!(result.config.sort_lists);
        assert_eq!(result.config.end_command_args, EndCommandArgs::Match);
        assert!(!result.config.indent_block_body);
    }

    // -----------------------------------------------------------------------
    // Edge case: per-command config with unknown sub-key emits diagnostic
    // -----------------------------------------------------------------------

    #[test]
    fn per_command_unknown_sub_key_diagnostic() {
        let result = load_from_toml(
            r#"
[perCommandConfig.set]
unknownSubKey = true
"#,
        );

        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("Unknown")),
            "expected diagnostic for unknown sub-key, got: {:?}",
            result.diagnostics
        );
    }

    // -----------------------------------------------------------------------
    // Edge case: ignore patterns is an array of strings
    // -----------------------------------------------------------------------

    #[test]
    fn ignore_patterns_array() {
        let result = load_from_toml(
            "ignorePatterns = [\"build/**\", \"third_party/**\", \"*.gen.cmake\"]\n",
        );

        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.ignore_patterns.len(), 3);
    }

    // -----------------------------------------------------------------------
    // Edge case: ignore commands is an array of strings
    // -----------------------------------------------------------------------

    #[test]
    fn ignore_commands_array() {
        let result = load_from_toml(
            "ignoreCommands = [\"ExternalProject_Add\", \"FetchContent_Declare\"]\n",
        );

        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.ignore_commands.len(), 2);
        assert_eq!(result.config.ignore_commands[0], "ExternalProject_Add");
        assert_eq!(result.config.ignore_commands[1], "FetchContent_Declare");
    }

    // -----------------------------------------------------------------------
    // Edge case: multiple diagnostics from a single config
    // -----------------------------------------------------------------------

    #[test]
    fn multiple_diagnostics_collected() {
        let result = load_from_toml(
            r#"
commandCase = "invalid"
keywordCase = "invalid"
unknownKey1 = true
unknownKey2 = 42
"#,
        );

        assert!(
            result.diagnostics.len() >= 4,
            "expected at least 4 diagnostics, got {}: {:?}",
            result.diagnostics.len(),
            result.diagnostics
        );
    }

    // -----------------------------------------------------------------------
    // Edge case: inline override format parsing
    // -----------------------------------------------------------------------

    #[test]
    fn inline_override_json_format() {
        let base = Configuration::default();
        let overridden =
            apply_inline_overrides(&base, r#"{ "lineWidth": 40, "commandCase": "upper" }"#);

        assert_eq!(overridden.line_width, 40);
        assert_eq!(overridden.command_case, CaseStyle::Upper);
    }

    #[test]
    fn inline_override_toml_format() {
        let base = Configuration::default();
        let overridden = apply_inline_overrides(&base, "{ lineWidth = 40 }");

        assert_eq!(overridden.line_width, 40);
    }

    #[test]
    fn inline_override_malformed_returns_base() {
        let base = Configuration::default();
        let overridden = apply_inline_overrides(&base, "{ this is not valid }");

        // Malformed input should return the base config unchanged
        assert_eq!(overridden.line_width, base.line_width);
        assert_eq!(overridden.command_case, base.command_case);
    }
}

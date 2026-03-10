use std::collections::BTreeMap;

use dprint_core::configuration::{
    ConfigKeyMap, ConfigKeyValue, GlobalConfiguration, NewLineKind as DprintNewLineKind,
};

use super::types::{
    CaseStyle, CommandConfiguration, CommentPreservation, Configuration, EndCommandArgs, GenexWrap,
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

/// Loads plugin configuration from dprint's key/value map and global defaults.
pub fn load_from_dprint(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ConfigLoadResult {
    let mut loader = Loader::new(base_from_global(global_config));
    for (key, value) in config.drain(..) {
        loader.apply_raw_value(&key, RawConfigValue::Dprint(value));
    }
    loader.finish()
}

/// Parses a `.cmakefmt.toml` document into formatter configuration plus diagnostics.
pub fn load_from_toml(toml: &str) -> ConfigLoadResult {
    let mut loader = Loader::new(Configuration::default());

    let parsed = match toml::from_str::<toml::Table>(toml) {
        Ok(table) => table,
        Err(error) => {
            loader.diagnostics.push(ConfigDiagnostic {
                key: "<toml>".to_string(),
                message: format!("Failed to parse TOML document: {error}"),
                severity: ConfigDiagnosticSeverity::Warning,
            });
            return loader.finish();
        }
    };

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

    loader.finish()
}

/// Reads and parses a `.cmakefmt.toml` file from disk into formatter configuration plus diagnostics.
pub fn load_from_toml_path(path: &std::path::Path) -> ConfigLoadResult {
    match std::fs::read_to_string(path) {
        Ok(contents) => load_from_toml(&contents),
        Err(error) => ConfigLoadResult {
            config: Configuration::default(),
            diagnostics: vec![ConfigDiagnostic {
                key: path.display().to_string(),
                message: format!("Failed to read TOML document: {error}"),
                severity: ConfigDiagnosticSeverity::Warning,
            }],
            overrides: Vec::new(),
        },
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

#[derive(Debug, Clone, Copy)]
enum CanonicalKey {
    LineWidth,
    WrapStyle,
    FirstArgSameLine,
    WrapArgThreshold,
    MagicTrailingNewline,
    IndentWidth,
    UseTabs,
    IndentStyle,
    ContinuationIndentWidth,
    GenexIndentWidth,
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
    GenexWrap,
    GenexClosingAngleNewline,
    PerCommandConfig,
    SortArguments,
    SortKeywordSections,
    DisableFormatting,
    IgnorePatterns,
    IgnoreCommands,
    IndentBlockBody,
    EndCommandArgs,
    Schema,
    Extends,
    IndentMode,
}

fn canonical_key(key: &str) -> Option<CanonicalKey> {
    match key {
        "lineWidth" | "line_width" => Some(CanonicalKey::LineWidth),
        "wrapStyle" | "wrap_style" => Some(CanonicalKey::WrapStyle),
        "firstArgSameLine" | "first_arg_same_line" => Some(CanonicalKey::FirstArgSameLine),
        "wrapArgThreshold" | "wrap_arg_threshold" => Some(CanonicalKey::WrapArgThreshold),
        "magicTrailingNewline" | "magic_trailing_newline" => {
            Some(CanonicalKey::MagicTrailingNewline)
        }
        "indentWidth" | "indent_width" => Some(CanonicalKey::IndentWidth),
        "useTabs" | "use_tabs" => Some(CanonicalKey::UseTabs),
        "indentStyle" | "indent_style" => Some(CanonicalKey::IndentStyle),
        "continuationIndentWidth" | "continuation_indent_width" => {
            Some(CanonicalKey::ContinuationIndentWidth)
        }
        "genexIndentWidth" | "genex_indent_width" => Some(CanonicalKey::GenexIndentWidth),
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
        "genexWrap" | "genex_wrap" => Some(CanonicalKey::GenexWrap),
        "genexClosingAngleNewline" | "genex_closing_angle_newline" => {
            Some(CanonicalKey::GenexClosingAngleNewline)
        }
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
        "$schema" => Some(CanonicalKey::Schema),
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
            CanonicalKey::MagicTrailingNewline => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.magic_trailing_newline = parsed;
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
            CanonicalKey::GenexIndentWidth => {
                if let Some(parsed) = self.parse_nullable_u8(key, value) {
                    self.config.genex_indent_width = parsed;
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
            CanonicalKey::GenexWrap => {
                if let Some(parsed) = self.parse_genex_wrap(key, value) {
                    self.config.genex_wrap = parsed;
                    self.record_override(key, format!("{parsed:?}"));
                }
            }
            CanonicalKey::GenexClosingAngleNewline => {
                if let Some(parsed) = self.parse_bool(key, value) {
                    self.config.genex_closing_angle_newline = parsed;
                    self.record_override(key, parsed.to_string());
                }
            }
            CanonicalKey::PerCommandConfig => self.parse_per_command_config(key, value),
            CanonicalKey::SortArguments => {
                if let Some(parsed) = self.parse_sort_arguments(key, value) {
                    self.config.sort_lists = matches!(parsed, SortArguments::Enabled);
                    self.config.sort_arguments = parsed;
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
            CanonicalKey::Schema => {
                if let Some(parsed) = self.parse_string(key, value) {
                    self.config.schema = Some(parsed.clone());
                    self.record_override(key, parsed);
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

    fn parse_genex_wrap(&mut self, key: &str, value: RawConfigValue) -> Option<GenexWrap> {
        let parsed = self.parse_string(key, value)?;
        let Ok(style) = parsed.parse::<GenexWrap>() else {
            self.push_invalid_value(
                key,
                format!("Invalid genexWrap value '{parsed}'. Expected one of: cascade, never"),
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

        self.config.per_command_config = parsed;
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
            CanonicalKey::MagicTrailingNewline => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.magic_trailing_newline = Some(parsed);
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
            CanonicalKey::GenexIndentWidth => {
                if let Some(parsed) = self.parse_nullable_u8(&scoped_key, option_value) {
                    target.genex_indent_width = parsed;
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
            CanonicalKey::GenexWrap => {
                if let Some(parsed) = self.parse_genex_wrap(&scoped_key, option_value) {
                    target.genex_wrap = Some(parsed);
                }
            }
            CanonicalKey::GenexClosingAngleNewline => {
                if let Some(parsed) = self.parse_bool(&scoped_key, option_value) {
                    target.genex_closing_angle_newline = Some(parsed);
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
            | CanonicalKey::Schema
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
            RawConfigValue::Dprint(ConfigKeyValue::Array(items)) => {
                let mut values = Vec::new();
                for item in items {
                    match item {
                        ConfigKeyValue::String(text) => values.push(text),
                        _ => {
                            self.push_invalid_type(key, expected);
                            return None;
                        }
                    }
                }
                values
            }
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
        match self {
            RawConfigValue::Dprint(ConfigKeyValue::String(value)) => {
                Some(value.to_ascii_lowercase())
            }
            RawConfigValue::Json(serde_json::Value::String(value)) => {
                Some(value.to_ascii_lowercase())
            }
            RawConfigValue::Text(value) => Some(trim_quotes(value).to_ascii_lowercase()),
            _ => None,
        }
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
            RawConfigValue::Dprint(ConfigKeyValue::Number(value)) => Some(i64::from(*value)),
            RawConfigValue::Json(serde_json::Value::Number(value)) => value.as_i64(),
            RawConfigValue::Text(value) => trim_quotes(value).parse::<i64>().ok(),
            _ => None,
        }
    }

    fn is_null(&self) -> bool {
        match self {
            RawConfigValue::Dprint(ConfigKeyValue::Null) => true,
            RawConfigValue::Json(serde_json::Value::Null) => true,
            RawConfigValue::Text(value) => trim_quotes(value).eq_ignore_ascii_case("null"),
            _ => false,
        }
    }

    fn into_object_entries(self) -> Option<Vec<(String, RawConfigValue)>> {
        match self {
            RawConfigValue::Dprint(ConfigKeyValue::Object(map)) => Some(
                map.into_iter()
                    .map(|(key, value)| (key, RawConfigValue::Dprint(value)))
                    .collect(),
            ),
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
    use dprint_core::configuration::ConfigKeyMap;

    use super::*;

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
    fn parses_schema_without_diagnostic() {
        let result = load_from_header(
            r#"{"$schema": "https://example.com/cmakefmt-schema.json", "commandCase": "upper"}"#,
        );

        assert_eq!(
            result.config.schema,
            Some("https://example.com/cmakefmt-schema.json".to_string())
        );
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn parses_nullable_inherited_fields() {
        let result = load_from_header(
            r#"{"commentWidth": null, "continuationIndentWidth": 6, "genexIndentWidth": null}"#,
        );

        assert_eq!(result.config.comment_width, None);
        assert_eq!(result.config.continuation_indent_width, Some(6));
        assert_eq!(result.config.genex_indent_width, None);
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
    fn parses_toml_schema_without_diagnostic() {
        let result = load_from_toml(
            r#"
"$schema" = "https://example.com/cmakefmt-schema.json"
commandCase = "upper"
"#,
        );

        assert_eq!(
            result.config.schema,
            Some("https://example.com/cmakefmt-schema.json".to_string())
        );
        assert_eq!(result.config.command_case, CaseStyle::Upper);
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
        assert_eq!(result.config.align_property_values, true);
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
    fn loads_toml_configuration_from_path() {
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/formatter/15_config_meta/01_schema/schema_ignored/.cmakefmt.toml");

        let result = load_from_toml_path(&config_path);

        assert_eq!(
            result.config.schema,
            Some("https://example.com/cmakefmt-schema.json".to_string())
        );
        assert_eq!(result.config.command_case, CaseStyle::Upper);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn applies_toml_configuration_fixture_to_formatting() {
        let fixture_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/formatter/15_config_meta/01_schema/schema_ignored");
        let config_path = fixture_dir.join(".cmakefmt.toml");
        let input_path = fixture_dir.join("schema_ignored.in.cmake");
        let expected_path = fixture_dir.join("schema_ignored.out.cmake");

        let load_result = load_from_toml_path(&config_path);
        assert!(
            load_result.diagnostics.is_empty(),
            "unexpected config diagnostics: {:?}",
            load_result.diagnostics
        );

        let input = std::fs::read_to_string(&input_path).expect("failed to read fixture input");
        let expected =
            std::fs::read_to_string(&expected_path).expect("failed to read fixture output");
        let formatted = crate::format_text::format_text(
            std::path::Path::new("CMakeLists.txt"),
            &input,
            &load_result.config,
        )
        .expect("formatting failed");

        assert_eq!(formatted.as_deref(), Some(expected.as_str()));
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
}

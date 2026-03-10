use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Configuration {
    pub line_width: u32,
    pub wrap_style: WrapStyle,
    pub first_arg_same_line: bool,
    pub wrap_arg_threshold: u32,
    pub magic_trailing_newline: bool,

    pub indent_width: u8,
    pub use_tabs: bool,
    pub indent_style: IndentStyle,
    pub continuation_indent_width: Option<u8>,
    pub genex_indent_width: Option<u8>,

    pub new_line_kind: NewLineKind,
    pub final_newline: bool,

    pub max_blank_lines: u8,
    pub min_blank_lines_between_blocks: u8,
    pub blank_line_between_sections: bool,

    pub command_case: CaseStyle,
    pub keyword_case: CaseStyle,
    pub custom_keywords: Vec<String>,
    pub literal_case: CaseStyle,

    pub closing_paren_newline: bool,
    pub space_before_paren: SpaceBeforeParen,
    pub space_inside_paren: SpaceInsideParen,

    pub comment_preservation: CommentPreservation,
    pub comment_width: Option<u32>,
    pub align_trailing_comments: bool,
    pub comment_gap: u8,

    pub trim_trailing_whitespace: bool,
    pub collapse_spaces: bool,

    pub align_property_values: bool,
    pub align_consecutive_set: bool,
    pub align_arg_groups: bool,

    pub genex_wrap: GenexWrap,
    pub genex_closing_angle_newline: bool,

    pub per_command_config: BTreeMap<String, CommandConfiguration>,

    pub sort_arguments: SortArguments,
    pub sort_keyword_sections: bool,

    pub disable_formatting: bool,
    pub ignore_patterns: Vec<String>,
    pub ignore_commands: Vec<String>,

    pub indent_block_body: bool,
    pub end_command_args: EndCommandArgs,

    pub schema: Option<String>,
    pub extends: Option<String>,

    /// Legacy compatibility field used by existing sorting logic.
    pub sort_lists: bool,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct CommandConfiguration {
    pub line_width: Option<u32>,
    pub wrap_style: Option<WrapStyle>,
    pub first_arg_same_line: Option<bool>,
    pub wrap_arg_threshold: Option<u32>,
    pub magic_trailing_newline: Option<bool>,

    pub indent_width: Option<u8>,
    pub indent_style: Option<IndentStyle>,
    pub continuation_indent_width: Option<u8>,
    pub genex_indent_width: Option<u8>,

    pub command_case: Option<CaseStyle>,
    pub keyword_case: Option<CaseStyle>,
    pub custom_keywords: Option<Vec<String>>,
    pub literal_case: Option<CaseStyle>,

    pub closing_paren_newline: Option<bool>,
    pub space_before_paren: Option<SpaceBeforeParen>,
    pub space_inside_paren: Option<SpaceInsideParen>,

    pub comment_preservation: Option<CommentPreservation>,
    pub comment_width: Option<u32>,
    pub align_trailing_comments: Option<bool>,
    pub comment_gap: Option<u8>,

    pub align_property_values: Option<bool>,
    pub align_consecutive_set: Option<bool>,
    pub align_arg_groups: Option<bool>,

    pub genex_wrap: Option<GenexWrap>,
    pub genex_closing_angle_newline: Option<bool>,

    pub sort_arguments: Option<SortArguments>,
    pub sort_keyword_sections: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
pub enum NewLineKind {
    Auto,
    Lf,
    CrLf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
pub enum CaseStyle {
    Lower,
    Upper,
    #[value(name = "unchanged", alias = "preserve")]
    Preserve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WrapStyle {
    Cascade,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IndentStyle {
    Space,
    Tab,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub enum SpaceBeforeParen {
    #[default]
    Never,
    Always,
    CommandList(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SpaceInsideParen {
    Insert,
    Remove,
    Preserve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentPreservation {
    Preserve,
    Reflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GenexWrap {
    Cascade,
    Never,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub enum SortArguments {
    #[default]
    Disabled,
    Enabled,
    CommandList(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EndCommandArgs {
    Remove,
    Preserve,
    Match,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            line_width: 80,
            wrap_style: WrapStyle::Cascade,
            first_arg_same_line: true,
            wrap_arg_threshold: 0,
            magic_trailing_newline: true,
            indent_width: 2,
            use_tabs: false,
            indent_style: IndentStyle::Space,
            continuation_indent_width: None,
            genex_indent_width: None,
            new_line_kind: NewLineKind::Auto,
            final_newline: true,
            max_blank_lines: 1,
            min_blank_lines_between_blocks: 0,
            blank_line_between_sections: false,
            command_case: CaseStyle::Lower,
            keyword_case: CaseStyle::Upper,
            custom_keywords: Vec::new(),
            literal_case: CaseStyle::Preserve,
            closing_paren_newline: true,
            space_before_paren: SpaceBeforeParen::Never,
            space_inside_paren: SpaceInsideParen::Remove,
            comment_preservation: CommentPreservation::Preserve,
            comment_width: None,
            align_trailing_comments: false,
            comment_gap: 1,
            trim_trailing_whitespace: true,
            collapse_spaces: true,
            align_property_values: false,
            align_consecutive_set: false,
            align_arg_groups: false,
            genex_wrap: GenexWrap::Cascade,
            genex_closing_angle_newline: true,
            per_command_config: BTreeMap::new(),
            sort_arguments: SortArguments::Disabled,
            sort_keyword_sections: false,
            disable_formatting: false,
            ignore_patterns: Vec::new(),
            ignore_commands: Vec::new(),
            indent_block_body: true,
            end_command_args: EndCommandArgs::Remove,
            schema: None,
            extends: None,
            sort_lists: false,
        }
    }
}

impl Configuration {
    /// Returns whether a command should include a space before the opening parenthesis.
    pub fn has_space_before_paren(&self, command_name: &str) -> bool {
        match &self.space_before_paren {
            SpaceBeforeParen::Never => false,
            SpaceBeforeParen::Always => true,
            SpaceBeforeParen::CommandList(command_names) => command_names
                .iter()
                .any(|configured_name| configured_name.eq_ignore_ascii_case(command_name)),
        }
    }

    /// Returns whether arguments should be sorted for the provided command.
    pub fn should_sort_arguments_for(&self, command_name: &str) -> bool {
        if self.sort_lists {
            return true;
        }

        match &self.sort_arguments {
            SortArguments::Disabled => false,
            SortArguments::Enabled => true,
            SortArguments::CommandList(command_names) => command_names
                .iter()
                .any(|configured_name| configured_name.eq_ignore_ascii_case(command_name)),
        }
    }

    /// Returns the effective continuation indent width, inheriting from `indentWidth` when unset.
    pub fn effective_continuation_indent_width(&self) -> u8 {
        self.continuation_indent_width.unwrap_or(self.indent_width)
    }

    /// Returns the effective generator-expression indent width, inheriting from `indentWidth` when unset.
    pub fn effective_genex_indent_width(&self) -> u8 {
        self.genex_indent_width.unwrap_or(self.indent_width)
    }

    /// Returns the effective comment width, inheriting from `lineWidth` when unset.
    pub fn effective_comment_width(&self) -> u32 {
        self.comment_width.unwrap_or(self.line_width)
    }

    /// Returns an effective configuration for a specific command, with per-command
    /// overrides applied on top of the global configuration.
    ///
    /// Per-command entries override the global config field-by-field: any `Some` value
    /// in the `CommandConfiguration` replaces the corresponding global value.
    pub fn effective_config_for_command(&self, command_name: &str) -> Configuration {
        let key = command_name.to_ascii_lowercase();
        let overrides = match self.per_command_config.get(&key) {
            Some(cmd_cfg) => cmd_cfg,
            None => return self.clone(),
        };

        let mut cfg = self.clone();
        if let Some(v) = overrides.line_width {
            cfg.line_width = v;
        }
        if let Some(v) = overrides.wrap_style {
            cfg.wrap_style = v;
        }
        if let Some(v) = overrides.first_arg_same_line {
            cfg.first_arg_same_line = v;
        }
        if let Some(v) = overrides.wrap_arg_threshold {
            cfg.wrap_arg_threshold = v;
        }
        if let Some(v) = overrides.magic_trailing_newline {
            cfg.magic_trailing_newline = v;
        }
        if let Some(v) = overrides.indent_width {
            cfg.indent_width = v;
            cfg.use_tabs = matches!(overrides.indent_style, Some(IndentStyle::Tab));
        }
        if let Some(v) = overrides.indent_style {
            cfg.indent_style = v;
            cfg.use_tabs = matches!(v, IndentStyle::Tab);
        }
        if let Some(v) = overrides.continuation_indent_width {
            cfg.continuation_indent_width = Some(v);
        }
        if let Some(v) = overrides.genex_indent_width {
            cfg.genex_indent_width = Some(v);
        }
        if let Some(v) = overrides.command_case {
            cfg.command_case = v;
        }
        if let Some(v) = overrides.keyword_case {
            cfg.keyword_case = v;
        }
        if let Some(ref v) = overrides.custom_keywords {
            cfg.custom_keywords = v.clone();
        }
        if let Some(v) = overrides.literal_case {
            cfg.literal_case = v;
        }
        if let Some(v) = overrides.closing_paren_newline {
            cfg.closing_paren_newline = v;
        }
        if let Some(ref v) = overrides.space_before_paren {
            cfg.space_before_paren = v.clone();
        }
        if let Some(v) = overrides.space_inside_paren {
            cfg.space_inside_paren = v;
        }
        if let Some(v) = overrides.comment_preservation {
            cfg.comment_preservation = v;
        }
        if let Some(v) = overrides.comment_width {
            cfg.comment_width = Some(v);
        }
        if let Some(v) = overrides.align_trailing_comments {
            cfg.align_trailing_comments = v;
        }
        if let Some(v) = overrides.comment_gap {
            cfg.comment_gap = v;
        }
        if let Some(v) = overrides.align_property_values {
            cfg.align_property_values = v;
        }
        if let Some(v) = overrides.align_consecutive_set {
            cfg.align_consecutive_set = v;
        }
        if let Some(v) = overrides.align_arg_groups {
            cfg.align_arg_groups = v;
        }
        if let Some(v) = overrides.genex_wrap {
            cfg.genex_wrap = v;
        }
        if let Some(v) = overrides.genex_closing_angle_newline {
            cfg.genex_closing_angle_newline = v;
        }
        if let Some(ref v) = overrides.sort_arguments {
            cfg.sort_arguments = v.clone();
        }
        if let Some(v) = overrides.sort_keyword_sections {
            cfg.sort_keyword_sections = v;
        }
        cfg
    }
}

impl fmt::Display for NewLineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NewLineKind::Auto => write!(f, "auto"),
            NewLineKind::Lf => write!(f, "lf"),
            NewLineKind::CrLf => write!(f, "crlf"),
        }
    }
}

impl fmt::Display for CaseStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaseStyle::Lower => write!(f, "lower"),
            CaseStyle::Upper => write!(f, "upper"),
            CaseStyle::Preserve => write!(f, "unchanged"),
        }
    }
}

impl fmt::Display for WrapStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WrapStyle::Cascade => write!(f, "cascade"),
            WrapStyle::Vertical => write!(f, "vertical"),
        }
    }
}

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndentStyle::Space => write!(f, "space"),
            IndentStyle::Tab => write!(f, "tab"),
        }
    }
}

impl fmt::Display for SpaceInsideParen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpaceInsideParen::Insert => write!(f, "insert"),
            SpaceInsideParen::Remove => write!(f, "remove"),
            SpaceInsideParen::Preserve => write!(f, "preserve"),
        }
    }
}

impl fmt::Display for CommentPreservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentPreservation::Preserve => write!(f, "preserve"),
            CommentPreservation::Reflow => write!(f, "reflow"),
        }
    }
}

impl fmt::Display for GenexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenexWrap::Cascade => write!(f, "cascade"),
            GenexWrap::Never => write!(f, "never"),
        }
    }
}

impl fmt::Display for EndCommandArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EndCommandArgs::Remove => write!(f, "remove"),
            EndCommandArgs::Preserve => write!(f, "preserve"),
            EndCommandArgs::Match => write!(f, "match"),
        }
    }
}

impl FromStr for NewLineKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "auto" => Ok(NewLineKind::Auto),
            "lf" => Ok(NewLineKind::Lf),
            "crlf" => Ok(NewLineKind::CrLf),
            _ => Err(format!(
                "Invalid lineEnding/newLineKind value '{s}'. Expected one of: auto, lf, crlf"
            )),
        }
    }
}

impl FromStr for CaseStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "lower" => Ok(CaseStyle::Lower),
            "upper" => Ok(CaseStyle::Upper),
            "unchanged" | "preserve" => Ok(CaseStyle::Preserve),
            _ => Err(format!(
                "Invalid value '{s}'. Expected one of: lower, upper, unchanged"
            )),
        }
    }
}

impl FromStr for WrapStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cascade" => Ok(WrapStyle::Cascade),
            "vertical" => Ok(WrapStyle::Vertical),
            _ => Err(format!(
                "Invalid wrapStyle value '{s}'. Expected one of: cascade, vertical"
            )),
        }
    }
}

impl FromStr for IndentStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "space" | "spaces" => Ok(IndentStyle::Space),
            "tab" | "tabs" => Ok(IndentStyle::Tab),
            _ => Err(format!(
                "Invalid indentStyle value '{s}'. Expected one of: space, tab"
            )),
        }
    }
}

impl FromStr for SpaceInsideParen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "insert" => Ok(SpaceInsideParen::Insert),
            "remove" => Ok(SpaceInsideParen::Remove),
            "preserve" => Ok(SpaceInsideParen::Preserve),
            _ => Err(format!(
                "Invalid spaceInsideParen value '{s}'. Expected one of: insert, remove, preserve"
            )),
        }
    }
}

impl FromStr for CommentPreservation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "preserve" => Ok(CommentPreservation::Preserve),
            "reflow" => Ok(CommentPreservation::Reflow),
            _ => Err(format!(
                "Invalid commentPreservation value '{s}'. Expected one of: preserve, reflow"
            )),
        }
    }
}

impl FromStr for GenexWrap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cascade" => Ok(GenexWrap::Cascade),
            "never" => Ok(GenexWrap::Never),
            _ => Err(format!(
                "Invalid genexWrap value '{s}'. Expected one of: cascade, never"
            )),
        }
    }
}

impl FromStr for EndCommandArgs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "remove" => Ok(EndCommandArgs::Remove),
            "preserve" => Ok(EndCommandArgs::Preserve),
            "match" => Ok(EndCommandArgs::Match),
            _ => Err(format!(
                "Invalid endCommandArgs value '{s}'. Expected one of: remove, preserve, match"
            )),
        }
    }
}

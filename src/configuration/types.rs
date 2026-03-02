use std::fmt;
use std::str::FromStr;

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u8,
    pub use_tabs: bool,
    pub new_line_kind: NewLineKind,
    pub command_case: CaseStyle,
    pub keyword_case: CaseStyle,
    pub closing_paren_newline: bool,
    pub sort_lists: bool,
    pub max_blank_lines: u8,
    pub space_before_paren: bool,
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
    Preserve,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            line_width: 80,
            indent_width: 2,
            use_tabs: false,
            new_line_kind: NewLineKind::Auto,
            command_case: CaseStyle::Lower,
            keyword_case: CaseStyle::Upper,
            closing_paren_newline: true,
            sort_lists: false,
            max_blank_lines: 1,
            space_before_paren: false,
        }
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
            CaseStyle::Preserve => write!(f, "preserve"),
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
            _ => Err(format!("Invalid newLineKind value '{s}'. Expected one of: auto, lf, crlf")),
        }
    }
}

impl FromStr for CaseStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "lower" => Ok(CaseStyle::Lower),
            "upper" => Ok(CaseStyle::Upper),
            "preserve" => Ok(CaseStyle::Preserve),
            _ => Err(format!("Invalid value '{s}'. Expected one of: lower, upper, preserve")),
        }
    }
}

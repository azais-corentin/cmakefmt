mod load;
mod types;

pub use load::{
    ConfigDiagnostic, ConfigDiagnosticSeverity, ConfigLoadResult, apply_inline_overrides,
    load_from_cli, load_from_header, load_from_json_map, load_from_toml, load_from_toml_path,
};
pub use types::{
    CaseStyle, CommandConfiguration, CommentPreservation, Configuration, EndCommandArgs, GenexWrap,
    IndentStyle, NewLineKind, SortArguments, SpaceBeforeParen, SpaceInsideParen, WrapStyle,
};

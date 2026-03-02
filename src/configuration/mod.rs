mod resolve;
mod load;
mod types;

pub use resolve::resolve_config;
pub use load::{
    ConfigDiagnostic, ConfigDiagnosticSeverity, ConfigLoadResult, load_from_cli, load_from_dprint,
    load_from_header,
};
pub use types::{CaseStyle, Configuration, NewLineKind};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KwType {
    Option,
    OneValue,
    MultiValue,
    /// A group keyword that starts a nested sub-section with its own sub-keywords.
    /// (front_positional_count, sub_keywords)
    Group(usize, &'static [(&'static str, KwType)]),
}

/// Section definition: (keyword_name, front_positional_count, sub_keywords).
pub type SectionDef = (&'static str, usize, &'static [(&'static str, KwType)]);
#[allow(dead_code)]
pub struct CommandSpec {
    pub front_positional: usize,
    pub back_positional: usize,
    pub keywords: &'static [(&'static str, KwType)],
    pub sections: &'static [SectionDef],
    pub command_line_keywords: &'static [&'static str],
    pub pair_keywords: &'static [&'static str],
    /// Property keywords: first value is the property name (at one indent),
    /// remaining values are property values (at two indents from keyword).
    pub property_keywords: &'static [&'static str],
    pub flow_keywords: &'static [&'static str],
    pub flow_positional: bool,
    /// Compound keywords: when keyword A is immediately followed by value B,
    /// they are treated as a single keyword unit "A B" with subsequent args
    /// as its values.
    pub compound_keywords: &'static [(&'static str, &'static str)],
    /// Once keywords: keywords that should only match once in the argument list.
    /// After their first occurrence, subsequent case-insensitive matches are
    /// treated as regular arguments (not uppercased, not keyword-split).
    pub once_keywords: &'static [&'static str],
}

pub enum CommandKind {
    Known(&'static CommandSpec),
    ConditionSyntax,
}

// ---------------------------------------------------------------------------
// Helper macro to reduce boilerplate
// ---------------------------------------------------------------------------

macro_rules! spec {
    (
        front: $fp:expr, back: $bp:expr,
        kw: $kw:expr,
        sections: $sec:expr,
        cmd_line: $cl:expr,
        pair: $pair:expr,
        flow: $flow:expr $(,)?
    ) => {
        CommandSpec {
            front_positional: $fp,
            back_positional: $bp,
            keywords: $kw,
            sections: $sec,
            command_line_keywords: $cl,
            pair_keywords: $pair,
            flow_keywords: $flow,
            flow_positional: false,
            compound_keywords: &[],
            once_keywords: &[],
            property_keywords: &[],
        }
    };
    (
        front: $fp:expr, back: $bp:expr,
        kw: $kw:expr,
        sections: $sec:expr,
        cmd_line: $cl:expr,
        pair: $pair:expr,
        flow: $flow:expr,
        flow_positional: $fp2:expr $(,)?
    ) => {
        CommandSpec {
            front_positional: $fp,
            back_positional: $bp,
            keywords: $kw,
            sections: $sec,
            command_line_keywords: $cl,
            pair_keywords: $pair,
            flow_keywords: $flow,
            flow_positional: $fp2,
            compound_keywords: &[],
            once_keywords: &[],
            property_keywords: &[],
        }
    };
    (
        front: $fp:expr, back: $bp:expr,
        kw: $kw:expr,
        sections: $sec:expr,
        cmd_line: $cl:expr,
        pair: $pair:expr $(,)?
    ) => {
        CommandSpec {
            front_positional: $fp,
            back_positional: $bp,
            keywords: $kw,
            sections: $sec,
            command_line_keywords: $cl,
            pair_keywords: $pair,
            flow_keywords: &[],
            flow_positional: false,
            compound_keywords: &[],
            once_keywords: &[],
            property_keywords: &[],
        }
    };
}

// ---------------------------------------------------------------------------
// 1. set
// ---------------------------------------------------------------------------

static SET_KW: &[(&str, KwType)] = &[
    ("PARENT_SCOPE", KwType::Option),
    ("FORCE", KwType::Option),
    ("TYPE", KwType::OneValue),
    ("HELP", KwType::OneValue),
    ("VALUE", KwType::MultiValue),
];

static SET_SECTIONS: &[SectionDef] = &[("CACHE", 1, &[])];

static SET_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: SET_KW, sections: SET_SECTIONS, cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 2. target_link_libraries
// ---------------------------------------------------------------------------

static TARGET_LINK_LIBRARIES_KW: &[(&str, KwType)] = &[
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
    ("LINK_PRIVATE", KwType::MultiValue),
    ("LINK_PUBLIC", KwType::MultiValue),
    ("LINK_INTERFACE_LIBRARIES", KwType::MultiValue),
];

static TLL_SEC_SUB: &[(&str, KwType)] = &[
    ("debug", KwType::OneValue),
    ("optimized", KwType::OneValue),
    ("general", KwType::OneValue),
];

static TARGET_LINK_LIBRARIES_SECTIONS: &[SectionDef] = &[
    ("PUBLIC", 0, TLL_SEC_SUB),
    ("PRIVATE", 0, TLL_SEC_SUB),
    ("INTERFACE", 0, TLL_SEC_SUB),
    ("LINK_PUBLIC", 0, TLL_SEC_SUB),
    ("LINK_PRIVATE", 0, TLL_SEC_SUB),
    ("LINK_INTERFACE_LIBRARIES", 0, TLL_SEC_SUB),
];

static TARGET_LINK_LIBRARIES_SPEC: CommandSpec = spec! {
    front: 1, back: 0,
    kw: TARGET_LINK_LIBRARIES_KW,
    sections: TARGET_LINK_LIBRARIES_SECTIONS,
    cmd_line: &["debug", "optimized", "general"], pair: &[],
};

// ---------------------------------------------------------------------------
// 3. target_sources
// ---------------------------------------------------------------------------

static TARGET_SOURCES_KW: &[(&str, KwType)] = &[
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
    (
        "FILE_SET",
        KwType::Group(1, TARGET_SOURCES_FILE_SET_GROUP_KW),
    ),
];

static TARGET_SOURCES_FILE_SET_GROUP_KW: &[(&str, KwType)] = &[
    ("TYPE", KwType::OneValue),
    ("BASE_DIRS", KwType::MultiValue),
    ("FILES", KwType::MultiValue),
];

static TARGET_SOURCES_FILE_SET_SUB: &[(&str, KwType)] = &[(
    "FILE_SET",
    KwType::Group(1, TARGET_SOURCES_FILE_SET_GROUP_KW),
)];

static TARGET_SOURCES_SECTIONS: &[SectionDef] = &[
    ("PUBLIC", 0, TARGET_SOURCES_FILE_SET_SUB),
    ("PRIVATE", 0, TARGET_SOURCES_FILE_SET_SUB),
    ("INTERFACE", 0, TARGET_SOURCES_FILE_SET_SUB),
];

static TARGET_SOURCES_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_SOURCES_KW, sections: TARGET_SOURCES_SECTIONS, cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 4. target_compile_definitions
// ---------------------------------------------------------------------------

static TARGET_COMPILE_DEFINITIONS_KW: &[(&str, KwType)] = &[
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_COMPILE_DEFINITIONS_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_COMPILE_DEFINITIONS_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 5. target_compile_options
// ---------------------------------------------------------------------------

static TARGET_COMPILE_OPTIONS_KW: &[(&str, KwType)] = &[
    ("BEFORE", KwType::Option),
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_COMPILE_OPTIONS_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_COMPILE_OPTIONS_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 6. target_compile_features
// ---------------------------------------------------------------------------

static TARGET_COMPILE_FEATURES_KW: &[(&str, KwType)] = &[
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_COMPILE_FEATURES_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_COMPILE_FEATURES_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 7. target_include_directories
// ---------------------------------------------------------------------------

static TARGET_INCLUDE_DIRECTORIES_KW: &[(&str, KwType)] = &[
    ("BEFORE", KwType::Option),
    ("SYSTEM", KwType::Option),
    ("AFTER", KwType::Option),
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_INCLUDE_DIRECTORIES_SPEC: CommandSpec = spec! { front: 1, back: 0, kw: TARGET_INCLUDE_DIRECTORIES_KW, sections: &[], cmd_line: &[], pair: &[], };

// ---------------------------------------------------------------------------
// 8. target_link_directories
// ---------------------------------------------------------------------------

static TARGET_LINK_DIRECTORIES_KW: &[(&str, KwType)] = &[
    ("BEFORE", KwType::Option),
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_LINK_DIRECTORIES_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_LINK_DIRECTORIES_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 9. target_link_options
// ---------------------------------------------------------------------------

static TARGET_LINK_OPTIONS_KW: &[(&str, KwType)] = &[
    ("BEFORE", KwType::Option),
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_LINK_OPTIONS_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_LINK_OPTIONS_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 10. target_precompile_headers
// ---------------------------------------------------------------------------

static TARGET_PRECOMPILE_HEADERS_KW: &[(&str, KwType)] = &[
    ("REUSE_FROM", KwType::OneValue),
    ("INTERFACE", KwType::MultiValue),
    ("PUBLIC", KwType::MultiValue),
    ("PRIVATE", KwType::MultiValue),
];

static TARGET_PRECOMPILE_HEADERS_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: TARGET_PRECOMPILE_HEADERS_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 11. add_executable
// ---------------------------------------------------------------------------

static ADD_EXECUTABLE_KW: &[(&str, KwType)] = &[
    ("WIN32", KwType::Option),
    ("MACOSX_BUNDLE", KwType::Option),
    ("EXCLUDE_FROM_ALL", KwType::Option),
    ("IMPORTED", KwType::MultiValue),
    ("ALIAS", KwType::OneValue),
];

static ADD_EXECUTABLE_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: ADD_EXECUTABLE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 12. add_library
// ---------------------------------------------------------------------------

static ADD_LIBRARY_KW: &[(&str, KwType)] = &[
    ("STATIC", KwType::Option),
    ("SHARED", KwType::Option),
    ("MODULE", KwType::Option),
    ("EXCLUDE_FROM_ALL", KwType::Option),
    ("OBJECT", KwType::Option),
    ("IMPORTED", KwType::MultiValue),
    ("UNKNOWN", KwType::Option),
    ("INTERFACE", KwType::Option),
    ("ALIAS", KwType::OneValue),
];

static ADD_LIBRARY_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: ADD_LIBRARY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 13. add_test
// ---------------------------------------------------------------------------

static ADD_TEST_KW: &[(&str, KwType)] = &[
    ("COMMAND_EXPAND_LISTS", KwType::Option),
    ("NAME", KwType::OneValue),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("COMMAND", KwType::MultiValue),
    ("CONFIGURATIONS", KwType::MultiValue),
];

static ADD_TEST_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: ADD_TEST_KW, sections: &[], cmd_line: &[], pair: &[], flow: &["COMMAND"],
};

// ---------------------------------------------------------------------------
// 14. project
// ---------------------------------------------------------------------------

static PROJECT_KW: &[(&str, KwType)] = &[
    ("VERSION", KwType::OneValue),
    ("DESCRIPTION", KwType::OneValue),
    ("HOMEPAGE_URL", KwType::OneValue),
    ("COMPAT_VERSION", KwType::OneValue),
    ("LANGUAGES", KwType::MultiValue),
];

static PROJECT_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: PROJECT_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 15. message
// ---------------------------------------------------------------------------

static MESSAGE_KW: &[(&str, KwType)] = &[
    ("FATAL_ERROR", KwType::Option),
    ("SEND_ERROR", KwType::Option),
    ("WARNING", KwType::Option),
    ("AUTHOR_WARNING", KwType::Option),
    ("DEPRECATION", KwType::Option),
    ("NOTICE", KwType::Option),
    ("STATUS", KwType::Option),
    ("VERBOSE", KwType::Option),
    ("DEBUG", KwType::Option),
    ("TRACE", KwType::Option),
    ("CHECK_START", KwType::Option),
    ("CHECK_PASS", KwType::Option),
    ("CHECK_FAIL", KwType::Option),
    ("CONFIGURE_LOG", KwType::Option),
];

static MESSAGE_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: MESSAGE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 16. find_package
// ---------------------------------------------------------------------------

static FIND_PACKAGE_KW: &[(&str, KwType)] = &[
    ("EXACT", KwType::Option),
    ("QUIET", KwType::Option),
    ("MODULE", KwType::Option),
    ("CONFIG", KwType::Option),
    ("NO_MODULE", KwType::Option),
    ("NO_POLICY_SCOPE", KwType::Option),
    ("NO_DEFAULT_PATH", KwType::Option),
    ("NO_PACKAGE_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_PATH", KwType::Option),
    ("NO_CMAKE_ENVIRONMENT_PATH", KwType::Option),
    ("NO_SYSTEM_ENVIRONMENT_PATH", KwType::Option),
    ("NO_CMAKE_PACKAGE_REGISTRY", KwType::Option),
    ("NO_CMAKE_BUILDS_PATH", KwType::Option),
    ("NO_CMAKE_SYSTEM_PATH", KwType::Option),
    ("NO_CMAKE_SYSTEM_PACKAGE_REGISTRY", KwType::Option),
    ("CMAKE_FIND_ROOT_PATH_BOTH", KwType::Option),
    ("ONLY_CMAKE_FIND_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_FIND_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_INSTALL_PREFIX", KwType::Option),
    ("GLOBAL", KwType::Option),
    ("COMPONENTS", KwType::MultiValue),
    ("OPTIONAL_COMPONENTS", KwType::MultiValue),
    ("NAMES", KwType::MultiValue),
    ("CONFIGS", KwType::MultiValue),
    ("HINTS", KwType::MultiValue),
    ("PATHS", KwType::MultiValue),
    ("PATH_SUFFIXES", KwType::MultiValue),
    ("REQUIRED", KwType::MultiValue),
];

static FIND_PACKAGE_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: FIND_PACKAGE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 17. find_library
// ---------------------------------------------------------------------------

static FIND_LIBRARY_KW: &[(&str, KwType)] = &[
    ("NAMES_PER_DIR", KwType::Option),
    ("NO_DEFAULT_PATH", KwType::Option),
    ("NO_PACKAGE_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_PATH", KwType::Option),
    ("NO_CMAKE_ENVIRONMENT_PATH", KwType::Option),
    ("NO_SYSTEM_ENVIRONMENT_PATH", KwType::Option),
    ("NO_CMAKE_SYSTEM_PATH", KwType::Option),
    ("CMAKE_FIND_ROOT_PATH_BOTH", KwType::Option),
    ("ONLY_CMAKE_FIND_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_FIND_ROOT_PATH", KwType::Option),
    ("REQUIRED", KwType::Option),
    ("NO_CMAKE_INSTALL_PREFIX", KwType::Option),
    ("DOC", KwType::OneValue),
    ("ENV", KwType::OneValue),
    ("VALIDATOR", KwType::OneValue),
    ("NAMES", KwType::MultiValue),
    ("HINTS", KwType::MultiValue),
    ("PATHS", KwType::MultiValue),
    ("PATH_SUFFIXES", KwType::MultiValue),
];

static FIND_LIBRARY_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: FIND_LIBRARY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 18. find_file (same as find_library minus NAMES_PER_DIR)
// ---------------------------------------------------------------------------

static FIND_FILE_KW: &[(&str, KwType)] = &[
    ("NO_DEFAULT_PATH", KwType::Option),
    ("NO_PACKAGE_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_PATH", KwType::Option),
    ("NO_CMAKE_ENVIRONMENT_PATH", KwType::Option),
    ("NO_SYSTEM_ENVIRONMENT_PATH", KwType::Option),
    ("NO_CMAKE_SYSTEM_PATH", KwType::Option),
    ("CMAKE_FIND_ROOT_PATH_BOTH", KwType::Option),
    ("ONLY_CMAKE_FIND_ROOT_PATH", KwType::Option),
    ("NO_CMAKE_FIND_ROOT_PATH", KwType::Option),
    ("REQUIRED", KwType::Option),
    ("NO_CMAKE_INSTALL_PREFIX", KwType::Option),
    ("DOC", KwType::OneValue),
    ("ENV", KwType::OneValue),
    ("VALIDATOR", KwType::OneValue),
    ("NAMES", KwType::MultiValue),
    ("HINTS", KwType::MultiValue),
    ("PATHS", KwType::MultiValue),
    ("PATH_SUFFIXES", KwType::MultiValue),
];

static FIND_FILE_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: FIND_FILE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 19. find_path (same as find_file)
// ---------------------------------------------------------------------------

static FIND_PATH_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: FIND_FILE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 20. find_program (same as find_library)
// ---------------------------------------------------------------------------

static FIND_PROGRAM_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: FIND_LIBRARY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 21. execute_process
// ---------------------------------------------------------------------------

static EXECUTE_PROCESS_KW: &[(&str, KwType)] = &[
    ("OUTPUT_QUIET", KwType::Option),
    ("ERROR_QUIET", KwType::Option),
    ("OUTPUT_STRIP_TRAILING_WHITESPACE", KwType::Option),
    ("ERROR_STRIP_TRAILING_WHITESPACE", KwType::Option),
    ("ECHO_OUTPUT_VARIABLE", KwType::Option),
    ("ECHO_ERROR_VARIABLE", KwType::Option),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("TIMEOUT", KwType::OneValue),
    ("RESULT_VARIABLE", KwType::OneValue),
    ("RESULTS_VARIABLE", KwType::OneValue),
    ("OUTPUT_VARIABLE", KwType::OneValue),
    ("ERROR_VARIABLE", KwType::OneValue),
    ("INPUT_FILE", KwType::OneValue),
    ("OUTPUT_FILE", KwType::OneValue),
    ("ERROR_FILE", KwType::OneValue),
    ("COMMAND_ECHO", KwType::OneValue),
    ("ENCODING", KwType::OneValue),
    ("COMMAND_ERROR_IS_FATAL", KwType::OneValue),
    ("COMMAND", KwType::MultiValue),
];

static EXECUTE_PROCESS_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: EXECUTE_PROCESS_KW, sections: &[], cmd_line: &[], pair: &[], flow: &["COMMAND"],
};

// ---------------------------------------------------------------------------
// 22. cmake_parse_arguments
// ---------------------------------------------------------------------------

static CMAKE_PARSE_ARGUMENTS_KW: &[(&str, KwType)] = &[("PARSE_ARGV", KwType::OneValue)];

static CMAKE_PARSE_ARGUMENTS_SPEC: CommandSpec = spec! {
    front: 4, back: 0, kw: CMAKE_PARSE_ARGUMENTS_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 23. define_property
// ---------------------------------------------------------------------------

static DEFINE_PROPERTY_KW: &[(&str, KwType)] = &[
    ("GLOBAL", KwType::Option),
    ("DIRECTORY", KwType::Option),
    ("TARGET", KwType::Option),
    ("SOURCE", KwType::Option),
    ("TEST", KwType::Option),
    ("VARIABLE", KwType::Option),
    ("CACHED_VARIABLE", KwType::Option),
    ("INHERITED", KwType::Option),
    ("PROPERTY", KwType::OneValue),
    ("INITIALIZE_FROM_VARIABLE", KwType::OneValue),
    ("BRIEF_DOCS", KwType::MultiValue),
    ("FULL_DOCS", KwType::MultiValue),
];

static DEFINE_PROPERTY_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: DEFINE_PROPERTY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 24. get_property
// ---------------------------------------------------------------------------

static GET_PROPERTY_KW: &[(&str, KwType)] = &[
    ("GLOBAL", KwType::Option),
    ("VARIABLE", KwType::Option),
    ("SET", KwType::Option),
    ("DEFINED", KwType::Option),
    ("BRIEF_DOCS", KwType::Option),
    ("FULL_DOCS", KwType::Option),
    ("TARGET", KwType::OneValue),
    ("INSTALL", KwType::OneValue),
    ("TEST", KwType::OneValue),
    ("CACHE", KwType::OneValue),
    ("PROPERTY", KwType::OneValue),
    ("TARGET_DIRECTORY", KwType::OneValue),
    ("SOURCE", KwType::OneValue),
    ("DIRECTORY", KwType::MultiValue),
];

static GET_PROPERTY_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: GET_PROPERTY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 25. set_property
// ---------------------------------------------------------------------------

static SET_PROPERTY_KW: &[(&str, KwType)] = &[
    ("GLOBAL", KwType::Option),
    ("APPEND", KwType::Option),
    ("APPEND_STRING", KwType::Option),
    ("DIRECTORY", KwType::MultiValue),
    ("TARGET_DIRECTORY", KwType::OneValue),
    ("TARGET_DIRECTORIES", KwType::MultiValue),
    ("TARGET", KwType::MultiValue),
    ("SOURCE", KwType::MultiValue),
    ("INSTALL", KwType::MultiValue),
    ("TEST", KwType::MultiValue),
    ("CACHE", KwType::MultiValue),
    ("PROPERTY", KwType::MultiValue),
];

static SET_PROPERTY_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: SET_PROPERTY_KW,
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    property_keywords: &["PROPERTY"],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[],
    once_keywords: &[],
};

// ---------------------------------------------------------------------------
// 26. export
// ---------------------------------------------------------------------------

static EXPORT_KW: &[(&str, KwType)] = &[
    ("APPEND", KwType::Option),
    ("EXPORT_LINK_INTERFACE_LIBRARIES", KwType::Option),
    ("EXPORT_PACKAGE_DEPENDENCIES", KwType::Option),
    ("NO_PROJECT_METADATA", KwType::Option),
    ("LOWER_CASE_FILE", KwType::Option),
    ("EXPORT", KwType::OneValue),
    ("NAMESPACE", KwType::OneValue),
    ("FILE", KwType::OneValue),
    ("PACKAGE_INFO", KwType::OneValue),
    ("PROJECT", KwType::OneValue),
    ("APPENDIX", KwType::OneValue),
    ("LICENSE", KwType::OneValue),
    ("DEFAULT_LICENSE", KwType::OneValue),
    ("DESCRIPTION", KwType::OneValue),
    ("HOMEPAGE_URL", KwType::OneValue),
    ("PACKAGE", KwType::OneValue),
    ("SETUP", KwType::OneValue),
    ("ANDROID_MK", KwType::OneValue),
    ("TARGETS", KwType::MultiValue),
    ("DEFAULT_TARGETS", KwType::MultiValue),
    ("DEFAULT_CONFIGURATIONS", KwType::MultiValue),
    ("PACKAGE_DEPENDENCY", KwType::MultiValue),
    ("TARGET", KwType::MultiValue),
    ("VERSION", KwType::MultiValue),
];

static EXPORT_SECTIONS: &[SectionDef] = &[
    (
        "PACKAGE_DEPENDENCY",
        0,
        &[
            ("ENABLED", KwType::OneValue),
            ("EXTRA_ARGS", KwType::MultiValue),
        ],
    ),
    ("TARGET", 0, &[("XCFRAMEWORK_LOCATION", KwType::OneValue)]),
    (
        "VERSION",
        1,
        &[
            ("COMPAT_VERSION", KwType::OneValue),
            ("VERSION_SCHEMA", KwType::OneValue),
        ],
    ),
];

static EXPORT_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: EXPORT_KW, sections: EXPORT_SECTIONS, cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 27. foreach
// ---------------------------------------------------------------------------

static FOREACH_KW: &[(&str, KwType)] = &[
    ("IN", KwType::Option),
    ("RANGE", KwType::MultiValue),
    ("LISTS", KwType::MultiValue),
    ("ITEMS", KwType::MultiValue),
    ("ZIP_LISTS", KwType::MultiValue),
];

static FOREACH_SPEC: CommandSpec =
    spec! { front: 0, back: 0, kw: FOREACH_KW, sections: &[], cmd_line: &[], pair: &[], };

// ---------------------------------------------------------------------------
// 28. function
// ---------------------------------------------------------------------------

static FUNCTION_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: &[], sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 29. macro
// ---------------------------------------------------------------------------

static MACRO_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: &[], sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 30. cmake_minimum_required
// ---------------------------------------------------------------------------

static CMAKE_MINIMUM_REQUIRED_KW: &[(&str, KwType)] = &[
    ("FATAL_ERROR", KwType::Option),
    ("VERSION", KwType::OneValue),
];

static CMAKE_MINIMUM_REQUIRED_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CMAKE_MINIMUM_REQUIRED_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 31. configure_file
// ---------------------------------------------------------------------------

static CONFIGURE_FILE_KW: &[(&str, KwType)] = &[
    ("COPYONLY", KwType::Option),
    ("ESCAPE_QUOTES", KwType::Option),
    ("@ONLY", KwType::Option),
    ("NO_SOURCE_PERMISSIONS", KwType::Option),
    ("USE_SOURCE_PERMISSIONS", KwType::Option),
    ("NEWLINE_STYLE", KwType::OneValue),
    ("FILE_PERMISSIONS", KwType::MultiValue),
];

static CONFIGURE_FILE_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: CONFIGURE_FILE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 32. include
// ---------------------------------------------------------------------------

static INCLUDE_KW: &[(&str, KwType)] = &[
    ("OPTIONAL", KwType::Option),
    ("NO_POLICY_SCOPE", KwType::Option),
    ("RESULT_VARIABLE", KwType::OneValue),
];

static INCLUDE_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: INCLUDE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 33. math
// ---------------------------------------------------------------------------

static MATH_KW: &[(&str, KwType)] = &[("OUTPUT_FORMAT", KwType::OneValue)];

static MATH_SPEC: CommandSpec = spec! {
    front: 3, back: 0, kw: MATH_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 34. add_subdirectory
// ---------------------------------------------------------------------------

static ADD_SUBDIRECTORY_KW: &[(&str, KwType)] = &[
    ("EXCLUDE_FROM_ALL", KwType::Option),
    ("SYSTEM", KwType::Option),
];

static ADD_SUBDIRECTORY_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: ADD_SUBDIRECTORY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 35. string (merged)
// ---------------------------------------------------------------------------

static STRING_KW: &[(&str, KwType)] = &[
    ("REVERSE", KwType::Option),
    ("@ONLY", KwType::Option),
    ("ESCAPE_QUOTES", KwType::Option),
    ("UTC", KwType::Option),
    ("UPPER", KwType::Option),
    ("TOLOWER", KwType::OneValue),
    ("TOUPPER", KwType::OneValue),
    ("STRIP", KwType::OneValue),
    ("GENEX_STRIP", KwType::OneValue),
    ("INCLUDE", KwType::Option),
    ("EXCLUDE", KwType::Option),
    ("FIND", KwType::OneValue),
    ("REPLACE", KwType::OneValue),
    ("APPEND", KwType::OneValue),
    ("PREPEND", KwType::OneValue),
    ("CONCAT", KwType::OneValue),
    ("JOIN", KwType::OneValue),
    ("LENGTH", KwType::OneValue),
    ("SUBSTRING", KwType::OneValue),
    ("REPEAT", KwType::OneValue),
    ("MD5", KwType::OneValue),
    ("SHA1", KwType::OneValue),
    ("SHA224", KwType::OneValue),
    ("SHA256", KwType::OneValue),
    ("SHA384", KwType::OneValue),
    ("SHA512", KwType::OneValue),
    ("SHA3_224", KwType::OneValue),
    ("SHA3_256", KwType::OneValue),
    ("SHA3_384", KwType::OneValue),
    ("SHA3_512", KwType::OneValue),
    ("CONFIGURE", KwType::OneValue),
    ("MAKE_C_IDENTIFIER", KwType::OneValue),
    ("RANDOM", KwType::Option),
    ("TIMESTAMP", KwType::OneValue),
    ("UUID", KwType::OneValue),
    ("NAMESPACE", KwType::OneValue),
    ("NAME", KwType::OneValue),
    ("TYPE", KwType::OneValue),
    ("REGEX", KwType::OneValue),
    ("MATCH", KwType::OneValue),
    ("MATCHALL", KwType::OneValue),
    ("COMPARE", KwType::OneValue),
    ("LESS", KwType::OneValue),
    ("GREATER", KwType::OneValue),
    ("EQUAL", KwType::OneValue),
    ("NOTEQUAL", KwType::OneValue),
    ("LESS_EQUAL", KwType::OneValue),
    ("GREATER_EQUAL", KwType::OneValue),
    ("HEX", KwType::OneValue),
    ("ASCII", KwType::MultiValue),
    ("JSON", KwType::OneValue),
    ("ERROR_VARIABLE", KwType::OneValue),
    ("GET", KwType::OneValue),
    ("MEMBER", KwType::OneValue),
    ("REMOVE", KwType::OneValue),
    ("SET", KwType::OneValue),
    ("ALPHABET", KwType::OneValue),
    ("RANDOM_SEED", KwType::OneValue),
    ("OUTPUT_VARIABLE", KwType::OneValue),
    ("QUOTE", KwType::OneValue),
    ("FILTER", KwType::OneValue),
];

static STRING_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: STRING_KW,
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[
        ("REGEX", "MATCH"),
        ("REGEX", "MATCHALL"),
        ("REGEX", "REPLACE"),
        ("REGEX", "QUOTE"),
    ],
    once_keywords: &[],
    property_keywords: &[],
};

// ---------------------------------------------------------------------------
// 36. list (merged)
// ---------------------------------------------------------------------------

static LIST_KW: &[(&str, KwType)] = &[
    ("INCLUDE", KwType::Option),
    ("EXCLUDE", KwType::Option),
    ("TOLOWER", KwType::Option),
    ("TOUPPER", KwType::Option),
    ("STRIP", KwType::Option),
    ("GENEX_STRIP", KwType::Option),
    ("LENGTH", KwType::OneValue),
    ("GET", KwType::OneValue),
    ("JOIN", KwType::OneValue),
    ("SUBLIST", KwType::OneValue),
    ("FIND", KwType::OneValue),
    ("APPEND", KwType::OneValue),
    ("FILTER", KwType::OneValue),
    ("INSERT", KwType::OneValue),
    ("POP_BACK", KwType::OneValue),
    ("POP_FRONT", KwType::OneValue),
    ("PREPEND", KwType::OneValue),
    ("REMOVE_ITEM", KwType::OneValue),
    ("REMOVE_AT", KwType::OneValue),
    ("REMOVE_DUPLICATES", KwType::OneValue),
    ("TRANSFORM", KwType::OneValue),
    ("REVERSE", KwType::OneValue),
    ("SORT", KwType::OneValue),
    ("COMPARE", KwType::OneValue),
    ("CASE", KwType::OneValue),
    ("ORDER", KwType::OneValue),
    ("OUTPUT_VARIABLE", KwType::OneValue),
    ("REGEX", KwType::OneValue),
    ("REPLACE", KwType::MultiValue),
    ("AT", KwType::MultiValue),
    ("FOR", KwType::MultiValue),
];

static LIST_REPLACE_SEC_SUB: &[(&str, KwType)] = &[("REGEX", KwType::OneValue)];

static LIST_SECTIONS: &[SectionDef] = &[("REPLACE", 0, LIST_REPLACE_SEC_SUB)];

static LIST_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: LIST_KW, sections: LIST_SECTIONS, cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 37. file (merged)
// ---------------------------------------------------------------------------

static FILE_KW: &[(&str, KwType)] = &[
    // Options
    ("HEX", KwType::Option),
    ("NEWLINE_CONSUME", KwType::Option),
    ("NO_HEX_CONVERSION", KwType::Option),
    ("CONFIGURE_DEPENDS", KwType::Option),
    ("FOLLOW_SYMLINKS", KwType::Option),
    ("NO_SOURCE_PERMISSIONS", KwType::Option),
    ("USE_SOURCE_PERMISSIONS", KwType::Option),
    ("FOLLOW_SYMLINK_CHAIN", KwType::Option),
    ("FILES_MATCHING", KwType::Option),
    ("EXCLUDE", KwType::Option),
    ("NO_REPLACE", KwType::Option),
    ("ONLY_IF_DIFFERENT", KwType::Option),
    ("COPY_ON_ERROR", KwType::Option),
    ("SYMBOLIC", KwType::Option),
    ("VERBOSE", KwType::Option),
    ("EXPAND_TILDE", KwType::Option),
    ("LIST_ONLY", KwType::Option),
    ("SHOW_PROGRESS", KwType::Option),
    ("DIRECTORY", KwType::Option),
    ("RELEASE", KwType::Option),
    // OneValue
    ("READ", KwType::OneValue),
    ("STRINGS", KwType::OneValue),
    ("TIMESTAMP", KwType::OneValue),
    ("WRITE", KwType::OneValue),
    ("APPEND", KwType::OneValue),
    // OUTPUT removed from keywords to support GENERATE OUTPUT compound keyword
    ("INPUT", KwType::OneValue),
    ("CONTENT", KwType::OneValue),
    ("CONDITION", KwType::OneValue),
    ("NEWLINE_STYLE", KwType::OneValue),
    ("GLOB", KwType::OneValue),
    ("GLOB_RECURSE", KwType::OneValue),
    ("LIST_DIRECTORIES", KwType::OneValue),
    ("RELATIVE", KwType::OneValue),
    ("RENAME", KwType::OneValue),
    ("RESULT", KwType::OneValue),
    ("COPY_FILE", KwType::OneValue),
    ("SIZE", KwType::OneValue),
    ("READ_SYMLINK", KwType::OneValue),
    ("REAL_PATH", KwType::OneValue),
    ("RELATIVE_PATH", KwType::Option),
    ("TO_CMAKE_PATH", KwType::Option),
    ("TO_NATIVE_PATH", KwType::Option),
    ("DOWNLOAD", KwType::OneValue),
    ("UPLOAD", KwType::OneValue),
    ("LOCK", KwType::OneValue),
    ("GUARD", KwType::OneValue),
    ("RESULT_VARIABLE", KwType::OneValue),
    ("TIMEOUT", KwType::OneValue),
    ("ENCODING", KwType::OneValue),
    ("DESTINATION", KwType::OneValue),
    ("PATTERN", KwType::OneValue),
    ("REGEX", KwType::OneValue),
    ("OFFSET", KwType::OneValue),
    ("LIMIT", KwType::OneValue),
    ("LENGTH_MAXIMUM", KwType::OneValue),
    ("LENGTH_MINIMUM", KwType::OneValue),
    ("LIMIT_COUNT", KwType::OneValue),
    ("LIMIT_INPUT", KwType::OneValue),
    ("LIMIT_OUTPUT", KwType::OneValue),
    ("INACTIVITY_TIMEOUT", KwType::OneValue),
    ("LOG", KwType::OneValue),
    ("STATUS", KwType::OneValue),
    ("USERPWD", KwType::OneValue),
    ("HTTPHEADER", KwType::OneValue),
    ("NETRC", KwType::OneValue),
    ("NETRC_FILE", KwType::OneValue),
    ("EXPECTED_HASH", KwType::OneValue),
    ("EXPECTED_MD5", KwType::OneValue),
    ("TLS_VERIFY", KwType::OneValue),
    ("TLS_CAINFO", KwType::OneValue),
    ("RANGE_START", KwType::OneValue),
    ("RANGE_END", KwType::OneValue),
    ("TLS_VERSION", KwType::OneValue),
    // FORMAT removed from keywords to prevent collision with TIMESTAMP's format arg
    ("COMPRESSION", KwType::OneValue),
    ("COMPRESSION_LEVEL", KwType::OneValue),
    ("MTIME", KwType::OneValue),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("RESOLVED_DEPENDENCIES_VAR", KwType::OneValue),
    ("UNRESOLVED_DEPENDENCIES_VAR", KwType::OneValue),
    ("CONFLICTING_DEPENDENCIES_PREFIX", KwType::OneValue),
    ("BUNDLE_EXECUTABLE", KwType::OneValue),
    ("BASE_DIRECTORY", KwType::OneValue),
    ("TO_CMAKE_PATH_LIST", KwType::OneValue),
    ("TO_NATIVE_PATH_LIST", KwType::OneValue),
    // MultiValue
    ("COPY", KwType::MultiValue),
    ("INSTALL", KwType::MultiValue),
    ("FILE_PERMISSIONS", KwType::MultiValue),
    ("DIRECTORY_PERMISSIONS", KwType::MultiValue),
    ("PERMISSIONS", KwType::MultiValue),
    ("CREATE_LINK", KwType::MultiValue),
    ("CHMOD", KwType::MultiValue),
    ("CHMOD_RECURSE", KwType::MultiValue),
    ("MAKE_DIRECTORY", KwType::MultiValue),
    ("ARCHIVE_CREATE", KwType::MultiValue),
    ("ARCHIVE_EXTRACT", KwType::MultiValue),
    ("PATHS", KwType::MultiValue),
    ("PATTERNS", KwType::MultiValue),
    ("EXECUTABLES", KwType::MultiValue),
    ("LIBRARIES", KwType::MultiValue),
    ("MODULES", KwType::MultiValue),
    ("DIRECTORIES", KwType::MultiValue),
    ("PRE_INCLUDE_REGEXES", KwType::MultiValue),
    ("PRE_EXCLUDE_REGEXES", KwType::MultiValue),
    ("POST_INCLUDE_REGEXES", KwType::MultiValue),
    ("POST_EXCLUDE_REGEXES", KwType::MultiValue),
    ("POST_INCLUDE_FILES", KwType::MultiValue),
    ("POST_EXCLUDE_FILES", KwType::MultiValue),
    ("REMOVE", KwType::Option),
    ("REMOVE_RECURSE", KwType::Option),
    ("TOUCH", KwType::Option),
    ("TOUCH_NOCREATE", KwType::Option),
    ("GET_RUNTIME_DEPENDENCIES", KwType::MultiValue),
    ("MD5", KwType::OneValue),
    ("SHA1", KwType::OneValue),
    ("SHA224", KwType::OneValue),
    ("SHA256", KwType::OneValue),
    ("SHA384", KwType::OneValue),
    ("SHA512", KwType::OneValue),
    ("SHA3_224", KwType::OneValue),
    ("SHA3_256", KwType::OneValue),
    ("SHA3_384", KwType::OneValue),
    ("SHA3_512", KwType::OneValue),
    ("GENERATE", KwType::MultiValue),
];

static FILE_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: FILE_KW,
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[("GENERATE", "OUTPUT")],
    once_keywords: &["GLOB", "GLOB_RECURSE"],
    property_keywords: &[],
};

// ---------------------------------------------------------------------------
// 38. install (merged)
// ---------------------------------------------------------------------------

static INSTALL_KW: &[(&str, KwType)] = &[
    // Options
    ("OPTIONAL", KwType::Option),
    ("EXCLUDE_FROM_ALL", KwType::Option),
    ("NAMELINK_ONLY", KwType::Option),
    ("NAMELINK_SKIP", KwType::Option),
    ("USE_SOURCE_PERMISSIONS", KwType::Option),
    ("MESSAGE_NEVER", KwType::Option),
    ("FILES_MATCHING", KwType::Option),
    ("EXCLUDE_EMPTY_DIRECTORIES", KwType::Option),
    ("EXPORT_LINK_INTERFACE_LIBRARIES", KwType::Option),
    ("EXPORT_PACKAGE_DEPENDENCIES", KwType::Option),
    ("ALL_COMPONENTS", KwType::Option),
    ("LOWER_CASE_FILE", KwType::Option),
    // OneValue
    ("TYPE", KwType::OneValue),
    ("DESTINATION", KwType::OneValue),
    ("COMPONENT", KwType::OneValue),
    ("NAMELINK_COMPONENT", KwType::OneValue),
    ("EXPORT", KwType::OneValue),
    ("RUNTIME_DEPENDENCY_SET", KwType::OneValue),
    ("RENAME", KwType::OneValue),
    ("SCRIPT", KwType::OneValue),
    ("CODE", KwType::OneValue),
    ("NAMESPACE", KwType::OneValue),
    ("FILE", KwType::OneValue),
    ("PACKAGE_INFO", KwType::OneValue),
    ("APPENDIX", KwType::OneValue),
    ("DEFAULT_LICENSE", KwType::OneValue),
    ("LICENSE", KwType::OneValue),
    ("DESCRIPTION", KwType::OneValue),
    ("HOMEPAGE_URL", KwType::OneValue),
    ("PROJECT", KwType::OneValue),
    // MultiValue
    ("TARGETS", KwType::MultiValue),
    ("FILES", KwType::MultiValue),
    ("PROGRAMS", KwType::MultiValue),
    ("DIRECTORY", KwType::MultiValue),
    ("PERMISSIONS", KwType::MultiValue),
    ("CONFIGURATIONS", KwType::MultiValue),
    ("ARCHIVE", KwType::MultiValue),
    ("LIBRARY", KwType::MultiValue),
    ("RUNTIME", KwType::MultiValue),
    ("OBJECTS", KwType::MultiValue),
    ("FRAMEWORK", KwType::MultiValue),
    ("BUNDLE", KwType::MultiValue),
    ("PUBLIC_HEADER", KwType::MultiValue),
    ("PRIVATE_HEADER", KwType::MultiValue),
    ("RESOURCE", KwType::MultiValue),
    ("CXX_MODULES_BMI", KwType::MultiValue),
    ("FILE_SET", KwType::MultiValue),
    ("INCLUDES", KwType::MultiValue),
    ("EXPORT_ANDROID_MK", KwType::MultiValue),
    ("IMPORTED_RUNTIME_ARTIFACTS", KwType::MultiValue),
    ("RUNTIME_DEPENDENCIES", KwType::MultiValue),
    ("FILE_PERMISSIONS", KwType::MultiValue),
    ("DIRECTORY_PERMISSIONS", KwType::MultiValue),
    ("DEFAULT_TARGETS", KwType::MultiValue),
    ("DEFAULT_CONFIGURATIONS", KwType::MultiValue),
    ("PATTERN", KwType::MultiValue),
    ("REGEX", KwType::MultiValue),
    ("VERSION", KwType::MultiValue),
    ("COMPAT_VERSION", KwType::MultiValue),
    ("VERSION_SCHEMA", KwType::MultiValue),
];

static INSTALL_ARTIFACT_SEC_SUB: &[(&str, KwType)] = &[
    ("DESTINATION", KwType::OneValue),
    ("COMPONENT", KwType::OneValue),
    ("NAMELINK_COMPONENT", KwType::OneValue),
    ("OPTIONAL", KwType::Option),
    ("EXCLUDE_FROM_ALL", KwType::Option),
    ("NAMELINK_ONLY", KwType::Option),
    ("NAMELINK_SKIP", KwType::Option),
    ("PERMISSIONS", KwType::MultiValue),
    ("CONFIGURATIONS", KwType::MultiValue),
];

static INSTALL_PATTERN_SEC_SUB: &[(&str, KwType)] = &[
    ("EXCLUDE", KwType::Option),
    ("PERMISSIONS", KwType::MultiValue),
];

static INSTALL_VERSION_SEC_SUB: &[(&str, KwType)] = &[
    ("COMPAT_VERSION", KwType::OneValue),
    ("VERSION_SCHEMA", KwType::OneValue),
];
static INSTALL_SECTIONS: &[SectionDef] = &[
    ("ARCHIVE", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("LIBRARY", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("RUNTIME", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("OBJECTS", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("FRAMEWORK", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("BUNDLE", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("PUBLIC_HEADER", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("PRIVATE_HEADER", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("RESOURCE", 0, INSTALL_ARTIFACT_SEC_SUB),
    ("FILE_SET", 1, INSTALL_ARTIFACT_SEC_SUB),
    ("PATTERN", 1, INSTALL_PATTERN_SEC_SUB),
    ("REGEX", 1, INSTALL_PATTERN_SEC_SUB),
    ("VERSION", 1, INSTALL_VERSION_SEC_SUB),
];

static INSTALL_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: INSTALL_KW,
    sections: INSTALL_SECTIONS,
    command_line_keywords: &[],
    pair_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[("INCLUDES", "DESTINATION")],
    once_keywords: &[],
    property_keywords: &[],
};

// ---------------------------------------------------------------------------
// 39. add_custom_command
// ---------------------------------------------------------------------------

static ADD_CUSTOM_COMMAND_KW: &[(&str, KwType)] = &[
    ("VERBATIM", KwType::Option),
    ("APPEND", KwType::Option),
    ("USES_TERMINAL", KwType::Option),
    ("COMMAND_EXPAND_LISTS", KwType::Option),
    ("DEPENDS_EXPLICIT_ONLY", KwType::Option),
    ("CODEGEN", KwType::Option),
    ("PRE_BUILD", KwType::Option),
    ("PRE_LINK", KwType::Option),
    ("POST_BUILD", KwType::Option),
    ("TARGET", KwType::OneValue),
    ("MAIN_DEPENDENCY", KwType::OneValue),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("COMMENT", KwType::OneValue),
    ("DEPFILE", KwType::OneValue),
    ("JOB_POOL", KwType::OneValue),
    ("JOB_SERVER_AWARE", KwType::OneValue),
    ("OUTPUT", KwType::MultiValue),
    ("COMMAND", KwType::MultiValue),
    ("ARGS", KwType::MultiValue),
    ("DEPENDS", KwType::MultiValue),
    ("BYPRODUCTS", KwType::MultiValue),
    ("IMPLICIT_DEPENDS", KwType::MultiValue),
];

static ADD_CUSTOM_COMMAND_SPEC: CommandSpec = spec! {
    front: 0, back: 0,
    kw: ADD_CUSTOM_COMMAND_KW,
    sections: &[],
    cmd_line: &[],
    pair: &["IMPLICIT_DEPENDS"],
    flow: &["COMMAND", "ARGS"],
};

// ---------------------------------------------------------------------------
// 40. add_custom_target
// ---------------------------------------------------------------------------

static ADD_CUSTOM_TARGET_KW: &[(&str, KwType)] = &[
    ("ALL", KwType::Option),
    ("VERBATIM", KwType::Option),
    ("USES_TERMINAL", KwType::Option),
    ("COMMAND_EXPAND_LISTS", KwType::Option),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("COMMENT", KwType::OneValue),
    ("JOB_POOL", KwType::OneValue),
    ("JOB_SERVER_AWARE", KwType::OneValue),
    ("COMMAND", KwType::MultiValue),
    ("DEPENDS", KwType::MultiValue),
    ("BYPRODUCTS", KwType::MultiValue),
    ("SOURCES", KwType::MultiValue),
];

static ADD_CUSTOM_TARGET_SPEC: CommandSpec = spec! {
    front: 1, back: 0,
    kw: ADD_CUSTOM_TARGET_KW,
    sections: &[],
    cmd_line: &[],
    pair: &[],
    flow: &["COMMAND"],
    flow_positional: true,
};

// ---------------------------------------------------------------------------
// 41. try_compile
// ---------------------------------------------------------------------------

static TRY_COMPILE_KW: &[(&str, KwType)] = &[
    ("NO_CACHE", KwType::Option),
    ("NO_LOG", KwType::Option),
    ("OUTPUT_VARIABLE", KwType::OneValue),
    ("COPY_FILE", KwType::OneValue),
    ("COPY_FILE_ERROR", KwType::OneValue),
    ("C_STANDARD", KwType::OneValue),
    ("C_STANDARD_REQUIRED", KwType::OneValue),
    ("C_EXTENSIONS", KwType::OneValue),
    ("CXX_STANDARD", KwType::OneValue),
    ("CXX_STANDARD_REQUIRED", KwType::OneValue),
    ("CXX_EXTENSIONS", KwType::OneValue),
    ("OBJC_STANDARD", KwType::OneValue),
    ("OBJC_STANDARD_REQUIRED", KwType::OneValue),
    ("OBJC_EXTENSIONS", KwType::OneValue),
    ("OBJCXX_STANDARD", KwType::OneValue),
    ("OBJCXX_STANDARD_REQUIRED", KwType::OneValue),
    ("OBJCXX_EXTENSIONS", KwType::OneValue),
    ("CUDA_STANDARD", KwType::OneValue),
    ("CUDA_STANDARD_REQUIRED", KwType::OneValue),
    ("CUDA_EXTENSIONS", KwType::OneValue),
    ("PROJECT", KwType::OneValue),
    ("SOURCE_DIR", KwType::OneValue),
    ("BINARY_DIR", KwType::OneValue),
    ("TARGET", KwType::OneValue),
    ("LOG_DESCRIPTION", KwType::OneValue),
    ("SOURCES_TYPE", KwType::OneValue),
    ("LINKER_LANGUAGE", KwType::OneValue),
    ("SOURCES", KwType::MultiValue),
    ("CMAKE_FLAGS", KwType::MultiValue),
    ("COMPILE_DEFINITIONS", KwType::MultiValue),
    ("LINK_OPTIONS", KwType::MultiValue),
    ("LINK_LIBRARIES", KwType::MultiValue),
    ("SOURCE_FROM_CONTENT", KwType::MultiValue),
    ("SOURCE_FROM_VAR", KwType::MultiValue),
    ("SOURCE_FROM_FILE", KwType::MultiValue),
];

static TRY_COMPILE_SPEC: CommandSpec = spec! {
    front: 5, back: 0, kw: TRY_COMPILE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 42. try_run
// ---------------------------------------------------------------------------

static TRY_RUN_KW: &[(&str, KwType)] = &[
    ("NO_CACHE", KwType::Option),
    ("NO_LOG", KwType::Option),
    ("COMPILE_OUTPUT_VARIABLE", KwType::OneValue),
    ("RUN_OUTPUT_VARIABLE", KwType::OneValue),
    ("OUTPUT_VARIABLE", KwType::OneValue),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("COPY_FILE", KwType::OneValue),
    ("COPY_FILE_ERROR", KwType::OneValue),
    ("CMAKE_FLAGS", KwType::MultiValue),
    ("COMPILE_DEFINITIONS", KwType::MultiValue),
    ("LINK_OPTIONS", KwType::MultiValue),
    ("LINK_LIBRARIES", KwType::MultiValue),
    ("ARGS", KwType::MultiValue),
    ("SOURCES", KwType::MultiValue),
    ("SOURCE_FROM_CONTENT", KwType::MultiValue),
    ("SOURCE_FROM_VAR", KwType::MultiValue),
    ("SOURCE_FROM_FILE", KwType::MultiValue),
];

static TRY_RUN_SPEC: CommandSpec = spec! {
    front: 4, back: 0, kw: TRY_RUN_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 43. source_group
// ---------------------------------------------------------------------------

static SOURCE_GROUP_KW: &[(&str, KwType)] = &[
    ("REGULAR_EXPRESSION", KwType::OneValue),
    ("TREE", KwType::OneValue),
    ("PREFIX", KwType::OneValue),
    ("FILES", KwType::MultiValue),
];

static SOURCE_GROUP_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: SOURCE_GROUP_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 44. set_target_properties
// ---------------------------------------------------------------------------

static SET_TARGET_PROPERTIES_KW: &[(&str, KwType)] = &[("PROPERTIES", KwType::MultiValue)];

static SET_TARGET_PROPERTIES_SPEC: CommandSpec = spec! { front: 1, back: 0, kw: SET_TARGET_PROPERTIES_KW, sections: &[], cmd_line: &[], pair: &["PROPERTIES"], };

// ---------------------------------------------------------------------------
// 45. set_source_files_properties
// ---------------------------------------------------------------------------

static SET_SOURCE_FILES_PROPERTIES_KW: &[(&str, KwType)] = &[
    ("PROPERTIES", KwType::MultiValue),
    ("DIRECTORY", KwType::MultiValue),
    ("TARGET_DIRECTORY", KwType::MultiValue),
];

static SET_SOURCE_FILES_PROPERTIES_SPEC: CommandSpec = spec! {
    front: 0, back: 0,
    kw: SET_SOURCE_FILES_PROPERTIES_KW,
    sections: &[],
    cmd_line: &[],
    pair: &["PROPERTIES"],
};

// ---------------------------------------------------------------------------
// 46. set_tests_properties
// ---------------------------------------------------------------------------

static SET_TESTS_PROPERTIES_KW: &[(&str, KwType)] = &[
    ("DIRECTORY", KwType::OneValue),
    ("PROPERTIES", KwType::MultiValue),
];

static SET_TESTS_PROPERTIES_SPEC: CommandSpec = spec! {
    front: 0, back: 0,
    kw: SET_TESTS_PROPERTIES_KW,
    sections: &[],
    cmd_line: &[],
    pair: &["PROPERTIES"],
};

// ---------------------------------------------------------------------------
// 47. set_directory_properties
// ---------------------------------------------------------------------------

static SET_DIRECTORY_PROPERTIES_KW: &[(&str, KwType)] = &[("PROPERTIES", KwType::MultiValue)];

static SET_DIRECTORY_PROPERTIES_SPEC: CommandSpec = spec! {
    front: 0, back: 0,
    kw: SET_DIRECTORY_PROPERTIES_KW,
    sections: &[],
    cmd_line: &[],
    pair: &["PROPERTIES"],
};

// ---------------------------------------------------------------------------
// 48. set_package_properties
// ---------------------------------------------------------------------------

static SET_PACKAGE_PROPERTIES_KW: &[(&str, KwType)] = &[("PROPERTIES", KwType::MultiValue)];

static SET_PACKAGE_PROPERTIES_SECTIONS: &[SectionDef] = &[(
    "PROPERTIES",
    0,
    &[
        ("URL", KwType::OneValue),
        ("DESCRIPTION", KwType::OneValue),
        ("TYPE", KwType::OneValue),
        ("PURPOSE", KwType::OneValue),
    ],
)];

static SET_PACKAGE_PROPERTIES_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: SET_PACKAGE_PROPERTIES_KW, sections: SET_PACKAGE_PROPERTIES_SECTIONS, cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 49. get_directory_property
// ---------------------------------------------------------------------------

static GET_DIRECTORY_PROPERTY_KW: &[(&str, KwType)] = &[
    ("DIRECTORY", KwType::OneValue),
    ("DEFINITION", KwType::OneValue),
];

static GET_DIRECTORY_PROPERTY_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: GET_DIRECTORY_PROPERTY_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 50. get_filename_component
// ---------------------------------------------------------------------------

static GET_FILENAME_COMPONENT_KW: &[(&str, KwType)] = &[
    ("DIRECTORY", KwType::Option),
    ("NAME", KwType::Option),
    ("EXT", KwType::Option),
    ("NAME_WE", KwType::Option),
    ("LAST_EXT", KwType::Option),
    ("NAME_WLE", KwType::Option),
    ("PATH", KwType::Option),
    ("PROGRAM", KwType::Option),
    ("CACHE", KwType::Option),
    ("BASE_DIR", KwType::OneValue),
    ("PROGRAM_ARGS", KwType::OneValue),
];

static GET_FILENAME_COMPONENT_SPEC: CommandSpec = spec! {
    front: 3, back: 0, kw: GET_FILENAME_COMPONENT_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 51. gtest_discover_tests
// ---------------------------------------------------------------------------

static GTEST_DISCOVER_TESTS_KW: &[(&str, KwType)] = &[
    ("NO_PRETTY_TYPES", KwType::Option),
    ("NO_PRETTY_VALUES", KwType::Option),
    ("WORKING_DIRECTORY", KwType::OneValue),
    ("TEST_PREFIX", KwType::OneValue),
    ("TEST_SUFFIX", KwType::OneValue),
    ("TEST_FILTER", KwType::OneValue),
    ("TEST_LIST", KwType::OneValue),
    ("DISCOVERY_TIMEOUT", KwType::OneValue),
    ("XML_OUTPUT_DIR", KwType::OneValue),
    ("DISCOVERY_MODE", KwType::OneValue),
    ("EXTRA_ARGS", KwType::MultiValue),
    ("PROPERTIES", KwType::MultiValue),
    ("DISCOVERY_EXTRA_ARGS", KwType::MultiValue),
];

static GTEST_DISCOVER_TESTS_SPEC: CommandSpec = spec! {
    front: 1, back: 0,
    kw: GTEST_DISCOVER_TESTS_KW,
    sections: &[],
    cmd_line: &[],
    pair: &["PROPERTIES"],
    flow: &["EXTRA_ARGS", "DISCOVERY_EXTRA_ARGS"],
};

// ---------------------------------------------------------------------------
// 52. build_command
// ---------------------------------------------------------------------------

static BUILD_COMMAND_KW: &[(&str, KwType)] = &[
    ("CONFIGURATION", KwType::OneValue),
    ("TARGET", KwType::OneValue),
    ("PROJECT_NAME", KwType::OneValue),
    ("PARALLEL_LEVEL", KwType::OneValue),
];

static BUILD_COMMAND_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: BUILD_COMMAND_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 53. cmake_host_system_information
// ---------------------------------------------------------------------------

static CMAKE_HOST_SYSTEM_INFORMATION_KW: &[(&str, KwType)] = &[
    ("RESULT", KwType::OneValue),
    ("VALUE", KwType::OneValue),
    ("VIEW", KwType::OneValue),
    ("SEPARATOR", KwType::OneValue),
    ("ERROR_VARIABLE", KwType::OneValue),
    ("QUERY", KwType::MultiValue),
];

static CMAKE_HOST_SYSTEM_INFORMATION_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: CMAKE_HOST_SYSTEM_INFORMATION_KW,
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[("QUERY", "WINDOWS_REGISTRY")],
    once_keywords: &[],
    property_keywords: &[],
};

// ---------------------------------------------------------------------------
// 54. cmake_language (merged)
// ---------------------------------------------------------------------------

static CMAKE_LANGUAGE_KW: &[(&str, KwType)] = &[
    ("ON", KwType::Option),
    ("OFF", KwType::Option),
    ("EXPAND", KwType::Option),
    ("TRACE", KwType::Option),
    ("CALL", KwType::OneValue),
    ("GET_CALL_IDS", KwType::OneValue),
    ("SET_DEPENDENCY_PROVIDER", KwType::OneValue),
    ("GET_MESSAGE_LOG_LEVEL", KwType::OneValue),
    ("EXIT", KwType::OneValue),
    ("DIRECTORY", KwType::OneValue),
    ("ID", KwType::OneValue),
    ("ID_VAR", KwType::OneValue),
    ("DEFER", KwType::MultiValue),
    ("GET_CALL", KwType::MultiValue),
    ("CANCEL_CALL", KwType::MultiValue),
    ("SUPPORTED_METHODS", KwType::MultiValue),
    ("EVAL", KwType::MultiValue),
];

static CMAKE_LANGUAGE_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: CMAKE_LANGUAGE_KW,
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[("EVAL", "CODE")],
    once_keywords: &[],
    property_keywords: &[],
};

// ---------------------------------------------------------------------------
// 55. cmake_path (merged)
// ---------------------------------------------------------------------------

static CMAKE_PATH_KW: &[(&str, KwType)] = &[
    ("LAST_ONLY", KwType::Option),
    ("NORMALIZE", KwType::Option),
    ("HASH", KwType::Option),
    ("GET", KwType::OneValue),
    ("ROOT_NAME", KwType::OneValue),
    ("ROOT_DIRECTORY", KwType::OneValue),
    ("ROOT_PATH", KwType::OneValue),
    ("FILENAME", KwType::OneValue),
    ("EXTENSION", KwType::OneValue),
    ("STEM", KwType::OneValue),
    ("RELATIVE_PART", KwType::OneValue),
    ("PARENT_PATH", KwType::OneValue),
    ("APPEND", KwType::OneValue),
    ("APPEND_STRING", KwType::OneValue),
    ("REMOVE_FILENAME", KwType::OneValue),
    ("REPLACE_FILENAME", KwType::OneValue),
    ("REMOVE_EXTENSION", KwType::OneValue),
    ("REPLACE_EXTENSION", KwType::OneValue),
    ("NORMAL_PATH", KwType::OneValue),
    ("RELATIVE_PATH", KwType::OneValue),
    ("ABSOLUTE_PATH", KwType::OneValue),
    ("BASE_DIRECTORY", KwType::OneValue),
    ("OUTPUT_VARIABLE", KwType::OneValue),
    ("NATIVE_PATH", KwType::OneValue),
    ("CONVERT", KwType::OneValue),
    ("TO_CMAKE_PATH_LIST", KwType::OneValue),
    ("TO_NATIVE_PATH_LIST", KwType::OneValue),
];

static CMAKE_PATH_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: CMAKE_PATH_KW,
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[("EXTENSION", "LAST_ONLY"), ("STEM", "LAST_ONLY")],
    once_keywords: &[],
    property_keywords: &[],
};

// ---------------------------------------------------------------------------
// 56. cmake_pkg_config
// ---------------------------------------------------------------------------

static CMAKE_PKG_CONFIG_KW: &[(&str, KwType)] = &[
    ("REQUIRED", KwType::Option),
    ("EXACT", KwType::Option),
    ("QUIET", KwType::Option),
    ("STRICTNESS", KwType::Option),
    ("EXTRACT", KwType::MultiValue),
    ("POPULATE", KwType::MultiValue),
    ("IMPORT", KwType::MultiValue),
    ("PREFIX", KwType::OneValue),
    ("NAME", KwType::OneValue),
    ("ENV_MODE", KwType::OneValue),
    ("PC_PATH", KwType::MultiValue),
];

static CMAKE_PKG_CONFIG_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CMAKE_PKG_CONFIG_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 57. separate_arguments
// ---------------------------------------------------------------------------

static SEPARATE_ARGUMENTS_KW: &[(&str, KwType)] = &[
    ("PROGRAM", KwType::Option),
    ("SEPARATE_ARGS", KwType::Option),
    ("UNIX_COMMAND", KwType::Option),
    ("WINDOWS_COMMAND", KwType::Option),
    ("NATIVE_COMMAND", KwType::Option),
];

static SEPARATE_ARGUMENTS_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: SEPARATE_ARGUMENTS_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 58. enable_language
// ---------------------------------------------------------------------------

static ENABLE_LANGUAGE_KW: &[(&str, KwType)] = &[("OPTIONAL", KwType::Option)];

static ENABLE_LANGUAGE_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: ENABLE_LANGUAGE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 59. mark_as_advanced
// ---------------------------------------------------------------------------

static MARK_AS_ADVANCED_KW: &[(&str, KwType)] =
    &[("CLEAR", KwType::Option), ("FORCE", KwType::Option)];

static MARK_AS_ADVANCED_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: MARK_AS_ADVANCED_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 60. option
// ---------------------------------------------------------------------------

static OPTION_SPEC: CommandSpec =
    spec! { front: 0, back: 0, kw: &[], sections: &[], cmd_line: &[], pair: &[], };

// ---------------------------------------------------------------------------
// 61. unset
// ---------------------------------------------------------------------------

static UNSET_KW: &[(&str, KwType)] = &[("CACHE", KwType::Option), ("PARENT_SCOPE", KwType::Option)];

static UNSET_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: UNSET_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 62. block
// ---------------------------------------------------------------------------

static BLOCK_KW: &[(&str, KwType)] = &[
    ("SCOPE_FOR", KwType::MultiValue),
    ("PROPAGATE", KwType::MultiValue),
];

static BLOCK_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: BLOCK_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 63. return
// ---------------------------------------------------------------------------

static RETURN_KW: &[(&str, KwType)] = &[("PROPAGATE", KwType::MultiValue)];

static RETURN_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: RETURN_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 64. include_directories
// ---------------------------------------------------------------------------

static INCLUDE_DIRECTORIES_KW: &[(&str, KwType)] = &[
    ("AFTER", KwType::Option),
    ("BEFORE", KwType::Option),
    ("SYSTEM", KwType::Option),
];

static INCLUDE_DIRECTORIES_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: INCLUDE_DIRECTORIES_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 65. link_directories
// ---------------------------------------------------------------------------

static LINK_DIRECTORIES_KW: &[(&str, KwType)] =
    &[("AFTER", KwType::Option), ("BEFORE", KwType::Option)];

static LINK_DIRECTORIES_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: LINK_DIRECTORIES_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 66. link_libraries
// ---------------------------------------------------------------------------

static LINK_LIBRARIES_SECTIONS: &[SectionDef] = &[
    ("PUBLIC", 0, TLL_SEC_SUB),
    ("PRIVATE", 0, TLL_SEC_SUB),
    ("INTERFACE", 0, TLL_SEC_SUB),
    ("LINK_PUBLIC", 0, TLL_SEC_SUB),
    ("LINK_PRIVATE", 0, TLL_SEC_SUB),
    ("LINK_INTERFACE_LIBRARIES", 0, TLL_SEC_SUB),
];

static LINK_LIBRARIES_SPEC: CommandSpec = spec! {
    front: 0, back: 0,
    kw: &[],
    sections: LINK_LIBRARIES_SECTIONS,
    cmd_line: &["debug", "optimized", "general"], pair: &[],
};

// ---------------------------------------------------------------------------
// 67. include_external_msproject
// ---------------------------------------------------------------------------

static INCLUDE_EXTERNAL_MSPROJECT_KW: &[(&str, KwType)] = &[
    ("TYPE", KwType::OneValue),
    ("GUID", KwType::OneValue),
    ("PLATFORM", KwType::OneValue),
];

static INCLUDE_EXTERNAL_MSPROJECT_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: INCLUDE_EXTERNAL_MSPROJECT_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 68. load_cache
// ---------------------------------------------------------------------------

static LOAD_CACHE_KW: &[(&str, KwType)] = &[
    ("READ_WITH_PREFIX", KwType::OneValue),
    ("EXCLUDE", KwType::MultiValue),
    ("INCLUDE_INTERNALS", KwType::MultiValue),
];

static LOAD_CACHE_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: LOAD_CACHE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 69. create_test_sourcelist
// ---------------------------------------------------------------------------

static CREATE_TEST_SOURCELIST_KW: &[(&str, KwType)] = &[
    ("EXTRA_INCLUDE", KwType::OneValue),
    ("FUNCTION", KwType::OneValue),
];

static CREATE_TEST_SOURCELIST_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: CREATE_TEST_SOURCELIST_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 70. ctest_build
// ---------------------------------------------------------------------------

static CTEST_BUILD_KW: &[(&str, KwType)] = &[
    ("APPEND", KwType::Option),
    ("QUIET", KwType::Option),
    ("BUILD", KwType::OneValue),
    ("CONFIGURATION", KwType::OneValue),
    ("FLAGS", KwType::OneValue),
    ("PROJECT_NAME", KwType::OneValue),
    ("TARGET", KwType::OneValue),
    ("NUMBER_ERRORS", KwType::OneValue),
    ("NUMBER_WARNINGS", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
    ("PARALLEL_LEVEL", KwType::OneValue),
];

static CTEST_BUILD_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_BUILD_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 71. ctest_configure
// ---------------------------------------------------------------------------

static CTEST_CONFIGURE_KW: &[(&str, KwType)] = &[
    ("APPEND", KwType::Option),
    ("QUIET", KwType::Option),
    ("BUILD", KwType::OneValue),
    ("SOURCE", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
    ("OPTIONS", KwType::MultiValue),
];

static CTEST_CONFIGURE_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_CONFIGURE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 72. ctest_coverage
// ---------------------------------------------------------------------------

static CTEST_COVERAGE_KW: &[(&str, KwType)] = &[
    ("APPEND", KwType::Option),
    ("QUIET", KwType::Option),
    ("BUILD", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
    ("LABELS", KwType::MultiValue),
];

static CTEST_COVERAGE_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_COVERAGE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 73. ctest_empty_binary_directory
// ---------------------------------------------------------------------------

static CTEST_EMPTY_BINARY_DIRECTORY_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: &[], sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 74. ctest_memcheck
// ---------------------------------------------------------------------------

static CTEST_MEMCHECK_KW: &[(&str, KwType)] = &[
    ("APPEND", KwType::Option),
    ("QUIET", KwType::Option),
    ("BUILD", KwType::OneValue),
    ("START", KwType::OneValue),
    ("END", KwType::OneValue),
    ("STRIDE", KwType::OneValue),
    ("EXCLUDE", KwType::OneValue),
    ("INCLUDE", KwType::OneValue),
    ("EXCLUDE_LABEL", KwType::OneValue),
    ("INCLUDE_LABEL", KwType::OneValue),
    ("EXCLUDE_FIXTURE", KwType::OneValue),
    ("EXCLUDE_FIXTURE_SETUP", KwType::OneValue),
    ("EXCLUDE_FIXTURE_CLEANUP", KwType::OneValue),
    ("PARALLEL_LEVEL", KwType::OneValue),
    ("TEST_LOAD", KwType::OneValue),
    ("SCHEDULE_RANDOM", KwType::OneValue),
    ("STOP_TIME", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("DEFECT_COUNT", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
];

static CTEST_MEMCHECK_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_MEMCHECK_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 75. ctest_read_custom_files
// ---------------------------------------------------------------------------

static CTEST_READ_CUSTOM_FILES_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: &[], sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 76. ctest_run_script
// ---------------------------------------------------------------------------

static CTEST_RUN_SCRIPT_KW: &[(&str, KwType)] = &[
    ("NEW_PROCESS", KwType::Option),
    ("RETURN_VALUE", KwType::OneValue),
];

static CTEST_RUN_SCRIPT_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_RUN_SCRIPT_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 77. ctest_sleep
// ---------------------------------------------------------------------------

static CTEST_SLEEP_SPEC: CommandSpec = spec! {
    front: 3, back: 0, kw: &[], sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 78. ctest_start
// ---------------------------------------------------------------------------

static CTEST_START_KW: &[(&str, KwType)] = &[
    ("QUIET", KwType::Option),
    ("APPEND", KwType::Option),
    ("GROUP", KwType::OneValue),
];

static CTEST_START_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: CTEST_START_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 79. ctest_submit
// ---------------------------------------------------------------------------

static CTEST_SUBMIT_KW: &[(&str, KwType)] = &[
    ("QUIET", KwType::Option),
    ("CDASH_UPLOAD", KwType::OneValue),
    ("CDASH_UPLOAD_TYPE", KwType::OneValue),
    ("SUBMIT_URL", KwType::OneValue),
    ("BUILD_ID", KwType::OneValue),
    ("RETRY_COUNT", KwType::OneValue),
    ("RETRY_DELAY", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
    ("PARTS", KwType::MultiValue),
    ("FILES", KwType::MultiValue),
    ("HTTPHEADER", KwType::MultiValue),
];

static CTEST_SUBMIT_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_SUBMIT_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 80. ctest_test
// ---------------------------------------------------------------------------

static CTEST_TEST_KW: &[(&str, KwType)] = &[
    ("APPEND", KwType::Option),
    ("QUIET", KwType::Option),
    ("BUILD", KwType::OneValue),
    ("START", KwType::OneValue),
    ("END", KwType::OneValue),
    ("STRIDE", KwType::OneValue),
    ("EXCLUDE", KwType::OneValue),
    ("INCLUDE", KwType::OneValue),
    ("EXCLUDE_LABEL", KwType::OneValue),
    ("INCLUDE_LABEL", KwType::OneValue),
    ("EXCLUDE_FIXTURE", KwType::OneValue),
    ("EXCLUDE_FIXTURE_SETUP", KwType::OneValue),
    ("EXCLUDE_FIXTURE_CLEANUP", KwType::OneValue),
    ("PARALLEL_LEVEL", KwType::OneValue),
    ("RESOURCE_SPEC_FILE", KwType::OneValue),
    ("TEST_LOAD", KwType::OneValue),
    ("SCHEDULE_RANDOM", KwType::OneValue),
    ("STOP_TIME", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
    ("INCLUDE_FROM_FILE", KwType::OneValue),
    ("EXCLUDE_FROM_FILE", KwType::OneValue),
];

static CTEST_TEST_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_TEST_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 81. ctest_update
// ---------------------------------------------------------------------------

static CTEST_UPDATE_KW: &[(&str, KwType)] = &[
    ("QUIET", KwType::Option),
    ("SOURCE", KwType::OneValue),
    ("RETURN_VALUE", KwType::OneValue),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
];

static CTEST_UPDATE_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_UPDATE_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// 82. ctest_upload
// ---------------------------------------------------------------------------

static CTEST_UPLOAD_KW: &[(&str, KwType)] = &[
    ("QUIET", KwType::Option),
    ("CAPTURE_CMAKE_ERROR", KwType::OneValue),
    ("FILES", KwType::MultiValue),
];

static CTEST_UPLOAD_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: CTEST_UPLOAD_KW, sections: &[], cmd_line: &[], pair: &[],
};

// ---------------------------------------------------------------------------
// Simple commands (no keywords, all positional)
// ---------------------------------------------------------------------------

static NO_KW: &[(&str, KwType)] = &[];

// Generic spec: no keywords, up to 1 positional arg
static NO_KW_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// add_compile_definitions(<definition> ...)
static ADD_COMPILE_DEFINITIONS_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// add_compile_options(<option> ...)
static ADD_COMPILE_OPTIONS_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// add_definitions(-D<def> ...)
static ADD_DEFINITIONS_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// add_dependencies(<target> <dep>...)
static ADD_DEPENDENCIES_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// add_link_options(<option>...)
static ADD_LINK_OPTIONS_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// aux_source_directory(<dir> <var>)
static AUX_SOURCE_DIRECTORY_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// enable_testing()
static ENABLE_TESTING_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// fltk_wrap_ui(<target> <source>...)
static FLTK_WRAP_UI_SPEC: CommandSpec = spec! {
    front: 1, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// get_source_file_property(<var> <filename> <property>)
static GET_SOURCE_FILE_PROPERTY_SPEC: CommandSpec = spec! {
    front: 3, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// get_target_property(<var> <target> <property>)
static GET_TARGET_PROPERTY_SPEC: CommandSpec = spec! {
    front: 3, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// get_test_property(<test> <property> <var>)
static GET_TEST_PROPERTY_SPEC: CommandSpec = spec! {
    front: 3, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// include_regular_expression(<regex_match> [<regex_complain>])
static INCLUDE_REGULAR_EXPRESSION_SPEC: CommandSpec = spec! {
    front: 2, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

// remove_definitions(-D<def>...)
static REMOVE_DEFINITIONS_SPEC: CommandSpec = spec! {
    front: 0, back: 0, kw: NO_KW, sections: &[], cmd_line: &[], pair: &[],
};

/// Empty spec used for unknown commands that have customKeywords applied.
/// Allows the formatting engine to treat them as keyword-structured commands.
pub static EMPTY_SPEC: CommandSpec = CommandSpec {
    front_positional: 0,
    back_positional: 0,
    keywords: &[],
    sections: &[],
    command_line_keywords: &[],
    pair_keywords: &[],
    property_keywords: &[],
    flow_keywords: &[],
    flow_positional: false,
    compound_keywords: &[],
    once_keywords: &[],
};

// ---------------------------------------------------------------------------
// lookup_command
// ---------------------------------------------------------------------------

pub fn lookup_command(name: &str) -> Option<CommandKind> {
    // Stack-allocated lowercase buffer for short names (covers all CMake commands)
    let mut buf = [0u8; 64];
    let len = name.len();
    if len > buf.len() {
        return None; // No CMake command is this long
    }
    buf[..len].copy_from_slice(name.as_bytes());
    buf[..len].make_ascii_lowercase();
    // SAFETY: input was valid UTF-8 and make_ascii_lowercase preserves UTF-8
    let lower = std::str::from_utf8(&buf[..len]).unwrap();
    match lower {
        // Condition syntax
        "if" | "while" | "elseif" | "else" | "endif" | "endwhile" => {
            Some(CommandKind::ConditionSyntax)
        }

        // Known commands
        "set" => Some(CommandKind::Known(&SET_SPEC)),
        "target_link_libraries" => Some(CommandKind::Known(&TARGET_LINK_LIBRARIES_SPEC)),
        "target_sources" => Some(CommandKind::Known(&TARGET_SOURCES_SPEC)),
        "target_compile_definitions" => Some(CommandKind::Known(&TARGET_COMPILE_DEFINITIONS_SPEC)),
        "target_compile_options" => Some(CommandKind::Known(&TARGET_COMPILE_OPTIONS_SPEC)),
        "target_compile_features" => Some(CommandKind::Known(&TARGET_COMPILE_FEATURES_SPEC)),
        "target_include_directories" => Some(CommandKind::Known(&TARGET_INCLUDE_DIRECTORIES_SPEC)),
        "target_link_directories" => Some(CommandKind::Known(&TARGET_LINK_DIRECTORIES_SPEC)),
        "target_link_options" => Some(CommandKind::Known(&TARGET_LINK_OPTIONS_SPEC)),
        "target_precompile_headers" => Some(CommandKind::Known(&TARGET_PRECOMPILE_HEADERS_SPEC)),
        "add_executable" => Some(CommandKind::Known(&ADD_EXECUTABLE_SPEC)),
        "add_library" => Some(CommandKind::Known(&ADD_LIBRARY_SPEC)),
        "add_test" => Some(CommandKind::Known(&ADD_TEST_SPEC)),
        "project" => Some(CommandKind::Known(&PROJECT_SPEC)),
        "message" => Some(CommandKind::Known(&MESSAGE_SPEC)),
        "find_package" => Some(CommandKind::Known(&FIND_PACKAGE_SPEC)),
        "find_library" => Some(CommandKind::Known(&FIND_LIBRARY_SPEC)),
        "find_file" => Some(CommandKind::Known(&FIND_FILE_SPEC)),
        "find_path" => Some(CommandKind::Known(&FIND_PATH_SPEC)),
        "find_program" => Some(CommandKind::Known(&FIND_PROGRAM_SPEC)),
        "execute_process" => Some(CommandKind::Known(&EXECUTE_PROCESS_SPEC)),
        "cmake_parse_arguments" => Some(CommandKind::Known(&CMAKE_PARSE_ARGUMENTS_SPEC)),
        "define_property" => Some(CommandKind::Known(&DEFINE_PROPERTY_SPEC)),
        "get_property" => Some(CommandKind::Known(&GET_PROPERTY_SPEC)),
        "set_property" => Some(CommandKind::Known(&SET_PROPERTY_SPEC)),
        "export" => Some(CommandKind::Known(&EXPORT_SPEC)),
        "foreach" => Some(CommandKind::Known(&FOREACH_SPEC)),
        "function" => Some(CommandKind::Known(&FUNCTION_SPEC)),
        "macro" => Some(CommandKind::Known(&MACRO_SPEC)),
        "cmake_minimum_required" => Some(CommandKind::Known(&CMAKE_MINIMUM_REQUIRED_SPEC)),
        "configure_file" => Some(CommandKind::Known(&CONFIGURE_FILE_SPEC)),
        "include" => Some(CommandKind::Known(&INCLUDE_SPEC)),
        "math" => Some(CommandKind::Known(&MATH_SPEC)),
        "add_subdirectory" => Some(CommandKind::Known(&ADD_SUBDIRECTORY_SPEC)),
        "string" => Some(CommandKind::Known(&STRING_SPEC)),
        "list" => Some(CommandKind::Known(&LIST_SPEC)),
        "file" => Some(CommandKind::Known(&FILE_SPEC)),
        "install" => Some(CommandKind::Known(&INSTALL_SPEC)),
        "add_custom_command" => Some(CommandKind::Known(&ADD_CUSTOM_COMMAND_SPEC)),
        "add_custom_target" => Some(CommandKind::Known(&ADD_CUSTOM_TARGET_SPEC)),
        "try_compile" => Some(CommandKind::Known(&TRY_COMPILE_SPEC)),
        "try_run" => Some(CommandKind::Known(&TRY_RUN_SPEC)),
        "source_group" => Some(CommandKind::Known(&SOURCE_GROUP_SPEC)),
        "set_target_properties" => Some(CommandKind::Known(&SET_TARGET_PROPERTIES_SPEC)),
        "set_source_files_properties" => {
            Some(CommandKind::Known(&SET_SOURCE_FILES_PROPERTIES_SPEC))
        }
        "set_tests_properties" => Some(CommandKind::Known(&SET_TESTS_PROPERTIES_SPEC)),
        "set_directory_properties" => Some(CommandKind::Known(&SET_DIRECTORY_PROPERTIES_SPEC)),
        "set_package_properties" => Some(CommandKind::Known(&SET_PACKAGE_PROPERTIES_SPEC)),
        "get_directory_property" => Some(CommandKind::Known(&GET_DIRECTORY_PROPERTY_SPEC)),
        "get_filename_component" => Some(CommandKind::Known(&GET_FILENAME_COMPONENT_SPEC)),
        "gtest_discover_tests" => Some(CommandKind::Known(&GTEST_DISCOVER_TESTS_SPEC)),
        "build_command" => Some(CommandKind::Known(&BUILD_COMMAND_SPEC)),
        "cmake_host_system_information" => {
            Some(CommandKind::Known(&CMAKE_HOST_SYSTEM_INFORMATION_SPEC))
        }
        "cmake_language" => Some(CommandKind::Known(&CMAKE_LANGUAGE_SPEC)),
        "cmake_path" => Some(CommandKind::Known(&CMAKE_PATH_SPEC)),
        "cmake_pkg_config" => Some(CommandKind::Known(&CMAKE_PKG_CONFIG_SPEC)),
        "separate_arguments" => Some(CommandKind::Known(&SEPARATE_ARGUMENTS_SPEC)),
        "enable_language" => Some(CommandKind::Known(&ENABLE_LANGUAGE_SPEC)),
        "mark_as_advanced" => Some(CommandKind::Known(&MARK_AS_ADVANCED_SPEC)),
        "option" => Some(CommandKind::Known(&OPTION_SPEC)),
        "unset" => Some(CommandKind::Known(&UNSET_SPEC)),
        "block" => Some(CommandKind::Known(&BLOCK_SPEC)),
        "return" => Some(CommandKind::Known(&RETURN_SPEC)),
        "include_directories" => Some(CommandKind::Known(&INCLUDE_DIRECTORIES_SPEC)),
        "link_directories" => Some(CommandKind::Known(&LINK_DIRECTORIES_SPEC)),
        "link_libraries" => Some(CommandKind::Known(&LINK_LIBRARIES_SPEC)),
        "include_external_msproject" => Some(CommandKind::Known(&INCLUDE_EXTERNAL_MSPROJECT_SPEC)),
        "load_cache" => Some(CommandKind::Known(&LOAD_CACHE_SPEC)),
        "create_test_sourcelist" => Some(CommandKind::Known(&CREATE_TEST_SOURCELIST_SPEC)),
        "ctest_build" => Some(CommandKind::Known(&CTEST_BUILD_SPEC)),
        "ctest_configure" => Some(CommandKind::Known(&CTEST_CONFIGURE_SPEC)),
        "ctest_coverage" => Some(CommandKind::Known(&CTEST_COVERAGE_SPEC)),
        "ctest_empty_binary_directory" => {
            Some(CommandKind::Known(&CTEST_EMPTY_BINARY_DIRECTORY_SPEC))
        }
        "ctest_memcheck" => Some(CommandKind::Known(&CTEST_MEMCHECK_SPEC)),
        "ctest_read_custom_files" => Some(CommandKind::Known(&CTEST_READ_CUSTOM_FILES_SPEC)),
        "ctest_run_script" => Some(CommandKind::Known(&CTEST_RUN_SCRIPT_SPEC)),
        "ctest_sleep" => Some(CommandKind::Known(&CTEST_SLEEP_SPEC)),
        "ctest_start" => Some(CommandKind::Known(&CTEST_START_SPEC)),
        "ctest_submit" => Some(CommandKind::Known(&CTEST_SUBMIT_SPEC)),
        "ctest_test" => Some(CommandKind::Known(&CTEST_TEST_SPEC)),
        "ctest_update" => Some(CommandKind::Known(&CTEST_UPDATE_SPEC)),
        "ctest_upload" => Some(CommandKind::Known(&CTEST_UPLOAD_SPEC)),

        // Simple commands
        "add_compile_definitions" => Some(CommandKind::Known(&ADD_COMPILE_DEFINITIONS_SPEC)),
        "add_compile_options" => Some(CommandKind::Known(&ADD_COMPILE_OPTIONS_SPEC)),
        "add_definitions" => Some(CommandKind::Known(&ADD_DEFINITIONS_SPEC)),
        "add_dependencies" => Some(CommandKind::Known(&ADD_DEPENDENCIES_SPEC)),
        "add_link_options" => Some(CommandKind::Known(&ADD_LINK_OPTIONS_SPEC)),
        "aux_source_directory" => Some(CommandKind::Known(&AUX_SOURCE_DIRECTORY_SPEC)),
        "enable_testing" => Some(CommandKind::Known(&ENABLE_TESTING_SPEC)),
        "fltk_wrap_ui" => Some(CommandKind::Known(&FLTK_WRAP_UI_SPEC)),
        "get_source_file_property" => Some(CommandKind::Known(&GET_SOURCE_FILE_PROPERTY_SPEC)),
        "get_target_property" => Some(CommandKind::Known(&GET_TARGET_PROPERTY_SPEC)),
        "get_test_property" => Some(CommandKind::Known(&GET_TEST_PROPERTY_SPEC)),
        "include_regular_expression" => Some(CommandKind::Known(&INCLUDE_REGULAR_EXPRESSION_SPEC)),
        "remove_definitions" => Some(CommandKind::Known(&REMOVE_DEFINITIONS_SPEC)),

        // Block closers: take 0 or 1 positional arg
        "endforeach" | "endfunction" | "endmacro" | "endblock" => {
            Some(CommandKind::Known(&NO_KW_SPEC))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// All command names in the lookup table should be lowercase.
    #[test]
    fn lookup_command_names_are_lowercase() {
        let commands = [
            "set",
            "target_link_libraries",
            "target_sources",
            "target_compile_definitions",
            "target_compile_options",
            "target_compile_features",
            "target_include_directories",
            "target_link_directories",
            "target_link_options",
            "target_precompile_headers",
            "add_executable",
            "add_library",
            "add_test",
            "project",
            "message",
            "find_package",
            "find_library",
            "find_file",
            "find_path",
            "find_program",
            "execute_process",
            "cmake_parse_arguments",
            "define_property",
            "get_property",
            "set_property",
            "export",
            "foreach",
            "function",
            "macro",
            "cmake_minimum_required",
            "configure_file",
            "include",
            "math",
            "add_subdirectory",
            "string",
            "list",
            "file",
            "install",
            "add_custom_command",
            "add_custom_target",
            "try_compile",
            "try_run",
            "source_group",
            "set_target_properties",
            "set_source_files_properties",
            "set_tests_properties",
            "set_directory_properties",
            "set_package_properties",
            "get_directory_property",
            "get_filename_component",
            "gtest_discover_tests",
            "build_command",
            "cmake_host_system_information",
            "cmake_language",
            "cmake_path",
            "cmake_pkg_config",
            "separate_arguments",
            "enable_language",
            "mark_as_advanced",
            "option",
            "unset",
            "block",
            "return",
            "include_directories",
            "link_directories",
            "link_libraries",
            "include_external_msproject",
            "load_cache",
            "create_test_sourcelist",
            "add_compile_definitions",
            "add_compile_options",
            "add_definitions",
            "add_dependencies",
            "add_link_options",
            "aux_source_directory",
            "enable_testing",
            "fltk_wrap_ui",
            "get_source_file_property",
            "get_target_property",
            "get_test_property",
            "include_regular_expression",
            "remove_definitions",
        ];

        for cmd in &commands {
            assert!(
                lookup_command(cmd).is_some(),
                "expected known command for lowercase '{cmd}'"
            );
        }
    }

    /// Case-insensitive lookup should work for all known commands.
    #[test]
    fn lookup_command_case_insensitive() {
        assert!(lookup_command("SET").is_some());
        assert!(lookup_command("Set").is_some());
        assert!(lookup_command("MESSAGE").is_some());
        assert!(lookup_command("Find_Package").is_some());
        assert!(lookup_command("CMAKE_MINIMUM_REQUIRED").is_some());
        assert!(lookup_command("TARGET_LINK_LIBRARIES").is_some());
    }

    /// Unknown commands should return None.
    #[test]
    fn lookup_command_unknown_returns_none() {
        assert!(lookup_command("not_a_cmake_command").is_none());
        assert!(lookup_command("my_custom_function").is_none());
        assert!(lookup_command("").is_none());
    }

    /// Very long command names (>64 bytes) should return None without panicking.
    #[test]
    fn lookup_command_very_long_name() {
        let long_name = "a".repeat(100);
        assert!(lookup_command(&long_name).is_none());
    }

    /// Condition syntax commands should be identified correctly.
    #[test]
    fn condition_syntax_commands() {
        let condition_commands = ["if", "while", "elseif", "else", "endif", "endwhile"];
        for cmd in &condition_commands {
            assert!(
                matches!(lookup_command(cmd), Some(CommandKind::ConditionSyntax)),
                "expected ConditionSyntax for '{cmd}'"
            );
        }
    }

    /// Block closers should resolve to Known with the NO_KW_SPEC.
    #[test]
    fn block_closers_are_known() {
        let closers = ["endforeach", "endfunction", "endmacro", "endblock"];
        for cmd in &closers {
            assert!(
                matches!(lookup_command(cmd), Some(CommandKind::Known(_))),
                "expected Known for '{cmd}'"
            );
        }
    }

    /// Core commands should have non-empty keyword lists.
    #[test]
    fn core_commands_have_keywords() {
        let commands_with_keywords = [
            "target_link_libraries",
            "install",
            "find_package",
            "add_custom_command",
            "set_property",
        ];

        for cmd in &commands_with_keywords {
            if let Some(CommandKind::Known(spec)) = lookup_command(cmd) {
                assert!(
                    !spec.keywords.is_empty() || !spec.sections.is_empty(),
                    "expected non-empty keywords or sections for '{cmd}'"
                );
            } else {
                panic!("expected Known for '{cmd}'");
            }
        }
    }

    /// Verify critical CMake commands are present in the database.
    #[test]
    fn critical_commands_are_registered() {
        let critical = [
            "set",
            "add_executable",
            "add_library",
            "target_link_libraries",
            "find_package",
            "install",
            "project",
            "cmake_minimum_required",
            "message",
            "if",
            "foreach",
            "function",
            "macro",
            "add_custom_command",
            "add_custom_target",
            "string",
            "list",
            "file",
        ];

        for cmd in &critical {
            assert!(
                lookup_command(cmd).is_some(),
                "critical command '{cmd}' is missing from the signature database"
            );
        }
    }
}

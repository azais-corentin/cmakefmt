## Appendix F — Keyword Dictionary

> This appendix is normative. Keyword classification, sortable sections (§12.1),
> canonical section ordering (§12.2), and keyword-vs-literal precedence (§4.4)
> are defined here.

### F.1 Command classes

#### F.1.1 Condition-syntax commands

> Arguments are parsed as condition expressions, not keyword/value sections.

- `if`
- `elseif`
- `else`
- `endif`
- `while`
- `endwhile`

#### F.1.2 Simple commands (no keyword sections)

These commands are parsed as positional-only argument lists. They have no keyword sections,
so `sortArguments` and `sortKeywordSections` never apply to them unless keywords are introduced
through `customKeywords` (§4.3).

- `add_compile_definitions`
- `add_compile_options`
- `add_definitions`
- `add_dependencies`
- `add_link_options`
- `aux_source_directory`
- `enable_testing`
- `fltk_wrap_ui`
- `get_source_file_property`
- `get_target_property`
- `get_test_property`
- `include_regular_expression`
- `remove_definitions`

#### F.1.3 Keyword-structured commands

The tables below are exhaustive for keyword-structured behavior defined by this spec.
If a command is not listed below and not added via `customKeywords`, its arguments are treated
as positional values only.

However, a set of common CMake keywords — `PUBLIC`, `PRIVATE`, `INTERFACE`,
`COMPONENTS`, `OPTIONAL_COMPONENTS`, `SOURCES`, `DEPENDS`, `BYPRODUCTS` — are
recognized as keyword sections in *any* command, even when they do not appear in
that command's keyword table above. This ensures correct indentation and sorting
when these widely-used tokens appear in commands whose spec does not explicitly
list them (for example, `PRIVATE` used in `add_executable`). The recognition
follows the same rules as for listed keywords: `keywordCase` applies, values
beneath the keyword receive `continuationIndentWidth`, and `sortArguments`
sorts within the section when enabled.

### F.2 Per-command keyword sections

`Sortable` indicates whether `sortArguments = true` is allowed to reorder values inside that
section. Only simple-value sections are sortable.

| Command family                                                                                                                             | Section keywords                                                                                                             | Sortable sections                                                    | Canonical section order (`sortKeywordSections`)                                                                                       |
| ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `target_link_libraries`                                                                                                                    | `PUBLIC`, `INTERFACE`, `PRIVATE`, `LINK_PUBLIC`, `LINK_PRIVATE`, `LINK_INTERFACE_LIBRARIES`                                  | all listed sections                                                  | `PUBLIC` → `INTERFACE` → `PRIVATE` → `LINK_PUBLIC` → `LINK_PRIVATE` → `LINK_INTERFACE_LIBRARIES`                                      |
| `target_sources`                                                                                                                           | `PUBLIC`, `INTERFACE`, `PRIVATE`                                                                                             | direct values under `PUBLIC`/`INTERFACE`/`PRIVATE` only              | `PUBLIC` → `INTERFACE` → `PRIVATE`                                                                                                    |
| `target_compile_definitions` / `target_compile_options` / `target_compile_features` / `target_link_options` / `target_include_directories` | `PUBLIC`, `INTERFACE`, `PRIVATE`                                                                                             | all listed sections                                                  | none (original section order preserved)                                                                                               |
| `install`                                                                                                                                  | `ARCHIVE`, `LIBRARY`, `RUNTIME`, `OBJECTS`, `FRAMEWORK`, `BUNDLE`, `PUBLIC_HEADER`, `PRIVATE_HEADER`, `RESOURCE`, `FILE_SET` | no (sections contain nested option structures)                       | `ARCHIVE` → `LIBRARY` → `RUNTIME` → `OBJECTS` → `FRAMEWORK` → `BUNDLE` → `PUBLIC_HEADER` → `PRIVATE_HEADER` → `RESOURCE` → `FILE_SET` |
| `export`                                                                                                                                   | `PACKAGE_DEPENDENCY`, `TARGET`, `VERSION`                                                                                    | no                                                                   | `PACKAGE_DEPENDENCY` → `TARGET` → `VERSION`                                                                                           |
| `add_custom_command`                                                                                                                       | `OUTPUT`, `COMMAND`, `DEPENDS`, `BYPRODUCTS`, `WORKING_DIRECTORY`, `COMMENT`, `VERBATIM`, `APPEND`                           | `DEPENDS`, `BYPRODUCTS`                                              | none (original section order preserved)                                                                                               |
| `find_package`                                                                                                                             | `REQUIRED`, `QUIET`, `EXACT`, `MODULE`, `CONFIG`, `COMPONENTS`, `OPTIONAL_COMPONENTS`                                        | `COMPONENTS`, `OPTIONAL_COMPONENTS`                                  | none (original section order preserved)                                                                                               |
| `add_library`                                                                                                                              | `STATIC`, `SHARED`, `MODULE`, `OBJECT`, `INTERFACE`, `UNKNOWN`, `EXCLUDE_FROM_ALL`, `IMPORTED`, `ALIAS`                      | trailing source arguments (after target name + type/option keywords) | none                                                                                                                                  |
| `add_executable`                                                                                                                           | `WIN32`, `MACOSX_BUNDLE`, `EXCLUDE_FROM_ALL`, `IMPORTED`, `ALIAS`                                                            | trailing source arguments (after target name + option keywords)      | none                                                                                                                                  |
| `cmake_minimum_required`                                                                                                                   | `VERSION`                                                                                                                    | no                                                                   | none                                                                                                                                  |
| `set_target_properties` / `set_source_files_properties` / `set_tests_properties` / `set_directory_properties`                              | `PROPERTIES`                                                                                                                 | no (alternating key/value structure)                                 | none                                                                                                                                  |
| `foreach`                                                                                                                                  | `IN`, `ITEMS`, `LISTS`, `RANGE`                                                                                              | no                                                                   | none                                                                                                                                  |
| `block`                                                                                                                                    | `SCOPE_FOR`, `PROPAGATE`                                                                                                     | no                                                                   | none                                                                                                                                  |

### F.3 Nested section structures (non-sortable)

The following nested structures are recognized and are never value-sorted by `sortArguments`:

- `target_sources(... FILE_SET <name> BASE_DIRS ... FILES ...)`
- `install(...)` sections (`ARCHIVE`, `LIBRARY`, `RUNTIME`, etc.) with nested destination/options
- `set_*_properties(... PROPERTIES <key> <value> ...)` alternating property pairs

### F.4 Block closers

The following commands are block closers and accept zero or one positional argument in source:

- `endforeach`
- `endfunction`
- `endmacro`
- `endblock`

`endCommandArgs` (§14.2) governs whether those optional arguments are removed/preserved/matched.

### F.5 Keyword vs literal overlap

Tokens that appear in both keyword sections and the literal list (notably `TARGET`, `COMMAND`,
`POLICY`, `TEST`) follow this precedence rule:

1. If the current command/position is a recognized keyword slot per this appendix, treat token as a keyword (`keywordCase` applies).
2. Otherwise, treat token as a plain value token (`literalCase` may apply if listed in §4.4).

> `customKeywords` (§4.3) extends keyword recognition and also takes precedence over `literalCase`.

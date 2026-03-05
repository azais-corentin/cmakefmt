## Appendix F — Keyword Dictionary

> **Note:** The keyword dictionary in `src/generation/signatures.rs` is the authoritative reference.
> This appendix provides a condensed overview of recognized commands and canonical section orders
> used by `sortKeywordSections` (§12.2). Full keyword tables per command are intentionally omitted
> to avoid drift from the source of truth.

### Condition-syntax commands

`if`, `elseif`, `else`, `endif`, `while`, `endwhile` — parsed as condition expressions, not
keyword-structured commands.

### Commands with canonical section orders

| Command                 | Canonical Section Order                                                                                                               |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `target_link_libraries` | `PUBLIC` → `INTERFACE` → `PRIVATE` → `LINK_PUBLIC` → `LINK_PRIVATE` → `LINK_INTERFACE_LIBRARIES`                                      |
| `target_sources`        | `PUBLIC` → `INTERFACE` → `PRIVATE`                                                                                                    |
| `install`               | `ARCHIVE` → `LIBRARY` → `RUNTIME` → `OBJECTS` → `FRAMEWORK` → `BUNDLE` → `PUBLIC_HEADER` → `PRIVATE_HEADER` → `RESOURCE` → `FILE_SET` |
| `export`                | `PACKAGE_DEPENDENCY` → `TARGET` → `VERSION`                                                                                           |

### Other recognized commands

The following commands are in the keyword dictionary with their own keyword/section definitions:

**Target commands:** `target_compile_definitions`, `target_compile_options`, `target_compile_features`,
`target_link_options`, `target_include_directories`, `add_executable`, `add_library`,
`set_target_properties`, `set_source_files_properties`, `set_tests_properties`, `set_directory_properties`

**Project & package:** `project`, `find_package`, `cmake_minimum_required`, `cmake_pkg_config`

**Custom commands:** `add_custom_command`, `add_custom_target`, `execute_process`

**Variables & properties:** `set`, `unset`, `option`, `return`, `mark_as_advanced`,
`define_property`, `get_property`, `set_property`, `get_directory_property`,
`get_filename_component`, `set_package_properties`

**Control flow:** `foreach`, `block`

**String/list/file:** `string`, `list`, `file`, `cmake_path`, `cmake_language`,
`cmake_host_system_information`, `math`, `separate_arguments`

**Build & test:** `add_test`, `gtest_discover_tests`, `build_command`, `try_compile`, `try_run`,
`message`, `source_group`, `configure_file`, `include`, `add_subdirectory`,
`enable_language`, `load_cache`, `create_test_sourcelist`,
`include_external_msproject`, and the `ctest_*` family.

**Find modules:** `find_library`, `find_file`, `find_path`, `find_program`,
`cmake_parse_arguments`

### Simple commands (no keywords)

`add_compile_definitions`, `add_compile_options`, `add_definitions`, `add_dependencies`,
`add_link_options`, `aux_source_directory`, `enable_testing`, `fltk_wrap_ui`,
`get_source_file_property`, `get_target_property`, `get_test_property`,
`include_regular_expression`, `remove_definitions`

### Block closers

`endforeach`, `endfunction`, `endmacro`, `endblock` — accept 0 or 1 positional argument.

#!/usr/bin/env python3
"""Generate a large synthetic CMake fixture for benchmarking cmakefmt.

Deterministic output (no randomness), no external dependencies.
Produces a CMake file (~10,000+ lines) exercising as many formatting code
paths as possible, using deliberate anti-patterns so the formatter has real
work to do.

Usage:
    python3 scripts/generate_benchmark_fixture.py [output_path]

If output_path is omitted, writes to stdout.
"""

from __future__ import annotations

import sys
from typing import Iterator


def header() -> Iterator[str]:
    """cmake_minimum_required, project, top-level comments."""
    yield "# ============================================================================"
    yield "# Synthetic CMake benchmark fixture"
    yield "# Exercises all formatting code paths for cmakefmt"
    yield "# ============================================================================"
    yield ""
    yield "CMAKE_MINIMUM_REQUIRED(  VERSION  3.28  FATAL_ERROR )"
    yield ""
    yield "PROJECT( SyntheticBenchmark"
    yield "  VERSION 1.2.3.4"
    yield '  DESCRIPTION "A synthetic benchmark fixture for cmakefmt"'
    yield '  HOMEPAGE_URL "https://example.com/synthetic"'
    yield "  LANGUAGES C CXX ASM )"
    yield ""


def options_and_vars() -> Iterator[str]:
    """option(), set() with CACHE, consecutive set() calls for alignment."""
    yield "# --------------------------------------------------------------------------"
    yield "# Options and variables"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # option() calls with mixed casing
    for i in range(20):
        case = ["option", "OPTION", "Option"][i % 3]
        yield f'{case}(SYNTHETIC_OPTION_{i:02d} "Option number {i} for benchmark" {"ON" if i % 2 == 0 else "OFF"})'
    yield ""
    # Consecutive set() calls for alignment testing
    yield "# Consecutive set() calls for alignment"
    for i in range(30):
        name = f"VAR_{chr(65 + i % 26)}{'_LONG' if i % 5 == 0 else ''}{'_EXTRA_LONG_NAME' if i % 7 == 0 else ''}"
        val = f'"value_{i}"' if i % 3 != 0 else f"{i * 100}"
        case = ["set", "SET", "Set"][i % 3]
        yield f"{case}({name}  {val})"
    yield ""
    # CACHE variables
    yield "# Cache variables with types"
    cache_types = ["STRING", "BOOL", "PATH", "FILEPATH", "INTERNAL"]
    for i in range(15):
        ct = cache_types[i % len(cache_types)]
        yield f'SET(SYNTHETIC_CACHE_{i:02d} "default_{i}" CACHE {ct} "Cache variable {i}"  FORCE)'
    yield ""
    # PARENT_SCOPE and ENV variables
    yield "SET(SYNTHETIC_PARENT_VAR  ${SOME_VALUE}  PARENT_SCOPE)"
    yield "SET( ENV{SYNTHETIC_ENV_VAR}  $ENV{PATH}:/opt/synthetic )"
    yield ""


def find_packages() -> Iterator[str]:
    """find_package with COMPONENTS, REQUIRED, version constraints."""
    yield "# --------------------------------------------------------------------------"
    yield "# Package discovery"
    yield "# --------------------------------------------------------------------------"
    yield ""
    packages = [
        (
            "Boost",
            "1.84",
            ["filesystem", "system", "thread", "program_options", "regex"],
        ),
        ("Qt6", "6.5", ["Core", "Gui", "Widgets", "Network", "Sql", "Xml"]),
        ("OpenCV", "4.8", ["core", "imgproc", "highgui", "videoio", "calib3d"]),
        ("Protobuf", "3.21", []),
        ("gRPC", "1.50", []),
        ("CUDA", "12.0", []),
        ("OpenSSL", "3.0", []),
        ("ZLIB", None, []),
        ("Threads", None, []),
    ]
    for pkg, ver, comps in packages:
        case = ["FIND_PACKAGE", "find_package", "Find_Package"][hash(pkg) % 3]
        parts = [f"{case}({pkg}"]
        if ver:
            parts.append(f"  {ver}")
        parts.append("  REQUIRED")
        if comps:
            parts.append("  COMPONENTS")
            for c in comps:
                parts.append(f"    {c}")
        parts.append(")")
        yield from parts
    yield ""
    # find_library, find_path, find_program, find_file
    yield "FIND_LIBRARY(SYNTHETIC_LIB synthetic HINTS ${CMAKE_PREFIX_PATH}/lib  /usr/lib  /usr/local/lib  REQUIRED)"
    yield "find_path( SYNTHETIC_INCLUDE_DIR  synthetic.h  HINTS  ${CMAKE_PREFIX_PATH}/include  /usr/include )"
    yield "Find_Program(SYNTHETIC_TOOL syntool HINTS ${CMAKE_PREFIX_PATH}/bin  /usr/bin)"
    yield "FIND_FILE(SYNTHETIC_CONFIG  synthetic.cfg  HINTS  ${CMAKE_PREFIX_PATH}/etc  /etc )"
    yield ""


def string_operations() -> Iterator[str]:
    """string(REGEX, APPEND, REPLACE, etc.)."""
    yield "# --------------------------------------------------------------------------"
    yield "# String operations"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Various string subcommands
    yield 'STRING(  REGEX MATCH "([0-9]+)\\.([0-9]+)\\.([0-9]+)" SYNTHETIC_VERSION_MATCH ${SYNTHETIC_VERSION_STRING} )'
    yield 'STRING(REGEX  MATCHALL "[a-zA-Z_][a-zA-Z0-9_]*" SYNTHETIC_IDENTIFIERS "${SYNTHETIC_SOURCE_TEXT}")'
    yield 'string(REGEX REPLACE "([^;]+)" "prefix_\\1" SYNTHETIC_PREFIXED ${SYNTHETIC_LIST}  )'
    yield 'STRING(  REPLACE "old_pattern" "new_pattern" SYNTHETIC_REPLACED "${SYNTHETIC_INPUT}" )'
    yield 'string( APPEND SYNTHETIC_BUFFER  "first part "  "second part "  "third part")'
    yield 'STRING(PREPEND SYNTHETIC_BUFFER "header: ")'
    yield 'string(CONCAT SYNTHETIC_FULL  "part1"  "_"  "part2"  "_"  "part3")'
    yield 'STRING(  JOIN ";" SYNTHETIC_JOINED item1 item2 item3 item4 item5 item6 item7 item8 item9 item10 )'
    yield "string(TOLOWER ${SYNTHETIC_MIXED} SYNTHETIC_LOWER)"
    yield "STRING(TOUPPER ${SYNTHETIC_MIXED}  SYNTHETIC_UPPER )"
    yield "string(LENGTH ${SYNTHETIC_INPUT} SYNTHETIC_LEN)"
    yield "STRING(SUBSTRING ${SYNTHETIC_INPUT} 0 10 SYNTHETIC_SUB)"
    yield "string(STRIP ${SYNTHETIC_INPUT} SYNTHETIC_STRIPPED)"
    yield 'STRING(REPEAT  "abc"  10  SYNTHETIC_REPEATED)'
    yield 'string(  CONFIGURE "config @VAR@ done" SYNTHETIC_CONFIGURED )'
    yield 'STRING( MAKE_C_IDENTIFIER "some-header.h" SYNTHETIC_C_ID )'
    yield 'string(RANDOM LENGTH 32 ALPHABET "abcdef0123456789" SYNTHETIC_RANDOM)'
    yield 'STRING( COMPARE LESS "aaa" "bbb" SYNTHETIC_CMP_RESULT)'
    yield 'string( TIMESTAMP SYNTHETIC_TIMESTAMP "%Y-%m-%d" UTC)'
    yield 'STRING( UUID SYNTHETIC_UUID NAMESPACE "6ba7b810-9dad-11d1-80b4-00c04fd430c8" NAME "synthetic" TYPE SHA1)'
    yield ""


def list_operations() -> Iterator[str]:
    """list(APPEND, SORT, FILTER, etc.)."""
    yield "# --------------------------------------------------------------------------"
    yield "# List operations"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "LIST( LENGTH SYNTHETIC_SOURCES SYNTHETIC_SRC_COUNT )"
    yield "LIST( GET SYNTHETIC_SOURCES  0  2  4  SYNTHETIC_SELECTED )"
    yield "list( APPEND SYNTHETIC_SOURCES"
    for i in range(25):
        yield f"  src/synthetic_{i:03d}.cpp"
    yield ")"
    yield ""
    yield 'list( JOIN SYNTHETIC_SOURCES "," SYNTHETIC_CSV )'
    yield "LIST( SUBLIST SYNTHETIC_SOURCES  5  10  SYNTHETIC_SLICE )"
    yield "LIST(  FIND SYNTHETIC_SOURCES  src/synthetic_010.cpp  SYNTHETIC_IDX )"
    yield "list(INSERT SYNTHETIC_SOURCES 0 src/synthetic_new.cpp)"
    yield "LIST( REMOVE_ITEM SYNTHETIC_SOURCES src/synthetic_000.cpp src/synthetic_001.cpp )"
    yield "list(REMOVE_AT SYNTHETIC_SOURCES 0 1 2)"
    yield "LIST( REMOVE_DUPLICATES  SYNTHETIC_SOURCES )"
    yield 'list( FILTER  SYNTHETIC_SOURCES  INCLUDE  REGEX  ".*\\.cpp$" )'
    yield 'LIST( FILTER SYNTHETIC_SOURCES EXCLUDE REGEX ".*_test\\.cpp$"  )'
    yield "LIST( SORT SYNTHETIC_SOURCES  COMPARE  NATURAL  CASE  INSENSITIVE  ORDER  ASCENDING )"
    yield "list( REVERSE SYNTHETIC_SOURCES )"
    yield "list(POP_BACK SYNTHETIC_SOURCES SYNTHETIC_LAST)"
    yield "LIST(POP_FRONT SYNTHETIC_SOURCES SYNTHETIC_FIRST)"
    yield "list(  TRANSFORM SYNTHETIC_SOURCES  PREPEND  src/  OUTPUT_VARIABLE  SYNTHETIC_FULL_PATHS )"
    yield "LIST(TRANSFORM SYNTHETIC_SOURCES TOUPPER)"
    yield ""


def file_operations() -> Iterator[str]:
    """file(GLOB, READ, WRITE, GENERATE, etc.)."""
    yield "# --------------------------------------------------------------------------"
    yield "# File operations"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "FILE(  GLOB  SYNTHETIC_GLOB_SOURCES"
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"'
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c"'
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h" )'
    yield ""
    yield "file( GLOB_RECURSE SYNTHETIC_ALL_SOURCES"
    yield "  CONFIGURE_DEPENDS"
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"'
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h"'
    yield ")"
    yield ""
    yield 'FILE( READ "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" SYNTHETIC_VER_CONTENT )'
    yield "file(  STRINGS ${CMAKE_CURRENT_SOURCE_DIR}/config.txt SYNTHETIC_CONFIG_LINES )"
    yield 'FILE(  WRITE "${CMAKE_BINARY_DIR}/generated.h" "#pragma once\\n// Generated\\n" )'
    yield 'file( APPEND "${CMAKE_BINARY_DIR}/generated.h" "#define SYNTHETIC 1\\n")'
    yield ""
    yield "FILE(GENERATE"
    yield '  OUTPUT "${CMAKE_BINARY_DIR}/config_$<CONFIG>.h"'
    yield '  CONTENT "#define BUILD_TYPE \\"$<CONFIG>\\"\\n"'
    yield "  CONDITION $<BOOL:${SYNTHETIC_GENERATE}>"
    yield ")"
    yield ""
    yield 'file( COPY "${CMAKE_CURRENT_SOURCE_DIR}/data" DESTINATION "${CMAKE_BINARY_DIR}"'
    yield "  FILE_PERMISSIONS OWNER_READ OWNER_WRITE GROUP_READ WORLD_READ"
    yield "  FILES_MATCHING"
    yield '  PATTERN "*.dat"'
    yield '  PATTERN ".svn" EXCLUDE )'
    yield ""
    yield 'FILE(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/output")'
    yield "file(RELATIVE_PATH SYNTHETIC_REL ${CMAKE_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR})"
    yield 'FILE( TO_CMAKE_PATH "${SYNTHETIC_NATIVE_PATH}" SYNTHETIC_CMAKE_PATH )'
    yield 'file( TO_NATIVE_PATH "${SYNTHETIC_CMAKE_PATH}" SYNTHETIC_NATIVE )'
    yield 'FILE(DOWNLOAD "https://example.com/data.tar.gz" "${CMAKE_BINARY_DIR}/data.tar.gz"'
    yield "  TIMEOUT 60"
    yield "  EXPECTED_HASH SHA256=abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
    yield "  TLS_VERIFY ON"
    yield "  STATUS SYNTHETIC_DL_STATUS )"
    yield ""
    yield 'file(ARCHIVE_CREATE OUTPUT "${CMAKE_BINARY_DIR}/archive.tar.gz"'
    yield "  PATHS ${SYNTHETIC_SOURCES}"
    yield "  FORMAT gnutar"
    yield "  COMPRESSION GZip"
    yield "  COMPRESSION_LEVEL 9 )"
    yield ""
    yield 'FILE(  SIZE "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" SYNTHETIC_FILE_SIZE )'
    yield 'file(SHA256 "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" SYNTHETIC_FILE_HASH)'
    yield ""


def math_operations() -> Iterator[str]:
    """math(EXPR ...)."""
    yield "# --------------------------------------------------------------------------"
    yield "# Math operations"
    yield "# --------------------------------------------------------------------------"
    yield ""
    for i in range(10):
        op = ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"][i]
        yield f'MATH(  EXPR  SYNTHETIC_RESULT_{i}  "${{SYNTHETIC_A}} {op} ${{SYNTHETIC_B}}" )'
    yield 'math( EXPR SYNTHETIC_HEX "0xff + 1" OUTPUT_FORMAT HEXADECIMAL )'
    yield ""


def _nested_if_block(depth: int, idx: int) -> Iterator[str]:
    """Generate a nested if/elseif/else/endif block."""
    indent = "  " * depth
    conds = [
        f"SYNTHETIC_COND_{idx}_A",
        f'SYNTHETIC_VAR_{idx} STREQUAL "value_{idx}"',
        f"SYNTHETIC_NUM_{idx} GREATER 100",
        f"DEFINED SYNTHETIC_DEF_{idx}",
        f"EXISTS ${{CMAKE_CURRENT_SOURCE_DIR}}/file_{idx}.txt",
        f'SYNTHETIC_LIST_{idx} MATCHES "^pattern"',
        f"TARGET synthetic_target_{idx}",
        f"SYNTHETIC_A_{idx} VERSION_GREATER_EQUAL 1.2.3",
    ]
    cond = conds[idx % len(conds)]
    yield f"{indent}IF( {cond} )"
    yield f'{indent}  MESSAGE(STATUS "Condition {idx} depth {depth} met"  )'
    if depth < 4:
        yield from _nested_if_block(depth + 1, idx * 2 + 1)
    yield f"{indent}ELSEIF(  NOT {cond}  AND  SYNTHETIC_FALLBACK_{idx} )"
    yield f'{indent}  message(  WARNING  "Fallback {idx} depth {depth}" )'
    if depth < 3:
        yield from _nested_if_block(depth + 1, idx * 2 + 2)
    yield f"{indent}ELSE()"
    yield f'{indent}  MESSAGE(  AUTHOR_WARNING  "Neither condition for {idx}"  )'
    yield f"{indent}ENDIF()"


def flow_control() -> Iterator[str]:
    """Nested if/foreach/while/function/macro/block."""
    yield "# --------------------------------------------------------------------------"
    yield "# Flow control (nested)"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Nested if blocks
    for i in range(8):
        yield from _nested_if_block(0, i)
        yield ""

    # foreach loops - various forms
    yield "FOREACH(  item  IN  LISTS  SYNTHETIC_SOURCES )"
    yield '  MESSAGE( STATUS  "Processing: ${item}" )'
    yield "ENDFOREACH()"
    yield ""
    yield "foreach( idx RANGE 0 99 )"
    yield '  MATH(EXPR remainder "${idx} % 10")'
    yield "  IF(  remainder EQUAL 0  )"
    yield '    message(STATUS "Milestone: ${idx}"  )'
    yield "  ENDIF()"
    yield "ENDFOREACH()"
    yield ""
    yield "FOREACH(  item  IN  ITEMS  alpha  bravo  charlie  delta  echo  foxtrot )"
    yield '  message(STATUS "Item: ${item}")'
    yield "endforeach()"
    yield ""
    yield "foreach(  key  value  IN ZIP_LISTS  SYNTHETIC_KEYS  SYNTHETIC_VALUES )"
    yield '  SET(  RESULT_${key}  "${value}"  )'
    yield "ENDFOREACH()"
    yield ""

    # while loop
    yield "SET(SYNTHETIC_COUNTER  0)"
    yield "WHILE(  SYNTHETIC_COUNTER LESS 100  )"
    yield '  MATH( EXPR  SYNTHETIC_COUNTER  "${SYNTHETIC_COUNTER} + 1" )'
    yield "  IF(SYNTHETIC_COUNTER GREATER 50)"
    yield "    BREAK()"
    yield "  ENDIF()"
    yield "  IF(SYNTHETIC_COUNTER LESS 10)"
    yield "    CONTINUE()"
    yield "  ENDIF()"
    yield "ENDWHILE()"
    yield ""

    # function definitions
    for i in range(5):
        yield f"FUNCTION(  synthetic_function_{i}  target_name  )"
        yield f"  SET(local_var_{i} ${{ARGN}})"
        yield f'  MESSAGE(STATUS "Function {i}: ${{target_name}}")'
        yield f"  IF(  DEFINED local_var_{i} )"
        yield f'    MESSAGE(STATUS "  args: ${{local_var_{i}}}"  )'
        yield f"  ENDIF()"
        yield f"  SET(${{target_name}}_PROCESSED TRUE PARENT_SCOPE)"
        yield f"ENDFUNCTION()"
        yield ""

    # macro definitions
    for i in range(5):
        yield f"MACRO(  synthetic_macro_{i}  first_arg  )"
        yield f'  SET(SYNTHETIC_MACRO_RESULT_{i} "${{first_arg}}_processed")'
        yield f"  IF(ARGC GREATER 1)"
        yield f"    LIST(  APPEND  SYNTHETIC_MACRO_RESULT_{i}  ${{ARGN}}  )"
        yield f"  ENDIF()"
        yield f"ENDMACRO()"
        yield ""

    # block() - CMake 3.25+
    yield "BLOCK(  SCOPE_FOR  VARIABLES  POLICIES  )"
    yield '  SET(BLOCK_LOCAL_VAR "only visible in block")'
    yield "  CMAKE_POLICY(SET CMP0077 NEW)"
    yield "  message(STATUS  ${BLOCK_LOCAL_VAR})"
    yield "ENDBLOCK()"
    yield ""


def _source_list(prefix: str, count: int, ext: str = "cpp") -> Iterator[str]:
    """Generate a list of source file paths."""
    for i in range(count):
        # Deliberately unsorted for sorting tests
        idx = (i * 7 + 3) % count  # Scramble order
        yield f"    {prefix}/file_{idx:04d}.{ext}"


def libraries() -> Iterator[str]:
    """add_library with long source lists, genex, mixed case."""
    yield "# --------------------------------------------------------------------------"
    yield "# Library targets"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Static library with large unsorted source list
    yield "ADD_LIBRARY(  synthetic_core  STATIC"
    yield from _source_list("src/core", 40, "cpp")
    yield from _source_list("src/core", 20, "c")
    yield from _source_list("include/core", 30, "h")
    yield ")"
    yield ""
    # Shared library
    yield "add_library( synthetic_shared SHARED"
    yield from _source_list("src/shared", 35, "cpp")
    yield ")"
    yield ""
    # Object library
    yield "Add_Library(synthetic_objects OBJECT"
    yield from _source_list("src/objects", 25, "cpp")
    yield ")"
    yield ""
    # Interface library
    yield "ADD_LIBRARY(synthetic_interface INTERFACE)"
    yield ""
    # Imported library
    yield "ADD_LIBRARY(synthetic_imported  SHARED  IMPORTED  GLOBAL)"
    yield ""
    # ALIAS
    yield "add_library(  Synthetic::Core  ALIAS  synthetic_core  )"
    yield "ADD_LIBRARY(  Synthetic::Shared  ALIAS  synthetic_shared  )"
    yield ""
    # Library with generator expressions in sources
    yield "add_library(synthetic_genex_sources"
    yield "  $<$<PLATFORM_ID:Linux>:src/platform/linux.cpp>"
    yield "  $<$<PLATFORM_ID:Windows>:src/platform/windows.cpp>"
    yield "  $<$<PLATFORM_ID:Darwin>:src/platform/macos.cpp>"
    yield "  src/platform/common.cpp"
    yield ")"
    yield ""


def executables() -> Iterator[str]:
    """add_executable with sorted sources."""
    yield "# --------------------------------------------------------------------------"
    yield "# Executable targets"
    yield "# --------------------------------------------------------------------------"
    yield ""
    for i in range(8):
        case = ["ADD_EXECUTABLE", "add_executable", "Add_Executable"][i % 3]
        yield f"{case}(  synthetic_app_{i}"
        yield from _source_list(f"src/app{i}", 20 + i * 3, "cpp")
        yield ")"
        yield ""

    # WIN32/MACOSX_BUNDLE variants
    yield "ADD_EXECUTABLE(  synthetic_win32  WIN32"
    yield "  src/main_win32.cpp"
    yield "  src/resource.rc"
    yield ")"
    yield ""
    yield "add_executable(  synthetic_bundle  MACOSX_BUNDLE"
    yield "  src/main_bundle.cpp"
    yield "  resources/Info.plist"
    yield ")"
    yield ""
    # IMPORTED executable
    yield "ADD_EXECUTABLE(  synthetic_tool_imported  IMPORTED )"
    yield ""


def target_props() -> Iterator[str]:
    """set_target_properties, target_compile_*, target_link_*."""
    yield "# --------------------------------------------------------------------------"
    yield "# Target properties"
    yield "# --------------------------------------------------------------------------"
    yield ""
    targets = ["synthetic_core", "synthetic_shared", "synthetic_objects"]

    # set_target_properties with PROPERTIES pairs
    for t in targets:
        yield f"SET_TARGET_PROPERTIES(  {t}  PROPERTIES"
        yield "  CXX_STANDARD 20"
        yield "  CXX_STANDARD_REQUIRED ON"
        yield "  CXX_EXTENSIONS OFF"
        yield "  POSITION_INDEPENDENT_CODE ON"
        yield '  OUTPUT_NAME "synthetic"'
        yield '  VERSION "${PROJECT_VERSION}"'
        yield '  SOVERSION "${PROJECT_VERSION_MAJOR}"'
        yield '  EXPORT_NAME "Synthetic"'
        yield "  LINKER_LANGUAGE CXX"
        yield '  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"'
        yield '  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"'
        yield '  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"'
        yield ")"
        yield ""

    # target_compile_definitions
    for t in targets:
        yield f"TARGET_COMPILE_DEFINITIONS( {t}"
        yield "  PUBLIC"
        yield "    SYNTHETIC_VERSION_MAJOR=${PROJECT_VERSION_MAJOR}"
        yield "    SYNTHETIC_VERSION_MINOR=${PROJECT_VERSION_MINOR}"
        yield "    $<$<CONFIG:Debug>:SYNTHETIC_DEBUG=1>"
        yield "    $<$<CONFIG:Release>:SYNTHETIC_NDEBUG=1>"
        yield "  PRIVATE"
        yield "    SYNTHETIC_INTERNAL=1"
        yield "    $<$<BOOL:${SYNTHETIC_OPTION_00}>:SYNTHETIC_FEATURE_A>"
        yield "  INTERFACE"
        yield "    SYNTHETIC_API=1"
        yield ")"
        yield ""

    # target_compile_options with conditions
    for t in targets:
        yield f"target_compile_options(  {t}"
        yield "  PUBLIC"
        yield "    $<$<CXX_COMPILER_ID:GNU>:-Wall -Wextra -Wpedantic>"
        yield "    $<$<CXX_COMPILER_ID:Clang>:-Wall -Wextra -Wpedantic -Wno-unused-parameter>"
        yield "    $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>"
        yield "  PRIVATE"
        yield "    $<$<CONFIG:Debug>:-O0 -g>"
        yield "    $<$<CONFIG:Release>:-O3 -DNDEBUG>"
        yield "    $<$<CONFIG:RelWithDebInfo>:-O2 -g>"
        yield ")"
        yield ""

    # target_compile_features
    yield "target_compile_features(synthetic_core PUBLIC cxx_std_20)"
    yield "TARGET_COMPILE_FEATURES(synthetic_shared  PUBLIC  cxx_std_17  PRIVATE  cxx_std_20)"
    yield ""

    # target_include_directories
    for t in targets:
        yield f"TARGET_INCLUDE_DIRECTORIES(  {t}"
        yield "  PUBLIC"
        yield "    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>"
        yield "    $<INSTALL_INTERFACE:include>"
        yield "  PRIVATE"
        yield "    ${CMAKE_CURRENT_SOURCE_DIR}/src"
        yield "    ${CMAKE_CURRENT_BINARY_DIR}"
        yield "  SYSTEM INTERFACE"
        yield "    ${Boost_INCLUDE_DIRS}"
        yield ")"
        yield ""

    # target_link_libraries with sections
    for t in targets:
        yield f"target_link_libraries( {t}"
        yield "  PUBLIC"
        yield "    Boost::filesystem"
        yield "    Boost::system"
        yield "    ${OpenSSL_LIBRARIES}"
        yield "  PRIVATE"
        yield "    ZLIB::ZLIB"
        yield "    Threads::Threads"
        yield "    $<$<PLATFORM_ID:Linux>:dl>"
        yield "    $<$<PLATFORM_ID:Linux>:rt>"
        yield "  INTERFACE"
        yield "    synthetic_interface"
        yield ")"
        yield ""

    # target_link_options
    yield "TARGET_LINK_OPTIONS(  synthetic_shared"
    yield "  PRIVATE"
    yield "    $<$<CXX_COMPILER_ID:GNU>:-Wl,--as-needed>"
    yield "    $<$<PLATFORM_ID:Linux>:-Wl,-rpath,$ORIGIN/../lib>"
    yield ")"
    yield ""

    # target_link_directories
    yield "target_link_directories(  synthetic_core  PRIVATE"
    yield '  "${CMAKE_BINARY_DIR}/deps/lib"'
    yield '  "${CMAKE_PREFIX_PATH}/lib"'
    yield ")"
    yield ""

    # target_precompile_headers
    yield "TARGET_PRECOMPILE_HEADERS( synthetic_core"
    yield "  PUBLIC"
    yield "    <vector>"
    yield "    <string>"
    yield "    <memory>"
    yield "    <unordered_map>"
    yield "  PRIVATE"
    yield "    <iostream>"
    yield "    <fstream>"
    yield '    "src/internal.h"'
    yield ")"
    yield ""

    # target_sources
    yield "TARGET_SOURCES(  synthetic_core"
    yield "  PUBLIC"
    yield "    FILE_SET HEADERS"
    yield "    BASE_DIRS include"
    yield "    FILES"
    yield "      include/synthetic/core.h"
    yield "      include/synthetic/types.h"
    yield "      include/synthetic/utils.h"
    yield "  PRIVATE"
    yield "    src/internal_impl.cpp"
    yield ")"
    yield ""


def install_rules() -> Iterator[str]:
    """install(TARGETS, FILES, DIRECTORY) with sections."""
    yield "# --------------------------------------------------------------------------"
    yield "# Install rules"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "INSTALL(  TARGETS"
    yield "  synthetic_core"
    yield "  synthetic_shared"
    yield "  EXPORT SyntheticTargets"
    yield "  RUNTIME DESTINATION  ${CMAKE_INSTALL_BINDIR}"
    yield "    COMPONENT Runtime"
    yield "  LIBRARY DESTINATION  ${CMAKE_INSTALL_LIBDIR}"
    yield "    COMPONENT Runtime"
    yield "    NAMELINK_COMPONENT Development"
    yield "  ARCHIVE DESTINATION  ${CMAKE_INSTALL_LIBDIR}"
    yield "    COMPONENT Development"
    yield "  INCLUDES DESTINATION  ${CMAKE_INSTALL_INCLUDEDIR}"
    yield "  PUBLIC_HEADER DESTINATION  ${CMAKE_INSTALL_INCLUDEDIR}/synthetic"
    yield "    COMPONENT Development"
    yield ")"
    yield ""
    yield "install(  FILES"
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/core.h"'
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/types.h"'
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/utils.h"'
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/config.h"'
    yield '  DESTINATION  "${CMAKE_INSTALL_INCLUDEDIR}/synthetic"'
    yield "  COMPONENT  Development"
    yield ")"
    yield ""
    yield "INSTALL(  DIRECTORY"
    yield '  "${CMAKE_CURRENT_SOURCE_DIR}/include/"'
    yield '  DESTINATION  "${CMAKE_INSTALL_INCLUDEDIR}"'
    yield "  COMPONENT  Development"
    yield '  FILES_MATCHING PATTERN "*.h"'
    yield '  PATTERN ".svn" EXCLUDE'
    yield '  PATTERN "internal" EXCLUDE'
    yield ")"
    yield ""
    yield "install(  EXPORT  SyntheticTargets"
    yield "  FILE  SyntheticTargets.cmake"
    yield "  NAMESPACE  Synthetic::"
    yield '  DESTINATION  "${CMAKE_INSTALL_LIBDIR}/cmake/Synthetic"'
    yield "  COMPONENT  Development"
    yield ")"
    yield ""
    # configure_file for package config
    yield 'CONFIGURE_FILE(  "${CMAKE_CURRENT_SOURCE_DIR}/cmake/SyntheticConfig.cmake.in"'
    yield '  "${CMAKE_CURRENT_BINARY_DIR}/SyntheticConfig.cmake"'
    yield "  @ONLY )"
    yield ""
    yield "install(  FILES"
    yield '  "${CMAKE_CURRENT_BINARY_DIR}/SyntheticConfig.cmake"'
    yield '  "${CMAKE_CURRENT_BINARY_DIR}/SyntheticConfigVersion.cmake"'
    yield '  DESTINATION  "${CMAKE_INSTALL_LIBDIR}/cmake/Synthetic"'
    yield "  COMPONENT  Development"
    yield ")"
    yield ""


def custom_commands() -> Iterator[str]:
    """add_custom_command, add_custom_target."""
    yield "# --------------------------------------------------------------------------"
    yield "# Custom commands and targets"
    yield "# --------------------------------------------------------------------------"
    yield ""
    for i in range(6):
        yield f"ADD_CUSTOM_COMMAND("
        yield f'  OUTPUT "${{CMAKE_BINARY_DIR}}/generated/synthetic_{i:02d}.cpp"'
        yield f'  COMMAND ${{CMAKE_COMMAND}} -E echo "Generating synthetic_{i:02d}.cpp"'
        yield f'  COMMAND ${{Python3_EXECUTABLE}} "${{CMAKE_CURRENT_SOURCE_DIR}}/scripts/generate.py"'
        yield f'    --output "${{CMAKE_BINARY_DIR}}/generated/synthetic_{i:02d}.cpp"'
        yield f"    --index {i}"
        yield f"    --config ${{SYNTHETIC_CONFIG}}"
        yield f'  DEPENDS "${{CMAKE_CURRENT_SOURCE_DIR}}/scripts/generate.py"'
        yield f"    ${{SYNTHETIC_CONFIG}}"
        yield f'  WORKING_DIRECTORY "${{CMAKE_CURRENT_SOURCE_DIR}}"'
        yield f'  COMMENT "Generating synthetic_{i:02d}.cpp"'
        yield f"  VERBATIM )"
        yield ""

    # POST_BUILD command
    yield "ADD_CUSTOM_COMMAND(  TARGET  synthetic_core  POST_BUILD"
    yield '  COMMAND ${CMAKE_COMMAND} -E copy "$<TARGET_FILE:synthetic_core>" "${CMAKE_BINARY_DIR}/output/"'
    yield '  COMMENT "Copying synthetic_core to output"'
    yield "  VERBATIM"
    yield ")"
    yield ""

    # Custom targets
    for i in range(4):
        yield f"ADD_CUSTOM_TARGET(  synthetic_generate_{i}"
        yield f"  DEPENDS"
        for j in range(3):
            yield f'    "${{CMAKE_BINARY_DIR}}/generated/synthetic_{i * 3 + j:02d}.cpp"'
        yield f'  COMMENT "Generate batch {i}"'
        yield f")"
        yield ""

    yield "ADD_CUSTOM_TARGET( synthetic_all_generated  ALL"
    yield "  DEPENDS synthetic_generate_0 synthetic_generate_1 synthetic_generate_2"
    yield ")"
    yield ""


def testing() -> Iterator[str]:
    """enable_testing, add_test, gtest_discover_tests."""
    yield "# --------------------------------------------------------------------------"
    yield "# Testing"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "ENABLE_TESTING()"
    yield ""
    yield "INCLUDE(  GoogleTest  )"
    yield "INCLUDE(  CTest  )"
    yield ""
    for i in range(10):
        case = ["add_executable", "ADD_EXECUTABLE", "Add_Executable"][i % 3]
        yield f"{case}(  synthetic_test_{i}"
        yield from _source_list(f"tests/test{i}", 8 + i, "cpp")
        yield ")"
        yield ""
        yield f"target_link_libraries(  synthetic_test_{i}"
        yield "  PRIVATE"
        yield "    synthetic_core"
        yield "    GTest::gtest"
        yield "    GTest::gtest_main"
        yield "    GTest::gmock"
        yield ")"
        yield ""
        yield f"GTEST_DISCOVER_TESTS(  synthetic_test_{i}"
        yield '  WORKING_DIRECTORY  "${CMAKE_BINARY_DIR}"'
        yield f'  TEST_PREFIX  "synthetic_{i}::"'
        yield f"  DISCOVERY_TIMEOUT  120"
        yield f"  PROPERTIES"
        yield f'    LABELS "synthetic;unit"'
        yield f"    TIMEOUT  300"
        yield ")"
        yield ""

    # add_test with command-line args
    for i in range(5):
        yield f"ADD_TEST(  NAME  synthetic_integration_{i}"
        yield f"  COMMAND  synthetic_app_0"
        yield f'    --config "${{CMAKE_BINARY_DIR}}/test_config_{i}.json"'
        yield f"    --verbose"
        yield f"    --threads  4"
        yield f")"
        yield ""
        yield f"SET_TESTS_PROPERTIES(  synthetic_integration_{i}  PROPERTIES"
        yield f"  TIMEOUT 600"
        yield f'  LABELS "integration"'
        yield f'  ENVIRONMENT "SYNTHETIC_TEST=1;SYNTHETIC_IDX={i}"'
        yield f"  WORKING_DIRECTORY ${{CMAKE_BINARY_DIR}}"
        yield f")"
        yield ""


def genex_heavy() -> Iterator[str]:
    """Commands with complex generator expressions."""
    yield "# --------------------------------------------------------------------------"
    yield "# Generator expressions (complex)"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Nested genex
    yield "target_compile_definitions(  synthetic_core  PRIVATE"
    yield "  $<$<AND:$<CONFIG:Debug>,$<PLATFORM_ID:Linux>>:SYNTHETIC_DEBUG_LINUX>"
    yield "  $<$<AND:$<CONFIG:Debug>,$<PLATFORM_ID:Windows>>:SYNTHETIC_DEBUG_WINDOWS>"
    yield "  $<$<OR:$<CONFIG:Release>,$<CONFIG:RelWithDebInfo>>:SYNTHETIC_OPTIMIZED>"
    yield "  $<$<NOT:$<BOOL:${SYNTHETIC_MINIMAL}>>:SYNTHETIC_FULL_FEATURES>"
    yield "  $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:SYNTHETIC_DLL_EXPORT>"
    yield ")"
    yield ""

    # Complex genex in target_link_libraries
    yield "target_link_libraries(  synthetic_core  PRIVATE"
    yield "  $<$<AND:$<PLATFORM_ID:Linux>,$<CXX_COMPILER_ID:GNU>>:asan>"
    yield "  $<$<AND:$<PLATFORM_ID:Linux>,$<CXX_COMPILER_ID:GNU>>:ubsan>"
    yield "  $<IF:$<BOOL:${SYNTHETIC_USE_JEMALLOC}>,jemalloc,>"
    yield "  $<TARGET_OBJECTS:synthetic_objects>"
    yield ")"
    yield ""

    # Genex in install
    yield "INSTALL(  TARGETS  synthetic_core"
    yield "  RUNTIME  DESTINATION  $<IF:$<PLATFORM_ID:Windows>,bin,libexec>"
    yield "  LIBRARY  DESTINATION  $<IF:$<PLATFORM_ID:Windows>,bin,lib>"
    yield ")"
    yield ""

    # Genex in add_custom_command
    yield "ADD_CUSTOM_COMMAND(  TARGET  synthetic_core  POST_BUILD"
    yield '  COMMAND ${CMAKE_COMMAND} -E echo "Target file: $<TARGET_FILE:synthetic_core>"'
    yield '  COMMAND ${CMAKE_COMMAND} -E echo "Target dir:  $<TARGET_FILE_DIR:synthetic_core>"'
    yield '  COMMAND ${CMAKE_COMMAND} -E echo "Linker lang: $<TARGET_PROPERTY:synthetic_core,LINKER_LANGUAGE>"'
    yield "  VERBATIM"
    yield ")"
    yield ""

    # Deeply nested genex
    yield "SET(SYNTHETIC_COMPLEX_GENEX"
    yield "  $<$<AND:$<NOT:$<BOOL:${SYNTHETIC_DISABLE_FEATURE_A}>>,$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>>:FEATURE_A_DEBUG>"
    yield ")"
    yield ""

    # Genex in compile options
    yield "target_compile_options(  synthetic_shared  PRIVATE"
    yield "  $<$<AND:$<CXX_COMPILER_ID:GNU>,$<VERSION_GREATER_EQUAL:$<CXX_COMPILER_VERSION>,12.0>>:-Wno-dangling-reference>"
    yield "  $<$<AND:$<CXX_COMPILER_ID:Clang>,$<VERSION_GREATER_EQUAL:$<CXX_COMPILER_VERSION>,15.0>>:-Wno-unused-but-set-variable>"
    yield ")"
    yield ""

    # Multi-valued genex (semicolons separate list items inside genex)
    yield "SET(SYNTHETIC_PLATFORM_SOURCES"
    yield "  $<$<PLATFORM_ID:Linux>:src/platform/linux_impl.cpp;src/platform/linux_sysctl.cpp;src/platform/linux_epoll.cpp>"
    yield "  $<$<PLATFORM_ID:Windows>:src/platform/windows_impl.cpp;src/platform/windows_iocp.cpp>"
    yield "  $<$<PLATFORM_ID:Darwin>:src/platform/macos_impl.cpp;src/platform/macos_kqueue.cpp>"
    yield ")"
    yield ""


def comments_heavy() -> Iterator[str]:
    """Long prose comments, bracket comments, code block comments."""
    yield "# --------------------------------------------------------------------------"
    yield "# Comments section"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Long prose comment for reflow testing
    yield "# This is a very long comment that should be reflowed by the formatter because it exceeds the configured comment width of 80 characters and needs to be wrapped properly. The formatter should handle this gracefully by breaking the text at word boundaries while preserving the comment prefix."
    yield ""
    yield "# Another long comment for reflow: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
    yield ""
    # Multiple short comments
    yield "# Short comment 1"
    yield "# Short comment 2"
    yield "# Short comment 3"
    yield ""
    # Bracket comment
    yield "#[[This is a bracket comment."
    yield "It can span multiple lines."
    yield "The formatter should preserve it verbatim."
    yield "No reflow should happen here.]]"
    yield ""
    # Bracket comment with =
    yield "#[==[This is a bracket comment with equals."
    yield "It uses level-2 bracket syntax."
    yield "Content here is also preserved verbatim."
    yield "]==]"
    yield ""
    # Inline trailing comments for alignment testing
    yield "SET(COMMENT_ALIGN_A  value_a)  # first value"
    yield "SET(COMMENT_ALIGN_BB  value_bb)  # second value"
    yield "SET(COMMENT_ALIGN_CCC  value_ccc)  # third value"
    yield "SET(COMMENT_ALIGN_DDDD  value_dddd)  # fourth value"
    yield "SET(COMMENT_ALIGN_EEEEE  value_eeeee)  # fifth value"
    yield ""
    # Comments with code blocks (should not be reflowed)
    yield "# Example usage:"
    yield "#"
    yield "#   cmake -B build -DSYNTHETIC_OPTION_00=ON"
    yield "#   cmake --build build --config Release"
    yield "#   ctest --test-dir build --output-on-failure"
    yield "#"
    yield "# End of example."
    yield ""
    # Comment before a command (preservation)
    yield "# This comment documents the following set() call"
    yield "SET(DOCUMENTED_VAR  42)"
    yield ""
    # Multi-paragraph comment
    yield "# First paragraph of a multi-paragraph comment that provides extensive"
    yield "# documentation about the module below. This should be reflowed as a"
    yield "# single block."
    yield "#"
    yield "# Second paragraph with different content. The formatter should preserve the paragraph break (blank comment line) while still reflowing each paragraph individually to fit within the configured comment width."
    yield "#"
    yield "# Third paragraph: implementation notes and caveats that developers should be aware of when modifying the code below. These are important to preserve."
    yield ""


def alignment_groups() -> Iterator[str]:
    """Consecutive set() blocks, PROPERTIES, arg groups."""
    yield "# --------------------------------------------------------------------------"
    yield "# Alignment groups"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Group 1: consecutive set() with varying name lengths
    for i in range(15):
        name_parts = [
            "SHORT",
            "MEDIUM_LEN",
            "VERY_LONG_VARIABLE_NAME",
            "X",
            "SOMEWHAT_LONG",
        ][i % 5]
        yield f'SET({name_parts}_{i}  "value_{i}")'
    yield ""
    # Group 2: separated by blank line (should NOT align across gap)
    yield 'SET(GROUP2_A  "alpha")'
    yield 'SET(GROUP2_BB  "bravo")'
    yield 'SET(GROUP2_CCC  "charlie")'
    yield ""
    yield 'SET(GROUP3_X  "x-ray")'
    yield 'SET(GROUP3_YY  "yankee")'
    yield 'SET(GROUP3_ZZZ  "zulu")'
    yield ""

    # PROPERTIES alignment via set_target_properties
    yield "SET_TARGET_PROPERTIES(  synthetic_core  PROPERTIES"
    yield "  A  value_a"
    yield "  BB  value_bb"
    yield "  CCC  value_ccc"
    yield "  DDDDDDDD  value_dddddddd"
    yield "  E  value_e"
    yield ")"
    yield ""

    # Keyword-grouped args for alignment
    yield "target_link_libraries(  synthetic_shared"
    yield "  PUBLIC"
    yield "    Boost::filesystem"
    yield "    Boost::system"
    yield "    Boost::thread"
    yield "    Boost::program_options"
    yield "    Boost::regex"
    yield "  PRIVATE"
    yield "    ${OPENSSL_LIBRARIES}"
    yield "    ZLIB::ZLIB"
    yield "    Threads::Threads"
    yield "  INTERFACE"
    yield "    Boost::headers"
    yield ")"
    yield ""


def pragmas() -> Iterator[str]:
    """push/pop/off/on/skip regions."""
    yield "# --------------------------------------------------------------------------"
    yield "# Pragma regions"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # cmakefmt: off/on
    yield "# cmakefmt: off"
    yield "# This block is not formatted"
    yield "SET(  UNFORMATTED_VAR"
    yield "     value1     value2     value3"
    yield ")"
    yield "# cmakefmt: on"
    yield ""
    # cmakefmt: skip (single line)
    yield "# cmakefmt: skip"
    yield "SET(  SKIP_SINGLE_LINE    value1    value2  )"
    yield ""
    # cmakefmt: push/pop (change config temporarily)
    yield '# cmakefmt: push { commandCase = "upper" }'
    yield 'set(PUSH_TEST_VAR "this set should become uppercase")'
    yield "# cmakefmt: pop"
    yield ""
    # Another push/pop with different option
    yield '# cmakefmt: push { "lineWidth": 40 }'
    yield "set(NARROW_VAR value1 value2 value3 value4 value5 value6 value7 value8)"
    yield "# cmakefmt: pop"
    yield ""
    # Nested push/pop
    yield '# cmakefmt: push { "commandCase": "upper" }'
    yield 'set(OUTER_PUSH "should be uppercase")'
    yield '# cmakefmt: push { "keywordCase": "lower" }'
    yield "target_link_libraries(synthetic_core PUBLIC Boost::filesystem)"
    yield "# cmakefmt: pop"
    yield 'set(BACK_TO_OUTER "still uppercase")'
    yield "# cmakefmt: pop"
    yield ""
    # Multiple skips
    for i in range(5):
        yield "# cmakefmt: skip"
        yield f"SET(   SKIP_{i}    value_{i}   )"
    yield ""


def cmake_language_section() -> Iterator[str]:
    """cmake_language(CALL/EVAL/DEFER)."""
    yield "# --------------------------------------------------------------------------"
    yield "# cmake_language()"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield 'CMAKE_LANGUAGE(  CALL  message  STATUS  "Called via cmake_language"  )'
    yield ""
    yield 'cmake_language(  EVAL  CODE  "'
    yield '  message(STATUS \\"Evaluated code\\")'
    yield '")'
    yield ""
    yield "CMAKE_LANGUAGE(DEFER"
    yield '  CALL message STATUS "Deferred message"'
    yield ")"
    yield ""
    yield "cmake_language(  DEFER  DIRECTORY  ${CMAKE_CURRENT_SOURCE_DIR}"
    yield '  CALL  message  STATUS  "Deferred to directory"'
    yield ")"
    yield ""
    yield "CMAKE_LANGUAGE(  DEFER  ID  synthetic_defer_id"
    yield "  CALL  synthetic_cleanup"
    yield ")"
    yield ""
    yield "CMAKE_LANGUAGE( DEFER  ID_VAR  SYNTHETIC_DEFER_VARIABLE"
    yield "  CALL  synthetic_late_init"
    yield ")"
    yield ""
    yield "CMAKE_LANGUAGE(  DEFER GET_CALL_IDS  SYNTHETIC_ALL_DEFERS )"
    yield "CMAKE_LANGUAGE(  DEFER CANCEL_CALL  synthetic_defer_id )"
    yield ""


def cmake_path_section() -> Iterator[str]:
    """cmake_path()."""
    yield "# --------------------------------------------------------------------------"
    yield "# cmake_path()"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield 'CMAKE_PATH(  SET  SYNTHETIC_PATH  NORMALIZE  "/usr/local/bin/../lib"  )'
    yield 'cmake_path(  APPEND  SYNTHETIC_PATH  "subdir"  "file.txt"  OUTPUT_VARIABLE  SYNTHETIC_FULL_PATH )'
    yield "CMAKE_PATH(  GET  SYNTHETIC_PATH  ROOT_NAME  SYNTHETIC_ROOT )"
    yield "cmake_path(  GET  SYNTHETIC_PATH  FILENAME  SYNTHETIC_FILENAME )"
    yield "CMAKE_PATH(  GET  SYNTHETIC_PATH  EXTENSION  LAST_ONLY  SYNTHETIC_EXT )"
    yield "cmake_path(  GET  SYNTHETIC_PATH  STEM  SYNTHETIC_STEM )"
    yield "CMAKE_PATH(  GET  SYNTHETIC_PATH  PARENT_PATH  SYNTHETIC_PARENT )"
    yield 'cmake_path(  REPLACE_FILENAME  SYNTHETIC_PATH  "new_file.txt" )'
    yield 'CMAKE_PATH(  REPLACE_EXTENSION  SYNTHETIC_PATH  ".hpp" )'
    yield "cmake_path(  REMOVE_FILENAME  SYNTHETIC_PATH )"
    yield "cmake_path(  REMOVE_EXTENSION  SYNTHETIC_PATH  LAST_ONLY )"
    yield "CMAKE_PATH(  COMPARE  SYNTHETIC_PATH  EQUAL  SYNTHETIC_OTHER  SYNTHETIC_PATHS_EQUAL )"
    yield "cmake_path(  HAS_ROOT_NAME  SYNTHETIC_PATH  SYNTHETIC_HAS_ROOT )"
    yield "CMAKE_PATH(  IS_RELATIVE  SYNTHETIC_PATH  SYNTHETIC_IS_REL )"
    yield "cmake_path(  NATIVE_PATH  SYNTHETIC_PATH  NORMALIZE  SYNTHETIC_NATIVE_P )"
    yield 'cmake_path(  CONVERT  "${SYNTHETIC_NATIVE_P}"  TO_CMAKE_PATH_LIST  SYNTHETIC_CONVERTED  NORMALIZE )'
    yield ""


def execute_process_section() -> Iterator[str]:
    """execute_process()."""
    yield "# --------------------------------------------------------------------------"
    yield "# execute_process()"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "EXECUTE_PROCESS("
    yield "  COMMAND  git rev-parse HEAD"
    yield "  WORKING_DIRECTORY  ${CMAKE_CURRENT_SOURCE_DIR}"
    yield "  OUTPUT_VARIABLE  SYNTHETIC_GIT_HASH"
    yield "  OUTPUT_STRIP_TRAILING_WHITESPACE"
    yield "  ERROR_QUIET"
    yield "  RESULT_VARIABLE  SYNTHETIC_GIT_RESULT"
    yield ")"
    yield ""
    yield "execute_process("
    yield '  COMMAND  ${Python3_EXECUTABLE}  -c  "import sys; print(sys.version)"'
    yield "  OUTPUT_VARIABLE  SYNTHETIC_PYTHON_VERSION"
    yield "  OUTPUT_STRIP_TRAILING_WHITESPACE"
    yield "  TIMEOUT  10"
    yield "  COMMAND_ERROR_IS_FATAL  ANY"
    yield ")"
    yield ""
    # Multi-command pipeline
    yield "EXECUTE_PROCESS("
    yield "  COMMAND  ${CMAKE_COMMAND}  -E  echo  hello"
    yield "  COMMAND  ${CMAKE_COMMAND}  -E  echo  world"
    yield "  OUTPUT_VARIABLE  SYNTHETIC_PIPELINE"
    yield "  ERROR_VARIABLE  SYNTHETIC_PIPELINE_ERR"
    yield "  INPUT_FILE  /dev/null"
    yield "  TIMEOUT  30"
    yield "  ENCODING  UTF-8"
    yield ")"
    yield ""


def try_compile_section() -> Iterator[str]:
    """try_compile() and try_run()."""
    yield "# --------------------------------------------------------------------------"
    yield "# try_compile() and try_run()"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "TRY_COMPILE(  SYNTHETIC_COMPILE_RESULT"
    yield '  SOURCE_FROM_CONTENT  test_feature.cpp  "int main() { return 0; }"'
    yield "  CXX_STANDARD  20"
    yield "  CXX_STANDARD_REQUIRED  TRUE"
    yield "  OUTPUT_VARIABLE  SYNTHETIC_COMPILE_OUTPUT"
    yield ")"
    yield ""
    yield "try_run(  SYNTHETIC_RUN_RESULT  SYNTHETIC_COMPILE_RESULT2"
    yield '  SOURCE_FROM_CONTENT  test_run.cpp  "#include <iostream>\\nint main() { std::cout << 42; }"'
    yield "  CXX_STANDARD  20"
    yield "  RUN_OUTPUT_VARIABLE  SYNTHETIC_RUN_OUTPUT"
    yield ")"
    yield ""


def cmake_parse_args_section() -> Iterator[str]:
    """cmake_parse_arguments()."""
    yield "# --------------------------------------------------------------------------"
    yield "# cmake_parse_arguments()"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "# Define a function that uses cmake_parse_arguments"
    yield "FUNCTION(  synthetic_add_module  module_name  )"
    yield "  CMAKE_PARSE_ARGUMENTS("
    yield "    PARSE_ARGV  1"
    yield "    ARG"
    yield '    "SHARED;STATIC;INTERFACE;EXCLUDE_FROM_ALL"'
    yield '    "OUTPUT_NAME;FOLDER;NAMESPACE"'
    yield '    "SOURCES;HEADERS;DEPENDS;COMPILE_FEATURES;COMPILE_DEFINITIONS"'
    yield "  )"
    yield ""
    yield "  IF(ARG_SHARED)"
    yield "    ADD_LIBRARY(${module_name}  SHARED  ${ARG_SOURCES})"
    yield "  ELSEIF(ARG_STATIC)"
    yield "    add_library(${module_name}  STATIC  ${ARG_SOURCES})"
    yield "  ELSEIF(ARG_INTERFACE)"
    yield "    ADD_LIBRARY(${module_name}  INTERFACE)"
    yield "  ELSE()"
    yield "    add_library(${module_name}  ${ARG_SOURCES})"
    yield "  ENDIF()"
    yield ""
    yield "  IF(DEFINED ARG_HEADERS)"
    yield "    TARGET_SOURCES(${module_name}  PUBLIC  ${ARG_HEADERS})"
    yield "  ENDIF()"
    yield ""
    yield "  IF(DEFINED ARG_DEPENDS)"
    yield "    target_link_libraries(${module_name}  PRIVATE  ${ARG_DEPENDS})"
    yield "  ENDIF()"
    yield ""
    yield "  IF(DEFINED ARG_COMPILE_FEATURES)"
    yield "    target_compile_features(${module_name}  PUBLIC  ${ARG_COMPILE_FEATURES})"
    yield "  ENDIF()"
    yield ""
    yield "  IF(DEFINED ARG_OUTPUT_NAME)"
    yield '    SET_TARGET_PROPERTIES(${module_name}  PROPERTIES  OUTPUT_NAME  "${ARG_OUTPUT_NAME}")'
    yield "  ENDIF()"
    yield ""
    yield "  IF(DEFINED ARG_FOLDER)"
    yield '    SET_TARGET_PROPERTIES(${module_name}  PROPERTIES  FOLDER  "${ARG_FOLDER}")'
    yield "  ENDIF()"
    yield "ENDFUNCTION()"
    yield ""


def bracket_args() -> Iterator[str]:
    """Bracket arguments and multiline quoted strings."""
    yield "# --------------------------------------------------------------------------"
    yield "# Bracket arguments and multiline strings"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Bracket argument
    yield "SET(SYNTHETIC_BRACKET_VAR  [==[This is a bracket argument."
    yield "It spans multiple lines."
    yield "Special chars: ]] ] [ [[ are fine here."
    yield 'Even "quotes" and ${variables} are literal.'
    yield "]==])"
    yield ""
    # Multiline quoted string
    yield 'SET(SYNTHETIC_MULTILINE_STR  "This is a multiline'
    yield "quoted string that spans"
    yield "multiple lines. The formatter"
    yield 'should preserve it verbatim.")'
    yield ""
    # Bracket argument in command
    yield "MESSAGE(STATUS [=[Bracket message"
    yield "with level-1 brackets."
    yield "Preserved verbatim.]=])"
    yield ""
    # configure_file with bracket content
    yield 'FILE(WRITE "${CMAKE_BINARY_DIR}/test.cmake" [=['
    yield "cmake_minimum_required(VERSION 3.20)"
    yield 'message(STATUS "Generated file")'
    yield "]=])"
    yield ""


def unknown_commands() -> Iterator[str]:
    """Custom/unrecognized commands."""
    yield "# --------------------------------------------------------------------------"
    yield "# Unknown / custom commands"
    yield "# --------------------------------------------------------------------------"
    yield ""
    for i in range(15):
        cmd = f"synthetic_custom_cmd_{i}"
        case = [cmd, cmd.upper(), cmd.replace("_c", "_C")][i % 3]
        args = " ".join(f"arg_{j}" for j in range(5 + i % 8))
        yield f"{case}(  {args}  )"
    yield ""
    # Unknown commands with keyword-like args
    yield "my_project_setup(  TARGET  synthetic_core  MODE  Release  FEATURES  feature_a  feature_b  feature_c  )"
    yield ""
    yield "internal_configure("
    yield '  CONFIG_FILE  "${CMAKE_CURRENT_SOURCE_DIR}/config.yml"'
    yield '  OUTPUT_DIR  "${CMAKE_BINARY_DIR}/configured"'
    yield "  TEMPLATES"
    yield "    template_a.in"
    yield "    template_b.in"
    yield "    template_c.in"
    yield "  VARIABLES"
    yield "    VERSION=${PROJECT_VERSION}"
    yield "    BUILD_TYPE=$<CONFIG>"
    yield ")"
    yield ""


def whitespace_edge_cases() -> Iterator[str]:
    """Trailing whitespace, multiple spaces, mixed spacing."""
    yield "# --------------------------------------------------------------------------"
    yield "# Whitespace edge cases"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # Extra spaces between tokens
    yield "SET(   EXTRA_SPACES_VAR     value1     value2     value3   )"
    yield "SET(   TABS_AND_SPACES\tvalue1\t\tvalue2   )"
    yield ""
    # Trailing whitespace (deliberate)
    yield "SET(TRAILING_WS  value)   "
    yield "# Comment with trailing spaces   "
    yield ""
    # Empty arguments
    yield 'SET(EMPTY_STR  "")'
    yield 'SET(EMPTY_LIST  "" "" "")'
    yield ""
    # Paren spacing edge cases
    yield "SET (  SPACE_BEFORE_PAREN  value )"
    yield "SET(NO_SPACE_INSIDE value)"
    yield "SET(  EXTRA_SPACE_INSIDE  value  )"
    yield ""


def misc_commands() -> Iterator[str]:
    """Various other recognized commands."""
    yield "# --------------------------------------------------------------------------"
    yield "# Miscellaneous commands"
    yield "# --------------------------------------------------------------------------"
    yield ""
    # include
    yield 'INCLUDE(  "${CMAKE_CURRENT_LIST_DIR}/SyntheticHelpers.cmake"  OPTIONAL  RESULT_VARIABLE  SYNTHETIC_HELPERS_FOUND )'
    yield "include(  CMakePackageConfigHelpers )"
    yield "INCLUDE(  CheckCXXCompilerFlag  )"
    yield "include(  GNUInstallDirs  )"
    yield "include(  FetchContent  )"
    yield ""
    # add_subdirectory
    yield "ADD_SUBDIRECTORY(  src/module_a  )"
    yield "add_subdirectory(  src/module_b  EXCLUDE_FROM_ALL )"
    yield 'ADD_SUBDIRECTORY(  "${CMAKE_CURRENT_SOURCE_DIR}/external/dep"  "${CMAKE_BINARY_DIR}/dep_build" )'
    yield ""
    # add_dependencies
    yield "ADD_DEPENDENCIES(  synthetic_core  synthetic_all_generated )"
    yield "add_dependencies(  synthetic_shared  synthetic_core  )"
    yield ""
    # message with various modes
    for mode in [
        "STATUS",
        "WARNING",
        "AUTHOR_WARNING",
        "SEND_ERROR",
        "DEPRECATION",
        "NOTICE",
        "VERBOSE",
        "DEBUG",
        "TRACE",
        "CHECK_START",
        "CHECK_PASS",
        "CHECK_FAIL",
    ]:
        yield f'MESSAGE(  {mode}  "Synthetic {mode.lower()} message: ${{PROJECT_NAME}} v${{PROJECT_VERSION}}"  )'
    yield ""
    # define_property
    yield "DEFINE_PROPERTY(  TARGET  PROPERTY  SYNTHETIC_CUSTOM_PROP"
    yield '  BRIEF_DOCS  "A custom property for synthetic targets"'
    yield '  FULL_DOCS  "This property is used by the synthetic benchmark fixture to test property handling. It accepts a string value."'
    yield ")"
    yield ""
    # get_property / set_property
    yield 'SET_PROPERTY(  TARGET  synthetic_core  PROPERTY  SYNTHETIC_CUSTOM_PROP  "custom_value"  )'
    yield "GET_PROPERTY(  SYNTHETIC_PROP_VALUE  TARGET  synthetic_core  PROPERTY  SYNTHETIC_CUSTOM_PROP )"
    yield ""
    yield "set_property(  DIRECTORY  ${CMAKE_CURRENT_SOURCE_DIR}  PROPERTY  ADDITIONAL_CLEAN_FILES"
    yield '  "${CMAKE_BINARY_DIR}/generated"'
    yield '  "${CMAKE_BINARY_DIR}/output"'
    yield ")"
    yield ""
    # export
    yield "EXPORT(  TARGETS  synthetic_core  synthetic_shared"
    yield "  NAMESPACE  Synthetic::"
    yield '  FILE  "${CMAKE_BINARY_DIR}/SyntheticTargets.cmake"'
    yield ")"
    yield ""
    # mark_as_advanced
    yield "MARK_AS_ADVANCED("
    yield "  SYNTHETIC_CACHE_00"
    yield "  SYNTHETIC_CACHE_01"
    yield "  SYNTHETIC_CACHE_02"
    yield "  SYNTHETIC_CACHE_03"
    yield "  SYNTHETIC_CACHE_04"
    yield ")"
    yield ""
    # unset
    yield "UNSET(  SYNTHETIC_TEMP_VAR  )"
    yield "unset(  SYNTHETIC_CACHE_TEMP  CACHE  )"
    yield "UNSET(  ENV{SYNTHETIC_ENV_TEMP}  )"
    yield ""
    # return
    yield "# return() is valid at file scope (exits processing of current file)"
    yield "# We won't actually call it here as it would stop processing"
    yield ""
    # source_group
    yield 'SOURCE_GROUP(  "Source Files\\\\Core"  FILES  src/core/main.cpp  src/core/init.cpp )'
    yield 'source_group(  TREE  "${CMAKE_CURRENT_SOURCE_DIR}/src"  PREFIX  "Sources"  FILES  ${SYNTHETIC_SOURCES} )'
    yield ""
    # separate_arguments
    yield 'SEPARATE_ARGUMENTS(  SYNTHETIC_ARGS  UNIX_COMMAND  "${SYNTHETIC_CMD_LINE}" )'
    yield 'separate_arguments(  SYNTHETIC_WIN_ARGS  WINDOWS_COMMAND  "${SYNTHETIC_WIN_CMD}" )'
    yield ""
    # enable_language
    yield "ENABLE_LANGUAGE(  Fortran  OPTIONAL )"
    yield ""
    # get_filename_component
    yield "GET_FILENAME_COMPONENT(  SYNTHETIC_DIR  ${SYNTHETIC_PATH}  DIRECTORY )"
    yield "get_filename_component(  SYNTHETIC_NAMEWE  ${SYNTHETIC_PATH}  NAME_WE )"
    yield "GET_FILENAME_COMPONENT(  SYNTHETIC_EXT2  ${SYNTHETIC_PATH}  LAST_EXT )"
    yield ""
    # cmake_host_system_information
    yield "CMAKE_HOST_SYSTEM_INFORMATION(  RESULT  SYNTHETIC_CPU_COUNT  QUERY  NUMBER_OF_PHYSICAL_CORES )"
    yield "cmake_host_system_information(  RESULT  SYNTHETIC_TOTAL_MEM  QUERY  TOTAL_PHYSICAL_MEMORY )"
    yield "CMAKE_HOST_SYSTEM_INFORMATION(  RESULT  SYNTHETIC_HOSTNAME  QUERY  HOSTNAME )"
    yield ""
    # cmake_pkg_config
    yield "# cmake_pkg_config (CMake 3.28+)"
    yield ""
    # build_command
    yield "BUILD_COMMAND(  SYNTHETIC_BUILD_CMD  TARGET  synthetic_core  CONFIGURATION  Release )"
    yield ""
    # add_compile_definitions / options / link_options
    yield 'ADD_COMPILE_DEFINITIONS(  SYNTHETIC_GLOBAL_DEF=1  "SYNTHETIC_VERSION=\\"${PROJECT_VERSION}\\""  )'
    yield "add_compile_options(  -fPIC  $<$<CONFIG:Debug>:-fsanitize=address>  )"
    yield "ADD_LINK_OPTIONS(  $<$<CONFIG:Debug>:-fsanitize=address>  )"
    yield ""
    # Deprecated commands
    yield "ADD_DEFINITIONS(  -DSYNTHETIC_COMPAT=1  )"
    yield ""
    # FetchContent
    yield "INCLUDE(  FetchContent  )"
    yield "FetchContent_Declare(  synthetic_dep"
    yield "  GIT_REPOSITORY  https://github.com/example/synthetic_dep.git"
    yield "  GIT_TAG  v1.2.3"
    yield "  GIT_SHALLOW  TRUE"
    yield ")"
    yield "FetchContent_MakeAvailable(  synthetic_dep  )"
    yield ""


def set_properties_section() -> Iterator[str]:
    """Various set_*_properties commands."""
    yield "# --------------------------------------------------------------------------"
    yield "# set_*_properties()"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "SET_SOURCE_FILES_PROPERTIES("
    yield "  src/core/main.cpp"
    yield "  src/core/init.cpp"
    yield "  PROPERTIES"
    yield '  COMPILE_FLAGS  "-O2 -DSPECIAL"'
    yield "  LANGUAGE  CXX"
    yield ")"
    yield ""
    yield "set_directory_properties(  PROPERTIES"
    yield '  ADDITIONAL_CLEAN_FILES  "${CMAKE_BINARY_DIR}/temp"'
    yield '  COMPILE_DEFINITIONS  "DIR_LEVEL_DEF=1"'
    yield ")"
    yield ""
    yield "set_package_properties(  Boost  PROPERTIES"
    yield '  DESCRIPTION  "Boost C++ Libraries"'
    yield '  URL  "https://www.boost.org"'
    yield "  TYPE  REQUIRED"
    yield '  PURPOSE  "Core dependency for filesystem and threading"'
    yield ")"
    yield ""
    yield "GET_DIRECTORY_PROPERTY(  SYNTHETIC_DIR_DEFS  COMPILE_DEFINITIONS  )"
    yield ""


def ctest_commands() -> Iterator[str]:
    """CTest commands."""
    yield "# --------------------------------------------------------------------------"
    yield "# CTest commands"
    yield "# --------------------------------------------------------------------------"
    yield ""
    yield "CTEST_START(  Experimental  )"
    yield "ctest_configure(  OPTIONS  -DSYNTHETIC_TEST=ON  RETURN_VALUE  SYNTHETIC_CONFIGURE_RESULT )"
    yield "CTEST_BUILD(  TARGET  synthetic_core  NUMBER_ERRORS  SYNTHETIC_BUILD_ERRORS  NUMBER_WARNINGS  SYNTHETIC_BUILD_WARNINGS  RETURN_VALUE  SYNTHETIC_BUILD_RESULT )"
    yield "ctest_test(  PARALLEL_LEVEL  4  EXCLUDE_LABEL  slow  RETURN_VALUE  SYNTHETIC_TEST_RESULT )"
    yield "CTEST_COVERAGE(  RETURN_VALUE  SYNTHETIC_COVERAGE_RESULT  )"
    yield "ctest_memcheck(  RETURN_VALUE  SYNTHETIC_MEMCHECK_RESULT  )"
    yield "CTEST_SUBMIT(  RETURN_VALUE  SYNTHETIC_SUBMIT_RESULT  )"
    yield ""


def repeat_pattern(batch: int) -> Iterator[str]:
    """Parameterized section that can be repeated to hit target line count.

    Each batch generates ~100 lines of diverse commands.
    """
    yield f"# =========================================================================="
    yield f"# Repeated pattern batch {batch}"
    yield f"# =========================================================================="
    yield ""

    # A library with source list
    lib_name = f"synthetic_module_{batch:03d}"
    yield f"ADD_LIBRARY(  {lib_name}  STATIC"
    for i in range(15):
        idx = (i * 7 + batch) % 100
        yield f"    src/modules/mod{batch:03d}/file_{idx:04d}.cpp"
    yield ")"
    yield ""

    # Target properties
    yield f"SET_TARGET_PROPERTIES(  {lib_name}  PROPERTIES"
    yield f"  CXX_STANDARD  20"
    yield f"  CXX_STANDARD_REQUIRED  ON"
    yield f"  CXX_EXTENSIONS  OFF"
    yield f'  FOLDER  "Modules/Batch{batch}"'
    yield f'  OUTPUT_NAME  "mod{batch:03d}"'
    yield f")"
    yield ""

    # Include directories
    yield f"TARGET_INCLUDE_DIRECTORIES(  {lib_name}"
    yield f"  PUBLIC"
    yield f"    $<BUILD_INTERFACE:${{CMAKE_CURRENT_SOURCE_DIR}}/include/mod{batch:03d}>"
    yield f"    $<INSTALL_INTERFACE:include>"
    yield f"  PRIVATE"
    yield f"    ${{CMAKE_CURRENT_SOURCE_DIR}}/src/modules/mod{batch:03d}"
    yield f")"
    yield ""

    # Compile definitions with genex
    yield f"target_compile_definitions(  {lib_name}"
    yield f"  PUBLIC"
    yield f"    SYNTHETIC_MODULE_{batch:03d}=1"
    yield f"    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_{batch:03d}_DEBUG>"
    yield f"  PRIVATE"
    yield f"    SYNTHETIC_MODULE_INTERNAL_{batch:03d}"
    yield f")"
    yield ""

    # Link libraries
    if batch > 0:
        prev_lib = f"synthetic_module_{batch - 1:03d}"
        yield f"target_link_libraries(  {lib_name}"
        yield f"  PUBLIC"
        yield f"    {prev_lib}"
        yield f"  PRIVATE"
        yield f"    Threads::Threads"
        yield f"    $<$<PLATFORM_ID:Linux>:dl>"
        yield f")"
    else:
        yield f"target_link_libraries(  {lib_name}"
        yield f"  PRIVATE"
        yield f"    Threads::Threads"
        yield f")"
    yield ""

    # Consecutive set() for alignment
    for i in range(5):
        name = f"MOD{batch:03d}_SETTING_{chr(65 + i)}"
        yield f'SET({name}  "value_{batch}_{i}")'
    yield ""

    # Some flow control
    yield f"IF(  SYNTHETIC_ENABLE_MODULE_{batch:03d}  )"
    yield f'  MESSAGE(  STATUS  "Module {batch:03d} enabled"  )'
    yield f"  TARGET_COMPILE_DEFINITIONS(  {lib_name}  PUBLIC  SYNTHETIC_MODULE_{batch:03d}_ENABLED )"
    yield f"ELSE()"
    yield f'  MESSAGE(  STATUS  "Module {batch:03d} disabled"  )'
    yield f"ENDIF()"
    yield ""

    # A test executable
    yield f"ADD_EXECUTABLE(  test_module_{batch:03d}"
    for i in range(5):
        yield f"    tests/modules/mod{batch:03d}/test_{i}.cpp"
    yield ")"
    yield ""
    yield f"target_link_libraries(  test_module_{batch:03d}  PRIVATE  {lib_name}  GTest::gtest_main )"
    yield f"GTEST_DISCOVER_TESTS(  test_module_{batch:03d}"
    yield f'  TEST_PREFIX  "mod{batch:03d}::"'
    yield f"  DISCOVERY_TIMEOUT  60"
    yield f")"
    yield ""

    # Comments for reflow
    yield f"# Module {batch:03d} provides synthetic functionality for benchmark testing purposes. This module includes source files, compile definitions, and test infrastructure that exercises the formatter's handling of various CMake constructs."
    yield ""

    # Trailing comment alignment
    yield f"SET(MOD{batch:03d}_A  val_a)  # first"
    yield f"SET(MOD{batch:03d}_BB  val_bb)  # second"
    yield f"SET(MOD{batch:03d}_CCC  val_ccc)  # third"
    yield ""


def main() -> None:
    sections = [
        header,
        options_and_vars,
        find_packages,
        string_operations,
        list_operations,
        file_operations,
        math_operations,
        flow_control,
        libraries,
        executables,
        target_props,
        install_rules,
        custom_commands,
        testing,
        genex_heavy,
        comments_heavy,
        alignment_groups,
        pragmas,
        cmake_language_section,
        cmake_path_section,
        execute_process_section,
        try_compile_section,
        cmake_parse_args_section,
        bracket_args,
        unknown_commands,
        whitespace_edge_cases,
        misc_commands,
        set_properties_section,
        ctest_commands,
    ]

    lines: list[str] = []
    for section in sections:
        lines.extend(section())

    # Add repeated batches to reach target line count
    batch = 0
    while len(lines) < 10000:
        lines.extend(repeat_pattern(batch))
        batch += 1

    output = "\n".join(lines)
    # Ensure final newline
    if not output.endswith("\n"):
        output += "\n"

    if len(sys.argv) > 1:
        with open(sys.argv[1], "w", newline="\n") as f:
            f.write(output)
        print(f"Generated {len(lines)} lines to {sys.argv[1]}", file=sys.stderr)
    else:
        sys.stdout.write(output)
        print(f"Generated {len(lines)} lines", file=sys.stderr)


if __name__ == "__main__":
    main()

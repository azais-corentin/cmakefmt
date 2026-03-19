# ============================================================================
# Synthetic CMake benchmark fixture Exercises all formatting code paths for
# cmakefmt
# ============================================================================

cmake_minimum_required(VERSION 3.28 FATAL_ERROR)

project(SyntheticBenchmark
  VERSION
    1.2.3.4

  DESCRIPTION
    "A synthetic benchmark fixture for cmakefmt"

  HOMEPAGE_URL
    "https://example.com/synthetic"

  LANGUAGES
    C CXX ASM
)

# --------------------------------------------------------------------------
# Options and variables
# --------------------------------------------------------------------------

option(SYNTHETIC_OPTION_00 "Option number 0 for benchmark" ON)
option(SYNTHETIC_OPTION_01 "Option number 1 for benchmark" OFF)
option(SYNTHETIC_OPTION_02 "Option number 2 for benchmark" ON)
option(SYNTHETIC_OPTION_03 "Option number 3 for benchmark" OFF)
option(SYNTHETIC_OPTION_04 "Option number 4 for benchmark" ON)
option(SYNTHETIC_OPTION_05 "Option number 5 for benchmark" OFF)
option(SYNTHETIC_OPTION_06 "Option number 6 for benchmark" ON)
option(SYNTHETIC_OPTION_07 "Option number 7 for benchmark" OFF)
option(SYNTHETIC_OPTION_08 "Option number 8 for benchmark" ON)
option(SYNTHETIC_OPTION_09 "Option number 9 for benchmark" OFF)
option(SYNTHETIC_OPTION_10 "Option number 10 for benchmark" ON)
option(SYNTHETIC_OPTION_11 "Option number 11 for benchmark" OFF)
option(SYNTHETIC_OPTION_12 "Option number 12 for benchmark" ON)
option(SYNTHETIC_OPTION_13 "Option number 13 for benchmark" OFF)
option(SYNTHETIC_OPTION_14 "Option number 14 for benchmark" ON)
option(SYNTHETIC_OPTION_15 "Option number 15 for benchmark" OFF)
option(SYNTHETIC_OPTION_16 "Option number 16 for benchmark" ON)
option(SYNTHETIC_OPTION_17 "Option number 17 for benchmark" OFF)
option(SYNTHETIC_OPTION_18 "Option number 18 for benchmark" ON)
option(SYNTHETIC_OPTION_19 "Option number 19 for benchmark" OFF)

# Consecutive set() calls for alignment
set(VAR_A_LONG_EXTRA_LONG_NAME 0)
set(VAR_B                      "value_1")
set(VAR_C                      "value_2")
set(VAR_D                      300)
set(VAR_E                      "value_4")
set(VAR_F_LONG                 "value_5")
set(VAR_G                      600)
set(VAR_H_EXTRA_LONG_NAME      "value_7")
set(VAR_I                      "value_8")
set(VAR_J                      900)
set(VAR_K_LONG                 "value_10")
set(VAR_L                      "value_11")
set(VAR_M                      1200)
set(VAR_N                      "value_13")
set(VAR_O_EXTRA_LONG_NAME      "value_14")
set(VAR_P_LONG                 1500)
set(VAR_Q                      "value_16")
set(VAR_R                      "value_17")
set(VAR_S                      1800)
set(VAR_T                      "value_19")
set(VAR_U_LONG                 "value_20")
set(VAR_V_EXTRA_LONG_NAME      2100)
set(VAR_W                      "value_22")
set(VAR_X                      "value_23")
set(VAR_Y                      2400)
set(VAR_Z_LONG                 "value_25")
set(VAR_A                      "value_26")
set(VAR_B                      2700)
set(VAR_C_EXTRA_LONG_NAME      "value_28")
set(VAR_D                      "value_29")

# Cache variables with types
set(SYNTHETIC_CACHE_00
  "default_0"
  CACHE
    STRING "Cache variable 0"
  FORCE
)
set(SYNTHETIC_CACHE_01
  "default_1"
  CACHE
    BOOL "Cache variable 1"
  FORCE
)
set(SYNTHETIC_CACHE_02
  "default_2"
  CACHE
    PATH "Cache variable 2"
  FORCE
)
set(SYNTHETIC_CACHE_03
  "default_3"
  CACHE
    FILEPATH "Cache variable 3"
  FORCE
)
set(SYNTHETIC_CACHE_04
  "default_4"
  CACHE
    INTERNAL "Cache variable 4"
  FORCE
)
set(SYNTHETIC_CACHE_05
  "default_5"
  CACHE
    STRING "Cache variable 5"
  FORCE
)
set(SYNTHETIC_CACHE_06
  "default_6"
  CACHE
    BOOL "Cache variable 6"
  FORCE
)
set(SYNTHETIC_CACHE_07
  "default_7"
  CACHE
    PATH "Cache variable 7"
  FORCE
)
set(SYNTHETIC_CACHE_08
  "default_8"
  CACHE
    FILEPATH "Cache variable 8"
  FORCE
)
set(SYNTHETIC_CACHE_09
  "default_9"
  CACHE
    INTERNAL "Cache variable 9"
  FORCE
)
set(SYNTHETIC_CACHE_10
  "default_10"
  CACHE
    STRING "Cache variable 10"
  FORCE
)
set(SYNTHETIC_CACHE_11
  "default_11"
  CACHE
    BOOL "Cache variable 11"
  FORCE
)
set(SYNTHETIC_CACHE_12
  "default_12"
  CACHE
    PATH "Cache variable 12"
  FORCE
)
set(SYNTHETIC_CACHE_13
  "default_13"
  CACHE
    FILEPATH "Cache variable 13"
  FORCE
)
set(SYNTHETIC_CACHE_14
  "default_14"
  CACHE
    INTERNAL "Cache variable 14"
  FORCE
)

set(SYNTHETIC_PARENT_VAR   ${SOME_VALUE} PARENT_SCOPE)
set(ENV{SYNTHETIC_ENV_VAR} $ENV{PATH}:/opt/synthetic)

# --------------------------------------------------------------------------
# Package discovery
# --------------------------------------------------------------------------

find_package(Boost 1.84
  REQUIRED

  COMPONENTS
    filesystem
    program_options
    regex
    system
    thread
)
find_package(Qt6 6.5
  REQUIRED

  COMPONENTS
    Core
    Gui
    Network
    Sql
    Widgets
    Xml
)
find_package(OpenCV 4.8
  REQUIRED

  COMPONENTS
    calib3d
    core
    highgui
    imgproc
    videoio
)
find_package(Protobuf 3.21
  REQUIRED
)
find_package(gRPC 1.50
  REQUIRED
)
find_package(CUDA 12.0
  REQUIRED
)
find_package(OpenSSL 3.0
  REQUIRED
)
find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

find_library(SYNTHETIC_LIB synthetic
  HINTS
    ${CMAKE_PREFIX_PATH}/lib /usr/lib /usr/local/lib

  REQUIRED
)
find_path(SYNTHETIC_INCLUDE_DIR synthetic.h
  HINTS
    ${CMAKE_PREFIX_PATH}/include /usr/include
)
find_program(SYNTHETIC_TOOL syntool HINTS ${CMAKE_PREFIX_PATH}/bin /usr/bin)
find_file(SYNTHETIC_CONFIG synthetic.cfg HINTS ${CMAKE_PREFIX_PATH}/etc /etc)

# --------------------------------------------------------------------------
# String operations
# --------------------------------------------------------------------------

string(
  REGEX MATCH "([0-9]+)\.([0-9]+)\.([0-9]+)"
  SYNTHETIC_VERSION_MATCH ${SYNTHETIC_VERSION_STRING}
)
string(
  REGEX MATCHALL "[a-zA-Z_][a-zA-Z0-9_]*"
  SYNTHETIC_IDENTIFIERS "${SYNTHETIC_SOURCE_TEXT}"
)
string(REGEX REPLACE "[;]+" "," SYNTHETIC_PREFIXED ${SYNTHETIC_LIST})
string(
  REPLACE
    "old_pattern"
  "new_pattern" SYNTHETIC_REPLACED "${SYNTHETIC_INPUT}"
)
string(APPEND SYNTHETIC_BUFFER "first part " "second part " "third part")
string(PREPEND SYNTHETIC_BUFFER "header: ")
string(CONCAT SYNTHETIC_FULL "part1" "_" "part2" "_" "part3")
string(
  JOIN
    ";"
  SYNTHETIC_JOINED item1 item2 item3 item4 item5 item6 item7 item8 item9 item10
)
string(TOLOWER ${SYNTHETIC_MIXED} SYNTHETIC_LOWER)
string(TOUPPER ${SYNTHETIC_MIXED} SYNTHETIC_UPPER)
string(LENGTH ${SYNTHETIC_INPUT} SYNTHETIC_LEN)
string(SUBSTRING ${SYNTHETIC_INPUT} 0 10 SYNTHETIC_SUB)
string(STRIP ${SYNTHETIC_INPUT} SYNTHETIC_STRIPPED)
string(REPEAT "abc" 10 SYNTHETIC_REPEATED)
string(CONFIGURE "config @VAR@ done" SYNTHETIC_CONFIGURED)
string(MAKE_C_IDENTIFIER "some-header.h" SYNTHETIC_C_ID)
string(RANDOM LENGTH 32 ALPHABET "abcdef0123456789" SYNTHETIC_RANDOM)
string(COMPARE LESS "aaa" "bbb" SYNTHETIC_CMP_RESULT)
string(TIMESTAMP SYNTHETIC_TIMESTAMP "%Y-%m-%d" UTC)
string(
  UUID
    SYNTHETIC_UUID

  NAMESPACE
    "6ba7b810-9dad-11d1-80b4-00c04fd430c8"

  NAME
    "synthetic"

  TYPE
    SHA1
)

# --------------------------------------------------------------------------
# List operations
# --------------------------------------------------------------------------

list(LENGTH SYNTHETIC_SOURCES SYNTHETIC_SRC_COUNT)
list(GET SYNTHETIC_SOURCES 0 2 4 SYNTHETIC_SELECTED)
list(
  APPEND SYNTHETIC_SOURCES
  src/synthetic_000.cpp
  src/synthetic_001.cpp
  src/synthetic_002.cpp
  src/synthetic_003.cpp
  src/synthetic_004.cpp
  src/synthetic_005.cpp
  src/synthetic_006.cpp
  src/synthetic_007.cpp
  src/synthetic_008.cpp
  src/synthetic_009.cpp
  src/synthetic_010.cpp
  src/synthetic_011.cpp
  src/synthetic_012.cpp
  src/synthetic_013.cpp
  src/synthetic_014.cpp
  src/synthetic_015.cpp
  src/synthetic_016.cpp
  src/synthetic_017.cpp
  src/synthetic_018.cpp
  src/synthetic_019.cpp
  src/synthetic_020.cpp
  src/synthetic_021.cpp
  src/synthetic_022.cpp
  src/synthetic_023.cpp
  src/synthetic_024.cpp
)

list(JOIN SYNTHETIC_SOURCES "," SYNTHETIC_CSV)
list(SUBLIST SYNTHETIC_SOURCES 5 10 SYNTHETIC_SLICE)
list(FIND SYNTHETIC_SOURCES src/synthetic_010.cpp SYNTHETIC_IDX)
list(INSERT SYNTHETIC_SOURCES 0 src/synthetic_new.cpp)
list(REMOVE_ITEM SYNTHETIC_SOURCES src/synthetic_000.cpp src/synthetic_001.cpp)
list(REMOVE_AT SYNTHETIC_SOURCES 0 1 2)
list(REMOVE_DUPLICATES SYNTHETIC_SOURCES)
list(
  FILTER SYNTHETIC_SOURCES
  INCLUDE
  REGEX  ".*\.cpp$"
)
list(
  FILTER SYNTHETIC_SOURCES
  EXCLUDE
  REGEX  ".*_test\.cpp$"
)
list(
  SORT    SYNTHETIC_SOURCES
  COMPARE NATURAL
  CASE    INSENSITIVE
  ORDER   ASCENDING
)
list(REVERSE SYNTHETIC_SOURCES)
list(POP_BACK SYNTHETIC_SOURCES SYNTHETIC_LAST)
list(POP_FRONT SYNTHETIC_SOURCES SYNTHETIC_FIRST)
list(
  TRANSFORM       SYNTHETIC_SOURCES
  PREPEND         src/
  OUTPUT_VARIABLE SYNTHETIC_FULL_PATHS
)
list(
  TRANSFORM SYNTHETIC_SOURCES
  TOUPPER
)

# --------------------------------------------------------------------------
# File operations
# --------------------------------------------------------------------------

file(
  GLOB
    SYNTHETIC_GLOB_SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"
  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c"
  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h"
)

file(
  GLOB_RECURSE
    SYNTHETIC_ALL_SOURCES

  CONFIGURE_DEPENDS
  "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"
  "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h"
)

file(READ "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" SYNTHETIC_VER_CONTENT)
file(STRINGS ${CMAKE_CURRENT_SOURCE_DIR}/config.txt SYNTHETIC_CONFIG_LINES)
file(WRITE "${CMAKE_BINARY_DIR}/generated.h" "#pragma once\n// Generated\n")
file(APPEND "${CMAKE_BINARY_DIR}/generated.h" "#define SYNTHETIC 1\n")

file(
  GENERATE OUTPUT "${CMAKE_BINARY_DIR}/config_$<CONFIG>.h"

  CONTENT
    "#define BUILD_TYPE \"$<CONFIG>\"\n"

  CONDITION
    $<BOOL:${SYNTHETIC_GENERATE}>
)

file(
  COPY
    "${CMAKE_CURRENT_SOURCE_DIR}/data"

  DESTINATION
    "${CMAKE_BINARY_DIR}"

  FILE_PERMISSIONS
    OWNER_READ OWNER_WRITE GROUP_READ WORLD_READ

  FILES_MATCHING

  PATTERN
    "*.dat"

  PATTERN
    ".svn"

  EXCLUDE
)

file(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/output")
file(
  RELATIVE_PATH
  SYNTHETIC_REL ${CMAKE_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR}
)
file(TO_CMAKE_PATH "${SYNTHETIC_NATIVE_PATH}" SYNTHETIC_CMAKE_PATH)
file(TO_NATIVE_PATH "${SYNTHETIC_CMAKE_PATH}" SYNTHETIC_NATIVE)
file(
  DOWNLOAD
    "https://example.com/data.tar.gz"
  "${CMAKE_BINARY_DIR}/data.tar.gz"

  TIMEOUT
    60

  EXPECTED_HASH
    SHA256=abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890

  TLS_VERIFY
    ON

  STATUS
    SYNTHETIC_DL_STATUS
)

file(
  ARCHIVE_CREATE

  OUTPUT
    "${CMAKE_BINARY_DIR}/archive.tar.gz"

  PATHS
    ${SYNTHETIC_SOURCES}
    FORMAT gnutar

  COMPRESSION
    GZip

  COMPRESSION_LEVEL
    9
)

file(SIZE "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" SYNTHETIC_FILE_SIZE)
file(SHA256 "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" SYNTHETIC_FILE_HASH)

# --------------------------------------------------------------------------
# Math operations
# --------------------------------------------------------------------------

math(EXPR SYNTHETIC_RESULT_0 "${SYNTHETIC_A} + ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_1 "${SYNTHETIC_A} - ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_2 "${SYNTHETIC_A} * ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_3 "${SYNTHETIC_A} / ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_4 "${SYNTHETIC_A} % ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_5 "${SYNTHETIC_A} & ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_6 "${SYNTHETIC_A} | ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_7 "${SYNTHETIC_A} ^ ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_8 "${SYNTHETIC_A} << ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_RESULT_9 "${SYNTHETIC_A} >> ${SYNTHETIC_B}")
math(EXPR SYNTHETIC_HEX "0xff + 1" OUTPUT_FORMAT HEXADECIMAL)

# --------------------------------------------------------------------------
# Flow control (nested)
# --------------------------------------------------------------------------

if(SYNTHETIC_COND_0_A)
  message(STATUS "Condition 0 depth 0 met")

  if(SYNTHETIC_VAR_1 STREQUAL "value_1")
    message(STATUS "Condition 1 depth 1 met")

    if(DEFINED SYNTHETIC_DEF_3)
      message(STATUS "Condition 3 depth 2 met")

      if(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 7 depth 3 met")

        if(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 15 depth 4 met")
        elseif(NOT SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_15)
          message(WARNING "Fallback 15 depth 4")
        else(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 15")
        endif(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_7)
        message(WARNING "Fallback 7 depth 3")
      else(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 7")
      endif(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT DEFINED SYNTHETIC_DEF_3 AND SYNTHETIC_FALLBACK_3)
      message(WARNING "Fallback 3 depth 2")

      if(SYNTHETIC_COND_8_A)
        message(STATUS "Condition 8 depth 3 met")

        if(SYNTHETIC_VAR_17 STREQUAL "value_17")
          message(STATUS "Condition 17 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_17 STREQUAL "value_17" AND SYNTHETIC_FALLBACK_17)
          message(WARNING "Fallback 17 depth 4")
        else(SYNTHETIC_VAR_17 STREQUAL "value_17")
          message(AUTHOR_WARNING "Neither condition for 17")
        endif(SYNTHETIC_VAR_17 STREQUAL "value_17")
      elseif(NOT SYNTHETIC_COND_8_A AND SYNTHETIC_FALLBACK_8)
        message(WARNING "Fallback 8 depth 3")
      else(SYNTHETIC_COND_8_A)
        message(AUTHOR_WARNING "Neither condition for 8")
      endif(SYNTHETIC_COND_8_A)
    else(DEFINED SYNTHETIC_DEF_3)
      message(AUTHOR_WARNING "Neither condition for 3")
    endif(DEFINED SYNTHETIC_DEF_3)
  elseif(NOT SYNTHETIC_VAR_1 STREQUAL "value_1" AND SYNTHETIC_FALLBACK_1)
    message(WARNING "Fallback 1 depth 1")

    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
      message(STATUS "Condition 4 depth 2 met")

      if(SYNTHETIC_VAR_9 STREQUAL "value_9")
        message(STATUS "Condition 9 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_19)
          message(STATUS "Condition 19 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_19 AND SYNTHETIC_FALLBACK_19)
          message(WARNING "Fallback 19 depth 4")
        else(DEFINED SYNTHETIC_DEF_19)
          message(AUTHOR_WARNING "Neither condition for 19")
        endif(DEFINED SYNTHETIC_DEF_19)
      elseif(NOT SYNTHETIC_VAR_9 STREQUAL "value_9" AND SYNTHETIC_FALLBACK_9)
        message(WARNING "Fallback 9 depth 3")
      else(SYNTHETIC_VAR_9 STREQUAL "value_9")
        message(AUTHOR_WARNING "Neither condition for 9")
      endif(SYNTHETIC_VAR_9 STREQUAL "value_9")
    elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt AND SYNTHETIC_FALLBACK_4)
      message(WARNING "Fallback 4 depth 2")

      if(SYNTHETIC_NUM_10 GREATER 100)
        message(STATUS "Condition 10 depth 3 met")

        if(SYNTHETIC_LIST_21 MATCHES "^pattern")
          message(STATUS "Condition 21 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_21 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_21)
          message(WARNING "Fallback 21 depth 4")
        else(SYNTHETIC_LIST_21 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 21")
        endif(SYNTHETIC_LIST_21 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_10 GREATER 100 AND SYNTHETIC_FALLBACK_10)
        message(WARNING "Fallback 10 depth 3")
      else(SYNTHETIC_NUM_10 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 10")
      endif(SYNTHETIC_NUM_10 GREATER 100)
    else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
      message(AUTHOR_WARNING "Neither condition for 4")
    endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
  else(SYNTHETIC_VAR_1 STREQUAL "value_1")
    message(AUTHOR_WARNING "Neither condition for 1")
  endif(SYNTHETIC_VAR_1 STREQUAL "value_1")
elseif(NOT SYNTHETIC_COND_0_A AND SYNTHETIC_FALLBACK_0)
  message(WARNING "Fallback 0 depth 0")

  if(SYNTHETIC_NUM_2 GREATER 100)
    message(STATUS "Condition 2 depth 1 met")

    if(SYNTHETIC_LIST_5 MATCHES "^pattern")
      message(STATUS "Condition 5 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_11)
        message(STATUS "Condition 11 depth 3 met")

        if(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 23 depth 4 met")
        elseif(NOT SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_23)
          message(WARNING "Fallback 23 depth 4")
        else(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 23")
        endif(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_11 AND SYNTHETIC_FALLBACK_11)
        message(WARNING "Fallback 11 depth 3")
      else(DEFINED SYNTHETIC_DEF_11)
        message(AUTHOR_WARNING "Neither condition for 11")
      endif(DEFINED SYNTHETIC_DEF_11)
    elseif(NOT SYNTHETIC_LIST_5 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_5)
      message(WARNING "Fallback 5 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
        message(STATUS "Condition 12 depth 3 met")

        if(SYNTHETIC_VAR_25 STREQUAL "value_25")
          message(STATUS "Condition 25 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_25 STREQUAL "value_25" AND SYNTHETIC_FALLBACK_25)
          message(WARNING "Fallback 25 depth 4")
        else(SYNTHETIC_VAR_25 STREQUAL "value_25")
          message(AUTHOR_WARNING "Neither condition for 25")
        endif(SYNTHETIC_VAR_25 STREQUAL "value_25")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt AND SYNTHETIC_FALLBACK_12)
        message(WARNING "Fallback 12 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
        message(AUTHOR_WARNING "Neither condition for 12")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
    else(SYNTHETIC_LIST_5 MATCHES "^pattern")
      message(AUTHOR_WARNING "Neither condition for 5")
    endif(SYNTHETIC_LIST_5 MATCHES "^pattern")
  elseif(NOT SYNTHETIC_NUM_2 GREATER 100 AND SYNTHETIC_FALLBACK_2)
    message(WARNING "Fallback 2 depth 1")

    if(TARGET synthetic_target_6)
      message(STATUS "Condition 6 depth 2 met")

      if(SYNTHETIC_LIST_13 MATCHES "^pattern")
        message(STATUS "Condition 13 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_27)
          message(STATUS "Condition 27 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_27 AND SYNTHETIC_FALLBACK_27)
          message(WARNING "Fallback 27 depth 4")
        else(DEFINED SYNTHETIC_DEF_27)
          message(AUTHOR_WARNING "Neither condition for 27")
        endif(DEFINED SYNTHETIC_DEF_27)
      elseif(NOT SYNTHETIC_LIST_13 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_13)
        message(WARNING "Fallback 13 depth 3")
      else(SYNTHETIC_LIST_13 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 13")
      endif(SYNTHETIC_LIST_13 MATCHES "^pattern")
    elseif(NOT TARGET synthetic_target_6 AND SYNTHETIC_FALLBACK_6)
      message(WARNING "Fallback 6 depth 2")

      if(TARGET synthetic_target_14)
        message(STATUS "Condition 14 depth 3 met")

        if(SYNTHETIC_LIST_29 MATCHES "^pattern")
          message(STATUS "Condition 29 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_29 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_29)
          message(WARNING "Fallback 29 depth 4")
        else(SYNTHETIC_LIST_29 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 29")
        endif(SYNTHETIC_LIST_29 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_14 AND SYNTHETIC_FALLBACK_14)
        message(WARNING "Fallback 14 depth 3")
      else(TARGET synthetic_target_14)
        message(AUTHOR_WARNING "Neither condition for 14")
      endif(TARGET synthetic_target_14)
    else(TARGET synthetic_target_6)
      message(AUTHOR_WARNING "Neither condition for 6")
    endif(TARGET synthetic_target_6)
  else(SYNTHETIC_NUM_2 GREATER 100)
    message(AUTHOR_WARNING "Neither condition for 2")
  endif(SYNTHETIC_NUM_2 GREATER 100)
else(SYNTHETIC_COND_0_A)
  message(AUTHOR_WARNING "Neither condition for 0")
endif(SYNTHETIC_COND_0_A)

if(SYNTHETIC_VAR_1 STREQUAL "value_1")
  message(STATUS "Condition 1 depth 0 met")

  if(DEFINED SYNTHETIC_DEF_3)
    message(STATUS "Condition 3 depth 1 met")

    if(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
      message(STATUS "Condition 7 depth 2 met")

      if(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 15 depth 3 met")

        if(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 31 depth 4 met")
        elseif(NOT SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_31)
          message(WARNING "Fallback 31 depth 4")
        else(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 31")
        endif(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_15)
        message(WARNING "Fallback 15 depth 3")
      else(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 15")
      endif(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_7)
      message(WARNING "Fallback 7 depth 2")

      if(SYNTHETIC_COND_16_A)
        message(STATUS "Condition 16 depth 3 met")

        if(SYNTHETIC_VAR_33 STREQUAL "value_33")
          message(STATUS "Condition 33 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_33 STREQUAL "value_33" AND SYNTHETIC_FALLBACK_33)
          message(WARNING "Fallback 33 depth 4")
        else(SYNTHETIC_VAR_33 STREQUAL "value_33")
          message(AUTHOR_WARNING "Neither condition for 33")
        endif(SYNTHETIC_VAR_33 STREQUAL "value_33")
      elseif(NOT SYNTHETIC_COND_16_A AND SYNTHETIC_FALLBACK_16)
        message(WARNING "Fallback 16 depth 3")
      else(SYNTHETIC_COND_16_A)
        message(AUTHOR_WARNING "Neither condition for 16")
      endif(SYNTHETIC_COND_16_A)
    else(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
      message(AUTHOR_WARNING "Neither condition for 7")
    endif(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
  elseif(NOT DEFINED SYNTHETIC_DEF_3 AND SYNTHETIC_FALLBACK_3)
    message(WARNING "Fallback 3 depth 1")

    if(SYNTHETIC_COND_8_A)
      message(STATUS "Condition 8 depth 2 met")

      if(SYNTHETIC_VAR_17 STREQUAL "value_17")
        message(STATUS "Condition 17 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_35)
          message(STATUS "Condition 35 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_35 AND SYNTHETIC_FALLBACK_35)
          message(WARNING "Fallback 35 depth 4")
        else(DEFINED SYNTHETIC_DEF_35)
          message(AUTHOR_WARNING "Neither condition for 35")
        endif(DEFINED SYNTHETIC_DEF_35)
      elseif(NOT SYNTHETIC_VAR_17 STREQUAL "value_17" AND SYNTHETIC_FALLBACK_17)
        message(WARNING "Fallback 17 depth 3")
      else(SYNTHETIC_VAR_17 STREQUAL "value_17")
        message(AUTHOR_WARNING "Neither condition for 17")
      endif(SYNTHETIC_VAR_17 STREQUAL "value_17")
    elseif(NOT SYNTHETIC_COND_8_A AND SYNTHETIC_FALLBACK_8)
      message(WARNING "Fallback 8 depth 2")

      if(SYNTHETIC_NUM_18 GREATER 100)
        message(STATUS "Condition 18 depth 3 met")

        if(SYNTHETIC_LIST_37 MATCHES "^pattern")
          message(STATUS "Condition 37 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_37 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_37)
          message(WARNING "Fallback 37 depth 4")
        else(SYNTHETIC_LIST_37 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 37")
        endif(SYNTHETIC_LIST_37 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_18 GREATER 100 AND SYNTHETIC_FALLBACK_18)
        message(WARNING "Fallback 18 depth 3")
      else(SYNTHETIC_NUM_18 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 18")
      endif(SYNTHETIC_NUM_18 GREATER 100)
    else(SYNTHETIC_COND_8_A)
      message(AUTHOR_WARNING "Neither condition for 8")
    endif(SYNTHETIC_COND_8_A)
  else(DEFINED SYNTHETIC_DEF_3)
    message(AUTHOR_WARNING "Neither condition for 3")
  endif(DEFINED SYNTHETIC_DEF_3)
elseif(NOT SYNTHETIC_VAR_1 STREQUAL "value_1" AND SYNTHETIC_FALLBACK_1)
  message(WARNING "Fallback 1 depth 0")

  if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
    message(STATUS "Condition 4 depth 1 met")

    if(SYNTHETIC_VAR_9 STREQUAL "value_9")
      message(STATUS "Condition 9 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_19)
        message(STATUS "Condition 19 depth 3 met")

        if(SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 39 depth 4 met")
        elseif(NOT SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_39)
          message(WARNING "Fallback 39 depth 4")
        else(SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 39")
        endif(SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_19 AND SYNTHETIC_FALLBACK_19)
        message(WARNING "Fallback 19 depth 3")
      else(DEFINED SYNTHETIC_DEF_19)
        message(AUTHOR_WARNING "Neither condition for 19")
      endif(DEFINED SYNTHETIC_DEF_19)
    elseif(NOT SYNTHETIC_VAR_9 STREQUAL "value_9" AND SYNTHETIC_FALLBACK_9)
      message(WARNING "Fallback 9 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt)
        message(STATUS "Condition 20 depth 3 met")

        if(SYNTHETIC_VAR_41 STREQUAL "value_41")
          message(STATUS "Condition 41 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_41 STREQUAL "value_41" AND SYNTHETIC_FALLBACK_41)
          message(WARNING "Fallback 41 depth 4")
        else(SYNTHETIC_VAR_41 STREQUAL "value_41")
          message(AUTHOR_WARNING "Neither condition for 41")
        endif(SYNTHETIC_VAR_41 STREQUAL "value_41")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt AND SYNTHETIC_FALLBACK_20)
        message(WARNING "Fallback 20 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt)
        message(AUTHOR_WARNING "Neither condition for 20")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt)
    else(SYNTHETIC_VAR_9 STREQUAL "value_9")
      message(AUTHOR_WARNING "Neither condition for 9")
    endif(SYNTHETIC_VAR_9 STREQUAL "value_9")
  elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt AND SYNTHETIC_FALLBACK_4)
    message(WARNING "Fallback 4 depth 1")

    if(SYNTHETIC_NUM_10 GREATER 100)
      message(STATUS "Condition 10 depth 2 met")

      if(SYNTHETIC_LIST_21 MATCHES "^pattern")
        message(STATUS "Condition 21 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_43)
          message(STATUS "Condition 43 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_43 AND SYNTHETIC_FALLBACK_43)
          message(WARNING "Fallback 43 depth 4")
        else(DEFINED SYNTHETIC_DEF_43)
          message(AUTHOR_WARNING "Neither condition for 43")
        endif(DEFINED SYNTHETIC_DEF_43)
      elseif(NOT SYNTHETIC_LIST_21 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_21)
        message(WARNING "Fallback 21 depth 3")
      else(SYNTHETIC_LIST_21 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 21")
      endif(SYNTHETIC_LIST_21 MATCHES "^pattern")
    elseif(NOT SYNTHETIC_NUM_10 GREATER 100 AND SYNTHETIC_FALLBACK_10)
      message(WARNING "Fallback 10 depth 2")

      if(TARGET synthetic_target_22)
        message(STATUS "Condition 22 depth 3 met")

        if(SYNTHETIC_LIST_45 MATCHES "^pattern")
          message(STATUS "Condition 45 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_45 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_45)
          message(WARNING "Fallback 45 depth 4")
        else(SYNTHETIC_LIST_45 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 45")
        endif(SYNTHETIC_LIST_45 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_22 AND SYNTHETIC_FALLBACK_22)
        message(WARNING "Fallback 22 depth 3")
      else(TARGET synthetic_target_22)
        message(AUTHOR_WARNING "Neither condition for 22")
      endif(TARGET synthetic_target_22)
    else(SYNTHETIC_NUM_10 GREATER 100)
      message(AUTHOR_WARNING "Neither condition for 10")
    endif(SYNTHETIC_NUM_10 GREATER 100)
  else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
    message(AUTHOR_WARNING "Neither condition for 4")
  endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
else(SYNTHETIC_VAR_1 STREQUAL "value_1")
  message(AUTHOR_WARNING "Neither condition for 1")
endif(SYNTHETIC_VAR_1 STREQUAL "value_1")

if(SYNTHETIC_NUM_2 GREATER 100)
  message(STATUS "Condition 2 depth 0 met")

  if(SYNTHETIC_LIST_5 MATCHES "^pattern")
    message(STATUS "Condition 5 depth 1 met")

    if(DEFINED SYNTHETIC_DEF_11)
      message(STATUS "Condition 11 depth 2 met")

      if(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 23 depth 3 met")

        if(SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 47 depth 4 met")
        elseif(NOT SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_47)
          message(WARNING "Fallback 47 depth 4")
        else(SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 47")
        endif(SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_23)
        message(WARNING "Fallback 23 depth 3")
      else(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 23")
      endif(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT DEFINED SYNTHETIC_DEF_11 AND SYNTHETIC_FALLBACK_11)
      message(WARNING "Fallback 11 depth 2")

      if(SYNTHETIC_COND_24_A)
        message(STATUS "Condition 24 depth 3 met")

        if(SYNTHETIC_VAR_49 STREQUAL "value_49")
          message(STATUS "Condition 49 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_49 STREQUAL "value_49" AND SYNTHETIC_FALLBACK_49)
          message(WARNING "Fallback 49 depth 4")
        else(SYNTHETIC_VAR_49 STREQUAL "value_49")
          message(AUTHOR_WARNING "Neither condition for 49")
        endif(SYNTHETIC_VAR_49 STREQUAL "value_49")
      elseif(NOT SYNTHETIC_COND_24_A AND SYNTHETIC_FALLBACK_24)
        message(WARNING "Fallback 24 depth 3")
      else(SYNTHETIC_COND_24_A)
        message(AUTHOR_WARNING "Neither condition for 24")
      endif(SYNTHETIC_COND_24_A)
    else(DEFINED SYNTHETIC_DEF_11)
      message(AUTHOR_WARNING "Neither condition for 11")
    endif(DEFINED SYNTHETIC_DEF_11)
  elseif(NOT SYNTHETIC_LIST_5 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_5)
    message(WARNING "Fallback 5 depth 1")

    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
      message(STATUS "Condition 12 depth 2 met")

      if(SYNTHETIC_VAR_25 STREQUAL "value_25")
        message(STATUS "Condition 25 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_51)
          message(STATUS "Condition 51 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_51 AND SYNTHETIC_FALLBACK_51)
          message(WARNING "Fallback 51 depth 4")
        else(DEFINED SYNTHETIC_DEF_51)
          message(AUTHOR_WARNING "Neither condition for 51")
        endif(DEFINED SYNTHETIC_DEF_51)
      elseif(NOT SYNTHETIC_VAR_25 STREQUAL "value_25" AND SYNTHETIC_FALLBACK_25)
        message(WARNING "Fallback 25 depth 3")
      else(SYNTHETIC_VAR_25 STREQUAL "value_25")
        message(AUTHOR_WARNING "Neither condition for 25")
      endif(SYNTHETIC_VAR_25 STREQUAL "value_25")
    elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt AND SYNTHETIC_FALLBACK_12)
      message(WARNING "Fallback 12 depth 2")

      if(SYNTHETIC_NUM_26 GREATER 100)
        message(STATUS "Condition 26 depth 3 met")

        if(SYNTHETIC_LIST_53 MATCHES "^pattern")
          message(STATUS "Condition 53 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_53 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_53)
          message(WARNING "Fallback 53 depth 4")
        else(SYNTHETIC_LIST_53 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 53")
        endif(SYNTHETIC_LIST_53 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_26 GREATER 100 AND SYNTHETIC_FALLBACK_26)
        message(WARNING "Fallback 26 depth 3")
      else(SYNTHETIC_NUM_26 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 26")
      endif(SYNTHETIC_NUM_26 GREATER 100)
    else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
      message(AUTHOR_WARNING "Neither condition for 12")
    endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
  else(SYNTHETIC_LIST_5 MATCHES "^pattern")
    message(AUTHOR_WARNING "Neither condition for 5")
  endif(SYNTHETIC_LIST_5 MATCHES "^pattern")
elseif(NOT SYNTHETIC_NUM_2 GREATER 100 AND SYNTHETIC_FALLBACK_2)
  message(WARNING "Fallback 2 depth 0")

  if(TARGET synthetic_target_6)
    message(STATUS "Condition 6 depth 1 met")

    if(SYNTHETIC_LIST_13 MATCHES "^pattern")
      message(STATUS "Condition 13 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_27)
        message(STATUS "Condition 27 depth 3 met")

        if(SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 55 depth 4 met")
        elseif(NOT SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_55)
          message(WARNING "Fallback 55 depth 4")
        else(SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 55")
        endif(SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_27 AND SYNTHETIC_FALLBACK_27)
        message(WARNING "Fallback 27 depth 3")
      else(DEFINED SYNTHETIC_DEF_27)
        message(AUTHOR_WARNING "Neither condition for 27")
      endif(DEFINED SYNTHETIC_DEF_27)
    elseif(NOT SYNTHETIC_LIST_13 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_13)
      message(WARNING "Fallback 13 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt)
        message(STATUS "Condition 28 depth 3 met")

        if(SYNTHETIC_VAR_57 STREQUAL "value_57")
          message(STATUS "Condition 57 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_57 STREQUAL "value_57" AND SYNTHETIC_FALLBACK_57)
          message(WARNING "Fallback 57 depth 4")
        else(SYNTHETIC_VAR_57 STREQUAL "value_57")
          message(AUTHOR_WARNING "Neither condition for 57")
        endif(SYNTHETIC_VAR_57 STREQUAL "value_57")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt AND SYNTHETIC_FALLBACK_28)
        message(WARNING "Fallback 28 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt)
        message(AUTHOR_WARNING "Neither condition for 28")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt)
    else(SYNTHETIC_LIST_13 MATCHES "^pattern")
      message(AUTHOR_WARNING "Neither condition for 13")
    endif(SYNTHETIC_LIST_13 MATCHES "^pattern")
  elseif(NOT TARGET synthetic_target_6 AND SYNTHETIC_FALLBACK_6)
    message(WARNING "Fallback 6 depth 1")

    if(TARGET synthetic_target_14)
      message(STATUS "Condition 14 depth 2 met")

      if(SYNTHETIC_LIST_29 MATCHES "^pattern")
        message(STATUS "Condition 29 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_59)
          message(STATUS "Condition 59 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_59 AND SYNTHETIC_FALLBACK_59)
          message(WARNING "Fallback 59 depth 4")
        else(DEFINED SYNTHETIC_DEF_59)
          message(AUTHOR_WARNING "Neither condition for 59")
        endif(DEFINED SYNTHETIC_DEF_59)
      elseif(NOT SYNTHETIC_LIST_29 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_29)
        message(WARNING "Fallback 29 depth 3")
      else(SYNTHETIC_LIST_29 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 29")
      endif(SYNTHETIC_LIST_29 MATCHES "^pattern")
    elseif(NOT TARGET synthetic_target_14 AND SYNTHETIC_FALLBACK_14)
      message(WARNING "Fallback 14 depth 2")

      if(TARGET synthetic_target_30)
        message(STATUS "Condition 30 depth 3 met")

        if(SYNTHETIC_LIST_61 MATCHES "^pattern")
          message(STATUS "Condition 61 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_61 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_61)
          message(WARNING "Fallback 61 depth 4")
        else(SYNTHETIC_LIST_61 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 61")
        endif(SYNTHETIC_LIST_61 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_30 AND SYNTHETIC_FALLBACK_30)
        message(WARNING "Fallback 30 depth 3")
      else(TARGET synthetic_target_30)
        message(AUTHOR_WARNING "Neither condition for 30")
      endif(TARGET synthetic_target_30)
    else(TARGET synthetic_target_14)
      message(AUTHOR_WARNING "Neither condition for 14")
    endif(TARGET synthetic_target_14)
  else(TARGET synthetic_target_6)
    message(AUTHOR_WARNING "Neither condition for 6")
  endif(TARGET synthetic_target_6)
else(SYNTHETIC_NUM_2 GREATER 100)
  message(AUTHOR_WARNING "Neither condition for 2")
endif(SYNTHETIC_NUM_2 GREATER 100)

if(DEFINED SYNTHETIC_DEF_3)
  message(STATUS "Condition 3 depth 0 met")

  if(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
    message(STATUS "Condition 7 depth 1 met")

    if(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
      message(STATUS "Condition 15 depth 2 met")

      if(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 31 depth 3 met")

        if(SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 63 depth 4 met")
        elseif(NOT SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_63)
          message(WARNING "Fallback 63 depth 4")
        else(SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 63")
        endif(SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_31)
        message(WARNING "Fallback 31 depth 3")
      else(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 31")
      endif(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_15)
      message(WARNING "Fallback 15 depth 2")

      if(SYNTHETIC_COND_32_A)
        message(STATUS "Condition 32 depth 3 met")

        if(SYNTHETIC_VAR_65 STREQUAL "value_65")
          message(STATUS "Condition 65 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_65 STREQUAL "value_65" AND SYNTHETIC_FALLBACK_65)
          message(WARNING "Fallback 65 depth 4")
        else(SYNTHETIC_VAR_65 STREQUAL "value_65")
          message(AUTHOR_WARNING "Neither condition for 65")
        endif(SYNTHETIC_VAR_65 STREQUAL "value_65")
      elseif(NOT SYNTHETIC_COND_32_A AND SYNTHETIC_FALLBACK_32)
        message(WARNING "Fallback 32 depth 3")
      else(SYNTHETIC_COND_32_A)
        message(AUTHOR_WARNING "Neither condition for 32")
      endif(SYNTHETIC_COND_32_A)
    else(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
      message(AUTHOR_WARNING "Neither condition for 15")
    endif(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
  elseif(NOT SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_7)
    message(WARNING "Fallback 7 depth 1")

    if(SYNTHETIC_COND_16_A)
      message(STATUS "Condition 16 depth 2 met")

      if(SYNTHETIC_VAR_33 STREQUAL "value_33")
        message(STATUS "Condition 33 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_67)
          message(STATUS "Condition 67 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_67 AND SYNTHETIC_FALLBACK_67)
          message(WARNING "Fallback 67 depth 4")
        else(DEFINED SYNTHETIC_DEF_67)
          message(AUTHOR_WARNING "Neither condition for 67")
        endif(DEFINED SYNTHETIC_DEF_67)
      elseif(NOT SYNTHETIC_VAR_33 STREQUAL "value_33" AND SYNTHETIC_FALLBACK_33)
        message(WARNING "Fallback 33 depth 3")
      else(SYNTHETIC_VAR_33 STREQUAL "value_33")
        message(AUTHOR_WARNING "Neither condition for 33")
      endif(SYNTHETIC_VAR_33 STREQUAL "value_33")
    elseif(NOT SYNTHETIC_COND_16_A AND SYNTHETIC_FALLBACK_16)
      message(WARNING "Fallback 16 depth 2")

      if(SYNTHETIC_NUM_34 GREATER 100)
        message(STATUS "Condition 34 depth 3 met")

        if(SYNTHETIC_LIST_69 MATCHES "^pattern")
          message(STATUS "Condition 69 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_69 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_69)
          message(WARNING "Fallback 69 depth 4")
        else(SYNTHETIC_LIST_69 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 69")
        endif(SYNTHETIC_LIST_69 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_34 GREATER 100 AND SYNTHETIC_FALLBACK_34)
        message(WARNING "Fallback 34 depth 3")
      else(SYNTHETIC_NUM_34 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 34")
      endif(SYNTHETIC_NUM_34 GREATER 100)
    else(SYNTHETIC_COND_16_A)
      message(AUTHOR_WARNING "Neither condition for 16")
    endif(SYNTHETIC_COND_16_A)
  else(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
    message(AUTHOR_WARNING "Neither condition for 7")
  endif(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
elseif(NOT DEFINED SYNTHETIC_DEF_3 AND SYNTHETIC_FALLBACK_3)
  message(WARNING "Fallback 3 depth 0")

  if(SYNTHETIC_COND_8_A)
    message(STATUS "Condition 8 depth 1 met")

    if(SYNTHETIC_VAR_17 STREQUAL "value_17")
      message(STATUS "Condition 17 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_35)
        message(STATUS "Condition 35 depth 3 met")

        if(SYNTHETIC_A_71 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 71 depth 4 met")
        elseif(NOT SYNTHETIC_A_71 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_71)
          message(WARNING "Fallback 71 depth 4")
        else(SYNTHETIC_A_71 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 71")
        endif(SYNTHETIC_A_71 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_35 AND SYNTHETIC_FALLBACK_35)
        message(WARNING "Fallback 35 depth 3")
      else(DEFINED SYNTHETIC_DEF_35)
        message(AUTHOR_WARNING "Neither condition for 35")
      endif(DEFINED SYNTHETIC_DEF_35)
    elseif(NOT SYNTHETIC_VAR_17 STREQUAL "value_17" AND SYNTHETIC_FALLBACK_17)
      message(WARNING "Fallback 17 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_36.txt)
        message(STATUS "Condition 36 depth 3 met")

        if(SYNTHETIC_VAR_73 STREQUAL "value_73")
          message(STATUS "Condition 73 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_73 STREQUAL "value_73" AND SYNTHETIC_FALLBACK_73)
          message(WARNING "Fallback 73 depth 4")
        else(SYNTHETIC_VAR_73 STREQUAL "value_73")
          message(AUTHOR_WARNING "Neither condition for 73")
        endif(SYNTHETIC_VAR_73 STREQUAL "value_73")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_36.txt AND SYNTHETIC_FALLBACK_36)
        message(WARNING "Fallback 36 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_36.txt)
        message(AUTHOR_WARNING "Neither condition for 36")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_36.txt)
    else(SYNTHETIC_VAR_17 STREQUAL "value_17")
      message(AUTHOR_WARNING "Neither condition for 17")
    endif(SYNTHETIC_VAR_17 STREQUAL "value_17")
  elseif(NOT SYNTHETIC_COND_8_A AND SYNTHETIC_FALLBACK_8)
    message(WARNING "Fallback 8 depth 1")

    if(SYNTHETIC_NUM_18 GREATER 100)
      message(STATUS "Condition 18 depth 2 met")

      if(SYNTHETIC_LIST_37 MATCHES "^pattern")
        message(STATUS "Condition 37 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_75)
          message(STATUS "Condition 75 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_75 AND SYNTHETIC_FALLBACK_75)
          message(WARNING "Fallback 75 depth 4")
        else(DEFINED SYNTHETIC_DEF_75)
          message(AUTHOR_WARNING "Neither condition for 75")
        endif(DEFINED SYNTHETIC_DEF_75)
      elseif(NOT SYNTHETIC_LIST_37 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_37)
        message(WARNING "Fallback 37 depth 3")
      else(SYNTHETIC_LIST_37 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 37")
      endif(SYNTHETIC_LIST_37 MATCHES "^pattern")
    elseif(NOT SYNTHETIC_NUM_18 GREATER 100 AND SYNTHETIC_FALLBACK_18)
      message(WARNING "Fallback 18 depth 2")

      if(TARGET synthetic_target_38)
        message(STATUS "Condition 38 depth 3 met")

        if(SYNTHETIC_LIST_77 MATCHES "^pattern")
          message(STATUS "Condition 77 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_77 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_77)
          message(WARNING "Fallback 77 depth 4")
        else(SYNTHETIC_LIST_77 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 77")
        endif(SYNTHETIC_LIST_77 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_38 AND SYNTHETIC_FALLBACK_38)
        message(WARNING "Fallback 38 depth 3")
      else(TARGET synthetic_target_38)
        message(AUTHOR_WARNING "Neither condition for 38")
      endif(TARGET synthetic_target_38)
    else(SYNTHETIC_NUM_18 GREATER 100)
      message(AUTHOR_WARNING "Neither condition for 18")
    endif(SYNTHETIC_NUM_18 GREATER 100)
  else(SYNTHETIC_COND_8_A)
    message(AUTHOR_WARNING "Neither condition for 8")
  endif(SYNTHETIC_COND_8_A)
else(DEFINED SYNTHETIC_DEF_3)
  message(AUTHOR_WARNING "Neither condition for 3")
endif(DEFINED SYNTHETIC_DEF_3)

if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
  message(STATUS "Condition 4 depth 0 met")

  if(SYNTHETIC_VAR_9 STREQUAL "value_9")
    message(STATUS "Condition 9 depth 1 met")

    if(DEFINED SYNTHETIC_DEF_19)
      message(STATUS "Condition 19 depth 2 met")

      if(SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 39 depth 3 met")

        if(SYNTHETIC_A_79 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 79 depth 4 met")
        elseif(NOT SYNTHETIC_A_79 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_79)
          message(WARNING "Fallback 79 depth 4")
        else(SYNTHETIC_A_79 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 79")
        endif(SYNTHETIC_A_79 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_39)
        message(WARNING "Fallback 39 depth 3")
      else(SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 39")
      endif(SYNTHETIC_A_39 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT DEFINED SYNTHETIC_DEF_19 AND SYNTHETIC_FALLBACK_19)
      message(WARNING "Fallback 19 depth 2")

      if(SYNTHETIC_COND_40_A)
        message(STATUS "Condition 40 depth 3 met")

        if(SYNTHETIC_VAR_81 STREQUAL "value_81")
          message(STATUS "Condition 81 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_81 STREQUAL "value_81" AND SYNTHETIC_FALLBACK_81)
          message(WARNING "Fallback 81 depth 4")
        else(SYNTHETIC_VAR_81 STREQUAL "value_81")
          message(AUTHOR_WARNING "Neither condition for 81")
        endif(SYNTHETIC_VAR_81 STREQUAL "value_81")
      elseif(NOT SYNTHETIC_COND_40_A AND SYNTHETIC_FALLBACK_40)
        message(WARNING "Fallback 40 depth 3")
      else(SYNTHETIC_COND_40_A)
        message(AUTHOR_WARNING "Neither condition for 40")
      endif(SYNTHETIC_COND_40_A)
    else(DEFINED SYNTHETIC_DEF_19)
      message(AUTHOR_WARNING "Neither condition for 19")
    endif(DEFINED SYNTHETIC_DEF_19)
  elseif(NOT SYNTHETIC_VAR_9 STREQUAL "value_9" AND SYNTHETIC_FALLBACK_9)
    message(WARNING "Fallback 9 depth 1")

    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt)
      message(STATUS "Condition 20 depth 2 met")

      if(SYNTHETIC_VAR_41 STREQUAL "value_41")
        message(STATUS "Condition 41 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_83)
          message(STATUS "Condition 83 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_83 AND SYNTHETIC_FALLBACK_83)
          message(WARNING "Fallback 83 depth 4")
        else(DEFINED SYNTHETIC_DEF_83)
          message(AUTHOR_WARNING "Neither condition for 83")
        endif(DEFINED SYNTHETIC_DEF_83)
      elseif(NOT SYNTHETIC_VAR_41 STREQUAL "value_41" AND SYNTHETIC_FALLBACK_41)
        message(WARNING "Fallback 41 depth 3")
      else(SYNTHETIC_VAR_41 STREQUAL "value_41")
        message(AUTHOR_WARNING "Neither condition for 41")
      endif(SYNTHETIC_VAR_41 STREQUAL "value_41")
    elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt AND SYNTHETIC_FALLBACK_20)
      message(WARNING "Fallback 20 depth 2")

      if(SYNTHETIC_NUM_42 GREATER 100)
        message(STATUS "Condition 42 depth 3 met")

        if(SYNTHETIC_LIST_85 MATCHES "^pattern")
          message(STATUS "Condition 85 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_85 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_85)
          message(WARNING "Fallback 85 depth 4")
        else(SYNTHETIC_LIST_85 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 85")
        endif(SYNTHETIC_LIST_85 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_42 GREATER 100 AND SYNTHETIC_FALLBACK_42)
        message(WARNING "Fallback 42 depth 3")
      else(SYNTHETIC_NUM_42 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 42")
      endif(SYNTHETIC_NUM_42 GREATER 100)
    else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt)
      message(AUTHOR_WARNING "Neither condition for 20")
    endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_20.txt)
  else(SYNTHETIC_VAR_9 STREQUAL "value_9")
    message(AUTHOR_WARNING "Neither condition for 9")
  endif(SYNTHETIC_VAR_9 STREQUAL "value_9")
elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt AND SYNTHETIC_FALLBACK_4)
  message(WARNING "Fallback 4 depth 0")

  if(SYNTHETIC_NUM_10 GREATER 100)
    message(STATUS "Condition 10 depth 1 met")

    if(SYNTHETIC_LIST_21 MATCHES "^pattern")
      message(STATUS "Condition 21 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_43)
        message(STATUS "Condition 43 depth 3 met")

        if(SYNTHETIC_A_87 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 87 depth 4 met")
        elseif(NOT SYNTHETIC_A_87 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_87)
          message(WARNING "Fallback 87 depth 4")
        else(SYNTHETIC_A_87 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 87")
        endif(SYNTHETIC_A_87 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_43 AND SYNTHETIC_FALLBACK_43)
        message(WARNING "Fallback 43 depth 3")
      else(DEFINED SYNTHETIC_DEF_43)
        message(AUTHOR_WARNING "Neither condition for 43")
      endif(DEFINED SYNTHETIC_DEF_43)
    elseif(NOT SYNTHETIC_LIST_21 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_21)
      message(WARNING "Fallback 21 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_44.txt)
        message(STATUS "Condition 44 depth 3 met")

        if(SYNTHETIC_VAR_89 STREQUAL "value_89")
          message(STATUS "Condition 89 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_89 STREQUAL "value_89" AND SYNTHETIC_FALLBACK_89)
          message(WARNING "Fallback 89 depth 4")
        else(SYNTHETIC_VAR_89 STREQUAL "value_89")
          message(AUTHOR_WARNING "Neither condition for 89")
        endif(SYNTHETIC_VAR_89 STREQUAL "value_89")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_44.txt AND SYNTHETIC_FALLBACK_44)
        message(WARNING "Fallback 44 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_44.txt)
        message(AUTHOR_WARNING "Neither condition for 44")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_44.txt)
    else(SYNTHETIC_LIST_21 MATCHES "^pattern")
      message(AUTHOR_WARNING "Neither condition for 21")
    endif(SYNTHETIC_LIST_21 MATCHES "^pattern")
  elseif(NOT SYNTHETIC_NUM_10 GREATER 100 AND SYNTHETIC_FALLBACK_10)
    message(WARNING "Fallback 10 depth 1")

    if(TARGET synthetic_target_22)
      message(STATUS "Condition 22 depth 2 met")

      if(SYNTHETIC_LIST_45 MATCHES "^pattern")
        message(STATUS "Condition 45 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_91)
          message(STATUS "Condition 91 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_91 AND SYNTHETIC_FALLBACK_91)
          message(WARNING "Fallback 91 depth 4")
        else(DEFINED SYNTHETIC_DEF_91)
          message(AUTHOR_WARNING "Neither condition for 91")
        endif(DEFINED SYNTHETIC_DEF_91)
      elseif(NOT SYNTHETIC_LIST_45 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_45)
        message(WARNING "Fallback 45 depth 3")
      else(SYNTHETIC_LIST_45 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 45")
      endif(SYNTHETIC_LIST_45 MATCHES "^pattern")
    elseif(NOT TARGET synthetic_target_22 AND SYNTHETIC_FALLBACK_22)
      message(WARNING "Fallback 22 depth 2")

      if(TARGET synthetic_target_46)
        message(STATUS "Condition 46 depth 3 met")

        if(SYNTHETIC_LIST_93 MATCHES "^pattern")
          message(STATUS "Condition 93 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_93 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_93)
          message(WARNING "Fallback 93 depth 4")
        else(SYNTHETIC_LIST_93 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 93")
        endif(SYNTHETIC_LIST_93 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_46 AND SYNTHETIC_FALLBACK_46)
        message(WARNING "Fallback 46 depth 3")
      else(TARGET synthetic_target_46)
        message(AUTHOR_WARNING "Neither condition for 46")
      endif(TARGET synthetic_target_46)
    else(TARGET synthetic_target_22)
      message(AUTHOR_WARNING "Neither condition for 22")
    endif(TARGET synthetic_target_22)
  else(SYNTHETIC_NUM_10 GREATER 100)
    message(AUTHOR_WARNING "Neither condition for 10")
  endif(SYNTHETIC_NUM_10 GREATER 100)
else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)
  message(AUTHOR_WARNING "Neither condition for 4")
endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_4.txt)

if(SYNTHETIC_LIST_5 MATCHES "^pattern")
  message(STATUS "Condition 5 depth 0 met")

  if(DEFINED SYNTHETIC_DEF_11)
    message(STATUS "Condition 11 depth 1 met")

    if(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
      message(STATUS "Condition 23 depth 2 met")

      if(SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 47 depth 3 met")

        if(SYNTHETIC_A_95 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 95 depth 4 met")
        elseif(NOT SYNTHETIC_A_95 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_95)
          message(WARNING "Fallback 95 depth 4")
        else(SYNTHETIC_A_95 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 95")
        endif(SYNTHETIC_A_95 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_47)
        message(WARNING "Fallback 47 depth 3")
      else(SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 47")
      endif(SYNTHETIC_A_47 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_23)
      message(WARNING "Fallback 23 depth 2")

      if(SYNTHETIC_COND_48_A)
        message(STATUS "Condition 48 depth 3 met")

        if(SYNTHETIC_VAR_97 STREQUAL "value_97")
          message(STATUS "Condition 97 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_97 STREQUAL "value_97" AND SYNTHETIC_FALLBACK_97)
          message(WARNING "Fallback 97 depth 4")
        else(SYNTHETIC_VAR_97 STREQUAL "value_97")
          message(AUTHOR_WARNING "Neither condition for 97")
        endif(SYNTHETIC_VAR_97 STREQUAL "value_97")
      elseif(NOT SYNTHETIC_COND_48_A AND SYNTHETIC_FALLBACK_48)
        message(WARNING "Fallback 48 depth 3")
      else(SYNTHETIC_COND_48_A)
        message(AUTHOR_WARNING "Neither condition for 48")
      endif(SYNTHETIC_COND_48_A)
    else(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
      message(AUTHOR_WARNING "Neither condition for 23")
    endif(SYNTHETIC_A_23 VERSION_GREATER_EQUAL 1.2.3)
  elseif(NOT DEFINED SYNTHETIC_DEF_11 AND SYNTHETIC_FALLBACK_11)
    message(WARNING "Fallback 11 depth 1")

    if(SYNTHETIC_COND_24_A)
      message(STATUS "Condition 24 depth 2 met")

      if(SYNTHETIC_VAR_49 STREQUAL "value_49")
        message(STATUS "Condition 49 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_99)
          message(STATUS "Condition 99 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_99 AND SYNTHETIC_FALLBACK_99)
          message(WARNING "Fallback 99 depth 4")
        else(DEFINED SYNTHETIC_DEF_99)
          message(AUTHOR_WARNING "Neither condition for 99")
        endif(DEFINED SYNTHETIC_DEF_99)
      elseif(NOT SYNTHETIC_VAR_49 STREQUAL "value_49" AND SYNTHETIC_FALLBACK_49)
        message(WARNING "Fallback 49 depth 3")
      else(SYNTHETIC_VAR_49 STREQUAL "value_49")
        message(AUTHOR_WARNING "Neither condition for 49")
      endif(SYNTHETIC_VAR_49 STREQUAL "value_49")
    elseif(NOT SYNTHETIC_COND_24_A AND SYNTHETIC_FALLBACK_24)
      message(WARNING "Fallback 24 depth 2")

      if(SYNTHETIC_NUM_50 GREATER 100)
        message(STATUS "Condition 50 depth 3 met")

        if(SYNTHETIC_LIST_101 MATCHES "^pattern")
          message(STATUS "Condition 101 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_101 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_101)
          message(WARNING "Fallback 101 depth 4")
        else(SYNTHETIC_LIST_101 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 101")
        endif(SYNTHETIC_LIST_101 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_50 GREATER 100 AND SYNTHETIC_FALLBACK_50)
        message(WARNING "Fallback 50 depth 3")
      else(SYNTHETIC_NUM_50 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 50")
      endif(SYNTHETIC_NUM_50 GREATER 100)
    else(SYNTHETIC_COND_24_A)
      message(AUTHOR_WARNING "Neither condition for 24")
    endif(SYNTHETIC_COND_24_A)
  else(DEFINED SYNTHETIC_DEF_11)
    message(AUTHOR_WARNING "Neither condition for 11")
  endif(DEFINED SYNTHETIC_DEF_11)
elseif(NOT SYNTHETIC_LIST_5 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_5)
  message(WARNING "Fallback 5 depth 0")

  if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
    message(STATUS "Condition 12 depth 1 met")

    if(SYNTHETIC_VAR_25 STREQUAL "value_25")
      message(STATUS "Condition 25 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_51)
        message(STATUS "Condition 51 depth 3 met")

        if(SYNTHETIC_A_103 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 103 depth 4 met")
        elseif(NOT SYNTHETIC_A_103 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_103)
          message(WARNING "Fallback 103 depth 4")
        else(SYNTHETIC_A_103 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 103")
        endif(SYNTHETIC_A_103 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_51 AND SYNTHETIC_FALLBACK_51)
        message(WARNING "Fallback 51 depth 3")
      else(DEFINED SYNTHETIC_DEF_51)
        message(AUTHOR_WARNING "Neither condition for 51")
      endif(DEFINED SYNTHETIC_DEF_51)
    elseif(NOT SYNTHETIC_VAR_25 STREQUAL "value_25" AND SYNTHETIC_FALLBACK_25)
      message(WARNING "Fallback 25 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_52.txt)
        message(STATUS "Condition 52 depth 3 met")

        if(SYNTHETIC_VAR_105 STREQUAL "value_105")
          message(STATUS "Condition 105 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_105 STREQUAL "value_105" AND SYNTHETIC_FALLBACK_105)
          message(WARNING "Fallback 105 depth 4")
        else(SYNTHETIC_VAR_105 STREQUAL "value_105")
          message(AUTHOR_WARNING "Neither condition for 105")
        endif(SYNTHETIC_VAR_105 STREQUAL "value_105")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_52.txt AND SYNTHETIC_FALLBACK_52)
        message(WARNING "Fallback 52 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_52.txt)
        message(AUTHOR_WARNING "Neither condition for 52")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_52.txt)
    else(SYNTHETIC_VAR_25 STREQUAL "value_25")
      message(AUTHOR_WARNING "Neither condition for 25")
    endif(SYNTHETIC_VAR_25 STREQUAL "value_25")
  elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt AND SYNTHETIC_FALLBACK_12)
    message(WARNING "Fallback 12 depth 1")

    if(SYNTHETIC_NUM_26 GREATER 100)
      message(STATUS "Condition 26 depth 2 met")

      if(SYNTHETIC_LIST_53 MATCHES "^pattern")
        message(STATUS "Condition 53 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_107)
          message(STATUS "Condition 107 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_107 AND SYNTHETIC_FALLBACK_107)
          message(WARNING "Fallback 107 depth 4")
        else(DEFINED SYNTHETIC_DEF_107)
          message(AUTHOR_WARNING "Neither condition for 107")
        endif(DEFINED SYNTHETIC_DEF_107)
      elseif(NOT SYNTHETIC_LIST_53 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_53)
        message(WARNING "Fallback 53 depth 3")
      else(SYNTHETIC_LIST_53 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 53")
      endif(SYNTHETIC_LIST_53 MATCHES "^pattern")
    elseif(NOT SYNTHETIC_NUM_26 GREATER 100 AND SYNTHETIC_FALLBACK_26)
      message(WARNING "Fallback 26 depth 2")

      if(TARGET synthetic_target_54)
        message(STATUS "Condition 54 depth 3 met")

        if(SYNTHETIC_LIST_109 MATCHES "^pattern")
          message(STATUS "Condition 109 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_109 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_109)
          message(WARNING "Fallback 109 depth 4")
        else(SYNTHETIC_LIST_109 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 109")
        endif(SYNTHETIC_LIST_109 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_54 AND SYNTHETIC_FALLBACK_54)
        message(WARNING "Fallback 54 depth 3")
      else(TARGET synthetic_target_54)
        message(AUTHOR_WARNING "Neither condition for 54")
      endif(TARGET synthetic_target_54)
    else(SYNTHETIC_NUM_26 GREATER 100)
      message(AUTHOR_WARNING "Neither condition for 26")
    endif(SYNTHETIC_NUM_26 GREATER 100)
  else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
    message(AUTHOR_WARNING "Neither condition for 12")
  endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_12.txt)
else(SYNTHETIC_LIST_5 MATCHES "^pattern")
  message(AUTHOR_WARNING "Neither condition for 5")
endif(SYNTHETIC_LIST_5 MATCHES "^pattern")

if(TARGET synthetic_target_6)
  message(STATUS "Condition 6 depth 0 met")

  if(SYNTHETIC_LIST_13 MATCHES "^pattern")
    message(STATUS "Condition 13 depth 1 met")

    if(DEFINED SYNTHETIC_DEF_27)
      message(STATUS "Condition 27 depth 2 met")

      if(SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 55 depth 3 met")

        if(SYNTHETIC_A_111 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 111 depth 4 met")
        elseif(NOT SYNTHETIC_A_111 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_111)
          message(WARNING "Fallback 111 depth 4")
        else(SYNTHETIC_A_111 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 111")
        endif(SYNTHETIC_A_111 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_55)
        message(WARNING "Fallback 55 depth 3")
      else(SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 55")
      endif(SYNTHETIC_A_55 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT DEFINED SYNTHETIC_DEF_27 AND SYNTHETIC_FALLBACK_27)
      message(WARNING "Fallback 27 depth 2")

      if(SYNTHETIC_COND_56_A)
        message(STATUS "Condition 56 depth 3 met")

        if(SYNTHETIC_VAR_113 STREQUAL "value_113")
          message(STATUS "Condition 113 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_113 STREQUAL "value_113" AND SYNTHETIC_FALLBACK_113)
          message(WARNING "Fallback 113 depth 4")
        else(SYNTHETIC_VAR_113 STREQUAL "value_113")
          message(AUTHOR_WARNING "Neither condition for 113")
        endif(SYNTHETIC_VAR_113 STREQUAL "value_113")
      elseif(NOT SYNTHETIC_COND_56_A AND SYNTHETIC_FALLBACK_56)
        message(WARNING "Fallback 56 depth 3")
      else(SYNTHETIC_COND_56_A)
        message(AUTHOR_WARNING "Neither condition for 56")
      endif(SYNTHETIC_COND_56_A)
    else(DEFINED SYNTHETIC_DEF_27)
      message(AUTHOR_WARNING "Neither condition for 27")
    endif(DEFINED SYNTHETIC_DEF_27)
  elseif(NOT SYNTHETIC_LIST_13 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_13)
    message(WARNING "Fallback 13 depth 1")

    if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt)
      message(STATUS "Condition 28 depth 2 met")

      if(SYNTHETIC_VAR_57 STREQUAL "value_57")
        message(STATUS "Condition 57 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_115)
          message(STATUS "Condition 115 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_115 AND SYNTHETIC_FALLBACK_115)
          message(WARNING "Fallback 115 depth 4")
        else(DEFINED SYNTHETIC_DEF_115)
          message(AUTHOR_WARNING "Neither condition for 115")
        endif(DEFINED SYNTHETIC_DEF_115)
      elseif(NOT SYNTHETIC_VAR_57 STREQUAL "value_57" AND SYNTHETIC_FALLBACK_57)
        message(WARNING "Fallback 57 depth 3")
      else(SYNTHETIC_VAR_57 STREQUAL "value_57")
        message(AUTHOR_WARNING "Neither condition for 57")
      endif(SYNTHETIC_VAR_57 STREQUAL "value_57")
    elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt AND SYNTHETIC_FALLBACK_28)
      message(WARNING "Fallback 28 depth 2")

      if(SYNTHETIC_NUM_58 GREATER 100)
        message(STATUS "Condition 58 depth 3 met")

        if(SYNTHETIC_LIST_117 MATCHES "^pattern")
          message(STATUS "Condition 117 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_117 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_117)
          message(WARNING "Fallback 117 depth 4")
        else(SYNTHETIC_LIST_117 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 117")
        endif(SYNTHETIC_LIST_117 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_58 GREATER 100 AND SYNTHETIC_FALLBACK_58)
        message(WARNING "Fallback 58 depth 3")
      else(SYNTHETIC_NUM_58 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 58")
      endif(SYNTHETIC_NUM_58 GREATER 100)
    else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt)
      message(AUTHOR_WARNING "Neither condition for 28")
    endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_28.txt)
  else(SYNTHETIC_LIST_13 MATCHES "^pattern")
    message(AUTHOR_WARNING "Neither condition for 13")
  endif(SYNTHETIC_LIST_13 MATCHES "^pattern")
elseif(NOT TARGET synthetic_target_6 AND SYNTHETIC_FALLBACK_6)
  message(WARNING "Fallback 6 depth 0")

  if(TARGET synthetic_target_14)
    message(STATUS "Condition 14 depth 1 met")

    if(SYNTHETIC_LIST_29 MATCHES "^pattern")
      message(STATUS "Condition 29 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_59)
        message(STATUS "Condition 59 depth 3 met")

        if(SYNTHETIC_A_119 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 119 depth 4 met")
        elseif(NOT SYNTHETIC_A_119 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_119)
          message(WARNING "Fallback 119 depth 4")
        else(SYNTHETIC_A_119 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 119")
        endif(SYNTHETIC_A_119 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_59 AND SYNTHETIC_FALLBACK_59)
        message(WARNING "Fallback 59 depth 3")
      else(DEFINED SYNTHETIC_DEF_59)
        message(AUTHOR_WARNING "Neither condition for 59")
      endif(DEFINED SYNTHETIC_DEF_59)
    elseif(NOT SYNTHETIC_LIST_29 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_29)
      message(WARNING "Fallback 29 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_60.txt)
        message(STATUS "Condition 60 depth 3 met")

        if(SYNTHETIC_VAR_121 STREQUAL "value_121")
          message(STATUS "Condition 121 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_121 STREQUAL "value_121" AND SYNTHETIC_FALLBACK_121)
          message(WARNING "Fallback 121 depth 4")
        else(SYNTHETIC_VAR_121 STREQUAL "value_121")
          message(AUTHOR_WARNING "Neither condition for 121")
        endif(SYNTHETIC_VAR_121 STREQUAL "value_121")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_60.txt AND SYNTHETIC_FALLBACK_60)
        message(WARNING "Fallback 60 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_60.txt)
        message(AUTHOR_WARNING "Neither condition for 60")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_60.txt)
    else(SYNTHETIC_LIST_29 MATCHES "^pattern")
      message(AUTHOR_WARNING "Neither condition for 29")
    endif(SYNTHETIC_LIST_29 MATCHES "^pattern")
  elseif(NOT TARGET synthetic_target_14 AND SYNTHETIC_FALLBACK_14)
    message(WARNING "Fallback 14 depth 1")

    if(TARGET synthetic_target_30)
      message(STATUS "Condition 30 depth 2 met")

      if(SYNTHETIC_LIST_61 MATCHES "^pattern")
        message(STATUS "Condition 61 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_123)
          message(STATUS "Condition 123 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_123 AND SYNTHETIC_FALLBACK_123)
          message(WARNING "Fallback 123 depth 4")
        else(DEFINED SYNTHETIC_DEF_123)
          message(AUTHOR_WARNING "Neither condition for 123")
        endif(DEFINED SYNTHETIC_DEF_123)
      elseif(NOT SYNTHETIC_LIST_61 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_61)
        message(WARNING "Fallback 61 depth 3")
      else(SYNTHETIC_LIST_61 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 61")
      endif(SYNTHETIC_LIST_61 MATCHES "^pattern")
    elseif(NOT TARGET synthetic_target_30 AND SYNTHETIC_FALLBACK_30)
      message(WARNING "Fallback 30 depth 2")

      if(TARGET synthetic_target_62)
        message(STATUS "Condition 62 depth 3 met")

        if(SYNTHETIC_LIST_125 MATCHES "^pattern")
          message(STATUS "Condition 125 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_125 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_125)
          message(WARNING "Fallback 125 depth 4")
        else(SYNTHETIC_LIST_125 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 125")
        endif(SYNTHETIC_LIST_125 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_62 AND SYNTHETIC_FALLBACK_62)
        message(WARNING "Fallback 62 depth 3")
      else(TARGET synthetic_target_62)
        message(AUTHOR_WARNING "Neither condition for 62")
      endif(TARGET synthetic_target_62)
    else(TARGET synthetic_target_30)
      message(AUTHOR_WARNING "Neither condition for 30")
    endif(TARGET synthetic_target_30)
  else(TARGET synthetic_target_14)
    message(AUTHOR_WARNING "Neither condition for 14")
  endif(TARGET synthetic_target_14)
else(TARGET synthetic_target_6)
  message(AUTHOR_WARNING "Neither condition for 6")
endif(TARGET synthetic_target_6)

if(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
  message(STATUS "Condition 7 depth 0 met")

  if(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
    message(STATUS "Condition 15 depth 1 met")

    if(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
      message(STATUS "Condition 31 depth 2 met")

      if(SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3)
        message(STATUS "Condition 63 depth 3 met")

        if(SYNTHETIC_A_127 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 127 depth 4 met")
        elseif(NOT SYNTHETIC_A_127 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_127)
          message(WARNING "Fallback 127 depth 4")
        else(SYNTHETIC_A_127 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 127")
        endif(SYNTHETIC_A_127 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_63)
        message(WARNING "Fallback 63 depth 3")
      else(SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3)
        message(AUTHOR_WARNING "Neither condition for 63")
      endif(SYNTHETIC_A_63 VERSION_GREATER_EQUAL 1.2.3)
    elseif(NOT SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_31)
      message(WARNING "Fallback 31 depth 2")

      if(SYNTHETIC_COND_64_A)
        message(STATUS "Condition 64 depth 3 met")

        if(SYNTHETIC_VAR_129 STREQUAL "value_129")
          message(STATUS "Condition 129 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_129 STREQUAL "value_129" AND SYNTHETIC_FALLBACK_129)
          message(WARNING "Fallback 129 depth 4")
        else(SYNTHETIC_VAR_129 STREQUAL "value_129")
          message(AUTHOR_WARNING "Neither condition for 129")
        endif(SYNTHETIC_VAR_129 STREQUAL "value_129")
      elseif(NOT SYNTHETIC_COND_64_A AND SYNTHETIC_FALLBACK_64)
        message(WARNING "Fallback 64 depth 3")
      else(SYNTHETIC_COND_64_A)
        message(AUTHOR_WARNING "Neither condition for 64")
      endif(SYNTHETIC_COND_64_A)
    else(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
      message(AUTHOR_WARNING "Neither condition for 31")
    endif(SYNTHETIC_A_31 VERSION_GREATER_EQUAL 1.2.3)
  elseif(NOT SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_15)
    message(WARNING "Fallback 15 depth 1")

    if(SYNTHETIC_COND_32_A)
      message(STATUS "Condition 32 depth 2 met")

      if(SYNTHETIC_VAR_65 STREQUAL "value_65")
        message(STATUS "Condition 65 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_131)
          message(STATUS "Condition 131 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_131 AND SYNTHETIC_FALLBACK_131)
          message(WARNING "Fallback 131 depth 4")
        else(DEFINED SYNTHETIC_DEF_131)
          message(AUTHOR_WARNING "Neither condition for 131")
        endif(DEFINED SYNTHETIC_DEF_131)
      elseif(NOT SYNTHETIC_VAR_65 STREQUAL "value_65" AND SYNTHETIC_FALLBACK_65)
        message(WARNING "Fallback 65 depth 3")
      else(SYNTHETIC_VAR_65 STREQUAL "value_65")
        message(AUTHOR_WARNING "Neither condition for 65")
      endif(SYNTHETIC_VAR_65 STREQUAL "value_65")
    elseif(NOT SYNTHETIC_COND_32_A AND SYNTHETIC_FALLBACK_32)
      message(WARNING "Fallback 32 depth 2")

      if(SYNTHETIC_NUM_66 GREATER 100)
        message(STATUS "Condition 66 depth 3 met")

        if(SYNTHETIC_LIST_133 MATCHES "^pattern")
          message(STATUS "Condition 133 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_133 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_133)
          message(WARNING "Fallback 133 depth 4")
        else(SYNTHETIC_LIST_133 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 133")
        endif(SYNTHETIC_LIST_133 MATCHES "^pattern")
      elseif(NOT SYNTHETIC_NUM_66 GREATER 100 AND SYNTHETIC_FALLBACK_66)
        message(WARNING "Fallback 66 depth 3")
      else(SYNTHETIC_NUM_66 GREATER 100)
        message(AUTHOR_WARNING "Neither condition for 66")
      endif(SYNTHETIC_NUM_66 GREATER 100)
    else(SYNTHETIC_COND_32_A)
      message(AUTHOR_WARNING "Neither condition for 32")
    endif(SYNTHETIC_COND_32_A)
  else(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
    message(AUTHOR_WARNING "Neither condition for 15")
  endif(SYNTHETIC_A_15 VERSION_GREATER_EQUAL 1.2.3)
elseif(NOT SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_7)
  message(WARNING "Fallback 7 depth 0")

  if(SYNTHETIC_COND_16_A)
    message(STATUS "Condition 16 depth 1 met")

    if(SYNTHETIC_VAR_33 STREQUAL "value_33")
      message(STATUS "Condition 33 depth 2 met")

      if(DEFINED SYNTHETIC_DEF_67)
        message(STATUS "Condition 67 depth 3 met")

        if(SYNTHETIC_A_135 VERSION_GREATER_EQUAL 1.2.3)
          message(STATUS "Condition 135 depth 4 met")
        elseif(NOT SYNTHETIC_A_135 VERSION_GREATER_EQUAL 1.2.3 AND SYNTHETIC_FALLBACK_135)
          message(WARNING "Fallback 135 depth 4")
        else(SYNTHETIC_A_135 VERSION_GREATER_EQUAL 1.2.3)
          message(AUTHOR_WARNING "Neither condition for 135")
        endif(SYNTHETIC_A_135 VERSION_GREATER_EQUAL 1.2.3)
      elseif(NOT DEFINED SYNTHETIC_DEF_67 AND SYNTHETIC_FALLBACK_67)
        message(WARNING "Fallback 67 depth 3")
      else(DEFINED SYNTHETIC_DEF_67)
        message(AUTHOR_WARNING "Neither condition for 67")
      endif(DEFINED SYNTHETIC_DEF_67)
    elseif(NOT SYNTHETIC_VAR_33 STREQUAL "value_33" AND SYNTHETIC_FALLBACK_33)
      message(WARNING "Fallback 33 depth 2")

      if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_68.txt)
        message(STATUS "Condition 68 depth 3 met")

        if(SYNTHETIC_VAR_137 STREQUAL "value_137")
          message(STATUS "Condition 137 depth 4 met")
        elseif(NOT SYNTHETIC_VAR_137 STREQUAL "value_137" AND SYNTHETIC_FALLBACK_137)
          message(WARNING "Fallback 137 depth 4")
        else(SYNTHETIC_VAR_137 STREQUAL "value_137")
          message(AUTHOR_WARNING "Neither condition for 137")
        endif(SYNTHETIC_VAR_137 STREQUAL "value_137")
      elseif(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_68.txt AND SYNTHETIC_FALLBACK_68)
        message(WARNING "Fallback 68 depth 3")
      else(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_68.txt)
        message(AUTHOR_WARNING "Neither condition for 68")
      endif(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/file_68.txt)
    else(SYNTHETIC_VAR_33 STREQUAL "value_33")
      message(AUTHOR_WARNING "Neither condition for 33")
    endif(SYNTHETIC_VAR_33 STREQUAL "value_33")
  elseif(NOT SYNTHETIC_COND_16_A AND SYNTHETIC_FALLBACK_16)
    message(WARNING "Fallback 16 depth 1")

    if(SYNTHETIC_NUM_34 GREATER 100)
      message(STATUS "Condition 34 depth 2 met")

      if(SYNTHETIC_LIST_69 MATCHES "^pattern")
        message(STATUS "Condition 69 depth 3 met")

        if(DEFINED SYNTHETIC_DEF_139)
          message(STATUS "Condition 139 depth 4 met")
        elseif(NOT DEFINED SYNTHETIC_DEF_139 AND SYNTHETIC_FALLBACK_139)
          message(WARNING "Fallback 139 depth 4")
        else(DEFINED SYNTHETIC_DEF_139)
          message(AUTHOR_WARNING "Neither condition for 139")
        endif(DEFINED SYNTHETIC_DEF_139)
      elseif(NOT SYNTHETIC_LIST_69 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_69)
        message(WARNING "Fallback 69 depth 3")
      else(SYNTHETIC_LIST_69 MATCHES "^pattern")
        message(AUTHOR_WARNING "Neither condition for 69")
      endif(SYNTHETIC_LIST_69 MATCHES "^pattern")
    elseif(NOT SYNTHETIC_NUM_34 GREATER 100 AND SYNTHETIC_FALLBACK_34)
      message(WARNING "Fallback 34 depth 2")

      if(TARGET synthetic_target_70)
        message(STATUS "Condition 70 depth 3 met")

        if(SYNTHETIC_LIST_141 MATCHES "^pattern")
          message(STATUS "Condition 141 depth 4 met")
        elseif(NOT SYNTHETIC_LIST_141 MATCHES "^pattern" AND SYNTHETIC_FALLBACK_141)
          message(WARNING "Fallback 141 depth 4")
        else(SYNTHETIC_LIST_141 MATCHES "^pattern")
          message(AUTHOR_WARNING "Neither condition for 141")
        endif(SYNTHETIC_LIST_141 MATCHES "^pattern")
      elseif(NOT TARGET synthetic_target_70 AND SYNTHETIC_FALLBACK_70)
        message(WARNING "Fallback 70 depth 3")
      else(TARGET synthetic_target_70)
        message(AUTHOR_WARNING "Neither condition for 70")
      endif(TARGET synthetic_target_70)
    else(SYNTHETIC_NUM_34 GREATER 100)
      message(AUTHOR_WARNING "Neither condition for 34")
    endif(SYNTHETIC_NUM_34 GREATER 100)
  else(SYNTHETIC_COND_16_A)
    message(AUTHOR_WARNING "Neither condition for 16")
  endif(SYNTHETIC_COND_16_A)
else(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)
  message(AUTHOR_WARNING "Neither condition for 7")
endif(SYNTHETIC_A_7 VERSION_GREATER_EQUAL 1.2.3)

foreach(item IN LISTS SYNTHETIC_SOURCES)
  message(STATUS "Processing: ${item}")
endforeach(item IN LISTS SYNTHETIC_SOURCES)

foreach(idx RANGE 0 99)
  math(EXPR remainder "${idx} % 10")

  if(remainder EQUAL 0)
    message(STATUS "Milestone: ${idx}")
  endif(remainder EQUAL 0)
endforeach(idx RANGE 0 99)

foreach(item IN ITEMS alpha bravo charlie delta echo foxtrot)
  message(STATUS "Item: ${item}")
endforeach(item IN ITEMS alpha bravo charlie delta echo foxtrot)

foreach(key value IN ZIP_LISTS SYNTHETIC_KEYS SYNTHETIC_VALUES)
  set(RESULT_${key} "${value}")
endforeach(key value IN ZIP_LISTS SYNTHETIC_KEYS SYNTHETIC_VALUES)

set(SYNTHETIC_COUNTER 0)

while(SYNTHETIC_COUNTER LESS 100)
  math(EXPR SYNTHETIC_COUNTER "${SYNTHETIC_COUNTER} + 1")

  if(SYNTHETIC_COUNTER GREATER 50)
    break()
  endif(SYNTHETIC_COUNTER GREATER 50)

  if(SYNTHETIC_COUNTER LESS 10)
    continue()
  endif(SYNTHETIC_COUNTER LESS 10)
endwhile(SYNTHETIC_COUNTER LESS 100)

function(synthetic_function_0 target_name)
  set(local_var_0 ${ARGN})
  message(STATUS "Function 0: ${target_name}")

  if(DEFINED local_var_0)
    message(STATUS "  args: ${local_var_0}")
  endif(DEFINED local_var_0)
  set(${target_name}_PROCESSED TRUE PARENT_SCOPE)
endfunction(synthetic_function_0 target_name)

function(synthetic_function_1 target_name)
  set(local_var_1 ${ARGN})
  message(STATUS "Function 1: ${target_name}")

  if(DEFINED local_var_1)
    message(STATUS "  args: ${local_var_1}")
  endif(DEFINED local_var_1)
  set(${target_name}_PROCESSED TRUE PARENT_SCOPE)
endfunction(synthetic_function_1 target_name)

function(synthetic_function_2 target_name)
  set(local_var_2 ${ARGN})
  message(STATUS "Function 2: ${target_name}")

  if(DEFINED local_var_2)
    message(STATUS "  args: ${local_var_2}")
  endif(DEFINED local_var_2)
  set(${target_name}_PROCESSED TRUE PARENT_SCOPE)
endfunction(synthetic_function_2 target_name)

function(synthetic_function_3 target_name)
  set(local_var_3 ${ARGN})
  message(STATUS "Function 3: ${target_name}")

  if(DEFINED local_var_3)
    message(STATUS "  args: ${local_var_3}")
  endif(DEFINED local_var_3)
  set(${target_name}_PROCESSED TRUE PARENT_SCOPE)
endfunction(synthetic_function_3 target_name)

function(synthetic_function_4 target_name)
  set(local_var_4 ${ARGN})
  message(STATUS "Function 4: ${target_name}")

  if(DEFINED local_var_4)
    message(STATUS "  args: ${local_var_4}")
  endif(DEFINED local_var_4)
  set(${target_name}_PROCESSED TRUE PARENT_SCOPE)
endfunction(synthetic_function_4 target_name)

macro(synthetic_macro_0 first_arg)
  set(SYNTHETIC_MACRO_RESULT_0 "${first_arg}_processed")

  if(ARGC GREATER 1)
    list(APPEND SYNTHETIC_MACRO_RESULT_0 ${ARGN})
  endif(ARGC GREATER 1)
endmacro(synthetic_macro_0 first_arg)

macro(synthetic_macro_1 first_arg)
  set(SYNTHETIC_MACRO_RESULT_1 "${first_arg}_processed")

  if(ARGC GREATER 1)
    list(APPEND SYNTHETIC_MACRO_RESULT_1 ${ARGN})
  endif(ARGC GREATER 1)
endmacro(synthetic_macro_1 first_arg)

macro(synthetic_macro_2 first_arg)
  set(SYNTHETIC_MACRO_RESULT_2 "${first_arg}_processed")

  if(ARGC GREATER 1)
    list(APPEND SYNTHETIC_MACRO_RESULT_2 ${ARGN})
  endif(ARGC GREATER 1)
endmacro(synthetic_macro_2 first_arg)

macro(synthetic_macro_3 first_arg)
  set(SYNTHETIC_MACRO_RESULT_3 "${first_arg}_processed")

  if(ARGC GREATER 1)
    list(APPEND SYNTHETIC_MACRO_RESULT_3 ${ARGN})
  endif(ARGC GREATER 1)
endmacro(synthetic_macro_3 first_arg)

macro(synthetic_macro_4 first_arg)
  set(SYNTHETIC_MACRO_RESULT_4 "${first_arg}_processed")

  if(ARGC GREATER 1)
    list(APPEND SYNTHETIC_MACRO_RESULT_4 ${ARGN})
  endif(ARGC GREATER 1)
endmacro(synthetic_macro_4 first_arg)

block(SCOPE_FOR VARIABLES POLICIES)
  set(BLOCK_LOCAL_VAR "only visible in block")
  cmake_policy(SET CMP0077 NEW)
  message(STATUS ${BLOCK_LOCAL_VAR})
endblock()

# --------------------------------------------------------------------------
# Library targets
# --------------------------------------------------------------------------

add_library(synthetic_core
  STATIC
  include/core/file_0000.h
  include/core/file_0001.h
  include/core/file_0002.h
  include/core/file_0003.h
  include/core/file_0004.h
  include/core/file_0005.h
  include/core/file_0006.h
  include/core/file_0007.h
  include/core/file_0008.h
  include/core/file_0009.h
  include/core/file_0010.h
  include/core/file_0011.h
  include/core/file_0012.h
  include/core/file_0013.h
  include/core/file_0014.h
  include/core/file_0015.h
  include/core/file_0016.h
  include/core/file_0017.h
  include/core/file_0018.h
  include/core/file_0019.h
  include/core/file_0020.h
  include/core/file_0021.h
  include/core/file_0022.h
  include/core/file_0023.h
  include/core/file_0024.h
  include/core/file_0025.h
  include/core/file_0026.h
  include/core/file_0027.h
  include/core/file_0028.h
  include/core/file_0029.h
  src/core/file_0000.c
  src/core/file_0000.cpp
  src/core/file_0001.c
  src/core/file_0001.cpp
  src/core/file_0002.c
  src/core/file_0002.cpp
  src/core/file_0003.c
  src/core/file_0003.cpp
  src/core/file_0004.c
  src/core/file_0004.cpp
  src/core/file_0005.c
  src/core/file_0005.cpp
  src/core/file_0006.c
  src/core/file_0006.cpp
  src/core/file_0007.c
  src/core/file_0007.cpp
  src/core/file_0008.c
  src/core/file_0008.cpp
  src/core/file_0009.c
  src/core/file_0009.cpp
  src/core/file_0010.c
  src/core/file_0010.cpp
  src/core/file_0011.c
  src/core/file_0011.cpp
  src/core/file_0012.c
  src/core/file_0012.cpp
  src/core/file_0013.c
  src/core/file_0013.cpp
  src/core/file_0014.c
  src/core/file_0014.cpp
  src/core/file_0015.c
  src/core/file_0015.cpp
  src/core/file_0016.c
  src/core/file_0016.cpp
  src/core/file_0017.c
  src/core/file_0017.cpp
  src/core/file_0018.c
  src/core/file_0018.cpp
  src/core/file_0019.c
  src/core/file_0019.cpp
  src/core/file_0020.cpp
  src/core/file_0021.cpp
  src/core/file_0022.cpp
  src/core/file_0023.cpp
  src/core/file_0024.cpp
  src/core/file_0025.cpp
  src/core/file_0026.cpp
  src/core/file_0027.cpp
  src/core/file_0028.cpp
  src/core/file_0029.cpp
  src/core/file_0030.cpp
  src/core/file_0031.cpp
  src/core/file_0032.cpp
  src/core/file_0033.cpp
  src/core/file_0034.cpp
  src/core/file_0035.cpp
  src/core/file_0036.cpp
  src/core/file_0037.cpp
  src/core/file_0038.cpp
  src/core/file_0039.cpp
)

add_library(synthetic_shared
  SHARED
  src/shared/file_0003.cpp
  src/shared/file_0003.cpp
  src/shared/file_0003.cpp
  src/shared/file_0003.cpp
  src/shared/file_0003.cpp
  src/shared/file_0003.cpp
  src/shared/file_0003.cpp
  src/shared/file_0010.cpp
  src/shared/file_0010.cpp
  src/shared/file_0010.cpp
  src/shared/file_0010.cpp
  src/shared/file_0010.cpp
  src/shared/file_0010.cpp
  src/shared/file_0010.cpp
  src/shared/file_0017.cpp
  src/shared/file_0017.cpp
  src/shared/file_0017.cpp
  src/shared/file_0017.cpp
  src/shared/file_0017.cpp
  src/shared/file_0017.cpp
  src/shared/file_0017.cpp
  src/shared/file_0024.cpp
  src/shared/file_0024.cpp
  src/shared/file_0024.cpp
  src/shared/file_0024.cpp
  src/shared/file_0024.cpp
  src/shared/file_0024.cpp
  src/shared/file_0024.cpp
  src/shared/file_0031.cpp
  src/shared/file_0031.cpp
  src/shared/file_0031.cpp
  src/shared/file_0031.cpp
  src/shared/file_0031.cpp
  src/shared/file_0031.cpp
  src/shared/file_0031.cpp
)

add_library(synthetic_objects
  OBJECT
  src/objects/file_0000.cpp
  src/objects/file_0001.cpp
  src/objects/file_0002.cpp
  src/objects/file_0003.cpp
  src/objects/file_0004.cpp
  src/objects/file_0005.cpp
  src/objects/file_0006.cpp
  src/objects/file_0007.cpp
  src/objects/file_0008.cpp
  src/objects/file_0009.cpp
  src/objects/file_0010.cpp
  src/objects/file_0011.cpp
  src/objects/file_0012.cpp
  src/objects/file_0013.cpp
  src/objects/file_0014.cpp
  src/objects/file_0015.cpp
  src/objects/file_0016.cpp
  src/objects/file_0017.cpp
  src/objects/file_0018.cpp
  src/objects/file_0019.cpp
  src/objects/file_0020.cpp
  src/objects/file_0021.cpp
  src/objects/file_0022.cpp
  src/objects/file_0023.cpp
  src/objects/file_0024.cpp
)

add_library(synthetic_interface INTERFACE)

add_library(synthetic_imported SHARED IMPORTED GLOBAL)

add_library(Synthetic::Core ALIAS synthetic_core)
add_library(Synthetic::Shared ALIAS synthetic_shared)

add_library(synthetic_genex_sources
  $<$<PLATFORM_ID:Darwin>:src/platform/macos.cpp>
  $<$<PLATFORM_ID:Linux>:src/platform/linux.cpp>
  $<$<PLATFORM_ID:Windows>:src/platform/windows.cpp>
  src/platform/common.cpp
)

# --------------------------------------------------------------------------
# Executable targets
# --------------------------------------------------------------------------

add_executable(synthetic_app_0
  src/app0/file_0000.cpp
  src/app0/file_0001.cpp
  src/app0/file_0002.cpp
  src/app0/file_0003.cpp
  src/app0/file_0004.cpp
  src/app0/file_0005.cpp
  src/app0/file_0006.cpp
  src/app0/file_0007.cpp
  src/app0/file_0008.cpp
  src/app0/file_0009.cpp
  src/app0/file_0010.cpp
  src/app0/file_0011.cpp
  src/app0/file_0012.cpp
  src/app0/file_0013.cpp
  src/app0/file_0014.cpp
  src/app0/file_0015.cpp
  src/app0/file_0016.cpp
  src/app0/file_0017.cpp
  src/app0/file_0018.cpp
  src/app0/file_0019.cpp
)

add_executable(synthetic_app_1
  src/app1/file_0000.cpp
  src/app1/file_0001.cpp
  src/app1/file_0002.cpp
  src/app1/file_0003.cpp
  src/app1/file_0004.cpp
  src/app1/file_0005.cpp
  src/app1/file_0006.cpp
  src/app1/file_0007.cpp
  src/app1/file_0008.cpp
  src/app1/file_0009.cpp
  src/app1/file_0010.cpp
  src/app1/file_0011.cpp
  src/app1/file_0012.cpp
  src/app1/file_0013.cpp
  src/app1/file_0014.cpp
  src/app1/file_0015.cpp
  src/app1/file_0016.cpp
  src/app1/file_0017.cpp
  src/app1/file_0018.cpp
  src/app1/file_0019.cpp
  src/app1/file_0020.cpp
  src/app1/file_0021.cpp
  src/app1/file_0022.cpp
)

add_executable(synthetic_app_2
  src/app2/file_0000.cpp
  src/app2/file_0001.cpp
  src/app2/file_0002.cpp
  src/app2/file_0003.cpp
  src/app2/file_0004.cpp
  src/app2/file_0005.cpp
  src/app2/file_0006.cpp
  src/app2/file_0007.cpp
  src/app2/file_0008.cpp
  src/app2/file_0009.cpp
  src/app2/file_0010.cpp
  src/app2/file_0011.cpp
  src/app2/file_0012.cpp
  src/app2/file_0013.cpp
  src/app2/file_0014.cpp
  src/app2/file_0015.cpp
  src/app2/file_0016.cpp
  src/app2/file_0017.cpp
  src/app2/file_0018.cpp
  src/app2/file_0019.cpp
  src/app2/file_0020.cpp
  src/app2/file_0021.cpp
  src/app2/file_0022.cpp
  src/app2/file_0023.cpp
  src/app2/file_0024.cpp
  src/app2/file_0025.cpp
)

add_executable(synthetic_app_3
  src/app3/file_0000.cpp
  src/app3/file_0001.cpp
  src/app3/file_0002.cpp
  src/app3/file_0003.cpp
  src/app3/file_0004.cpp
  src/app3/file_0005.cpp
  src/app3/file_0006.cpp
  src/app3/file_0007.cpp
  src/app3/file_0008.cpp
  src/app3/file_0009.cpp
  src/app3/file_0010.cpp
  src/app3/file_0011.cpp
  src/app3/file_0012.cpp
  src/app3/file_0013.cpp
  src/app3/file_0014.cpp
  src/app3/file_0015.cpp
  src/app3/file_0016.cpp
  src/app3/file_0017.cpp
  src/app3/file_0018.cpp
  src/app3/file_0019.cpp
  src/app3/file_0020.cpp
  src/app3/file_0021.cpp
  src/app3/file_0022.cpp
  src/app3/file_0023.cpp
  src/app3/file_0024.cpp
  src/app3/file_0025.cpp
  src/app3/file_0026.cpp
  src/app3/file_0027.cpp
  src/app3/file_0028.cpp
)

add_executable(synthetic_app_4
  src/app4/file_0000.cpp
  src/app4/file_0001.cpp
  src/app4/file_0002.cpp
  src/app4/file_0003.cpp
  src/app4/file_0004.cpp
  src/app4/file_0005.cpp
  src/app4/file_0006.cpp
  src/app4/file_0007.cpp
  src/app4/file_0008.cpp
  src/app4/file_0009.cpp
  src/app4/file_0010.cpp
  src/app4/file_0011.cpp
  src/app4/file_0012.cpp
  src/app4/file_0013.cpp
  src/app4/file_0014.cpp
  src/app4/file_0015.cpp
  src/app4/file_0016.cpp
  src/app4/file_0017.cpp
  src/app4/file_0018.cpp
  src/app4/file_0019.cpp
  src/app4/file_0020.cpp
  src/app4/file_0021.cpp
  src/app4/file_0022.cpp
  src/app4/file_0023.cpp
  src/app4/file_0024.cpp
  src/app4/file_0025.cpp
  src/app4/file_0026.cpp
  src/app4/file_0027.cpp
  src/app4/file_0028.cpp
  src/app4/file_0029.cpp
  src/app4/file_0030.cpp
  src/app4/file_0031.cpp
)

add_executable(synthetic_app_5
  src/app5/file_0003.cpp
  src/app5/file_0003.cpp
  src/app5/file_0003.cpp
  src/app5/file_0003.cpp
  src/app5/file_0003.cpp
  src/app5/file_0003.cpp
  src/app5/file_0003.cpp
  src/app5/file_0010.cpp
  src/app5/file_0010.cpp
  src/app5/file_0010.cpp
  src/app5/file_0010.cpp
  src/app5/file_0010.cpp
  src/app5/file_0010.cpp
  src/app5/file_0010.cpp
  src/app5/file_0017.cpp
  src/app5/file_0017.cpp
  src/app5/file_0017.cpp
  src/app5/file_0017.cpp
  src/app5/file_0017.cpp
  src/app5/file_0017.cpp
  src/app5/file_0017.cpp
  src/app5/file_0024.cpp
  src/app5/file_0024.cpp
  src/app5/file_0024.cpp
  src/app5/file_0024.cpp
  src/app5/file_0024.cpp
  src/app5/file_0024.cpp
  src/app5/file_0024.cpp
  src/app5/file_0031.cpp
  src/app5/file_0031.cpp
  src/app5/file_0031.cpp
  src/app5/file_0031.cpp
  src/app5/file_0031.cpp
  src/app5/file_0031.cpp
  src/app5/file_0031.cpp
)

add_executable(synthetic_app_6
  src/app6/file_0000.cpp
  src/app6/file_0001.cpp
  src/app6/file_0002.cpp
  src/app6/file_0003.cpp
  src/app6/file_0004.cpp
  src/app6/file_0005.cpp
  src/app6/file_0006.cpp
  src/app6/file_0007.cpp
  src/app6/file_0008.cpp
  src/app6/file_0009.cpp
  src/app6/file_0010.cpp
  src/app6/file_0011.cpp
  src/app6/file_0012.cpp
  src/app6/file_0013.cpp
  src/app6/file_0014.cpp
  src/app6/file_0015.cpp
  src/app6/file_0016.cpp
  src/app6/file_0017.cpp
  src/app6/file_0018.cpp
  src/app6/file_0019.cpp
  src/app6/file_0020.cpp
  src/app6/file_0021.cpp
  src/app6/file_0022.cpp
  src/app6/file_0023.cpp
  src/app6/file_0024.cpp
  src/app6/file_0025.cpp
  src/app6/file_0026.cpp
  src/app6/file_0027.cpp
  src/app6/file_0028.cpp
  src/app6/file_0029.cpp
  src/app6/file_0030.cpp
  src/app6/file_0031.cpp
  src/app6/file_0032.cpp
  src/app6/file_0033.cpp
  src/app6/file_0034.cpp
  src/app6/file_0035.cpp
  src/app6/file_0036.cpp
  src/app6/file_0037.cpp
)

add_executable(synthetic_app_7
  src/app7/file_0000.cpp
  src/app7/file_0001.cpp
  src/app7/file_0002.cpp
  src/app7/file_0003.cpp
  src/app7/file_0004.cpp
  src/app7/file_0005.cpp
  src/app7/file_0006.cpp
  src/app7/file_0007.cpp
  src/app7/file_0008.cpp
  src/app7/file_0009.cpp
  src/app7/file_0010.cpp
  src/app7/file_0011.cpp
  src/app7/file_0012.cpp
  src/app7/file_0013.cpp
  src/app7/file_0014.cpp
  src/app7/file_0015.cpp
  src/app7/file_0016.cpp
  src/app7/file_0017.cpp
  src/app7/file_0018.cpp
  src/app7/file_0019.cpp
  src/app7/file_0020.cpp
  src/app7/file_0021.cpp
  src/app7/file_0022.cpp
  src/app7/file_0023.cpp
  src/app7/file_0024.cpp
  src/app7/file_0025.cpp
  src/app7/file_0026.cpp
  src/app7/file_0027.cpp
  src/app7/file_0028.cpp
  src/app7/file_0029.cpp
  src/app7/file_0030.cpp
  src/app7/file_0031.cpp
  src/app7/file_0032.cpp
  src/app7/file_0033.cpp
  src/app7/file_0034.cpp
  src/app7/file_0035.cpp
  src/app7/file_0036.cpp
  src/app7/file_0037.cpp
  src/app7/file_0038.cpp
  src/app7/file_0039.cpp
  src/app7/file_0040.cpp
)

add_executable(synthetic_win32
  WIN32
  src/main_win32.cpp
  src/resource.rc
)

add_executable(synthetic_bundle
  MACOSX_BUNDLE
  resources/Info.plist
  src/main_bundle.cpp
)

add_executable(synthetic_tool_imported IMPORTED)

# --------------------------------------------------------------------------
# Target properties
# --------------------------------------------------------------------------

set_target_properties(synthetic_core PROPERTIES
  CXX_STANDARD              20
  CXX_STANDARD_REQUIRED     ON
  CXX_EXTENSIONS            OFF
  POSITION_INDEPENDENT_CODE ON
  OUTPUT_NAME               "synthetic"
  VERSION                   "${PROJECT_VERSION}"
  SOVERSION                 "${PROJECT_VERSION_MAJOR}"
  EXPORT_NAME               "Synthetic"
  LINKER_LANGUAGE           CXX
  RUNTIME_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/lib"
  ARCHIVE_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(synthetic_shared PROPERTIES
  CXX_STANDARD              20
  CXX_STANDARD_REQUIRED     ON
  CXX_EXTENSIONS            OFF
  POSITION_INDEPENDENT_CODE ON
  OUTPUT_NAME               "synthetic"
  VERSION                   "${PROJECT_VERSION}"
  SOVERSION                 "${PROJECT_VERSION_MAJOR}"
  EXPORT_NAME               "Synthetic"
  LINKER_LANGUAGE           CXX
  RUNTIME_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/lib"
  ARCHIVE_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(synthetic_objects PROPERTIES
  CXX_STANDARD              20
  CXX_STANDARD_REQUIRED     ON
  CXX_EXTENSIONS            OFF
  POSITION_INDEPENDENT_CODE ON
  OUTPUT_NAME               "synthetic"
  VERSION                   "${PROJECT_VERSION}"
  SOVERSION                 "${PROJECT_VERSION_MAJOR}"
  EXPORT_NAME               "Synthetic"
  LINKER_LANGUAGE           CXX
  RUNTIME_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/lib"
  ARCHIVE_OUTPUT_DIRECTORY  "${CMAKE_BINARY_DIR}/lib"
)

target_compile_definitions(synthetic_core
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_DEBUG=1>
    $<$<CONFIG:Release>:SYNTHETIC_NDEBUG=1>
    SYNTHETIC_VERSION_MAJOR=${PROJECT_VERSION_MAJOR}
    SYNTHETIC_VERSION_MINOR=${PROJECT_VERSION_MINOR}

  PRIVATE
    $<$<BOOL:${SYNTHETIC_OPTION_00}>:SYNTHETIC_FEATURE_A>
    SYNTHETIC_INTERNAL=1

  INTERFACE
    SYNTHETIC_API=1
)

target_compile_definitions(synthetic_shared
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_DEBUG=1>
    $<$<CONFIG:Release>:SYNTHETIC_NDEBUG=1>
    SYNTHETIC_VERSION_MAJOR=${PROJECT_VERSION_MAJOR}
    SYNTHETIC_VERSION_MINOR=${PROJECT_VERSION_MINOR}

  PRIVATE
    $<$<BOOL:${SYNTHETIC_OPTION_00}>:SYNTHETIC_FEATURE_A>
    SYNTHETIC_INTERNAL=1

  INTERFACE
    SYNTHETIC_API=1
)

target_compile_definitions(synthetic_objects
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_DEBUG=1>
    $<$<CONFIG:Release>:SYNTHETIC_NDEBUG=1>
    SYNTHETIC_VERSION_MAJOR=${PROJECT_VERSION_MAJOR}
    SYNTHETIC_VERSION_MINOR=${PROJECT_VERSION_MINOR}

  PRIVATE
    $<$<BOOL:${SYNTHETIC_OPTION_00}>:SYNTHETIC_FEATURE_A>
    SYNTHETIC_INTERNAL=1

  INTERFACE
    SYNTHETIC_API=1
)

target_compile_options(synthetic_core
  PUBLIC
    $<$<CXX_COMPILER_ID:Clang>:-Wall -Wextra -Wpedantic -Wno-unused-parameter>
    $<$<CXX_COMPILER_ID:GNU>:-Wall -Wextra -Wpedantic>
    $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>

  PRIVATE
    $<$<CONFIG:Debug>:-O0 -g>
    $<$<CONFIG:Release>:-O3 -DNDEBUG>
    $<$<CONFIG:RelWithDebInfo>:-O2 -g>
)

target_compile_options(synthetic_shared
  PUBLIC
    $<$<CXX_COMPILER_ID:Clang>:-Wall -Wextra -Wpedantic -Wno-unused-parameter>
    $<$<CXX_COMPILER_ID:GNU>:-Wall -Wextra -Wpedantic>
    $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>

  PRIVATE
    $<$<CONFIG:Debug>:-O0 -g>
    $<$<CONFIG:Release>:-O3 -DNDEBUG>
    $<$<CONFIG:RelWithDebInfo>:-O2 -g>
)

target_compile_options(synthetic_objects
  PUBLIC
    $<$<CXX_COMPILER_ID:Clang>:-Wall -Wextra -Wpedantic -Wno-unused-parameter>
    $<$<CXX_COMPILER_ID:GNU>:-Wall -Wextra -Wpedantic>
    $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>

  PRIVATE
    $<$<CONFIG:Debug>:-O0 -g>
    $<$<CONFIG:Release>:-O3 -DNDEBUG>
    $<$<CONFIG:RelWithDebInfo>:-O2 -g>
)

target_compile_features(synthetic_core PUBLIC cxx_std_20)
target_compile_features(synthetic_shared PUBLIC cxx_std_17 PRIVATE cxx_std_20)

target_include_directories(synthetic_core
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/src

  SYSTEM

  INTERFACE
    ${Boost_INCLUDE_DIRS}
)

target_include_directories(synthetic_shared
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/src

  SYSTEM

  INTERFACE
    ${Boost_INCLUDE_DIRS}
)

target_include_directories(synthetic_objects
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/src

  SYSTEM

  INTERFACE
    ${Boost_INCLUDE_DIRS}
)

target_link_libraries(synthetic_core
  PUBLIC
    ${OpenSSL_LIBRARIES}
    Boost::filesystem
    Boost::system

  INTERFACE
    synthetic_interface

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    $<$<PLATFORM_ID:Linux>:rt>
    Threads::Threads
    ZLIB::ZLIB
)

target_link_libraries(synthetic_shared
  PUBLIC
    ${OpenSSL_LIBRARIES}
    Boost::filesystem
    Boost::system

  INTERFACE
    synthetic_interface

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    $<$<PLATFORM_ID:Linux>:rt>
    Threads::Threads
    ZLIB::ZLIB
)

target_link_libraries(synthetic_objects
  PUBLIC
    ${OpenSSL_LIBRARIES}
    Boost::filesystem
    Boost::system

  INTERFACE
    synthetic_interface

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    $<$<PLATFORM_ID:Linux>:rt>
    Threads::Threads
    ZLIB::ZLIB
)

target_link_options(synthetic_shared
  PRIVATE
    $<$<CXX_COMPILER_ID:GNU>:-Wl,--as-needed>
    $<$<PLATFORM_ID:Linux>:-Wl,-rpath,$ORIGIN/../lib>
)

target_link_directories(synthetic_core
  PRIVATE
    "${CMAKE_BINARY_DIR}/deps/lib"
    "${CMAKE_PREFIX_PATH}/lib"
)

target_precompile_headers(synthetic_core
  PUBLIC
    <vector>
    <string>
    <memory>
    <unordered_map>

  PRIVATE
    <iostream>
    <fstream>
    "src/internal.h"
)

target_sources(synthetic_core
  PUBLIC
    FILE_SET HEADERS
      BASE_DIRS include
      FILES
        include/synthetic/core.h
        include/synthetic/types.h
        include/synthetic/utils.h

  PRIVATE
    src/internal_impl.cpp
)

# --------------------------------------------------------------------------
# Install rules
# --------------------------------------------------------------------------

install(
  TARGETS synthetic_core synthetic_shared
  EXPORT  SyntheticTargets
  RUNTIME
    DESTINATION ${CMAKE_INSTALL_BINDIR}
    COMPONENT

  RUNTIME

  LIBRARY
    DESTINATION ${CMAKE_INSTALL_LIBDIR}
    COMPONENT

  RUNTIME
    NAMELINK_COMPONENT Development

  ARCHIVE
    DESTINATION ${CMAKE_INSTALL_LIBDIR}
    COMPONENT Development
  INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}

  PUBLIC_HEADER
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/synthetic
    COMPONENT Development
)

install(
  FILES
    "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/core.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/types.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/utils.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/include/synthetic/config.h"
  DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}/synthetic"
  COMPONENT   Development
)

install(
  DIRECTORY   "${CMAKE_CURRENT_SOURCE_DIR}/include/"
  DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
  COMPONENT   Development
  FILES_MATCHING
  PATTERN
    "*.h"

  PATTERN
    ".svn"
    EXCLUDE

  PATTERN
    "internal"
    EXCLUDE
)

install(
  EXPORT      SyntheticTargets
  FILE        SyntheticTargets.cmake
  NAMESPACE   Synthetic::
  DESTINATION "${CMAKE_INSTALL_LIBDIR}/cmake/Synthetic"
  COMPONENT   Development
)

configure_file("${CMAKE_CURRENT_SOURCE_DIR}/cmake/SyntheticConfig.cmake.in"
  "${CMAKE_CURRENT_BINARY_DIR}/SyntheticConfig.cmake"
  @ONLY
)

install(
  FILES
    "${CMAKE_CURRENT_BINARY_DIR}/SyntheticConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/SyntheticConfigVersion.cmake"
  DESTINATION "${CMAKE_INSTALL_LIBDIR}/cmake/Synthetic"
  COMPONENT   Development
)

# --------------------------------------------------------------------------
# Custom commands and targets
# --------------------------------------------------------------------------

add_custom_command(
  OUTPUT
    "${CMAKE_BINARY_DIR}/generated/synthetic_00.cpp"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Generating synthetic_00.cpp"

  COMMAND
    ${Python3_EXECUTABLE} "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    --output "${CMAKE_BINARY_DIR}/generated/synthetic_00.cpp" --index 0
    --config ${SYNTHETIC_CONFIG}

  DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    ${SYNTHETIC_CONFIG}

  WORKING_DIRECTORY
    "${CMAKE_CURRENT_SOURCE_DIR}"

  COMMENT
    "Generating synthetic_00.cpp"

  VERBATIM
)

add_custom_command(
  OUTPUT
    "${CMAKE_BINARY_DIR}/generated/synthetic_01.cpp"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Generating synthetic_01.cpp"

  COMMAND
    ${Python3_EXECUTABLE} "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    --output "${CMAKE_BINARY_DIR}/generated/synthetic_01.cpp" --index 1
    --config ${SYNTHETIC_CONFIG}

  DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    ${SYNTHETIC_CONFIG}

  WORKING_DIRECTORY
    "${CMAKE_CURRENT_SOURCE_DIR}"

  COMMENT
    "Generating synthetic_01.cpp"

  VERBATIM
)

add_custom_command(
  OUTPUT
    "${CMAKE_BINARY_DIR}/generated/synthetic_02.cpp"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Generating synthetic_02.cpp"

  COMMAND
    ${Python3_EXECUTABLE} "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    --output "${CMAKE_BINARY_DIR}/generated/synthetic_02.cpp" --index 2
    --config ${SYNTHETIC_CONFIG}

  DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    ${SYNTHETIC_CONFIG}

  WORKING_DIRECTORY
    "${CMAKE_CURRENT_SOURCE_DIR}"

  COMMENT
    "Generating synthetic_02.cpp"

  VERBATIM
)

add_custom_command(
  OUTPUT
    "${CMAKE_BINARY_DIR}/generated/synthetic_03.cpp"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Generating synthetic_03.cpp"

  COMMAND
    ${Python3_EXECUTABLE} "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    --output "${CMAKE_BINARY_DIR}/generated/synthetic_03.cpp" --index 3
    --config ${SYNTHETIC_CONFIG}

  DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    ${SYNTHETIC_CONFIG}

  WORKING_DIRECTORY
    "${CMAKE_CURRENT_SOURCE_DIR}"

  COMMENT
    "Generating synthetic_03.cpp"

  VERBATIM
)

add_custom_command(
  OUTPUT
    "${CMAKE_BINARY_DIR}/generated/synthetic_04.cpp"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Generating synthetic_04.cpp"

  COMMAND
    ${Python3_EXECUTABLE} "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    --output "${CMAKE_BINARY_DIR}/generated/synthetic_04.cpp" --index 4
    --config ${SYNTHETIC_CONFIG}

  DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    ${SYNTHETIC_CONFIG}

  WORKING_DIRECTORY
    "${CMAKE_CURRENT_SOURCE_DIR}"

  COMMENT
    "Generating synthetic_04.cpp"

  VERBATIM
)

add_custom_command(
  OUTPUT
    "${CMAKE_BINARY_DIR}/generated/synthetic_05.cpp"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Generating synthetic_05.cpp"

  COMMAND
    ${Python3_EXECUTABLE} "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    --output "${CMAKE_BINARY_DIR}/generated/synthetic_05.cpp" --index 5
    --config ${SYNTHETIC_CONFIG}

  DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/scripts/generate.py"
    ${SYNTHETIC_CONFIG}

  WORKING_DIRECTORY
    "${CMAKE_CURRENT_SOURCE_DIR}"

  COMMENT
    "Generating synthetic_05.cpp"

  VERBATIM
)

add_custom_command(
  TARGET
    synthetic_core

  POST_BUILD

  COMMAND
    ${CMAKE_COMMAND} -E copy "$<TARGET_FILE:synthetic_core>"
    "${CMAKE_BINARY_DIR}/output/"

  COMMENT
    "Copying synthetic_core to output"

  VERBATIM
)

add_custom_target(synthetic_generate_0
  DEPENDS
    "${CMAKE_BINARY_DIR}/generated/synthetic_00.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_01.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_02.cpp"

  COMMENT
    "Generate batch 0"
)

add_custom_target(synthetic_generate_1
  DEPENDS
    "${CMAKE_BINARY_DIR}/generated/synthetic_03.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_04.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_05.cpp"

  COMMENT
    "Generate batch 1"
)

add_custom_target(synthetic_generate_2
  DEPENDS
    "${CMAKE_BINARY_DIR}/generated/synthetic_06.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_07.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_08.cpp"

  COMMENT
    "Generate batch 2"
)

add_custom_target(synthetic_generate_3
  DEPENDS
    "${CMAKE_BINARY_DIR}/generated/synthetic_09.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_10.cpp"
    "${CMAKE_BINARY_DIR}/generated/synthetic_11.cpp"

  COMMENT
    "Generate batch 3"
)

add_custom_target(synthetic_all_generated
  ALL

  DEPENDS
    synthetic_generate_0 synthetic_generate_1 synthetic_generate_2
)

# --------------------------------------------------------------------------
# Testing
# --------------------------------------------------------------------------

enable_testing()

include(GoogleTest)
include(CTest)

add_executable(synthetic_test_0
  tests/test0/file_0000.cpp
  tests/test0/file_0001.cpp
  tests/test0/file_0002.cpp
  tests/test0/file_0003.cpp
  tests/test0/file_0004.cpp
  tests/test0/file_0005.cpp
  tests/test0/file_0006.cpp
  tests/test0/file_0007.cpp
)

target_link_libraries(synthetic_test_0
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_0
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_0::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_1
  tests/test1/file_0000.cpp
  tests/test1/file_0001.cpp
  tests/test1/file_0002.cpp
  tests/test1/file_0003.cpp
  tests/test1/file_0004.cpp
  tests/test1/file_0005.cpp
  tests/test1/file_0006.cpp
  tests/test1/file_0007.cpp
  tests/test1/file_0008.cpp
)

target_link_libraries(synthetic_test_1
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_1
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_1::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_2
  tests/test2/file_0000.cpp
  tests/test2/file_0001.cpp
  tests/test2/file_0002.cpp
  tests/test2/file_0003.cpp
  tests/test2/file_0004.cpp
  tests/test2/file_0005.cpp
  tests/test2/file_0006.cpp
  tests/test2/file_0007.cpp
  tests/test2/file_0008.cpp
  tests/test2/file_0009.cpp
)

target_link_libraries(synthetic_test_2
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_2
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_2::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_3
  tests/test3/file_0000.cpp
  tests/test3/file_0001.cpp
  tests/test3/file_0002.cpp
  tests/test3/file_0003.cpp
  tests/test3/file_0004.cpp
  tests/test3/file_0005.cpp
  tests/test3/file_0006.cpp
  tests/test3/file_0007.cpp
  tests/test3/file_0008.cpp
  tests/test3/file_0009.cpp
  tests/test3/file_0010.cpp
)

target_link_libraries(synthetic_test_3
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_3
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_3::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_4
  tests/test4/file_0000.cpp
  tests/test4/file_0001.cpp
  tests/test4/file_0002.cpp
  tests/test4/file_0003.cpp
  tests/test4/file_0004.cpp
  tests/test4/file_0005.cpp
  tests/test4/file_0006.cpp
  tests/test4/file_0007.cpp
  tests/test4/file_0008.cpp
  tests/test4/file_0009.cpp
  tests/test4/file_0010.cpp
  tests/test4/file_0011.cpp
)

target_link_libraries(synthetic_test_4
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_4
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_4::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_5
  tests/test5/file_0000.cpp
  tests/test5/file_0001.cpp
  tests/test5/file_0002.cpp
  tests/test5/file_0003.cpp
  tests/test5/file_0004.cpp
  tests/test5/file_0005.cpp
  tests/test5/file_0006.cpp
  tests/test5/file_0007.cpp
  tests/test5/file_0008.cpp
  tests/test5/file_0009.cpp
  tests/test5/file_0010.cpp
  tests/test5/file_0011.cpp
  tests/test5/file_0012.cpp
)

target_link_libraries(synthetic_test_5
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_5
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_5::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_6
  tests/test6/file_0003.cpp
  tests/test6/file_0003.cpp
  tests/test6/file_0003.cpp
  tests/test6/file_0003.cpp
  tests/test6/file_0003.cpp
  tests/test6/file_0003.cpp
  tests/test6/file_0003.cpp
  tests/test6/file_0010.cpp
  tests/test6/file_0010.cpp
  tests/test6/file_0010.cpp
  tests/test6/file_0010.cpp
  tests/test6/file_0010.cpp
  tests/test6/file_0010.cpp
  tests/test6/file_0010.cpp
)

target_link_libraries(synthetic_test_6
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_6
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_6::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_7
  tests/test7/file_0000.cpp
  tests/test7/file_0001.cpp
  tests/test7/file_0002.cpp
  tests/test7/file_0003.cpp
  tests/test7/file_0004.cpp
  tests/test7/file_0005.cpp
  tests/test7/file_0006.cpp
  tests/test7/file_0007.cpp
  tests/test7/file_0008.cpp
  tests/test7/file_0009.cpp
  tests/test7/file_0010.cpp
  tests/test7/file_0011.cpp
  tests/test7/file_0012.cpp
  tests/test7/file_0013.cpp
  tests/test7/file_0014.cpp
)

target_link_libraries(synthetic_test_7
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_7
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_7::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_8
  tests/test8/file_0000.cpp
  tests/test8/file_0001.cpp
  tests/test8/file_0002.cpp
  tests/test8/file_0003.cpp
  tests/test8/file_0004.cpp
  tests/test8/file_0005.cpp
  tests/test8/file_0006.cpp
  tests/test8/file_0007.cpp
  tests/test8/file_0008.cpp
  tests/test8/file_0009.cpp
  tests/test8/file_0010.cpp
  tests/test8/file_0011.cpp
  tests/test8/file_0012.cpp
  tests/test8/file_0013.cpp
  tests/test8/file_0014.cpp
  tests/test8/file_0015.cpp
)

target_link_libraries(synthetic_test_8
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_8
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_8::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_executable(synthetic_test_9
  tests/test9/file_0000.cpp
  tests/test9/file_0001.cpp
  tests/test9/file_0002.cpp
  tests/test9/file_0003.cpp
  tests/test9/file_0004.cpp
  tests/test9/file_0005.cpp
  tests/test9/file_0006.cpp
  tests/test9/file_0007.cpp
  tests/test9/file_0008.cpp
  tests/test9/file_0009.cpp
  tests/test9/file_0010.cpp
  tests/test9/file_0011.cpp
  tests/test9/file_0012.cpp
  tests/test9/file_0013.cpp
  tests/test9/file_0014.cpp
  tests/test9/file_0015.cpp
  tests/test9/file_0016.cpp
)

target_link_libraries(synthetic_test_9
  PRIVATE
    GTest::gmock
    GTest::gtest
    GTest::gtest_main
    synthetic_core
)

gtest_discover_tests(synthetic_test_9
  WORKING_DIRECTORY
    "${CMAKE_BINARY_DIR}"

  TEST_PREFIX
    "synthetic_9::"

  DISCOVERY_TIMEOUT
    120

  PROPERTIES
    LABELS  "synthetic;unit"
    TIMEOUT 300
)

add_test(
  NAME
    synthetic_integration_0

  COMMAND
    synthetic_app_0 --config "${CMAKE_BINARY_DIR}/test_config_0.json" --verbose
    --threads 4
)

set_tests_properties(
  synthetic_integration_0
  PROPERTIES
    TIMEOUT           600
    LABELS            "integration"
    ENVIRONMENT       "SYNTHETIC_TEST=1;SYNTHETIC_IDX=0"
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(
  NAME
    synthetic_integration_1

  COMMAND
    synthetic_app_0 --config "${CMAKE_BINARY_DIR}/test_config_1.json" --verbose
    --threads 4
)

set_tests_properties(
  synthetic_integration_1
  PROPERTIES
    TIMEOUT           600
    LABELS            "integration"
    ENVIRONMENT       "SYNTHETIC_TEST=1;SYNTHETIC_IDX=1"
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(
  NAME
    synthetic_integration_2

  COMMAND
    synthetic_app_0 --config "${CMAKE_BINARY_DIR}/test_config_2.json" --verbose
    --threads 4
)

set_tests_properties(
  synthetic_integration_2
  PROPERTIES
    TIMEOUT           600
    LABELS            "integration"
    ENVIRONMENT       "SYNTHETIC_TEST=1;SYNTHETIC_IDX=2"
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(
  NAME
    synthetic_integration_3

  COMMAND
    synthetic_app_0 --config "${CMAKE_BINARY_DIR}/test_config_3.json" --verbose
    --threads 4
)

set_tests_properties(
  synthetic_integration_3
  PROPERTIES
    TIMEOUT           600
    LABELS            "integration"
    ENVIRONMENT       "SYNTHETIC_TEST=1;SYNTHETIC_IDX=3"
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

add_test(
  NAME
    synthetic_integration_4

  COMMAND
    synthetic_app_0 --config "${CMAKE_BINARY_DIR}/test_config_4.json" --verbose
    --threads 4
)

set_tests_properties(
  synthetic_integration_4
  PROPERTIES
    TIMEOUT           600
    LABELS            "integration"
    ENVIRONMENT       "SYNTHETIC_TEST=1;SYNTHETIC_IDX=4"
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# --------------------------------------------------------------------------
# Generator expressions (complex)
# --------------------------------------------------------------------------

target_compile_definitions(synthetic_core
  PRIVATE
    $<$<AND:$<CONFIG:Debug>,$<PLATFORM_ID:Linux>>:SYNTHETIC_DEBUG_LINUX>
    $<$<AND:$<CONFIG:Debug>,$<PLATFORM_ID:Windows>>:SYNTHETIC_DEBUG_WINDOWS>
    $<$<NOT:$<BOOL:${SYNTHETIC_MINIMAL}>>:SYNTHETIC_FULL_FEATURES>
    $<$<OR:$<CONFIG:Release>,$<CONFIG:RelWithDebInfo>>:SYNTHETIC_OPTIMIZED>
    $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:SYNTHETIC_DLL_EXPORT>
)

target_link_libraries(synthetic_core
  PRIVATE
    $<$<AND:$<PLATFORM_ID:Linux>,$<CXX_COMPILER_ID:GNU>>:asan>
    $<$<AND:$<PLATFORM_ID:Linux>,$<CXX_COMPILER_ID:GNU>>:ubsan>
    $<IF:$<BOOL:${SYNTHETIC_USE_JEMALLOC}>,jemalloc,>
    $<TARGET_OBJECTS:synthetic_objects>
)

install(TARGETS
  synthetic_core
  RUNTIME DESTINATION $<IF:$<PLATFORM_ID:Windows>,bin,libexec>
  LIBRARY DESTINATION $<IF:$<PLATFORM_ID:Windows>,bin,lib>
)

add_custom_command(
  TARGET
    synthetic_core

  POST_BUILD

  COMMAND
    ${CMAKE_COMMAND} -E echo "Target file: $<TARGET_FILE:synthetic_core>"

  COMMAND
    ${CMAKE_COMMAND} -E echo "Target dir:  $<TARGET_FILE_DIR:synthetic_core>"

  COMMAND
    ${CMAKE_COMMAND} -E echo
    "Linker lang: $<TARGET_PROPERTY:synthetic_core,LINKER_LANGUAGE>"

  VERBATIM
)

set(SYNTHETIC_COMPLEX_GENEX
  $<$<AND:$<NOT:$<BOOL:${SYNTHETIC_DISABLE_FEATURE_A}>>,$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>>:FEATURE_A_DEBUG>
)

target_compile_options(synthetic_shared
  PRIVATE
    $<$<AND:$<CXX_COMPILER_ID:Clang>,$<VERSION_GREATER_EQUAL:$<CXX_COMPILER_VERSION>,15.0>>:-Wno-unused-but-set-variable>
    $<$<AND:$<CXX_COMPILER_ID:GNU>,$<VERSION_GREATER_EQUAL:$<CXX_COMPILER_VERSION>,12.0>>:-Wno-dangling-reference>
)

set(SYNTHETIC_PLATFORM_SOURCES
  $<$<PLATFORM_ID:Linux>:src/platform/linux_impl.cpp;src/platform/linux_sysctl.cpp;src/platform/linux_epoll.cpp>
  $<$<PLATFORM_ID:Windows>:src/platform/windows_impl.cpp;src/platform/windows_iocp.cpp>
  $<$<PLATFORM_ID:Darwin>:src/platform/macos_impl.cpp;src/platform/macos_kqueue.cpp>
)

# --------------------------------------------------------------------------
# Comments section
# --------------------------------------------------------------------------

# This is a very long comment that should be reflowed by the formatter because
# it exceeds the configured comment width of 80 characters and needs to be
# wrapped properly. The formatter should handle this gracefully by breaking the
# text at word boundaries while preserving the comment prefix.

# Another long comment for reflow: Lorem ipsum dolor sit amet, consectetur
# adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
# aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
# nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
# reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.

# Short comment 1 Short comment 2 Short comment 3

#[[This is a bracket comment.
It can span multiple lines.
The formatter should preserve it verbatim.
No reflow should happen here.]]

#[==[This is a bracket comment with equals.
It uses level-2 bracket syntax.
Content here is also preserved verbatim.
]==]

set(COMMENT_ALIGN_A     value_a)     # first value
set(COMMENT_ALIGN_BB    value_bb)    # second value
set(COMMENT_ALIGN_CCC   value_ccc)   # third value
set(COMMENT_ALIGN_DDDD  value_dddd)  # fourth value
set(COMMENT_ALIGN_EEEEE value_eeeee) # fifth value

# Example usage:
#
#   cmake -B build -DSYNTHETIC_OPTION_00=ON cmake --build build --config
#   Release ctest --test-dir build --output-on-failure
#
# End of example.

# This comment documents the following set() call
set(DOCUMENTED_VAR 42)

# First paragraph of a multi-paragraph comment that provides extensive
# documentation about the module below. This should be reflowed as a single
# block.
#
# Second paragraph with different content. The formatter should preserve the
# paragraph break (blank comment line) while still reflowing each paragraph
# individually to fit within the configured comment width.
#
# Third paragraph: implementation notes and caveats that developers should be
# aware of when modifying the code below. These are important to preserve.

# --------------------------------------------------------------------------
# Alignment groups
# --------------------------------------------------------------------------

set(SHORT_0                    "value_0")
set(MEDIUM_LEN_1               "value_1")
set(VERY_LONG_VARIABLE_NAME_2  "value_2")
set(X_3                        "value_3")
set(SOMEWHAT_LONG_4            "value_4")
set(SHORT_5                    "value_5")
set(MEDIUM_LEN_6               "value_6")
set(VERY_LONG_VARIABLE_NAME_7  "value_7")
set(X_8                        "value_8")
set(SOMEWHAT_LONG_9            "value_9")
set(SHORT_10                   "value_10")
set(MEDIUM_LEN_11              "value_11")
set(VERY_LONG_VARIABLE_NAME_12 "value_12")
set(X_13                       "value_13")
set(SOMEWHAT_LONG_14           "value_14")

set(GROUP2_A   "alpha")
set(GROUP2_BB  "bravo")
set(GROUP2_CCC "charlie")

set(GROUP3_X   "x-ray")
set(GROUP3_YY  "yankee")
set(GROUP3_ZZZ "zulu")

set_target_properties(synthetic_core PROPERTIES
  A        value_a
  BB       value_bb
  CCC      value_ccc
  DDDDDDDD value_dddddddd
  E        value_e
)

target_link_libraries(synthetic_shared
  PUBLIC
    Boost::filesystem
    Boost::program_options
    Boost::regex
    Boost::system
    Boost::thread

  INTERFACE
    Boost::headers

  PRIVATE
    ${OPENSSL_LIBRARIES}
    Threads::Threads
    ZLIB::ZLIB
)

# --------------------------------------------------------------------------
# Pragma regions
# --------------------------------------------------------------------------

# cmakefmt: off
# This block is not formatted
SET(  UNFORMATTED_VAR
     value1     value2     value3
)
# cmakefmt: on

# cmakefmt: skip
SET(  SKIP_SINGLE_LINE    value1    value2  )

# cmakefmt: push { commandCase = "upper" }
SET(PUSH_TEST_VAR "this set should become uppercase")
# cmakefmt: pop

# cmakefmt: push { "lineWidth": 40 }
set(NARROW_VAR
  value1 value2 value3 value4 value5
  value6 value7 value8
)
# cmakefmt: pop

# cmakefmt: push { "commandCase": "upper" }
SET(OUTER_PUSH "should be uppercase")
# cmakefmt: push { "keywordCase": "lower" }
TARGET_LINK_LIBRARIES(synthetic_core public Boost::filesystem)
# cmakefmt: pop
SET(BACK_TO_OUTER "still uppercase")
# cmakefmt: pop

# cmakefmt: skip
SET(   SKIP_0    value_0   )
# cmakefmt: skip
SET(   SKIP_1    value_1   )
# cmakefmt: skip
SET(   SKIP_2    value_2   )
# cmakefmt: skip
SET(   SKIP_3    value_3   )
# cmakefmt: skip
SET(   SKIP_4    value_4   )

# --------------------------------------------------------------------------
# cmake_language()
# --------------------------------------------------------------------------

cmake_language(CALL message STATUS "Called via cmake_language")

cmake_language(EVAL CODE "
  message(STATUS \"Evaluated code\")
")

cmake_language(
  DEFER

  CALL
    message
  STATUS "Deferred message"
)

cmake_language(
  DEFER

  DIRECTORY
    ${CMAKE_CURRENT_SOURCE_DIR}

  CALL
    message
  STATUS "Deferred to directory"
)

cmake_language(
  DEFER

  ID
    synthetic_defer_id

  CALL
    synthetic_cleanup
)

cmake_language(
  DEFER

  ID_VAR
    SYNTHETIC_DEFER_VARIABLE

  CALL
    synthetic_late_init
)

cmake_language(DEFER GET_CALL_IDS SYNTHETIC_ALL_DEFERS)
cmake_language(DEFER CANCEL_CALL synthetic_defer_id)

# --------------------------------------------------------------------------
# cmake_path()
# --------------------------------------------------------------------------

cmake_path(SET SYNTHETIC_PATH NORMALIZE "/usr/local/bin/../lib")
cmake_path(
  APPEND
    SYNTHETIC_PATH
  "subdir" "file.txt"

  OUTPUT_VARIABLE
    SYNTHETIC_FULL_PATH
)
cmake_path(GET SYNTHETIC_PATH ROOT_NAME SYNTHETIC_ROOT)
cmake_path(GET SYNTHETIC_PATH FILENAME SYNTHETIC_FILENAME)
cmake_path(GET SYNTHETIC_PATH EXTENSION LAST_ONLY SYNTHETIC_EXT)
cmake_path(GET SYNTHETIC_PATH STEM SYNTHETIC_STEM)
cmake_path(GET SYNTHETIC_PATH PARENT_PATH SYNTHETIC_PARENT)
cmake_path(REPLACE_FILENAME SYNTHETIC_PATH "new_file.txt")
cmake_path(REPLACE_EXTENSION SYNTHETIC_PATH ".hpp")
cmake_path(REMOVE_FILENAME SYNTHETIC_PATH)
cmake_path(REMOVE_EXTENSION SYNTHETIC_PATH LAST_ONLY)
cmake_path(COMPARE SYNTHETIC_PATH EQUAL SYNTHETIC_OTHER SYNTHETIC_PATHS_EQUAL)
cmake_path(HAS_ROOT_NAME SYNTHETIC_PATH SYNTHETIC_HAS_ROOT)
cmake_path(IS_RELATIVE SYNTHETIC_PATH SYNTHETIC_IS_REL)
cmake_path(NATIVE_PATH SYNTHETIC_PATH NORMALIZE SYNTHETIC_NATIVE_P)
cmake_path(
  CONVERT
    "${SYNTHETIC_NATIVE_P}"

  TO_CMAKE_PATH_LIST
    SYNTHETIC_CONVERTED

  NORMALIZE
)

# --------------------------------------------------------------------------
# execute_process()
# --------------------------------------------------------------------------

execute_process(
  COMMAND
    git rev-parse HEAD

  WORKING_DIRECTORY
    ${CMAKE_CURRENT_SOURCE_DIR}

  OUTPUT_VARIABLE
    SYNTHETIC_GIT_HASH

  OUTPUT_STRIP_TRAILING_WHITESPACE

  ERROR_QUIET

  RESULT_VARIABLE
    SYNTHETIC_GIT_RESULT
)

execute_process(
  COMMAND
    ${Python3_EXECUTABLE} -c "import sys; print(sys.version)"

  OUTPUT_VARIABLE
    SYNTHETIC_PYTHON_VERSION

  OUTPUT_STRIP_TRAILING_WHITESPACE

  TIMEOUT
    10

  COMMAND_ERROR_IS_FATAL
    ANY
)

execute_process(
  COMMAND
    ${CMAKE_COMMAND} -E echo hello

  COMMAND
    ${CMAKE_COMMAND} -E echo world

  OUTPUT_VARIABLE
    SYNTHETIC_PIPELINE

  ERROR_VARIABLE
    SYNTHETIC_PIPELINE_ERR

  INPUT_FILE
    /dev/null

  TIMEOUT
    30

  ENCODING
    UTF-8
)

# --------------------------------------------------------------------------
# try_compile() and try_run()
# --------------------------------------------------------------------------

try_compile(SYNTHETIC_COMPILE_RESULT
  SOURCE_FROM_CONTENT
  test_feature.cpp
  "int main() { return 0; }"
  CXX_STANDARD
  20
  CXX_STANDARD_REQUIRED
    TRUE

  OUTPUT_VARIABLE
    SYNTHETIC_COMPILE_OUTPUT
)

try_run(SYNTHETIC_RUN_RESULT
  SYNTHETIC_COMPILE_RESULT2
  SOURCE_FROM_CONTENT
  test_run.cpp
  "#include <iostream>\nint main() { std::cout << 42; }"
  CXX_STANDARD 20
  RUN_OUTPUT_VARIABLE
    SYNTHETIC_RUN_OUTPUT
)

# --------------------------------------------------------------------------
# cmake_parse_arguments()
# --------------------------------------------------------------------------

# Define a function that uses cmake_parse_arguments
function(synthetic_add_module module_name)
  cmake_parse_arguments(PARSE_ARGV
    1
    ARG
    "SHARED;STATIC;INTERFACE;EXCLUDE_FROM_ALL"
    "OUTPUT_NAME;FOLDER;NAMESPACE"
    "SOURCES;HEADERS;DEPENDS;COMPILE_FEATURES;COMPILE_DEFINITIONS"
  )

  if(ARG_SHARED)
    add_library(${module_name} SHARED ${ARG_SOURCES})
  elseif(ARG_STATIC)
    add_library(${module_name} STATIC ${ARG_SOURCES})
  elseif(ARG_INTERFACE)
    add_library(${module_name} INTERFACE)
  else(ARG_SHARED)
    add_library(${module_name} ${ARG_SOURCES})
  endif(ARG_SHARED)

  if(DEFINED ARG_HEADERS)
    target_sources(${module_name} PUBLIC ${ARG_HEADERS})
  endif(DEFINED ARG_HEADERS)

  if(DEFINED ARG_DEPENDS)
    target_link_libraries(${module_name} PRIVATE ${ARG_DEPENDS})
  endif(DEFINED ARG_DEPENDS)

  if(DEFINED ARG_COMPILE_FEATURES)
    target_compile_features(${module_name} PUBLIC ${ARG_COMPILE_FEATURES})
  endif(DEFINED ARG_COMPILE_FEATURES)

  if(DEFINED ARG_OUTPUT_NAME)
    set_target_properties(${module_name} PROPERTIES
      OUTPUT_NAME "${ARG_OUTPUT_NAME}"
    )
  endif(DEFINED ARG_OUTPUT_NAME)

  if(DEFINED ARG_FOLDER)
    set_target_properties(${module_name} PROPERTIES FOLDER "${ARG_FOLDER}")
  endif(DEFINED ARG_FOLDER)
endfunction(synthetic_add_module module_name)

# --------------------------------------------------------------------------
# Bracket arguments and multiline strings
# --------------------------------------------------------------------------

set(SYNTHETIC_BRACKET_VAR [==[This is a bracket argument.
It spans multiple lines.
Special chars: ]] ] [ [[ are fine here.
Even "quotes" and ${variables} are literal.
]==])

set(SYNTHETIC_MULTILINE_STR "This is a multiline
quoted string that spans
multiple lines. The formatter
should preserve it verbatim.")

message(STATUS [=[Bracket message
with level-1 brackets.
Preserved verbatim.]=])

file(WRITE "${CMAKE_BINARY_DIR}/test.cmake" [=[
cmake_minimum_required(VERSION 3.20)
message(STATUS "Generated file")
]=])

# --------------------------------------------------------------------------
# Unknown / custom commands
# --------------------------------------------------------------------------

synthetic_custom_cmd_0(arg_0 arg_1 arg_2 arg_3 arg_4)
synthetic_custom_cmd_1(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5)
synthetic_custom_cmd_2(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5 arg_6)
synthetic_custom_cmd_3(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5 arg_6 arg_7)
synthetic_custom_cmd_4(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5 arg_6 arg_7 arg_8)
synthetic_custom_cmd_5(
  arg_0
  arg_1
  arg_2
  arg_3
  arg_4
  arg_5
  arg_6
  arg_7
  arg_8
  arg_9
)
synthetic_custom_cmd_6(
  arg_0
  arg_1
  arg_2
  arg_3
  arg_4
  arg_5
  arg_6
  arg_7
  arg_8
  arg_9
  arg_10
)
synthetic_custom_cmd_7(
  arg_0
  arg_1
  arg_2
  arg_3
  arg_4
  arg_5
  arg_6
  arg_7
  arg_8
  arg_9
  arg_10
  arg_11
)
synthetic_custom_cmd_8(arg_0 arg_1 arg_2 arg_3 arg_4)
synthetic_custom_cmd_9(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5)
synthetic_custom_cmd_10(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5 arg_6)
synthetic_custom_cmd_11(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5 arg_6 arg_7)
synthetic_custom_cmd_12(arg_0 arg_1 arg_2 arg_3 arg_4 arg_5 arg_6 arg_7 arg_8)
synthetic_custom_cmd_13(
  arg_0
  arg_1
  arg_2
  arg_3
  arg_4
  arg_5
  arg_6
  arg_7
  arg_8
  arg_9
)
synthetic_custom_cmd_14(
  arg_0
  arg_1
  arg_2
  arg_3
  arg_4
  arg_5
  arg_6
  arg_7
  arg_8
  arg_9
  arg_10
)

my_project_setup(
  TARGET
  synthetic_core
  MODE
  Release
  FEATURES
  feature_a
  feature_b
  feature_c
)

internal_configure(
  CONFIG_FILE "${CMAKE_CURRENT_SOURCE_DIR}/config.yml"
  OUTPUT_DIR  "${CMAKE_BINARY_DIR}/configured"
  TEMPLATES
    template_a.in
    template_b.in
    template_c.in
  VARIABLES
    VERSION=${PROJECT_VERSION}
    BUILD_TYPE=$<CONFIG>
)

# --------------------------------------------------------------------------
# Whitespace edge cases
# --------------------------------------------------------------------------

set(EXTRA_SPACES_VAR value1 value2 value3)
set(TABS_AND_SPACES  value1 value2)

set(TRAILING_WS VALUE)
# Comment with trailing spaces

set(EMPTY_STR  "")
set(EMPTY_LIST "" "" "")

set(SPACE_BEFORE_PAREN VALUE)
set(NO_SPACE_INSIDE    VALUE)
set(EXTRA_SPACE_INSIDE VALUE)

# --------------------------------------------------------------------------
# Miscellaneous commands
# --------------------------------------------------------------------------

include("${CMAKE_CURRENT_LIST_DIR}/SyntheticHelpers.cmake"
  OPTIONAL

  RESULT_VARIABLE
    SYNTHETIC_HELPERS_FOUND
)
include(CMakePackageConfigHelpers)
include(CheckCXXCompilerFlag)
include(GNUInstallDirs)
include(FetchContent)

add_subdirectory(src/module_a)
add_subdirectory(src/module_b EXCLUDE_FROM_ALL)
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/external/dep"
  "${CMAKE_BINARY_DIR}/dep_build"
)

add_dependencies(synthetic_core synthetic_all_generated)
add_dependencies(synthetic_shared synthetic_core)

message(STATUS "Synthetic status message: ${PROJECT_NAME} v${PROJECT_VERSION}")
message(WARNING
  "Synthetic warning message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(AUTHOR_WARNING
  "Synthetic author_warning message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(SEND_ERROR
  "Synthetic send_error message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(DEPRECATION
  "Synthetic deprecation message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(NOTICE "Synthetic notice message: ${PROJECT_NAME} v${PROJECT_VERSION}")
message(VERBOSE
  "Synthetic verbose message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(DEBUG "Synthetic debug message: ${PROJECT_NAME} v${PROJECT_VERSION}")
message(TRACE "Synthetic trace message: ${PROJECT_NAME} v${PROJECT_VERSION}")
message(CHECK_START
  "Synthetic check_start message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(CHECK_PASS
  "Synthetic check_pass message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)
message(CHECK_FAIL
  "Synthetic check_fail message: ${PROJECT_NAME} v${PROJECT_VERSION}"
)

define_property(
  TARGET

  PROPERTY
    SYNTHETIC_CUSTOM_PROP

  BRIEF_DOCS
    "A custom property for synthetic targets"

  FULL_DOCS
    "This property is used by the synthetic benchmark fixture to test property handling. It accepts a string value."
)

set_property(
  TARGET
    synthetic_core

  PROPERTY
    SYNTHETIC_CUSTOM_PROP "custom_value"
)
get_property(SYNTHETIC_PROP_VALUE
  TARGET
    synthetic_core

  PROPERTY
    SYNTHETIC_CUSTOM_PROP
)

set_property(
  DIRECTORY
    ${CMAKE_CURRENT_SOURCE_DIR}

  PROPERTY
    ADDITIONAL_CLEAN_FILES
      "${CMAKE_BINARY_DIR}/generated"
      "${CMAKE_BINARY_DIR}/output"
)

export(
  TARGETS   synthetic_core synthetic_shared
  NAMESPACE Synthetic::
  FILE      "${CMAKE_BINARY_DIR}/SyntheticTargets.cmake"
)

mark_as_advanced(
  SYNTHETIC_CACHE_00
  SYNTHETIC_CACHE_01
  SYNTHETIC_CACHE_02
  SYNTHETIC_CACHE_03
  SYNTHETIC_CACHE_04
)

unset(SYNTHETIC_TEMP_VAR)
unset(SYNTHETIC_CACHE_TEMP CACHE)
unset(ENV{SYNTHETIC_ENV_TEMP})

# return() is valid at file scope (exits processing of current file) We won't
# actually call it here as it would stop processing

source_group("Source Files\\Core" FILES src/core/main.cpp src/core/init.cpp)
source_group(TREE
  "${CMAKE_CURRENT_SOURCE_DIR}/src"
  PREFIX
    "Sources"

  FILES
    ${SYNTHETIC_SOURCES}
)

separate_arguments(SYNTHETIC_ARGS UNIX_COMMAND "${SYNTHETIC_CMD_LINE}")
separate_arguments(SYNTHETIC_WIN_ARGS WINDOWS_COMMAND "${SYNTHETIC_WIN_CMD}")

enable_language(Fortran OPTIONAL)

get_filename_component(SYNTHETIC_DIR ${SYNTHETIC_PATH} DIRECTORY)
get_filename_component(SYNTHETIC_NAMEWE ${SYNTHETIC_PATH} NAME_WE)
get_filename_component(SYNTHETIC_EXT2 ${SYNTHETIC_PATH} LAST_EXT)

cmake_host_system_information(
  RESULT
    SYNTHETIC_CPU_COUNT

  QUERY
    NUMBER_OF_PHYSICAL_CORES
)
cmake_host_system_information(
  RESULT
    SYNTHETIC_TOTAL_MEM

  QUERY
    TOTAL_PHYSICAL_MEMORY
)
cmake_host_system_information(RESULT SYNTHETIC_HOSTNAME QUERY HOSTNAME)

# cmake_pkg_config (CMake 3.28+)

build_command(SYNTHETIC_BUILD_CMD TARGET synthetic_core CONFIGURATION Release)

add_compile_definitions(
  SYNTHETIC_GLOBAL_DEF=1 "SYNTHETIC_VERSION=\"${PROJECT_VERSION}\""
)
add_compile_options(-fPIC $<$<CONFIG:Debug>:-fsanitize=address>)
add_link_options($<$<CONFIG:Debug>:-fsanitize=address>)

add_definitions(-DSYNTHETIC_COMPAT=1)

include(FetchContent)
fetchcontent_declare(synthetic_dep
  GIT_REPOSITORY https://github.com/example/synthetic_dep.git
  GIT_TAG        v1.2.3
  GIT_SHALLOW    TRUE
)
fetchcontent_makeavailable(synthetic_dep)

# --------------------------------------------------------------------------
# set_*_properties()
# --------------------------------------------------------------------------

set_source_files_properties(
  src/core/main.cpp
  src/core/init.cpp
  PROPERTIES
    COMPILE_FLAGS "-O2 -DSPECIAL"
    LANGUAGE      CXX
)

set_directory_properties(
  PROPERTIES
    ADDITIONAL_CLEAN_FILES "${CMAKE_BINARY_DIR}/temp"
    COMPILE_DEFINITIONS    "DIR_LEVEL_DEF=1"
)

set_package_properties(Boost
  PROPERTIES
    DESCRIPTION "Boost C++ Libraries"
    URL "https://www.boost.org"
    TYPE REQUIRED
    PURPOSE "Core dependency for filesystem and threading"
)

get_directory_property(SYNTHETIC_DIR_DEFS COMPILE_DEFINITIONS)

# --------------------------------------------------------------------------
# CTest commands
# --------------------------------------------------------------------------

ctest_start(Experimental)
ctest_configure(
  OPTIONS
    -DSYNTHETIC_TEST=ON

  RETURN_VALUE
    SYNTHETIC_CONFIGURE_RESULT
)
ctest_build(
  TARGET
    synthetic_core

  NUMBER_ERRORS
    SYNTHETIC_BUILD_ERRORS

  NUMBER_WARNINGS
    SYNTHETIC_BUILD_WARNINGS

  RETURN_VALUE
    SYNTHETIC_BUILD_RESULT
)
ctest_test(
  PARALLEL_LEVEL
    4

  EXCLUDE_LABEL
    slow

  RETURN_VALUE
    SYNTHETIC_TEST_RESULT
)
ctest_coverage(RETURN_VALUE SYNTHETIC_COVERAGE_RESULT)
ctest_memcheck(RETURN_VALUE SYNTHETIC_MEMCHECK_RESULT)
ctest_submit(RETURN_VALUE SYNTHETIC_SUBMIT_RESULT)

# ==========================================================================
# Repeated pattern batch 0
# ==========================================================================

add_library(synthetic_module_000
  STATIC
  src/modules/mod000/file_0000.cpp
  src/modules/mod000/file_0007.cpp
  src/modules/mod000/file_0014.cpp
  src/modules/mod000/file_0021.cpp
  src/modules/mod000/file_0028.cpp
  src/modules/mod000/file_0035.cpp
  src/modules/mod000/file_0042.cpp
  src/modules/mod000/file_0049.cpp
  src/modules/mod000/file_0056.cpp
  src/modules/mod000/file_0063.cpp
  src/modules/mod000/file_0070.cpp
  src/modules/mod000/file_0077.cpp
  src/modules/mod000/file_0084.cpp
  src/modules/mod000/file_0091.cpp
  src/modules/mod000/file_0098.cpp
)

set_target_properties(synthetic_module_000 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch0"
  OUTPUT_NAME           "mod000"
)

target_include_directories(synthetic_module_000
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod000>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod000
)

target_compile_definitions(synthetic_module_000
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_000_DEBUG>
    SYNTHETIC_MODULE_000=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_000
)

target_link_libraries(synthetic_module_000
  PRIVATE
    Threads::Threads
)

set(MOD000_SETTING_A "value_0_0")
set(MOD000_SETTING_B "value_0_1")
set(MOD000_SETTING_C "value_0_2")
set(MOD000_SETTING_D "value_0_3")
set(MOD000_SETTING_E "value_0_4")

if(SYNTHETIC_ENABLE_MODULE_000)
  message(STATUS "Module 000 enabled")
  target_compile_definitions(synthetic_module_000
    PUBLIC
      SYNTHETIC_MODULE_000_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_000)
  message(STATUS "Module 000 disabled")
endif(SYNTHETIC_ENABLE_MODULE_000)

add_executable(test_module_000
  tests/modules/mod000/test_0.cpp
  tests/modules/mod000/test_1.cpp
  tests/modules/mod000/test_2.cpp
  tests/modules/mod000/test_3.cpp
  tests/modules/mod000/test_4.cpp
)

target_link_libraries(test_module_000
  PRIVATE
    GTest::gtest_main synthetic_module_000
)
gtest_discover_tests(test_module_000
  TEST_PREFIX
    "mod000::"

  DISCOVERY_TIMEOUT
    60
)

# Module 000 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD000_A   val_a)   # first
set(MOD000_BB  val_bb)  # second
set(MOD000_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 1
# ==========================================================================

add_library(synthetic_module_001
  STATIC
  src/modules/mod001/file_0001.cpp
  src/modules/mod001/file_0008.cpp
  src/modules/mod001/file_0015.cpp
  src/modules/mod001/file_0022.cpp
  src/modules/mod001/file_0029.cpp
  src/modules/mod001/file_0036.cpp
  src/modules/mod001/file_0043.cpp
  src/modules/mod001/file_0050.cpp
  src/modules/mod001/file_0057.cpp
  src/modules/mod001/file_0064.cpp
  src/modules/mod001/file_0071.cpp
  src/modules/mod001/file_0078.cpp
  src/modules/mod001/file_0085.cpp
  src/modules/mod001/file_0092.cpp
  src/modules/mod001/file_0099.cpp
)

set_target_properties(synthetic_module_001 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch1"
  OUTPUT_NAME           "mod001"
)

target_include_directories(synthetic_module_001
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod001>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod001
)

target_compile_definitions(synthetic_module_001
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_001_DEBUG>
    SYNTHETIC_MODULE_001=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_001
)

target_link_libraries(synthetic_module_001
  PUBLIC
    synthetic_module_000

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD001_SETTING_A "value_1_0")
set(MOD001_SETTING_B "value_1_1")
set(MOD001_SETTING_C "value_1_2")
set(MOD001_SETTING_D "value_1_3")
set(MOD001_SETTING_E "value_1_4")

if(SYNTHETIC_ENABLE_MODULE_001)
  message(STATUS "Module 001 enabled")
  target_compile_definitions(synthetic_module_001
    PUBLIC
      SYNTHETIC_MODULE_001_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_001)
  message(STATUS "Module 001 disabled")
endif(SYNTHETIC_ENABLE_MODULE_001)

add_executable(test_module_001
  tests/modules/mod001/test_0.cpp
  tests/modules/mod001/test_1.cpp
  tests/modules/mod001/test_2.cpp
  tests/modules/mod001/test_3.cpp
  tests/modules/mod001/test_4.cpp
)

target_link_libraries(test_module_001
  PRIVATE
    GTest::gtest_main synthetic_module_001
)
gtest_discover_tests(test_module_001
  TEST_PREFIX
    "mod001::"

  DISCOVERY_TIMEOUT
    60
)

# Module 001 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD001_A   val_a)   # first
set(MOD001_BB  val_bb)  # second
set(MOD001_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 2
# ==========================================================================

add_library(synthetic_module_002
  STATIC
  src/modules/mod002/file_0000.cpp
  src/modules/mod002/file_0002.cpp
  src/modules/mod002/file_0009.cpp
  src/modules/mod002/file_0016.cpp
  src/modules/mod002/file_0023.cpp
  src/modules/mod002/file_0030.cpp
  src/modules/mod002/file_0037.cpp
  src/modules/mod002/file_0044.cpp
  src/modules/mod002/file_0051.cpp
  src/modules/mod002/file_0058.cpp
  src/modules/mod002/file_0065.cpp
  src/modules/mod002/file_0072.cpp
  src/modules/mod002/file_0079.cpp
  src/modules/mod002/file_0086.cpp
  src/modules/mod002/file_0093.cpp
)

set_target_properties(synthetic_module_002 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch2"
  OUTPUT_NAME           "mod002"
)

target_include_directories(synthetic_module_002
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod002>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod002
)

target_compile_definitions(synthetic_module_002
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_002_DEBUG>
    SYNTHETIC_MODULE_002=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_002
)

target_link_libraries(synthetic_module_002
  PUBLIC
    synthetic_module_001

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD002_SETTING_A "value_2_0")
set(MOD002_SETTING_B "value_2_1")
set(MOD002_SETTING_C "value_2_2")
set(MOD002_SETTING_D "value_2_3")
set(MOD002_SETTING_E "value_2_4")

if(SYNTHETIC_ENABLE_MODULE_002)
  message(STATUS "Module 002 enabled")
  target_compile_definitions(synthetic_module_002
    PUBLIC
      SYNTHETIC_MODULE_002_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_002)
  message(STATUS "Module 002 disabled")
endif(SYNTHETIC_ENABLE_MODULE_002)

add_executable(test_module_002
  tests/modules/mod002/test_0.cpp
  tests/modules/mod002/test_1.cpp
  tests/modules/mod002/test_2.cpp
  tests/modules/mod002/test_3.cpp
  tests/modules/mod002/test_4.cpp
)

target_link_libraries(test_module_002
  PRIVATE
    GTest::gtest_main synthetic_module_002
)
gtest_discover_tests(test_module_002
  TEST_PREFIX
    "mod002::"

  DISCOVERY_TIMEOUT
    60
)

# Module 002 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD002_A   val_a)   # first
set(MOD002_BB  val_bb)  # second
set(MOD002_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 3
# ==========================================================================

add_library(synthetic_module_003
  STATIC
  src/modules/mod003/file_0001.cpp
  src/modules/mod003/file_0003.cpp
  src/modules/mod003/file_0010.cpp
  src/modules/mod003/file_0017.cpp
  src/modules/mod003/file_0024.cpp
  src/modules/mod003/file_0031.cpp
  src/modules/mod003/file_0038.cpp
  src/modules/mod003/file_0045.cpp
  src/modules/mod003/file_0052.cpp
  src/modules/mod003/file_0059.cpp
  src/modules/mod003/file_0066.cpp
  src/modules/mod003/file_0073.cpp
  src/modules/mod003/file_0080.cpp
  src/modules/mod003/file_0087.cpp
  src/modules/mod003/file_0094.cpp
)

set_target_properties(synthetic_module_003 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch3"
  OUTPUT_NAME           "mod003"
)

target_include_directories(synthetic_module_003
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod003>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod003
)

target_compile_definitions(synthetic_module_003
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_003_DEBUG>
    SYNTHETIC_MODULE_003=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_003
)

target_link_libraries(synthetic_module_003
  PUBLIC
    synthetic_module_002

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD003_SETTING_A "value_3_0")
set(MOD003_SETTING_B "value_3_1")
set(MOD003_SETTING_C "value_3_2")
set(MOD003_SETTING_D "value_3_3")
set(MOD003_SETTING_E "value_3_4")

if(SYNTHETIC_ENABLE_MODULE_003)
  message(STATUS "Module 003 enabled")
  target_compile_definitions(synthetic_module_003
    PUBLIC
      SYNTHETIC_MODULE_003_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_003)
  message(STATUS "Module 003 disabled")
endif(SYNTHETIC_ENABLE_MODULE_003)

add_executable(test_module_003
  tests/modules/mod003/test_0.cpp
  tests/modules/mod003/test_1.cpp
  tests/modules/mod003/test_2.cpp
  tests/modules/mod003/test_3.cpp
  tests/modules/mod003/test_4.cpp
)

target_link_libraries(test_module_003
  PRIVATE
    GTest::gtest_main synthetic_module_003
)
gtest_discover_tests(test_module_003
  TEST_PREFIX
    "mod003::"

  DISCOVERY_TIMEOUT
    60
)

# Module 003 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD003_A   val_a)   # first
set(MOD003_BB  val_bb)  # second
set(MOD003_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 4
# ==========================================================================

add_library(synthetic_module_004
  STATIC
  src/modules/mod004/file_0002.cpp
  src/modules/mod004/file_0004.cpp
  src/modules/mod004/file_0011.cpp
  src/modules/mod004/file_0018.cpp
  src/modules/mod004/file_0025.cpp
  src/modules/mod004/file_0032.cpp
  src/modules/mod004/file_0039.cpp
  src/modules/mod004/file_0046.cpp
  src/modules/mod004/file_0053.cpp
  src/modules/mod004/file_0060.cpp
  src/modules/mod004/file_0067.cpp
  src/modules/mod004/file_0074.cpp
  src/modules/mod004/file_0081.cpp
  src/modules/mod004/file_0088.cpp
  src/modules/mod004/file_0095.cpp
)

set_target_properties(synthetic_module_004 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch4"
  OUTPUT_NAME           "mod004"
)

target_include_directories(synthetic_module_004
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod004>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod004
)

target_compile_definitions(synthetic_module_004
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_004_DEBUG>
    SYNTHETIC_MODULE_004=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_004
)

target_link_libraries(synthetic_module_004
  PUBLIC
    synthetic_module_003

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD004_SETTING_A "value_4_0")
set(MOD004_SETTING_B "value_4_1")
set(MOD004_SETTING_C "value_4_2")
set(MOD004_SETTING_D "value_4_3")
set(MOD004_SETTING_E "value_4_4")

if(SYNTHETIC_ENABLE_MODULE_004)
  message(STATUS "Module 004 enabled")
  target_compile_definitions(synthetic_module_004
    PUBLIC
      SYNTHETIC_MODULE_004_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_004)
  message(STATUS "Module 004 disabled")
endif(SYNTHETIC_ENABLE_MODULE_004)

add_executable(test_module_004
  tests/modules/mod004/test_0.cpp
  tests/modules/mod004/test_1.cpp
  tests/modules/mod004/test_2.cpp
  tests/modules/mod004/test_3.cpp
  tests/modules/mod004/test_4.cpp
)

target_link_libraries(test_module_004
  PRIVATE
    GTest::gtest_main synthetic_module_004
)
gtest_discover_tests(test_module_004
  TEST_PREFIX
    "mod004::"

  DISCOVERY_TIMEOUT
    60
)

# Module 004 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD004_A   val_a)   # first
set(MOD004_BB  val_bb)  # second
set(MOD004_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 5
# ==========================================================================

add_library(synthetic_module_005
  STATIC
  src/modules/mod005/file_0003.cpp
  src/modules/mod005/file_0005.cpp
  src/modules/mod005/file_0012.cpp
  src/modules/mod005/file_0019.cpp
  src/modules/mod005/file_0026.cpp
  src/modules/mod005/file_0033.cpp
  src/modules/mod005/file_0040.cpp
  src/modules/mod005/file_0047.cpp
  src/modules/mod005/file_0054.cpp
  src/modules/mod005/file_0061.cpp
  src/modules/mod005/file_0068.cpp
  src/modules/mod005/file_0075.cpp
  src/modules/mod005/file_0082.cpp
  src/modules/mod005/file_0089.cpp
  src/modules/mod005/file_0096.cpp
)

set_target_properties(synthetic_module_005 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch5"
  OUTPUT_NAME           "mod005"
)

target_include_directories(synthetic_module_005
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod005>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod005
)

target_compile_definitions(synthetic_module_005
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_005_DEBUG>
    SYNTHETIC_MODULE_005=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_005
)

target_link_libraries(synthetic_module_005
  PUBLIC
    synthetic_module_004

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD005_SETTING_A "value_5_0")
set(MOD005_SETTING_B "value_5_1")
set(MOD005_SETTING_C "value_5_2")
set(MOD005_SETTING_D "value_5_3")
set(MOD005_SETTING_E "value_5_4")

if(SYNTHETIC_ENABLE_MODULE_005)
  message(STATUS "Module 005 enabled")
  target_compile_definitions(synthetic_module_005
    PUBLIC
      SYNTHETIC_MODULE_005_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_005)
  message(STATUS "Module 005 disabled")
endif(SYNTHETIC_ENABLE_MODULE_005)

add_executable(test_module_005
  tests/modules/mod005/test_0.cpp
  tests/modules/mod005/test_1.cpp
  tests/modules/mod005/test_2.cpp
  tests/modules/mod005/test_3.cpp
  tests/modules/mod005/test_4.cpp
)

target_link_libraries(test_module_005
  PRIVATE
    GTest::gtest_main synthetic_module_005
)
gtest_discover_tests(test_module_005
  TEST_PREFIX
    "mod005::"

  DISCOVERY_TIMEOUT
    60
)

# Module 005 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD005_A   val_a)   # first
set(MOD005_BB  val_bb)  # second
set(MOD005_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 6
# ==========================================================================

add_library(synthetic_module_006
  STATIC
  src/modules/mod006/file_0004.cpp
  src/modules/mod006/file_0006.cpp
  src/modules/mod006/file_0013.cpp
  src/modules/mod006/file_0020.cpp
  src/modules/mod006/file_0027.cpp
  src/modules/mod006/file_0034.cpp
  src/modules/mod006/file_0041.cpp
  src/modules/mod006/file_0048.cpp
  src/modules/mod006/file_0055.cpp
  src/modules/mod006/file_0062.cpp
  src/modules/mod006/file_0069.cpp
  src/modules/mod006/file_0076.cpp
  src/modules/mod006/file_0083.cpp
  src/modules/mod006/file_0090.cpp
  src/modules/mod006/file_0097.cpp
)

set_target_properties(synthetic_module_006 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch6"
  OUTPUT_NAME           "mod006"
)

target_include_directories(synthetic_module_006
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod006>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod006
)

target_compile_definitions(synthetic_module_006
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_006_DEBUG>
    SYNTHETIC_MODULE_006=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_006
)

target_link_libraries(synthetic_module_006
  PUBLIC
    synthetic_module_005

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD006_SETTING_A "value_6_0")
set(MOD006_SETTING_B "value_6_1")
set(MOD006_SETTING_C "value_6_2")
set(MOD006_SETTING_D "value_6_3")
set(MOD006_SETTING_E "value_6_4")

if(SYNTHETIC_ENABLE_MODULE_006)
  message(STATUS "Module 006 enabled")
  target_compile_definitions(synthetic_module_006
    PUBLIC
      SYNTHETIC_MODULE_006_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_006)
  message(STATUS "Module 006 disabled")
endif(SYNTHETIC_ENABLE_MODULE_006)

add_executable(test_module_006
  tests/modules/mod006/test_0.cpp
  tests/modules/mod006/test_1.cpp
  tests/modules/mod006/test_2.cpp
  tests/modules/mod006/test_3.cpp
  tests/modules/mod006/test_4.cpp
)

target_link_libraries(test_module_006
  PRIVATE
    GTest::gtest_main synthetic_module_006
)
gtest_discover_tests(test_module_006
  TEST_PREFIX
    "mod006::"

  DISCOVERY_TIMEOUT
    60
)

# Module 006 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD006_A   val_a)   # first
set(MOD006_BB  val_bb)  # second
set(MOD006_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 7
# ==========================================================================

add_library(synthetic_module_007
  STATIC
  src/modules/mod007/file_0005.cpp
  src/modules/mod007/file_0007.cpp
  src/modules/mod007/file_0014.cpp
  src/modules/mod007/file_0021.cpp
  src/modules/mod007/file_0028.cpp
  src/modules/mod007/file_0035.cpp
  src/modules/mod007/file_0042.cpp
  src/modules/mod007/file_0049.cpp
  src/modules/mod007/file_0056.cpp
  src/modules/mod007/file_0063.cpp
  src/modules/mod007/file_0070.cpp
  src/modules/mod007/file_0077.cpp
  src/modules/mod007/file_0084.cpp
  src/modules/mod007/file_0091.cpp
  src/modules/mod007/file_0098.cpp
)

set_target_properties(synthetic_module_007 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch7"
  OUTPUT_NAME           "mod007"
)

target_include_directories(synthetic_module_007
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod007>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod007
)

target_compile_definitions(synthetic_module_007
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_007_DEBUG>
    SYNTHETIC_MODULE_007=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_007
)

target_link_libraries(synthetic_module_007
  PUBLIC
    synthetic_module_006

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD007_SETTING_A "value_7_0")
set(MOD007_SETTING_B "value_7_1")
set(MOD007_SETTING_C "value_7_2")
set(MOD007_SETTING_D "value_7_3")
set(MOD007_SETTING_E "value_7_4")

if(SYNTHETIC_ENABLE_MODULE_007)
  message(STATUS "Module 007 enabled")
  target_compile_definitions(synthetic_module_007
    PUBLIC
      SYNTHETIC_MODULE_007_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_007)
  message(STATUS "Module 007 disabled")
endif(SYNTHETIC_ENABLE_MODULE_007)

add_executable(test_module_007
  tests/modules/mod007/test_0.cpp
  tests/modules/mod007/test_1.cpp
  tests/modules/mod007/test_2.cpp
  tests/modules/mod007/test_3.cpp
  tests/modules/mod007/test_4.cpp
)

target_link_libraries(test_module_007
  PRIVATE
    GTest::gtest_main synthetic_module_007
)
gtest_discover_tests(test_module_007
  TEST_PREFIX
    "mod007::"

  DISCOVERY_TIMEOUT
    60
)

# Module 007 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD007_A   val_a)   # first
set(MOD007_BB  val_bb)  # second
set(MOD007_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 8
# ==========================================================================

add_library(synthetic_module_008
  STATIC
  src/modules/mod008/file_0006.cpp
  src/modules/mod008/file_0008.cpp
  src/modules/mod008/file_0015.cpp
  src/modules/mod008/file_0022.cpp
  src/modules/mod008/file_0029.cpp
  src/modules/mod008/file_0036.cpp
  src/modules/mod008/file_0043.cpp
  src/modules/mod008/file_0050.cpp
  src/modules/mod008/file_0057.cpp
  src/modules/mod008/file_0064.cpp
  src/modules/mod008/file_0071.cpp
  src/modules/mod008/file_0078.cpp
  src/modules/mod008/file_0085.cpp
  src/modules/mod008/file_0092.cpp
  src/modules/mod008/file_0099.cpp
)

set_target_properties(synthetic_module_008 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch8"
  OUTPUT_NAME           "mod008"
)

target_include_directories(synthetic_module_008
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod008>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod008
)

target_compile_definitions(synthetic_module_008
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_008_DEBUG>
    SYNTHETIC_MODULE_008=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_008
)

target_link_libraries(synthetic_module_008
  PUBLIC
    synthetic_module_007

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD008_SETTING_A "value_8_0")
set(MOD008_SETTING_B "value_8_1")
set(MOD008_SETTING_C "value_8_2")
set(MOD008_SETTING_D "value_8_3")
set(MOD008_SETTING_E "value_8_4")

if(SYNTHETIC_ENABLE_MODULE_008)
  message(STATUS "Module 008 enabled")
  target_compile_definitions(synthetic_module_008
    PUBLIC
      SYNTHETIC_MODULE_008_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_008)
  message(STATUS "Module 008 disabled")
endif(SYNTHETIC_ENABLE_MODULE_008)

add_executable(test_module_008
  tests/modules/mod008/test_0.cpp
  tests/modules/mod008/test_1.cpp
  tests/modules/mod008/test_2.cpp
  tests/modules/mod008/test_3.cpp
  tests/modules/mod008/test_4.cpp
)

target_link_libraries(test_module_008
  PRIVATE
    GTest::gtest_main synthetic_module_008
)
gtest_discover_tests(test_module_008
  TEST_PREFIX
    "mod008::"

  DISCOVERY_TIMEOUT
    60
)

# Module 008 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD008_A   val_a)   # first
set(MOD008_BB  val_bb)  # second
set(MOD008_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 9
# ==========================================================================

add_library(synthetic_module_009
  STATIC
  src/modules/mod009/file_0000.cpp
  src/modules/mod009/file_0007.cpp
  src/modules/mod009/file_0009.cpp
  src/modules/mod009/file_0016.cpp
  src/modules/mod009/file_0023.cpp
  src/modules/mod009/file_0030.cpp
  src/modules/mod009/file_0037.cpp
  src/modules/mod009/file_0044.cpp
  src/modules/mod009/file_0051.cpp
  src/modules/mod009/file_0058.cpp
  src/modules/mod009/file_0065.cpp
  src/modules/mod009/file_0072.cpp
  src/modules/mod009/file_0079.cpp
  src/modules/mod009/file_0086.cpp
  src/modules/mod009/file_0093.cpp
)

set_target_properties(synthetic_module_009 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch9"
  OUTPUT_NAME           "mod009"
)

target_include_directories(synthetic_module_009
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod009>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod009
)

target_compile_definitions(synthetic_module_009
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_009_DEBUG>
    SYNTHETIC_MODULE_009=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_009
)

target_link_libraries(synthetic_module_009
  PUBLIC
    synthetic_module_008

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD009_SETTING_A "value_9_0")
set(MOD009_SETTING_B "value_9_1")
set(MOD009_SETTING_C "value_9_2")
set(MOD009_SETTING_D "value_9_3")
set(MOD009_SETTING_E "value_9_4")

if(SYNTHETIC_ENABLE_MODULE_009)
  message(STATUS "Module 009 enabled")
  target_compile_definitions(synthetic_module_009
    PUBLIC
      SYNTHETIC_MODULE_009_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_009)
  message(STATUS "Module 009 disabled")
endif(SYNTHETIC_ENABLE_MODULE_009)

add_executable(test_module_009
  tests/modules/mod009/test_0.cpp
  tests/modules/mod009/test_1.cpp
  tests/modules/mod009/test_2.cpp
  tests/modules/mod009/test_3.cpp
  tests/modules/mod009/test_4.cpp
)

target_link_libraries(test_module_009
  PRIVATE
    GTest::gtest_main synthetic_module_009
)
gtest_discover_tests(test_module_009
  TEST_PREFIX
    "mod009::"

  DISCOVERY_TIMEOUT
    60
)

# Module 009 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD009_A   val_a)   # first
set(MOD009_BB  val_bb)  # second
set(MOD009_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 10
# ==========================================================================

add_library(synthetic_module_010
  STATIC
  src/modules/mod010/file_0001.cpp
  src/modules/mod010/file_0008.cpp
  src/modules/mod010/file_0010.cpp
  src/modules/mod010/file_0017.cpp
  src/modules/mod010/file_0024.cpp
  src/modules/mod010/file_0031.cpp
  src/modules/mod010/file_0038.cpp
  src/modules/mod010/file_0045.cpp
  src/modules/mod010/file_0052.cpp
  src/modules/mod010/file_0059.cpp
  src/modules/mod010/file_0066.cpp
  src/modules/mod010/file_0073.cpp
  src/modules/mod010/file_0080.cpp
  src/modules/mod010/file_0087.cpp
  src/modules/mod010/file_0094.cpp
)

set_target_properties(synthetic_module_010 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch10"
  OUTPUT_NAME           "mod010"
)

target_include_directories(synthetic_module_010
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod010>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod010
)

target_compile_definitions(synthetic_module_010
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_010_DEBUG>
    SYNTHETIC_MODULE_010=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_010
)

target_link_libraries(synthetic_module_010
  PUBLIC
    synthetic_module_009

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD010_SETTING_A "value_10_0")
set(MOD010_SETTING_B "value_10_1")
set(MOD010_SETTING_C "value_10_2")
set(MOD010_SETTING_D "value_10_3")
set(MOD010_SETTING_E "value_10_4")

if(SYNTHETIC_ENABLE_MODULE_010)
  message(STATUS "Module 010 enabled")
  target_compile_definitions(synthetic_module_010
    PUBLIC
      SYNTHETIC_MODULE_010_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_010)
  message(STATUS "Module 010 disabled")
endif(SYNTHETIC_ENABLE_MODULE_010)

add_executable(test_module_010
  tests/modules/mod010/test_0.cpp
  tests/modules/mod010/test_1.cpp
  tests/modules/mod010/test_2.cpp
  tests/modules/mod010/test_3.cpp
  tests/modules/mod010/test_4.cpp
)

target_link_libraries(test_module_010
  PRIVATE
    GTest::gtest_main synthetic_module_010
)
gtest_discover_tests(test_module_010
  TEST_PREFIX
    "mod010::"

  DISCOVERY_TIMEOUT
    60
)

# Module 010 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD010_A   val_a)   # first
set(MOD010_BB  val_bb)  # second
set(MOD010_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 11
# ==========================================================================

add_library(synthetic_module_011
  STATIC
  src/modules/mod011/file_0002.cpp
  src/modules/mod011/file_0009.cpp
  src/modules/mod011/file_0011.cpp
  src/modules/mod011/file_0018.cpp
  src/modules/mod011/file_0025.cpp
  src/modules/mod011/file_0032.cpp
  src/modules/mod011/file_0039.cpp
  src/modules/mod011/file_0046.cpp
  src/modules/mod011/file_0053.cpp
  src/modules/mod011/file_0060.cpp
  src/modules/mod011/file_0067.cpp
  src/modules/mod011/file_0074.cpp
  src/modules/mod011/file_0081.cpp
  src/modules/mod011/file_0088.cpp
  src/modules/mod011/file_0095.cpp
)

set_target_properties(synthetic_module_011 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch11"
  OUTPUT_NAME           "mod011"
)

target_include_directories(synthetic_module_011
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod011>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod011
)

target_compile_definitions(synthetic_module_011
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_011_DEBUG>
    SYNTHETIC_MODULE_011=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_011
)

target_link_libraries(synthetic_module_011
  PUBLIC
    synthetic_module_010

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD011_SETTING_A "value_11_0")
set(MOD011_SETTING_B "value_11_1")
set(MOD011_SETTING_C "value_11_2")
set(MOD011_SETTING_D "value_11_3")
set(MOD011_SETTING_E "value_11_4")

if(SYNTHETIC_ENABLE_MODULE_011)
  message(STATUS "Module 011 enabled")
  target_compile_definitions(synthetic_module_011
    PUBLIC
      SYNTHETIC_MODULE_011_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_011)
  message(STATUS "Module 011 disabled")
endif(SYNTHETIC_ENABLE_MODULE_011)

add_executable(test_module_011
  tests/modules/mod011/test_0.cpp
  tests/modules/mod011/test_1.cpp
  tests/modules/mod011/test_2.cpp
  tests/modules/mod011/test_3.cpp
  tests/modules/mod011/test_4.cpp
)

target_link_libraries(test_module_011
  PRIVATE
    GTest::gtest_main synthetic_module_011
)
gtest_discover_tests(test_module_011
  TEST_PREFIX
    "mod011::"

  DISCOVERY_TIMEOUT
    60
)

# Module 011 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD011_A   val_a)   # first
set(MOD011_BB  val_bb)  # second
set(MOD011_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 12
# ==========================================================================

add_library(synthetic_module_012
  STATIC
  src/modules/mod012/file_0003.cpp
  src/modules/mod012/file_0010.cpp
  src/modules/mod012/file_0012.cpp
  src/modules/mod012/file_0019.cpp
  src/modules/mod012/file_0026.cpp
  src/modules/mod012/file_0033.cpp
  src/modules/mod012/file_0040.cpp
  src/modules/mod012/file_0047.cpp
  src/modules/mod012/file_0054.cpp
  src/modules/mod012/file_0061.cpp
  src/modules/mod012/file_0068.cpp
  src/modules/mod012/file_0075.cpp
  src/modules/mod012/file_0082.cpp
  src/modules/mod012/file_0089.cpp
  src/modules/mod012/file_0096.cpp
)

set_target_properties(synthetic_module_012 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch12"
  OUTPUT_NAME           "mod012"
)

target_include_directories(synthetic_module_012
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod012>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod012
)

target_compile_definitions(synthetic_module_012
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_012_DEBUG>
    SYNTHETIC_MODULE_012=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_012
)

target_link_libraries(synthetic_module_012
  PUBLIC
    synthetic_module_011

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD012_SETTING_A "value_12_0")
set(MOD012_SETTING_B "value_12_1")
set(MOD012_SETTING_C "value_12_2")
set(MOD012_SETTING_D "value_12_3")
set(MOD012_SETTING_E "value_12_4")

if(SYNTHETIC_ENABLE_MODULE_012)
  message(STATUS "Module 012 enabled")
  target_compile_definitions(synthetic_module_012
    PUBLIC
      SYNTHETIC_MODULE_012_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_012)
  message(STATUS "Module 012 disabled")
endif(SYNTHETIC_ENABLE_MODULE_012)

add_executable(test_module_012
  tests/modules/mod012/test_0.cpp
  tests/modules/mod012/test_1.cpp
  tests/modules/mod012/test_2.cpp
  tests/modules/mod012/test_3.cpp
  tests/modules/mod012/test_4.cpp
)

target_link_libraries(test_module_012
  PRIVATE
    GTest::gtest_main synthetic_module_012
)
gtest_discover_tests(test_module_012
  TEST_PREFIX
    "mod012::"

  DISCOVERY_TIMEOUT
    60
)

# Module 012 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD012_A   val_a)   # first
set(MOD012_BB  val_bb)  # second
set(MOD012_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 13
# ==========================================================================

add_library(synthetic_module_013
  STATIC
  src/modules/mod013/file_0004.cpp
  src/modules/mod013/file_0011.cpp
  src/modules/mod013/file_0013.cpp
  src/modules/mod013/file_0020.cpp
  src/modules/mod013/file_0027.cpp
  src/modules/mod013/file_0034.cpp
  src/modules/mod013/file_0041.cpp
  src/modules/mod013/file_0048.cpp
  src/modules/mod013/file_0055.cpp
  src/modules/mod013/file_0062.cpp
  src/modules/mod013/file_0069.cpp
  src/modules/mod013/file_0076.cpp
  src/modules/mod013/file_0083.cpp
  src/modules/mod013/file_0090.cpp
  src/modules/mod013/file_0097.cpp
)

set_target_properties(synthetic_module_013 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch13"
  OUTPUT_NAME           "mod013"
)

target_include_directories(synthetic_module_013
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod013>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod013
)

target_compile_definitions(synthetic_module_013
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_013_DEBUG>
    SYNTHETIC_MODULE_013=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_013
)

target_link_libraries(synthetic_module_013
  PUBLIC
    synthetic_module_012

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD013_SETTING_A "value_13_0")
set(MOD013_SETTING_B "value_13_1")
set(MOD013_SETTING_C "value_13_2")
set(MOD013_SETTING_D "value_13_3")
set(MOD013_SETTING_E "value_13_4")

if(SYNTHETIC_ENABLE_MODULE_013)
  message(STATUS "Module 013 enabled")
  target_compile_definitions(synthetic_module_013
    PUBLIC
      SYNTHETIC_MODULE_013_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_013)
  message(STATUS "Module 013 disabled")
endif(SYNTHETIC_ENABLE_MODULE_013)

add_executable(test_module_013
  tests/modules/mod013/test_0.cpp
  tests/modules/mod013/test_1.cpp
  tests/modules/mod013/test_2.cpp
  tests/modules/mod013/test_3.cpp
  tests/modules/mod013/test_4.cpp
)

target_link_libraries(test_module_013
  PRIVATE
    GTest::gtest_main synthetic_module_013
)
gtest_discover_tests(test_module_013
  TEST_PREFIX
    "mod013::"

  DISCOVERY_TIMEOUT
    60
)

# Module 013 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD013_A   val_a)   # first
set(MOD013_BB  val_bb)  # second
set(MOD013_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 14
# ==========================================================================

add_library(synthetic_module_014
  STATIC
  src/modules/mod014/file_0005.cpp
  src/modules/mod014/file_0012.cpp
  src/modules/mod014/file_0014.cpp
  src/modules/mod014/file_0021.cpp
  src/modules/mod014/file_0028.cpp
  src/modules/mod014/file_0035.cpp
  src/modules/mod014/file_0042.cpp
  src/modules/mod014/file_0049.cpp
  src/modules/mod014/file_0056.cpp
  src/modules/mod014/file_0063.cpp
  src/modules/mod014/file_0070.cpp
  src/modules/mod014/file_0077.cpp
  src/modules/mod014/file_0084.cpp
  src/modules/mod014/file_0091.cpp
  src/modules/mod014/file_0098.cpp
)

set_target_properties(synthetic_module_014 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch14"
  OUTPUT_NAME           "mod014"
)

target_include_directories(synthetic_module_014
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod014>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod014
)

target_compile_definitions(synthetic_module_014
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_014_DEBUG>
    SYNTHETIC_MODULE_014=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_014
)

target_link_libraries(synthetic_module_014
  PUBLIC
    synthetic_module_013

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD014_SETTING_A "value_14_0")
set(MOD014_SETTING_B "value_14_1")
set(MOD014_SETTING_C "value_14_2")
set(MOD014_SETTING_D "value_14_3")
set(MOD014_SETTING_E "value_14_4")

if(SYNTHETIC_ENABLE_MODULE_014)
  message(STATUS "Module 014 enabled")
  target_compile_definitions(synthetic_module_014
    PUBLIC
      SYNTHETIC_MODULE_014_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_014)
  message(STATUS "Module 014 disabled")
endif(SYNTHETIC_ENABLE_MODULE_014)

add_executable(test_module_014
  tests/modules/mod014/test_0.cpp
  tests/modules/mod014/test_1.cpp
  tests/modules/mod014/test_2.cpp
  tests/modules/mod014/test_3.cpp
  tests/modules/mod014/test_4.cpp
)

target_link_libraries(test_module_014
  PRIVATE
    GTest::gtest_main synthetic_module_014
)
gtest_discover_tests(test_module_014
  TEST_PREFIX
    "mod014::"

  DISCOVERY_TIMEOUT
    60
)

# Module 014 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD014_A   val_a)   # first
set(MOD014_BB  val_bb)  # second
set(MOD014_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 15
# ==========================================================================

add_library(synthetic_module_015
  STATIC
  src/modules/mod015/file_0006.cpp
  src/modules/mod015/file_0013.cpp
  src/modules/mod015/file_0015.cpp
  src/modules/mod015/file_0022.cpp
  src/modules/mod015/file_0029.cpp
  src/modules/mod015/file_0036.cpp
  src/modules/mod015/file_0043.cpp
  src/modules/mod015/file_0050.cpp
  src/modules/mod015/file_0057.cpp
  src/modules/mod015/file_0064.cpp
  src/modules/mod015/file_0071.cpp
  src/modules/mod015/file_0078.cpp
  src/modules/mod015/file_0085.cpp
  src/modules/mod015/file_0092.cpp
  src/modules/mod015/file_0099.cpp
)

set_target_properties(synthetic_module_015 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch15"
  OUTPUT_NAME           "mod015"
)

target_include_directories(synthetic_module_015
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod015>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod015
)

target_compile_definitions(synthetic_module_015
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_015_DEBUG>
    SYNTHETIC_MODULE_015=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_015
)

target_link_libraries(synthetic_module_015
  PUBLIC
    synthetic_module_014

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD015_SETTING_A "value_15_0")
set(MOD015_SETTING_B "value_15_1")
set(MOD015_SETTING_C "value_15_2")
set(MOD015_SETTING_D "value_15_3")
set(MOD015_SETTING_E "value_15_4")

if(SYNTHETIC_ENABLE_MODULE_015)
  message(STATUS "Module 015 enabled")
  target_compile_definitions(synthetic_module_015
    PUBLIC
      SYNTHETIC_MODULE_015_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_015)
  message(STATUS "Module 015 disabled")
endif(SYNTHETIC_ENABLE_MODULE_015)

add_executable(test_module_015
  tests/modules/mod015/test_0.cpp
  tests/modules/mod015/test_1.cpp
  tests/modules/mod015/test_2.cpp
  tests/modules/mod015/test_3.cpp
  tests/modules/mod015/test_4.cpp
)

target_link_libraries(test_module_015
  PRIVATE
    GTest::gtest_main synthetic_module_015
)
gtest_discover_tests(test_module_015
  TEST_PREFIX
    "mod015::"

  DISCOVERY_TIMEOUT
    60
)

# Module 015 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD015_A   val_a)   # first
set(MOD015_BB  val_bb)  # second
set(MOD015_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 16
# ==========================================================================

add_library(synthetic_module_016
  STATIC
  src/modules/mod016/file_0000.cpp
  src/modules/mod016/file_0007.cpp
  src/modules/mod016/file_0014.cpp
  src/modules/mod016/file_0016.cpp
  src/modules/mod016/file_0023.cpp
  src/modules/mod016/file_0030.cpp
  src/modules/mod016/file_0037.cpp
  src/modules/mod016/file_0044.cpp
  src/modules/mod016/file_0051.cpp
  src/modules/mod016/file_0058.cpp
  src/modules/mod016/file_0065.cpp
  src/modules/mod016/file_0072.cpp
  src/modules/mod016/file_0079.cpp
  src/modules/mod016/file_0086.cpp
  src/modules/mod016/file_0093.cpp
)

set_target_properties(synthetic_module_016 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch16"
  OUTPUT_NAME           "mod016"
)

target_include_directories(synthetic_module_016
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod016>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod016
)

target_compile_definitions(synthetic_module_016
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_016_DEBUG>
    SYNTHETIC_MODULE_016=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_016
)

target_link_libraries(synthetic_module_016
  PUBLIC
    synthetic_module_015

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD016_SETTING_A "value_16_0")
set(MOD016_SETTING_B "value_16_1")
set(MOD016_SETTING_C "value_16_2")
set(MOD016_SETTING_D "value_16_3")
set(MOD016_SETTING_E "value_16_4")

if(SYNTHETIC_ENABLE_MODULE_016)
  message(STATUS "Module 016 enabled")
  target_compile_definitions(synthetic_module_016
    PUBLIC
      SYNTHETIC_MODULE_016_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_016)
  message(STATUS "Module 016 disabled")
endif(SYNTHETIC_ENABLE_MODULE_016)

add_executable(test_module_016
  tests/modules/mod016/test_0.cpp
  tests/modules/mod016/test_1.cpp
  tests/modules/mod016/test_2.cpp
  tests/modules/mod016/test_3.cpp
  tests/modules/mod016/test_4.cpp
)

target_link_libraries(test_module_016
  PRIVATE
    GTest::gtest_main synthetic_module_016
)
gtest_discover_tests(test_module_016
  TEST_PREFIX
    "mod016::"

  DISCOVERY_TIMEOUT
    60
)

# Module 016 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD016_A   val_a)   # first
set(MOD016_BB  val_bb)  # second
set(MOD016_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 17
# ==========================================================================

add_library(synthetic_module_017
  STATIC
  src/modules/mod017/file_0001.cpp
  src/modules/mod017/file_0008.cpp
  src/modules/mod017/file_0015.cpp
  src/modules/mod017/file_0017.cpp
  src/modules/mod017/file_0024.cpp
  src/modules/mod017/file_0031.cpp
  src/modules/mod017/file_0038.cpp
  src/modules/mod017/file_0045.cpp
  src/modules/mod017/file_0052.cpp
  src/modules/mod017/file_0059.cpp
  src/modules/mod017/file_0066.cpp
  src/modules/mod017/file_0073.cpp
  src/modules/mod017/file_0080.cpp
  src/modules/mod017/file_0087.cpp
  src/modules/mod017/file_0094.cpp
)

set_target_properties(synthetic_module_017 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch17"
  OUTPUT_NAME           "mod017"
)

target_include_directories(synthetic_module_017
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod017>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod017
)

target_compile_definitions(synthetic_module_017
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_017_DEBUG>
    SYNTHETIC_MODULE_017=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_017
)

target_link_libraries(synthetic_module_017
  PUBLIC
    synthetic_module_016

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD017_SETTING_A "value_17_0")
set(MOD017_SETTING_B "value_17_1")
set(MOD017_SETTING_C "value_17_2")
set(MOD017_SETTING_D "value_17_3")
set(MOD017_SETTING_E "value_17_4")

if(SYNTHETIC_ENABLE_MODULE_017)
  message(STATUS "Module 017 enabled")
  target_compile_definitions(synthetic_module_017
    PUBLIC
      SYNTHETIC_MODULE_017_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_017)
  message(STATUS "Module 017 disabled")
endif(SYNTHETIC_ENABLE_MODULE_017)

add_executable(test_module_017
  tests/modules/mod017/test_0.cpp
  tests/modules/mod017/test_1.cpp
  tests/modules/mod017/test_2.cpp
  tests/modules/mod017/test_3.cpp
  tests/modules/mod017/test_4.cpp
)

target_link_libraries(test_module_017
  PRIVATE
    GTest::gtest_main synthetic_module_017
)
gtest_discover_tests(test_module_017
  TEST_PREFIX
    "mod017::"

  DISCOVERY_TIMEOUT
    60
)

# Module 017 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD017_A   val_a)   # first
set(MOD017_BB  val_bb)  # second
set(MOD017_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 18
# ==========================================================================

add_library(synthetic_module_018
  STATIC
  src/modules/mod018/file_0002.cpp
  src/modules/mod018/file_0009.cpp
  src/modules/mod018/file_0016.cpp
  src/modules/mod018/file_0018.cpp
  src/modules/mod018/file_0025.cpp
  src/modules/mod018/file_0032.cpp
  src/modules/mod018/file_0039.cpp
  src/modules/mod018/file_0046.cpp
  src/modules/mod018/file_0053.cpp
  src/modules/mod018/file_0060.cpp
  src/modules/mod018/file_0067.cpp
  src/modules/mod018/file_0074.cpp
  src/modules/mod018/file_0081.cpp
  src/modules/mod018/file_0088.cpp
  src/modules/mod018/file_0095.cpp
)

set_target_properties(synthetic_module_018 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch18"
  OUTPUT_NAME           "mod018"
)

target_include_directories(synthetic_module_018
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod018>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod018
)

target_compile_definitions(synthetic_module_018
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_018_DEBUG>
    SYNTHETIC_MODULE_018=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_018
)

target_link_libraries(synthetic_module_018
  PUBLIC
    synthetic_module_017

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD018_SETTING_A "value_18_0")
set(MOD018_SETTING_B "value_18_1")
set(MOD018_SETTING_C "value_18_2")
set(MOD018_SETTING_D "value_18_3")
set(MOD018_SETTING_E "value_18_4")

if(SYNTHETIC_ENABLE_MODULE_018)
  message(STATUS "Module 018 enabled")
  target_compile_definitions(synthetic_module_018
    PUBLIC
      SYNTHETIC_MODULE_018_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_018)
  message(STATUS "Module 018 disabled")
endif(SYNTHETIC_ENABLE_MODULE_018)

add_executable(test_module_018
  tests/modules/mod018/test_0.cpp
  tests/modules/mod018/test_1.cpp
  tests/modules/mod018/test_2.cpp
  tests/modules/mod018/test_3.cpp
  tests/modules/mod018/test_4.cpp
)

target_link_libraries(test_module_018
  PRIVATE
    GTest::gtest_main synthetic_module_018
)
gtest_discover_tests(test_module_018
  TEST_PREFIX
    "mod018::"

  DISCOVERY_TIMEOUT
    60
)

# Module 018 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD018_A   val_a)   # first
set(MOD018_BB  val_bb)  # second
set(MOD018_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 19
# ==========================================================================

add_library(synthetic_module_019
  STATIC
  src/modules/mod019/file_0003.cpp
  src/modules/mod019/file_0010.cpp
  src/modules/mod019/file_0017.cpp
  src/modules/mod019/file_0019.cpp
  src/modules/mod019/file_0026.cpp
  src/modules/mod019/file_0033.cpp
  src/modules/mod019/file_0040.cpp
  src/modules/mod019/file_0047.cpp
  src/modules/mod019/file_0054.cpp
  src/modules/mod019/file_0061.cpp
  src/modules/mod019/file_0068.cpp
  src/modules/mod019/file_0075.cpp
  src/modules/mod019/file_0082.cpp
  src/modules/mod019/file_0089.cpp
  src/modules/mod019/file_0096.cpp
)

set_target_properties(synthetic_module_019 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch19"
  OUTPUT_NAME           "mod019"
)

target_include_directories(synthetic_module_019
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod019>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod019
)

target_compile_definitions(synthetic_module_019
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_019_DEBUG>
    SYNTHETIC_MODULE_019=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_019
)

target_link_libraries(synthetic_module_019
  PUBLIC
    synthetic_module_018

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD019_SETTING_A "value_19_0")
set(MOD019_SETTING_B "value_19_1")
set(MOD019_SETTING_C "value_19_2")
set(MOD019_SETTING_D "value_19_3")
set(MOD019_SETTING_E "value_19_4")

if(SYNTHETIC_ENABLE_MODULE_019)
  message(STATUS "Module 019 enabled")
  target_compile_definitions(synthetic_module_019
    PUBLIC
      SYNTHETIC_MODULE_019_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_019)
  message(STATUS "Module 019 disabled")
endif(SYNTHETIC_ENABLE_MODULE_019)

add_executable(test_module_019
  tests/modules/mod019/test_0.cpp
  tests/modules/mod019/test_1.cpp
  tests/modules/mod019/test_2.cpp
  tests/modules/mod019/test_3.cpp
  tests/modules/mod019/test_4.cpp
)

target_link_libraries(test_module_019
  PRIVATE
    GTest::gtest_main synthetic_module_019
)
gtest_discover_tests(test_module_019
  TEST_PREFIX
    "mod019::"

  DISCOVERY_TIMEOUT
    60
)

# Module 019 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD019_A   val_a)   # first
set(MOD019_BB  val_bb)  # second
set(MOD019_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 20
# ==========================================================================

add_library(synthetic_module_020
  STATIC
  src/modules/mod020/file_0004.cpp
  src/modules/mod020/file_0011.cpp
  src/modules/mod020/file_0018.cpp
  src/modules/mod020/file_0020.cpp
  src/modules/mod020/file_0027.cpp
  src/modules/mod020/file_0034.cpp
  src/modules/mod020/file_0041.cpp
  src/modules/mod020/file_0048.cpp
  src/modules/mod020/file_0055.cpp
  src/modules/mod020/file_0062.cpp
  src/modules/mod020/file_0069.cpp
  src/modules/mod020/file_0076.cpp
  src/modules/mod020/file_0083.cpp
  src/modules/mod020/file_0090.cpp
  src/modules/mod020/file_0097.cpp
)

set_target_properties(synthetic_module_020 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch20"
  OUTPUT_NAME           "mod020"
)

target_include_directories(synthetic_module_020
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod020>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod020
)

target_compile_definitions(synthetic_module_020
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_020_DEBUG>
    SYNTHETIC_MODULE_020=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_020
)

target_link_libraries(synthetic_module_020
  PUBLIC
    synthetic_module_019

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD020_SETTING_A "value_20_0")
set(MOD020_SETTING_B "value_20_1")
set(MOD020_SETTING_C "value_20_2")
set(MOD020_SETTING_D "value_20_3")
set(MOD020_SETTING_E "value_20_4")

if(SYNTHETIC_ENABLE_MODULE_020)
  message(STATUS "Module 020 enabled")
  target_compile_definitions(synthetic_module_020
    PUBLIC
      SYNTHETIC_MODULE_020_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_020)
  message(STATUS "Module 020 disabled")
endif(SYNTHETIC_ENABLE_MODULE_020)

add_executable(test_module_020
  tests/modules/mod020/test_0.cpp
  tests/modules/mod020/test_1.cpp
  tests/modules/mod020/test_2.cpp
  tests/modules/mod020/test_3.cpp
  tests/modules/mod020/test_4.cpp
)

target_link_libraries(test_module_020
  PRIVATE
    GTest::gtest_main synthetic_module_020
)
gtest_discover_tests(test_module_020
  TEST_PREFIX
    "mod020::"

  DISCOVERY_TIMEOUT
    60
)

# Module 020 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD020_A   val_a)   # first
set(MOD020_BB  val_bb)  # second
set(MOD020_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 21
# ==========================================================================

add_library(synthetic_module_021
  STATIC
  src/modules/mod021/file_0005.cpp
  src/modules/mod021/file_0012.cpp
  src/modules/mod021/file_0019.cpp
  src/modules/mod021/file_0021.cpp
  src/modules/mod021/file_0028.cpp
  src/modules/mod021/file_0035.cpp
  src/modules/mod021/file_0042.cpp
  src/modules/mod021/file_0049.cpp
  src/modules/mod021/file_0056.cpp
  src/modules/mod021/file_0063.cpp
  src/modules/mod021/file_0070.cpp
  src/modules/mod021/file_0077.cpp
  src/modules/mod021/file_0084.cpp
  src/modules/mod021/file_0091.cpp
  src/modules/mod021/file_0098.cpp
)

set_target_properties(synthetic_module_021 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch21"
  OUTPUT_NAME           "mod021"
)

target_include_directories(synthetic_module_021
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod021>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod021
)

target_compile_definitions(synthetic_module_021
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_021_DEBUG>
    SYNTHETIC_MODULE_021=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_021
)

target_link_libraries(synthetic_module_021
  PUBLIC
    synthetic_module_020

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD021_SETTING_A "value_21_0")
set(MOD021_SETTING_B "value_21_1")
set(MOD021_SETTING_C "value_21_2")
set(MOD021_SETTING_D "value_21_3")
set(MOD021_SETTING_E "value_21_4")

if(SYNTHETIC_ENABLE_MODULE_021)
  message(STATUS "Module 021 enabled")
  target_compile_definitions(synthetic_module_021
    PUBLIC
      SYNTHETIC_MODULE_021_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_021)
  message(STATUS "Module 021 disabled")
endif(SYNTHETIC_ENABLE_MODULE_021)

add_executable(test_module_021
  tests/modules/mod021/test_0.cpp
  tests/modules/mod021/test_1.cpp
  tests/modules/mod021/test_2.cpp
  tests/modules/mod021/test_3.cpp
  tests/modules/mod021/test_4.cpp
)

target_link_libraries(test_module_021
  PRIVATE
    GTest::gtest_main synthetic_module_021
)
gtest_discover_tests(test_module_021
  TEST_PREFIX
    "mod021::"

  DISCOVERY_TIMEOUT
    60
)

# Module 021 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD021_A   val_a)   # first
set(MOD021_BB  val_bb)  # second
set(MOD021_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 22
# ==========================================================================

add_library(synthetic_module_022
  STATIC
  src/modules/mod022/file_0006.cpp
  src/modules/mod022/file_0013.cpp
  src/modules/mod022/file_0020.cpp
  src/modules/mod022/file_0022.cpp
  src/modules/mod022/file_0029.cpp
  src/modules/mod022/file_0036.cpp
  src/modules/mod022/file_0043.cpp
  src/modules/mod022/file_0050.cpp
  src/modules/mod022/file_0057.cpp
  src/modules/mod022/file_0064.cpp
  src/modules/mod022/file_0071.cpp
  src/modules/mod022/file_0078.cpp
  src/modules/mod022/file_0085.cpp
  src/modules/mod022/file_0092.cpp
  src/modules/mod022/file_0099.cpp
)

set_target_properties(synthetic_module_022 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch22"
  OUTPUT_NAME           "mod022"
)

target_include_directories(synthetic_module_022
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod022>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod022
)

target_compile_definitions(synthetic_module_022
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_022_DEBUG>
    SYNTHETIC_MODULE_022=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_022
)

target_link_libraries(synthetic_module_022
  PUBLIC
    synthetic_module_021

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD022_SETTING_A "value_22_0")
set(MOD022_SETTING_B "value_22_1")
set(MOD022_SETTING_C "value_22_2")
set(MOD022_SETTING_D "value_22_3")
set(MOD022_SETTING_E "value_22_4")

if(SYNTHETIC_ENABLE_MODULE_022)
  message(STATUS "Module 022 enabled")
  target_compile_definitions(synthetic_module_022
    PUBLIC
      SYNTHETIC_MODULE_022_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_022)
  message(STATUS "Module 022 disabled")
endif(SYNTHETIC_ENABLE_MODULE_022)

add_executable(test_module_022
  tests/modules/mod022/test_0.cpp
  tests/modules/mod022/test_1.cpp
  tests/modules/mod022/test_2.cpp
  tests/modules/mod022/test_3.cpp
  tests/modules/mod022/test_4.cpp
)

target_link_libraries(test_module_022
  PRIVATE
    GTest::gtest_main synthetic_module_022
)
gtest_discover_tests(test_module_022
  TEST_PREFIX
    "mod022::"

  DISCOVERY_TIMEOUT
    60
)

# Module 022 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD022_A   val_a)   # first
set(MOD022_BB  val_bb)  # second
set(MOD022_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 23
# ==========================================================================

add_library(synthetic_module_023
  STATIC
  src/modules/mod023/file_0000.cpp
  src/modules/mod023/file_0007.cpp
  src/modules/mod023/file_0014.cpp
  src/modules/mod023/file_0021.cpp
  src/modules/mod023/file_0023.cpp
  src/modules/mod023/file_0030.cpp
  src/modules/mod023/file_0037.cpp
  src/modules/mod023/file_0044.cpp
  src/modules/mod023/file_0051.cpp
  src/modules/mod023/file_0058.cpp
  src/modules/mod023/file_0065.cpp
  src/modules/mod023/file_0072.cpp
  src/modules/mod023/file_0079.cpp
  src/modules/mod023/file_0086.cpp
  src/modules/mod023/file_0093.cpp
)

set_target_properties(synthetic_module_023 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch23"
  OUTPUT_NAME           "mod023"
)

target_include_directories(synthetic_module_023
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod023>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod023
)

target_compile_definitions(synthetic_module_023
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_023_DEBUG>
    SYNTHETIC_MODULE_023=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_023
)

target_link_libraries(synthetic_module_023
  PUBLIC
    synthetic_module_022

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD023_SETTING_A "value_23_0")
set(MOD023_SETTING_B "value_23_1")
set(MOD023_SETTING_C "value_23_2")
set(MOD023_SETTING_D "value_23_3")
set(MOD023_SETTING_E "value_23_4")

if(SYNTHETIC_ENABLE_MODULE_023)
  message(STATUS "Module 023 enabled")
  target_compile_definitions(synthetic_module_023
    PUBLIC
      SYNTHETIC_MODULE_023_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_023)
  message(STATUS "Module 023 disabled")
endif(SYNTHETIC_ENABLE_MODULE_023)

add_executable(test_module_023
  tests/modules/mod023/test_0.cpp
  tests/modules/mod023/test_1.cpp
  tests/modules/mod023/test_2.cpp
  tests/modules/mod023/test_3.cpp
  tests/modules/mod023/test_4.cpp
)

target_link_libraries(test_module_023
  PRIVATE
    GTest::gtest_main synthetic_module_023
)
gtest_discover_tests(test_module_023
  TEST_PREFIX
    "mod023::"

  DISCOVERY_TIMEOUT
    60
)

# Module 023 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD023_A   val_a)   # first
set(MOD023_BB  val_bb)  # second
set(MOD023_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 24
# ==========================================================================

add_library(synthetic_module_024
  STATIC
  src/modules/mod024/file_0001.cpp
  src/modules/mod024/file_0008.cpp
  src/modules/mod024/file_0015.cpp
  src/modules/mod024/file_0022.cpp
  src/modules/mod024/file_0024.cpp
  src/modules/mod024/file_0031.cpp
  src/modules/mod024/file_0038.cpp
  src/modules/mod024/file_0045.cpp
  src/modules/mod024/file_0052.cpp
  src/modules/mod024/file_0059.cpp
  src/modules/mod024/file_0066.cpp
  src/modules/mod024/file_0073.cpp
  src/modules/mod024/file_0080.cpp
  src/modules/mod024/file_0087.cpp
  src/modules/mod024/file_0094.cpp
)

set_target_properties(synthetic_module_024 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch24"
  OUTPUT_NAME           "mod024"
)

target_include_directories(synthetic_module_024
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod024>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod024
)

target_compile_definitions(synthetic_module_024
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_024_DEBUG>
    SYNTHETIC_MODULE_024=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_024
)

target_link_libraries(synthetic_module_024
  PUBLIC
    synthetic_module_023

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD024_SETTING_A "value_24_0")
set(MOD024_SETTING_B "value_24_1")
set(MOD024_SETTING_C "value_24_2")
set(MOD024_SETTING_D "value_24_3")
set(MOD024_SETTING_E "value_24_4")

if(SYNTHETIC_ENABLE_MODULE_024)
  message(STATUS "Module 024 enabled")
  target_compile_definitions(synthetic_module_024
    PUBLIC
      SYNTHETIC_MODULE_024_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_024)
  message(STATUS "Module 024 disabled")
endif(SYNTHETIC_ENABLE_MODULE_024)

add_executable(test_module_024
  tests/modules/mod024/test_0.cpp
  tests/modules/mod024/test_1.cpp
  tests/modules/mod024/test_2.cpp
  tests/modules/mod024/test_3.cpp
  tests/modules/mod024/test_4.cpp
)

target_link_libraries(test_module_024
  PRIVATE
    GTest::gtest_main synthetic_module_024
)
gtest_discover_tests(test_module_024
  TEST_PREFIX
    "mod024::"

  DISCOVERY_TIMEOUT
    60
)

# Module 024 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD024_A   val_a)   # first
set(MOD024_BB  val_bb)  # second
set(MOD024_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 25
# ==========================================================================

add_library(synthetic_module_025
  STATIC
  src/modules/mod025/file_0002.cpp
  src/modules/mod025/file_0009.cpp
  src/modules/mod025/file_0016.cpp
  src/modules/mod025/file_0023.cpp
  src/modules/mod025/file_0025.cpp
  src/modules/mod025/file_0032.cpp
  src/modules/mod025/file_0039.cpp
  src/modules/mod025/file_0046.cpp
  src/modules/mod025/file_0053.cpp
  src/modules/mod025/file_0060.cpp
  src/modules/mod025/file_0067.cpp
  src/modules/mod025/file_0074.cpp
  src/modules/mod025/file_0081.cpp
  src/modules/mod025/file_0088.cpp
  src/modules/mod025/file_0095.cpp
)

set_target_properties(synthetic_module_025 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch25"
  OUTPUT_NAME           "mod025"
)

target_include_directories(synthetic_module_025
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod025>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod025
)

target_compile_definitions(synthetic_module_025
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_025_DEBUG>
    SYNTHETIC_MODULE_025=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_025
)

target_link_libraries(synthetic_module_025
  PUBLIC
    synthetic_module_024

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD025_SETTING_A "value_25_0")
set(MOD025_SETTING_B "value_25_1")
set(MOD025_SETTING_C "value_25_2")
set(MOD025_SETTING_D "value_25_3")
set(MOD025_SETTING_E "value_25_4")

if(SYNTHETIC_ENABLE_MODULE_025)
  message(STATUS "Module 025 enabled")
  target_compile_definitions(synthetic_module_025
    PUBLIC
      SYNTHETIC_MODULE_025_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_025)
  message(STATUS "Module 025 disabled")
endif(SYNTHETIC_ENABLE_MODULE_025)

add_executable(test_module_025
  tests/modules/mod025/test_0.cpp
  tests/modules/mod025/test_1.cpp
  tests/modules/mod025/test_2.cpp
  tests/modules/mod025/test_3.cpp
  tests/modules/mod025/test_4.cpp
)

target_link_libraries(test_module_025
  PRIVATE
    GTest::gtest_main synthetic_module_025
)
gtest_discover_tests(test_module_025
  TEST_PREFIX
    "mod025::"

  DISCOVERY_TIMEOUT
    60
)

# Module 025 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD025_A   val_a)   # first
set(MOD025_BB  val_bb)  # second
set(MOD025_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 26
# ==========================================================================

add_library(synthetic_module_026
  STATIC
  src/modules/mod026/file_0003.cpp
  src/modules/mod026/file_0010.cpp
  src/modules/mod026/file_0017.cpp
  src/modules/mod026/file_0024.cpp
  src/modules/mod026/file_0026.cpp
  src/modules/mod026/file_0033.cpp
  src/modules/mod026/file_0040.cpp
  src/modules/mod026/file_0047.cpp
  src/modules/mod026/file_0054.cpp
  src/modules/mod026/file_0061.cpp
  src/modules/mod026/file_0068.cpp
  src/modules/mod026/file_0075.cpp
  src/modules/mod026/file_0082.cpp
  src/modules/mod026/file_0089.cpp
  src/modules/mod026/file_0096.cpp
)

set_target_properties(synthetic_module_026 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch26"
  OUTPUT_NAME           "mod026"
)

target_include_directories(synthetic_module_026
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod026>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod026
)

target_compile_definitions(synthetic_module_026
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_026_DEBUG>
    SYNTHETIC_MODULE_026=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_026
)

target_link_libraries(synthetic_module_026
  PUBLIC
    synthetic_module_025

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD026_SETTING_A "value_26_0")
set(MOD026_SETTING_B "value_26_1")
set(MOD026_SETTING_C "value_26_2")
set(MOD026_SETTING_D "value_26_3")
set(MOD026_SETTING_E "value_26_4")

if(SYNTHETIC_ENABLE_MODULE_026)
  message(STATUS "Module 026 enabled")
  target_compile_definitions(synthetic_module_026
    PUBLIC
      SYNTHETIC_MODULE_026_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_026)
  message(STATUS "Module 026 disabled")
endif(SYNTHETIC_ENABLE_MODULE_026)

add_executable(test_module_026
  tests/modules/mod026/test_0.cpp
  tests/modules/mod026/test_1.cpp
  tests/modules/mod026/test_2.cpp
  tests/modules/mod026/test_3.cpp
  tests/modules/mod026/test_4.cpp
)

target_link_libraries(test_module_026
  PRIVATE
    GTest::gtest_main synthetic_module_026
)
gtest_discover_tests(test_module_026
  TEST_PREFIX
    "mod026::"

  DISCOVERY_TIMEOUT
    60
)

# Module 026 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD026_A   val_a)   # first
set(MOD026_BB  val_bb)  # second
set(MOD026_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 27
# ==========================================================================

add_library(synthetic_module_027
  STATIC
  src/modules/mod027/file_0004.cpp
  src/modules/mod027/file_0011.cpp
  src/modules/mod027/file_0018.cpp
  src/modules/mod027/file_0025.cpp
  src/modules/mod027/file_0027.cpp
  src/modules/mod027/file_0034.cpp
  src/modules/mod027/file_0041.cpp
  src/modules/mod027/file_0048.cpp
  src/modules/mod027/file_0055.cpp
  src/modules/mod027/file_0062.cpp
  src/modules/mod027/file_0069.cpp
  src/modules/mod027/file_0076.cpp
  src/modules/mod027/file_0083.cpp
  src/modules/mod027/file_0090.cpp
  src/modules/mod027/file_0097.cpp
)

set_target_properties(synthetic_module_027 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch27"
  OUTPUT_NAME           "mod027"
)

target_include_directories(synthetic_module_027
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod027>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod027
)

target_compile_definitions(synthetic_module_027
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_027_DEBUG>
    SYNTHETIC_MODULE_027=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_027
)

target_link_libraries(synthetic_module_027
  PUBLIC
    synthetic_module_026

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD027_SETTING_A "value_27_0")
set(MOD027_SETTING_B "value_27_1")
set(MOD027_SETTING_C "value_27_2")
set(MOD027_SETTING_D "value_27_3")
set(MOD027_SETTING_E "value_27_4")

if(SYNTHETIC_ENABLE_MODULE_027)
  message(STATUS "Module 027 enabled")
  target_compile_definitions(synthetic_module_027
    PUBLIC
      SYNTHETIC_MODULE_027_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_027)
  message(STATUS "Module 027 disabled")
endif(SYNTHETIC_ENABLE_MODULE_027)

add_executable(test_module_027
  tests/modules/mod027/test_0.cpp
  tests/modules/mod027/test_1.cpp
  tests/modules/mod027/test_2.cpp
  tests/modules/mod027/test_3.cpp
  tests/modules/mod027/test_4.cpp
)

target_link_libraries(test_module_027
  PRIVATE
    GTest::gtest_main synthetic_module_027
)
gtest_discover_tests(test_module_027
  TEST_PREFIX
    "mod027::"

  DISCOVERY_TIMEOUT
    60
)

# Module 027 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD027_A   val_a)   # first
set(MOD027_BB  val_bb)  # second
set(MOD027_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 28
# ==========================================================================

add_library(synthetic_module_028
  STATIC
  src/modules/mod028/file_0005.cpp
  src/modules/mod028/file_0012.cpp
  src/modules/mod028/file_0019.cpp
  src/modules/mod028/file_0026.cpp
  src/modules/mod028/file_0028.cpp
  src/modules/mod028/file_0035.cpp
  src/modules/mod028/file_0042.cpp
  src/modules/mod028/file_0049.cpp
  src/modules/mod028/file_0056.cpp
  src/modules/mod028/file_0063.cpp
  src/modules/mod028/file_0070.cpp
  src/modules/mod028/file_0077.cpp
  src/modules/mod028/file_0084.cpp
  src/modules/mod028/file_0091.cpp
  src/modules/mod028/file_0098.cpp
)

set_target_properties(synthetic_module_028 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch28"
  OUTPUT_NAME           "mod028"
)

target_include_directories(synthetic_module_028
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod028>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod028
)

target_compile_definitions(synthetic_module_028
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_028_DEBUG>
    SYNTHETIC_MODULE_028=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_028
)

target_link_libraries(synthetic_module_028
  PUBLIC
    synthetic_module_027

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD028_SETTING_A "value_28_0")
set(MOD028_SETTING_B "value_28_1")
set(MOD028_SETTING_C "value_28_2")
set(MOD028_SETTING_D "value_28_3")
set(MOD028_SETTING_E "value_28_4")

if(SYNTHETIC_ENABLE_MODULE_028)
  message(STATUS "Module 028 enabled")
  target_compile_definitions(synthetic_module_028
    PUBLIC
      SYNTHETIC_MODULE_028_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_028)
  message(STATUS "Module 028 disabled")
endif(SYNTHETIC_ENABLE_MODULE_028)

add_executable(test_module_028
  tests/modules/mod028/test_0.cpp
  tests/modules/mod028/test_1.cpp
  tests/modules/mod028/test_2.cpp
  tests/modules/mod028/test_3.cpp
  tests/modules/mod028/test_4.cpp
)

target_link_libraries(test_module_028
  PRIVATE
    GTest::gtest_main synthetic_module_028
)
gtest_discover_tests(test_module_028
  TEST_PREFIX
    "mod028::"

  DISCOVERY_TIMEOUT
    60
)

# Module 028 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD028_A   val_a)   # first
set(MOD028_BB  val_bb)  # second
set(MOD028_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 29
# ==========================================================================

add_library(synthetic_module_029
  STATIC
  src/modules/mod029/file_0006.cpp
  src/modules/mod029/file_0013.cpp
  src/modules/mod029/file_0020.cpp
  src/modules/mod029/file_0027.cpp
  src/modules/mod029/file_0029.cpp
  src/modules/mod029/file_0036.cpp
  src/modules/mod029/file_0043.cpp
  src/modules/mod029/file_0050.cpp
  src/modules/mod029/file_0057.cpp
  src/modules/mod029/file_0064.cpp
  src/modules/mod029/file_0071.cpp
  src/modules/mod029/file_0078.cpp
  src/modules/mod029/file_0085.cpp
  src/modules/mod029/file_0092.cpp
  src/modules/mod029/file_0099.cpp
)

set_target_properties(synthetic_module_029 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch29"
  OUTPUT_NAME           "mod029"
)

target_include_directories(synthetic_module_029
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod029>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod029
)

target_compile_definitions(synthetic_module_029
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_029_DEBUG>
    SYNTHETIC_MODULE_029=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_029
)

target_link_libraries(synthetic_module_029
  PUBLIC
    synthetic_module_028

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD029_SETTING_A "value_29_0")
set(MOD029_SETTING_B "value_29_1")
set(MOD029_SETTING_C "value_29_2")
set(MOD029_SETTING_D "value_29_3")
set(MOD029_SETTING_E "value_29_4")

if(SYNTHETIC_ENABLE_MODULE_029)
  message(STATUS "Module 029 enabled")
  target_compile_definitions(synthetic_module_029
    PUBLIC
      SYNTHETIC_MODULE_029_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_029)
  message(STATUS "Module 029 disabled")
endif(SYNTHETIC_ENABLE_MODULE_029)

add_executable(test_module_029
  tests/modules/mod029/test_0.cpp
  tests/modules/mod029/test_1.cpp
  tests/modules/mod029/test_2.cpp
  tests/modules/mod029/test_3.cpp
  tests/modules/mod029/test_4.cpp
)

target_link_libraries(test_module_029
  PRIVATE
    GTest::gtest_main synthetic_module_029
)
gtest_discover_tests(test_module_029
  TEST_PREFIX
    "mod029::"

  DISCOVERY_TIMEOUT
    60
)

# Module 029 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD029_A   val_a)   # first
set(MOD029_BB  val_bb)  # second
set(MOD029_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 30
# ==========================================================================

add_library(synthetic_module_030
  STATIC
  src/modules/mod030/file_0000.cpp
  src/modules/mod030/file_0007.cpp
  src/modules/mod030/file_0014.cpp
  src/modules/mod030/file_0021.cpp
  src/modules/mod030/file_0028.cpp
  src/modules/mod030/file_0030.cpp
  src/modules/mod030/file_0037.cpp
  src/modules/mod030/file_0044.cpp
  src/modules/mod030/file_0051.cpp
  src/modules/mod030/file_0058.cpp
  src/modules/mod030/file_0065.cpp
  src/modules/mod030/file_0072.cpp
  src/modules/mod030/file_0079.cpp
  src/modules/mod030/file_0086.cpp
  src/modules/mod030/file_0093.cpp
)

set_target_properties(synthetic_module_030 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch30"
  OUTPUT_NAME           "mod030"
)

target_include_directories(synthetic_module_030
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod030>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod030
)

target_compile_definitions(synthetic_module_030
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_030_DEBUG>
    SYNTHETIC_MODULE_030=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_030
)

target_link_libraries(synthetic_module_030
  PUBLIC
    synthetic_module_029

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD030_SETTING_A "value_30_0")
set(MOD030_SETTING_B "value_30_1")
set(MOD030_SETTING_C "value_30_2")
set(MOD030_SETTING_D "value_30_3")
set(MOD030_SETTING_E "value_30_4")

if(SYNTHETIC_ENABLE_MODULE_030)
  message(STATUS "Module 030 enabled")
  target_compile_definitions(synthetic_module_030
    PUBLIC
      SYNTHETIC_MODULE_030_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_030)
  message(STATUS "Module 030 disabled")
endif(SYNTHETIC_ENABLE_MODULE_030)

add_executable(test_module_030
  tests/modules/mod030/test_0.cpp
  tests/modules/mod030/test_1.cpp
  tests/modules/mod030/test_2.cpp
  tests/modules/mod030/test_3.cpp
  tests/modules/mod030/test_4.cpp
)

target_link_libraries(test_module_030
  PRIVATE
    GTest::gtest_main synthetic_module_030
)
gtest_discover_tests(test_module_030
  TEST_PREFIX
    "mod030::"

  DISCOVERY_TIMEOUT
    60
)

# Module 030 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD030_A   val_a)   # first
set(MOD030_BB  val_bb)  # second
set(MOD030_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 31
# ==========================================================================

add_library(synthetic_module_031
  STATIC
  src/modules/mod031/file_0001.cpp
  src/modules/mod031/file_0008.cpp
  src/modules/mod031/file_0015.cpp
  src/modules/mod031/file_0022.cpp
  src/modules/mod031/file_0029.cpp
  src/modules/mod031/file_0031.cpp
  src/modules/mod031/file_0038.cpp
  src/modules/mod031/file_0045.cpp
  src/modules/mod031/file_0052.cpp
  src/modules/mod031/file_0059.cpp
  src/modules/mod031/file_0066.cpp
  src/modules/mod031/file_0073.cpp
  src/modules/mod031/file_0080.cpp
  src/modules/mod031/file_0087.cpp
  src/modules/mod031/file_0094.cpp
)

set_target_properties(synthetic_module_031 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch31"
  OUTPUT_NAME           "mod031"
)

target_include_directories(synthetic_module_031
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod031>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod031
)

target_compile_definitions(synthetic_module_031
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_031_DEBUG>
    SYNTHETIC_MODULE_031=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_031
)

target_link_libraries(synthetic_module_031
  PUBLIC
    synthetic_module_030

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD031_SETTING_A "value_31_0")
set(MOD031_SETTING_B "value_31_1")
set(MOD031_SETTING_C "value_31_2")
set(MOD031_SETTING_D "value_31_3")
set(MOD031_SETTING_E "value_31_4")

if(SYNTHETIC_ENABLE_MODULE_031)
  message(STATUS "Module 031 enabled")
  target_compile_definitions(synthetic_module_031
    PUBLIC
      SYNTHETIC_MODULE_031_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_031)
  message(STATUS "Module 031 disabled")
endif(SYNTHETIC_ENABLE_MODULE_031)

add_executable(test_module_031
  tests/modules/mod031/test_0.cpp
  tests/modules/mod031/test_1.cpp
  tests/modules/mod031/test_2.cpp
  tests/modules/mod031/test_3.cpp
  tests/modules/mod031/test_4.cpp
)

target_link_libraries(test_module_031
  PRIVATE
    GTest::gtest_main synthetic_module_031
)
gtest_discover_tests(test_module_031
  TEST_PREFIX
    "mod031::"

  DISCOVERY_TIMEOUT
    60
)

# Module 031 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD031_A   val_a)   # first
set(MOD031_BB  val_bb)  # second
set(MOD031_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 32
# ==========================================================================

add_library(synthetic_module_032
  STATIC
  src/modules/mod032/file_0002.cpp
  src/modules/mod032/file_0009.cpp
  src/modules/mod032/file_0016.cpp
  src/modules/mod032/file_0023.cpp
  src/modules/mod032/file_0030.cpp
  src/modules/mod032/file_0032.cpp
  src/modules/mod032/file_0039.cpp
  src/modules/mod032/file_0046.cpp
  src/modules/mod032/file_0053.cpp
  src/modules/mod032/file_0060.cpp
  src/modules/mod032/file_0067.cpp
  src/modules/mod032/file_0074.cpp
  src/modules/mod032/file_0081.cpp
  src/modules/mod032/file_0088.cpp
  src/modules/mod032/file_0095.cpp
)

set_target_properties(synthetic_module_032 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch32"
  OUTPUT_NAME           "mod032"
)

target_include_directories(synthetic_module_032
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod032>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod032
)

target_compile_definitions(synthetic_module_032
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_032_DEBUG>
    SYNTHETIC_MODULE_032=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_032
)

target_link_libraries(synthetic_module_032
  PUBLIC
    synthetic_module_031

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD032_SETTING_A "value_32_0")
set(MOD032_SETTING_B "value_32_1")
set(MOD032_SETTING_C "value_32_2")
set(MOD032_SETTING_D "value_32_3")
set(MOD032_SETTING_E "value_32_4")

if(SYNTHETIC_ENABLE_MODULE_032)
  message(STATUS "Module 032 enabled")
  target_compile_definitions(synthetic_module_032
    PUBLIC
      SYNTHETIC_MODULE_032_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_032)
  message(STATUS "Module 032 disabled")
endif(SYNTHETIC_ENABLE_MODULE_032)

add_executable(test_module_032
  tests/modules/mod032/test_0.cpp
  tests/modules/mod032/test_1.cpp
  tests/modules/mod032/test_2.cpp
  tests/modules/mod032/test_3.cpp
  tests/modules/mod032/test_4.cpp
)

target_link_libraries(test_module_032
  PRIVATE
    GTest::gtest_main synthetic_module_032
)
gtest_discover_tests(test_module_032
  TEST_PREFIX
    "mod032::"

  DISCOVERY_TIMEOUT
    60
)

# Module 032 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD032_A   val_a)   # first
set(MOD032_BB  val_bb)  # second
set(MOD032_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 33
# ==========================================================================

add_library(synthetic_module_033
  STATIC
  src/modules/mod033/file_0003.cpp
  src/modules/mod033/file_0010.cpp
  src/modules/mod033/file_0017.cpp
  src/modules/mod033/file_0024.cpp
  src/modules/mod033/file_0031.cpp
  src/modules/mod033/file_0033.cpp
  src/modules/mod033/file_0040.cpp
  src/modules/mod033/file_0047.cpp
  src/modules/mod033/file_0054.cpp
  src/modules/mod033/file_0061.cpp
  src/modules/mod033/file_0068.cpp
  src/modules/mod033/file_0075.cpp
  src/modules/mod033/file_0082.cpp
  src/modules/mod033/file_0089.cpp
  src/modules/mod033/file_0096.cpp
)

set_target_properties(synthetic_module_033 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch33"
  OUTPUT_NAME           "mod033"
)

target_include_directories(synthetic_module_033
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod033>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod033
)

target_compile_definitions(synthetic_module_033
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_033_DEBUG>
    SYNTHETIC_MODULE_033=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_033
)

target_link_libraries(synthetic_module_033
  PUBLIC
    synthetic_module_032

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD033_SETTING_A "value_33_0")
set(MOD033_SETTING_B "value_33_1")
set(MOD033_SETTING_C "value_33_2")
set(MOD033_SETTING_D "value_33_3")
set(MOD033_SETTING_E "value_33_4")

if(SYNTHETIC_ENABLE_MODULE_033)
  message(STATUS "Module 033 enabled")
  target_compile_definitions(synthetic_module_033
    PUBLIC
      SYNTHETIC_MODULE_033_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_033)
  message(STATUS "Module 033 disabled")
endif(SYNTHETIC_ENABLE_MODULE_033)

add_executable(test_module_033
  tests/modules/mod033/test_0.cpp
  tests/modules/mod033/test_1.cpp
  tests/modules/mod033/test_2.cpp
  tests/modules/mod033/test_3.cpp
  tests/modules/mod033/test_4.cpp
)

target_link_libraries(test_module_033
  PRIVATE
    GTest::gtest_main synthetic_module_033
)
gtest_discover_tests(test_module_033
  TEST_PREFIX
    "mod033::"

  DISCOVERY_TIMEOUT
    60
)

# Module 033 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD033_A   val_a)   # first
set(MOD033_BB  val_bb)  # second
set(MOD033_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 34
# ==========================================================================

add_library(synthetic_module_034
  STATIC
  src/modules/mod034/file_0004.cpp
  src/modules/mod034/file_0011.cpp
  src/modules/mod034/file_0018.cpp
  src/modules/mod034/file_0025.cpp
  src/modules/mod034/file_0032.cpp
  src/modules/mod034/file_0034.cpp
  src/modules/mod034/file_0041.cpp
  src/modules/mod034/file_0048.cpp
  src/modules/mod034/file_0055.cpp
  src/modules/mod034/file_0062.cpp
  src/modules/mod034/file_0069.cpp
  src/modules/mod034/file_0076.cpp
  src/modules/mod034/file_0083.cpp
  src/modules/mod034/file_0090.cpp
  src/modules/mod034/file_0097.cpp
)

set_target_properties(synthetic_module_034 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch34"
  OUTPUT_NAME           "mod034"
)

target_include_directories(synthetic_module_034
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod034>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod034
)

target_compile_definitions(synthetic_module_034
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_034_DEBUG>
    SYNTHETIC_MODULE_034=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_034
)

target_link_libraries(synthetic_module_034
  PUBLIC
    synthetic_module_033

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD034_SETTING_A "value_34_0")
set(MOD034_SETTING_B "value_34_1")
set(MOD034_SETTING_C "value_34_2")
set(MOD034_SETTING_D "value_34_3")
set(MOD034_SETTING_E "value_34_4")

if(SYNTHETIC_ENABLE_MODULE_034)
  message(STATUS "Module 034 enabled")
  target_compile_definitions(synthetic_module_034
    PUBLIC
      SYNTHETIC_MODULE_034_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_034)
  message(STATUS "Module 034 disabled")
endif(SYNTHETIC_ENABLE_MODULE_034)

add_executable(test_module_034
  tests/modules/mod034/test_0.cpp
  tests/modules/mod034/test_1.cpp
  tests/modules/mod034/test_2.cpp
  tests/modules/mod034/test_3.cpp
  tests/modules/mod034/test_4.cpp
)

target_link_libraries(test_module_034
  PRIVATE
    GTest::gtest_main synthetic_module_034
)
gtest_discover_tests(test_module_034
  TEST_PREFIX
    "mod034::"

  DISCOVERY_TIMEOUT
    60
)

# Module 034 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD034_A   val_a)   # first
set(MOD034_BB  val_bb)  # second
set(MOD034_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 35
# ==========================================================================

add_library(synthetic_module_035
  STATIC
  src/modules/mod035/file_0005.cpp
  src/modules/mod035/file_0012.cpp
  src/modules/mod035/file_0019.cpp
  src/modules/mod035/file_0026.cpp
  src/modules/mod035/file_0033.cpp
  src/modules/mod035/file_0035.cpp
  src/modules/mod035/file_0042.cpp
  src/modules/mod035/file_0049.cpp
  src/modules/mod035/file_0056.cpp
  src/modules/mod035/file_0063.cpp
  src/modules/mod035/file_0070.cpp
  src/modules/mod035/file_0077.cpp
  src/modules/mod035/file_0084.cpp
  src/modules/mod035/file_0091.cpp
  src/modules/mod035/file_0098.cpp
)

set_target_properties(synthetic_module_035 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch35"
  OUTPUT_NAME           "mod035"
)

target_include_directories(synthetic_module_035
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod035>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod035
)

target_compile_definitions(synthetic_module_035
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_035_DEBUG>
    SYNTHETIC_MODULE_035=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_035
)

target_link_libraries(synthetic_module_035
  PUBLIC
    synthetic_module_034

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD035_SETTING_A "value_35_0")
set(MOD035_SETTING_B "value_35_1")
set(MOD035_SETTING_C "value_35_2")
set(MOD035_SETTING_D "value_35_3")
set(MOD035_SETTING_E "value_35_4")

if(SYNTHETIC_ENABLE_MODULE_035)
  message(STATUS "Module 035 enabled")
  target_compile_definitions(synthetic_module_035
    PUBLIC
      SYNTHETIC_MODULE_035_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_035)
  message(STATUS "Module 035 disabled")
endif(SYNTHETIC_ENABLE_MODULE_035)

add_executable(test_module_035
  tests/modules/mod035/test_0.cpp
  tests/modules/mod035/test_1.cpp
  tests/modules/mod035/test_2.cpp
  tests/modules/mod035/test_3.cpp
  tests/modules/mod035/test_4.cpp
)

target_link_libraries(test_module_035
  PRIVATE
    GTest::gtest_main synthetic_module_035
)
gtest_discover_tests(test_module_035
  TEST_PREFIX
    "mod035::"

  DISCOVERY_TIMEOUT
    60
)

# Module 035 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD035_A   val_a)   # first
set(MOD035_BB  val_bb)  # second
set(MOD035_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 36
# ==========================================================================

add_library(synthetic_module_036
  STATIC
  src/modules/mod036/file_0006.cpp
  src/modules/mod036/file_0013.cpp
  src/modules/mod036/file_0020.cpp
  src/modules/mod036/file_0027.cpp
  src/modules/mod036/file_0034.cpp
  src/modules/mod036/file_0036.cpp
  src/modules/mod036/file_0043.cpp
  src/modules/mod036/file_0050.cpp
  src/modules/mod036/file_0057.cpp
  src/modules/mod036/file_0064.cpp
  src/modules/mod036/file_0071.cpp
  src/modules/mod036/file_0078.cpp
  src/modules/mod036/file_0085.cpp
  src/modules/mod036/file_0092.cpp
  src/modules/mod036/file_0099.cpp
)

set_target_properties(synthetic_module_036 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch36"
  OUTPUT_NAME           "mod036"
)

target_include_directories(synthetic_module_036
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod036>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod036
)

target_compile_definitions(synthetic_module_036
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_036_DEBUG>
    SYNTHETIC_MODULE_036=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_036
)

target_link_libraries(synthetic_module_036
  PUBLIC
    synthetic_module_035

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD036_SETTING_A "value_36_0")
set(MOD036_SETTING_B "value_36_1")
set(MOD036_SETTING_C "value_36_2")
set(MOD036_SETTING_D "value_36_3")
set(MOD036_SETTING_E "value_36_4")

if(SYNTHETIC_ENABLE_MODULE_036)
  message(STATUS "Module 036 enabled")
  target_compile_definitions(synthetic_module_036
    PUBLIC
      SYNTHETIC_MODULE_036_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_036)
  message(STATUS "Module 036 disabled")
endif(SYNTHETIC_ENABLE_MODULE_036)

add_executable(test_module_036
  tests/modules/mod036/test_0.cpp
  tests/modules/mod036/test_1.cpp
  tests/modules/mod036/test_2.cpp
  tests/modules/mod036/test_3.cpp
  tests/modules/mod036/test_4.cpp
)

target_link_libraries(test_module_036
  PRIVATE
    GTest::gtest_main synthetic_module_036
)
gtest_discover_tests(test_module_036
  TEST_PREFIX
    "mod036::"

  DISCOVERY_TIMEOUT
    60
)

# Module 036 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD036_A   val_a)   # first
set(MOD036_BB  val_bb)  # second
set(MOD036_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 37
# ==========================================================================

add_library(synthetic_module_037
  STATIC
  src/modules/mod037/file_0000.cpp
  src/modules/mod037/file_0007.cpp
  src/modules/mod037/file_0014.cpp
  src/modules/mod037/file_0021.cpp
  src/modules/mod037/file_0028.cpp
  src/modules/mod037/file_0035.cpp
  src/modules/mod037/file_0037.cpp
  src/modules/mod037/file_0044.cpp
  src/modules/mod037/file_0051.cpp
  src/modules/mod037/file_0058.cpp
  src/modules/mod037/file_0065.cpp
  src/modules/mod037/file_0072.cpp
  src/modules/mod037/file_0079.cpp
  src/modules/mod037/file_0086.cpp
  src/modules/mod037/file_0093.cpp
)

set_target_properties(synthetic_module_037 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch37"
  OUTPUT_NAME           "mod037"
)

target_include_directories(synthetic_module_037
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod037>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod037
)

target_compile_definitions(synthetic_module_037
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_037_DEBUG>
    SYNTHETIC_MODULE_037=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_037
)

target_link_libraries(synthetic_module_037
  PUBLIC
    synthetic_module_036

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD037_SETTING_A "value_37_0")
set(MOD037_SETTING_B "value_37_1")
set(MOD037_SETTING_C "value_37_2")
set(MOD037_SETTING_D "value_37_3")
set(MOD037_SETTING_E "value_37_4")

if(SYNTHETIC_ENABLE_MODULE_037)
  message(STATUS "Module 037 enabled")
  target_compile_definitions(synthetic_module_037
    PUBLIC
      SYNTHETIC_MODULE_037_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_037)
  message(STATUS "Module 037 disabled")
endif(SYNTHETIC_ENABLE_MODULE_037)

add_executable(test_module_037
  tests/modules/mod037/test_0.cpp
  tests/modules/mod037/test_1.cpp
  tests/modules/mod037/test_2.cpp
  tests/modules/mod037/test_3.cpp
  tests/modules/mod037/test_4.cpp
)

target_link_libraries(test_module_037
  PRIVATE
    GTest::gtest_main synthetic_module_037
)
gtest_discover_tests(test_module_037
  TEST_PREFIX
    "mod037::"

  DISCOVERY_TIMEOUT
    60
)

# Module 037 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD037_A   val_a)   # first
set(MOD037_BB  val_bb)  # second
set(MOD037_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 38
# ==========================================================================

add_library(synthetic_module_038
  STATIC
  src/modules/mod038/file_0001.cpp
  src/modules/mod038/file_0008.cpp
  src/modules/mod038/file_0015.cpp
  src/modules/mod038/file_0022.cpp
  src/modules/mod038/file_0029.cpp
  src/modules/mod038/file_0036.cpp
  src/modules/mod038/file_0038.cpp
  src/modules/mod038/file_0045.cpp
  src/modules/mod038/file_0052.cpp
  src/modules/mod038/file_0059.cpp
  src/modules/mod038/file_0066.cpp
  src/modules/mod038/file_0073.cpp
  src/modules/mod038/file_0080.cpp
  src/modules/mod038/file_0087.cpp
  src/modules/mod038/file_0094.cpp
)

set_target_properties(synthetic_module_038 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch38"
  OUTPUT_NAME           "mod038"
)

target_include_directories(synthetic_module_038
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod038>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod038
)

target_compile_definitions(synthetic_module_038
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_038_DEBUG>
    SYNTHETIC_MODULE_038=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_038
)

target_link_libraries(synthetic_module_038
  PUBLIC
    synthetic_module_037

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD038_SETTING_A "value_38_0")
set(MOD038_SETTING_B "value_38_1")
set(MOD038_SETTING_C "value_38_2")
set(MOD038_SETTING_D "value_38_3")
set(MOD038_SETTING_E "value_38_4")

if(SYNTHETIC_ENABLE_MODULE_038)
  message(STATUS "Module 038 enabled")
  target_compile_definitions(synthetic_module_038
    PUBLIC
      SYNTHETIC_MODULE_038_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_038)
  message(STATUS "Module 038 disabled")
endif(SYNTHETIC_ENABLE_MODULE_038)

add_executable(test_module_038
  tests/modules/mod038/test_0.cpp
  tests/modules/mod038/test_1.cpp
  tests/modules/mod038/test_2.cpp
  tests/modules/mod038/test_3.cpp
  tests/modules/mod038/test_4.cpp
)

target_link_libraries(test_module_038
  PRIVATE
    GTest::gtest_main synthetic_module_038
)
gtest_discover_tests(test_module_038
  TEST_PREFIX
    "mod038::"

  DISCOVERY_TIMEOUT
    60
)

# Module 038 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD038_A   val_a)   # first
set(MOD038_BB  val_bb)  # second
set(MOD038_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 39
# ==========================================================================

add_library(synthetic_module_039
  STATIC
  src/modules/mod039/file_0002.cpp
  src/modules/mod039/file_0009.cpp
  src/modules/mod039/file_0016.cpp
  src/modules/mod039/file_0023.cpp
  src/modules/mod039/file_0030.cpp
  src/modules/mod039/file_0037.cpp
  src/modules/mod039/file_0039.cpp
  src/modules/mod039/file_0046.cpp
  src/modules/mod039/file_0053.cpp
  src/modules/mod039/file_0060.cpp
  src/modules/mod039/file_0067.cpp
  src/modules/mod039/file_0074.cpp
  src/modules/mod039/file_0081.cpp
  src/modules/mod039/file_0088.cpp
  src/modules/mod039/file_0095.cpp
)

set_target_properties(synthetic_module_039 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch39"
  OUTPUT_NAME           "mod039"
)

target_include_directories(synthetic_module_039
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod039>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod039
)

target_compile_definitions(synthetic_module_039
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_039_DEBUG>
    SYNTHETIC_MODULE_039=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_039
)

target_link_libraries(synthetic_module_039
  PUBLIC
    synthetic_module_038

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD039_SETTING_A "value_39_0")
set(MOD039_SETTING_B "value_39_1")
set(MOD039_SETTING_C "value_39_2")
set(MOD039_SETTING_D "value_39_3")
set(MOD039_SETTING_E "value_39_4")

if(SYNTHETIC_ENABLE_MODULE_039)
  message(STATUS "Module 039 enabled")
  target_compile_definitions(synthetic_module_039
    PUBLIC
      SYNTHETIC_MODULE_039_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_039)
  message(STATUS "Module 039 disabled")
endif(SYNTHETIC_ENABLE_MODULE_039)

add_executable(test_module_039
  tests/modules/mod039/test_0.cpp
  tests/modules/mod039/test_1.cpp
  tests/modules/mod039/test_2.cpp
  tests/modules/mod039/test_3.cpp
  tests/modules/mod039/test_4.cpp
)

target_link_libraries(test_module_039
  PRIVATE
    GTest::gtest_main synthetic_module_039
)
gtest_discover_tests(test_module_039
  TEST_PREFIX
    "mod039::"

  DISCOVERY_TIMEOUT
    60
)

# Module 039 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD039_A   val_a)   # first
set(MOD039_BB  val_bb)  # second
set(MOD039_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 40
# ==========================================================================

add_library(synthetic_module_040
  STATIC
  src/modules/mod040/file_0003.cpp
  src/modules/mod040/file_0010.cpp
  src/modules/mod040/file_0017.cpp
  src/modules/mod040/file_0024.cpp
  src/modules/mod040/file_0031.cpp
  src/modules/mod040/file_0038.cpp
  src/modules/mod040/file_0040.cpp
  src/modules/mod040/file_0047.cpp
  src/modules/mod040/file_0054.cpp
  src/modules/mod040/file_0061.cpp
  src/modules/mod040/file_0068.cpp
  src/modules/mod040/file_0075.cpp
  src/modules/mod040/file_0082.cpp
  src/modules/mod040/file_0089.cpp
  src/modules/mod040/file_0096.cpp
)

set_target_properties(synthetic_module_040 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch40"
  OUTPUT_NAME           "mod040"
)

target_include_directories(synthetic_module_040
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod040>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod040
)

target_compile_definitions(synthetic_module_040
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_040_DEBUG>
    SYNTHETIC_MODULE_040=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_040
)

target_link_libraries(synthetic_module_040
  PUBLIC
    synthetic_module_039

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD040_SETTING_A "value_40_0")
set(MOD040_SETTING_B "value_40_1")
set(MOD040_SETTING_C "value_40_2")
set(MOD040_SETTING_D "value_40_3")
set(MOD040_SETTING_E "value_40_4")

if(SYNTHETIC_ENABLE_MODULE_040)
  message(STATUS "Module 040 enabled")
  target_compile_definitions(synthetic_module_040
    PUBLIC
      SYNTHETIC_MODULE_040_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_040)
  message(STATUS "Module 040 disabled")
endif(SYNTHETIC_ENABLE_MODULE_040)

add_executable(test_module_040
  tests/modules/mod040/test_0.cpp
  tests/modules/mod040/test_1.cpp
  tests/modules/mod040/test_2.cpp
  tests/modules/mod040/test_3.cpp
  tests/modules/mod040/test_4.cpp
)

target_link_libraries(test_module_040
  PRIVATE
    GTest::gtest_main synthetic_module_040
)
gtest_discover_tests(test_module_040
  TEST_PREFIX
    "mod040::"

  DISCOVERY_TIMEOUT
    60
)

# Module 040 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD040_A   val_a)   # first
set(MOD040_BB  val_bb)  # second
set(MOD040_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 41
# ==========================================================================

add_library(synthetic_module_041
  STATIC
  src/modules/mod041/file_0004.cpp
  src/modules/mod041/file_0011.cpp
  src/modules/mod041/file_0018.cpp
  src/modules/mod041/file_0025.cpp
  src/modules/mod041/file_0032.cpp
  src/modules/mod041/file_0039.cpp
  src/modules/mod041/file_0041.cpp
  src/modules/mod041/file_0048.cpp
  src/modules/mod041/file_0055.cpp
  src/modules/mod041/file_0062.cpp
  src/modules/mod041/file_0069.cpp
  src/modules/mod041/file_0076.cpp
  src/modules/mod041/file_0083.cpp
  src/modules/mod041/file_0090.cpp
  src/modules/mod041/file_0097.cpp
)

set_target_properties(synthetic_module_041 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch41"
  OUTPUT_NAME           "mod041"
)

target_include_directories(synthetic_module_041
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod041>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod041
)

target_compile_definitions(synthetic_module_041
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_041_DEBUG>
    SYNTHETIC_MODULE_041=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_041
)

target_link_libraries(synthetic_module_041
  PUBLIC
    synthetic_module_040

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD041_SETTING_A "value_41_0")
set(MOD041_SETTING_B "value_41_1")
set(MOD041_SETTING_C "value_41_2")
set(MOD041_SETTING_D "value_41_3")
set(MOD041_SETTING_E "value_41_4")

if(SYNTHETIC_ENABLE_MODULE_041)
  message(STATUS "Module 041 enabled")
  target_compile_definitions(synthetic_module_041
    PUBLIC
      SYNTHETIC_MODULE_041_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_041)
  message(STATUS "Module 041 disabled")
endif(SYNTHETIC_ENABLE_MODULE_041)

add_executable(test_module_041
  tests/modules/mod041/test_0.cpp
  tests/modules/mod041/test_1.cpp
  tests/modules/mod041/test_2.cpp
  tests/modules/mod041/test_3.cpp
  tests/modules/mod041/test_4.cpp
)

target_link_libraries(test_module_041
  PRIVATE
    GTest::gtest_main synthetic_module_041
)
gtest_discover_tests(test_module_041
  TEST_PREFIX
    "mod041::"

  DISCOVERY_TIMEOUT
    60
)

# Module 041 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD041_A   val_a)   # first
set(MOD041_BB  val_bb)  # second
set(MOD041_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 42
# ==========================================================================

add_library(synthetic_module_042
  STATIC
  src/modules/mod042/file_0005.cpp
  src/modules/mod042/file_0012.cpp
  src/modules/mod042/file_0019.cpp
  src/modules/mod042/file_0026.cpp
  src/modules/mod042/file_0033.cpp
  src/modules/mod042/file_0040.cpp
  src/modules/mod042/file_0042.cpp
  src/modules/mod042/file_0049.cpp
  src/modules/mod042/file_0056.cpp
  src/modules/mod042/file_0063.cpp
  src/modules/mod042/file_0070.cpp
  src/modules/mod042/file_0077.cpp
  src/modules/mod042/file_0084.cpp
  src/modules/mod042/file_0091.cpp
  src/modules/mod042/file_0098.cpp
)

set_target_properties(synthetic_module_042 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch42"
  OUTPUT_NAME           "mod042"
)

target_include_directories(synthetic_module_042
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod042>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod042
)

target_compile_definitions(synthetic_module_042
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_042_DEBUG>
    SYNTHETIC_MODULE_042=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_042
)

target_link_libraries(synthetic_module_042
  PUBLIC
    synthetic_module_041

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD042_SETTING_A "value_42_0")
set(MOD042_SETTING_B "value_42_1")
set(MOD042_SETTING_C "value_42_2")
set(MOD042_SETTING_D "value_42_3")
set(MOD042_SETTING_E "value_42_4")

if(SYNTHETIC_ENABLE_MODULE_042)
  message(STATUS "Module 042 enabled")
  target_compile_definitions(synthetic_module_042
    PUBLIC
      SYNTHETIC_MODULE_042_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_042)
  message(STATUS "Module 042 disabled")
endif(SYNTHETIC_ENABLE_MODULE_042)

add_executable(test_module_042
  tests/modules/mod042/test_0.cpp
  tests/modules/mod042/test_1.cpp
  tests/modules/mod042/test_2.cpp
  tests/modules/mod042/test_3.cpp
  tests/modules/mod042/test_4.cpp
)

target_link_libraries(test_module_042
  PRIVATE
    GTest::gtest_main synthetic_module_042
)
gtest_discover_tests(test_module_042
  TEST_PREFIX
    "mod042::"

  DISCOVERY_TIMEOUT
    60
)

# Module 042 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD042_A   val_a)   # first
set(MOD042_BB  val_bb)  # second
set(MOD042_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 43
# ==========================================================================

add_library(synthetic_module_043
  STATIC
  src/modules/mod043/file_0006.cpp
  src/modules/mod043/file_0013.cpp
  src/modules/mod043/file_0020.cpp
  src/modules/mod043/file_0027.cpp
  src/modules/mod043/file_0034.cpp
  src/modules/mod043/file_0041.cpp
  src/modules/mod043/file_0043.cpp
  src/modules/mod043/file_0050.cpp
  src/modules/mod043/file_0057.cpp
  src/modules/mod043/file_0064.cpp
  src/modules/mod043/file_0071.cpp
  src/modules/mod043/file_0078.cpp
  src/modules/mod043/file_0085.cpp
  src/modules/mod043/file_0092.cpp
  src/modules/mod043/file_0099.cpp
)

set_target_properties(synthetic_module_043 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch43"
  OUTPUT_NAME           "mod043"
)

target_include_directories(synthetic_module_043
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod043>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod043
)

target_compile_definitions(synthetic_module_043
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_043_DEBUG>
    SYNTHETIC_MODULE_043=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_043
)

target_link_libraries(synthetic_module_043
  PUBLIC
    synthetic_module_042

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD043_SETTING_A "value_43_0")
set(MOD043_SETTING_B "value_43_1")
set(MOD043_SETTING_C "value_43_2")
set(MOD043_SETTING_D "value_43_3")
set(MOD043_SETTING_E "value_43_4")

if(SYNTHETIC_ENABLE_MODULE_043)
  message(STATUS "Module 043 enabled")
  target_compile_definitions(synthetic_module_043
    PUBLIC
      SYNTHETIC_MODULE_043_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_043)
  message(STATUS "Module 043 disabled")
endif(SYNTHETIC_ENABLE_MODULE_043)

add_executable(test_module_043
  tests/modules/mod043/test_0.cpp
  tests/modules/mod043/test_1.cpp
  tests/modules/mod043/test_2.cpp
  tests/modules/mod043/test_3.cpp
  tests/modules/mod043/test_4.cpp
)

target_link_libraries(test_module_043
  PRIVATE
    GTest::gtest_main synthetic_module_043
)
gtest_discover_tests(test_module_043
  TEST_PREFIX
    "mod043::"

  DISCOVERY_TIMEOUT
    60
)

# Module 043 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD043_A   val_a)   # first
set(MOD043_BB  val_bb)  # second
set(MOD043_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 44
# ==========================================================================

add_library(synthetic_module_044
  STATIC
  src/modules/mod044/file_0000.cpp
  src/modules/mod044/file_0007.cpp
  src/modules/mod044/file_0014.cpp
  src/modules/mod044/file_0021.cpp
  src/modules/mod044/file_0028.cpp
  src/modules/mod044/file_0035.cpp
  src/modules/mod044/file_0042.cpp
  src/modules/mod044/file_0044.cpp
  src/modules/mod044/file_0051.cpp
  src/modules/mod044/file_0058.cpp
  src/modules/mod044/file_0065.cpp
  src/modules/mod044/file_0072.cpp
  src/modules/mod044/file_0079.cpp
  src/modules/mod044/file_0086.cpp
  src/modules/mod044/file_0093.cpp
)

set_target_properties(synthetic_module_044 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch44"
  OUTPUT_NAME           "mod044"
)

target_include_directories(synthetic_module_044
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod044>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod044
)

target_compile_definitions(synthetic_module_044
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_044_DEBUG>
    SYNTHETIC_MODULE_044=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_044
)

target_link_libraries(synthetic_module_044
  PUBLIC
    synthetic_module_043

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD044_SETTING_A "value_44_0")
set(MOD044_SETTING_B "value_44_1")
set(MOD044_SETTING_C "value_44_2")
set(MOD044_SETTING_D "value_44_3")
set(MOD044_SETTING_E "value_44_4")

if(SYNTHETIC_ENABLE_MODULE_044)
  message(STATUS "Module 044 enabled")
  target_compile_definitions(synthetic_module_044
    PUBLIC
      SYNTHETIC_MODULE_044_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_044)
  message(STATUS "Module 044 disabled")
endif(SYNTHETIC_ENABLE_MODULE_044)

add_executable(test_module_044
  tests/modules/mod044/test_0.cpp
  tests/modules/mod044/test_1.cpp
  tests/modules/mod044/test_2.cpp
  tests/modules/mod044/test_3.cpp
  tests/modules/mod044/test_4.cpp
)

target_link_libraries(test_module_044
  PRIVATE
    GTest::gtest_main synthetic_module_044
)
gtest_discover_tests(test_module_044
  TEST_PREFIX
    "mod044::"

  DISCOVERY_TIMEOUT
    60
)

# Module 044 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD044_A   val_a)   # first
set(MOD044_BB  val_bb)  # second
set(MOD044_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 45
# ==========================================================================

add_library(synthetic_module_045
  STATIC
  src/modules/mod045/file_0001.cpp
  src/modules/mod045/file_0008.cpp
  src/modules/mod045/file_0015.cpp
  src/modules/mod045/file_0022.cpp
  src/modules/mod045/file_0029.cpp
  src/modules/mod045/file_0036.cpp
  src/modules/mod045/file_0043.cpp
  src/modules/mod045/file_0045.cpp
  src/modules/mod045/file_0052.cpp
  src/modules/mod045/file_0059.cpp
  src/modules/mod045/file_0066.cpp
  src/modules/mod045/file_0073.cpp
  src/modules/mod045/file_0080.cpp
  src/modules/mod045/file_0087.cpp
  src/modules/mod045/file_0094.cpp
)

set_target_properties(synthetic_module_045 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch45"
  OUTPUT_NAME           "mod045"
)

target_include_directories(synthetic_module_045
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod045>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod045
)

target_compile_definitions(synthetic_module_045
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_045_DEBUG>
    SYNTHETIC_MODULE_045=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_045
)

target_link_libraries(synthetic_module_045
  PUBLIC
    synthetic_module_044

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD045_SETTING_A "value_45_0")
set(MOD045_SETTING_B "value_45_1")
set(MOD045_SETTING_C "value_45_2")
set(MOD045_SETTING_D "value_45_3")
set(MOD045_SETTING_E "value_45_4")

if(SYNTHETIC_ENABLE_MODULE_045)
  message(STATUS "Module 045 enabled")
  target_compile_definitions(synthetic_module_045
    PUBLIC
      SYNTHETIC_MODULE_045_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_045)
  message(STATUS "Module 045 disabled")
endif(SYNTHETIC_ENABLE_MODULE_045)

add_executable(test_module_045
  tests/modules/mod045/test_0.cpp
  tests/modules/mod045/test_1.cpp
  tests/modules/mod045/test_2.cpp
  tests/modules/mod045/test_3.cpp
  tests/modules/mod045/test_4.cpp
)

target_link_libraries(test_module_045
  PRIVATE
    GTest::gtest_main synthetic_module_045
)
gtest_discover_tests(test_module_045
  TEST_PREFIX
    "mod045::"

  DISCOVERY_TIMEOUT
    60
)

# Module 045 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD045_A   val_a)   # first
set(MOD045_BB  val_bb)  # second
set(MOD045_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 46
# ==========================================================================

add_library(synthetic_module_046
  STATIC
  src/modules/mod046/file_0002.cpp
  src/modules/mod046/file_0009.cpp
  src/modules/mod046/file_0016.cpp
  src/modules/mod046/file_0023.cpp
  src/modules/mod046/file_0030.cpp
  src/modules/mod046/file_0037.cpp
  src/modules/mod046/file_0044.cpp
  src/modules/mod046/file_0046.cpp
  src/modules/mod046/file_0053.cpp
  src/modules/mod046/file_0060.cpp
  src/modules/mod046/file_0067.cpp
  src/modules/mod046/file_0074.cpp
  src/modules/mod046/file_0081.cpp
  src/modules/mod046/file_0088.cpp
  src/modules/mod046/file_0095.cpp
)

set_target_properties(synthetic_module_046 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch46"
  OUTPUT_NAME           "mod046"
)

target_include_directories(synthetic_module_046
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod046>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod046
)

target_compile_definitions(synthetic_module_046
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_046_DEBUG>
    SYNTHETIC_MODULE_046=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_046
)

target_link_libraries(synthetic_module_046
  PUBLIC
    synthetic_module_045

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD046_SETTING_A "value_46_0")
set(MOD046_SETTING_B "value_46_1")
set(MOD046_SETTING_C "value_46_2")
set(MOD046_SETTING_D "value_46_3")
set(MOD046_SETTING_E "value_46_4")

if(SYNTHETIC_ENABLE_MODULE_046)
  message(STATUS "Module 046 enabled")
  target_compile_definitions(synthetic_module_046
    PUBLIC
      SYNTHETIC_MODULE_046_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_046)
  message(STATUS "Module 046 disabled")
endif(SYNTHETIC_ENABLE_MODULE_046)

add_executable(test_module_046
  tests/modules/mod046/test_0.cpp
  tests/modules/mod046/test_1.cpp
  tests/modules/mod046/test_2.cpp
  tests/modules/mod046/test_3.cpp
  tests/modules/mod046/test_4.cpp
)

target_link_libraries(test_module_046
  PRIVATE
    GTest::gtest_main synthetic_module_046
)
gtest_discover_tests(test_module_046
  TEST_PREFIX
    "mod046::"

  DISCOVERY_TIMEOUT
    60
)

# Module 046 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD046_A   val_a)   # first
set(MOD046_BB  val_bb)  # second
set(MOD046_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 47
# ==========================================================================

add_library(synthetic_module_047
  STATIC
  src/modules/mod047/file_0003.cpp
  src/modules/mod047/file_0010.cpp
  src/modules/mod047/file_0017.cpp
  src/modules/mod047/file_0024.cpp
  src/modules/mod047/file_0031.cpp
  src/modules/mod047/file_0038.cpp
  src/modules/mod047/file_0045.cpp
  src/modules/mod047/file_0047.cpp
  src/modules/mod047/file_0054.cpp
  src/modules/mod047/file_0061.cpp
  src/modules/mod047/file_0068.cpp
  src/modules/mod047/file_0075.cpp
  src/modules/mod047/file_0082.cpp
  src/modules/mod047/file_0089.cpp
  src/modules/mod047/file_0096.cpp
)

set_target_properties(synthetic_module_047 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch47"
  OUTPUT_NAME           "mod047"
)

target_include_directories(synthetic_module_047
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod047>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod047
)

target_compile_definitions(synthetic_module_047
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_047_DEBUG>
    SYNTHETIC_MODULE_047=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_047
)

target_link_libraries(synthetic_module_047
  PUBLIC
    synthetic_module_046

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD047_SETTING_A "value_47_0")
set(MOD047_SETTING_B "value_47_1")
set(MOD047_SETTING_C "value_47_2")
set(MOD047_SETTING_D "value_47_3")
set(MOD047_SETTING_E "value_47_4")

if(SYNTHETIC_ENABLE_MODULE_047)
  message(STATUS "Module 047 enabled")
  target_compile_definitions(synthetic_module_047
    PUBLIC
      SYNTHETIC_MODULE_047_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_047)
  message(STATUS "Module 047 disabled")
endif(SYNTHETIC_ENABLE_MODULE_047)

add_executable(test_module_047
  tests/modules/mod047/test_0.cpp
  tests/modules/mod047/test_1.cpp
  tests/modules/mod047/test_2.cpp
  tests/modules/mod047/test_3.cpp
  tests/modules/mod047/test_4.cpp
)

target_link_libraries(test_module_047
  PRIVATE
    GTest::gtest_main synthetic_module_047
)
gtest_discover_tests(test_module_047
  TEST_PREFIX
    "mod047::"

  DISCOVERY_TIMEOUT
    60
)

# Module 047 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD047_A   val_a)   # first
set(MOD047_BB  val_bb)  # second
set(MOD047_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 48
# ==========================================================================

add_library(synthetic_module_048
  STATIC
  src/modules/mod048/file_0004.cpp
  src/modules/mod048/file_0011.cpp
  src/modules/mod048/file_0018.cpp
  src/modules/mod048/file_0025.cpp
  src/modules/mod048/file_0032.cpp
  src/modules/mod048/file_0039.cpp
  src/modules/mod048/file_0046.cpp
  src/modules/mod048/file_0048.cpp
  src/modules/mod048/file_0055.cpp
  src/modules/mod048/file_0062.cpp
  src/modules/mod048/file_0069.cpp
  src/modules/mod048/file_0076.cpp
  src/modules/mod048/file_0083.cpp
  src/modules/mod048/file_0090.cpp
  src/modules/mod048/file_0097.cpp
)

set_target_properties(synthetic_module_048 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch48"
  OUTPUT_NAME           "mod048"
)

target_include_directories(synthetic_module_048
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod048>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod048
)

target_compile_definitions(synthetic_module_048
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_048_DEBUG>
    SYNTHETIC_MODULE_048=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_048
)

target_link_libraries(synthetic_module_048
  PUBLIC
    synthetic_module_047

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD048_SETTING_A "value_48_0")
set(MOD048_SETTING_B "value_48_1")
set(MOD048_SETTING_C "value_48_2")
set(MOD048_SETTING_D "value_48_3")
set(MOD048_SETTING_E "value_48_4")

if(SYNTHETIC_ENABLE_MODULE_048)
  message(STATUS "Module 048 enabled")
  target_compile_definitions(synthetic_module_048
    PUBLIC
      SYNTHETIC_MODULE_048_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_048)
  message(STATUS "Module 048 disabled")
endif(SYNTHETIC_ENABLE_MODULE_048)

add_executable(test_module_048
  tests/modules/mod048/test_0.cpp
  tests/modules/mod048/test_1.cpp
  tests/modules/mod048/test_2.cpp
  tests/modules/mod048/test_3.cpp
  tests/modules/mod048/test_4.cpp
)

target_link_libraries(test_module_048
  PRIVATE
    GTest::gtest_main synthetic_module_048
)
gtest_discover_tests(test_module_048
  TEST_PREFIX
    "mod048::"

  DISCOVERY_TIMEOUT
    60
)

# Module 048 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD048_A   val_a)   # first
set(MOD048_BB  val_bb)  # second
set(MOD048_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 49
# ==========================================================================

add_library(synthetic_module_049
  STATIC
  src/modules/mod049/file_0005.cpp
  src/modules/mod049/file_0012.cpp
  src/modules/mod049/file_0019.cpp
  src/modules/mod049/file_0026.cpp
  src/modules/mod049/file_0033.cpp
  src/modules/mod049/file_0040.cpp
  src/modules/mod049/file_0047.cpp
  src/modules/mod049/file_0049.cpp
  src/modules/mod049/file_0056.cpp
  src/modules/mod049/file_0063.cpp
  src/modules/mod049/file_0070.cpp
  src/modules/mod049/file_0077.cpp
  src/modules/mod049/file_0084.cpp
  src/modules/mod049/file_0091.cpp
  src/modules/mod049/file_0098.cpp
)

set_target_properties(synthetic_module_049 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch49"
  OUTPUT_NAME           "mod049"
)

target_include_directories(synthetic_module_049
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod049>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod049
)

target_compile_definitions(synthetic_module_049
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_049_DEBUG>
    SYNTHETIC_MODULE_049=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_049
)

target_link_libraries(synthetic_module_049
  PUBLIC
    synthetic_module_048

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD049_SETTING_A "value_49_0")
set(MOD049_SETTING_B "value_49_1")
set(MOD049_SETTING_C "value_49_2")
set(MOD049_SETTING_D "value_49_3")
set(MOD049_SETTING_E "value_49_4")

if(SYNTHETIC_ENABLE_MODULE_049)
  message(STATUS "Module 049 enabled")
  target_compile_definitions(synthetic_module_049
    PUBLIC
      SYNTHETIC_MODULE_049_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_049)
  message(STATUS "Module 049 disabled")
endif(SYNTHETIC_ENABLE_MODULE_049)

add_executable(test_module_049
  tests/modules/mod049/test_0.cpp
  tests/modules/mod049/test_1.cpp
  tests/modules/mod049/test_2.cpp
  tests/modules/mod049/test_3.cpp
  tests/modules/mod049/test_4.cpp
)

target_link_libraries(test_module_049
  PRIVATE
    GTest::gtest_main synthetic_module_049
)
gtest_discover_tests(test_module_049
  TEST_PREFIX
    "mod049::"

  DISCOVERY_TIMEOUT
    60
)

# Module 049 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD049_A   val_a)   # first
set(MOD049_BB  val_bb)  # second
set(MOD049_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 50
# ==========================================================================

add_library(synthetic_module_050
  STATIC
  src/modules/mod050/file_0006.cpp
  src/modules/mod050/file_0013.cpp
  src/modules/mod050/file_0020.cpp
  src/modules/mod050/file_0027.cpp
  src/modules/mod050/file_0034.cpp
  src/modules/mod050/file_0041.cpp
  src/modules/mod050/file_0048.cpp
  src/modules/mod050/file_0050.cpp
  src/modules/mod050/file_0057.cpp
  src/modules/mod050/file_0064.cpp
  src/modules/mod050/file_0071.cpp
  src/modules/mod050/file_0078.cpp
  src/modules/mod050/file_0085.cpp
  src/modules/mod050/file_0092.cpp
  src/modules/mod050/file_0099.cpp
)

set_target_properties(synthetic_module_050 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch50"
  OUTPUT_NAME           "mod050"
)

target_include_directories(synthetic_module_050
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod050>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod050
)

target_compile_definitions(synthetic_module_050
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_050_DEBUG>
    SYNTHETIC_MODULE_050=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_050
)

target_link_libraries(synthetic_module_050
  PUBLIC
    synthetic_module_049

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD050_SETTING_A "value_50_0")
set(MOD050_SETTING_B "value_50_1")
set(MOD050_SETTING_C "value_50_2")
set(MOD050_SETTING_D "value_50_3")
set(MOD050_SETTING_E "value_50_4")

if(SYNTHETIC_ENABLE_MODULE_050)
  message(STATUS "Module 050 enabled")
  target_compile_definitions(synthetic_module_050
    PUBLIC
      SYNTHETIC_MODULE_050_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_050)
  message(STATUS "Module 050 disabled")
endif(SYNTHETIC_ENABLE_MODULE_050)

add_executable(test_module_050
  tests/modules/mod050/test_0.cpp
  tests/modules/mod050/test_1.cpp
  tests/modules/mod050/test_2.cpp
  tests/modules/mod050/test_3.cpp
  tests/modules/mod050/test_4.cpp
)

target_link_libraries(test_module_050
  PRIVATE
    GTest::gtest_main synthetic_module_050
)
gtest_discover_tests(test_module_050
  TEST_PREFIX
    "mod050::"

  DISCOVERY_TIMEOUT
    60
)

# Module 050 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD050_A   val_a)   # first
set(MOD050_BB  val_bb)  # second
set(MOD050_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 51
# ==========================================================================

add_library(synthetic_module_051
  STATIC
  src/modules/mod051/file_0000.cpp
  src/modules/mod051/file_0007.cpp
  src/modules/mod051/file_0014.cpp
  src/modules/mod051/file_0021.cpp
  src/modules/mod051/file_0028.cpp
  src/modules/mod051/file_0035.cpp
  src/modules/mod051/file_0042.cpp
  src/modules/mod051/file_0049.cpp
  src/modules/mod051/file_0051.cpp
  src/modules/mod051/file_0058.cpp
  src/modules/mod051/file_0065.cpp
  src/modules/mod051/file_0072.cpp
  src/modules/mod051/file_0079.cpp
  src/modules/mod051/file_0086.cpp
  src/modules/mod051/file_0093.cpp
)

set_target_properties(synthetic_module_051 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch51"
  OUTPUT_NAME           "mod051"
)

target_include_directories(synthetic_module_051
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod051>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod051
)

target_compile_definitions(synthetic_module_051
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_051_DEBUG>
    SYNTHETIC_MODULE_051=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_051
)

target_link_libraries(synthetic_module_051
  PUBLIC
    synthetic_module_050

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD051_SETTING_A "value_51_0")
set(MOD051_SETTING_B "value_51_1")
set(MOD051_SETTING_C "value_51_2")
set(MOD051_SETTING_D "value_51_3")
set(MOD051_SETTING_E "value_51_4")

if(SYNTHETIC_ENABLE_MODULE_051)
  message(STATUS "Module 051 enabled")
  target_compile_definitions(synthetic_module_051
    PUBLIC
      SYNTHETIC_MODULE_051_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_051)
  message(STATUS "Module 051 disabled")
endif(SYNTHETIC_ENABLE_MODULE_051)

add_executable(test_module_051
  tests/modules/mod051/test_0.cpp
  tests/modules/mod051/test_1.cpp
  tests/modules/mod051/test_2.cpp
  tests/modules/mod051/test_3.cpp
  tests/modules/mod051/test_4.cpp
)

target_link_libraries(test_module_051
  PRIVATE
    GTest::gtest_main synthetic_module_051
)
gtest_discover_tests(test_module_051
  TEST_PREFIX
    "mod051::"

  DISCOVERY_TIMEOUT
    60
)

# Module 051 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD051_A   val_a)   # first
set(MOD051_BB  val_bb)  # second
set(MOD051_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 52
# ==========================================================================

add_library(synthetic_module_052
  STATIC
  src/modules/mod052/file_0001.cpp
  src/modules/mod052/file_0008.cpp
  src/modules/mod052/file_0015.cpp
  src/modules/mod052/file_0022.cpp
  src/modules/mod052/file_0029.cpp
  src/modules/mod052/file_0036.cpp
  src/modules/mod052/file_0043.cpp
  src/modules/mod052/file_0050.cpp
  src/modules/mod052/file_0052.cpp
  src/modules/mod052/file_0059.cpp
  src/modules/mod052/file_0066.cpp
  src/modules/mod052/file_0073.cpp
  src/modules/mod052/file_0080.cpp
  src/modules/mod052/file_0087.cpp
  src/modules/mod052/file_0094.cpp
)

set_target_properties(synthetic_module_052 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch52"
  OUTPUT_NAME           "mod052"
)

target_include_directories(synthetic_module_052
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod052>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod052
)

target_compile_definitions(synthetic_module_052
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_052_DEBUG>
    SYNTHETIC_MODULE_052=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_052
)

target_link_libraries(synthetic_module_052
  PUBLIC
    synthetic_module_051

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD052_SETTING_A "value_52_0")
set(MOD052_SETTING_B "value_52_1")
set(MOD052_SETTING_C "value_52_2")
set(MOD052_SETTING_D "value_52_3")
set(MOD052_SETTING_E "value_52_4")

if(SYNTHETIC_ENABLE_MODULE_052)
  message(STATUS "Module 052 enabled")
  target_compile_definitions(synthetic_module_052
    PUBLIC
      SYNTHETIC_MODULE_052_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_052)
  message(STATUS "Module 052 disabled")
endif(SYNTHETIC_ENABLE_MODULE_052)

add_executable(test_module_052
  tests/modules/mod052/test_0.cpp
  tests/modules/mod052/test_1.cpp
  tests/modules/mod052/test_2.cpp
  tests/modules/mod052/test_3.cpp
  tests/modules/mod052/test_4.cpp
)

target_link_libraries(test_module_052
  PRIVATE
    GTest::gtest_main synthetic_module_052
)
gtest_discover_tests(test_module_052
  TEST_PREFIX
    "mod052::"

  DISCOVERY_TIMEOUT
    60
)

# Module 052 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD052_A   val_a)   # first
set(MOD052_BB  val_bb)  # second
set(MOD052_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 53
# ==========================================================================

add_library(synthetic_module_053
  STATIC
  src/modules/mod053/file_0002.cpp
  src/modules/mod053/file_0009.cpp
  src/modules/mod053/file_0016.cpp
  src/modules/mod053/file_0023.cpp
  src/modules/mod053/file_0030.cpp
  src/modules/mod053/file_0037.cpp
  src/modules/mod053/file_0044.cpp
  src/modules/mod053/file_0051.cpp
  src/modules/mod053/file_0053.cpp
  src/modules/mod053/file_0060.cpp
  src/modules/mod053/file_0067.cpp
  src/modules/mod053/file_0074.cpp
  src/modules/mod053/file_0081.cpp
  src/modules/mod053/file_0088.cpp
  src/modules/mod053/file_0095.cpp
)

set_target_properties(synthetic_module_053 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch53"
  OUTPUT_NAME           "mod053"
)

target_include_directories(synthetic_module_053
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod053>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod053
)

target_compile_definitions(synthetic_module_053
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_053_DEBUG>
    SYNTHETIC_MODULE_053=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_053
)

target_link_libraries(synthetic_module_053
  PUBLIC
    synthetic_module_052

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD053_SETTING_A "value_53_0")
set(MOD053_SETTING_B "value_53_1")
set(MOD053_SETTING_C "value_53_2")
set(MOD053_SETTING_D "value_53_3")
set(MOD053_SETTING_E "value_53_4")

if(SYNTHETIC_ENABLE_MODULE_053)
  message(STATUS "Module 053 enabled")
  target_compile_definitions(synthetic_module_053
    PUBLIC
      SYNTHETIC_MODULE_053_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_053)
  message(STATUS "Module 053 disabled")
endif(SYNTHETIC_ENABLE_MODULE_053)

add_executable(test_module_053
  tests/modules/mod053/test_0.cpp
  tests/modules/mod053/test_1.cpp
  tests/modules/mod053/test_2.cpp
  tests/modules/mod053/test_3.cpp
  tests/modules/mod053/test_4.cpp
)

target_link_libraries(test_module_053
  PRIVATE
    GTest::gtest_main synthetic_module_053
)
gtest_discover_tests(test_module_053
  TEST_PREFIX
    "mod053::"

  DISCOVERY_TIMEOUT
    60
)

# Module 053 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD053_A   val_a)   # first
set(MOD053_BB  val_bb)  # second
set(MOD053_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 54
# ==========================================================================

add_library(synthetic_module_054
  STATIC
  src/modules/mod054/file_0003.cpp
  src/modules/mod054/file_0010.cpp
  src/modules/mod054/file_0017.cpp
  src/modules/mod054/file_0024.cpp
  src/modules/mod054/file_0031.cpp
  src/modules/mod054/file_0038.cpp
  src/modules/mod054/file_0045.cpp
  src/modules/mod054/file_0052.cpp
  src/modules/mod054/file_0054.cpp
  src/modules/mod054/file_0061.cpp
  src/modules/mod054/file_0068.cpp
  src/modules/mod054/file_0075.cpp
  src/modules/mod054/file_0082.cpp
  src/modules/mod054/file_0089.cpp
  src/modules/mod054/file_0096.cpp
)

set_target_properties(synthetic_module_054 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch54"
  OUTPUT_NAME           "mod054"
)

target_include_directories(synthetic_module_054
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod054>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod054
)

target_compile_definitions(synthetic_module_054
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_054_DEBUG>
    SYNTHETIC_MODULE_054=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_054
)

target_link_libraries(synthetic_module_054
  PUBLIC
    synthetic_module_053

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD054_SETTING_A "value_54_0")
set(MOD054_SETTING_B "value_54_1")
set(MOD054_SETTING_C "value_54_2")
set(MOD054_SETTING_D "value_54_3")
set(MOD054_SETTING_E "value_54_4")

if(SYNTHETIC_ENABLE_MODULE_054)
  message(STATUS "Module 054 enabled")
  target_compile_definitions(synthetic_module_054
    PUBLIC
      SYNTHETIC_MODULE_054_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_054)
  message(STATUS "Module 054 disabled")
endif(SYNTHETIC_ENABLE_MODULE_054)

add_executable(test_module_054
  tests/modules/mod054/test_0.cpp
  tests/modules/mod054/test_1.cpp
  tests/modules/mod054/test_2.cpp
  tests/modules/mod054/test_3.cpp
  tests/modules/mod054/test_4.cpp
)

target_link_libraries(test_module_054
  PRIVATE
    GTest::gtest_main synthetic_module_054
)
gtest_discover_tests(test_module_054
  TEST_PREFIX
    "mod054::"

  DISCOVERY_TIMEOUT
    60
)

# Module 054 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD054_A   val_a)   # first
set(MOD054_BB  val_bb)  # second
set(MOD054_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 55
# ==========================================================================

add_library(synthetic_module_055
  STATIC
  src/modules/mod055/file_0004.cpp
  src/modules/mod055/file_0011.cpp
  src/modules/mod055/file_0018.cpp
  src/modules/mod055/file_0025.cpp
  src/modules/mod055/file_0032.cpp
  src/modules/mod055/file_0039.cpp
  src/modules/mod055/file_0046.cpp
  src/modules/mod055/file_0053.cpp
  src/modules/mod055/file_0055.cpp
  src/modules/mod055/file_0062.cpp
  src/modules/mod055/file_0069.cpp
  src/modules/mod055/file_0076.cpp
  src/modules/mod055/file_0083.cpp
  src/modules/mod055/file_0090.cpp
  src/modules/mod055/file_0097.cpp
)

set_target_properties(synthetic_module_055 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch55"
  OUTPUT_NAME           "mod055"
)

target_include_directories(synthetic_module_055
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod055>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod055
)

target_compile_definitions(synthetic_module_055
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_055_DEBUG>
    SYNTHETIC_MODULE_055=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_055
)

target_link_libraries(synthetic_module_055
  PUBLIC
    synthetic_module_054

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD055_SETTING_A "value_55_0")
set(MOD055_SETTING_B "value_55_1")
set(MOD055_SETTING_C "value_55_2")
set(MOD055_SETTING_D "value_55_3")
set(MOD055_SETTING_E "value_55_4")

if(SYNTHETIC_ENABLE_MODULE_055)
  message(STATUS "Module 055 enabled")
  target_compile_definitions(synthetic_module_055
    PUBLIC
      SYNTHETIC_MODULE_055_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_055)
  message(STATUS "Module 055 disabled")
endif(SYNTHETIC_ENABLE_MODULE_055)

add_executable(test_module_055
  tests/modules/mod055/test_0.cpp
  tests/modules/mod055/test_1.cpp
  tests/modules/mod055/test_2.cpp
  tests/modules/mod055/test_3.cpp
  tests/modules/mod055/test_4.cpp
)

target_link_libraries(test_module_055
  PRIVATE
    GTest::gtest_main synthetic_module_055
)
gtest_discover_tests(test_module_055
  TEST_PREFIX
    "mod055::"

  DISCOVERY_TIMEOUT
    60
)

# Module 055 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD055_A   val_a)   # first
set(MOD055_BB  val_bb)  # second
set(MOD055_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 56
# ==========================================================================

add_library(synthetic_module_056
  STATIC
  src/modules/mod056/file_0005.cpp
  src/modules/mod056/file_0012.cpp
  src/modules/mod056/file_0019.cpp
  src/modules/mod056/file_0026.cpp
  src/modules/mod056/file_0033.cpp
  src/modules/mod056/file_0040.cpp
  src/modules/mod056/file_0047.cpp
  src/modules/mod056/file_0054.cpp
  src/modules/mod056/file_0056.cpp
  src/modules/mod056/file_0063.cpp
  src/modules/mod056/file_0070.cpp
  src/modules/mod056/file_0077.cpp
  src/modules/mod056/file_0084.cpp
  src/modules/mod056/file_0091.cpp
  src/modules/mod056/file_0098.cpp
)

set_target_properties(synthetic_module_056 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch56"
  OUTPUT_NAME           "mod056"
)

target_include_directories(synthetic_module_056
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod056>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod056
)

target_compile_definitions(synthetic_module_056
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_056_DEBUG>
    SYNTHETIC_MODULE_056=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_056
)

target_link_libraries(synthetic_module_056
  PUBLIC
    synthetic_module_055

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD056_SETTING_A "value_56_0")
set(MOD056_SETTING_B "value_56_1")
set(MOD056_SETTING_C "value_56_2")
set(MOD056_SETTING_D "value_56_3")
set(MOD056_SETTING_E "value_56_4")

if(SYNTHETIC_ENABLE_MODULE_056)
  message(STATUS "Module 056 enabled")
  target_compile_definitions(synthetic_module_056
    PUBLIC
      SYNTHETIC_MODULE_056_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_056)
  message(STATUS "Module 056 disabled")
endif(SYNTHETIC_ENABLE_MODULE_056)

add_executable(test_module_056
  tests/modules/mod056/test_0.cpp
  tests/modules/mod056/test_1.cpp
  tests/modules/mod056/test_2.cpp
  tests/modules/mod056/test_3.cpp
  tests/modules/mod056/test_4.cpp
)

target_link_libraries(test_module_056
  PRIVATE
    GTest::gtest_main synthetic_module_056
)
gtest_discover_tests(test_module_056
  TEST_PREFIX
    "mod056::"

  DISCOVERY_TIMEOUT
    60
)

# Module 056 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD056_A   val_a)   # first
set(MOD056_BB  val_bb)  # second
set(MOD056_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 57
# ==========================================================================

add_library(synthetic_module_057
  STATIC
  src/modules/mod057/file_0006.cpp
  src/modules/mod057/file_0013.cpp
  src/modules/mod057/file_0020.cpp
  src/modules/mod057/file_0027.cpp
  src/modules/mod057/file_0034.cpp
  src/modules/mod057/file_0041.cpp
  src/modules/mod057/file_0048.cpp
  src/modules/mod057/file_0055.cpp
  src/modules/mod057/file_0057.cpp
  src/modules/mod057/file_0064.cpp
  src/modules/mod057/file_0071.cpp
  src/modules/mod057/file_0078.cpp
  src/modules/mod057/file_0085.cpp
  src/modules/mod057/file_0092.cpp
  src/modules/mod057/file_0099.cpp
)

set_target_properties(synthetic_module_057 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch57"
  OUTPUT_NAME           "mod057"
)

target_include_directories(synthetic_module_057
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod057>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod057
)

target_compile_definitions(synthetic_module_057
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_057_DEBUG>
    SYNTHETIC_MODULE_057=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_057
)

target_link_libraries(synthetic_module_057
  PUBLIC
    synthetic_module_056

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD057_SETTING_A "value_57_0")
set(MOD057_SETTING_B "value_57_1")
set(MOD057_SETTING_C "value_57_2")
set(MOD057_SETTING_D "value_57_3")
set(MOD057_SETTING_E "value_57_4")

if(SYNTHETIC_ENABLE_MODULE_057)
  message(STATUS "Module 057 enabled")
  target_compile_definitions(synthetic_module_057
    PUBLIC
      SYNTHETIC_MODULE_057_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_057)
  message(STATUS "Module 057 disabled")
endif(SYNTHETIC_ENABLE_MODULE_057)

add_executable(test_module_057
  tests/modules/mod057/test_0.cpp
  tests/modules/mod057/test_1.cpp
  tests/modules/mod057/test_2.cpp
  tests/modules/mod057/test_3.cpp
  tests/modules/mod057/test_4.cpp
)

target_link_libraries(test_module_057
  PRIVATE
    GTest::gtest_main synthetic_module_057
)
gtest_discover_tests(test_module_057
  TEST_PREFIX
    "mod057::"

  DISCOVERY_TIMEOUT
    60
)

# Module 057 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD057_A   val_a)   # first
set(MOD057_BB  val_bb)  # second
set(MOD057_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 58
# ==========================================================================

add_library(synthetic_module_058
  STATIC
  src/modules/mod058/file_0000.cpp
  src/modules/mod058/file_0007.cpp
  src/modules/mod058/file_0014.cpp
  src/modules/mod058/file_0021.cpp
  src/modules/mod058/file_0028.cpp
  src/modules/mod058/file_0035.cpp
  src/modules/mod058/file_0042.cpp
  src/modules/mod058/file_0049.cpp
  src/modules/mod058/file_0056.cpp
  src/modules/mod058/file_0058.cpp
  src/modules/mod058/file_0065.cpp
  src/modules/mod058/file_0072.cpp
  src/modules/mod058/file_0079.cpp
  src/modules/mod058/file_0086.cpp
  src/modules/mod058/file_0093.cpp
)

set_target_properties(synthetic_module_058 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch58"
  OUTPUT_NAME           "mod058"
)

target_include_directories(synthetic_module_058
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod058>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod058
)

target_compile_definitions(synthetic_module_058
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_058_DEBUG>
    SYNTHETIC_MODULE_058=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_058
)

target_link_libraries(synthetic_module_058
  PUBLIC
    synthetic_module_057

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD058_SETTING_A "value_58_0")
set(MOD058_SETTING_B "value_58_1")
set(MOD058_SETTING_C "value_58_2")
set(MOD058_SETTING_D "value_58_3")
set(MOD058_SETTING_E "value_58_4")

if(SYNTHETIC_ENABLE_MODULE_058)
  message(STATUS "Module 058 enabled")
  target_compile_definitions(synthetic_module_058
    PUBLIC
      SYNTHETIC_MODULE_058_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_058)
  message(STATUS "Module 058 disabled")
endif(SYNTHETIC_ENABLE_MODULE_058)

add_executable(test_module_058
  tests/modules/mod058/test_0.cpp
  tests/modules/mod058/test_1.cpp
  tests/modules/mod058/test_2.cpp
  tests/modules/mod058/test_3.cpp
  tests/modules/mod058/test_4.cpp
)

target_link_libraries(test_module_058
  PRIVATE
    GTest::gtest_main synthetic_module_058
)
gtest_discover_tests(test_module_058
  TEST_PREFIX
    "mod058::"

  DISCOVERY_TIMEOUT
    60
)

# Module 058 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD058_A   val_a)   # first
set(MOD058_BB  val_bb)  # second
set(MOD058_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 59
# ==========================================================================

add_library(synthetic_module_059
  STATIC
  src/modules/mod059/file_0001.cpp
  src/modules/mod059/file_0008.cpp
  src/modules/mod059/file_0015.cpp
  src/modules/mod059/file_0022.cpp
  src/modules/mod059/file_0029.cpp
  src/modules/mod059/file_0036.cpp
  src/modules/mod059/file_0043.cpp
  src/modules/mod059/file_0050.cpp
  src/modules/mod059/file_0057.cpp
  src/modules/mod059/file_0059.cpp
  src/modules/mod059/file_0066.cpp
  src/modules/mod059/file_0073.cpp
  src/modules/mod059/file_0080.cpp
  src/modules/mod059/file_0087.cpp
  src/modules/mod059/file_0094.cpp
)

set_target_properties(synthetic_module_059 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch59"
  OUTPUT_NAME           "mod059"
)

target_include_directories(synthetic_module_059
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod059>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod059
)

target_compile_definitions(synthetic_module_059
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_059_DEBUG>
    SYNTHETIC_MODULE_059=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_059
)

target_link_libraries(synthetic_module_059
  PUBLIC
    synthetic_module_058

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD059_SETTING_A "value_59_0")
set(MOD059_SETTING_B "value_59_1")
set(MOD059_SETTING_C "value_59_2")
set(MOD059_SETTING_D "value_59_3")
set(MOD059_SETTING_E "value_59_4")

if(SYNTHETIC_ENABLE_MODULE_059)
  message(STATUS "Module 059 enabled")
  target_compile_definitions(synthetic_module_059
    PUBLIC
      SYNTHETIC_MODULE_059_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_059)
  message(STATUS "Module 059 disabled")
endif(SYNTHETIC_ENABLE_MODULE_059)

add_executable(test_module_059
  tests/modules/mod059/test_0.cpp
  tests/modules/mod059/test_1.cpp
  tests/modules/mod059/test_2.cpp
  tests/modules/mod059/test_3.cpp
  tests/modules/mod059/test_4.cpp
)

target_link_libraries(test_module_059
  PRIVATE
    GTest::gtest_main synthetic_module_059
)
gtest_discover_tests(test_module_059
  TEST_PREFIX
    "mod059::"

  DISCOVERY_TIMEOUT
    60
)

# Module 059 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD059_A   val_a)   # first
set(MOD059_BB  val_bb)  # second
set(MOD059_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 60
# ==========================================================================

add_library(synthetic_module_060
  STATIC
  src/modules/mod060/file_0002.cpp
  src/modules/mod060/file_0009.cpp
  src/modules/mod060/file_0016.cpp
  src/modules/mod060/file_0023.cpp
  src/modules/mod060/file_0030.cpp
  src/modules/mod060/file_0037.cpp
  src/modules/mod060/file_0044.cpp
  src/modules/mod060/file_0051.cpp
  src/modules/mod060/file_0058.cpp
  src/modules/mod060/file_0060.cpp
  src/modules/mod060/file_0067.cpp
  src/modules/mod060/file_0074.cpp
  src/modules/mod060/file_0081.cpp
  src/modules/mod060/file_0088.cpp
  src/modules/mod060/file_0095.cpp
)

set_target_properties(synthetic_module_060 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch60"
  OUTPUT_NAME           "mod060"
)

target_include_directories(synthetic_module_060
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod060>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod060
)

target_compile_definitions(synthetic_module_060
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_060_DEBUG>
    SYNTHETIC_MODULE_060=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_060
)

target_link_libraries(synthetic_module_060
  PUBLIC
    synthetic_module_059

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD060_SETTING_A "value_60_0")
set(MOD060_SETTING_B "value_60_1")
set(MOD060_SETTING_C "value_60_2")
set(MOD060_SETTING_D "value_60_3")
set(MOD060_SETTING_E "value_60_4")

if(SYNTHETIC_ENABLE_MODULE_060)
  message(STATUS "Module 060 enabled")
  target_compile_definitions(synthetic_module_060
    PUBLIC
      SYNTHETIC_MODULE_060_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_060)
  message(STATUS "Module 060 disabled")
endif(SYNTHETIC_ENABLE_MODULE_060)

add_executable(test_module_060
  tests/modules/mod060/test_0.cpp
  tests/modules/mod060/test_1.cpp
  tests/modules/mod060/test_2.cpp
  tests/modules/mod060/test_3.cpp
  tests/modules/mod060/test_4.cpp
)

target_link_libraries(test_module_060
  PRIVATE
    GTest::gtest_main synthetic_module_060
)
gtest_discover_tests(test_module_060
  TEST_PREFIX
    "mod060::"

  DISCOVERY_TIMEOUT
    60
)

# Module 060 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD060_A   val_a)   # first
set(MOD060_BB  val_bb)  # second
set(MOD060_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 61
# ==========================================================================

add_library(synthetic_module_061
  STATIC
  src/modules/mod061/file_0003.cpp
  src/modules/mod061/file_0010.cpp
  src/modules/mod061/file_0017.cpp
  src/modules/mod061/file_0024.cpp
  src/modules/mod061/file_0031.cpp
  src/modules/mod061/file_0038.cpp
  src/modules/mod061/file_0045.cpp
  src/modules/mod061/file_0052.cpp
  src/modules/mod061/file_0059.cpp
  src/modules/mod061/file_0061.cpp
  src/modules/mod061/file_0068.cpp
  src/modules/mod061/file_0075.cpp
  src/modules/mod061/file_0082.cpp
  src/modules/mod061/file_0089.cpp
  src/modules/mod061/file_0096.cpp
)

set_target_properties(synthetic_module_061 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch61"
  OUTPUT_NAME           "mod061"
)

target_include_directories(synthetic_module_061
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod061>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod061
)

target_compile_definitions(synthetic_module_061
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_061_DEBUG>
    SYNTHETIC_MODULE_061=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_061
)

target_link_libraries(synthetic_module_061
  PUBLIC
    synthetic_module_060

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD061_SETTING_A "value_61_0")
set(MOD061_SETTING_B "value_61_1")
set(MOD061_SETTING_C "value_61_2")
set(MOD061_SETTING_D "value_61_3")
set(MOD061_SETTING_E "value_61_4")

if(SYNTHETIC_ENABLE_MODULE_061)
  message(STATUS "Module 061 enabled")
  target_compile_definitions(synthetic_module_061
    PUBLIC
      SYNTHETIC_MODULE_061_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_061)
  message(STATUS "Module 061 disabled")
endif(SYNTHETIC_ENABLE_MODULE_061)

add_executable(test_module_061
  tests/modules/mod061/test_0.cpp
  tests/modules/mod061/test_1.cpp
  tests/modules/mod061/test_2.cpp
  tests/modules/mod061/test_3.cpp
  tests/modules/mod061/test_4.cpp
)

target_link_libraries(test_module_061
  PRIVATE
    GTest::gtest_main synthetic_module_061
)
gtest_discover_tests(test_module_061
  TEST_PREFIX
    "mod061::"

  DISCOVERY_TIMEOUT
    60
)

# Module 061 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD061_A   val_a)   # first
set(MOD061_BB  val_bb)  # second
set(MOD061_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 62
# ==========================================================================

add_library(synthetic_module_062
  STATIC
  src/modules/mod062/file_0004.cpp
  src/modules/mod062/file_0011.cpp
  src/modules/mod062/file_0018.cpp
  src/modules/mod062/file_0025.cpp
  src/modules/mod062/file_0032.cpp
  src/modules/mod062/file_0039.cpp
  src/modules/mod062/file_0046.cpp
  src/modules/mod062/file_0053.cpp
  src/modules/mod062/file_0060.cpp
  src/modules/mod062/file_0062.cpp
  src/modules/mod062/file_0069.cpp
  src/modules/mod062/file_0076.cpp
  src/modules/mod062/file_0083.cpp
  src/modules/mod062/file_0090.cpp
  src/modules/mod062/file_0097.cpp
)

set_target_properties(synthetic_module_062 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch62"
  OUTPUT_NAME           "mod062"
)

target_include_directories(synthetic_module_062
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod062>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod062
)

target_compile_definitions(synthetic_module_062
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_062_DEBUG>
    SYNTHETIC_MODULE_062=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_062
)

target_link_libraries(synthetic_module_062
  PUBLIC
    synthetic_module_061

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD062_SETTING_A "value_62_0")
set(MOD062_SETTING_B "value_62_1")
set(MOD062_SETTING_C "value_62_2")
set(MOD062_SETTING_D "value_62_3")
set(MOD062_SETTING_E "value_62_4")

if(SYNTHETIC_ENABLE_MODULE_062)
  message(STATUS "Module 062 enabled")
  target_compile_definitions(synthetic_module_062
    PUBLIC
      SYNTHETIC_MODULE_062_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_062)
  message(STATUS "Module 062 disabled")
endif(SYNTHETIC_ENABLE_MODULE_062)

add_executable(test_module_062
  tests/modules/mod062/test_0.cpp
  tests/modules/mod062/test_1.cpp
  tests/modules/mod062/test_2.cpp
  tests/modules/mod062/test_3.cpp
  tests/modules/mod062/test_4.cpp
)

target_link_libraries(test_module_062
  PRIVATE
    GTest::gtest_main synthetic_module_062
)
gtest_discover_tests(test_module_062
  TEST_PREFIX
    "mod062::"

  DISCOVERY_TIMEOUT
    60
)

# Module 062 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD062_A   val_a)   # first
set(MOD062_BB  val_bb)  # second
set(MOD062_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 63
# ==========================================================================

add_library(synthetic_module_063
  STATIC
  src/modules/mod063/file_0005.cpp
  src/modules/mod063/file_0012.cpp
  src/modules/mod063/file_0019.cpp
  src/modules/mod063/file_0026.cpp
  src/modules/mod063/file_0033.cpp
  src/modules/mod063/file_0040.cpp
  src/modules/mod063/file_0047.cpp
  src/modules/mod063/file_0054.cpp
  src/modules/mod063/file_0061.cpp
  src/modules/mod063/file_0063.cpp
  src/modules/mod063/file_0070.cpp
  src/modules/mod063/file_0077.cpp
  src/modules/mod063/file_0084.cpp
  src/modules/mod063/file_0091.cpp
  src/modules/mod063/file_0098.cpp
)

set_target_properties(synthetic_module_063 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch63"
  OUTPUT_NAME           "mod063"
)

target_include_directories(synthetic_module_063
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod063>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod063
)

target_compile_definitions(synthetic_module_063
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_063_DEBUG>
    SYNTHETIC_MODULE_063=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_063
)

target_link_libraries(synthetic_module_063
  PUBLIC
    synthetic_module_062

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD063_SETTING_A "value_63_0")
set(MOD063_SETTING_B "value_63_1")
set(MOD063_SETTING_C "value_63_2")
set(MOD063_SETTING_D "value_63_3")
set(MOD063_SETTING_E "value_63_4")

if(SYNTHETIC_ENABLE_MODULE_063)
  message(STATUS "Module 063 enabled")
  target_compile_definitions(synthetic_module_063
    PUBLIC
      SYNTHETIC_MODULE_063_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_063)
  message(STATUS "Module 063 disabled")
endif(SYNTHETIC_ENABLE_MODULE_063)

add_executable(test_module_063
  tests/modules/mod063/test_0.cpp
  tests/modules/mod063/test_1.cpp
  tests/modules/mod063/test_2.cpp
  tests/modules/mod063/test_3.cpp
  tests/modules/mod063/test_4.cpp
)

target_link_libraries(test_module_063
  PRIVATE
    GTest::gtest_main synthetic_module_063
)
gtest_discover_tests(test_module_063
  TEST_PREFIX
    "mod063::"

  DISCOVERY_TIMEOUT
    60
)

# Module 063 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD063_A   val_a)   # first
set(MOD063_BB  val_bb)  # second
set(MOD063_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 64
# ==========================================================================

add_library(synthetic_module_064
  STATIC
  src/modules/mod064/file_0006.cpp
  src/modules/mod064/file_0013.cpp
  src/modules/mod064/file_0020.cpp
  src/modules/mod064/file_0027.cpp
  src/modules/mod064/file_0034.cpp
  src/modules/mod064/file_0041.cpp
  src/modules/mod064/file_0048.cpp
  src/modules/mod064/file_0055.cpp
  src/modules/mod064/file_0062.cpp
  src/modules/mod064/file_0064.cpp
  src/modules/mod064/file_0071.cpp
  src/modules/mod064/file_0078.cpp
  src/modules/mod064/file_0085.cpp
  src/modules/mod064/file_0092.cpp
  src/modules/mod064/file_0099.cpp
)

set_target_properties(synthetic_module_064 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch64"
  OUTPUT_NAME           "mod064"
)

target_include_directories(synthetic_module_064
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod064>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod064
)

target_compile_definitions(synthetic_module_064
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_064_DEBUG>
    SYNTHETIC_MODULE_064=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_064
)

target_link_libraries(synthetic_module_064
  PUBLIC
    synthetic_module_063

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD064_SETTING_A "value_64_0")
set(MOD064_SETTING_B "value_64_1")
set(MOD064_SETTING_C "value_64_2")
set(MOD064_SETTING_D "value_64_3")
set(MOD064_SETTING_E "value_64_4")

if(SYNTHETIC_ENABLE_MODULE_064)
  message(STATUS "Module 064 enabled")
  target_compile_definitions(synthetic_module_064
    PUBLIC
      SYNTHETIC_MODULE_064_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_064)
  message(STATUS "Module 064 disabled")
endif(SYNTHETIC_ENABLE_MODULE_064)

add_executable(test_module_064
  tests/modules/mod064/test_0.cpp
  tests/modules/mod064/test_1.cpp
  tests/modules/mod064/test_2.cpp
  tests/modules/mod064/test_3.cpp
  tests/modules/mod064/test_4.cpp
)

target_link_libraries(test_module_064
  PRIVATE
    GTest::gtest_main synthetic_module_064
)
gtest_discover_tests(test_module_064
  TEST_PREFIX
    "mod064::"

  DISCOVERY_TIMEOUT
    60
)

# Module 064 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD064_A   val_a)   # first
set(MOD064_BB  val_bb)  # second
set(MOD064_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 65
# ==========================================================================

add_library(synthetic_module_065
  STATIC
  src/modules/mod065/file_0000.cpp
  src/modules/mod065/file_0007.cpp
  src/modules/mod065/file_0014.cpp
  src/modules/mod065/file_0021.cpp
  src/modules/mod065/file_0028.cpp
  src/modules/mod065/file_0035.cpp
  src/modules/mod065/file_0042.cpp
  src/modules/mod065/file_0049.cpp
  src/modules/mod065/file_0056.cpp
  src/modules/mod065/file_0063.cpp
  src/modules/mod065/file_0065.cpp
  src/modules/mod065/file_0072.cpp
  src/modules/mod065/file_0079.cpp
  src/modules/mod065/file_0086.cpp
  src/modules/mod065/file_0093.cpp
)

set_target_properties(synthetic_module_065 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch65"
  OUTPUT_NAME           "mod065"
)

target_include_directories(synthetic_module_065
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod065>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod065
)

target_compile_definitions(synthetic_module_065
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_065_DEBUG>
    SYNTHETIC_MODULE_065=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_065
)

target_link_libraries(synthetic_module_065
  PUBLIC
    synthetic_module_064

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD065_SETTING_A "value_65_0")
set(MOD065_SETTING_B "value_65_1")
set(MOD065_SETTING_C "value_65_2")
set(MOD065_SETTING_D "value_65_3")
set(MOD065_SETTING_E "value_65_4")

if(SYNTHETIC_ENABLE_MODULE_065)
  message(STATUS "Module 065 enabled")
  target_compile_definitions(synthetic_module_065
    PUBLIC
      SYNTHETIC_MODULE_065_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_065)
  message(STATUS "Module 065 disabled")
endif(SYNTHETIC_ENABLE_MODULE_065)

add_executable(test_module_065
  tests/modules/mod065/test_0.cpp
  tests/modules/mod065/test_1.cpp
  tests/modules/mod065/test_2.cpp
  tests/modules/mod065/test_3.cpp
  tests/modules/mod065/test_4.cpp
)

target_link_libraries(test_module_065
  PRIVATE
    GTest::gtest_main synthetic_module_065
)
gtest_discover_tests(test_module_065
  TEST_PREFIX
    "mod065::"

  DISCOVERY_TIMEOUT
    60
)

# Module 065 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD065_A   val_a)   # first
set(MOD065_BB  val_bb)  # second
set(MOD065_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 66
# ==========================================================================

add_library(synthetic_module_066
  STATIC
  src/modules/mod066/file_0001.cpp
  src/modules/mod066/file_0008.cpp
  src/modules/mod066/file_0015.cpp
  src/modules/mod066/file_0022.cpp
  src/modules/mod066/file_0029.cpp
  src/modules/mod066/file_0036.cpp
  src/modules/mod066/file_0043.cpp
  src/modules/mod066/file_0050.cpp
  src/modules/mod066/file_0057.cpp
  src/modules/mod066/file_0064.cpp
  src/modules/mod066/file_0066.cpp
  src/modules/mod066/file_0073.cpp
  src/modules/mod066/file_0080.cpp
  src/modules/mod066/file_0087.cpp
  src/modules/mod066/file_0094.cpp
)

set_target_properties(synthetic_module_066 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch66"
  OUTPUT_NAME           "mod066"
)

target_include_directories(synthetic_module_066
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod066>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod066
)

target_compile_definitions(synthetic_module_066
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_066_DEBUG>
    SYNTHETIC_MODULE_066=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_066
)

target_link_libraries(synthetic_module_066
  PUBLIC
    synthetic_module_065

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD066_SETTING_A "value_66_0")
set(MOD066_SETTING_B "value_66_1")
set(MOD066_SETTING_C "value_66_2")
set(MOD066_SETTING_D "value_66_3")
set(MOD066_SETTING_E "value_66_4")

if(SYNTHETIC_ENABLE_MODULE_066)
  message(STATUS "Module 066 enabled")
  target_compile_definitions(synthetic_module_066
    PUBLIC
      SYNTHETIC_MODULE_066_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_066)
  message(STATUS "Module 066 disabled")
endif(SYNTHETIC_ENABLE_MODULE_066)

add_executable(test_module_066
  tests/modules/mod066/test_0.cpp
  tests/modules/mod066/test_1.cpp
  tests/modules/mod066/test_2.cpp
  tests/modules/mod066/test_3.cpp
  tests/modules/mod066/test_4.cpp
)

target_link_libraries(test_module_066
  PRIVATE
    GTest::gtest_main synthetic_module_066
)
gtest_discover_tests(test_module_066
  TEST_PREFIX
    "mod066::"

  DISCOVERY_TIMEOUT
    60
)

# Module 066 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD066_A   val_a)   # first
set(MOD066_BB  val_bb)  # second
set(MOD066_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 67
# ==========================================================================

add_library(synthetic_module_067
  STATIC
  src/modules/mod067/file_0002.cpp
  src/modules/mod067/file_0009.cpp
  src/modules/mod067/file_0016.cpp
  src/modules/mod067/file_0023.cpp
  src/modules/mod067/file_0030.cpp
  src/modules/mod067/file_0037.cpp
  src/modules/mod067/file_0044.cpp
  src/modules/mod067/file_0051.cpp
  src/modules/mod067/file_0058.cpp
  src/modules/mod067/file_0065.cpp
  src/modules/mod067/file_0067.cpp
  src/modules/mod067/file_0074.cpp
  src/modules/mod067/file_0081.cpp
  src/modules/mod067/file_0088.cpp
  src/modules/mod067/file_0095.cpp
)

set_target_properties(synthetic_module_067 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch67"
  OUTPUT_NAME           "mod067"
)

target_include_directories(synthetic_module_067
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod067>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod067
)

target_compile_definitions(synthetic_module_067
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_067_DEBUG>
    SYNTHETIC_MODULE_067=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_067
)

target_link_libraries(synthetic_module_067
  PUBLIC
    synthetic_module_066

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD067_SETTING_A "value_67_0")
set(MOD067_SETTING_B "value_67_1")
set(MOD067_SETTING_C "value_67_2")
set(MOD067_SETTING_D "value_67_3")
set(MOD067_SETTING_E "value_67_4")

if(SYNTHETIC_ENABLE_MODULE_067)
  message(STATUS "Module 067 enabled")
  target_compile_definitions(synthetic_module_067
    PUBLIC
      SYNTHETIC_MODULE_067_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_067)
  message(STATUS "Module 067 disabled")
endif(SYNTHETIC_ENABLE_MODULE_067)

add_executable(test_module_067
  tests/modules/mod067/test_0.cpp
  tests/modules/mod067/test_1.cpp
  tests/modules/mod067/test_2.cpp
  tests/modules/mod067/test_3.cpp
  tests/modules/mod067/test_4.cpp
)

target_link_libraries(test_module_067
  PRIVATE
    GTest::gtest_main synthetic_module_067
)
gtest_discover_tests(test_module_067
  TEST_PREFIX
    "mod067::"

  DISCOVERY_TIMEOUT
    60
)

# Module 067 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD067_A   val_a)   # first
set(MOD067_BB  val_bb)  # second
set(MOD067_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 68
# ==========================================================================

add_library(synthetic_module_068
  STATIC
  src/modules/mod068/file_0003.cpp
  src/modules/mod068/file_0010.cpp
  src/modules/mod068/file_0017.cpp
  src/modules/mod068/file_0024.cpp
  src/modules/mod068/file_0031.cpp
  src/modules/mod068/file_0038.cpp
  src/modules/mod068/file_0045.cpp
  src/modules/mod068/file_0052.cpp
  src/modules/mod068/file_0059.cpp
  src/modules/mod068/file_0066.cpp
  src/modules/mod068/file_0068.cpp
  src/modules/mod068/file_0075.cpp
  src/modules/mod068/file_0082.cpp
  src/modules/mod068/file_0089.cpp
  src/modules/mod068/file_0096.cpp
)

set_target_properties(synthetic_module_068 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch68"
  OUTPUT_NAME           "mod068"
)

target_include_directories(synthetic_module_068
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod068>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod068
)

target_compile_definitions(synthetic_module_068
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_068_DEBUG>
    SYNTHETIC_MODULE_068=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_068
)

target_link_libraries(synthetic_module_068
  PUBLIC
    synthetic_module_067

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD068_SETTING_A "value_68_0")
set(MOD068_SETTING_B "value_68_1")
set(MOD068_SETTING_C "value_68_2")
set(MOD068_SETTING_D "value_68_3")
set(MOD068_SETTING_E "value_68_4")

if(SYNTHETIC_ENABLE_MODULE_068)
  message(STATUS "Module 068 enabled")
  target_compile_definitions(synthetic_module_068
    PUBLIC
      SYNTHETIC_MODULE_068_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_068)
  message(STATUS "Module 068 disabled")
endif(SYNTHETIC_ENABLE_MODULE_068)

add_executable(test_module_068
  tests/modules/mod068/test_0.cpp
  tests/modules/mod068/test_1.cpp
  tests/modules/mod068/test_2.cpp
  tests/modules/mod068/test_3.cpp
  tests/modules/mod068/test_4.cpp
)

target_link_libraries(test_module_068
  PRIVATE
    GTest::gtest_main synthetic_module_068
)
gtest_discover_tests(test_module_068
  TEST_PREFIX
    "mod068::"

  DISCOVERY_TIMEOUT
    60
)

# Module 068 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD068_A   val_a)   # first
set(MOD068_BB  val_bb)  # second
set(MOD068_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 69
# ==========================================================================

add_library(synthetic_module_069
  STATIC
  src/modules/mod069/file_0004.cpp
  src/modules/mod069/file_0011.cpp
  src/modules/mod069/file_0018.cpp
  src/modules/mod069/file_0025.cpp
  src/modules/mod069/file_0032.cpp
  src/modules/mod069/file_0039.cpp
  src/modules/mod069/file_0046.cpp
  src/modules/mod069/file_0053.cpp
  src/modules/mod069/file_0060.cpp
  src/modules/mod069/file_0067.cpp
  src/modules/mod069/file_0069.cpp
  src/modules/mod069/file_0076.cpp
  src/modules/mod069/file_0083.cpp
  src/modules/mod069/file_0090.cpp
  src/modules/mod069/file_0097.cpp
)

set_target_properties(synthetic_module_069 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch69"
  OUTPUT_NAME           "mod069"
)

target_include_directories(synthetic_module_069
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod069>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod069
)

target_compile_definitions(synthetic_module_069
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_069_DEBUG>
    SYNTHETIC_MODULE_069=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_069
)

target_link_libraries(synthetic_module_069
  PUBLIC
    synthetic_module_068

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD069_SETTING_A "value_69_0")
set(MOD069_SETTING_B "value_69_1")
set(MOD069_SETTING_C "value_69_2")
set(MOD069_SETTING_D "value_69_3")
set(MOD069_SETTING_E "value_69_4")

if(SYNTHETIC_ENABLE_MODULE_069)
  message(STATUS "Module 069 enabled")
  target_compile_definitions(synthetic_module_069
    PUBLIC
      SYNTHETIC_MODULE_069_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_069)
  message(STATUS "Module 069 disabled")
endif(SYNTHETIC_ENABLE_MODULE_069)

add_executable(test_module_069
  tests/modules/mod069/test_0.cpp
  tests/modules/mod069/test_1.cpp
  tests/modules/mod069/test_2.cpp
  tests/modules/mod069/test_3.cpp
  tests/modules/mod069/test_4.cpp
)

target_link_libraries(test_module_069
  PRIVATE
    GTest::gtest_main synthetic_module_069
)
gtest_discover_tests(test_module_069
  TEST_PREFIX
    "mod069::"

  DISCOVERY_TIMEOUT
    60
)

# Module 069 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD069_A   val_a)   # first
set(MOD069_BB  val_bb)  # second
set(MOD069_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 70
# ==========================================================================

add_library(synthetic_module_070
  STATIC
  src/modules/mod070/file_0005.cpp
  src/modules/mod070/file_0012.cpp
  src/modules/mod070/file_0019.cpp
  src/modules/mod070/file_0026.cpp
  src/modules/mod070/file_0033.cpp
  src/modules/mod070/file_0040.cpp
  src/modules/mod070/file_0047.cpp
  src/modules/mod070/file_0054.cpp
  src/modules/mod070/file_0061.cpp
  src/modules/mod070/file_0068.cpp
  src/modules/mod070/file_0070.cpp
  src/modules/mod070/file_0077.cpp
  src/modules/mod070/file_0084.cpp
  src/modules/mod070/file_0091.cpp
  src/modules/mod070/file_0098.cpp
)

set_target_properties(synthetic_module_070 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch70"
  OUTPUT_NAME           "mod070"
)

target_include_directories(synthetic_module_070
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod070>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod070
)

target_compile_definitions(synthetic_module_070
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_070_DEBUG>
    SYNTHETIC_MODULE_070=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_070
)

target_link_libraries(synthetic_module_070
  PUBLIC
    synthetic_module_069

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD070_SETTING_A "value_70_0")
set(MOD070_SETTING_B "value_70_1")
set(MOD070_SETTING_C "value_70_2")
set(MOD070_SETTING_D "value_70_3")
set(MOD070_SETTING_E "value_70_4")

if(SYNTHETIC_ENABLE_MODULE_070)
  message(STATUS "Module 070 enabled")
  target_compile_definitions(synthetic_module_070
    PUBLIC
      SYNTHETIC_MODULE_070_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_070)
  message(STATUS "Module 070 disabled")
endif(SYNTHETIC_ENABLE_MODULE_070)

add_executable(test_module_070
  tests/modules/mod070/test_0.cpp
  tests/modules/mod070/test_1.cpp
  tests/modules/mod070/test_2.cpp
  tests/modules/mod070/test_3.cpp
  tests/modules/mod070/test_4.cpp
)

target_link_libraries(test_module_070
  PRIVATE
    GTest::gtest_main synthetic_module_070
)
gtest_discover_tests(test_module_070
  TEST_PREFIX
    "mod070::"

  DISCOVERY_TIMEOUT
    60
)

# Module 070 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD070_A   val_a)   # first
set(MOD070_BB  val_bb)  # second
set(MOD070_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 71
# ==========================================================================

add_library(synthetic_module_071
  STATIC
  src/modules/mod071/file_0006.cpp
  src/modules/mod071/file_0013.cpp
  src/modules/mod071/file_0020.cpp
  src/modules/mod071/file_0027.cpp
  src/modules/mod071/file_0034.cpp
  src/modules/mod071/file_0041.cpp
  src/modules/mod071/file_0048.cpp
  src/modules/mod071/file_0055.cpp
  src/modules/mod071/file_0062.cpp
  src/modules/mod071/file_0069.cpp
  src/modules/mod071/file_0071.cpp
  src/modules/mod071/file_0078.cpp
  src/modules/mod071/file_0085.cpp
  src/modules/mod071/file_0092.cpp
  src/modules/mod071/file_0099.cpp
)

set_target_properties(synthetic_module_071 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch71"
  OUTPUT_NAME           "mod071"
)

target_include_directories(synthetic_module_071
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod071>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod071
)

target_compile_definitions(synthetic_module_071
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_071_DEBUG>
    SYNTHETIC_MODULE_071=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_071
)

target_link_libraries(synthetic_module_071
  PUBLIC
    synthetic_module_070

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD071_SETTING_A "value_71_0")
set(MOD071_SETTING_B "value_71_1")
set(MOD071_SETTING_C "value_71_2")
set(MOD071_SETTING_D "value_71_3")
set(MOD071_SETTING_E "value_71_4")

if(SYNTHETIC_ENABLE_MODULE_071)
  message(STATUS "Module 071 enabled")
  target_compile_definitions(synthetic_module_071
    PUBLIC
      SYNTHETIC_MODULE_071_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_071)
  message(STATUS "Module 071 disabled")
endif(SYNTHETIC_ENABLE_MODULE_071)

add_executable(test_module_071
  tests/modules/mod071/test_0.cpp
  tests/modules/mod071/test_1.cpp
  tests/modules/mod071/test_2.cpp
  tests/modules/mod071/test_3.cpp
  tests/modules/mod071/test_4.cpp
)

target_link_libraries(test_module_071
  PRIVATE
    GTest::gtest_main synthetic_module_071
)
gtest_discover_tests(test_module_071
  TEST_PREFIX
    "mod071::"

  DISCOVERY_TIMEOUT
    60
)

# Module 071 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD071_A   val_a)   # first
set(MOD071_BB  val_bb)  # second
set(MOD071_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 72
# ==========================================================================

add_library(synthetic_module_072
  STATIC
  src/modules/mod072/file_0000.cpp
  src/modules/mod072/file_0007.cpp
  src/modules/mod072/file_0014.cpp
  src/modules/mod072/file_0021.cpp
  src/modules/mod072/file_0028.cpp
  src/modules/mod072/file_0035.cpp
  src/modules/mod072/file_0042.cpp
  src/modules/mod072/file_0049.cpp
  src/modules/mod072/file_0056.cpp
  src/modules/mod072/file_0063.cpp
  src/modules/mod072/file_0070.cpp
  src/modules/mod072/file_0072.cpp
  src/modules/mod072/file_0079.cpp
  src/modules/mod072/file_0086.cpp
  src/modules/mod072/file_0093.cpp
)

set_target_properties(synthetic_module_072 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch72"
  OUTPUT_NAME           "mod072"
)

target_include_directories(synthetic_module_072
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod072>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod072
)

target_compile_definitions(synthetic_module_072
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_072_DEBUG>
    SYNTHETIC_MODULE_072=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_072
)

target_link_libraries(synthetic_module_072
  PUBLIC
    synthetic_module_071

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD072_SETTING_A "value_72_0")
set(MOD072_SETTING_B "value_72_1")
set(MOD072_SETTING_C "value_72_2")
set(MOD072_SETTING_D "value_72_3")
set(MOD072_SETTING_E "value_72_4")

if(SYNTHETIC_ENABLE_MODULE_072)
  message(STATUS "Module 072 enabled")
  target_compile_definitions(synthetic_module_072
    PUBLIC
      SYNTHETIC_MODULE_072_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_072)
  message(STATUS "Module 072 disabled")
endif(SYNTHETIC_ENABLE_MODULE_072)

add_executable(test_module_072
  tests/modules/mod072/test_0.cpp
  tests/modules/mod072/test_1.cpp
  tests/modules/mod072/test_2.cpp
  tests/modules/mod072/test_3.cpp
  tests/modules/mod072/test_4.cpp
)

target_link_libraries(test_module_072
  PRIVATE
    GTest::gtest_main synthetic_module_072
)
gtest_discover_tests(test_module_072
  TEST_PREFIX
    "mod072::"

  DISCOVERY_TIMEOUT
    60
)

# Module 072 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD072_A   val_a)   # first
set(MOD072_BB  val_bb)  # second
set(MOD072_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 73
# ==========================================================================

add_library(synthetic_module_073
  STATIC
  src/modules/mod073/file_0001.cpp
  src/modules/mod073/file_0008.cpp
  src/modules/mod073/file_0015.cpp
  src/modules/mod073/file_0022.cpp
  src/modules/mod073/file_0029.cpp
  src/modules/mod073/file_0036.cpp
  src/modules/mod073/file_0043.cpp
  src/modules/mod073/file_0050.cpp
  src/modules/mod073/file_0057.cpp
  src/modules/mod073/file_0064.cpp
  src/modules/mod073/file_0071.cpp
  src/modules/mod073/file_0073.cpp
  src/modules/mod073/file_0080.cpp
  src/modules/mod073/file_0087.cpp
  src/modules/mod073/file_0094.cpp
)

set_target_properties(synthetic_module_073 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch73"
  OUTPUT_NAME           "mod073"
)

target_include_directories(synthetic_module_073
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod073>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod073
)

target_compile_definitions(synthetic_module_073
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_073_DEBUG>
    SYNTHETIC_MODULE_073=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_073
)

target_link_libraries(synthetic_module_073
  PUBLIC
    synthetic_module_072

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD073_SETTING_A "value_73_0")
set(MOD073_SETTING_B "value_73_1")
set(MOD073_SETTING_C "value_73_2")
set(MOD073_SETTING_D "value_73_3")
set(MOD073_SETTING_E "value_73_4")

if(SYNTHETIC_ENABLE_MODULE_073)
  message(STATUS "Module 073 enabled")
  target_compile_definitions(synthetic_module_073
    PUBLIC
      SYNTHETIC_MODULE_073_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_073)
  message(STATUS "Module 073 disabled")
endif(SYNTHETIC_ENABLE_MODULE_073)

add_executable(test_module_073
  tests/modules/mod073/test_0.cpp
  tests/modules/mod073/test_1.cpp
  tests/modules/mod073/test_2.cpp
  tests/modules/mod073/test_3.cpp
  tests/modules/mod073/test_4.cpp
)

target_link_libraries(test_module_073
  PRIVATE
    GTest::gtest_main synthetic_module_073
)
gtest_discover_tests(test_module_073
  TEST_PREFIX
    "mod073::"

  DISCOVERY_TIMEOUT
    60
)

# Module 073 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD073_A   val_a)   # first
set(MOD073_BB  val_bb)  # second
set(MOD073_CCC val_ccc) # third

# ==========================================================================
# Repeated pattern batch 74
# ==========================================================================

add_library(synthetic_module_074
  STATIC
  src/modules/mod074/file_0002.cpp
  src/modules/mod074/file_0009.cpp
  src/modules/mod074/file_0016.cpp
  src/modules/mod074/file_0023.cpp
  src/modules/mod074/file_0030.cpp
  src/modules/mod074/file_0037.cpp
  src/modules/mod074/file_0044.cpp
  src/modules/mod074/file_0051.cpp
  src/modules/mod074/file_0058.cpp
  src/modules/mod074/file_0065.cpp
  src/modules/mod074/file_0072.cpp
  src/modules/mod074/file_0074.cpp
  src/modules/mod074/file_0081.cpp
  src/modules/mod074/file_0088.cpp
  src/modules/mod074/file_0095.cpp
)

set_target_properties(synthetic_module_074 PROPERTIES
  CXX_STANDARD          20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS        OFF
  FOLDER                "Modules/Batch74"
  OUTPUT_NAME           "mod074"
)

target_include_directories(synthetic_module_074
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mod074>
    $<INSTALL_INTERFACE:include>

  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/modules/mod074
)

target_compile_definitions(synthetic_module_074
  PUBLIC
    $<$<CONFIG:Debug>:SYNTHETIC_MODULE_074_DEBUG>
    SYNTHETIC_MODULE_074=1

  PRIVATE
    SYNTHETIC_MODULE_INTERNAL_074
)

target_link_libraries(synthetic_module_074
  PUBLIC
    synthetic_module_073

  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)

set(MOD074_SETTING_A "value_74_0")
set(MOD074_SETTING_B "value_74_1")
set(MOD074_SETTING_C "value_74_2")
set(MOD074_SETTING_D "value_74_3")
set(MOD074_SETTING_E "value_74_4")

if(SYNTHETIC_ENABLE_MODULE_074)
  message(STATUS "Module 074 enabled")
  target_compile_definitions(synthetic_module_074
    PUBLIC
      SYNTHETIC_MODULE_074_ENABLED
  )
else(SYNTHETIC_ENABLE_MODULE_074)
  message(STATUS "Module 074 disabled")
endif(SYNTHETIC_ENABLE_MODULE_074)

add_executable(test_module_074
  tests/modules/mod074/test_0.cpp
  tests/modules/mod074/test_1.cpp
  tests/modules/mod074/test_2.cpp
  tests/modules/mod074/test_3.cpp
  tests/modules/mod074/test_4.cpp
)

target_link_libraries(test_module_074
  PRIVATE
    GTest::gtest_main synthetic_module_074
)
gtest_discover_tests(test_module_074
  TEST_PREFIX
    "mod074::"

  DISCOVERY_TIMEOUT
    60
)

# Module 074 provides synthetic functionality for benchmark testing purposes.
# This module includes source files, compile definitions, and test
# infrastructure that exercises the formatter's handling of various CMake
# constructs.

set(MOD074_A   val_a)   # first
set(MOD074_BB  val_bb)  # second
set(MOD074_CCC val_ccc) # third

# =============================================================================
# Formatting Rules Demonstrated in This File
# =============================================================================
#
# This file showcases the default cmakefmt formatting behavior.
#
# 1. Command casing (commandCase: lower)
#    All command names are lowercase: cmake_minimum_required, project,
#    add_executable, target_sources, target_link_libraries, etc.
#
# 2. Keyword casing (keywordCase: upper)
#    All keywords are UPPERCASE: VERSION, PRIVATE, PUBLIC, INTERFACE,
#    FILE_SET, TYPE, HEADERS, BASE_DIRS, FILES.
#
# 3. Multi-line layout and indentation
#    The formatter uses a cascading strategy to fit commands within
#    lineWidth (default 80). Each nesting level increases indentation
#    by a configurable step (default 2 spaces). This applies recursively
#    to arbitrary depth — command keywords, values, sub-keywords,
#    sub-keyword values, and nested generator expressions ($<...>) all
#    follow the same algorithm.
#
#    Step 1 - Single line: If the entire invocation fits within lineWidth,
#      keep it on one line.
#      Example: target_sources(MyProgram PRIVATE main.cxx)
#
#    Step 2 - Keyword wraps, args inline: If step 1 overflows, move each
#      keyword onto a new line (indented 1 step). If all values under
#      that keyword fit on one line, keep them inline (indented 2 steps).
#      Example: target_sources(MyProgram
#        PRIVATE
#          main1.cxx main1.cxx main1.cxx main1.cxx ...)
#
#    Step 3 - One arg per line: If values still exceed lineWidth, place
#      each argument on its own line (indented 2 steps).
#      Example: target_sources(MyProgram
#        PRIVATE
#          main1.cxx
#          main1.cxx
#          ...)
#
# 4. Closing paren on its own line (closingParenNewline: true)
#    When a command spans multiple lines, the closing ')' is placed on
#    its own line at column 0 (no indentation).
#
# 5. Blank lines between top-level commands (maxBlankLines: 1)
#    At most one blank line separates consecutive top-level commands.
#
# 6. First positional arg on opening line
#    The target name (first positional argument) stays on the same line
#    as the command name: target_sources(MyProgram
#
# =============================================================================
cmake_minimum_required(VERSION 3.23)

project(MyProjectName)

add_executable(MyProgram)
target_sources(MyProgram PRIVATE main.cxx)
target_sources(MyProgram
  PRIVATE
    main1.cxx
    main2.cxx
    main3.cxx
    main4.cxx
    main5.cxx
    main6.cxx
    main7.cxx
    main8.cxx
    main9.cxx
)

target_sources(MyProgram
  PRIVATE
    main1.cxx
    main2.cxx
    main3.cxx
    main4.cxx
    main5.cxx
    main6.cxx
    main7.cxx
    main8.cxx
    main9.cxx
  PUBLIC main1.cxx main2.cxx main3.cxx main4.cxx main5.cxx main6.cxx main7.cxx
  INTERFACE main1.cxx main2.cxx main3.cxx
)

target_sources(MyProgram
  PRIVATE
    main1.cxx main2.cxx main3.cxx main4.cxx main5.cxx main6.cxx main7.cxx
)
add_library(MyLibrary)

target_sources(MyLibrary
  PRIVATE library_implementation.cxx
  PUBLIC
    FILE_SET myHeaders
    TYPE HEADERS
    BASE_DIRS include
    FILES include/library_header.h
)

target_sources(MyLibrary
  PRIVATE FILE_SET internalOnlyHeaders TYPE HEADERS FILES InternalOnlyHeader.h
  INTERFACE
    FILE_SET consumerOnlyHeaders TYPE HEADERS FILES ConsumerOnlyHeader.h
  PUBLIC FILE_SET publicHeaders TYPE HEADERS FILES PublicHeader.h
)

target_link_libraries(MyProgram PRIVATE MyLibrary)
target_link_libraries(MyProgram
  PRIVATE
    MyLibrary1
    MyLibrary2
    MyLibrary3
    MyLibrary4
    MyLibrary5
    MyLibrary6
    MyLibrary7
    MyLibrary8
    MyLibrary9
)

add_subdirectory(SubdirectoryName)

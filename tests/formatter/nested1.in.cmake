# =============================================================================
# Formatting Rules Demonstrated in This File
# =============================================================================
#
# This file tests the formatting of deeply nested generator expressions
# (genexes) alongside standard command formatting rules.
#
# 1. Command casing (commandCase: lower)
#    All command names are lowercased, even mixed-case input like
#    Target_Compile_Definitions becomes target_compile_definitions.
#
# 2. Keyword casing (keywordCase: upper)
#    All keywords are UPPERCASE: VERSION, LANGUAGES, PRIVATE, PUBLIC.
#
# 3. Blank line normalization (maxBlankLines: 1)
#    Runs of multiple blank lines between commands collapse to one.
#
# 4. Single-line commands when they fit
#    Simple commands that fit within lineWidth stay on one line:
#    cmake_minimum_required(VERSION 3.20), add_library(mylib src/mylib.c).
#
# 5. Generator expression (genex) multi-line formatting
#    This is the primary focus of this file. Nested $<...> expressions
#    follow the same incremental indentation algorithm as command
#    arguments — each nesting level adds one configurable step
#    (default 2 spaces):
#
#    - Condition genexes ($<$<AND:, $<$<CONFIG:, etc.) open a new indent
#      block; their arguments are each placed on their own line.
#    - The colon suffix separating condition from value starts the value
#      block at the same indent depth as the condition arguments.
#    - Closing > aligns with the opening $< of the genex it closes.
#    - Nested genexes within values follow the same rules recursively.
#    - $<IF:cond,then,else> is also broken across lines with each part
#      indented under the $<IF: opener.
#
# 6. First positional arg on opening line
#    The target name stays on the same line as the command:
#    target_compile_options(mylib
#
# 7. Closing paren on its own line (closingParenNewline: true)
#    Multi-line commands place ) on its own line at column 0.
#
# =============================================================================
CMAKE_MINIMUM_REQUIRED(  version   3.20  )


PROJECT(  NestedExample   languages  C   CXX  )




ADD_LIBRARY(  mylib   src/mylib.c  
)


TARGET_COMPILE_OPTIONS(  mylib private $<$<AND:$<CXX_COMPILER_ID:GNU>,$<VERSION_GREATER_EQUAL:$<CXX_COMPILER_VERSION>,12.0>>:-Wall -Wextra -Wpedantic $<$<CONFIG:Debug>:-O0 -g3 -fsanitize=address,undefined> $<$<CONFIG:Release>:-O3 -flto -DNDEBUG>> $<$<AND:$<CXX_COMPILER_ID:MSVC>,$<NOT:$<BOOL:${DISABLE_WARNINGS}>>>:/W4 /WX $<$<CONFIG:Debug>:/Od /Zi /RTC1> $<$<CONFIG:Release>:/O2 /GL>>

)



Target_Compile_Definitions(  mylib           Public $<$<AND:$<PLATFORM_ID:Linux>,$<OR:$<CONFIG:Debug>,$<BOOL:${FORCE_LOGGING}>>>:ENABLE_LOGGING LOG_LEVEL=$<IF:$<CONFIG:Debug>,3,1>>
)

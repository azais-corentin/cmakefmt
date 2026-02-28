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
CMAKE_MINIMUM_REQUIRED(  version   3.23  )


PROJECT(  MyProjectName  )



ADD_EXECUTABLE(  MyProgram  )
TARGET_SOURCES(  MyProgram   Private   main.cxx  )
TARGET_SOURCES( MyProgram private main1.cxx main2.cxx main3.cxx  main4.cxx main5.cxx main6.cxx             main7.cxx main8.cxx main9.cxx             )

TARGET_SOURCES( MyProgram Private main1.cxx main2.cxx main3.cxx         main4.cxx main5.cxx main6.cxx main7.cxx main8.cxx main9.cxx public main1.cxx main2.cxx main3.cxx main4.cxx main5.cxx main6.cxx main7.cxx     INTERFACE    main1.cxx main2.cxx main3.cxx)

TARGET_SOURCES(MyProgram private      main1.cxx main2.cxx main3.cxx  main4.cxx main5.cxx main6.cxx        main7.cxx)
ADD_LIBRARY(  MyLibrary      )

TARGET_SOURCES(  MyLibrary
		 private
					library_implementation.cxx
		 public
			 file_set  myHeaders
			 type  headers
			 base_dirs
							include
			 files
							include/library_header.h   )

TARGET_SOURCES(  MyLibrary   private  file_set  internalOnlyHeaders  type  headers  files  InternalOnlyHeader.h  Interface  file_set  consumerOnlyHeaders  type  headers  files  ConsumerOnlyHeader.h  public  file_set  publicHeaders  type  headers  files  PublicHeader.h  )


Target_Link_Libraries(MyProgram   Private   MyLibrary)
Target_Link_Libraries(  MyProgram  Private  MyLibrary1  MyLibrary2  MyLibrary3  MyLibrary4  MyLibrary5  MyLibrary6  MyLibrary7  MyLibrary8  MyLibrary9  )


Add_Subdirectory(  SubdirectoryName  )

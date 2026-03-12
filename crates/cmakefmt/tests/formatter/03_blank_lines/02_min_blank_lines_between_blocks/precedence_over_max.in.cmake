# cmakefmt: push { minBlankLinesBetweenBlocks = 2, maxBlankLines = 1 }
set(FOO "bar")
if(FOO)
  message(STATUS "yes")
endif()
# cmakefmt: pop

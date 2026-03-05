# cmakefmt: push { minBlankLinesBetweenBlocks = 1 }
set(FOO "bar")
if(FOO)
  message(STATUS "yes")
endif()
# cmakefmt: pop

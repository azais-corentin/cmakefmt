# cmakefmt: push { minBlankLinesBetweenBlocks = 1 }
set(FOO "bar")
# First comment
# Second comment
# Third comment
if(FOO)
  message(STATUS "yes")
endif()
# cmakefmt: pop

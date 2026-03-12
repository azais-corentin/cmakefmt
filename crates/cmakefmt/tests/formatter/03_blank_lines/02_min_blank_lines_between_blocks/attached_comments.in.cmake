# cmakefmt: push { minBlankLinesBetweenBlocks = 1 }
set(FOO "bar")
# This comment is attached to the if block
# So is this one
if(FOO)
  message(STATUS "yes")
endif()
# cmakefmt: pop

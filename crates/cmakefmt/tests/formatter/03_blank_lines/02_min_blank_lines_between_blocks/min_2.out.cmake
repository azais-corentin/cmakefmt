# cmakefmt: push { minBlankLinesBetweenBlocks = 2 }
set(FOO "bar")


if(FOO)
  message(STATUS "yes")
endif()
set(BAZ "qux")


foreach(item a b c)
  message(STATUS ${item})
endforeach()
# cmakefmt: pop

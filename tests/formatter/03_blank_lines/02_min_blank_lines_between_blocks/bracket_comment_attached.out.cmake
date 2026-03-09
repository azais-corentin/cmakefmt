# cmakefmt: push { minBlankLinesBetweenBlocks = 1 }
set(FOO "bar")

#[==[
Attached comment line 1
Attached comment line 2
]==]
if(FOO)
  message(STATUS "yes")
endif()
# cmakefmt: pop

### {"spaceBeforeParen": ["if", "endif", "set"]}
if(TRUE)
  message(STATUS "hello")
  set(MY_VAR "value")
endif()

project(MyProject)
foreach(item IN ITEMS a b c)
  message(STATUS ${item})
endforeach()

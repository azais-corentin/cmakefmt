# cmakefmt: push { spaceBeforeParen = ["if", "elseif"] }
if(FOO)
  message(STATUS "hello")
elseif(BAR)
  message(STATUS "bar")
endif()
set(X "y")
# cmakefmt: pop

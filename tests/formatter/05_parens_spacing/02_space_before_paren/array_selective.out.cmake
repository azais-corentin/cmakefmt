# cmakefmt: push { spaceBeforeParen = ["if", "elseif", "while"] }
if (TRUE)
  message(STATUS "hello")
endif()
while (FALSE)
endwhile()
# cmakefmt: pop

# cmakefmt: push { minBlankLinesBetweenBlocks = 1 }
set(A "1")

foreach(item IN ITEMS a b c)
  message(STATUS ${item})
endforeach()
set(B "2")

while(FALSE)
  message(STATUS "loop")
endwhile()
set(C "3")

function(my_func)
  message(STATUS "func")
endfunction()
set(D "4")

macro(my_macro)
  message(STATUS "macro")
endmacro()
set(E "5")

block()
  message(STATUS "block")
endblock()
# cmakefmt: pop

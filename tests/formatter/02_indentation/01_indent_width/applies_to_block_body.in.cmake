# cmakefmt: push { indentWidth = 4 }
function(my_func)
message(STATUS "in func")
foreach(item IN ITEMS a b c)
message(STATUS "item: ${item}")
endforeach()
endfunction()
# cmakefmt: pop

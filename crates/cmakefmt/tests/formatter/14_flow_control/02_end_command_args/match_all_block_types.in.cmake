# cmakefmt: push { endCommandArgs = "match" }
foreach(item IN ITEMS a b c)
  message(STATUS ${item})
endforeach()
function(my_func ARG1 ARG2)
  message(STATUS ${ARG1})
endfunction()
# cmakefmt: pop

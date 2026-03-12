# cmakefmt: push { endCommandArgs = "match" }
foreach(item IN ITEMS a b c)
  message(STATUS ${item})
endforeach(item IN ITEMS a b c)
function(my_func ARG1 ARG2)
  message(STATUS ${ARG1})
endfunction(my_func ARG1 ARG2)
# cmakefmt: pop


macro(my_macro ARG)
set(${ARG}_DONE TRUE)
message("done: ${ARG}")
endmacro()

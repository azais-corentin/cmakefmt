# cmakefmt: push { endCommandArgs = "match" }
if((A AND B) OR C)
  message(STATUS "yes")
endif((A AND B) OR C)
# cmakefmt: pop

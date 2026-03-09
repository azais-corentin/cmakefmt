# cmakefmt: push { endCommandArgs = "match" }
if(OUTER)
  function(inner)
    if(INNER)
      message(STATUS "hi")
    endif(INNER)
  endfunction(inner)
endif(OUTER)
# cmakefmt: pop

# cmakefmt: push { endCommandArgs = "match" }
if(OUTER)
  function(do_stuff)
    if(INNER)
      message(STATUS "inner")
    else(INNER)
      message(STATUS "fallback")
    endif(INNER)
  endfunction(do_stuff)
else(OUTER)
  message(STATUS "outer fallback")
endif(OUTER)
# cmakefmt: pop

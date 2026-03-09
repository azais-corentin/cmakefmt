# cmakefmt: push { endCommandArgs = "match" }
if(OUTER)
  function(do_stuff)
    if(INNER)
      message(STATUS "inner")
    else()
      message(STATUS "fallback")
    endif()
  endfunction()
else()
  message(STATUS "outer fallback")
endif()
# cmakefmt: pop

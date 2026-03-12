# cmakefmt: push { endCommandArgs = "match" }
if(OUTER)
  function(inner)
    if(INNER)
      message(STATUS "hi")
    endif()
  endfunction()
endif()
# cmakefmt: pop

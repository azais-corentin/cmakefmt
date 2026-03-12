# cmakefmt: push { minBlankLinesBetweenBlocks = 1 }
function(my_func)
  set(X "y")

  if(X)
    message(STATUS "nested")
  endif()
endfunction()
# cmakefmt: pop

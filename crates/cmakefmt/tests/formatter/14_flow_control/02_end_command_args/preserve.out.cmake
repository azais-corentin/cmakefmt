# cmakefmt: push { endCommandArgs = "preserve" }
if(WIN32)
  message(STATUS "Windows")
endif(WIN32)
function(my_func arg1)
  message(STATUS "func")
endfunction(my_func)
# cmakefmt: pop

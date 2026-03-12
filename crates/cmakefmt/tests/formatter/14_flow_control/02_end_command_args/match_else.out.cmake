# cmakefmt: push { endCommandArgs = "match" }
if(WIN32)
  message(STATUS "Windows")
else(WIN32)
  message(STATUS "Other")
endif(WIN32)
# cmakefmt: pop

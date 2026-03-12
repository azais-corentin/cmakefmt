# cmakefmt: push { endCommandArgs = "match" }
if(WIN32)
  message(STATUS "Windows")
else()
  message(STATUS "Other")
endif()
# cmakefmt: pop

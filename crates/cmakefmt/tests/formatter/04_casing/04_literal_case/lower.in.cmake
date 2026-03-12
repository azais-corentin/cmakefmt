# cmakefmt: push { literalCase = "lower" }
option(USE_FEATURE "Enable" ON)
if(DEFINED result AND OFF)
  message(STATUS "test")
endif()
# cmakefmt: pop

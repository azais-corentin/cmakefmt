# cmakefmt: push { literalCase = "upper" }
option(USE_FEATURE "Enable feature" ON)
if(DEFINED result AND OFF)
  message(STATUS "test")
endif()
# cmakefmt: pop

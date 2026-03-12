# cmakefmt: push { literalCase = "upper" }
if(result strequal "success")
  message(STATUS "ok")
endif()
if(count less 10)
  message(STATUS "small")
endif()
# cmakefmt: pop

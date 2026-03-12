# cmakefmt: push { literalCase = "upper" }
if(result STREQUAL "success")
  message(STATUS "ok")
endif()
if(count LESS 10)
  message(STATUS "small")
endif()
# cmakefmt: pop

# cmakefmt: push { endCommandArgs = "match" }
if(CMAKE_BUILD_TYPE STREQUAL "Debug" AND CMAKE_SYSTEM_NAME STREQUAL "Linux" AND ENABLE_TESTING)
  message(STATUS "debug linux testing")
endif(CMAKE_BUILD_TYPE STREQUAL "Debug" AND CMAKE_SYSTEM_NAME STREQUAL "Linux"
  AND ENABLE_TESTING
)
# cmakefmt: pop

# Before
# cmakefmt: push { alignTrailingComments = true, sortArguments = true, sortKeywordSections = true }
cmake_minimum_required(VERSION 3.20)
project(MyApp VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

add_library(mylib
  include/mylib/main.h
  include/mylib/utils.h
  src/config.cpp
  src/formatter.cpp
  src/main.cpp
  src/parser.cpp
  src/utils.cpp
)
target_link_libraries(mylib PUBLIC Boost::filesystem fmt::fmt spdlog::spdlog)

if(BUILD_TESTING)
  add_executable(mylib_test
    PRIVATE
      tests/test_main.cpp   # Comments
      tests/test_parser.cpp # can be
      tests/test_utils.cpp  # aligned
  )
  target_link_libraries(mylib_test PRIVATE GTest::gtest_main mylib)
endif()

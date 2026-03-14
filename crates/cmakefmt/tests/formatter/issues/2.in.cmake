# Before
# cmakefmt: push { alignTrailingComments = true, sortArguments = true, sortKeywordSections = true }
cmake_minimum_required( VERSION  3.20 )
PROJECT(MyApp VERSION 1.0.0 LANGUAGES CXX)

SET(CMAKE_CXX_STANDARD 17)

add_library(mylib   src/main.cpp src/utils.cpp src/parser.cpp src/formatter.cpp src/config.cpp  include/mylib/main.h include/mylib/utils.h)
TARGET_LINK_LIBRARIES(mylib public fmt::fmt spdlog::spdlog Boost::filesystem)

IF(  BUILD_TESTING)
  add_executable(
    mylib_test
    PRIVATE
    tests/test_utils.cpp# aligned   
    tests/test_parser.cpp # can be
    tests/test_main.cpp # Comments
  )
  target_link_libraries( mylib_test PRIVATE mylib GTest::gtest_main  )
ENDIF(BUILD_TESTING  )
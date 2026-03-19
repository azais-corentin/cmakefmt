# cmakefmt: push { "lineWidth": 100 }
target_include_directories(myproject_c_tests
  PRIVATE ${RUST_BUILD_DIR} ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/../common
)

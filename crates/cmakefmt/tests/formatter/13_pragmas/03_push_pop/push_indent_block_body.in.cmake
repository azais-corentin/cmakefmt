if(ENABLE_TESTS)
  add_subdirectory(tests)
endif()
# cmakefmt: push { indentBlockBody = false }
if(BUILD_DOCS)
add_subdirectory(docs)
endif()
# cmakefmt: pop

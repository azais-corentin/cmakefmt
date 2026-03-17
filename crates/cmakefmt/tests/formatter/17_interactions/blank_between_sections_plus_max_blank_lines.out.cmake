# cmakefmt: push { blankLineBetweenSections = true, maxBlankLines = 0, lineWidth = 40 }
target_link_libraries(MyTarget
  PUBLIC Boost::filesystem

  PRIVATE internal_lib
)
# cmakefmt: pop

# cmakefmt: push { blankLineBetweenSections = true, lineWidth = 40 }
target_link_libraries(MyTarget
  PUBLIC Boost::filesystem

  PRIVATE internal_lib
)
# cmakefmt: pop

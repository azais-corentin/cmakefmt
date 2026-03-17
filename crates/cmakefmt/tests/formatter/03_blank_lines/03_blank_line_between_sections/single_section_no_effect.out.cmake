# cmakefmt: push { blankLineBetweenSections = true, lineWidth = 40 }
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem internal_lib
)
# cmakefmt: pop

# cmakefmt: push { sortKeywordSections = true, blankLineBetweenSections = true, lineWidth = 50 }
target_link_libraries(MyTarget
  PUBLIC
    Boost::filesystem

  PRIVATE
    internal_lib
)
# cmakefmt: pop

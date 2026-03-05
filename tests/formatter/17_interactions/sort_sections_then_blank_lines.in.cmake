# cmakefmt: push { sortKeywordSections = true, blankLineBetweenSections = true, lineWidth = 50 }
target_link_libraries(MyTarget
  PRIVATE
    internal_lib
  PUBLIC
    Boost::filesystem
)
# cmakefmt: pop

# cmakefmt: push { sortKeywordSections = false }
target_link_libraries(MyTarget
  PRIVATE
    internal_lib
  PUBLIC
    Boost::filesystem
)
# cmakefmt: pop

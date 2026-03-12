# cmakefmt: push { sortKeywordSections = true }
target_link_libraries(MyTarget
  PRIVATE
    internal_lib
  PUBLIC
    Boost::filesystem
)
# cmakefmt: pop

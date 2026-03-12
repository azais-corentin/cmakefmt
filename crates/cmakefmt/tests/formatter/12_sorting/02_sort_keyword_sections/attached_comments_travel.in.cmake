# cmakefmt: push { sortKeywordSections = true }
target_link_libraries(MyTarget
  # Private deps
  PRIVATE
    internal_lib
  # Public deps
  PUBLIC
    Boost::filesystem
)
# cmakefmt: pop

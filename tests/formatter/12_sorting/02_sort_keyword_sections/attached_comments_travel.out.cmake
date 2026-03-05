# cmakefmt: push { sortKeywordSections = true }
target_link_libraries(MyTarget
  # Public deps
  PUBLIC
    Boost::filesystem
  # Private deps
  PRIVATE
    internal_lib
)
# cmakefmt: pop

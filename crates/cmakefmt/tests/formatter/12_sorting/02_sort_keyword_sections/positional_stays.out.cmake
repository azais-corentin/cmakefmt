# cmakefmt: push { sortKeywordSections = true }
target_link_libraries(MyTarget
  PUBLIC
    Boost::filesystem
  INTERFACE
    interface_lib
  PRIVATE
    internal_lib
)
# cmakefmt: pop

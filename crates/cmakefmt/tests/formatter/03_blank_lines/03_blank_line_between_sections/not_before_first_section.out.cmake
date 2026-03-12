# cmakefmt: push { blankLineBetweenSections = true }
target_link_libraries(MyTarget
  PUBLIC
    Boost::filesystem

  PRIVATE
    internal_lib
)
# cmakefmt: pop

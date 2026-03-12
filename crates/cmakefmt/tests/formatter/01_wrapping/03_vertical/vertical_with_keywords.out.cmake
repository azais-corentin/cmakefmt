# cmakefmt: push { wrapStyle = "vertical" }
target_link_libraries(MyTarget
  PUBLIC
    Boost::filesystem
  PRIVATE
    internal_lib
    another_lib
)
# cmakefmt: pop

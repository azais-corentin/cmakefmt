# cmakefmt: push { wrapStyle = "vertical" }
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem
    Threads::Threads
  PUBLIC
    some_other_lib
)
# cmakefmt: pop

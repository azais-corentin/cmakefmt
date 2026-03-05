# cmakefmt: push { commandCase = "upper" }
set(FOO "bar")
# cmakefmt: push { keywordCase = "lower" }
target_link_libraries(MyTarget PRIVATE Boost::filesystem)
# cmakefmt: pop
target_link_libraries(MyTarget PRIVATE Boost::filesystem)
# cmakefmt: pop
set(BAZ "qux")

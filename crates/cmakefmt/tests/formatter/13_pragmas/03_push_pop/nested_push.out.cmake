# cmakefmt: push { commandCase = "upper" }
SET(FOO "bar")
# cmakefmt: push { keywordCase = "lower" }
TARGET_LINK_LIBRARIES(MyTarget private Boost::filesystem)
# cmakefmt: pop
TARGET_LINK_LIBRARIES(MyTarget PRIVATE Boost::filesystem)
# cmakefmt: pop
set(BAZ "qux")

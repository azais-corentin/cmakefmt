# cmakefmt: push { indentWidth = 2, continuationIndentWidth = 1, lineWidth = 47 }
target_link_libraries(MyTarget PRIVATE Boost::filesystem Threads::Threads fmt::fmt spdlog::spdlog)
# cmakefmt: pop

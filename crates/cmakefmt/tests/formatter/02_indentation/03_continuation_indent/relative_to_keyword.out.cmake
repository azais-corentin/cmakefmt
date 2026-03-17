# cmakefmt: push { indentWidth = 2, continuationIndentWidth = 4 }
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem Threads::Threads fmt::fmt spdlog::spdlog
)
# cmakefmt: pop

# cmakefmt: push { continuationIndentWidth = 4, lineWidth = 40 }
target_link_libraries(MyTarget
  PRIVATE
      Boost::filesystem
      Threads::Threads
)
# cmakefmt: pop

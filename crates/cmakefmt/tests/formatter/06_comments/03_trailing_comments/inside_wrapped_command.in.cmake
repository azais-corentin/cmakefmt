# cmakefmt: push { alignTrailingComments = true }
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem # fs
    Threads::Threads # threads
    fmt::fmt # formatting
)
# cmakefmt: pop

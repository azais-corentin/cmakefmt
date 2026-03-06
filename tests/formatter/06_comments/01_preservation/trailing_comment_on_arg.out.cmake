target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem # filesystem support
    Threads::Threads
    fmt::fmt
    spdlog::spdlog
)

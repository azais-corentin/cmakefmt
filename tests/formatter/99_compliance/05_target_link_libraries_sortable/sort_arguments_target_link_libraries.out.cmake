# cmakefmt: push { sortArguments = true, lineWidth = 60 }
target_link_libraries(MyTarget
  PUBLIC
    Absl::strings
    Boost::filesystem
    Zlib::Zlib
  PRIVATE
    internal_a
    internal_z
)
# cmakefmt: pop

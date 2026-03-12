# cmakefmt: push { sortArguments = true, lineWidth = 60 }
target_link_libraries(MyTarget
  PUBLIC
    Zlib::Zlib
    Boost::filesystem
    Absl::strings
  PRIVATE
    internal_z
    internal_a
)
# cmakefmt: pop

# cmakefmt: push { lineWidth = 50 }
target_link_libraries(MyTarget
  PUBLIC Boost::filesystem Boost::system
  PRIVATE internal_lib
)

# cmakefmt: push { lineWidth = 40 }
target_link_libraries(MyTarget PRIVATE Boost::filesystem Threads::Threads PUBLIC some_other_lib)
# cmakefmt: pop

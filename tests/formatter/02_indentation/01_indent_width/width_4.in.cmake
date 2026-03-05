# cmakefmt: push { indentWidth = 4 }
target_link_libraries(MyTarget PRIVATE Boost::filesystem Threads::Threads PUBLIC some_other_lib)
# cmakefmt: pop

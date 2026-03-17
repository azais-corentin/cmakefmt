# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_link_libraries(MyLib
  PRIVATE
    $<$<PLATFORM_ID:Linux>:rt>
    $<$<PLATFORM_ID:Linux>:dl>
    Threads::Threads
)
# cmakefmt: pop

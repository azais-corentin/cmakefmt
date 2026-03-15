# cmakefmt: push { sortArguments = true, genexWrap = "cascade", genexClosingAngleNewline = true, lineWidth = 40 }
target_link_libraries(MyLib
  PRIVATE
    $<$<PLATFORM_ID:Linux>:dl>
    $<$<PLATFORM_ID:Linux>:rt>
    Threads::Threads
)
# cmakefmt: pop

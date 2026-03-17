# cmakefmt: push { lineWidth = 54 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:$<$<PLATFORM_ID:Linux>:dl>>)
# cmakefmt: pop

# cmakefmt: push { lineWidth = 40 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:DEBUG_MODE=1;VERBOSE_LOG=1>)
# cmakefmt: pop

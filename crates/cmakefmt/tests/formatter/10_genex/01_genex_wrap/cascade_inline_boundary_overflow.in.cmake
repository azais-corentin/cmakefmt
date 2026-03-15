# cmakefmt: push { lineWidth = 32 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:DEBUG_MODE=1>)
# cmakefmt: pop

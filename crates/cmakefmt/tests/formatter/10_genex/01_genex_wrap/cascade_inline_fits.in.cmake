# cmakefmt: push { lineWidth = 50 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:DEBUG_MODE=1>)
# cmakefmt: pop

# cmakefmt: push { continuationIndentWidth = 4, lineWidth = 34 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:DEBUG_MODE=1>)
# cmakefmt: pop

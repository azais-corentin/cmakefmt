# cmakefmt: push { lineWidth = 35 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:A=1;B=2>)
# cmakefmt: pop

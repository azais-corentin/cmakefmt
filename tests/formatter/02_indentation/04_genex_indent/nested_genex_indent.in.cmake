# cmakefmt: push { genexIndentWidth = 2, lineWidth = 40 }
target_compile_definitions(MyLib PRIVATE $<$<CONFIG:Debug>:$<IF:$<BOOL:${USE_FEATURE}>,FEAT=1,FEAT=0>>)
# cmakefmt: pop

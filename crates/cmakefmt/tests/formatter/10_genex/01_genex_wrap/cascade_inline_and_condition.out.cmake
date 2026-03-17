# cmakefmt: push { lineWidth = 60 }
target_compile_definitions(MyLib
  PRIVATE $<$<AND:$<CONFIG:Debug>,$<PLATFORM_ID:Linux>>:DBG>
)
# cmakefmt: pop

# cmakefmt: push { lineWidth = 50 }
target_compile_definitions(MyLib
  PRIVATE $<$<CONFIG:Debug>:>
)
# cmakefmt: pop

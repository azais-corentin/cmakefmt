# cmakefmt: push { lineWidth = 28 }
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:>
)
# cmakefmt: pop

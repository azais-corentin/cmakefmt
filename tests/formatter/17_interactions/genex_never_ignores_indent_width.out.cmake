# cmakefmt: push { genexWrap = "never", genexIndentWidth = 4, lineWidth = 40 }
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:DEBUG_MODE=1>
)
# cmakefmt: pop

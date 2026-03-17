# cmakefmt: push { lineWidth = 30 }
target_compile_definitions(MyLib
  PRIVATE
    $<$<$<CONFIG:Debug>:1>:value>
)
# cmakefmt: pop

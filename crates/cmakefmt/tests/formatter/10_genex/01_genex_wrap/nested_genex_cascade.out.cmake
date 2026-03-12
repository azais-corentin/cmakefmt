# cmakefmt: push { lineWidth = 50 }
target_compile_definitions(MyLib
  PRIVATE
    $<$<CONFIG:Debug>:
      $<IF:
        $<BOOL:${VERBOSE}>,
        VERBOSE=1,
        VERBOSE=0
      >
    >
)
# cmakefmt: pop

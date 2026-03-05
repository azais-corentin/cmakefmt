# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    ${Z_VAR}
    $<TARGET_FILE:foo>
    alpha.cpp
)
# cmakefmt: pop

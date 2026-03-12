# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    $<TARGET_FILE:foo>
    ${Z_VAR}
    alpha.cpp
)
# cmakefmt: pop

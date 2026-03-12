# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    [=[alpha_bracket]=]
    middle.cpp
)
# cmakefmt: pop

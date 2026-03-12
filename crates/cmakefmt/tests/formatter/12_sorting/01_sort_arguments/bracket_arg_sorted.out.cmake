# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    [=[alpha_bracket]=]
    middle.cpp
    zebra.cpp
)
# cmakefmt: pop

# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    middle.cpp
    zebra.cpp
)
# cmakefmt: pop

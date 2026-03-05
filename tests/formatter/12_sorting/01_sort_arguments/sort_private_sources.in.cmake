# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp
    middle.cpp
)
# cmakefmt: pop

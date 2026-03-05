# cmakefmt: push { sortArguments = true, alignArgGroups = true, lineWidth = 50 }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp
    middle.cpp
)
# cmakefmt: pop

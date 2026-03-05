# cmakefmt: push { sortArguments = true, alignArgGroups = true, lineWidth = 50 }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    middle.cpp
    zebra.cpp
)
# cmakefmt: pop

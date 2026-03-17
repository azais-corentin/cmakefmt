# cmakefmt: push { sortArguments = true, alignArgGroups = true }
target_sources(MyApp
  PRIVATE
    alpha.cpp impl_alpha.cpp
    middle.cpp impl_middle.cpp
    zebra.cpp impl_zebra.cpp
)
# cmakefmt: pop

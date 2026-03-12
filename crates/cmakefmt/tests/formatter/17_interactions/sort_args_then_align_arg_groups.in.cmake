# cmakefmt: push { sortArguments = true, alignArgGroups = true }
target_sources(MyApp
  PRIVATE
    zebra.cpp impl_zebra.cpp
    alpha.cpp impl_alpha.cpp
    middle.cpp impl_middle.cpp
)
# cmakefmt: pop

# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    zebra.cpp # the zebra module
    alpha.cpp # the alpha module
    middle.cpp
)
# cmakefmt: pop

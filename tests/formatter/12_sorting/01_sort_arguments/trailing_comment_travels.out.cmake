# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    alpha.cpp # the alpha module
    middle.cpp
    zebra.cpp # the zebra module
)
# cmakefmt: pop

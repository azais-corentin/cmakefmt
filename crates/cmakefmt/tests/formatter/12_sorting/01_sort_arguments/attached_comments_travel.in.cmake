# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    # z implementation
    zebra.cpp
    alpha.cpp
)
# cmakefmt: pop

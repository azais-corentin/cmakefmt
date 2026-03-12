# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    # z implementation
    zebra.cpp
)
# cmakefmt: pop

# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    zebra.cpp

    # Group 2
    beta.cpp
    delta.cpp
)
# cmakefmt: pop

# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp

    # Group 2
    delta.cpp
    beta.cpp
)
# cmakefmt: pop

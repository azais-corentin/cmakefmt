# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    zebra.cpp

    beta.cpp
    delta.cpp
)
# cmakefmt: pop

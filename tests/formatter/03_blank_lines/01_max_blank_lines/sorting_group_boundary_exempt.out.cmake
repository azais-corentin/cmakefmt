# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    zebra.cpp

    beta.cpp
    gamma.cpp
)
# cmakefmt: pop

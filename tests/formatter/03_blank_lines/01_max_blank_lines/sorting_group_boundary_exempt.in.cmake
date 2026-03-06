# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp

    gamma.cpp
    beta.cpp
)
# cmakefmt: pop

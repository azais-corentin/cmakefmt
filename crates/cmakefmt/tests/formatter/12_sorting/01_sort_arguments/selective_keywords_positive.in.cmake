# cmakefmt: push { sortArguments = ["PRIVATE"], lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp
  PUBLIC
    zeta.hpp
    beta.hpp
)
# cmakefmt: pop

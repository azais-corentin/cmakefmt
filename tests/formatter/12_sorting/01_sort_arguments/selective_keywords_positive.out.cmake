# cmakefmt: push { sortArguments = ["PRIVATE"], lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    zebra.cpp
  PUBLIC
    zeta.hpp
    beta.hpp
)
# cmakefmt: pop

# cmakefmt: push { sortArguments = ["SOURCES"], lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp
)
# cmakefmt: pop

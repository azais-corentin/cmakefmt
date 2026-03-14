# cmakefmt: push { sortArguments = true, lineWidth = 40 }
add_library(mylib
  STATIC
  alpha.cpp
  middle.cpp
  zebra.cpp
)
# cmakefmt: pop

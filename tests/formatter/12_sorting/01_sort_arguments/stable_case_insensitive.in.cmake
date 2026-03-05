# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    Banana.cpp
    apple.cpp
    APPLE.cpp
    cherry.cpp
)
# cmakefmt: pop

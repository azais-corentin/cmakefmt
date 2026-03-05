# cmakefmt: push { sortArguments = true, lineWidth = 40 }
target_sources(MyApp
  PRIVATE
    apple.cpp
    APPLE.cpp
    Banana.cpp
    cherry.cpp
)
# cmakefmt: pop

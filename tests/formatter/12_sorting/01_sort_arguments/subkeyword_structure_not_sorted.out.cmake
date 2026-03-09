# cmakefmt: push { sortArguments = true, lineWidth = 50 }
target_sources(MyLib
  PUBLIC
    FILE_SET HEADERS
      BASE_DIRS include
      FILES
        zebra.h
        alpha.h
)
# cmakefmt: pop

# cmakefmt: push { alignArgGroups = true, lineWidth = 50 }
target_sources(MyLib
  FILE_SET HEADERS
    BASE_DIRS include
    FILES
      include/mylib/core.h
      include/mylib/utils.h
  FILE_SET CXX_MODULES
    BASE_DIRS src
    FILES
      src/core.cppm
      src/utils.cppm
)
# cmakefmt: pop

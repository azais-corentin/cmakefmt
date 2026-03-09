# cmakefmt: push { customKeywords = ["MY_FILES"], sortArguments = true, lineWidth = 40 }
my_command(target
  MY_FILES
    alpha.cpp
    zebra.cpp
)
# cmakefmt: pop

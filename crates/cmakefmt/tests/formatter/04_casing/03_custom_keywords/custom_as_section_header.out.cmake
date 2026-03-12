# cmakefmt: push { customKeywords = ["MY_SOURCES", "MY_HEADERS"], lineWidth = 40 }
my_command(MyTarget
  MY_SOURCES a.cpp b.cpp
  MY_HEADERS a.h b.h
)
# cmakefmt: pop

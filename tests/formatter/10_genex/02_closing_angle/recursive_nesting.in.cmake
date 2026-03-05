# cmakefmt: push { lineWidth = 40 }
target_link_libraries(MyLib PRIVATE $<IF:$<CONFIG:Debug>,debug_lib,release_lib>)
# cmakefmt: pop

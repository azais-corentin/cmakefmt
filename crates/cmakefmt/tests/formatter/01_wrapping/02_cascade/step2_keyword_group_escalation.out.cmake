# cmakefmt: push { lineWidth = 60 }
target_link_libraries(MyTarget
  PRIVATE
    very_long_library_name_one very_long_library_name_two
  PUBLIC short_lib
)
# cmakefmt: pop

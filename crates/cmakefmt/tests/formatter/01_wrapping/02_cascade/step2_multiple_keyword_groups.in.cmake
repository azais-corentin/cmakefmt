# cmakefmt: push { lineWidth = 60 }
target_link_libraries(MyTarget PUBLIC short_lib PRIVATE very_long_library_name_alpha very_long_library_name_beta INTERFACE iface_lib)
# cmakefmt: pop

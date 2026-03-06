# cmakefmt: push { blankLineBetweenSections = true }
target_link_libraries(MyTarget
  PUBLIC
    pub_lib
  INTERFACE
    iface_lib
  PRIVATE
    priv_lib
)
# cmakefmt: pop

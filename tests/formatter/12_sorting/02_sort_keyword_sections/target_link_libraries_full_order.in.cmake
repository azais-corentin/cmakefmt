# cmakefmt: push { sortKeywordSections = true }
target_link_libraries(MyTarget
  PRIVATE
    priv_lib
  INTERFACE
    iface_lib
  PUBLIC
    pub_lib
)
# cmakefmt: pop

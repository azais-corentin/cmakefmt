# cmakefmt: push { alignPropertyValues = true }
set_target_properties(MyTarget PROPERTIES
  CXX_STANDARD              17
  LINK_LIBRARIES            lib1
                            lib2
                            lib3
  POSITION_INDEPENDENT_CODE ON
)
# cmakefmt: pop

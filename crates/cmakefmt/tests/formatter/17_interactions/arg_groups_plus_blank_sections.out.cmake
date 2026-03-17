# cmakefmt: push { alignArgGroups = true, blankLineBetweenSections = true, lineWidth = 50 }
target_link_libraries(MyTarget
  PUBLIC lib_a lib_b

  PRIVATE lib_c lib_d
)
# cmakefmt: pop

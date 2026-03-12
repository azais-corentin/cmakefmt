# cmakefmt: push { sortArguments = true, lineWidth = 80 }
set_target_properties(MyTarget
  PROPERTIES
    OUTPUT_NAME "my_output"
    CXX_STANDARD 17
    AUTOMOC ON
)
# cmakefmt: pop

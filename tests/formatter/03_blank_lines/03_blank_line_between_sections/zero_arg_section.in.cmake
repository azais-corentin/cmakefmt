# cmakefmt: push { blankLineBetweenSections = true, lineWidth = 40 }
add_custom_command(OUTPUT out.cpp COMMAND gen VERBATIM COMMENT "Generating")
# cmakefmt: pop

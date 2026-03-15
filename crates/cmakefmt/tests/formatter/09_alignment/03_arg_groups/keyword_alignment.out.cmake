# cmakefmt: push { alignArgGroups = true, lineWidth = 60 }
add_custom_command(
  OUTPUT  ${CMAKE_CURRENT_BINARY_DIR}/generated.cpp
  COMMAND generator --input schema.json --output
          generated.cpp
  DEPENDS schema.json
  COMMENT "Generating code"
  VERBATIM
)
# cmakefmt: pop

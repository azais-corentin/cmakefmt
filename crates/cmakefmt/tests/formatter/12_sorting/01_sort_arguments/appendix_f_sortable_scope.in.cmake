# cmakefmt: push { sortArguments = true, lineWidth = 200 }
add_custom_command(OUTPUT out.cpp DEPENDS zebra.idl alpha.idl BYPRODUCTS z.tmp a.tmp COMMENT "Generating")
install(TARGETS MyLib RUNTIME DESTINATION bin LIBRARY DESTINATION lib ARCHIVE DESTINATION archive)
# cmakefmt: pop

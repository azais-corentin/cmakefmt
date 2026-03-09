# cmakefmt: push { sortArguments = true, lineWidth = 200 }
add_custom_command(OUTPUT out.cpp DEPENDS alpha.idl zebra.idl BYPRODUCTS a.tmp z.tmp COMMENT "Generating")
install(TARGETS MyLib RUNTIME DESTINATION bin LIBRARY DESTINATION lib ARCHIVE DESTINATION archive)
# cmakefmt: pop

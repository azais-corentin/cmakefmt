# cmakefmt: push { alignArgGroups = true, lineWidth = 60 }
install(TARGETS
  MyLib      RUNTIME DESTINATION bin
  MyOtherLib RUNTIME DESTINATION lib
  MyPlugin   LIBRARY DESTINATION plugins
)
# cmakefmt: pop

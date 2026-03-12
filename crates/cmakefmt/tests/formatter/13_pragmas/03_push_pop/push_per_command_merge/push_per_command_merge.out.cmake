if (TRUE)
  SET(FOO "bar")
endif()
# cmakefmt: push { perCommandConfig = { set = { commandCase = "lower" } } }
if (TRUE)
  set(FOO "bar")
endif()
# cmakefmt: pop

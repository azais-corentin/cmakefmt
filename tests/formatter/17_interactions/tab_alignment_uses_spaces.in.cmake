# cmakefmt: push { useTabs = true, alignConsecutiveSet = true }
if(TRUE)
  set(FOO "bar")
  set(LONGER "value")
endif()
# cmakefmt: pop

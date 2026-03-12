# cmakefmt: push { perCommandConfig = { set = { commandCase = "upper" } } }
# cmakefmt: push { commandCase = "lower" }
set(FOO "bar")
# cmakefmt: pop
set(BAZ "qux")
# cmakefmt: pop

# cmakefmt: push { perCommandConfig = { set = { commandCase = "upper" } } }
# cmakefmt: push { commandCase = "lower" }
set(FOO "bar")
# cmakefmt: pop
SET(BAZ "qux")
# cmakefmt: pop

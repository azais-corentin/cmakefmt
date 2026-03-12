# cmakefmt: off
set(  FOO   "bar"  )
# cmakefmt: on
# cmakefmt: push { commandCase = "upper" }
set(BAZ   "qux")
# cmakefmt: pop
# cmakefmt: skip
message( STATUS  "hello" )

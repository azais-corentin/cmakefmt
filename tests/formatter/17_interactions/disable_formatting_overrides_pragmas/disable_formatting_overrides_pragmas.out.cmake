# cmakefmt: push { commandCase = "upper" }
set(  FOO   "bar"  )
# cmakefmt: pop
# cmakefmt: off
message( STATUS  "hello" )
# cmakefmt: on

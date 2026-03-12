# cmakefmt: push { alignTrailingComments = true }
set(FOO "bar")      # group 1
set(BAZ_LONG "qux") # group 1

set(X "y")              # group 2
set(ANOTHER_LONG "val") # group 2
# cmakefmt: pop

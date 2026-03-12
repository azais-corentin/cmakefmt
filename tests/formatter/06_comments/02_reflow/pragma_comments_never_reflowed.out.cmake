# cmakefmt: push { commentPreservation = "reflow", lineWidth = 40 }
# This is a long comment that will be
# reflowed to fit within the line
# width.
# cmakefmt: push { indentWidth = 4 }
set(FOO "bar")
# cmakefmt: pop
# cmakefmt: pop

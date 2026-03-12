# cmakefmt: push { wrapArgThreshold = 3 }
target_link_libraries(MyTarget
  PRIVATE
    foo
    bar
)
# cmakefmt: pop

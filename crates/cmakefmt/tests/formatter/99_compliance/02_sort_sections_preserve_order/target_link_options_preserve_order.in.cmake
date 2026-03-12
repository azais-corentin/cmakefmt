# cmakefmt: push { sortKeywordSections = true }
target_link_options(MyTarget
  PRIVATE
    -Wl,--no-undefined
  INTERFACE
    -Wl,--as-needed
  PUBLIC
    -Wl,-z,relro
)
# cmakefmt: pop

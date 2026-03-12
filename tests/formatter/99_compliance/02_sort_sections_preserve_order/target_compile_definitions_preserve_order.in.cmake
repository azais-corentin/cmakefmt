# cmakefmt: push { sortKeywordSections = true }
target_compile_definitions(MyTarget
  PRIVATE
    FOO_PRIVATE
  PUBLIC
    FOO_PUBLIC
  INTERFACE
    FOO_INTERFACE
)
# cmakefmt: pop

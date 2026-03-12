# cmakefmt: push { sortKeywordSections = true }
target_compile_options(MyTarget
  INTERFACE
    -Wall
  PRIVATE
    -Werror
  PUBLIC
    -O2
)
# cmakefmt: pop

# cmakefmt: push { sortKeywordSections = true }
target_include_directories(MyTarget
  INTERFACE
    include/interface
  PUBLIC
    include/public
  PRIVATE
    src/private
)
# cmakefmt: pop

# cmakefmt: push { sortKeywordSections = true }
my_custom_command(target
  PRIVATE
    lib1
  PUBLIC
    pub_lib
)
# cmakefmt: pop

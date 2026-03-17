# cmakefmt: push { customKeywords = ["MY_SECTION"], blankLineBetweenSections = true }
my_command(target
  PRIVATE lib1

  MY_SECTION custom_arg
)
# cmakefmt: pop

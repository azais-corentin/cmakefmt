# cmakefmt: push { customKeywords = ["MY_SECTION"], blankLineBetweenSections = true }
my_command(target
  PRIVATE
    lib1
  my_section
    custom_arg
)
# cmakefmt: pop

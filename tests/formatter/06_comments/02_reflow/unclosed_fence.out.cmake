# cmakefmt: push { commentPreservation = "reflow", lineWidth = 40 }
# Some text before.
# ```
# this_is_code_that_will_not_be_reflowed_even_though_it_is_very_long
# more code here without closing fence
set(FOO "bar")
# cmakefmt: pop

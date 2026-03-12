# cmakefmt: push { sortKeywordSections = true }
block(SCOPE_FOR VARIABLES PROPAGATE myvar)
  set(myvar 42)
endblock()
# cmakefmt: pop

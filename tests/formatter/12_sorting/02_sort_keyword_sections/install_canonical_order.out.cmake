# cmakefmt: push { sortKeywordSections = true, lineWidth = 40 }
install(TARGETS MyLib
  ARCHIVE
    DESTINATION lib
  LIBRARY
    DESTINATION lib
  RUNTIME
    DESTINATION bin
)
# cmakefmt: pop

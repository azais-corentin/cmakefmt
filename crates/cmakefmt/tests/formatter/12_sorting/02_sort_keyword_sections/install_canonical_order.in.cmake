# cmakefmt: push { sortKeywordSections = true, lineWidth = 40 }
install(TARGETS MyLib
  RUNTIME
    DESTINATION bin
  ARCHIVE
    DESTINATION lib
  LIBRARY
    DESTINATION lib
)
# cmakefmt: pop

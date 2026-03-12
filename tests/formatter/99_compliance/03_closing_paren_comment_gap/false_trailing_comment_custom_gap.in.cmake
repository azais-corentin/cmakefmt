# cmakefmt: push { closingParenNewline = false, commentGap = 3, lineWidth = 60 }
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem # link fs
)
# cmakefmt: pop

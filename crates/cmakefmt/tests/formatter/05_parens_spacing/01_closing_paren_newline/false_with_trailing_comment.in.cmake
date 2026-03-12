# cmakefmt: push { closingParenNewline = false, lineWidth = 40 }
target_link_libraries(MyTarget
  PRIVATE
    Boost::filesystem # link fs
)
# cmakefmt: pop

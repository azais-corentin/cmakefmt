# cmakefmt: push { indentStyle = "tab", indentWidth = 4, continuationIndentWidth = 6 }
target_link_libraries(MyTarget
	PRIVATE Boost::filesystem Threads::Threads
	PUBLIC some_other_lib
)
# cmakefmt: pop

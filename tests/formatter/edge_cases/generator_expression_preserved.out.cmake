target_link_libraries(foo $<$<BOOL:${flag}>:bar>)

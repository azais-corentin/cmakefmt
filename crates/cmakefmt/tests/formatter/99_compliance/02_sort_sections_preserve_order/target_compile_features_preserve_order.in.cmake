# cmakefmt: push { sortKeywordSections = true }
target_compile_features(MyTarget
  PUBLIC
    cxx_std_20
  PRIVATE
    c_std_11
  INTERFACE
    cxx_std_17
)
# cmakefmt: pop

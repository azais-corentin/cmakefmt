# cmakefmt: push { sortKeywordSections = true }
target_sources(MyLib
  PRIVATE
    src/private_impl.cpp
  PUBLIC
    include/mylib/public_api.hpp
  INTERFACE
    interface_only.hpp
)
# cmakefmt: pop

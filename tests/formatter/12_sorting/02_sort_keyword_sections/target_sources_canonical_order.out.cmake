# cmakefmt: push { sortKeywordSections = true }
target_sources(MyLib
  PUBLIC
    include/mylib/public_api.hpp
  INTERFACE
    interface_only.hpp
  PRIVATE
    src/private_impl.cpp
)
# cmakefmt: pop

set(FOO "bar")
target_link_libraries(MyTarget
  PRIVATE Boost::filesystem Threads::Threads OpenSSL::SSL OpenSSL::Crypto
)
set(BAZ "qux")

# cmakefmt: push { ignoreCommands = ["ExternalProject_Add", "FetchContent_Declare"] }
ExternalProject_Add(googletest
    GIT_REPOSITORY  https://github.com/google/googletest.git
)
FetchContent_Declare(fmt
    GIT_REPOSITORY  https://github.com/fmtlib/fmt.git
)
set(  FOO   "bar"  )
# cmakefmt: pop

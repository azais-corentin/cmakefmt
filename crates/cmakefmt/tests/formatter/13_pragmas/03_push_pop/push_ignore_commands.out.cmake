# cmakefmt: push { ignoreCommands = ["ExternalProject_Add"] }
ExternalProject_Add(googletest
    GIT_REPOSITORY  https://github.com/google/googletest.git
    GIT_TAG         release-1.12.1
)
message(STATUS "formatted")
# cmakefmt: pop
message(STATUS "also formatted")

ExternalProject_Add(googletest
    GIT_TAG         release-1.12.1
    GIT_REPOSITORY  https://github.com/google/googletest.git
)
set(FOO "bar")

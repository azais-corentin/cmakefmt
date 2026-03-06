# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    alpha.cpp
    zebra.cpp

    # This is an unattached comment (blank line above)
    beta.cpp
    gamma.cpp
)
# cmakefmt: pop

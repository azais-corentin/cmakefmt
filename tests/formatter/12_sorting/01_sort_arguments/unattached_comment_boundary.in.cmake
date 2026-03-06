# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    zebra.cpp
    alpha.cpp

    # This is an unattached comment (blank line above)
    gamma.cpp
    beta.cpp
)
# cmakefmt: pop

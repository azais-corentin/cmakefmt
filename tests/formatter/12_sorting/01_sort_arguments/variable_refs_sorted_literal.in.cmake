# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    ${ZEBRA_SOURCES}
    ${ALPHA_SOURCES}
    main.cpp
)
# cmakefmt: pop

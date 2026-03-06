# cmakefmt: push { sortArguments = true }
target_sources(MyApp
  PRIVATE
    ${ALPHA_SOURCES}
    ${ZEBRA_SOURCES}
    main.cpp
)
# cmakefmt: pop

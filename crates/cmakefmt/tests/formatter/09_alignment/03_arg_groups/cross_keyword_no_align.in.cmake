# cmakefmt: push { alignArgGroups = true }
target_sources(MyApp
  PUBLIC
    header1.h header1_impl.cpp
    header2.h header2_impl.cpp
  PRIVATE
    main.cpp main_utils.cpp
    test.cpp test_utils.cpp
)
# cmakefmt: pop

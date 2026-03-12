# cmakefmt: push { lineWidth = 50 }
target_sources(MyTarget
  PRIVATE
    main.cpp
    utils.cpp
    helpers.cpp
    extra_long_filename.cpp
)
# cmakefmt: pop

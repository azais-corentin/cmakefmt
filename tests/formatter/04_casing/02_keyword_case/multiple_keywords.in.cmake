cmake_minimum_required(version 3.20)
find_package(Boost required components filesystem system)
target_link_libraries(MyTarget private Boost::filesystem)

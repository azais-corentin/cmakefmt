cmake_minimum_required(VERSION 3.20)
find_package(Boost REQUIRED COMPONENTS filesystem system)
target_link_libraries(MyTarget PRIVATE Boost::filesystem)

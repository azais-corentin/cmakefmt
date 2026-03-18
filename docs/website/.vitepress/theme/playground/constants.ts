export const WASM_URL: string = "/cmakefmt/cmakefmt-dprint.wasm";

export const DEFAULT_INPUT: string = `cmake_minimum_required(VERSION 3.20)
project(MyProject LANGUAGES CXX)

set(SOURCES
  src/main.cpp
  src/util.cpp    src/helper.cpp
)

add_executable(myapp \${SOURCES})

target_include_directories(myapp PRIVATE \${CMAKE_CURRENT_SOURCE_DIR}/include)

if(CMAKE_BUILD_TYPE STREQUAL "Release")
target_compile_options(myapp PRIVATE -O2 -Wall -Wextra)
else()
  target_compile_options(myapp PRIVATE -g -Wall)
endif()
`;

export const DEFAULT_CONFIG: Record<string, unknown> = {
  lineWidth: 80,
  indentWidth: 2,
};

export const DEFAULT_CONFIG_JSON: string = JSON.stringify(DEFAULT_CONFIG, null, 2);

cmake_minimum_required(VERSION 3.10)

project(typeskia VERSION 1.0)

cmake_path(GET CMAKE_SOURCE_DIR PARENT_PATH PRJ_ROOT)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(SKIA_SDK "${PRJ_ROOT}/third-party/skia")

add_subdirectory(src)
add_subdirectory(tests)

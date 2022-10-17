cmake_minimum_required(VERSION 3.15)

set(LIBRARY_OUTPUT_PATH ${CMAKE_SOURCE_DIR}/dist)

add_library(typeskia typeskia.cpp)

target_include_directories(typeskia PUBLIC ${SKIA_SDK})

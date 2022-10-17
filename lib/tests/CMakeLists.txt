include(GoogleTest)

set(EXECUTABLE_OUTPUT_PATH ${PRJ_ROOT}/.tests)

find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

file(GLOB_RECURSE TEST_SOURCES LIST_DIRECTORIES false *.h *.cpp)

add_executable(test ${TEST_SOURCES})

target_link_libraries(test PUBLIC ${PRJ_ROOT}/lib/dist/libtypeskia.a)
target_link_libraries(test PUBLIC GTest::gtest GTest::gtest_main)
target_link_libraries(test LINK_PUBLIC ${SKIA_SDK}/out/Debug/libskia.a)
target_link_libraries(test LINK_PUBLIC ${SKIA_SDK}/out/Debug/libskshaper.a)

if (APPLE)
    target_link_libraries(test LINK_PUBLIC "-framework CoreServices")
    target_link_libraries(test LINK_PUBLIC "-framework CoreGraphics")
    target_link_libraries(test LINK_PUBLIC "-framework CoreText")
    target_link_libraries(test LINK_PUBLIC "-framework CoreFoundation")
endif()


gtest_add_tests(test "" AUTO)

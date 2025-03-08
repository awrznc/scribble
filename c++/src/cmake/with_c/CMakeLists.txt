cmake_minimum_required(VERSION 3.1)
project(piyo_project C CXX)

# static library
add_library(piyo STATIC ${CMAKE_CURRENT_SOURCE_DIR}/src/piyo.cpp)
target_compile_options(piyo PRIVATE "$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")
target_include_directories(piyo PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include/)

# test
add_executable(piyo_test ${CMAKE_CURRENT_SOURCE_DIR}/test/main.c)
target_include_directories(piyo_test PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include/)
target_compile_options(piyo_test PRIVATE "$<$<C_COMPILER_ID:MSVC>:/utf-8>")
target_link_libraries(piyo_test PRIVATE piyo)

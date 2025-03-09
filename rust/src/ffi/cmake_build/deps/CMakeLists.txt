cmake_minimum_required(VERSION 3.1)
project(piyo_project C CXX)

# static library
add_library(piyo STATIC ${CMAKE_CURRENT_SOURCE_DIR}/src/piyo.cpp)
target_compile_options(piyo PRIVATE "$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")
target_include_directories(piyo PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include/)

# cargo 用
install(TARGETS piyo DESTINATION .)

cmake_minimum_required(VERSION 3.10)

project(sample CXX)
set(CMAKE_CXX_STANDARD 17)

# msgpack
set(MSGPACK_BUILD_TESTS OFF)
set(MSGPACK_BUILD_EXAMPLES OFF)
add_subdirectory(msgpack-c)
include_directories(${msgpack_SOURCE_DIR}/include)

# main build
add_executable(main_app main.cpp)

# preprocess
find_package(Ruby)
add_custom_command(
    OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/pair_vector.mpack"
    COMMAND bundle exec ruby "${CMAKE_CURRENT_SOURCE_DIR}/serialize.rb" "${CMAKE_CURRENT_BINARY_DIR}/pair_vector.mpack"
    DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/serialize.rb")
add_custom_target(generate_mpack_file DEPENDS "${CMAKE_CURRENT_BINARY_DIR}/pair_vector.mpack")

# generate_mpack_file の後に main_app のビルドを実行する
add_dependencies( main_app generate_mpack_file )

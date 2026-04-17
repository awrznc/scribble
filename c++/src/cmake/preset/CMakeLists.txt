cmake_minimum_required(VERSION 4.3)
project(piyo_project CXX)

# define config
set( PIYODEF "PIYO_UNKNOWN" )
if(${CMAKE_GENERATOR_PLATFORM} STREQUAL "Win32")
    set( PIYODEF "PIYO_WIN32" )
elseif(${CMAKE_GENERATOR_PLATFORM} STREQUAL "x64")
    set( PIYODEF "PIYO_X64" )
endif()

# build
add_executable(piyo ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp)
target_compile_definitions( piyo PRIVATE ${PIYODEF} )
target_compile_options(
    piyo PRIVATE
    $<$<C_COMPILER_ID:MSVC>:/utf-8>
    $<$<C_COMPILER_ID:Clang>:"">
)

# VisualStudioのデフォルト実行する対象
set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT piyo)

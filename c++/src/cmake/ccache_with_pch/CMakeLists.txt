cmake_minimum_required( VERSION 3.10 )
project( ccache_with_pch CXX )

add_executable( app
  ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/src/app.cpp
)

# pch
target_precompile_headers( app PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/src/deka.hpp" )

# ccache
find_program(CCACHE_FOUND ccache)
if(CCACHE_FOUND)
  target_compile_options( app PRIVATE -fpch-preprocess )
  set_property(
    TARGET app
    PROPERTY RULE_LAUNCH_COMPILE "CCACHE_DEBUG=true CCACHE_SLOPPINESS=pch_defines,time_macros ccache"
  )
endif()

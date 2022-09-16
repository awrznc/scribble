cmake_minimum_required( VERSION 3.10 )
project( ccache_or_pch CXX )

# ccache
find_program(CCACHE_FOUND ccache)
if(CCACHE_FOUND)
  set(CMAKE_CXX_COMPILER_LAUNCHER ccache)
  set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE ccache)
else()
  # pch
  target_precompile_headers( main_app PRIVATE
    "$<$<COMPILE_LANGUAGE:CXX>:${CMAKE_CURRENT_SOURCE_DIR}/src/deka.hpp>"
  )
endif()

add_executable( main_app
  ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/src/app.cpp
)

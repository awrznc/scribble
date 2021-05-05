cmake_minimum_required(VERSION 3.9)
project(sample CXX)

# If you require libthread
find_package(Threads REQUIRED)

add_executable(main_app src/main.cpp)

install(TARGETS main_app DESTINATION ${CMAKE_INSTALL_PREFIX}/bin/)

# cpack(debian)
set(CPACK_GENERATOR "DEB")
set(CPACK_DEBIAN_PACKAGE_MAINTAINER "awrznc")
set(CPACK_PACKAGE_VERSION "1.0.0")
include(CPack)

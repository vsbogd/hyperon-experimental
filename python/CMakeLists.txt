cmake_minimum_required(VERSION 3.19)
project(hyperonpy)

option(GIT "Adds git features to hyperon library; requires OpenSSL and Zlib" ON)

enable_testing()
add_custom_target(check COMMAND ${CMAKE_CTEST_COMMAND} --output-on-failure)

# Fix behavior of CMAKE_CXX_STANDARD when targeting macOS.
if (POLICY CMP0025)
    cmake_policy(SET CMP0025 NEW)
endif ()
set(CMAKE_CXX_STANDARD 11)

# The default value ("FIRST") prefers the installation with the highest
# version. "ONLY" sticks to a virtualenv even when its version is smaller
# which is usually expected by an user.
if (NOT DEFINED Python3_FIND_VIRTUALENV)
    set(Python3_FIND_VIRTUALENV "ONLY")
endif()
# Development.Embed is not supported by cibuildwheel environment
find_package(Python3 3.7 REQUIRED COMPONENTS Interpreter Development.Module)
message(STATUS "Python native modules installation path (Python3_SITEARCH): ${Python3_SITEARCH}")
message(STATUS "Python modules installation path (Python3_SITELIB): ${Python3_SITELIB}")

if(GIT)
    find_package(OpenSSL REQUIRED)
    find_package(ZLIB REQUIRED)
endif()

find_package(pybind11 REQUIRED)
find_package(optional-lite REQUIRED)
include_directories(${nonstd_INCLUDE_DIRS})
find_package(hyperonc REQUIRED HINTS ${HYPERONC_INSTALL_PREFIX})
include_directories(${hyperonc_INCLUDE_DIRS})

if(APPLE)
    # macOS specific libraries
    set(PLATFORM_LIBRARIES "-framework CoreFoundation" "-framework Security")
endif()

pybind11_add_module(hyperonpy MODULE ./hyperonpy.cpp)
target_link_libraries(hyperonpy PRIVATE "${hyperonc_STATIC_LIBRARY}" nonstd::optional-lite)
if(GIT)
    target_link_libraries(hyperonpy PUBLIC ${PLATFORM_LIBRARIES} openssl::openssl ZLIB::ZLIB)
endif()

set(PYTHONPATH "${CMAKE_CURRENT_SOURCE_DIR}")
add_subdirectory(tests)

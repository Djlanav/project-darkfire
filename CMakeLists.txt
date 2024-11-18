cmake_minimum_required(VERSION 3.28)

project(darkfire-gdextension)

# Set variables
set(GODOT_CPP_PATH ${CMAKE_SOURCE_DIR}/godot-cpp)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

set(GODOT_CPP_SRC ${GODOT_CPP_PATH}/src)

# Include the Godot-C++ bindings
include_directories(${GODOT_CPP_PATH}/gen/include)
include_directories(${GODOT_CPP_PATH}/include)
include_directories(${GODOT_CPP_PATH}/gdextension)

# Add module library
add_library(darkfire_module MODULE
        src/register_types.cpp
        src/register_types.h
        src/SFXManager.cpp
        src/SFXManager.h
        src/Utilities.cpp
        src/Utilities.h)

# Link against the Godot C++ bindings and Godot engine
target_link_libraries(darkfire_module godot-cpp)
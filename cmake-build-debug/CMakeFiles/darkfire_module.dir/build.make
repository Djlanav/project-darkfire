# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.29

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /home/cmh/.local/share/JetBrains/Toolbox/apps/clion/bin/cmake/linux/x64/bin/cmake

# The command to remove a file.
RM = /home/cmh/.local/share/JetBrains/Toolbox/apps/clion/bin/cmake/linux/x64/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = "/home/cmh/Documents/Godot Projects/fps-project"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug"

# Include any dependencies generated for this target.
include CMakeFiles/darkfire_module.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/darkfire_module.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/darkfire_module.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/darkfire_module.dir/flags.make

CMakeFiles/darkfire_module.dir/src/register_types.cpp.o: CMakeFiles/darkfire_module.dir/flags.make
CMakeFiles/darkfire_module.dir/src/register_types.cpp.o: /home/cmh/Documents/Godot\ Projects/fps-project/src/register_types.cpp
CMakeFiles/darkfire_module.dir/src/register_types.cpp.o: CMakeFiles/darkfire_module.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir="/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug/CMakeFiles" --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/darkfire_module.dir/src/register_types.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/darkfire_module.dir/src/register_types.cpp.o -MF CMakeFiles/darkfire_module.dir/src/register_types.cpp.o.d -o CMakeFiles/darkfire_module.dir/src/register_types.cpp.o -c "/home/cmh/Documents/Godot Projects/fps-project/src/register_types.cpp"

CMakeFiles/darkfire_module.dir/src/register_types.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/darkfire_module.dir/src/register_types.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "/home/cmh/Documents/Godot Projects/fps-project/src/register_types.cpp" > CMakeFiles/darkfire_module.dir/src/register_types.cpp.i

CMakeFiles/darkfire_module.dir/src/register_types.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/darkfire_module.dir/src/register_types.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "/home/cmh/Documents/Godot Projects/fps-project/src/register_types.cpp" -o CMakeFiles/darkfire_module.dir/src/register_types.cpp.s

CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o: CMakeFiles/darkfire_module.dir/flags.make
CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o: /home/cmh/Documents/Godot\ Projects/fps-project/src/SFXManager.cpp
CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o: CMakeFiles/darkfire_module.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir="/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug/CMakeFiles" --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o -MF CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o.d -o CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o -c "/home/cmh/Documents/Godot Projects/fps-project/src/SFXManager.cpp"

CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E "/home/cmh/Documents/Godot Projects/fps-project/src/SFXManager.cpp" > CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.i

CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S "/home/cmh/Documents/Godot Projects/fps-project/src/SFXManager.cpp" -o CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.s

# Object files for target darkfire_module
darkfire_module_OBJECTS = \
"CMakeFiles/darkfire_module.dir/src/register_types.cpp.o" \
"CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o"

# External object files for target darkfire_module
darkfire_module_EXTERNAL_OBJECTS =

libdarkfire_module.so: CMakeFiles/darkfire_module.dir/src/register_types.cpp.o
libdarkfire_module.so: CMakeFiles/darkfire_module.dir/src/SFXManager.cpp.o
libdarkfire_module.so: CMakeFiles/darkfire_module.dir/build.make
libdarkfire_module.so: CMakeFiles/darkfire_module.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir="/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug/CMakeFiles" --progress-num=$(CMAKE_PROGRESS_3) "Linking CXX shared module libdarkfire_module.so"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/darkfire_module.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/darkfire_module.dir/build: libdarkfire_module.so
.PHONY : CMakeFiles/darkfire_module.dir/build

CMakeFiles/darkfire_module.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/darkfire_module.dir/cmake_clean.cmake
.PHONY : CMakeFiles/darkfire_module.dir/clean

CMakeFiles/darkfire_module.dir/depend:
	cd "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug" && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" "/home/cmh/Documents/Godot Projects/fps-project" "/home/cmh/Documents/Godot Projects/fps-project" "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug" "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug" "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug/CMakeFiles/darkfire_module.dir/DependInfo.cmake" "--color=$(COLOR)"
.PHONY : CMakeFiles/darkfire_module.dir/depend

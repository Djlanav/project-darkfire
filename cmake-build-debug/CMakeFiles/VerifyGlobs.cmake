# CMAKE generated file: DO NOT EDIT!
# Generated by CMake Version 3.29
cmake_policy(SET CMP0009 NEW)

# HEADERS at godot-cpp/cmake/godotcpp.cmake:160 (file)
file(GLOB_RECURSE NEW_GLOB LIST_DIRECTORIES false "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/*.h**")
set(OLD_GLOB
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/classes/editor_plugin_registration.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/classes/ref.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/classes/wrapped.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/binder_common.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/builtin_ptrcall.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/class_db.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/defs.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/engine_ptrcall.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/error_macros.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/math.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/memory.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/method_bind.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/method_ptrcall.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/mutex_lock.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/object.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/object_id.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/property_info.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/core/type_info.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/godot.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/cowdata.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/hash_map.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/hash_set.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/hashfuncs.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/list.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/local_vector.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/pair.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/rb_map.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/rb_set.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/rid_owner.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/safe_refcount.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/search_array.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/self_list.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/sort_array.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/spin_lock.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/thread_work_pool.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/vector.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/vmap.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/templates/vset.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/aabb.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/array_helpers.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/basis.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/callable_custom.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/callable_method_pointer.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/char_string.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/char_utils.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/color.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/color_names.inc.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/plane.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/projection.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/quaternion.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/rect2.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/rect2i.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/transform2d.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/transform3d.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/typed_array.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/variant.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/vector2.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/vector2i.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/vector3.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/vector3i.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/vector4.hpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/include/godot_cpp/variant/vector4i.hpp"
  )
if(NOT "${NEW_GLOB}" STREQUAL "${OLD_GLOB}")
  message("-- GLOB mismatch!")
  file(TOUCH_NOCREATE "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug/CMakeFiles/cmake.verify_globs")
endif()

# SOURCES at godot-cpp/cmake/godotcpp.cmake:159 (file)
file(GLOB_RECURSE NEW_GLOB LIST_DIRECTORIES false "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/*.c**")
set(OLD_GLOB
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/classes/editor_plugin_registration.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/classes/low_level.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/classes/wrapped.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/core/class_db.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/core/error_macros.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/core/memory.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/core/method_bind.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/core/object.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/godot.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/aabb.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/basis.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/callable_custom.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/callable_method_pointer.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/char_string.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/color.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/packed_arrays.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/plane.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/projection.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/quaternion.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/rect2.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/rect2i.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/transform2d.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/transform3d.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/variant.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/vector2.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/vector2i.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/vector3.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/vector3i.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/vector4.cpp"
  "/home/cmh/Documents/Godot Projects/fps-project/godot-cpp/src/variant/vector4i.cpp"
  )
if(NOT "${NEW_GLOB}" STREQUAL "${OLD_GLOB}")
  message("-- GLOB mismatch!")
  file(TOUCH_NOCREATE "/home/cmh/Documents/Godot Projects/fps-project/cmake-build-debug/CMakeFiles/cmake.verify_globs")
endif()

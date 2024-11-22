#![doc = "Sidecar module for class [`TileMapLayer`][crate::classes::TileMapLayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMapLayer` enums](https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, ObjectArg, ObjectCow, PtrcallSignatureTuple, RefArg, VarcallSignatureTuple
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `TileMapLayer.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`tile_map_layer`][crate::classes::tile_map_layer]: sidecar module with related enum/flag types\n* [`ITileMapLayer`][crate::classes::ITileMapLayer]: virtual methods\n\n\nSee also [Godot docs for `TileMapLayer`](https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TileMapLayer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMapLayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileMapLayer`][crate::classes::TileMapLayer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileMapLayer` methods](https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMapLayer: crate::obj::GodotClass < Base = TileMapLayer > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn use_tile_data_runtime_update(&mut self, coords: Vector2i,) -> bool {
            unimplemented !()
        }
        fn tile_data_runtime_update(&mut self, coords: Vector2i, tile_data: Option < Gd < crate::classes::TileData > >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    impl TileMapLayer {
        pub(crate) fn set_cell_full(&mut self, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallSig = ((), Vector2i, i32, Vector2i, i32);
            let args = (coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cell_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cell(&mut self, coords: Vector2i,) {
            self.set_cell_ex(coords,) . done()
        }
        #[inline]
        pub fn set_cell_ex < 'a > (&'a mut self, coords: Vector2i,) -> ExSetCell < 'a > {
            ExSetCell::new(self, coords,)
        }
        pub fn erase_cell(&mut self, coords: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "erase_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fix_invalid_tiles(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "fix_invalid_tiles", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_source_id(&self, coords: Vector2i,) -> i32 {
            type CallSig = (i32, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_atlas_coords(&self, coords: Vector2i,) -> Vector2i {
            type CallSig = (Vector2i, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_atlas_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_alternative_tile(&self, coords: Vector2i,) -> i32 {
            type CallSig = (i32, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_alternative_tile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_tile_data(&self, coords: Vector2i,) -> Option < Gd < crate::classes::TileData > > {
            type CallSig = (Option < Gd < crate::classes::TileData > >, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_cell_tile_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_used_cells(&self,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_used_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_used_cells_by_id_full(&self, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >, i32, Vector2i, i32);
            let args = (source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_used_cells_by_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_used_cells_by_id_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_used_cells_by_id(&self,) -> Array < Vector2i > {
            self.get_used_cells_by_id_ex() . done()
        }
        #[inline]
        pub fn get_used_cells_by_id_ex < 'a > (&'a self,) -> ExGetUsedCellsById < 'a > {
            ExGetUsedCellsById::new(self,)
        }
        pub fn get_used_rect(&self,) -> Rect2i {
            type CallSig = (Rect2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_used_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pattern(&mut self, coords_array: &Array < Vector2i >,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::TileMapPattern > >, RefArg < 'a0, Array < Vector2i > >);
            let args = (RefArg::new(coords_array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pattern(&mut self, position: Vector2i, pattern: impl AsObjectArg < crate::classes::TileMapPattern >,) {
            type CallSig = ((), Vector2i, ObjectArg < crate::classes::TileMapPattern >);
            let args = (position, pattern.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_cells_terrain_connect_full(&mut self, cells: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Vector2i > >, i32, i32, bool);
            let args = (cells, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_cells_terrain_connect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cells_terrain_connect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cells_terrain_connect(&mut self, cells: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_connect_ex(cells, terrain_set, terrain,) . done()
        }
        #[inline]
        pub fn set_cells_terrain_connect_ex < 'a > (&'a mut self, cells: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainConnect < 'a > {
            ExSetCellsTerrainConnect::new(self, cells, terrain_set, terrain,)
        }
        pub(crate) fn set_cells_terrain_path_full(&mut self, path: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < Vector2i > >, i32, i32, bool);
            let args = (path, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_cells_terrain_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cells_terrain_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cells_terrain_path(&mut self, path: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_path_ex(path, terrain_set, terrain,) . done()
        }
        #[inline]
        pub fn set_cells_terrain_path_ex < 'a > (&'a mut self, path: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainPath < 'a > {
            ExSetCellsTerrainPath::new(self, path, terrain_set, terrain,)
        }
        pub fn has_body_rid(&self, body: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "has_body_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_coords_for_body_rid(&self, body: Rid,) -> Vector2i {
            type CallSig = (Vector2i, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_coords_for_body_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_internals(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "update_internals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_runtime_tile_data_update(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "notify_runtime_tile_data_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_pattern(&mut self, position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: impl AsObjectArg < crate::classes::TileMapPattern >,) -> Vector2i {
            type CallSig = (Vector2i, Vector2i, Vector2i, ObjectArg < crate::classes::TileMapPattern >);
            let args = (position_in_tilemap, coords_in_pattern, pattern.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "map_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surrounding_cells(&mut self, coords: Vector2i,) -> Array < Vector2i > {
            type CallSig = (Array < Vector2i >, Vector2i);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_surrounding_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_neighbor_cell(&self, coords: Vector2i, neighbor: crate::classes::tile_set::CellNeighbor,) -> Vector2i {
            type CallSig = (Vector2i, Vector2i, crate::classes::tile_set::CellNeighbor);
            let args = (coords, neighbor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_neighbor_cell", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_to_local(&self, map_position: Vector2i,) -> Vector2 {
            type CallSig = (Vector2, Vector2i);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "map_to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn local_to_map(&self, local_position: Vector2,) -> Vector2i {
            type CallSig = (Vector2i, Vector2);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "local_to_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_map_data_from_array(&mut self, tile_map_layer_data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedByteArray >);
            let args = (RefArg::new(tile_map_layer_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_tile_map_data_from_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_map_data_as_array(&self,) -> PackedByteArray {
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_tile_map_data_as_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_set(&mut self, tile_set: impl AsObjectArg < crate::classes::TileSet >,) {
            type CallSig = ((), ObjectArg < crate::classes::TileSet >);
            let args = (tile_set.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_tile_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_set(&self,) -> Option < Gd < crate::classes::TileSet > > {
            type CallSig = (Option < Gd < crate::classes::TileSet > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_tile_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_y_sort_origin(&mut self, y_sort_origin: i32,) {
            type CallSig = ((), i32);
            let args = (y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_y_sort_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_y_sort_origin(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_y_sort_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_x_draw_order_reversed(&mut self, x_draw_order_reversed: bool,) {
            type CallSig = ((), bool);
            let args = (x_draw_order_reversed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_x_draw_order_reversed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_x_draw_order_reversed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_x_draw_order_reversed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rendering_quadrant_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_rendering_quadrant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rendering_quadrant_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_rendering_quadrant_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_collision_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collision_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_collision_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_kinematic_bodies(&mut self, use_kinematic_bodies: bool,) {
            type CallSig = ((), bool);
            let args = (use_kinematic_bodies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_use_kinematic_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_kinematic_bodies(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_using_kinematic_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_visibility_mode(&mut self, visibility_mode: crate::classes::tile_map_layer::DebugVisibilityMode,) {
            type CallSig = ((), crate::classes::tile_map_layer::DebugVisibilityMode);
            let args = (visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_collision_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_visibility_mode(&self,) -> crate::classes::tile_map_layer::DebugVisibilityMode {
            type CallSig = (crate::classes::tile_map_layer::DebugVisibilityMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_collision_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_navigation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_navigation_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "is_navigation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_map(&mut self, map: Rid,) {
            type CallSig = ((), Rid);
            let args = (map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_map(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_navigation_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_visibility_mode(&mut self, show_navigation: crate::classes::tile_map_layer::DebugVisibilityMode,) {
            type CallSig = ((), crate::classes::tile_map_layer::DebugVisibilityMode);
            let args = (show_navigation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "set_navigation_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_visibility_mode(&self,) -> crate::classes::tile_map_layer::DebugVisibilityMode {
            type CallSig = (crate::classes::tile_map_layer::DebugVisibilityMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileMapLayer", "get_navigation_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for TileMapLayer {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TileMapLayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMapLayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for TileMapLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TileMapLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TileMapLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileMapLayer {
        
    }
    impl crate::obj::cap::GodotDefault for TileMapLayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMapLayer {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMapLayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileMapLayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TileMapLayer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileMapLayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TileMapLayer::set_cell_ex`][super::TileMapLayer::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMapLayer, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCell < 'a > {
    fn new(surround_object: &'a mut re_export::TileMapLayer, coords: Vector2i,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, coords: coords, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, coords, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMapLayer::set_cell_full(surround_object, coords, source_id, atlas_coords, alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMapLayer::get_used_cells_by_id_ex`][super::TileMapLayer::get_used_cells_by_id_ex]."]
#[must_use]
pub struct ExGetUsedCellsById < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMapLayer, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUsedCellsById < 'a > {
    fn new(surround_object: &'a re_export::TileMapLayer,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2i > {
        let Self {
            _phantom, surround_object, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMapLayer::get_used_cells_by_id_full(surround_object, source_id, atlas_coords, alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMapLayer::set_cells_terrain_connect_ex`][super::TileMapLayer::set_cells_terrain_connect_ex]."]
#[must_use]
pub struct ExSetCellsTerrainConnect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMapLayer, cells: CowArg < 'a, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellsTerrainConnect < 'a > {
    fn new(surround_object: &'a mut re_export::TileMapLayer, cells: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cells: CowArg::Borrowed(cells), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, ignore_empty_terrains: bool) -> Self {
        Self {
            ignore_empty_terrains: ignore_empty_terrains, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, cells, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMapLayer::set_cells_terrain_connect_full(surround_object, cells.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMapLayer::set_cells_terrain_path_ex`][super::TileMapLayer::set_cells_terrain_path_ex]."]
#[must_use]
pub struct ExSetCellsTerrainPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMapLayer, path: CowArg < 'a, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellsTerrainPath < 'a > {
    fn new(surround_object: &'a mut re_export::TileMapLayer, path: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: CowArg::Borrowed(path), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, ignore_empty_terrains: bool) -> Self {
        Self {
            ignore_empty_terrains: ignore_empty_terrains, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, path, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMapLayer::set_cells_terrain_path_full(surround_object, path.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DebugVisibilityMode {
    ord: i32
}
impl DebugVisibilityMode {
    #[doc(alias = "DEBUG_VISIBILITY_MODE_DEFAULT")]
    #[doc = "Godot enumerator name: `DEBUG_VISIBILITY_MODE_DEFAULT`"]
    pub const DEFAULT: DebugVisibilityMode = DebugVisibilityMode {
        ord: 0i32
    };
    #[doc(alias = "DEBUG_VISIBILITY_MODE_FORCE_HIDE")]
    #[doc = "Godot enumerator name: `DEBUG_VISIBILITY_MODE_FORCE_HIDE`"]
    pub const FORCE_HIDE: DebugVisibilityMode = DebugVisibilityMode {
        ord: 2i32
    };
    #[doc(alias = "DEBUG_VISIBILITY_MODE_FORCE_SHOW")]
    #[doc = "Godot enumerator name: `DEBUG_VISIBILITY_MODE_FORCE_SHOW`"]
    pub const FORCE_SHOW: DebugVisibilityMode = DebugVisibilityMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DebugVisibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DebugVisibilityMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DebugVisibilityMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "DEFAULT", Self::FORCE_HIDE => "FORCE_HIDE", Self::FORCE_SHOW => "FORCE_SHOW", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "DEBUG_VISIBILITY_MODE_DEFAULT", Self::FORCE_HIDE => "DEBUG_VISIBILITY_MODE_FORCE_HIDE", Self::FORCE_SHOW => "DEBUG_VISIBILITY_MODE_FORCE_SHOW", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DebugVisibilityMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DebugVisibilityMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DebugVisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
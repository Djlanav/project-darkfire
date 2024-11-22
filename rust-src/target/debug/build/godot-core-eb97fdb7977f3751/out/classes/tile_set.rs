#![doc = "Sidecar module for class [`TileSet`][crate::classes::TileSet].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSet` enums](https://docs.godotengine.org/en/stable/classes/class_tileset.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSet.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`tile_set`][crate::classes::tile_set]: sidecar module with related enum/flag types\n* [`ITileSet`][crate::classes::ITileSet]: virtual methods\n\n\nSee also [Godot docs for `TileSet`](https://docs.godotengine.org/en/stable/classes/class_tileset.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileSet::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSet {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TileSet`][crate::classes::TileSet].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSet` methods](https://docs.godotengine.org/en/stable/classes/class_tileset.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSet: crate::obj::GodotClass < Base = TileSet > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl TileSet {
        pub fn get_next_source_id(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_next_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_source_full(&mut self, source: ObjectArg < crate::classes::TileSetSource >, atlas_source_id_override: i32,) -> i32 {
            type CallSig = (i32, ObjectArg < crate::classes::TileSetSource >, i32);
            let args = (source, atlas_source_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_source_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_source(&mut self, source: impl AsObjectArg < crate::classes::TileSetSource >,) -> i32 {
            self.add_source_ex(source,) . done()
        }
        #[inline]
        pub fn add_source_ex < 'a > (&'a mut self, source: impl AsObjectArg < crate::classes::TileSetSource >,) -> ExAddSource < 'a > {
            ExAddSource::new(self, source,)
        }
        pub fn remove_source(&mut self, source_id: i32,) {
            type CallSig = ((), i32);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_source_id(&mut self, source_id: i32, new_source_id: i32,) {
            type CallSig = ((), i32, i32);
            let args = (source_id, new_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_source_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_id(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_source_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_source(&self, source_id: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "has_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source(&self, source_id: i32,) -> Option < Gd < crate::classes::TileSetSource > > {
            type CallSig = (Option < Gd < crate::classes::TileSetSource > >, i32);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_shape(&mut self, shape: crate::classes::tile_set::TileShape,) {
            type CallSig = ((), crate::classes::tile_set::TileShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_shape(&self,) -> crate::classes::tile_set::TileShape {
            type CallSig = (crate::classes::tile_set::TileShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_layout(&mut self, layout: crate::classes::tile_set::TileLayout,) {
            type CallSig = ((), crate::classes::tile_set::TileLayout);
            let args = (layout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_layout(&self,) -> crate::classes::tile_set::TileLayout {
            type CallSig = (crate::classes::tile_set::TileLayout,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_offset_axis(&mut self, alignment: crate::classes::tile_set::TileOffsetAxis,) {
            type CallSig = ((), crate::classes::tile_set::TileOffsetAxis);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_offset_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_offset_axis(&self,) -> crate::classes::tile_set::TileOffsetAxis {
            type CallSig = (crate::classes::tile_set::TileOffsetAxis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_offset_axis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tile_size(&mut self, size: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tile_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv_clipping(&mut self, uv_clipping: bool,) {
            type CallSig = ((), bool);
            let args = (uv_clipping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_uv_clipping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_uv_clipping(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "is_uv_clipping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occlusion_layers_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_occlusion_layer_full(&mut self, to_position: i32,) {
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_occlusion_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_occlusion_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_occlusion_layer(&mut self,) {
            self.add_occlusion_layer_ex() . done()
        }
        #[inline]
        pub fn add_occlusion_layer_ex < 'a > (&'a mut self,) -> ExAddOcclusionLayer < 'a > {
            ExAddOcclusionLayer::new(self,)
        }
        pub fn move_occlusion_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "move_occlusion_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_occlusion_layer(&mut self, layer_index: i32,) {
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_occlusion_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_occlusion_layer_light_mask(&mut self, layer_index: i32, light_mask: i32,) {
            type CallSig = ((), i32, i32);
            let args = (layer_index, light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_occlusion_layer_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occlusion_layer_light_mask(&self, layer_index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layer_light_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_occlusion_layer_sdf_collision(&mut self, layer_index: i32, sdf_collision: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_index, sdf_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_occlusion_layer_sdf_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_occlusion_layer_sdf_collision(&self, layer_index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layer_sdf_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layers_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_physics_layer_full(&mut self, to_position: i32,) {
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_physics_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_physics_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_physics_layer(&mut self,) {
            self.add_physics_layer_ex() . done()
        }
        #[inline]
        pub fn add_physics_layer_ex < 'a > (&'a mut self,) -> ExAddPhysicsLayer < 'a > {
            ExAddPhysicsLayer::new(self,)
        }
        pub fn move_physics_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "move_physics_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_physics_layer(&mut self, layer_index: i32,) {
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_physics_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_layer_collision_layer(&mut self, layer_index: i32, layer: u32,) {
            type CallSig = ((), i32, u32);
            let args = (layer_index, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layer_collision_layer(&self, layer_index: i32,) -> u32 {
            type CallSig = (u32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_layer_collision_mask(&mut self, layer_index: i32, mask: u32,) {
            type CallSig = ((), i32, u32);
            let args = (layer_index, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layer_collision_mask(&self, layer_index: i32,) -> u32 {
            type CallSig = (u32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_layer_physics_material(&mut self, layer_index: i32, physics_material: impl AsObjectArg < crate::classes::PhysicsMaterial >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::PhysicsMaterial >);
            let args = (layer_index, physics_material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_layer_physics_material(&self, layer_index: i32,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallSig = (Option < Gd < crate::classes::PhysicsMaterial > >, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_physics_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_sets_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_sets_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_terrain_set_full(&mut self, to_position: i32,) {
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_terrain_set_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_terrain_set(&mut self,) {
            self.add_terrain_set_ex() . done()
        }
        #[inline]
        pub fn add_terrain_set_ex < 'a > (&'a mut self,) -> ExAddTerrainSet < 'a > {
            ExAddTerrainSet::new(self,)
        }
        pub fn move_terrain_set(&mut self, terrain_set: i32, to_position: i32,) {
            type CallSig = ((), i32, i32);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "move_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_terrain_set(&mut self, terrain_set: i32,) {
            type CallSig = ((), i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_terrain_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_set_mode(&mut self, terrain_set: i32, mode: crate::classes::tile_set::TerrainMode,) {
            type CallSig = ((), i32, crate::classes::tile_set::TerrainMode);
            let args = (terrain_set, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_set_mode(&self, terrain_set: i32,) -> crate::classes::tile_set::TerrainMode {
            type CallSig = (crate::classes::tile_set::TerrainMode, i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrains_count(&self, terrain_set: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_terrains_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_terrain_full(&mut self, terrain_set: i32, to_position: i32,) {
            type CallSig = ((), i32, i32);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_terrain_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_terrain(&mut self, terrain_set: i32,) {
            self.add_terrain_ex(terrain_set,) . done()
        }
        #[inline]
        pub fn add_terrain_ex < 'a > (&'a mut self, terrain_set: i32,) -> ExAddTerrain < 'a > {
            ExAddTerrain::new(self, terrain_set,)
        }
        pub fn move_terrain(&mut self, terrain_set: i32, terrain_index: i32, to_position: i32,) {
            type CallSig = ((), i32, i32, i32);
            let args = (terrain_set, terrain_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "move_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_terrain(&mut self, terrain_set: i32, terrain_index: i32,) {
            type CallSig = ((), i32, i32);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_terrain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_name(&mut self, terrain_set: i32, terrain_index: i32, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, i32, CowArg < 'a0, GString >);
            let args = (terrain_set, terrain_index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_name(&self, terrain_set: i32, terrain_index: i32,) -> GString {
            type CallSig = (GString, i32, i32);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_terrain_color(&mut self, terrain_set: i32, terrain_index: i32, color: Color,) {
            type CallSig = ((), i32, i32, Color);
            let args = (terrain_set, terrain_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_terrain_color(&self, terrain_set: i32, terrain_index: i32,) -> Color {
            type CallSig = (Color, i32, i32);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_layers_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_navigation_layer_full(&mut self, to_position: i32,) {
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_navigation_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_navigation_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_navigation_layer(&mut self,) {
            self.add_navigation_layer_ex() . done()
        }
        #[inline]
        pub fn add_navigation_layer_ex < 'a > (&'a mut self,) -> ExAddNavigationLayer < 'a > {
            ExAddNavigationLayer::new(self,)
        }
        pub fn move_navigation_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "move_navigation_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_navigation_layer(&mut self, layer_index: i32,) {
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_navigation_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_layer_layers(&mut self, layer_index: i32, layers: u32,) {
            type CallSig = ((), i32, u32);
            let args = (layer_index, layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_navigation_layer_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_layer_layers(&self, layer_index: i32,) -> u32 {
            type CallSig = (u32, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layer_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_navigation_layer_layer_value(&mut self, layer_index: i32, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, i32, bool);
            let args = (layer_index, layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_navigation_layer_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_navigation_layer_layer_value(&self, layer_index: i32, layer_number: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (layer_index, layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layer_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layers_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layers_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_custom_data_layer_full(&mut self, to_position: i32,) {
            type CallSig = ((), i32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_custom_data_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_custom_data_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_custom_data_layer(&mut self,) {
            self.add_custom_data_layer_ex() . done()
        }
        #[inline]
        pub fn add_custom_data_layer_ex < 'a > (&'a mut self,) -> ExAddCustomDataLayer < 'a > {
            ExAddCustomDataLayer::new(self,)
        }
        pub fn move_custom_data_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallSig = ((), i32, i32);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "move_custom_data_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_custom_data_layer(&mut self, layer_index: i32,) {
            type CallSig = ((), i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_custom_data_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layer_by_name(&self, layer_name: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_data_layer_name(&mut self, layer_index: i32, layer_name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (layer_index, layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_custom_data_layer_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layer_name(&self, layer_index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_data_layer_type(&mut self, layer_index: i32, layer_type: VariantType,) {
            type CallSig = ((), i32, VariantType);
            let args = (layer_index, layer_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_custom_data_layer_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_data_layer_type(&self, layer_index: i32,) -> VariantType {
            type CallSig = (VariantType, i32);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_source_level_tile_proxy(&mut self, source_from: i32, source_to: i32,) {
            type CallSig = ((), i32, i32);
            let args = (source_from, source_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_source_level_tile_proxy(&mut self, source_from: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_source_level_tile_proxy(&mut self, source_from: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "has_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_source_level_tile_proxy(&mut self, source_from: i32,) {
            type CallSig = ((), i32);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_source_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_coords_level_tile_proxy(&mut self, p_source_from: i32, coords_from: Vector2i, source_to: i32, coords_to: Vector2i,) {
            type CallSig = ((), i32, Vector2i, i32, Vector2i);
            let args = (p_source_from, coords_from, source_to, coords_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) -> VariantArray {
            type CallSig = (VariantArray, i32, Vector2i);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) -> bool {
            type CallSig = (bool, i32, Vector2i);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "has_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) {
            type CallSig = ((), i32, Vector2i);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_coords_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32, source_to: i32, coords_to: Vector2i, alternative_to: i32,) {
            type CallSig = ((), i32, Vector2i, i32, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from, source_to, coords_to, alternative_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "set_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> bool {
            type CallSig = (bool, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "has_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) {
            type CallSig = ((), i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_alternative_level_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn map_tile_proxy(&self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32, Vector2i, i32);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "map_tile_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cleanup_invalid_tile_proxies(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "cleanup_invalid_tile_proxies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_tile_proxies(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "clear_tile_proxies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_pattern_full(&mut self, pattern: ObjectArg < crate::classes::TileMapPattern >, index: i32,) -> i32 {
            type CallSig = (i32, ObjectArg < crate::classes::TileMapPattern >, i32);
            let args = (pattern, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "add_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_pattern_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_pattern(&mut self, pattern: impl AsObjectArg < crate::classes::TileMapPattern >,) -> i32 {
            self.add_pattern_ex(pattern,) . done()
        }
        #[inline]
        pub fn add_pattern_ex < 'a > (&'a mut self, pattern: impl AsObjectArg < crate::classes::TileMapPattern >,) -> ExAddPattern < 'a > {
            ExAddPattern::new(self, pattern,)
        }
        pub(crate) fn get_pattern_full(&mut self, index: i32,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallSig = (Option < Gd < crate::classes::TileMapPattern > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_pattern_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_pattern(&mut self,) -> Option < Gd < crate::classes::TileMapPattern > > {
            self.get_pattern_ex() . done()
        }
        #[inline]
        pub fn get_pattern_ex < 'a > (&'a mut self,) -> ExGetPattern < 'a > {
            ExGetPattern::new(self,)
        }
        pub fn remove_pattern(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "remove_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_patterns_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TileSet", "get_patterns_count", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TileSet {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TileSet"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSet {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSet {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSet {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSet {
        
    }
    impl crate::obj::cap::GodotDefault for TileSet {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSet {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSet {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileSet`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TileSet {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSet > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TileSet::add_source_ex`][super::TileSet::add_source_ex]."]
#[must_use]
pub struct ExAddSource < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, source: ObjectCow < crate::classes::TileSetSource >, atlas_source_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSource < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, source: impl AsObjectArg < crate::classes::TileSetSource >,) -> Self {
        let atlas_source_id_override = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source: source.consume_arg(), atlas_source_id_override: atlas_source_id_override,
        }
    }
    #[inline]
    pub fn atlas_source_id_override(self, atlas_source_id_override: i32) -> Self {
        Self {
            atlas_source_id_override: atlas_source_id_override, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, source, atlas_source_id_override,
        }
        = self;
        re_export::TileSet::add_source_full(surround_object, source.cow_as_object_arg(), atlas_source_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_occlusion_layer_ex`][super::TileSet::add_occlusion_layer_ex]."]
#[must_use]
pub struct ExAddOcclusionLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddOcclusionLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_occlusion_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_physics_layer_ex`][super::TileSet::add_physics_layer_ex]."]
#[must_use]
pub struct ExAddPhysicsLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPhysicsLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_physics_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_set_ex`][super::TileSet::add_terrain_set_ex]."]
#[must_use]
pub struct ExAddTerrainSet < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTerrainSet < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_terrain_set_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_ex`][super::TileSet::add_terrain_ex]."]
#[must_use]
pub struct ExAddTerrain < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, terrain_set: i32, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTerrain < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, terrain_set: i32,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, terrain_set: terrain_set, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, terrain_set, to_position,
        }
        = self;
        re_export::TileSet::add_terrain_full(surround_object, terrain_set, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_navigation_layer_ex`][super::TileSet::add_navigation_layer_ex]."]
#[must_use]
pub struct ExAddNavigationLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNavigationLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_navigation_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_custom_data_layer_ex`][super::TileSet::add_custom_data_layer_ex]."]
#[must_use]
pub struct ExAddCustomDataLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCustomDataLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_custom_data_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_pattern_ex`][super::TileSet::add_pattern_ex]."]
#[must_use]
pub struct ExAddPattern < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, pattern: ObjectCow < crate::classes::TileMapPattern >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPattern < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, pattern: impl AsObjectArg < crate::classes::TileMapPattern >,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.consume_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, pattern, index,
        }
        = self;
        re_export::TileSet::add_pattern_full(surround_object, pattern.cow_as_object_arg(), index,)
    }
}
#[doc = "Default-param extender for [`TileSet::get_pattern_ex`][super::TileSet::get_pattern_ex]."]
#[must_use]
pub struct ExGetPattern < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPattern < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TileMapPattern > > {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::TileSet::get_pattern_full(surround_object, index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileShape {
    ord: i32
}
impl TileShape {
    #[doc(alias = "TILE_SHAPE_SQUARE")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_SQUARE`"]
    pub const SQUARE: TileShape = TileShape {
        ord: 0i32
    };
    #[doc(alias = "TILE_SHAPE_ISOMETRIC")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_ISOMETRIC`"]
    pub const ISOMETRIC: TileShape = TileShape {
        ord: 1i32
    };
    #[doc(alias = "TILE_SHAPE_HALF_OFFSET_SQUARE")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_HALF_OFFSET_SQUARE`"]
    pub const HALF_OFFSET_SQUARE: TileShape = TileShape {
        ord: 2i32
    };
    #[doc(alias = "TILE_SHAPE_HEXAGON")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_HEXAGON`"]
    pub const HEXAGON: TileShape = TileShape {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TileShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::SQUARE => "SQUARE", Self::ISOMETRIC => "ISOMETRIC", Self::HALF_OFFSET_SQUARE => "HALF_OFFSET_SQUARE", Self::HEXAGON => "HEXAGON", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SQUARE => "TILE_SHAPE_SQUARE", Self::ISOMETRIC => "TILE_SHAPE_ISOMETRIC", Self::HALF_OFFSET_SQUARE => "TILE_SHAPE_HALF_OFFSET_SQUARE", Self::HEXAGON => "TILE_SHAPE_HEXAGON", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TileShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileShape {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileLayout {
    ord: i32
}
impl TileLayout {
    #[doc(alias = "TILE_LAYOUT_STACKED")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STACKED`"]
    pub const STACKED: TileLayout = TileLayout {
        ord: 0i32
    };
    #[doc(alias = "TILE_LAYOUT_STACKED_OFFSET")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STACKED_OFFSET`"]
    pub const STACKED_OFFSET: TileLayout = TileLayout {
        ord: 1i32
    };
    #[doc(alias = "TILE_LAYOUT_STAIRS_RIGHT")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STAIRS_RIGHT`"]
    pub const STAIRS_RIGHT: TileLayout = TileLayout {
        ord: 2i32
    };
    #[doc(alias = "TILE_LAYOUT_STAIRS_DOWN")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STAIRS_DOWN`"]
    pub const STAIRS_DOWN: TileLayout = TileLayout {
        ord: 3i32
    };
    #[doc(alias = "TILE_LAYOUT_DIAMOND_RIGHT")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_DIAMOND_RIGHT`"]
    pub const DIAMOND_RIGHT: TileLayout = TileLayout {
        ord: 4i32
    };
    #[doc(alias = "TILE_LAYOUT_DIAMOND_DOWN")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_DIAMOND_DOWN`"]
    pub const DIAMOND_DOWN: TileLayout = TileLayout {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for TileLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileLayout") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileLayout {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::STACKED => "STACKED", Self::STACKED_OFFSET => "STACKED_OFFSET", Self::STAIRS_RIGHT => "STAIRS_RIGHT", Self::STAIRS_DOWN => "STAIRS_DOWN", Self::DIAMOND_RIGHT => "DIAMOND_RIGHT", Self::DIAMOND_DOWN => "DIAMOND_DOWN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STACKED => "TILE_LAYOUT_STACKED", Self::STACKED_OFFSET => "TILE_LAYOUT_STACKED_OFFSET", Self::STAIRS_RIGHT => "TILE_LAYOUT_STAIRS_RIGHT", Self::STAIRS_DOWN => "TILE_LAYOUT_STAIRS_DOWN", Self::DIAMOND_RIGHT => "TILE_LAYOUT_DIAMOND_RIGHT", Self::DIAMOND_DOWN => "TILE_LAYOUT_DIAMOND_DOWN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TileLayout {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileLayout {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileOffsetAxis {
    ord: i32
}
impl TileOffsetAxis {
    #[doc(alias = "TILE_OFFSET_AXIS_HORIZONTAL")]
    #[doc = "Godot enumerator name: `TILE_OFFSET_AXIS_HORIZONTAL`"]
    pub const HORIZONTAL: TileOffsetAxis = TileOffsetAxis {
        ord: 0i32
    };
    #[doc(alias = "TILE_OFFSET_AXIS_VERTICAL")]
    #[doc = "Godot enumerator name: `TILE_OFFSET_AXIS_VERTICAL`"]
    pub const VERTICAL: TileOffsetAxis = TileOffsetAxis {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TileOffsetAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileOffsetAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileOffsetAxis {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::HORIZONTAL => "HORIZONTAL", Self::VERTICAL => "VERTICAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::HORIZONTAL => "TILE_OFFSET_AXIS_HORIZONTAL", Self::VERTICAL => "TILE_OFFSET_AXIS_VERTICAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TileOffsetAxis {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileOffsetAxis {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileOffsetAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellNeighbor {
    ord: i32
}
impl CellNeighbor {
    #[doc(alias = "CELL_NEIGHBOR_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_RIGHT_SIDE`"]
    pub const RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 0i32
    };
    #[doc(alias = "CELL_NEIGHBOR_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_RIGHT_CORNER`"]
    pub const RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 1i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE`"]
    pub const BOTTOM_RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 2i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER`"]
    pub const BOTTOM_RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 3i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_SIDE`"]
    pub const BOTTOM_SIDE: CellNeighbor = CellNeighbor {
        ord: 4i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_CORNER`"]
    pub const BOTTOM_CORNER: CellNeighbor = CellNeighbor {
        ord: 5i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_LEFT_SIDE`"]
    pub const BOTTOM_LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 6i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_LEFT_CORNER`"]
    pub const BOTTOM_LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 7i32
    };
    #[doc(alias = "CELL_NEIGHBOR_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_LEFT_SIDE`"]
    pub const LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 8i32
    };
    #[doc(alias = "CELL_NEIGHBOR_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_LEFT_CORNER`"]
    pub const LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 9i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_LEFT_SIDE`"]
    pub const TOP_LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 10i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_LEFT_CORNER`"]
    pub const TOP_LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 11i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_SIDE`"]
    pub const TOP_SIDE: CellNeighbor = CellNeighbor {
        ord: 12i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_CORNER`"]
    pub const TOP_CORNER: CellNeighbor = CellNeighbor {
        ord: 13i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_RIGHT_SIDE`"]
    pub const TOP_RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 14i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_RIGHT_CORNER`"]
    pub const TOP_RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 15i32
    };
    
}
impl std::fmt::Debug for CellNeighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CellNeighbor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CellNeighbor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
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
            Self::RIGHT_SIDE => "RIGHT_SIDE", Self::RIGHT_CORNER => "RIGHT_CORNER", Self::BOTTOM_RIGHT_SIDE => "BOTTOM_RIGHT_SIDE", Self::BOTTOM_RIGHT_CORNER => "BOTTOM_RIGHT_CORNER", Self::BOTTOM_SIDE => "BOTTOM_SIDE", Self::BOTTOM_CORNER => "BOTTOM_CORNER", Self::BOTTOM_LEFT_SIDE => "BOTTOM_LEFT_SIDE", Self::BOTTOM_LEFT_CORNER => "BOTTOM_LEFT_CORNER", Self::LEFT_SIDE => "LEFT_SIDE", Self::LEFT_CORNER => "LEFT_CORNER", Self::TOP_LEFT_SIDE => "TOP_LEFT_SIDE", Self::TOP_LEFT_CORNER => "TOP_LEFT_CORNER", Self::TOP_SIDE => "TOP_SIDE", Self::TOP_CORNER => "TOP_CORNER", Self::TOP_RIGHT_SIDE => "TOP_RIGHT_SIDE", Self::TOP_RIGHT_CORNER => "TOP_RIGHT_CORNER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RIGHT_SIDE => "CELL_NEIGHBOR_RIGHT_SIDE", Self::RIGHT_CORNER => "CELL_NEIGHBOR_RIGHT_CORNER", Self::BOTTOM_RIGHT_SIDE => "CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE", Self::BOTTOM_RIGHT_CORNER => "CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER", Self::BOTTOM_SIDE => "CELL_NEIGHBOR_BOTTOM_SIDE", Self::BOTTOM_CORNER => "CELL_NEIGHBOR_BOTTOM_CORNER", Self::BOTTOM_LEFT_SIDE => "CELL_NEIGHBOR_BOTTOM_LEFT_SIDE", Self::BOTTOM_LEFT_CORNER => "CELL_NEIGHBOR_BOTTOM_LEFT_CORNER", Self::LEFT_SIDE => "CELL_NEIGHBOR_LEFT_SIDE", Self::LEFT_CORNER => "CELL_NEIGHBOR_LEFT_CORNER", Self::TOP_LEFT_SIDE => "CELL_NEIGHBOR_TOP_LEFT_SIDE", Self::TOP_LEFT_CORNER => "CELL_NEIGHBOR_TOP_LEFT_CORNER", Self::TOP_SIDE => "CELL_NEIGHBOR_TOP_SIDE", Self::TOP_CORNER => "CELL_NEIGHBOR_TOP_CORNER", Self::TOP_RIGHT_SIDE => "CELL_NEIGHBOR_TOP_RIGHT_SIDE", Self::TOP_RIGHT_CORNER => "CELL_NEIGHBOR_TOP_RIGHT_CORNER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CellNeighbor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CellNeighbor {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CellNeighbor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TerrainMode {
    ord: i32
}
impl TerrainMode {
    #[doc(alias = "TERRAIN_MODE_MATCH_CORNERS_AND_SIDES")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_CORNERS_AND_SIDES`"]
    pub const CORNERS_AND_SIDES: TerrainMode = TerrainMode {
        ord: 0i32
    };
    #[doc(alias = "TERRAIN_MODE_MATCH_CORNERS")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_CORNERS`"]
    pub const CORNERS: TerrainMode = TerrainMode {
        ord: 1i32
    };
    #[doc(alias = "TERRAIN_MODE_MATCH_SIDES")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_SIDES`"]
    pub const SIDES: TerrainMode = TerrainMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TerrainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TerrainMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TerrainMode {
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
            Self::CORNERS_AND_SIDES => "CORNERS_AND_SIDES", Self::CORNERS => "CORNERS", Self::SIDES => "SIDES", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CORNERS_AND_SIDES => "TERRAIN_MODE_MATCH_CORNERS_AND_SIDES", Self::CORNERS => "TERRAIN_MODE_MATCH_CORNERS", Self::SIDES => "TERRAIN_MODE_MATCH_SIDES", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TerrainMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TerrainMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TerrainMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
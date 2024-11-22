#![doc = "Sidecar module for class [`MeshDataTool`][crate::classes::MeshDataTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshDataTool` enums](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshDataTool.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`mesh_data_tool`][crate::classes::mesh_data_tool]: sidecar module with related enum/flag types\n* [`IMeshDataTool`][crate::classes::IMeshDataTool]: virtual methods\n\n\nSee also [Godot docs for `MeshDataTool`](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MeshDataTool::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshDataTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MeshDataTool`][crate::classes::MeshDataTool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MeshDataTool` methods](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshDataTool: crate::obj::GodotClass < Base = MeshDataTool > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl MeshDataTool {
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_from_surface(&mut self, mesh: impl AsObjectArg < crate::classes::ArrayMesh >, surface: i32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, ObjectArg < crate::classes::ArrayMesh >, i32);
            let args = (mesh.as_object_arg(), surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "create_from_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn commit_to_surface_full(&mut self, mesh: ObjectArg < crate::classes::ArrayMesh >, compression_flags: u64,) -> crate::global::Error {
            type CallSig = (crate::global::Error, ObjectArg < crate::classes::ArrayMesh >, u64);
            let args = (mesh, compression_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "commit_to_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_to_surface_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit_to_surface(&mut self, mesh: impl AsObjectArg < crate::classes::ArrayMesh >,) -> crate::global::Error {
            self.commit_to_surface_ex(mesh,) . done()
        }
        #[inline]
        pub fn commit_to_surface_ex < 'a > (&'a mut self, mesh: impl AsObjectArg < crate::classes::ArrayMesh >,) -> ExCommitToSurface < 'a > {
            ExCommitToSurface::new(self, mesh,)
        }
        pub fn get_format(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex(&mut self, idx: i32, vertex: Vector3,) {
            type CallSig = ((), i32, Vector3);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex(&self, idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_normal(&mut self, idx: i32, normal: Vector3,) {
            type CallSig = ((), i32, Vector3);
            let args = (idx, normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_normal(&self, idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_tangent(&mut self, idx: i32, tangent: Plane,) {
            type CallSig = ((), i32, Plane);
            let args = (idx, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_tangent(&self, idx: i32,) -> Plane {
            type CallSig = (Plane, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_uv(&mut self, idx: i32, uv: Vector2,) {
            type CallSig = ((), i32, Vector2);
            let args = (idx, uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_uv(&self, idx: i32,) -> Vector2 {
            type CallSig = (Vector2, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_uv2(&mut self, idx: i32, uv2: Vector2,) {
            type CallSig = ((), i32, Vector2);
            let args = (idx, uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_uv2(&self, idx: i32,) -> Vector2 {
            type CallSig = (Vector2, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_color(&mut self, idx: i32, color: Color,) {
            type CallSig = ((), i32, Color);
            let args = (idx, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_color(&self, idx: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_bones(&mut self, idx: i32, bones: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, PackedInt32Array >);
            let args = (idx, RefArg::new(bones),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_bones(&self, idx: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_weights(&mut self, idx: i32, weights: &PackedFloat32Array,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, PackedFloat32Array >);
            let args = (idx, RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_weights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_weights(&self, idx: i32,) -> PackedFloat32Array {
            type CallSig = (PackedFloat32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_weights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_meta(&self, idx: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_edges(&self, idx: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_edges", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_faces(&self, idx: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_faces(&self, idx: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edge_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_edge_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_meta(&self, idx: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_edge(&self, idx: i32, edge: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (idx, edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_face_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_face_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_meta(&self, idx: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_normal(&self, idx: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_material", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MeshDataTool {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"MeshDataTool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshDataTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MeshDataTool {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshDataTool {
        
    }
    impl crate::obj::cap::GodotDefault for MeshDataTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshDataTool {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshDataTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshDataTool`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_MeshDataTool {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshDataTool > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MeshDataTool::commit_to_surface_ex`][super::MeshDataTool::commit_to_surface_ex]."]
#[must_use]
pub struct ExCommitToSurface < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MeshDataTool, mesh: ObjectCow < crate::classes::ArrayMesh >, compression_flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitToSurface < 'a > {
    fn new(surround_object: &'a mut re_export::MeshDataTool, mesh: impl AsObjectArg < crate::classes::ArrayMesh >,) -> Self {
        let compression_flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh.consume_arg(), compression_flags: compression_flags,
        }
    }
    #[inline]
    pub fn compression_flags(self, compression_flags: u64) -> Self {
        Self {
            compression_flags: compression_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mesh, compression_flags,
        }
        = self;
        re_export::MeshDataTool::commit_to_surface_full(surround_object, mesh.cow_as_object_arg(), compression_flags,)
    }
}
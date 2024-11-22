#![doc = "Sidecar module for class [`SurfaceTool`][crate::classes::SurfaceTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SurfaceTool` enums](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SurfaceTool.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`surface_tool`][crate::classes::surface_tool]: sidecar module with related enum/flag types\n* [`ISurfaceTool`][crate::classes::ISurfaceTool]: virtual methods\n\n\nSee also [Godot docs for `SurfaceTool`](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SurfaceTool::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SurfaceTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SurfaceTool`][crate::classes::SurfaceTool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SurfaceTool` methods](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISurfaceTool: crate::obj::GodotClass < Base = SurfaceTool > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SurfaceTool {
        pub fn set_skin_weight_count(&mut self, count: crate::classes::surface_tool::SkinWeightCount,) {
            type CallSig = ((), crate::classes::surface_tool::SkinWeightCount);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_skin_weight_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin_weight_count(&self,) -> crate::classes::surface_tool::SkinWeightCount {
            type CallSig = (crate::classes::surface_tool::SkinWeightCount,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_skin_weight_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_format(&mut self, channel_index: i32, format: crate::classes::surface_tool::CustomFormat,) {
            type CallSig = ((), i32, crate::classes::surface_tool::CustomFormat);
            let args = (channel_index, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_custom_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_format(&self, channel_index: i32,) -> crate::classes::surface_tool::CustomFormat {
            type CallSig = (crate::classes::surface_tool::CustomFormat, i32);
            let args = (channel_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_custom_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn begin(&mut self, primitive: crate::classes::mesh::PrimitiveType,) {
            type CallSig = ((), crate::classes::mesh::PrimitiveType);
            let args = (primitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_vertex(&mut self, vertex: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "add_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normal(&mut self, normal: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tangent(&mut self, tangent: Plane,) {
            type CallSig = ((), Plane);
            let args = (tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv(&mut self, uv: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2(&mut self, uv2: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bones(&mut self, bones: &PackedInt32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedInt32Array >);
            let args = (RefArg::new(bones),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_weights(&mut self, weights: &PackedFloat32Array,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedFloat32Array >);
            let args = (RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_weights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom(&mut self, channel_index: i32, custom_color: Color,) {
            type CallSig = ((), i32, Color);
            let args = (channel_index, custom_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_custom", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_smooth_group(&mut self, index: u32,) {
            type CallSig = ((), u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_smooth_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_triangle_fan_full(&mut self, vertices: RefArg < PackedVector3Array >, uvs: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uv2s: RefArg < PackedVector2Array >, normals: RefArg < PackedVector3Array >, tangents: RefArg < Array < Plane > >,) {
            type CallSig < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = ((), RefArg < 'a0, PackedVector3Array >, RefArg < 'a1, PackedVector2Array >, RefArg < 'a2, PackedColorArray >, RefArg < 'a3, PackedVector2Array >, RefArg < 'a4, PackedVector3Array >, RefArg < 'a5, Array < Plane > >);
            let args = (vertices, uvs, colors, uv2s, normals, tangents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "add_triangle_fan", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_triangle_fan_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_triangle_fan(&mut self, vertices: &PackedVector3Array,) {
            self.add_triangle_fan_ex(vertices,) . done()
        }
        #[inline]
        pub fn add_triangle_fan_ex < 'a > (&'a mut self, vertices: &'a PackedVector3Array,) -> ExAddTriangleFan < 'a > {
            ExAddTriangleFan::new(self, vertices,)
        }
        pub fn add_index(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "add_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn index(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deindex(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "deindex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn generate_normals_full(&mut self, flip: bool,) {
            type CallSig = ((), bool);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "generate_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::generate_normals_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn generate_normals(&mut self,) {
            self.generate_normals_ex() . done()
        }
        #[inline]
        pub fn generate_normals_ex < 'a > (&'a mut self,) -> ExGenerateNormals < 'a > {
            ExGenerateNormals::new(self,)
        }
        pub fn generate_tangents(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "generate_tangents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn optimize_indices_for_cache(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "optimize_indices_for_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn generate_lod_full(&mut self, nd_threshold: f32, target_index_count: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, f32, i32);
            let args = (nd_threshold, target_index_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "generate_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::generate_lod_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn generate_lod(&mut self, nd_threshold: f32,) -> PackedInt32Array {
            self.generate_lod_ex(nd_threshold,) . done()
        }
        #[inline]
        pub fn generate_lod_ex < 'a > (&'a mut self, nd_threshold: f32,) -> ExGenerateLod < 'a > {
            ExGenerateLod::new(self, nd_threshold,)
        }
        pub fn set_material(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_primitive_type(&self,) -> crate::classes::mesh::PrimitiveType {
            type CallSig = (crate::classes::mesh::PrimitiveType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_primitive_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_from(&mut self, existing: impl AsObjectArg < crate::classes::Mesh >, surface: i32,) {
            type CallSig = ((), ObjectArg < crate::classes::Mesh >, i32);
            let args = (existing.as_object_arg(), surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "create_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_from_arrays_full(&mut self, arrays: RefArg < VariantArray >, primitive_type: crate::classes::mesh::PrimitiveType,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >, crate::classes::mesh::PrimitiveType);
            let args = (arrays, primitive_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "create_from_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_from_arrays_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_from_arrays(&mut self, arrays: &VariantArray,) {
            self.create_from_arrays_ex(arrays,) . done()
        }
        #[inline]
        pub fn create_from_arrays_ex < 'a > (&'a mut self, arrays: &'a VariantArray,) -> ExCreateFromArrays < 'a > {
            ExCreateFromArrays::new(self, arrays,)
        }
        pub fn create_from_blend_shape(&mut self, existing: impl AsObjectArg < crate::classes::Mesh >, surface: i32, blend_shape: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Mesh >, i32, CowArg < 'a0, GString >);
            let args = (existing.as_object_arg(), surface, blend_shape.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "create_from_blend_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn append_from(&mut self, existing: impl AsObjectArg < crate::classes::Mesh >, surface: i32, transform: Transform3D,) {
            type CallSig = ((), ObjectArg < crate::classes::Mesh >, i32, Transform3D);
            let args = (existing.as_object_arg(), surface, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "append_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn commit_full(&mut self, existing: ObjectArg < crate::classes::ArrayMesh >, flags: u64,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallSig = (Option < Gd < crate::classes::ArrayMesh > >, ObjectArg < crate::classes::ArrayMesh >, u64);
            let args = (existing, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "commit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.commit_ex() . done()
        }
        #[inline]
        pub fn commit_ex < 'a > (&'a mut self,) -> ExCommit < 'a > {
            ExCommit::new(self,)
        }
        pub fn commit_to_arrays(&mut self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SurfaceTool", "commit_to_arrays", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SurfaceTool {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SurfaceTool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SurfaceTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SurfaceTool {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SurfaceTool {
        
    }
    impl crate::obj::cap::GodotDefault for SurfaceTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SurfaceTool {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SurfaceTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SurfaceTool`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SurfaceTool {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SurfaceTool > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SurfaceTool::add_triangle_fan_ex`][super::SurfaceTool::add_triangle_fan_ex]."]
#[must_use]
pub struct ExAddTriangleFan < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, vertices: CowArg < 'a, PackedVector3Array >, uvs: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, uv2s: CowArg < 'a, PackedVector2Array >, normals: CowArg < 'a, PackedVector3Array >, tangents: CowArg < 'a, Array < Plane > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTriangleFan < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, vertices: &'a PackedVector3Array,) -> Self {
        let uvs = PackedVector2Array::new();
        let colors = PackedColorArray::new();
        let uv2s = PackedVector2Array::new();
        let normals = PackedVector3Array::new();
        let tangents = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vertices: CowArg::Borrowed(vertices), uvs: CowArg::Owned(uvs), colors: CowArg::Owned(colors), uv2s: CowArg::Owned(uv2s), normals: CowArg::Owned(normals), tangents: CowArg::Owned(tangents),
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'a PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn colors(self, colors: &'a PackedColorArray) -> Self {
        Self {
            colors: CowArg::Borrowed(colors), .. self
        }
    }
    #[inline]
    pub fn uv2s(self, uv2s: &'a PackedVector2Array) -> Self {
        Self {
            uv2s: CowArg::Borrowed(uv2s), .. self
        }
    }
    #[inline]
    pub fn normals(self, normals: &'a PackedVector3Array) -> Self {
        Self {
            normals: CowArg::Borrowed(normals), .. self
        }
    }
    #[inline]
    pub fn tangents(self, tangents: &'a Array < Plane >) -> Self {
        Self {
            tangents: CowArg::Borrowed(tangents), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, vertices, uvs, colors, uv2s, normals, tangents,
        }
        = self;
        re_export::SurfaceTool::add_triangle_fan_full(surround_object, vertices.cow_as_arg(), uvs.cow_as_arg(), colors.cow_as_arg(), uv2s.cow_as_arg(), normals.cow_as_arg(), tangents.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::generate_normals_ex`][super::SurfaceTool::generate_normals_ex]."]
#[must_use]
pub struct ExGenerateNormals < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, flip: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateNormals < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool,) -> Self {
        let flip = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flip: flip,
        }
    }
    #[inline]
    pub fn flip(self, flip: bool) -> Self {
        Self {
            flip: flip, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, flip,
        }
        = self;
        re_export::SurfaceTool::generate_normals_full(surround_object, flip,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::generate_lod_ex`][super::SurfaceTool::generate_lod_ex]."]
#[must_use]
pub struct ExGenerateLod < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, nd_threshold: f32, target_index_count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateLod < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, nd_threshold: f32,) -> Self {
        let target_index_count = 3i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, nd_threshold: nd_threshold, target_index_count: target_index_count,
        }
    }
    #[inline]
    pub fn target_index_count(self, target_index_count: i32) -> Self {
        Self {
            target_index_count: target_index_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, nd_threshold, target_index_count,
        }
        = self;
        re_export::SurfaceTool::generate_lod_full(surround_object, nd_threshold, target_index_count,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::create_from_arrays_ex`][super::SurfaceTool::create_from_arrays_ex]."]
#[must_use]
pub struct ExCreateFromArrays < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, arrays: CowArg < 'a, VariantArray >, primitive_type: crate::classes::mesh::PrimitiveType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, arrays: &'a VariantArray,) -> Self {
        let primitive_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, arrays: CowArg::Borrowed(arrays), primitive_type: primitive_type,
        }
    }
    #[inline]
    pub fn primitive_type(self, primitive_type: crate::classes::mesh::PrimitiveType) -> Self {
        Self {
            primitive_type: primitive_type, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, arrays, primitive_type,
        }
        = self;
        re_export::SurfaceTool::create_from_arrays_full(surround_object, arrays.cow_as_arg(), primitive_type,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::commit_ex`][super::SurfaceTool::commit_ex]."]
#[must_use]
pub struct ExCommit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, existing: ObjectCow < crate::classes::ArrayMesh >, flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommit < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool,) -> Self {
        let existing = Gd::null_arg();
        let flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing: existing.consume_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn existing(self, existing: impl AsObjectArg < crate::classes::ArrayMesh >) -> Self {
        Self {
            existing: existing.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: u64) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, existing, flags,
        }
        = self;
        re_export::SurfaceTool::commit_full(surround_object, existing.cow_as_object_arg(), flags,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CustomFormat {
    ord: i32
}
impl CustomFormat {
    #[doc(alias = "CUSTOM_RGBA8_UNORM")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA8_UNORM`"]
    pub const RGBA8_UNORM: CustomFormat = CustomFormat {
        ord: 0i32
    };
    #[doc(alias = "CUSTOM_RGBA8_SNORM")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA8_SNORM`"]
    pub const RGBA8_SNORM: CustomFormat = CustomFormat {
        ord: 1i32
    };
    #[doc(alias = "CUSTOM_RG_HALF")]
    #[doc = "Godot enumerator name: `CUSTOM_RG_HALF`"]
    pub const RG_HALF: CustomFormat = CustomFormat {
        ord: 2i32
    };
    #[doc(alias = "CUSTOM_RGBA_HALF")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA_HALF`"]
    pub const RGBA_HALF: CustomFormat = CustomFormat {
        ord: 3i32
    };
    #[doc(alias = "CUSTOM_R_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_R_FLOAT`"]
    pub const R_FLOAT: CustomFormat = CustomFormat {
        ord: 4i32
    };
    #[doc(alias = "CUSTOM_RG_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_RG_FLOAT`"]
    pub const RG_FLOAT: CustomFormat = CustomFormat {
        ord: 5i32
    };
    #[doc(alias = "CUSTOM_RGB_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_RGB_FLOAT`"]
    pub const RGB_FLOAT: CustomFormat = CustomFormat {
        ord: 6i32
    };
    #[doc(alias = "CUSTOM_RGBA_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA_FLOAT`"]
    pub const RGBA_FLOAT: CustomFormat = CustomFormat {
        ord: 7i32
    };
    #[doc(alias = "CUSTOM_MAX")]
    #[doc = "Godot enumerator name: `CUSTOM_MAX`"]
    pub const MAX: CustomFormat = CustomFormat {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for CustomFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CustomFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CustomFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::RGBA8_UNORM => "RGBA8_UNORM", Self::RGBA8_SNORM => "RGBA8_SNORM", Self::RG_HALF => "RG_HALF", Self::RGBA_HALF => "RGBA_HALF", Self::R_FLOAT => "R_FLOAT", Self::RG_FLOAT => "RG_FLOAT", Self::RGB_FLOAT => "RGB_FLOAT", Self::RGBA_FLOAT => "RGBA_FLOAT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RGBA8_UNORM => "CUSTOM_RGBA8_UNORM", Self::RGBA8_SNORM => "CUSTOM_RGBA8_SNORM", Self::RG_HALF => "CUSTOM_RG_HALF", Self::RGBA_HALF => "CUSTOM_RGBA_HALF", Self::R_FLOAT => "CUSTOM_R_FLOAT", Self::RG_FLOAT => "CUSTOM_RG_FLOAT", Self::RGB_FLOAT => "CUSTOM_RGB_FLOAT", Self::RGBA_FLOAT => "CUSTOM_RGBA_FLOAT", Self::MAX => "CUSTOM_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for CustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for CustomFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CustomFormat {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SkinWeightCount {
    ord: i32
}
impl SkinWeightCount {
    pub const SKIN_4_WEIGHTS: SkinWeightCount = SkinWeightCount {
        ord: 0i32
    };
    pub const SKIN_8_WEIGHTS: SkinWeightCount = SkinWeightCount {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for SkinWeightCount {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SkinWeightCount") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SkinWeightCount {
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
            Self::SKIN_4_WEIGHTS => "SKIN_4_WEIGHTS", Self::SKIN_8_WEIGHTS => "SKIN_8_WEIGHTS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for SkinWeightCount {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SkinWeightCount {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SkinWeightCount {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
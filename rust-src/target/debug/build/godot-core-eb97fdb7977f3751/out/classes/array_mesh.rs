#![doc = "Sidecar module for class [`ArrayMesh`][crate::classes::ArrayMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ArrayMesh` enums](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ArrayMesh.`\n\nInherits [`Mesh`][crate::classes::Mesh].\n\nRelated symbols:\n\n* [`array_mesh`][crate::classes::array_mesh]: sidecar module with related enum/flag types\n* [`IArrayMesh`][crate::classes::IArrayMesh]: virtual methods\n\n\nSee also [Godot docs for `ArrayMesh`](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ArrayMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ArrayMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ArrayMesh`][crate::classes::ArrayMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ArrayMesh` methods](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArrayMesh: crate::obj::GodotClass < Base = ArrayMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_surface_count(&self,) -> i32;
        fn surface_get_array_len(&self, index: i32,) -> i32;
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        fn surface_get_arrays(&self, index: i32,) -> VariantArray;
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VariantArray >;
        fn surface_get_lods(&self, index: i32,) -> Dictionary;
        fn surface_get_format(&self, index: i32,) -> u32;
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        fn get_blend_shape_count(&self,) -> i32;
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        fn get_aabb(&self,) -> Aabb;
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl ArrayMesh {
        pub fn add_blend_shape(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "add_blend_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_name(&self, index: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_name(&mut self, index: i32, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_blend_shape_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_blend_shapes(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "clear_blend_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: crate::classes::mesh::BlendShapeMode,) {
            type CallSig = ((), crate::classes::mesh::BlendShapeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_mode(&self,) -> crate::classes::mesh::BlendShapeMode {
            type CallSig = (crate::classes::mesh::BlendShapeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_surface_from_arrays_full(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: RefArg < VariantArray >, blend_shapes: RefArg < Array < VariantArray > >, lods: RefArg < Dictionary >, flags: crate::classes::mesh::ArrayFormat,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), crate::classes::mesh::PrimitiveType, RefArg < 'a0, VariantArray >, RefArg < 'a1, Array < VariantArray > >, RefArg < 'a2, Dictionary >, crate::classes::mesh::ArrayFormat);
            let args = (primitive, arrays, blend_shapes, lods, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "add_surface_from_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_surface_from_arrays_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_surface_from_arrays(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &VariantArray,) {
            self.add_surface_from_arrays_ex(primitive, arrays,) . done()
        }
        #[inline]
        pub fn add_surface_from_arrays_ex < 'a > (&'a mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &'a VariantArray,) -> ExAddSurfaceFromArrays < 'a > {
            ExAddSurfaceFromArrays::new(self, primitive, arrays,)
        }
        pub fn clear_surfaces(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "clear_surfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_update_vertex_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), i32, i32, RefArg < 'a0, PackedByteArray >);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_vertex_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_update_attribute_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), i32, i32, RefArg < 'a0, PackedByteArray >);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_attribute_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_update_skin_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallSig < 'a0, > = ((), i32, i32, RefArg < 'a0, PackedByteArray >);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_skin_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_array_len(&self, surf_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_array_len", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_array_index_len(&self, surf_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_array_index_len", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_format(&self, surf_idx: i32,) -> crate::classes::mesh::ArrayFormat {
            type CallSig = (crate::classes::mesh::ArrayFormat, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_primitive_type(&self, surf_idx: i32,) -> crate::classes::mesh::PrimitiveType {
            type CallSig = (crate::classes::mesh::PrimitiveType, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_primitive_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_find_by_name(&self, name: impl AsArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_find_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_name(&mut self, surf_idx: i32, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (surf_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_set_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_name(&self, surf_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn regen_normal_maps(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "regen_normal_maps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_unwrap(&mut self, transform: Transform3D, texel_size: f32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, Transform3D, f32);
            let args = (transform, texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "lightmap_unwrap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_mesh(&mut self, mesh: impl AsObjectArg < crate::classes::ArrayMesh >,) {
            type CallSig = ((), ObjectArg < crate::classes::ArrayMesh >);
            let args = (mesh.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_shadow_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_mesh(&self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallSig = (Option < Gd < crate::classes::ArrayMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_shadow_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ArrayMesh {
        type Base = crate::classes::Mesh;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ArrayMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ArrayMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ArrayMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ArrayMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ArrayMesh {
        type Target = crate::classes::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ArrayMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ArrayMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ArrayMesh {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ArrayMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
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
#[doc = "Default-param extender for [`ArrayMesh::add_surface_from_arrays_ex`][super::ArrayMesh::add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExAddSurfaceFromArrays < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ArrayMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: CowArg < 'a, VariantArray >, blend_shapes: CowArg < 'a, Array < VariantArray > >, lods: CowArg < 'a, Dictionary >, flags: crate::classes::mesh::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSurfaceFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::ArrayMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: &'a VariantArray,) -> Self {
        let blend_shapes = Array::new();
        let lods = Dictionary::new();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), flags: flags,
        }
    }
    #[inline]
    pub fn blend_shapes(self, blend_shapes: &'a Array < VariantArray >) -> Self {
        Self {
            blend_shapes: CowArg::Borrowed(blend_shapes), .. self
        }
    }
    #[inline]
    pub fn lods(self, lods: &'a Dictionary) -> Self {
        Self {
            lods: CowArg::Borrowed(lods), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::mesh::ArrayFormat) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, primitive, arrays, blend_shapes, lods, flags,
        }
        = self;
        re_export::ArrayMesh::add_surface_from_arrays_full(surround_object, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), flags,)
    }
}
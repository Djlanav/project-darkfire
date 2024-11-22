#![doc = "Sidecar module for class [`ImporterMesh`][crate::classes::ImporterMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImporterMesh` enums](https://docs.godotengine.org/en/stable/classes/class_importermesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImporterMesh.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`importer_mesh`][crate::classes::importer_mesh]: sidecar module with related enum/flag types\n* [`IImporterMesh`][crate::classes::IImporterMesh]: virtual methods\n\n\nSee also [Godot docs for `ImporterMesh`](https://docs.godotengine.org/en/stable/classes/class_importermesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ImporterMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImporterMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ImporterMesh`][crate::classes::ImporterMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ImporterMesh` methods](https://docs.godotengine.org/en/stable/classes/class_importermesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImporterMesh: crate::obj::GodotClass < Base = ImporterMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ImporterMesh {
        pub fn add_blend_shape(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "add_blend_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_name(&self, blend_shape_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_blend_shape_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: crate::classes::mesh::BlendShapeMode,) {
            type CallSig = ((), crate::classes::mesh::BlendShapeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_mode(&self,) -> crate::classes::mesh::BlendShapeMode {
            type CallSig = (crate::classes::mesh::BlendShapeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_surface_full(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: RefArg < VariantArray >, blend_shapes: RefArg < Array < VariantArray > >, lods: RefArg < Dictionary >, material: ObjectArg < crate::classes::Material >, name: CowArg < GString >, flags: u64,) {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = ((), crate::classes::mesh::PrimitiveType, RefArg < 'a0, VariantArray >, RefArg < 'a1, Array < VariantArray > >, RefArg < 'a2, Dictionary >, ObjectArg < crate::classes::Material >, CowArg < 'a3, GString >, u64);
            let args = (primitive, arrays, blend_shapes, lods, material, name, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "add_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_surface_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_surface(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &VariantArray,) {
            self.add_surface_ex(primitive, arrays,) . done()
        }
        #[inline]
        pub fn add_surface_ex < 'a > (&'a mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &'a VariantArray,) -> ExAddSurface < 'a > {
            ExAddSurface::new(self, primitive, arrays,)
        }
        pub fn get_surface_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_primitive_type(&mut self, surface_idx: i32,) -> crate::classes::mesh::PrimitiveType {
            type CallSig = (crate::classes::mesh::PrimitiveType, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_primitive_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_name(&self, surface_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_arrays(&self, surface_idx: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_blend_shape_arrays(&self, surface_idx: i32, blend_shape_idx: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32, i32);
            let args = (surface_idx, blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_blend_shape_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_lod_count(&self, surface_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_lod_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_lod_size(&self, surface_idx: i32, lod_idx: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (surface_idx, lod_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_lod_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_lod_indices(&self, surface_idx: i32, lod_idx: i32,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, i32, i32);
            let args = (surface_idx, lod_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_lod_indices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_material(&self, surface_idx: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_format(&self, surface_idx: i32,) -> u64 {
            type CallSig = (u64, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_surface_name(&mut self, surface_idx: i32, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (surface_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_surface_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_surface_material(&mut self, surface_idx: i32, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Material >);
            let args = (surface_idx, material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_surface_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_lods(&mut self, normal_merge_angle: f32, normal_split_angle: f32, bone_transform_array: &VariantArray,) {
            type CallSig < 'a0, > = ((), f32, f32, RefArg < 'a0, VariantArray >);
            let args = (normal_merge_angle, normal_split_angle, RefArg::new(bone_transform_array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "generate_lods", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_mesh_full(&mut self, base_mesh: ObjectArg < crate::classes::ArrayMesh >,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallSig = (Option < Gd < crate::classes::ArrayMesh > >, ObjectArg < crate::classes::ArrayMesh >);
            let args = (base_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_mesh_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_mesh(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.get_mesh_ex() . done()
        }
        #[inline]
        pub fn get_mesh_ex < 'a > (&'a mut self,) -> ExGetMesh < 'a > {
            ExGetMesh::new(self,)
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_lightmap_size_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lightmap_size_hint(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_lightmap_size_hint", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ImporterMesh {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ImporterMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImporterMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ImporterMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ImporterMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ImporterMesh {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImporterMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ImporterMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ImporterMesh {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ImporterMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ImporterMesh::add_surface_ex`][super::ImporterMesh::add_surface_ex]."]
#[must_use]
pub struct ExAddSurface < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ImporterMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: CowArg < 'a, VariantArray >, blend_shapes: CowArg < 'a, Array < VariantArray > >, lods: CowArg < 'a, Dictionary >, material: ObjectCow < crate::classes::Material >, name: CowArg < 'a, GString >, flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSurface < 'a > {
    fn new(surround_object: &'a mut re_export::ImporterMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: &'a VariantArray,) -> Self {
        let blend_shapes = Array::new();
        let lods = Dictionary::new();
        let material = Gd::null_arg();
        let name = GString::from("");
        let flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), material: material.consume_arg(), name: CowArg::Owned(name), flags: flags,
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
    pub fn material(self, material: impl AsObjectArg < crate::classes::Material >) -> Self {
        Self {
            material: material.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: u64) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, primitive, arrays, blend_shapes, lods, material, name, flags,
        }
        = self;
        re_export::ImporterMesh::add_surface_full(surround_object, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), material.cow_as_object_arg(), name, flags,)
    }
}
#[doc = "Default-param extender for [`ImporterMesh::get_mesh_ex`][super::ImporterMesh::get_mesh_ex]."]
#[must_use]
pub struct ExGetMesh < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ImporterMesh, base_mesh: ObjectCow < crate::classes::ArrayMesh >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMesh < 'a > {
    fn new(surround_object: &'a mut re_export::ImporterMesh,) -> Self {
        let base_mesh = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, base_mesh: base_mesh.consume_arg(),
        }
    }
    #[inline]
    pub fn base_mesh(self, base_mesh: impl AsObjectArg < crate::classes::ArrayMesh >) -> Self {
        Self {
            base_mesh: base_mesh.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, base_mesh,
        }
        = self;
        re_export::ImporterMesh::get_mesh_full(surround_object, base_mesh.cow_as_object_arg(),)
    }
}
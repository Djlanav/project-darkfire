#![doc = "Sidecar module for class [`VoxelGiData`][crate::classes::VoxelGiData].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VoxelGIData` enums](https://docs.godotengine.org/en/stable/classes/class_voxelgidata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VoxelGIData.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IVoxelGiData`][crate::classes::IVoxelGiData]: virtual methods\n\n\nSee also [Godot docs for `VoxelGIData`](https://docs.godotengine.org/en/stable/classes/class_voxelgidata.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VoxelGiData::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VoxelGiData {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VoxelGiData`][crate::classes::VoxelGiData].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VoxelGIData` methods](https://docs.godotengine.org/en/stable/classes/class_voxelgidata.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVoxelGiData: crate::obj::GodotClass < Base = VoxelGiData > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VoxelGiData {
        pub fn allocate(&mut self, to_cell_xform: Transform3D, aabb: Aabb, octree_size: Vector3, octree_cells: &PackedByteArray, data_cells: &PackedByteArray, distance_field: &PackedByteArray, level_counts: &PackedInt32Array,) {
            type CallSig < 'a0, 'a1, 'a2, 'a3, > = ((), Transform3D, Aabb, Vector3, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >, RefArg < 'a2, PackedByteArray >, RefArg < 'a3, PackedInt32Array >);
            let args = (to_cell_xform, aabb, octree_size, RefArg::new(octree_cells), RefArg::new(data_cells), RefArg::new(distance_field), RefArg::new(level_counts),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "allocate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bounds(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_bounds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_octree_size(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_octree_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_to_cell_xform(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_to_cell_xform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_octree_cells(&self,) -> PackedByteArray {
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_octree_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data_cells(&self,) -> PackedByteArray {
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_data_cells", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_level_counts(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_level_counts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dynamic_range(&mut self, dynamic_range: f32,) {
            type CallSig = ((), f32);
            let args = (dynamic_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_dynamic_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dynamic_range(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_dynamic_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_energy(&mut self, energy: f32,) {
            type CallSig = ((), f32);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_energy(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_energy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bias(&mut self, bias: f32,) {
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bias(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normal_bias(&mut self, bias: f32,) {
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_normal_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normal_bias(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_normal_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_propagation(&mut self, propagation: f32,) {
            type CallSig = ((), f32);
            let args = (propagation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_propagation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_propagation(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_propagation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_interior(&mut self, interior: bool,) {
            type CallSig = ((), bool);
            let args = (interior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_interior(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "is_interior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_two_bounces(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_use_two_bounces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_two_bounces(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VoxelGiData", "is_using_two_bounces", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VoxelGiData {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VoxelGIData"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VoxelGiData {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VoxelGiData {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VoxelGiData {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VoxelGiData {
        
    }
    impl crate::obj::cap::GodotDefault for VoxelGiData {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VoxelGiData {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VoxelGiData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VoxelGiData`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VoxelGiData {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VoxelGiData > for $Class {
                
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
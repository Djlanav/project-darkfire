#![doc = "Sidecar module for class [`RdSamplerState`][crate::classes::RdSamplerState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDSamplerState` enums](https://docs.godotengine.org/en/stable/classes/class_rdsamplerstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDSamplerState.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IRdSamplerState`][crate::classes::IRdSamplerState]: virtual methods\n\n\nSee also [Godot docs for `RDSamplerState`](https://docs.godotengine.org/en/stable/classes/class_rdsamplerstate.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RdSamplerState::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdSamplerState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdSamplerState`][crate::classes::RdSamplerState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDSamplerState` methods](https://docs.godotengine.org/en/stable/classes/class_rdsamplerstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdSamplerState: crate::obj::GodotClass < Base = RdSamplerState > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdSamplerState {
        pub fn set_mag_filter(&mut self, p_member: crate::classes::rendering_device::SamplerFilter,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerFilter);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_mag_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mag_filter(&self,) -> crate::classes::rendering_device::SamplerFilter {
            type CallSig = (crate::classes::rendering_device::SamplerFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_mag_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_filter(&mut self, p_member: crate::classes::rendering_device::SamplerFilter,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerFilter);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_min_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_filter(&self,) -> crate::classes::rendering_device::SamplerFilter {
            type CallSig = (crate::classes::rendering_device::SamplerFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_min_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mip_filter(&mut self, p_member: crate::classes::rendering_device::SamplerFilter,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerFilter);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_mip_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mip_filter(&self,) -> crate::classes::rendering_device::SamplerFilter {
            type CallSig = (crate::classes::rendering_device::SamplerFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_mip_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat_u(&mut self, p_member: crate::classes::rendering_device::SamplerRepeatMode,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerRepeatMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_repeat_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat_u(&self,) -> crate::classes::rendering_device::SamplerRepeatMode {
            type CallSig = (crate::classes::rendering_device::SamplerRepeatMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_repeat_u", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat_v(&mut self, p_member: crate::classes::rendering_device::SamplerRepeatMode,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerRepeatMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_repeat_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat_v(&self,) -> crate::classes::rendering_device::SamplerRepeatMode {
            type CallSig = (crate::classes::rendering_device::SamplerRepeatMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_repeat_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_repeat_w(&mut self, p_member: crate::classes::rendering_device::SamplerRepeatMode,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerRepeatMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_repeat_w", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_repeat_w(&self,) -> crate::classes::rendering_device::SamplerRepeatMode {
            type CallSig = (crate::classes::rendering_device::SamplerRepeatMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_repeat_w", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lod_bias(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lod_bias(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_anisotropy(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_use_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_anisotropy(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_use_anisotropy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anisotropy_max(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_anisotropy_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anisotropy_max(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_anisotropy_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_compare(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_enable_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_compare(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_enable_compare", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_compare_op(&mut self, p_member: crate::classes::rendering_device::CompareOperator,) {
            type CallSig = ((), crate::classes::rendering_device::CompareOperator);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_compare_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_compare_op(&self,) -> crate::classes::rendering_device::CompareOperator {
            type CallSig = (crate::classes::rendering_device::CompareOperator,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_compare_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_lod(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_min_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_lod(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_min_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_lod(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_max_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_lod(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_max_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_border_color(&mut self, p_member: crate::classes::rendering_device::SamplerBorderColor,) {
            type CallSig = ((), crate::classes::rendering_device::SamplerBorderColor);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_border_color(&self,) -> crate::classes::rendering_device::SamplerBorderColor {
            type CallSig = (crate::classes::rendering_device::SamplerBorderColor,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_border_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unnormalized_uvw(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "set_unnormalized_uvw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unnormalized_uvw(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdSamplerState", "get_unnormalized_uvw", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdSamplerState {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RDSamplerState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdSamplerState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RdSamplerState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RdSamplerState {
        
    }
    impl crate::obj::cap::GodotDefault for RdSamplerState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdSamplerState {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdSamplerState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RdSamplerState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RdSamplerState {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RdSamplerState > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
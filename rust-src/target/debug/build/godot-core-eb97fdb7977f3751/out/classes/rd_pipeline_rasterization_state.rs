#![doc = "Sidecar module for class [`RdPipelineRasterizationState`][crate::classes::RdPipelineRasterizationState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDPipelineRasterizationState` enums](https://docs.godotengine.org/en/stable/classes/class_rdpipelinerasterizationstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDPipelineRasterizationState.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IRdPipelineRasterizationState`][crate::classes::IRdPipelineRasterizationState]: virtual methods\n\n\nSee also [Godot docs for `RDPipelineRasterizationState`](https://docs.godotengine.org/en/stable/classes/class_rdpipelinerasterizationstate.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RdPipelineRasterizationState::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdPipelineRasterizationState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdPipelineRasterizationState`][crate::classes::RdPipelineRasterizationState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDPipelineRasterizationState` methods](https://docs.godotengine.org/en/stable/classes/class_rdpipelinerasterizationstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdPipelineRasterizationState: crate::obj::GodotClass < Base = RdPipelineRasterizationState > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdPipelineRasterizationState {
        pub fn set_enable_depth_clamp(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_enable_depth_clamp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_depth_clamp(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_enable_depth_clamp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_discard_primitives(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_discard_primitives", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_discard_primitives(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_discard_primitives", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wireframe(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_wireframe", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wireframe(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_wireframe", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cull_mode(&mut self, p_member: crate::classes::rendering_device::PolygonCullMode,) {
            type CallSig = ((), crate::classes::rendering_device::PolygonCullMode);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cull_mode(&self,) -> crate::classes::rendering_device::PolygonCullMode {
            type CallSig = (crate::classes::rendering_device::PolygonCullMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_cull_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_front_face(&mut self, p_member: crate::classes::rendering_device::PolygonFrontFace,) {
            type CallSig = ((), crate::classes::rendering_device::PolygonFrontFace);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_front_face", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_front_face(&self,) -> crate::classes::rendering_device::PolygonFrontFace {
            type CallSig = (crate::classes::rendering_device::PolygonFrontFace,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_front_face", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_bias_enabled(&mut self, p_member: bool,) {
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_depth_bias_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_bias_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_depth_bias_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_bias_constant_factor(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_depth_bias_constant_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_bias_constant_factor(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_depth_bias_constant_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_bias_clamp(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_depth_bias_clamp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_bias_clamp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_depth_bias_clamp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth_bias_slope_factor(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_depth_bias_slope_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_bias_slope_factor(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_depth_bias_slope_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_width(&mut self, p_member: f32,) {
            type CallSig = ((), f32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_line_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_width(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_line_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_patch_control_points(&mut self, p_member: u32,) {
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "set_patch_control_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_patch_control_points(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdPipelineRasterizationState", "get_patch_control_points", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdPipelineRasterizationState {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RDPipelineRasterizationState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdPipelineRasterizationState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RdPipelineRasterizationState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RdPipelineRasterizationState {
        
    }
    impl crate::obj::cap::GodotDefault for RdPipelineRasterizationState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdPipelineRasterizationState {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdPipelineRasterizationState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RdPipelineRasterizationState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RdPipelineRasterizationState {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RdPipelineRasterizationState > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
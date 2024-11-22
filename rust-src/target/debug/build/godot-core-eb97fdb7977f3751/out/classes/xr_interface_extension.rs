#![doc = "Sidecar module for class [`XrInterfaceExtension`][crate::classes::XrInterfaceExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRInterfaceExtension` enums](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRInterfaceExtension.`\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`IXrInterfaceExtension`][crate::classes::IXrInterfaceExtension]: virtual methods\n\n\nSee also [Godot docs for `XRInterfaceExtension`](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`XrInterfaceExtension::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrInterfaceExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`XrInterfaceExtension`][crate::classes::XrInterfaceExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRInterfaceExtension` methods](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrInterfaceExtension: crate::obj::GodotClass < Base = XrInterfaceExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_name(&self,) -> StringName {
            unimplemented !()
        }
        fn get_capabilities(&self,) -> u32 {
            unimplemented !()
        }
        fn is_initialized(&self,) -> bool {
            unimplemented !()
        }
        fn initialize(&mut self,) -> bool {
            unimplemented !()
        }
        fn uninitialize(&mut self,) {
            unimplemented !()
        }
        fn get_system_info(&self,) -> Dictionary {
            unimplemented !()
        }
        fn supports_play_area_mode(&self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            unimplemented !()
        }
        fn get_play_area_mode(&self,) -> crate::classes::xr_interface::PlayAreaMode {
            unimplemented !()
        }
        fn set_play_area_mode(&self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            unimplemented !()
        }
        fn get_play_area(&self,) -> PackedVector3Array {
            unimplemented !()
        }
        fn get_render_target_size(&mut self,) -> Vector2 {
            unimplemented !()
        }
        fn get_view_count(&mut self,) -> u32 {
            unimplemented !()
        }
        fn get_camera_transform(&mut self,) -> Transform3D {
            unimplemented !()
        }
        fn get_transform_for_view(&mut self, view: u32, cam_transform: Transform3D,) -> Transform3D {
            unimplemented !()
        }
        fn get_projection_for_view(&mut self, view: u32, aspect: f64, z_near: f64, z_far: f64,) -> PackedFloat64Array {
            unimplemented !()
        }
        fn get_vrs_texture(&mut self,) -> Rid {
            unimplemented !()
        }
        fn process(&mut self,) {
            unimplemented !()
        }
        fn pre_render(&mut self,) {
            unimplemented !()
        }
        fn pre_draw_viewport(&mut self, render_target: Rid,) -> bool {
            unimplemented !()
        }
        fn post_draw_viewport(&mut self, render_target: Rid, screen_rect: Rect2,) {
            unimplemented !()
        }
        fn end_frame(&mut self,) {
            unimplemented !()
        }
        fn get_suggested_tracker_names(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_suggested_pose_names(&self, tracker_name: StringName,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_tracking_status(&self,) -> crate::classes::xr_interface::TrackingStatus {
            unimplemented !()
        }
        fn trigger_haptic_pulse(&mut self, action_name: GString, tracker_name: StringName, frequency: f64, amplitude: f64, duration_sec: f64, delay_sec: f64,) {
            unimplemented !()
        }
        fn get_anchor_detection_is_enabled(&self,) -> bool {
            unimplemented !()
        }
        fn set_anchor_detection_is_enabled(&mut self, enabled: bool,) {
            unimplemented !()
        }
        fn get_camera_feed_id(&self,) -> i32 {
            unimplemented !()
        }
        fn get_color_texture(&mut self,) -> Rid {
            unimplemented !()
        }
        fn get_depth_texture(&mut self,) -> Rid {
            unimplemented !()
        }
        fn get_velocity_texture(&mut self,) -> Rid {
            unimplemented !()
        }
    }
    impl XrInterfaceExtension {
        pub fn get_color_texture(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_texture(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_depth_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_texture(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_velocity_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_blit(&mut self, render_target: Rid, src_rect: Rect2, dst_rect: Rect2i, use_layer: bool, layer: u32, apply_lens_distortion: bool, eye_center: Vector2, k1: f64, k2: f64, upscale: f64, aspect_ratio: f64,) {
            type CallSig = ((), Rid, Rect2, Rect2i, bool, u32, bool, Vector2, f64, f64, f64, f64);
            let args = (render_target, src_rect, dst_rect, use_layer, layer, apply_lens_distortion, eye_center, k1, k2, upscale, aspect_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "add_blit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_target_texture(&mut self, render_target: Rid,) -> Rid {
            type CallSig = (Rid, Rid);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_render_target_texture", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for XrInterfaceExtension {
        type Base = crate::classes::XrInterface;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"XRInterfaceExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrInterfaceExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for XrInterfaceExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrInterfaceExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrInterfaceExtension {
        
    }
    impl crate::obj::cap::GodotDefault for XrInterfaceExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XrInterfaceExtension {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrInterfaceExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrInterfaceExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_XrInterfaceExtension {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterfaceExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterface > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
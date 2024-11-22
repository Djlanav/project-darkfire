#![doc = "Sidecar module for class [`OpenXrExtensionWrapperExtension`][crate::classes::OpenXrExtensionWrapperExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRExtensionWrapperExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrextensionwrapperextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRExtensionWrapperExtension.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`IOpenXrExtensionWrapperExtension`][crate::classes::IOpenXrExtensionWrapperExtension]: virtual methods\n\n\nSee also [Godot docs for `OpenXRExtensionWrapperExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrextensionwrapperextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OpenXrExtensionWrapperExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrExtensionWrapperExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrExtensionWrapperExtension`][crate::classes::OpenXrExtensionWrapperExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRExtensionWrapperExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrextensionwrapperextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrExtensionWrapperExtension: crate::obj::GodotClass < Base = OpenXrExtensionWrapperExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_requested_extensions(&mut self,) -> Dictionary {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_system_properties_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_instance_create_info_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_session_create_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_swapchain_create_info_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_hand_joint_locations_and_get_next_pointer(&mut self, hand_index: i32, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        fn get_composition_layer_count(&mut self,) -> i32 {
            unimplemented !()
        }
        fn get_composition_layer(&mut self, index: i32,) -> u64 {
            unimplemented !()
        }
        fn get_composition_layer_order(&mut self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn get_suggested_tracker_names(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        fn on_register_metadata(&mut self,) {
            unimplemented !()
        }
        fn on_before_instance_created(&mut self,) {
            unimplemented !()
        }
        fn on_instance_created(&mut self, instance: u64,) {
            unimplemented !()
        }
        fn on_instance_destroyed(&mut self,) {
            unimplemented !()
        }
        fn on_session_created(&mut self, session: u64,) {
            unimplemented !()
        }
        fn on_process(&mut self,) {
            unimplemented !()
        }
        fn on_pre_render(&mut self,) {
            unimplemented !()
        }
        fn on_main_swapchains_created(&mut self,) {
            unimplemented !()
        }
        fn on_session_destroyed(&mut self,) {
            unimplemented !()
        }
        fn on_state_idle(&mut self,) {
            unimplemented !()
        }
        fn on_state_ready(&mut self,) {
            unimplemented !()
        }
        fn on_state_synchronized(&mut self,) {
            unimplemented !()
        }
        fn on_state_visible(&mut self,) {
            unimplemented !()
        }
        fn on_state_focused(&mut self,) {
            unimplemented !()
        }
        fn on_state_stopping(&mut self,) {
            unimplemented !()
        }
        fn on_state_loss_pending(&mut self,) {
            unimplemented !()
        }
        fn on_state_exiting(&mut self,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn on_event_polled(&mut self, event: * const c_void,) -> bool {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_viewport_composition_layer_and_get_next_pointer(&mut self, layer: * const c_void, property_values: Dictionary, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        fn get_viewport_composition_layer_extension_properties(&mut self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_viewport_composition_layer_extension_property_defaults(&mut self,) -> Dictionary {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn on_viewport_composition_layer_destroyed(&mut self, layer: * const c_void,) {
            unimplemented !()
        }
    }
    impl OpenXrExtensionWrapperExtension {
        pub fn get_openxr_api(&mut self,) -> Option < Gd < crate::classes::OpenXrApiExtension > > {
            type CallSig = (Option < Gd < crate::classes::OpenXrApiExtension > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(99usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrExtensionWrapperExtension", "get_openxr_api", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_extension_wrapper(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrExtensionWrapperExtension", "register_extension_wrapper", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrExtensionWrapperExtension {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OpenXRExtensionWrapperExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrExtensionWrapperExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrExtensionWrapperExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrExtensionWrapperExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrExtensionWrapperExtension {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrExtensionWrapperExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrExtensionWrapperExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_OpenXrExtensionWrapperExtension {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrExtensionWrapperExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
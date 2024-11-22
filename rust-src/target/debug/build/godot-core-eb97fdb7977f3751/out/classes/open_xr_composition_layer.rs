#![doc = "Sidecar module for class [`OpenXrCompositionLayer`][crate::classes::OpenXrCompositionLayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRCompositionLayer` enums](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRCompositionLayer.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`IOpenXrCompositionLayer`][crate::classes::IOpenXrCompositionLayer]: virtual methods\n\n\nSee also [Godot docs for `OpenXRCompositionLayer`](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<OpenXrCompositionLayer>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrCompositionLayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrCompositionLayer`][crate::classes::OpenXrCompositionLayer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRCompositionLayer` methods](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrCompositionLayer: crate::obj::GodotClass < Base = OpenXrCompositionLayer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    impl OpenXrCompositionLayer {
        pub fn set_layer_viewport(&mut self, viewport: impl AsObjectArg < crate::classes::SubViewport >,) {
            type CallSig = ((), ObjectArg < crate::classes::SubViewport >);
            let args = (viewport.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_layer_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_viewport(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallSig = (Option < Gd < crate::classes::SubViewport > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_layer_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_hole_punch(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_enable_hole_punch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_hole_punch(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_enable_hole_punch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sort_order(&mut self, order: i32,) {
            type CallSig = ((), i32);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_sort_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sort_order(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_sort_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_blend(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_alpha_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_blend(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_alpha_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_natively_supported(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "is_natively_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn intersects_ray(&self, origin: Vector3, direction: Vector3,) -> Vector2 {
            type CallSig = (Vector2, Vector3, Vector3);
            let args = (origin, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "intersects_ray", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrCompositionLayer {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OpenXRCompositionLayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrCompositionLayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for OpenXrCompositionLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OpenXrCompositionLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrCompositionLayer {
        
    }
    impl std::ops::Deref for OpenXrCompositionLayer {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrCompositionLayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrCompositionLayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_OpenXrCompositionLayer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrCompositionLayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
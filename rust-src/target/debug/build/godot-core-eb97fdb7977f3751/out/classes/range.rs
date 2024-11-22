#![doc = "Sidecar module for class [`Range`][crate::classes::Range].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Range` enums](https://docs.godotengine.org/en/stable/classes/class_range.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Range.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`IRange`][crate::classes::IRange]: virtual methods\n\n\nSee also [Godot docs for `Range`](https://docs.godotengine.org/en/stable/classes/class_range.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Range::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Range {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Range`][crate::classes::Range].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Range` methods](https://docs.godotengine.org/en/stable/classes/class_range.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRange: crate::obj::GodotClass < Base = Range > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn value_changed(&mut self, new_value: f64,) {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl Range {
        pub fn get_value(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "get_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_page(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6805usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "get_page", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_as_ratio(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6806usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "get_as_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_value(&mut self, value: f64,) {
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_value_no_signal(&mut self, value: f64,) {
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_value_no_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, minimum: f64,) {
            type CallSig = ((), f64);
            let args = (minimum,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, maximum: f64,) {
            type CallSig = ((), f64);
            let args = (maximum,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_step(&mut self, step: f64,) {
            type CallSig = ((), f64);
            let args = (step,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_page(&mut self, pagesize: f64,) {
            type CallSig = ((), f64);
            let args = (pagesize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_page", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_ratio(&mut self, value: f64,) {
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_as_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_rounded_values(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_use_rounded_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_rounded_values(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "is_using_rounded_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exp_ratio(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_exp_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ratio_exp(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "is_ratio_exp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_greater(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_allow_greater", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_greater_allowed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "is_greater_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_lesser(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "set_allow_lesser", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_lesser_allowed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "is_lesser_allowed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn share(&mut self, with: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (with.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "share", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unshare(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Range", "unshare", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Range {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Range"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Range {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Range {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Range {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Range {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Range {
        
    }
    impl crate::obj::cap::GodotDefault for Range {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Range {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Range {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Range`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Range {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Range > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#![doc = "Sidecar module for class [`Button`][crate::classes::Button].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Button` enums](https://docs.godotengine.org/en/stable/classes/class_button.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Button.`\n\nInherits [`BaseButton`][crate::classes::BaseButton].\n\nRelated symbols:\n\n* [`IButton`][crate::classes::IButton]: virtual methods\n\n\nSee also [Godot docs for `Button`](https://docs.godotengine.org/en/stable/classes/class_button.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Button::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Button {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Button`][crate::classes::Button].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Button` methods](https://docs.godotengine.org/en/stable/classes/class_button.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IButton: crate::obj::GodotClass < Base = Button > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl Button {
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallSig = ((), crate::classes::text_server::OverrunBehavior);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallSig = (crate::classes::text_server::OverrunBehavior,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallSig = ((), crate::classes::text_server::AutowrapMode);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallSig = (crate::classes::text_server::AutowrapMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), crate::classes::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_icon(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flat(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_flat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flat(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "is_flat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_text(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_clip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_text(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_clip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), crate::global::HorizontalAlignment);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_text_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_text_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_alignment(&mut self, icon_alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), crate::global::HorizontalAlignment);
            let args = (icon_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_icon_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_icon_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertical_icon_alignment(&mut self, vertical_icon_alignment: crate::global::VerticalAlignment,) {
            type CallSig = ((), crate::global::VerticalAlignment);
            let args = (vertical_icon_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_vertical_icon_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertical_icon_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallSig = (crate::global::VerticalAlignment,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "get_vertical_icon_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_icon(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "set_expand_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_expand_icon(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Button", "is_expand_icon", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Button {
        type Base = crate::classes::BaseButton;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Button"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Button {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BaseButton > for Button {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Button {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Button {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Button {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Button {
        
    }
    impl crate::obj::cap::GodotDefault for Button {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Button {
        type Target = crate::classes::BaseButton;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Button {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Button`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Button {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Button > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
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
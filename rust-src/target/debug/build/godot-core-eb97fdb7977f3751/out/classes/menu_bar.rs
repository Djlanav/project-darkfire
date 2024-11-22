#![doc = "Sidecar module for class [`MenuBar`][crate::classes::MenuBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MenuBar` enums](https://docs.godotengine.org/en/stable/classes/class_menubar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MenuBar.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`IMenuBar`][crate::classes::IMenuBar]: virtual methods\n\n\nSee also [Godot docs for `MenuBar`](https://docs.godotengine.org/en/stable/classes/class_menubar.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MenuBar::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MenuBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MenuBar`][crate::classes::MenuBar].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MenuBar` methods](https://docs.godotengine.org/en/stable/classes/class_menubar.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMenuBar: crate::obj::GodotClass < Base = MenuBar > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MenuBar {
        pub fn set_switch_on_hover(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_switch_on_hover", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_switch_on_hover(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "is_switch_on_hover", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_shortcuts(&mut self, disabled: bool,) {
            type CallSig = ((), bool);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_disable_shortcuts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_prefer_global_menu(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_prefer_global_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_prefer_global_menu(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "is_prefer_global_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_native_menu(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "is_native_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), crate::classes::control::TextDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flat(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_flat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flat(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "is_flat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_start_index(&mut self, enabled: i32,) {
            type CallSig = ((), i32);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_start_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_start_index(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_start_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_menu_title(&mut self, menu: i32, title: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (menu, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu_title(&self, menu: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_menu_tooltip(&mut self, menu: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (menu, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu_tooltip(&self, menu: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_menu_disabled(&mut self, menu: i32, disabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (menu, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_menu_disabled(&self, menu: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "is_menu_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_menu_hidden(&mut self, menu: i32, hidden: bool,) {
            type CallSig = ((), i32, bool);
            let args = (menu, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_menu_hidden(&self, menu: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "is_menu_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_menu_popup(&self, menu: i32,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallSig = (Option < Gd < crate::classes::PopupMenu > >, i32);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_popup", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MenuBar {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"MenuBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MenuBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for MenuBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for MenuBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MenuBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MenuBar {
        
    }
    impl crate::obj::cap::GodotDefault for MenuBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MenuBar {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MenuBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MenuBar`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_MenuBar {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MenuBar > for $Class {
                
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
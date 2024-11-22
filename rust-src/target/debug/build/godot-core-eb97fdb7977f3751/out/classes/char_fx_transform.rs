#![doc = "Sidecar module for class [`CharFxTransform`][crate::classes::CharFxTransform].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CharFXTransform` enums](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CharFXTransform.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`ICharFxTransform`][crate::classes::ICharFxTransform]: virtual methods\n\n\nSee also [Godot docs for `CharFXTransform`](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CharFxTransform::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CharFxTransform {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CharFxTransform`][crate::classes::CharFxTransform].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CharFXTransform` methods](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICharFxTransform: crate::obj::GodotClass < Base = CharFxTransform > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CharFxTransform {
        pub fn get_transform(&mut self,) -> Transform2D {
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform2D,) {
            type CallSig = ((), Transform2D);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_range(&mut self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_range(&mut self, range: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_elapsed_time(&mut self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_elapsed_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_elapsed_time(&mut self, time: f64,) {
            type CallSig = ((), f64);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_elapsed_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility(&mut self, visibility: bool,) {
            type CallSig = ((), bool);
            let args = (visibility,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_visibility", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_outline(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "is_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outline(&mut self, outline: bool,) {
            type CallSig = ((), bool);
            let args = (outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_outline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&mut self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color(&mut self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment(&mut self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment(&mut self, environment: &Dictionary,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(environment),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_environment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_index(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_index(&mut self, glyph_index: u32,) {
            type CallSig = ((), u32);
            let args = (glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_glyph_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_relative_index(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_relative_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_relative_index(&mut self, relative_index: i32,) {
            type CallSig = ((), i32);
            let args = (relative_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_relative_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_count(&self,) -> u8 {
            type CallSig = (u8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_glyph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_count(&mut self, glyph_count: u8,) {
            type CallSig = ((), u8);
            let args = (glyph_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_glyph_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_glyph_flags(&self,) -> u16 {
            type CallSig = (u16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_glyph_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_glyph_flags(&mut self, glyph_flags: u16,) {
            type CallSig = ((), u16);
            let args = (glyph_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_glyph_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font(&mut self, font: Rid,) {
            type CallSig = ((), Rid);
            let args = (font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_font", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CharFxTransform {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CharFXTransform"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CharFxTransform {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CharFxTransform {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CharFxTransform {
        
    }
    impl crate::obj::cap::GodotDefault for CharFxTransform {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CharFxTransform {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CharFxTransform {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CharFxTransform`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CharFxTransform {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CharFxTransform > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
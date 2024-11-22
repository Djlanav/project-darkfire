#![doc = "Sidecar module for class [`SystemFont`][crate::classes::SystemFont].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SystemFont` enums](https://docs.godotengine.org/en/stable/classes/class_systemfont.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SystemFont.`\n\nInherits [`Font`][crate::classes::Font].\n\nRelated symbols:\n\n* [`ISystemFont`][crate::classes::ISystemFont]: virtual methods\n\n\nSee also [Godot docs for `SystemFont`](https://docs.godotengine.org/en/stable/classes/class_systemfont.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SystemFont::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SystemFont {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SystemFont`][crate::classes::SystemFont].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SystemFont` methods](https://docs.godotengine.org/en/stable/classes/class_systemfont.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISystemFont: crate::obj::GodotClass < Base = SystemFont > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SystemFont {
        pub fn set_antialiasing(&mut self, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallSig = ((), crate::classes::text_server::FontAntialiasing);
            let args = (antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_antialiasing(&self,) -> crate::classes::text_server::FontAntialiasing {
            type CallSig = (crate::classes::text_server::FontAntialiasing,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_antialiasing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_embedded_bitmaps(&mut self, disable_embedded_bitmaps: bool,) {
            type CallSig = ((), bool);
            let args = (disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_disable_embedded_bitmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_disable_embedded_bitmaps(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_disable_embedded_bitmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool,) {
            type CallSig = ((), bool);
            let args = (generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_generate_mipmaps(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool,) {
            type CallSig = ((), bool);
            let args = (allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_allow_system_fallback(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "is_allow_system_fallback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool,) {
            type CallSig = ((), bool);
            let args = (force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_force_autohinter(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "is_force_autohinter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hinting(&mut self, hinting: crate::classes::text_server::Hinting,) {
            type CallSig = ((), crate::classes::text_server::Hinting);
            let args = (hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hinting(&self,) -> crate::classes::text_server::Hinting {
            type CallSig = (crate::classes::text_server::Hinting,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_hinting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subpixel_positioning(&mut self, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallSig = ((), crate::classes::text_server::SubpixelPositioning);
            let args = (subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subpixel_positioning(&self,) -> crate::classes::text_server::SubpixelPositioning {
            type CallSig = (crate::classes::text_server::SubpixelPositioning,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_subpixel_positioning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool,) {
            type CallSig = ((), bool);
            let args = (msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multichannel_signed_distance_field(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "is_multichannel_signed_distance_field", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i32,) {
            type CallSig = ((), i32);
            let args = (msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_msdf_pixel_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i32,) {
            type CallSig = ((), i32);
            let args = (msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msdf_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_msdf_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f32,) {
            type CallSig = ((), f32);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_oversampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_names(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_font_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_names(&mut self, names: &PackedStringArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedStringArray >);
            let args = (RefArg::new(names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_names", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_font_italic(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "get_font_italic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_italic(&mut self, italic: bool,) {
            type CallSig = ((), bool);
            let args = (italic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_italic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_weight(&mut self, weight: i32,) {
            type CallSig = ((), i32);
            let args = (weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i32,) {
            type CallSig = ((), i32);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_stretch", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SystemFont {
        type Base = crate::classes::Font;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SystemFont"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SystemFont {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Font > for SystemFont {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SystemFont {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SystemFont {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SystemFont {
        
    }
    impl crate::obj::cap::GodotDefault for SystemFont {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SystemFont {
        type Target = crate::classes::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SystemFont {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SystemFont`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SystemFont {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SystemFont > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Font > for $Class {
                
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
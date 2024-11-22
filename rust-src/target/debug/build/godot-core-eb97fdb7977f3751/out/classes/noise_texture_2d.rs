#![doc = "Sidecar module for class [`NoiseTexture2D`][crate::classes::NoiseTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NoiseTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_noisetexture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NoiseTexture2D.`\n\nInherits [`Texture2D`][crate::classes::Texture2D].\n\nRelated symbols:\n\n* [`INoiseTexture2D`][crate::classes::INoiseTexture2D]: virtual methods\n\n\nSee also [Godot docs for `NoiseTexture2D`](https://docs.godotengine.org/en/stable/classes/class_noisetexture2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`NoiseTexture2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NoiseTexture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`NoiseTexture2D`][crate::classes::NoiseTexture2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `NoiseTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_noisetexture2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INoiseTexture2D: crate::obj::GodotClass < Base = NoiseTexture2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_width(&self,) -> i32;
        fn get_height(&self,) -> i32;
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl NoiseTexture2D {
        pub fn set_width(&mut self, width: i32,) {
            type CallSig = ((), i32);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: i32,) {
            type CallSig = ((), i32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_invert(&mut self, invert: bool,) {
            type CallSig = ((), bool);
            let args = (invert,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_invert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_invert(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_invert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_in_3d_space(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_in_3d_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_3d_space(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_in_3d_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, invert: bool,) {
            type CallSig = ((), bool);
            let args = (invert,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_generate_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_generating_mipmaps(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_generating_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_seamless(&mut self, seamless: bool,) {
            type CallSig = ((), bool);
            let args = (seamless,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_seamless", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seamless(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_seamless", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_seamless_blend_skirt(&mut self, seamless_blend_skirt: f32,) {
            type CallSig = ((), f32);
            let args = (seamless_blend_skirt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_seamless_blend_skirt", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seamless_blend_skirt(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_seamless_blend_skirt", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_normal_map(&mut self, as_normal_map: bool,) {
            type CallSig = ((), bool);
            let args = (as_normal_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_as_normal_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_normal_map(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_normal_map", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bump_strength(&mut self, bump_strength: f32,) {
            type CallSig = ((), f32);
            let args = (bump_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_bump_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bump_strength(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_bump_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normalize(&mut self, normalize: bool,) {
            type CallSig = ((), bool);
            let args = (normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_normalize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_normalized(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_normalized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_ramp(&mut self, gradient: impl AsObjectArg < crate::classes::Gradient >,) {
            type CallSig = ((), ObjectArg < crate::classes::Gradient >);
            let args = (gradient.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallSig = (Option < Gd < crate::classes::Gradient > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_color_ramp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_noise(&mut self, noise: impl AsObjectArg < crate::classes::Noise >,) {
            type CallSig = ((), ObjectArg < crate::classes::Noise >);
            let args = (noise.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_noise", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise(&mut self,) -> Option < Gd < crate::classes::Noise > > {
            type CallSig = (Option < Gd < crate::classes::Noise > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_noise", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for NoiseTexture2D {
        type Base = crate::classes::Texture2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"NoiseTexture2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NoiseTexture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture2D > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NoiseTexture2D {
        
    }
    impl crate::obj::cap::GodotDefault for NoiseTexture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for NoiseTexture2D {
        type Target = crate::classes::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NoiseTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`NoiseTexture2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_NoiseTexture2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::NoiseTexture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
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
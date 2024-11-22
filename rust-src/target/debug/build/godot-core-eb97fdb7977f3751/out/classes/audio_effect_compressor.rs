#![doc = "Sidecar module for class [`AudioEffectCompressor`][crate::classes::AudioEffectCompressor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectCompressor` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectcompressor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectCompressor.`\n\nInherits [`AudioEffect`][crate::classes::AudioEffect].\n\nRelated symbols:\n\n* [`IAudioEffectCompressor`][crate::classes::IAudioEffectCompressor]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectCompressor`](https://docs.godotengine.org/en/stable/classes/class_audioeffectcompressor.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioEffectCompressor::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectCompressor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectCompressor`][crate::classes::AudioEffectCompressor].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectCompressor` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectcompressor.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectCompressor: crate::obj::GodotClass < Base = AudioEffectCompressor > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn instantiate(&mut self,) -> Option < Gd < crate::classes::AudioEffectInstance > >;
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AudioEffectCompressor {
        pub fn set_threshold(&mut self, threshold: f32,) {
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_threshold(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gain(&mut self, gain: f32,) {
            type CallSig = ((), f32);
            let args = (gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gain(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attack_us(&mut self, attack_us: f32,) {
            type CallSig = ((), f32);
            let args = (attack_us,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_attack_us", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attack_us(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_attack_us", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_release_ms(&mut self, release_ms: f32,) {
            type CallSig = ((), f32);
            let args = (release_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_release_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_release_ms(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_release_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mix(&mut self, mix: f32,) {
            type CallSig = ((), f32);
            let args = (mix,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mix(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sidechain(&mut self, sidechain: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (sidechain.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "set_sidechain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sidechain(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectCompressor", "get_sidechain", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioEffectCompressor {
        type Base = crate::classes::AudioEffect;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioEffectCompressor"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectCompressor {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffect > for AudioEffectCompressor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioEffectCompressor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectCompressor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectCompressor {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectCompressor {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectCompressor {
        type Target = crate::classes::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectCompressor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectCompressor`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioEffectCompressor {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectCompressor > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffect > for $Class {
                
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
#![doc = "Sidecar module for class [`AudioEffectChorus`][crate::classes::AudioEffectChorus].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectChorus` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectchorus.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectChorus.`\n\nInherits [`AudioEffect`][crate::classes::AudioEffect].\n\nRelated symbols:\n\n* [`IAudioEffectChorus`][crate::classes::IAudioEffectChorus]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectChorus`](https://docs.godotengine.org/en/stable/classes/class_audioeffectchorus.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioEffectChorus::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectChorus {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectChorus`][crate::classes::AudioEffectChorus].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectChorus` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectchorus.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectChorus: crate::obj::GodotClass < Base = AudioEffectChorus > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioEffectChorus {
        pub fn set_voice_count(&mut self, voices: i32,) {
            type CallSig = ((), i32);
            let args = (voices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_delay_ms(&mut self, voice_idx: i32, delay_ms: f32,) {
            type CallSig = ((), i32, f32);
            let args = (voice_idx, delay_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_delay_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_delay_ms(&self, voice_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_delay_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_rate_hz(&mut self, voice_idx: i32, rate_hz: f32,) {
            type CallSig = ((), i32, f32);
            let args = (voice_idx, rate_hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_rate_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_rate_hz(&self, voice_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_rate_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_depth_ms(&mut self, voice_idx: i32, depth_ms: f32,) {
            type CallSig = ((), i32, f32);
            let args = (voice_idx, depth_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_depth_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_depth_ms(&self, voice_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_depth_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_level_db(&mut self, voice_idx: i32, level_db: f32,) {
            type CallSig = ((), i32, f32);
            let args = (voice_idx, level_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_level_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_level_db(&self, voice_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_level_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_cutoff_hz(&mut self, voice_idx: i32, cutoff_hz: f32,) {
            type CallSig = ((), i32, f32);
            let args = (voice_idx, cutoff_hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_cutoff_hz(&self, voice_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_pan(&mut self, voice_idx: i32, pan: f32,) {
            type CallSig = ((), i32, f32);
            let args = (voice_idx, pan,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_voice_pan", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_pan(&self, voice_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_voice_pan", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wet(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_wet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wet(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_wet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dry(&mut self, amount: f32,) {
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "set_dry", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dry(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectChorus", "get_dry", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioEffectChorus {
        type Base = crate::classes::AudioEffect;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioEffectChorus"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectChorus {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffect > for AudioEffectChorus {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioEffectChorus {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectChorus {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectChorus {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectChorus {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectChorus {
        type Target = crate::classes::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectChorus {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectChorus`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioEffectChorus {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectChorus > for $Class {
                
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
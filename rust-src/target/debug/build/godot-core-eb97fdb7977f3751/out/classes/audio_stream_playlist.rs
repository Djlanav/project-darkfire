#![doc = "Sidecar module for class [`AudioStreamPlaylist`][crate::classes::AudioStreamPlaylist].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlaylist` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaylist.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlaylist.`\n\nInherits [`AudioStream`][crate::classes::AudioStream].\n\nRelated symbols:\n\n* [`IAudioStreamPlaylist`][crate::classes::IAudioStreamPlaylist]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlaylist`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaylist.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamPlaylist::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlaylist {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamPlaylist`][crate::classes::AudioStreamPlaylist].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamPlaylist` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaylist.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlaylist: crate::obj::GodotClass < Base = AudioStreamPlaylist > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn instantiate_playback(&self,) -> Option < Gd < crate::classes::AudioStreamPlayback > > {
            unimplemented !()
        }
        fn get_stream_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_length(&self,) -> f64 {
            unimplemented !()
        }
        fn is_monophonic(&self,) -> bool {
            unimplemented !()
        }
        fn get_bpm(&self,) -> f64 {
            unimplemented !()
        }
        fn get_beat_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AudioStreamPlaylist {
        pub fn set_stream_count(&mut self, stream_count: i32,) {
            type CallSig = ((), i32);
            let args = (stream_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "set_stream_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "get_stream_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bpm(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "get_bpm", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_list_stream(&mut self, stream_index: i32, audio_stream: impl AsObjectArg < crate::classes::AudioStream >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::AudioStream >);
            let args = (stream_index, audio_stream.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "set_list_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_list_stream(&self, stream_index: i32,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallSig = (Option < Gd < crate::classes::AudioStream > >, i32);
            let args = (stream_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "get_list_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shuffle(&mut self, shuffle: bool,) {
            type CallSig = ((), bool);
            let args = (shuffle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "set_shuffle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shuffle(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "get_shuffle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fade_time(&mut self, dec: f32,) {
            type CallSig = ((), f32);
            let args = (dec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "set_fade_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fade_time(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "get_fade_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_loop(&mut self, loop_: bool,) {
            type CallSig = ((), bool);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "set_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_loop(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamPlaylist", "has_loop", self.object_ptr, self.__checked_id(), args,)
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
        pub const MAX_STREAMS: i32 = 64i32;
        
    }
    impl crate::obj::GodotClass for AudioStreamPlaylist {
        type Base = crate::classes::AudioStream;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioStreamPlaylist"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlaylist {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStream > for AudioStreamPlaylist {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStreamPlaylist {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamPlaylist {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlaylist {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlaylist {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlaylist {
        type Target = crate::classes::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlaylist {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamPlaylist`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioStreamPlaylist {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlaylist > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStream > for $Class {
                
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
#![doc = "Sidecar module for class [`AudioStreamInteractive`][crate::classes::AudioStreamInteractive].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamInteractive` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamInteractive.`\n\nInherits [`AudioStream`][crate::classes::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_interactive`][crate::classes::audio_stream_interactive]: sidecar module with related enum/flag types\n* [`IAudioStreamInteractive`][crate::classes::IAudioStreamInteractive]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamInteractive`](https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamInteractive::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamInteractive {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamInteractive`][crate::classes::AudioStreamInteractive].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamInteractive` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamInteractive: crate::obj::GodotClass < Base = AudioStreamInteractive > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStreamInteractive {
        pub fn set_clip_count(&mut self, clip_count: i32,) {
            type CallSig = ((), i32);
            let args = (clip_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_initial_clip(&mut self, clip_index: i32,) {
            type CallSig = ((), i32);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_initial_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_initial_clip(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_initial_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_name(&mut self, clip_index: i32, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (clip_index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_name(&self, clip_index: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_stream(&mut self, clip_index: i32, stream: impl AsObjectArg < crate::classes::AudioStream >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::AudioStream >);
            let args = (clip_index, stream.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_stream(&self, clip_index: i32,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallSig = (Option < Gd < crate::classes::AudioStream > >, i32);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_auto_advance(&mut self, clip_index: i32, mode: crate::classes::audio_stream_interactive::AutoAdvanceMode,) {
            type CallSig = ((), i32, crate::classes::audio_stream_interactive::AutoAdvanceMode);
            let args = (clip_index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_auto_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_auto_advance(&self, clip_index: i32,) -> crate::classes::audio_stream_interactive::AutoAdvanceMode {
            type CallSig = (crate::classes::audio_stream_interactive::AutoAdvanceMode, i32);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_auto_advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_auto_advance_next_clip(&mut self, clip_index: i32, auto_advance_next_clip: i32,) {
            type CallSig = ((), i32, i32);
            let args = (clip_index, auto_advance_next_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_auto_advance_next_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_auto_advance_next_clip(&self, clip_index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_auto_advance_next_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_transition_full(&mut self, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32, use_filler_clip: bool, filler_clip: i32, hold_previous: bool,) {
            type CallSig = ((), i32, i32, crate::classes::audio_stream_interactive::TransitionFromTime, crate::classes::audio_stream_interactive::TransitionToTime, crate::classes::audio_stream_interactive::FadeMode, f32, bool, i32, bool);
            let args = (from_clip, to_clip, from_time, to_time, fade_mode, fade_beats, use_filler_clip, filler_clip, hold_previous,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "add_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_transition_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_transition(&mut self, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32,) {
            self.add_transition_ex(from_clip, to_clip, from_time, to_time, fade_mode, fade_beats,) . done()
        }
        #[inline]
        pub fn add_transition_ex < 'a > (&'a mut self, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32,) -> ExAddTransition < 'a > {
            ExAddTransition::new(self, from_clip, to_clip, from_time, to_time, fade_mode, fade_beats,)
        }
        pub fn has_transition(&self, from_clip: i32, to_clip: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "has_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_transition(&mut self, from_clip: i32, to_clip: i32,) {
            type CallSig = ((), i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "erase_transition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_list(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_from_time(&self, from_clip: i32, to_clip: i32,) -> crate::classes::audio_stream_interactive::TransitionFromTime {
            type CallSig = (crate::classes::audio_stream_interactive::TransitionFromTime, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_from_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_to_time(&self, from_clip: i32, to_clip: i32,) -> crate::classes::audio_stream_interactive::TransitionToTime {
            type CallSig = (crate::classes::audio_stream_interactive::TransitionToTime, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_to_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_fade_mode(&self, from_clip: i32, to_clip: i32,) -> crate::classes::audio_stream_interactive::FadeMode {
            type CallSig = (crate::classes::audio_stream_interactive::FadeMode, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_fade_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_fade_beats(&self, from_clip: i32, to_clip: i32,) -> f32 {
            type CallSig = (f32, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_fade_beats", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_transition_using_filler_clip(&self, from_clip: i32, to_clip: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "is_transition_using_filler_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transition_filler_clip(&self, from_clip: i32, to_clip: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_filler_clip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_transition_holding_previous(&self, from_clip: i32, to_clip: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "is_transition_holding_previous", self.object_ptr, self.__checked_id(), args,)
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
        pub const CLIP_ANY: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for AudioStreamInteractive {
        type Base = crate::classes::AudioStream;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioStreamInteractive"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamInteractive {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStream > for AudioStreamInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStreamInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamInteractive {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamInteractive {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamInteractive {
        type Target = crate::classes::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamInteractive {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamInteractive`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioStreamInteractive {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamInteractive > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamInteractive::add_transition_ex`][super::AudioStreamInteractive::add_transition_ex]."]
#[must_use]
pub struct ExAddTransition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamInteractive, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32, use_filler_clip: bool, filler_clip: i32, hold_previous: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTransition < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamInteractive, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32,) -> Self {
        let use_filler_clip = false;
        let filler_clip = - 1i32;
        let hold_previous = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_clip: from_clip, to_clip: to_clip, from_time: from_time, to_time: to_time, fade_mode: fade_mode, fade_beats: fade_beats, use_filler_clip: use_filler_clip, filler_clip: filler_clip, hold_previous: hold_previous,
        }
    }
    #[inline]
    pub fn use_filler_clip(self, use_filler_clip: bool) -> Self {
        Self {
            use_filler_clip: use_filler_clip, .. self
        }
    }
    #[inline]
    pub fn filler_clip(self, filler_clip: i32) -> Self {
        Self {
            filler_clip: filler_clip, .. self
        }
    }
    #[inline]
    pub fn hold_previous(self, hold_previous: bool) -> Self {
        Self {
            hold_previous: hold_previous, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_clip, to_clip, from_time, to_time, fade_mode, fade_beats, use_filler_clip, filler_clip, hold_previous,
        }
        = self;
        re_export::AudioStreamInteractive::add_transition_full(surround_object, from_clip, to_clip, from_time, to_time, fade_mode, fade_beats, use_filler_clip, filler_clip, hold_previous,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionFromTime {
    ord: i32
}
impl TransitionFromTime {
    #[doc(alias = "TRANSITION_FROM_TIME_IMMEDIATE")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_IMMEDIATE`"]
    pub const IMMEDIATE: TransitionFromTime = TransitionFromTime {
        ord: 0i32
    };
    #[doc(alias = "TRANSITION_FROM_TIME_NEXT_BEAT")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_NEXT_BEAT`"]
    pub const NEXT_BEAT: TransitionFromTime = TransitionFromTime {
        ord: 1i32
    };
    #[doc(alias = "TRANSITION_FROM_TIME_NEXT_BAR")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_NEXT_BAR`"]
    pub const NEXT_BAR: TransitionFromTime = TransitionFromTime {
        ord: 2i32
    };
    #[doc(alias = "TRANSITION_FROM_TIME_END")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_END`"]
    pub const END: TransitionFromTime = TransitionFromTime {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TransitionFromTime {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionFromTime") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionFromTime {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::IMMEDIATE => "IMMEDIATE", Self::NEXT_BEAT => "NEXT_BEAT", Self::NEXT_BAR => "NEXT_BAR", Self::END => "END", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::IMMEDIATE => "TRANSITION_FROM_TIME_IMMEDIATE", Self::NEXT_BEAT => "TRANSITION_FROM_TIME_NEXT_BEAT", Self::NEXT_BAR => "TRANSITION_FROM_TIME_NEXT_BAR", Self::END => "TRANSITION_FROM_TIME_END", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TransitionFromTime {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransitionFromTime {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionFromTime {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionToTime {
    ord: i32
}
impl TransitionToTime {
    #[doc(alias = "TRANSITION_TO_TIME_SAME_POSITION")]
    #[doc = "Godot enumerator name: `TRANSITION_TO_TIME_SAME_POSITION`"]
    pub const SAME_POSITION: TransitionToTime = TransitionToTime {
        ord: 0i32
    };
    #[doc(alias = "TRANSITION_TO_TIME_START")]
    #[doc = "Godot enumerator name: `TRANSITION_TO_TIME_START`"]
    pub const START: TransitionToTime = TransitionToTime {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TransitionToTime {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionToTime") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionToTime {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SAME_POSITION => "SAME_POSITION", Self::START => "START", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SAME_POSITION => "TRANSITION_TO_TIME_SAME_POSITION", Self::START => "TRANSITION_TO_TIME_START", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TransitionToTime {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransitionToTime {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionToTime {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FadeMode {
    ord: i32
}
impl FadeMode {
    #[doc(alias = "FADE_DISABLED")]
    #[doc = "Godot enumerator name: `FADE_DISABLED`"]
    pub const DISABLED: FadeMode = FadeMode {
        ord: 0i32
    };
    #[doc(alias = "FADE_IN")]
    #[doc = "Godot enumerator name: `FADE_IN`"]
    pub const IN: FadeMode = FadeMode {
        ord: 1i32
    };
    #[doc(alias = "FADE_OUT")]
    #[doc = "Godot enumerator name: `FADE_OUT`"]
    pub const OUT: FadeMode = FadeMode {
        ord: 2i32
    };
    #[doc(alias = "FADE_CROSS")]
    #[doc = "Godot enumerator name: `FADE_CROSS`"]
    pub const CROSS: FadeMode = FadeMode {
        ord: 3i32
    };
    #[doc(alias = "FADE_AUTOMATIC")]
    #[doc = "Godot enumerator name: `FADE_AUTOMATIC`"]
    pub const AUTOMATIC: FadeMode = FadeMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FadeMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DISABLED", Self::IN => "IN", Self::OUT => "OUT", Self::CROSS => "CROSS", Self::AUTOMATIC => "AUTOMATIC", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "FADE_DISABLED", Self::IN => "FADE_IN", Self::OUT => "FADE_OUT", Self::CROSS => "FADE_CROSS", Self::AUTOMATIC => "FADE_AUTOMATIC", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FadeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutoAdvanceMode {
    ord: i32
}
impl AutoAdvanceMode {
    #[doc(alias = "AUTO_ADVANCE_DISABLED")]
    #[doc = "Godot enumerator name: `AUTO_ADVANCE_DISABLED`"]
    pub const DISABLED: AutoAdvanceMode = AutoAdvanceMode {
        ord: 0i32
    };
    #[doc(alias = "AUTO_ADVANCE_ENABLED")]
    #[doc = "Godot enumerator name: `AUTO_ADVANCE_ENABLED`"]
    pub const ENABLED: AutoAdvanceMode = AutoAdvanceMode {
        ord: 1i32
    };
    #[doc(alias = "AUTO_ADVANCE_RETURN_TO_HOLD")]
    #[doc = "Godot enumerator name: `AUTO_ADVANCE_RETURN_TO_HOLD`"]
    pub const RETURN_TO_HOLD: AutoAdvanceMode = AutoAdvanceMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AutoAdvanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutoAdvanceMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutoAdvanceMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::RETURN_TO_HOLD => "RETURN_TO_HOLD", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "AUTO_ADVANCE_DISABLED", Self::ENABLED => "AUTO_ADVANCE_ENABLED", Self::RETURN_TO_HOLD => "AUTO_ADVANCE_RETURN_TO_HOLD", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AutoAdvanceMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AutoAdvanceMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutoAdvanceMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
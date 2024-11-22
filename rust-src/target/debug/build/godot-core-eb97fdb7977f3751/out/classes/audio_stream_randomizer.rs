#![doc = "Sidecar module for class [`AudioStreamRandomizer`][crate::classes::AudioStreamRandomizer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamRandomizer` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamRandomizer.`\n\nInherits [`AudioStream`][crate::classes::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_randomizer`][crate::classes::audio_stream_randomizer]: sidecar module with related enum/flag types\n* [`IAudioStreamRandomizer`][crate::classes::IAudioStreamRandomizer]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamRandomizer`](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamRandomizer::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamRandomizer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamRandomizer`][crate::classes::AudioStreamRandomizer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamRandomizer` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamRandomizer: crate::obj::GodotClass < Base = AudioStreamRandomizer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStreamRandomizer {
        pub(crate) fn add_stream_full(&mut self, index: i32, stream: ObjectArg < crate::classes::AudioStream >, weight: f32,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::AudioStream >, f32);
            let args = (index, stream, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "add_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_stream_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_stream(&mut self, index: i32, stream: impl AsObjectArg < crate::classes::AudioStream >,) {
            self.add_stream_ex(index, stream,) . done()
        }
        #[inline]
        pub fn add_stream_ex < 'a > (&'a mut self, index: i32, stream: impl AsObjectArg < crate::classes::AudioStream >,) -> ExAddStream < 'a > {
            ExAddStream::new(self, index, stream,)
        }
        pub fn move_stream(&mut self, index_from: i32, index_to: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index_from, index_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "move_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_stream(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "remove_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream(&mut self, index: i32, stream: impl AsObjectArg < crate::classes::AudioStream >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::AudioStream >);
            let args = (index, stream.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self, index: i32,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallSig = (Option < Gd < crate::classes::AudioStream > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_probability_weight(&mut self, index: i32, weight: f32,) {
            type CallSig = ((), i32, f32);
            let args = (index, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_stream_probability_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_probability_weight(&self, index: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_stream_probability_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_streams_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_streams_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_streams_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_streams_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_random_pitch(&mut self, scale: f32,) {
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_random_pitch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_random_pitch(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_random_pitch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_random_volume_offset_db(&mut self, db_offset: f32,) {
            type CallSig = ((), f32);
            let args = (db_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_random_volume_offset_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_random_volume_offset_db(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_random_volume_offset_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_playback_mode(&mut self, mode: crate::classes::audio_stream_randomizer::PlaybackMode,) {
            type CallSig = ((), crate::classes::audio_stream_randomizer::PlaybackMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_playback_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_mode(&self,) -> crate::classes::audio_stream_randomizer::PlaybackMode {
            type CallSig = (crate::classes::audio_stream_randomizer::PlaybackMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_playback_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioStreamRandomizer {
        type Base = crate::classes::AudioStream;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioStreamRandomizer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamRandomizer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStream > for AudioStreamRandomizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStreamRandomizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamRandomizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamRandomizer {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamRandomizer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamRandomizer {
        type Target = crate::classes::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamRandomizer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamRandomizer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioStreamRandomizer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamRandomizer > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamRandomizer::add_stream_ex`][super::AudioStreamRandomizer::add_stream_ex]."]
#[must_use]
pub struct ExAddStream < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamRandomizer, index: i32, stream: ObjectCow < crate::classes::AudioStream >, weight: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStream < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamRandomizer, index: i32, stream: impl AsObjectArg < crate::classes::AudioStream >,) -> Self {
        let weight = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index, stream: stream.consume_arg(), weight: weight,
        }
    }
    #[inline]
    pub fn weight(self, weight: f32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index, stream, weight,
        }
        = self;
        re_export::AudioStreamRandomizer::add_stream_full(surround_object, index, stream.cow_as_object_arg(), weight,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlaybackMode {
    ord: i32
}
impl PlaybackMode {
    #[doc(alias = "PLAYBACK_RANDOM_NO_REPEATS")]
    #[doc = "Godot enumerator name: `PLAYBACK_RANDOM_NO_REPEATS`"]
    pub const RANDOM_NO_REPEATS: PlaybackMode = PlaybackMode {
        ord: 0i32
    };
    #[doc(alias = "PLAYBACK_RANDOM")]
    #[doc = "Godot enumerator name: `PLAYBACK_RANDOM`"]
    pub const RANDOM: PlaybackMode = PlaybackMode {
        ord: 1i32
    };
    #[doc(alias = "PLAYBACK_SEQUENTIAL")]
    #[doc = "Godot enumerator name: `PLAYBACK_SEQUENTIAL`"]
    pub const SEQUENTIAL: PlaybackMode = PlaybackMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PlaybackMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlaybackMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlaybackMode {
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
            Self::RANDOM_NO_REPEATS => "RANDOM_NO_REPEATS", Self::RANDOM => "RANDOM", Self::SEQUENTIAL => "SEQUENTIAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RANDOM_NO_REPEATS => "PLAYBACK_RANDOM_NO_REPEATS", Self::RANDOM => "PLAYBACK_RANDOM", Self::SEQUENTIAL => "PLAYBACK_SEQUENTIAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PlaybackMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlaybackMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlaybackMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
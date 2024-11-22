#![doc = "Sidecar module for class [`FastNoiseLite`][crate::classes::FastNoiseLite].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FastNoiseLite` enums](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FastNoiseLite.`\n\nInherits [`Noise`][crate::classes::Noise].\n\nRelated symbols:\n\n* [`fast_noise_lite`][crate::classes::fast_noise_lite]: sidecar module with related enum/flag types\n* [`IFastNoiseLite`][crate::classes::IFastNoiseLite]: virtual methods\n\n\nSee also [Godot docs for `FastNoiseLite`](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FastNoiseLite::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FastNoiseLite {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FastNoiseLite`][crate::classes::FastNoiseLite].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FastNoiseLite` methods](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFastNoiseLite: crate::obj::GodotClass < Base = FastNoiseLite > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FastNoiseLite {
        pub fn set_noise_type(&mut self, type_: crate::classes::fast_noise_lite::NoiseType,) {
            type CallSig = ((), crate::classes::fast_noise_lite::NoiseType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_noise_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_type(&self,) -> crate::classes::fast_noise_lite::NoiseType {
            type CallSig = (crate::classes::fast_noise_lite::NoiseType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_noise_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_seed(&mut self, seed: i32,) {
            type CallSig = ((), i32);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_seed(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_seed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frequency(&mut self, freq: f32,) {
            type CallSig = ((), f32);
            let args = (freq,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frequency(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_type(&mut self, type_: crate::classes::fast_noise_lite::FractalType,) {
            type CallSig = ((), crate::classes::fast_noise_lite::FractalType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_type(&self,) -> crate::classes::fast_noise_lite::FractalType {
            type CallSig = (crate::classes::fast_noise_lite::FractalType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_octaves(&mut self, octave_count: i32,) {
            type CallSig = ((), i32);
            let args = (octave_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_octaves(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_lacunarity(&mut self, lacunarity: f32,) {
            type CallSig = ((), f32);
            let args = (lacunarity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_lacunarity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_gain(&mut self, gain: f32,) {
            type CallSig = ((), f32);
            let args = (gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_fractal_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_gain(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_fractal_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_weighted_strength(&mut self, weighted_strength: f32,) {
            type CallSig = ((), f32);
            let args = (weighted_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_fractal_weighted_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_weighted_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_fractal_weighted_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fractal_ping_pong_strength(&mut self, ping_pong_strength: f32,) {
            type CallSig = ((), f32);
            let args = (ping_pong_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_fractal_ping_pong_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fractal_ping_pong_strength(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_fractal_ping_pong_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cellular_distance_function(&mut self, func: crate::classes::fast_noise_lite::CellularDistanceFunction,) {
            type CallSig = ((), crate::classes::fast_noise_lite::CellularDistanceFunction);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_cellular_distance_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cellular_distance_function(&self,) -> crate::classes::fast_noise_lite::CellularDistanceFunction {
            type CallSig = (crate::classes::fast_noise_lite::CellularDistanceFunction,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_cellular_distance_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cellular_jitter(&mut self, jitter: f32,) {
            type CallSig = ((), f32);
            let args = (jitter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_cellular_jitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cellular_jitter(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_cellular_jitter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cellular_return_type(&mut self, ret: crate::classes::fast_noise_lite::CellularReturnType,) {
            type CallSig = ((), crate::classes::fast_noise_lite::CellularReturnType);
            let args = (ret,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_cellular_return_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cellular_return_type(&self,) -> crate::classes::fast_noise_lite::CellularReturnType {
            type CallSig = (crate::classes::fast_noise_lite::CellularReturnType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_cellular_return_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_enabled(&mut self, domain_warp_enabled: bool,) {
            type CallSig = ((), bool);
            let args = (domain_warp_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_domain_warp_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "is_domain_warp_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_type(&mut self, domain_warp_type: crate::classes::fast_noise_lite::DomainWarpType,) {
            type CallSig = ((), crate::classes::fast_noise_lite::DomainWarpType);
            let args = (domain_warp_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_type(&self,) -> crate::classes::fast_noise_lite::DomainWarpType {
            type CallSig = (crate::classes::fast_noise_lite::DomainWarpType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_amplitude(&mut self, domain_warp_amplitude: f32,) {
            type CallSig = ((), f32);
            let args = (domain_warp_amplitude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_amplitude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_amplitude(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_amplitude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_frequency(&mut self, domain_warp_frequency: f32,) {
            type CallSig = ((), f32);
            let args = (domain_warp_frequency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_frequency(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_frequency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_type(&mut self, domain_warp_fractal_type: crate::classes::fast_noise_lite::DomainWarpFractalType,) {
            type CallSig = ((), crate::classes::fast_noise_lite::DomainWarpFractalType);
            let args = (domain_warp_fractal_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_type(&self,) -> crate::classes::fast_noise_lite::DomainWarpFractalType {
            type CallSig = (crate::classes::fast_noise_lite::DomainWarpFractalType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_fractal_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_octaves(&mut self, domain_warp_octave_count: i32,) {
            type CallSig = ((), i32);
            let args = (domain_warp_octave_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_octaves(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_fractal_octaves", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_lacunarity(&mut self, domain_warp_lacunarity: f32,) {
            type CallSig = ((), f32);
            let args = (domain_warp_lacunarity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_lacunarity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_fractal_lacunarity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_domain_warp_fractal_gain(&mut self, domain_warp_gain: f32,) {
            type CallSig = ((), f32);
            let args = (domain_warp_gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "set_domain_warp_fractal_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_domain_warp_fractal_gain(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FastNoiseLite", "get_domain_warp_fractal_gain", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FastNoiseLite {
        type Base = crate::classes::Noise;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"FastNoiseLite"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FastNoiseLite {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Noise > for FastNoiseLite {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FastNoiseLite {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FastNoiseLite {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FastNoiseLite {
        
    }
    impl crate::obj::cap::GodotDefault for FastNoiseLite {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FastNoiseLite {
        type Target = crate::classes::Noise;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FastNoiseLite {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FastNoiseLite`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_FastNoiseLite {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FastNoiseLite > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Noise > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NoiseType {
    ord: i32
}
impl NoiseType {
    #[doc(alias = "TYPE_VALUE")]
    #[doc = "Godot enumerator name: `TYPE_VALUE`"]
    pub const VALUE: NoiseType = NoiseType {
        ord: 5i32
    };
    #[doc(alias = "TYPE_VALUE_CUBIC")]
    #[doc = "Godot enumerator name: `TYPE_VALUE_CUBIC`"]
    pub const VALUE_CUBIC: NoiseType = NoiseType {
        ord: 4i32
    };
    #[doc(alias = "TYPE_PERLIN")]
    #[doc = "Godot enumerator name: `TYPE_PERLIN`"]
    pub const PERLIN: NoiseType = NoiseType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_CELLULAR")]
    #[doc = "Godot enumerator name: `TYPE_CELLULAR`"]
    pub const CELLULAR: NoiseType = NoiseType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_SIMPLEX")]
    #[doc = "Godot enumerator name: `TYPE_SIMPLEX`"]
    pub const SIMPLEX: NoiseType = NoiseType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_SIMPLEX_SMOOTH")]
    #[doc = "Godot enumerator name: `TYPE_SIMPLEX_SMOOTH`"]
    pub const SIMPLEX_SMOOTH: NoiseType = NoiseType {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for NoiseType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("NoiseType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for NoiseType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::VALUE => "VALUE", Self::VALUE_CUBIC => "VALUE_CUBIC", Self::PERLIN => "PERLIN", Self::CELLULAR => "CELLULAR", Self::SIMPLEX => "SIMPLEX", Self::SIMPLEX_SMOOTH => "SIMPLEX_SMOOTH", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VALUE => "TYPE_VALUE", Self::VALUE_CUBIC => "TYPE_VALUE_CUBIC", Self::PERLIN => "TYPE_PERLIN", Self::CELLULAR => "TYPE_CELLULAR", Self::SIMPLEX => "TYPE_SIMPLEX", Self::SIMPLEX_SMOOTH => "TYPE_SIMPLEX_SMOOTH", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for NoiseType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for NoiseType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for NoiseType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FractalType {
    ord: i32
}
impl FractalType {
    #[doc(alias = "FRACTAL_NONE")]
    #[doc = "Godot enumerator name: `FRACTAL_NONE`"]
    pub const NONE: FractalType = FractalType {
        ord: 0i32
    };
    #[doc(alias = "FRACTAL_FBM")]
    #[doc = "Godot enumerator name: `FRACTAL_FBM`"]
    pub const FBM: FractalType = FractalType {
        ord: 1i32
    };
    #[doc(alias = "FRACTAL_RIDGED")]
    #[doc = "Godot enumerator name: `FRACTAL_RIDGED`"]
    pub const RIDGED: FractalType = FractalType {
        ord: 2i32
    };
    #[doc(alias = "FRACTAL_PING_PONG")]
    #[doc = "Godot enumerator name: `FRACTAL_PING_PONG`"]
    pub const PING_PONG: FractalType = FractalType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for FractalType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FractalType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FractalType {
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
            Self::NONE => "NONE", Self::FBM => "FBM", Self::RIDGED => "RIDGED", Self::PING_PONG => "PING_PONG", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "FRACTAL_NONE", Self::FBM => "FRACTAL_FBM", Self::RIDGED => "FRACTAL_RIDGED", Self::PING_PONG => "FRACTAL_PING_PONG", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FractalType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FractalType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FractalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellularDistanceFunction {
    ord: i32
}
impl CellularDistanceFunction {
    #[doc(alias = "DISTANCE_EUCLIDEAN")]
    #[doc = "Godot enumerator name: `DISTANCE_EUCLIDEAN`"]
    pub const EUCLIDEAN: CellularDistanceFunction = CellularDistanceFunction {
        ord: 0i32
    };
    #[doc(alias = "DISTANCE_EUCLIDEAN_SQUARED")]
    #[doc = "Godot enumerator name: `DISTANCE_EUCLIDEAN_SQUARED`"]
    pub const EUCLIDEAN_SQUARED: CellularDistanceFunction = CellularDistanceFunction {
        ord: 1i32
    };
    #[doc(alias = "DISTANCE_MANHATTAN")]
    #[doc = "Godot enumerator name: `DISTANCE_MANHATTAN`"]
    pub const MANHATTAN: CellularDistanceFunction = CellularDistanceFunction {
        ord: 2i32
    };
    #[doc(alias = "DISTANCE_HYBRID")]
    #[doc = "Godot enumerator name: `DISTANCE_HYBRID`"]
    pub const HYBRID: CellularDistanceFunction = CellularDistanceFunction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CellularDistanceFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CellularDistanceFunction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CellularDistanceFunction {
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
            Self::EUCLIDEAN => "EUCLIDEAN", Self::EUCLIDEAN_SQUARED => "EUCLIDEAN_SQUARED", Self::MANHATTAN => "MANHATTAN", Self::HYBRID => "HYBRID", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::EUCLIDEAN => "DISTANCE_EUCLIDEAN", Self::EUCLIDEAN_SQUARED => "DISTANCE_EUCLIDEAN_SQUARED", Self::MANHATTAN => "DISTANCE_MANHATTAN", Self::HYBRID => "DISTANCE_HYBRID", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CellularDistanceFunction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CellularDistanceFunction {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CellularDistanceFunction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellularReturnType {
    ord: i32
}
impl CellularReturnType {
    #[doc(alias = "RETURN_CELL_VALUE")]
    #[doc = "Godot enumerator name: `RETURN_CELL_VALUE`"]
    pub const CELL_VALUE: CellularReturnType = CellularReturnType {
        ord: 0i32
    };
    #[doc(alias = "RETURN_DISTANCE")]
    #[doc = "Godot enumerator name: `RETURN_DISTANCE`"]
    pub const DISTANCE: CellularReturnType = CellularReturnType {
        ord: 1i32
    };
    #[doc(alias = "RETURN_DISTANCE2")]
    #[doc = "Godot enumerator name: `RETURN_DISTANCE2`"]
    pub const DISTANCE2: CellularReturnType = CellularReturnType {
        ord: 2i32
    };
    #[doc(alias = "RETURN_DISTANCE2_ADD")]
    #[doc = "Godot enumerator name: `RETURN_DISTANCE2_ADD`"]
    pub const DISTANCE2_ADD: CellularReturnType = CellularReturnType {
        ord: 3i32
    };
    #[doc(alias = "RETURN_DISTANCE2_SUB")]
    #[doc = "Godot enumerator name: `RETURN_DISTANCE2_SUB`"]
    pub const DISTANCE2_SUB: CellularReturnType = CellularReturnType {
        ord: 4i32
    };
    #[doc(alias = "RETURN_DISTANCE2_MUL")]
    #[doc = "Godot enumerator name: `RETURN_DISTANCE2_MUL`"]
    pub const DISTANCE2_MUL: CellularReturnType = CellularReturnType {
        ord: 5i32
    };
    #[doc(alias = "RETURN_DISTANCE2_DIV")]
    #[doc = "Godot enumerator name: `RETURN_DISTANCE2_DIV`"]
    pub const DISTANCE2_DIV: CellularReturnType = CellularReturnType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for CellularReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CellularReturnType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CellularReturnType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::CELL_VALUE => "CELL_VALUE", Self::DISTANCE => "DISTANCE", Self::DISTANCE2 => "DISTANCE2", Self::DISTANCE2_ADD => "DISTANCE2_ADD", Self::DISTANCE2_SUB => "DISTANCE2_SUB", Self::DISTANCE2_MUL => "DISTANCE2_MUL", Self::DISTANCE2_DIV => "DISTANCE2_DIV", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CELL_VALUE => "RETURN_CELL_VALUE", Self::DISTANCE => "RETURN_DISTANCE", Self::DISTANCE2 => "RETURN_DISTANCE2", Self::DISTANCE2_ADD => "RETURN_DISTANCE2_ADD", Self::DISTANCE2_SUB => "RETURN_DISTANCE2_SUB", Self::DISTANCE2_MUL => "RETURN_DISTANCE2_MUL", Self::DISTANCE2_DIV => "RETURN_DISTANCE2_DIV", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CellularReturnType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CellularReturnType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CellularReturnType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DomainWarpType {
    ord: i32
}
impl DomainWarpType {
    #[doc(alias = "DOMAIN_WARP_SIMPLEX")]
    #[doc = "Godot enumerator name: `DOMAIN_WARP_SIMPLEX`"]
    pub const SIMPLEX: DomainWarpType = DomainWarpType {
        ord: 0i32
    };
    #[doc(alias = "DOMAIN_WARP_SIMPLEX_REDUCED")]
    #[doc = "Godot enumerator name: `DOMAIN_WARP_SIMPLEX_REDUCED`"]
    pub const SIMPLEX_REDUCED: DomainWarpType = DomainWarpType {
        ord: 1i32
    };
    #[doc(alias = "DOMAIN_WARP_BASIC_GRID")]
    #[doc = "Godot enumerator name: `DOMAIN_WARP_BASIC_GRID`"]
    pub const BASIC_GRID: DomainWarpType = DomainWarpType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DomainWarpType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DomainWarpType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DomainWarpType {
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
            Self::SIMPLEX => "SIMPLEX", Self::SIMPLEX_REDUCED => "SIMPLEX_REDUCED", Self::BASIC_GRID => "BASIC_GRID", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SIMPLEX => "DOMAIN_WARP_SIMPLEX", Self::SIMPLEX_REDUCED => "DOMAIN_WARP_SIMPLEX_REDUCED", Self::BASIC_GRID => "DOMAIN_WARP_BASIC_GRID", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DomainWarpType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DomainWarpType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DomainWarpType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DomainWarpFractalType {
    ord: i32
}
impl DomainWarpFractalType {
    #[doc(alias = "DOMAIN_WARP_FRACTAL_NONE")]
    #[doc = "Godot enumerator name: `DOMAIN_WARP_FRACTAL_NONE`"]
    pub const NONE: DomainWarpFractalType = DomainWarpFractalType {
        ord: 0i32
    };
    #[doc(alias = "DOMAIN_WARP_FRACTAL_PROGRESSIVE")]
    #[doc = "Godot enumerator name: `DOMAIN_WARP_FRACTAL_PROGRESSIVE`"]
    pub const PROGRESSIVE: DomainWarpFractalType = DomainWarpFractalType {
        ord: 1i32
    };
    #[doc(alias = "DOMAIN_WARP_FRACTAL_INDEPENDENT")]
    #[doc = "Godot enumerator name: `DOMAIN_WARP_FRACTAL_INDEPENDENT`"]
    pub const INDEPENDENT: DomainWarpFractalType = DomainWarpFractalType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DomainWarpFractalType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DomainWarpFractalType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DomainWarpFractalType {
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
            Self::NONE => "NONE", Self::PROGRESSIVE => "PROGRESSIVE", Self::INDEPENDENT => "INDEPENDENT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "DOMAIN_WARP_FRACTAL_NONE", Self::PROGRESSIVE => "DOMAIN_WARP_FRACTAL_PROGRESSIVE", Self::INDEPENDENT => "DOMAIN_WARP_FRACTAL_INDEPENDENT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DomainWarpFractalType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DomainWarpFractalType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DomainWarpFractalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
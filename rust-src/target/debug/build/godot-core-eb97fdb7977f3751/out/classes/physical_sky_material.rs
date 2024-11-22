#![doc = "Sidecar module for class [`PhysicalSkyMaterial`][crate::classes::PhysicalSkyMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalSkyMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_physicalskymaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicalSkyMaterial.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`IPhysicalSkyMaterial`][crate::classes::IPhysicalSkyMaterial]: virtual methods\n\n\nSee also [Godot docs for `PhysicalSkyMaterial`](https://docs.godotengine.org/en/stable/classes/class_physicalskymaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PhysicalSkyMaterial::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicalSkyMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicalSkyMaterial`][crate::classes::PhysicalSkyMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicalSkyMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_physicalskymaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicalSkyMaterial: crate::obj::GodotClass < Base = PhysicalSkyMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_shader_rid(&self,) -> Rid;
        fn get_shader_mode(&self,) -> crate::classes::shader::Mode;
        fn can_do_next_pass(&self,) -> bool {
            unimplemented !()
        }
        fn can_use_render_priority(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl PhysicalSkyMaterial {
        pub fn set_rayleigh_coefficient(&mut self, rayleigh: f32,) {
            type CallSig = ((), f32);
            let args = (rayleigh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_rayleigh_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rayleigh_coefficient(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_rayleigh_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rayleigh_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_rayleigh_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rayleigh_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_rayleigh_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mie_coefficient(&mut self, mie: f32,) {
            type CallSig = ((), f32);
            let args = (mie,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_mie_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mie_coefficient(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_mie_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mie_eccentricity(&mut self, eccentricity: f32,) {
            type CallSig = ((), f32);
            let args = (eccentricity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_mie_eccentricity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mie_eccentricity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_mie_eccentricity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mie_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_mie_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mie_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_mie_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_turbidity(&mut self, turbidity: f32,) {
            type CallSig = ((), f32);
            let args = (turbidity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_turbidity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_turbidity(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_turbidity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sun_disk_scale(&mut self, scale: f32,) {
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_sun_disk_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sun_disk_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_sun_disk_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ground_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_ground_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ground_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_ground_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_energy_multiplier(&mut self, multiplier: f32,) {
            type CallSig = ((), f32);
            let args = (multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_energy_multiplier(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_energy_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_debanding(&mut self, use_debanding: bool,) {
            type CallSig = ((), bool);
            let args = (use_debanding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_use_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_debanding(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_use_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_night_sky(&mut self, night_sky: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (night_sky.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "set_night_sky", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_night_sky(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalSkyMaterial", "get_night_sky", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicalSkyMaterial {
        type Base = crate::classes::Material;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicalSkyMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicalSkyMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for PhysicalSkyMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for PhysicalSkyMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PhysicalSkyMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicalSkyMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicalSkyMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicalSkyMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicalSkyMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicalSkyMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicalSkyMaterial {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicalSkyMaterial > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Material > for $Class {
                
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
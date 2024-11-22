#![doc = "Sidecar module for class [`FogMaterial`][crate::classes::FogMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FogMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FogMaterial.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`IFogMaterial`][crate::classes::IFogMaterial]: virtual methods\n\n\nSee also [Godot docs for `FogMaterial`](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FogMaterial::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FogMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FogMaterial`][crate::classes::FogMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FogMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFogMaterial: crate::obj::GodotClass < Base = FogMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FogMaterial {
        pub fn set_density(&mut self, density: f32,) {
            type CallSig = ((), f32);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "set_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_density(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "get_density", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_albedo(&mut self, albedo: Color,) {
            type CallSig = ((), Color);
            let args = (albedo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "set_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_albedo(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "get_albedo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission(&mut self, emission: Color,) {
            type CallSig = ((), Color);
            let args = (emission,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "set_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "get_emission", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height_falloff(&mut self, height_falloff: f32,) {
            type CallSig = ((), f32);
            let args = (height_falloff,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "set_height_falloff", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height_falloff(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "get_height_falloff", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edge_fade(&mut self, edge_fade: f32,) {
            type CallSig = ((), f32);
            let args = (edge_fade,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "set_edge_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_fade(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "get_edge_fade", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_density_texture(&mut self, density_texture: impl AsObjectArg < crate::classes::Texture3D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture3D >);
            let args = (density_texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "set_density_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_density_texture(&self,) -> Option < Gd < crate::classes::Texture3D > > {
            type CallSig = (Option < Gd < crate::classes::Texture3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "FogMaterial", "get_density_texture", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for FogMaterial {
        type Base = crate::classes::Material;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"FogMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FogMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for FogMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FogMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FogMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FogMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for FogMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FogMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FogMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FogMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_FogMaterial {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FogMaterial > for $Class {
                
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
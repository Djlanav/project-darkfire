#![doc = "Sidecar module for class [`SphereMesh`][crate::classes::SphereMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SphereMesh` enums](https://docs.godotengine.org/en/stable/classes/class_spheremesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SphereMesh.`\n\nInherits [`PrimitiveMesh`][crate::classes::PrimitiveMesh].\n\nRelated symbols:\n\n* [`ISphereMesh`][crate::classes::ISphereMesh]: virtual methods\n\n\nSee also [Godot docs for `SphereMesh`](https://docs.godotengine.org/en/stable/classes/class_spheremesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SphereMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SphereMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SphereMesh`][crate::classes::SphereMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SphereMesh` methods](https://docs.godotengine.org/en/stable/classes/class_spheremesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISphereMesh: crate::obj::GodotClass < Base = SphereMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn create_mesh_array(&self,) -> VariantArray {
            unimplemented !()
        }
        fn get_surface_count(&self,) -> i32;
        fn surface_get_array_len(&self, index: i32,) -> i32;
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        fn surface_get_arrays(&self, index: i32,) -> VariantArray;
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VariantArray >;
        fn surface_get_lods(&self, index: i32,) -> Dictionary;
        fn surface_get_format(&self, index: i32,) -> u32;
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        fn get_blend_shape_count(&self,) -> i32;
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        fn get_aabb(&self,) -> Aabb;
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl SphereMesh {
        pub fn set_radius(&mut self, radius: f32,) {
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "set_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "get_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: f32,) {
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_radial_segments(&mut self, radial_segments: i32,) {
            type CallSig = ((), i32);
            let args = (radial_segments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "set_radial_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radial_segments(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "get_radial_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rings(&mut self, rings: i32,) {
            type CallSig = ((), i32);
            let args = (rings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "set_rings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rings(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "get_rings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_is_hemisphere(&mut self, is_hemisphere: bool,) {
            type CallSig = ((), bool);
            let args = (is_hemisphere,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "set_is_hemisphere", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_is_hemisphere(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SphereMesh", "get_is_hemisphere", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SphereMesh {
        type Base = crate::classes::PrimitiveMesh;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SphereMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SphereMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PrimitiveMesh > for SphereMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for SphereMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SphereMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SphereMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SphereMesh {
        
    }
    impl crate::obj::cap::GodotDefault for SphereMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SphereMesh {
        type Target = crate::classes::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SphereMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SphereMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SphereMesh {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SphereMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PrimitiveMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
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
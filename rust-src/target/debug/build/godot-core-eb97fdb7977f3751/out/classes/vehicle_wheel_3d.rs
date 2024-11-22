#![doc = "Sidecar module for class [`VehicleWheel3D`][crate::classes::VehicleWheel3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VehicleWheel3D` enums](https://docs.godotengine.org/en/stable/classes/class_vehiclewheel3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VehicleWheel3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`IVehicleWheel3D`][crate::classes::IVehicleWheel3D]: virtual methods\n\n\nSee also [Godot docs for `VehicleWheel3D`](https://docs.godotengine.org/en/stable/classes/class_vehiclewheel3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`VehicleWheel3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VehicleWheel3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VehicleWheel3D`][crate::classes::VehicleWheel3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VehicleWheel3D` methods](https://docs.godotengine.org/en/stable/classes/class_vehiclewheel3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVehicleWheel3D: crate::obj::GodotClass < Base = VehicleWheel3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    impl VehicleWheel3D {
        pub fn set_radius(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_suspension_rest_length(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_suspension_rest_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_suspension_rest_length(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_suspension_rest_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_suspension_travel(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_suspension_travel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_suspension_travel(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_suspension_travel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_suspension_stiffness(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_suspension_stiffness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_suspension_stiffness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_suspension_stiffness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_suspension_max_force(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9752usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_suspension_max_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_suspension_max_force(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9753usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_suspension_max_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_damping_compression(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9754usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_damping_compression", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_damping_compression(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9755usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_damping_compression", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_damping_relaxation(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9756usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_damping_relaxation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_damping_relaxation(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_damping_relaxation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_as_traction(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_use_as_traction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_used_as_traction(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "is_used_as_traction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_as_steering(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_use_as_steering", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_used_as_steering(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "is_used_as_steering", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_friction_slip(&mut self, length: f32,) {
            type CallSig = ((), f32);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_friction_slip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_friction_slip(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_friction_slip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_contact(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "is_in_contact", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_body(&self,) -> Option < Gd < crate::classes::Node3D > > {
            type CallSig = (Option < Gd < crate::classes::Node3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_contact_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_roll_influence(&mut self, roll_influence: f32,) {
            type CallSig = ((), f32);
            let args = (roll_influence,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_roll_influence", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_roll_influence(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_roll_influence", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skidinfo(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_skidinfo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rpm(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_rpm", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_engine_force(&mut self, engine_force: f32,) {
            type CallSig = ((), f32);
            let args = (engine_force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_engine_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_engine_force(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_engine_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_brake(&mut self, brake: f32,) {
            type CallSig = ((), f32);
            let args = (brake,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_brake", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_brake(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_brake", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_steering(&mut self, steering: f32,) {
            type CallSig = ((), f32);
            let args = (steering,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "set_steering", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_steering(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VehicleWheel3D", "get_steering", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VehicleWheel3D {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VehicleWheel3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VehicleWheel3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for VehicleWheel3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for VehicleWheel3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VehicleWheel3D {
        
    }
    impl crate::obj::cap::GodotDefault for VehicleWheel3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VehicleWheel3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VehicleWheel3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VehicleWheel3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VehicleWheel3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VehicleWheel3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#![doc = "Sidecar module for class [`PhysicsDirectBodyState2DExtension`][crate::classes::PhysicsDirectBodyState2DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState2DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2dextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState2DExtension.`\n\nInherits [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D].\n\nRelated symbols:\n\n* [`IPhysicsDirectBodyState2DExtension`][crate::classes::IPhysicsDirectBodyState2DExtension]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectBodyState2DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2dextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicsDirectBodyState2DExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState2DExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectBodyState2DExtension`][crate::classes::PhysicsDirectBodyState2DExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectBodyState2DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2dextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectBodyState2DExtension: crate::obj::GodotClass < Base = PhysicsDirectBodyState2DExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_total_gravity(&self,) -> Vector2;
        fn get_total_linear_damp(&self,) -> f32;
        fn get_total_angular_damp(&self,) -> f32;
        fn get_center_of_mass(&self,) -> Vector2;
        fn get_center_of_mass_local(&self,) -> Vector2;
        fn get_inverse_mass(&self,) -> f32;
        fn get_inverse_inertia(&self,) -> f32;
        fn set_linear_velocity(&mut self, velocity: Vector2,);
        fn get_linear_velocity(&self,) -> Vector2;
        fn set_angular_velocity(&mut self, velocity: f32,);
        fn get_angular_velocity(&self,) -> f32;
        fn set_transform(&mut self, transform: Transform2D,);
        fn get_transform(&self,) -> Transform2D;
        fn get_velocity_at_local_position(&self, local_position: Vector2,) -> Vector2;
        fn apply_central_impulse(&mut self, impulse: Vector2,);
        fn apply_impulse(&mut self, impulse: Vector2, position: Vector2,);
        fn apply_torque_impulse(&mut self, impulse: f32,);
        fn apply_central_force(&mut self, force: Vector2,);
        fn apply_force(&mut self, force: Vector2, position: Vector2,);
        fn apply_torque(&mut self, torque: f32,);
        fn add_constant_central_force(&mut self, force: Vector2,);
        fn add_constant_force(&mut self, force: Vector2, position: Vector2,);
        fn add_constant_torque(&mut self, torque: f32,);
        fn set_constant_force(&mut self, force: Vector2,);
        fn get_constant_force(&self,) -> Vector2;
        fn set_constant_torque(&mut self, torque: f32,);
        fn get_constant_torque(&self,) -> f32;
        fn set_sleep_state(&mut self, enabled: bool,);
        fn is_sleeping(&self,) -> bool;
        fn get_contact_count(&self,) -> i32;
        fn get_contact_local_position(&self, contact_idx: i32,) -> Vector2;
        fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector2;
        fn get_contact_local_shape(&self, contact_idx: i32,) -> i32;
        fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector2;
        fn get_contact_collider(&self, contact_idx: i32,) -> Rid;
        fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector2;
        fn get_contact_collider_id(&self, contact_idx: i32,) -> u64;
        fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::classes::Object > >;
        fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32;
        fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector2;
        fn get_contact_impulse(&self, contact_idx: i32,) -> Vector2;
        fn get_step(&self,) -> f32;
        fn integrate_forces(&mut self,);
        fn get_space_state(&mut self,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState2D > >;
        
    }
    impl PhysicsDirectBodyState2DExtension {
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
    impl crate::obj::GodotClass for PhysicsDirectBodyState2DExtension {
        type Base = crate::classes::PhysicsDirectBodyState2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsDirectBodyState2DExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState2DExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsDirectBodyState2D > for PhysicsDirectBodyState2DExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectBodyState2DExtension {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsDirectBodyState2DExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsDirectBodyState2DExtension {
        type Target = crate::classes::PhysicsDirectBodyState2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState2DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsDirectBodyState2DExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsDirectBodyState2DExtension {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsDirectBodyState2DExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsDirectBodyState2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
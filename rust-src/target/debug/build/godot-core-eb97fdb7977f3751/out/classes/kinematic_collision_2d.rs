#![doc = "Sidecar module for class [`KinematicCollision2D`][crate::classes::KinematicCollision2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `KinematicCollision2D` enums](https://docs.godotengine.org/en/stable/classes/class_kinematiccollision2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `KinematicCollision2D.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`kinematic_collision_2d`][crate::classes::kinematic_collision_2d]: sidecar module with related enum/flag types\n* [`IKinematicCollision2D`][crate::classes::IKinematicCollision2D]: virtual methods\n\n\nSee also [Godot docs for `KinematicCollision2D`](https://docs.godotengine.org/en/stable/classes/class_kinematiccollision2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`KinematicCollision2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct KinematicCollision2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`KinematicCollision2D`][crate::classes::KinematicCollision2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `KinematicCollision2D` methods](https://docs.godotengine.org/en/stable/classes/class_kinematiccollision2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IKinematicCollision2D: crate::obj::GodotClass < Base = KinematicCollision2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl KinematicCollision2D {
        pub fn get_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normal(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_travel(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_travel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remainder(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_remainder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_angle_full(&self, up_direction: Vector2,) -> f32 {
            type CallSig = (f32, Vector2);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_angle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_angle(&self,) -> f32 {
            self.get_angle_ex() . done()
        }
        #[inline]
        pub fn get_angle_ex < 'a > (&'a self,) -> ExGetAngle < 'a > {
            ExGetAngle::new(self,)
        }
        pub fn get_depth(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_shape(&self,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_local_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider(&self,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_id(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_collider_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_rid(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_collider_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_shape(&self,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_shape_index(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_collider_shape_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_velocity(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "KinematicCollision2D", "get_collider_velocity", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for KinematicCollision2D {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"KinematicCollision2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for KinematicCollision2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for KinematicCollision2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for KinematicCollision2D {
        
    }
    impl crate::obj::cap::GodotDefault for KinematicCollision2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for KinematicCollision2D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for KinematicCollision2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`KinematicCollision2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_KinematicCollision2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::KinematicCollision2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`KinematicCollision2D::get_angle_ex`][super::KinematicCollision2D::get_angle_ex]."]
#[must_use]
pub struct ExGetAngle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::KinematicCollision2D, up_direction: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetAngle < 'a > {
    fn new(surround_object: &'a re_export::KinematicCollision2D,) -> Self {
        let up_direction = Vector2::new(0 as _, - 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, up_direction: up_direction,
        }
    }
    #[inline]
    pub fn up_direction(self, up_direction: Vector2) -> Self {
        Self {
            up_direction: up_direction, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, up_direction,
        }
        = self;
        re_export::KinematicCollision2D::get_angle_full(surround_object, up_direction,)
    }
}
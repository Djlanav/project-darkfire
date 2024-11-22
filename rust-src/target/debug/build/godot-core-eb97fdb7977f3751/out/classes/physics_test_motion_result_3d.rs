#![doc = "Sidecar module for class [`PhysicsTestMotionResult3D`][crate::classes::PhysicsTestMotionResult3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsTestMotionResult3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionresult3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsTestMotionResult3D.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`physics_test_motion_result_3d`][crate::classes::physics_test_motion_result_3d]: sidecar module with related enum/flag types\n* [`IPhysicsTestMotionResult3D`][crate::classes::IPhysicsTestMotionResult3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsTestMotionResult3D`](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionresult3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PhysicsTestMotionResult3D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsTestMotionResult3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsTestMotionResult3D`][crate::classes::PhysicsTestMotionResult3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsTestMotionResult3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionresult3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsTestMotionResult3D: crate::obj::GodotClass < Base = PhysicsTestMotionResult3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsTestMotionResult3D {
        pub fn get_travel(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_travel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remainder(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_remainder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_safe_fraction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_safe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_unsafe_fraction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_unsafe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_collision_point_full(&self, collision_index: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collision_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collision_point(&self,) -> Vector3 {
            self.get_collision_point_ex() . done()
        }
        #[inline]
        pub fn get_collision_point_ex < 'a > (&'a self,) -> ExGetCollisionPoint < 'a > {
            ExGetCollisionPoint::new(self,)
        }
        pub(crate) fn get_collision_normal_full(&self, collision_index: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collision_normal_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collision_normal(&self,) -> Vector3 {
            self.get_collision_normal_ex() . done()
        }
        #[inline]
        pub fn get_collision_normal_ex < 'a > (&'a self,) -> ExGetCollisionNormal < 'a > {
            ExGetCollisionNormal::new(self,)
        }
        pub(crate) fn get_collider_velocity_full(&self, collision_index: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collider_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collider_velocity_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collider_velocity(&self,) -> Vector3 {
            self.get_collider_velocity_ex() . done()
        }
        #[inline]
        pub fn get_collider_velocity_ex < 'a > (&'a self,) -> ExGetColliderVelocity < 'a > {
            ExGetColliderVelocity::new(self,)
        }
        pub(crate) fn get_collider_id_full(&self, collision_index: i32,) -> u64 {
            type CallSig = (u64, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collider_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collider_id_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collider_id(&self,) -> u64 {
            self.get_collider_id_ex() . done()
        }
        #[inline]
        pub fn get_collider_id_ex < 'a > (&'a self,) -> ExGetColliderId < 'a > {
            ExGetColliderId::new(self,)
        }
        pub(crate) fn get_collider_rid_full(&self, collision_index: i32,) -> Rid {
            type CallSig = (Rid, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collider_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collider_rid_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collider_rid(&self,) -> Rid {
            self.get_collider_rid_ex() . done()
        }
        #[inline]
        pub fn get_collider_rid_ex < 'a > (&'a self,) -> ExGetColliderRid < 'a > {
            ExGetColliderRid::new(self,)
        }
        pub(crate) fn get_collider_full(&self, collision_index: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collider_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collider(&self,) -> Option < Gd < crate::classes::Object > > {
            self.get_collider_ex() . done()
        }
        #[inline]
        pub fn get_collider_ex < 'a > (&'a self,) -> ExGetCollider < 'a > {
            ExGetCollider::new(self,)
        }
        pub(crate) fn get_collider_shape_full(&self, collision_index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collider_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collider_shape(&self,) -> i32 {
            self.get_collider_shape_ex() . done()
        }
        #[inline]
        pub fn get_collider_shape_ex < 'a > (&'a self,) -> ExGetColliderShape < 'a > {
            ExGetColliderShape::new(self,)
        }
        pub(crate) fn get_collision_local_shape_full(&self, collision_index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_local_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collision_local_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collision_local_shape(&self,) -> i32 {
            self.get_collision_local_shape_ex() . done()
        }
        #[inline]
        pub fn get_collision_local_shape_ex < 'a > (&'a self,) -> ExGetCollisionLocalShape < 'a > {
            ExGetCollisionLocalShape::new(self,)
        }
        pub(crate) fn get_collision_depth_full(&self, collision_index: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicsTestMotionResult3D", "get_collision_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_collision_depth_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_collision_depth(&self,) -> f32 {
            self.get_collision_depth_ex() . done()
        }
        #[inline]
        pub fn get_collision_depth_ex < 'a > (&'a self,) -> ExGetCollisionDepth < 'a > {
            ExGetCollisionDepth::new(self,)
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
    impl crate::obj::GodotClass for PhysicsTestMotionResult3D {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicsTestMotionResult3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsTestMotionResult3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PhysicsTestMotionResult3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsTestMotionResult3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsTestMotionResult3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsTestMotionResult3D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsTestMotionResult3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsTestMotionResult3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicsTestMotionResult3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsTestMotionResult3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_point_ex`][super::PhysicsTestMotionResult3D::get_collision_point_ex]."]
#[must_use]
pub struct ExGetCollisionPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionPoint < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collision_point_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_normal_ex`][super::PhysicsTestMotionResult3D::get_collision_normal_ex]."]
#[must_use]
pub struct ExGetCollisionNormal < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionNormal < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collision_normal_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_velocity_ex`][super::PhysicsTestMotionResult3D::get_collider_velocity_ex]."]
#[must_use]
pub struct ExGetColliderVelocity < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderVelocity < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collider_velocity_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_id_ex`][super::PhysicsTestMotionResult3D::get_collider_id_ex]."]
#[must_use]
pub struct ExGetColliderId < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderId < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collider_id_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_rid_ex`][super::PhysicsTestMotionResult3D::get_collider_rid_ex]."]
#[must_use]
pub struct ExGetColliderRid < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderRid < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collider_rid_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_ex`][super::PhysicsTestMotionResult3D::get_collider_ex]."]
#[must_use]
pub struct ExGetCollider < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollider < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Object > > {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collider_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_shape_ex`][super::PhysicsTestMotionResult3D::get_collider_shape_ex]."]
#[must_use]
pub struct ExGetColliderShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderShape < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collider_shape_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_local_shape_ex`][super::PhysicsTestMotionResult3D::get_collision_local_shape_ex]."]
#[must_use]
pub struct ExGetCollisionLocalShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionLocalShape < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collision_local_shape_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_depth_ex`][super::PhysicsTestMotionResult3D::get_collision_depth_ex]."]
#[must_use]
pub struct ExGetCollisionDepth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionDepth < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::PhysicsTestMotionResult3D::get_collision_depth_full(surround_object, collision_index,)
    }
}
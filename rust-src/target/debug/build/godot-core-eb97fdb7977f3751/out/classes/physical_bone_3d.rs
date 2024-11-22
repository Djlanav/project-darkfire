#![doc = "Sidecar module for class [`PhysicalBone3D`][crate::classes::PhysicalBone3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalBone3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicalBone3D.`\n\nInherits [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nRelated symbols:\n\n* [`physical_bone_3d`][crate::classes::physical_bone_3d]: sidecar module with related enum/flag types\n* [`IPhysicalBone3D`][crate::classes::IPhysicalBone3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicalBone3D`](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicalBone3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicalBone3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicalBone3D`][crate::classes::PhysicalBone3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicalBone3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicalBone3D: crate::obj::GodotClass < Base = PhysicalBone3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn integrate_forces(&mut self, state: Option < Gd < crate::classes::PhysicsDirectBodyState3D > >,) {
            unimplemented !()
        }
        fn input_event(&mut self, camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >, event_position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
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
    impl PhysicalBone3D {
        pub fn apply_central_impulse(&mut self, impulse: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector3, position: Vector3,) {
            type CallSig = ((), Vector3, Vector3);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector3,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex < 'a > (&'a mut self, impulse: Vector3,) -> ExApplyImpulse < 'a > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn set_joint_type(&mut self, joint_type: crate::classes::physical_bone_3d::JointType,) {
            type CallSig = ((), crate::classes::physical_bone_3d::JointType);
            let args = (joint_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joint_type(&self,) -> crate::classes::physical_bone_3d::JointType {
            type CallSig = (crate::classes::physical_bone_3d::JointType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_joint_offset(&mut self, offset: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joint_offset(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_joint_rotation(&mut self, euler: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (euler,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_joint_rotation(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_body_offset(&mut self, offset: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_body_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_body_offset(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_body_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_simulate_physics(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_simulate_physics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_simulating_physics(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_simulating_physics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_id(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_bone_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mass(&mut self, mass: f32,) {
            type CallSig = ((), f32);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_friction(&mut self, friction: f32,) {
            type CallSig = ((), f32);
            let args = (friction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_friction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_friction(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_friction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bounce(&mut self, bounce: f32,) {
            type CallSig = ((), f32);
            let args = (bounce,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_bounce", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bounce(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_bounce", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type CallSig = ((), f32);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_gravity_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_gravity_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::classes::physical_bone_3d::DampMode,) {
            type CallSig = ((), crate::classes::physical_bone_3d::DampMode);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::classes::physical_bone_3d::DampMode {
            type CallSig = (crate::classes::physical_bone_3d::DampMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::classes::physical_bone_3d::DampMode,) {
            type CallSig = ((), crate::classes::physical_bone_3d::DampMode);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::classes::physical_bone_3d::DampMode {
            type CallSig = (crate::classes::physical_bone_3d::DampMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallSig = ((), f32);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallSig = ((), f32);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_use_custom_integrator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_custom_integrator(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_using_custom_integrator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type CallSig = ((), bool);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_can_sleep", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_able_to_sleep", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicalBone3D {
        type Base = crate::classes::PhysicsBody3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PhysicalBone3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicalBone3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicalBone3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicalBone3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicalBone3D {
        type Target = crate::classes::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicalBone3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicalBone3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PhysicalBone3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicalBone3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject3D > for $Class {
                
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
#[doc = "Default-param extender for [`PhysicalBone3D::apply_impulse_ex`][super::PhysicalBone3D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicalBone3D, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicalBone3D, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse, position,
        }
        = self;
        re_export::PhysicalBone3D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DampMode {
    ord: i32
}
impl DampMode {
    #[doc(alias = "DAMP_MODE_COMBINE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_COMBINE`"]
    pub const COMBINE: DampMode = DampMode {
        ord: 0i32
    };
    #[doc(alias = "DAMP_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_REPLACE`"]
    pub const REPLACE: DampMode = DampMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DampMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DampMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DampMode {
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
            Self::COMBINE => "COMBINE", Self::REPLACE => "REPLACE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::COMBINE => "DAMP_MODE_COMBINE", Self::REPLACE => "DAMP_MODE_REPLACE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DampMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DampMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JointType {
    ord: i32
}
impl JointType {
    #[doc(alias = "JOINT_TYPE_NONE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_NONE`"]
    pub const NONE: JointType = JointType {
        ord: 0i32
    };
    #[doc(alias = "JOINT_TYPE_PIN")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_PIN`"]
    pub const PIN: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_CONE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_CONE`"]
    pub const CONE: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_HINGE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_HINGE`"]
    pub const HINGE: JointType = JointType {
        ord: 3i32
    };
    #[doc(alias = "JOINT_TYPE_SLIDER")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_SLIDER`"]
    pub const SLIDER: JointType = JointType {
        ord: 4i32
    };
    #[doc(alias = "JOINT_TYPE_6DOF")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_6DOF`"]
    pub const TYPE_6DOF: JointType = JointType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for JointType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointType {
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
            Self::NONE => "NONE", Self::PIN => "PIN", Self::CONE => "CONE", Self::HINGE => "HINGE", Self::SLIDER => "SLIDER", Self::TYPE_6DOF => "TYPE_6DOF", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "JOINT_TYPE_NONE", Self::PIN => "JOINT_TYPE_PIN", Self::CONE => "JOINT_TYPE_CONE", Self::HINGE => "JOINT_TYPE_HINGE", Self::SLIDER => "JOINT_TYPE_SLIDER", Self::TYPE_6DOF => "JOINT_TYPE_6DOF", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for JointType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for JointType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
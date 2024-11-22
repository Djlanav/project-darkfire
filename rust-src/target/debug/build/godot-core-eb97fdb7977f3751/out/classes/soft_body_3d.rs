#![doc = "Sidecar module for class [`SoftBody3D`][crate::classes::SoftBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SoftBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_softbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SoftBody3D.`\n\nInherits [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nRelated symbols:\n\n* [`soft_body_3d`][crate::classes::soft_body_3d]: sidecar module with related enum/flag types\n* [`ISoftBody3D`][crate::classes::ISoftBody3D]: virtual methods\n\n\nSee also [Godot docs for `SoftBody3D`](https://docs.godotengine.org/en/stable/classes/class_softbody3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SoftBody3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SoftBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SoftBody3D`][crate::classes::SoftBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SoftBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_softbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISoftBody3D: crate::obj::GodotClass < Base = SoftBody3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl SoftBody3D {
        pub fn get_physics_rid(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_physics_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, collision_mask: u32,) {
            type CallSig = ((), u32);
            let args = (collision_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer(&mut self, collision_layer: u32,) {
            type CallSig = ((), u32);
            let args = (collision_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_parent_collision_ignore(&mut self, parent_collision_ignore: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (parent_collision_ignore.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_parent_collision_ignore", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_collision_ignore(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_parent_collision_ignore", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_mode(&mut self, mode: crate::classes::soft_body_3d::DisableMode,) {
            type CallSig = ((), crate::classes::soft_body_3d::DisableMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_disable_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_disable_mode(&self,) -> crate::classes::soft_body_3d::DisableMode {
            type CallSig = (crate::classes::soft_body_3d::DisableMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_disable_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::classes::PhysicsBody3D > > {
            type CallSig = (Array < Gd < crate::classes::PhysicsBody3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "add_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (body.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "remove_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_simulation_precision(&mut self, simulation_precision: i32,) {
            type CallSig = ((), i32);
            let args = (simulation_precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_simulation_precision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_simulation_precision(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_simulation_precision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_total_mass(&mut self, mass: f32,) {
            type CallSig = ((), f32);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_total_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_mass(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_total_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_stiffness(&mut self, linear_stiffness: f32,) {
            type CallSig = ((), f32);
            let args = (linear_stiffness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_linear_stiffness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_stiffness(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_linear_stiffness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pressure_coefficient(&mut self, pressure_coefficient: f32,) {
            type CallSig = ((), f32);
            let args = (pressure_coefficient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_pressure_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pressure_coefficient(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_pressure_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_damping_coefficient(&mut self, damping_coefficient: f32,) {
            type CallSig = ((), f32);
            let args = (damping_coefficient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_damping_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_damping_coefficient(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_damping_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_coefficient(&mut self, drag_coefficient: f32,) {
            type CallSig = ((), f32);
            let args = (drag_coefficient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_drag_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_coefficient(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_drag_coefficient", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_transform(&mut self, point_index: i32,) -> Vector3 {
            type CallSig = (Vector3, i32);
            let args = (point_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_point_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_point_pinned_full(&mut self, point_index: i32, pinned: bool, attachment_path: CowArg < NodePath >,) {
            type CallSig < 'a0, > = ((), i32, bool, CowArg < 'a0, NodePath >);
            let args = (point_index, pinned, attachment_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_point_pinned", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_point_pinned_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_point_pinned(&mut self, point_index: i32, pinned: bool,) {
            self.set_point_pinned_ex(point_index, pinned,) . done()
        }
        #[inline]
        pub fn set_point_pinned_ex < 'a > (&'a mut self, point_index: i32, pinned: bool,) -> ExSetPointPinned < 'a > {
            ExSetPointPinned::new(self, point_index, pinned,)
        }
        pub fn is_point_pinned(&self, point_index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (point_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "is_point_pinned", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ray_pickable(&mut self, ray_pickable: bool,) {
            type CallSig = ((), bool);
            let args = (ray_pickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_ray_pickable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ray_pickable(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SoftBody3D", "is_ray_pickable", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SoftBody3D {
        type Base = crate::classes::MeshInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SoftBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SoftBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MeshInstance3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SoftBody3D {
        
    }
    impl crate::obj::cap::GodotDefault for SoftBody3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SoftBody3D {
        type Target = crate::classes::MeshInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SoftBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SoftBody3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SoftBody3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SoftBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
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
#[doc = "Default-param extender for [`SoftBody3D::set_point_pinned_ex`][super::SoftBody3D::set_point_pinned_ex]."]
#[must_use]
pub struct ExSetPointPinned < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SoftBody3D, point_index: i32, pinned: bool, attachment_path: CowArg < 'a, NodePath >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPointPinned < 'a > {
    fn new(surround_object: &'a mut re_export::SoftBody3D, point_index: i32, pinned: bool,) -> Self {
        let attachment_path = NodePath::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, point_index: point_index, pinned: pinned, attachment_path: CowArg::Owned(attachment_path),
        }
    }
    #[inline]
    pub fn attachment_path(self, attachment_path: impl AsArg < NodePath > + 'a) -> Self {
        Self {
            attachment_path: attachment_path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, point_index, pinned, attachment_path,
        }
        = self;
        re_export::SoftBody3D::set_point_pinned_full(surround_object, point_index, pinned, attachment_path,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisableMode {
    ord: i32
}
impl DisableMode {
    #[doc(alias = "DISABLE_MODE_REMOVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_REMOVE`"]
    pub const REMOVE: DisableMode = DisableMode {
        ord: 0i32
    };
    #[doc(alias = "DISABLE_MODE_KEEP_ACTIVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_KEEP_ACTIVE`"]
    pub const KEEP_ACTIVE: DisableMode = DisableMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DisableMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisableMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisableMode {
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
            Self::REMOVE => "REMOVE", Self::KEEP_ACTIVE => "KEEP_ACTIVE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::REMOVE => "DISABLE_MODE_REMOVE", Self::KEEP_ACTIVE => "DISABLE_MODE_KEEP_ACTIVE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DisableMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DisableMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisableMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
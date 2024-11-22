#![doc = "Sidecar module for class [`CharacterBody3D`][crate::classes::CharacterBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CharacterBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CharacterBody3D.`\n\nInherits [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nRelated symbols:\n\n* [`character_body_3d`][crate::classes::character_body_3d]: sidecar module with related enum/flag types\n* [`ICharacterBody3D`][crate::classes::ICharacterBody3D]: virtual methods\n\n\nSee also [Godot docs for `CharacterBody3D`](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CharacterBody3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CharacterBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CharacterBody3D`][crate::classes::CharacterBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CharacterBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICharacterBody3D: crate::obj::GodotClass < Base = CharacterBody3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CharacterBody3D {
        pub fn move_and_slide(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "move_and_slide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_floor_snap(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "apply_floor_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_velocity(&mut self, velocity: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_safe_margin(&mut self, margin: f32,) {
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_safe_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_safe_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_safe_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_floor_stop_on_slope_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_floor_stop_on_slope_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_stop_on_slope_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_stop_on_slope_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_constant_speed_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_constant_speed_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_floor_constant_speed_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_floor_constant_speed_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_block_on_wall_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_block_on_wall_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_floor_block_on_wall_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_floor_block_on_wall_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_slide_on_ceiling_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_slide_on_ceiling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_slide_on_ceiling_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_slide_on_ceiling_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_platform_floor_layers(&mut self, exclude_layer: u32,) {
            type CallSig = ((), u32);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_platform_floor_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_floor_layers(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_floor_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_platform_wall_layers(&mut self, exclude_layer: u32,) {
            type CallSig = ((), u32);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_platform_wall_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_wall_layers(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_wall_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_slides(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_max_slides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_slides(&mut self, max_slides: i32,) {
            type CallSig = ((), i32);
            let args = (max_slides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_max_slides", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_floor_max_angle(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_max_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_max_angle(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_max_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_floor_snap_length(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_snap_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_floor_snap_length(&mut self, floor_snap_length: f32,) {
            type CallSig = ((), f32);
            let args = (floor_snap_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_snap_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wall_min_slide_angle(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_wall_min_slide_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wall_min_slide_angle(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_wall_min_slide_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_up_direction(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_up_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_up_direction(&mut self, up_direction: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_up_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_mode(&mut self, mode: crate::classes::character_body_3d::MotionMode,) {
            type CallSig = ((), crate::classes::character_body_3d::MotionMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_motion_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_mode(&self,) -> crate::classes::character_body_3d::MotionMode {
            type CallSig = (crate::classes::character_body_3d::MotionMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_motion_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_platform_on_leave(&mut self, on_leave_apply_velocity: crate::classes::character_body_3d::PlatformOnLeave,) {
            type CallSig = ((), crate::classes::character_body_3d::PlatformOnLeave);
            let args = (on_leave_apply_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_platform_on_leave", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_on_leave(&self,) -> crate::classes::character_body_3d::PlatformOnLeave {
            type CallSig = (crate::classes::character_body_3d::PlatformOnLeave,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_on_leave", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_floor(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_floor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_floor_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_floor_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_ceiling(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_ceiling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_ceiling_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_ceiling_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_wall(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_wall", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_on_wall_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_wall_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_floor_normal(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wall_normal(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_wall_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_motion(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_last_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position_delta(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_position_delta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_real_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_real_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_floor_angle_full(&self, up_direction: Vector3,) -> f32 {
            type CallSig = (f32, Vector3);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_floor_angle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_floor_angle(&self,) -> f32 {
            self.get_floor_angle_ex() . done()
        }
        #[inline]
        pub fn get_floor_angle_ex < 'a > (&'a self,) -> ExGetFloorAngle < 'a > {
            ExGetFloorAngle::new(self,)
        }
        pub fn get_platform_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_platform_angular_velocity(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_slide_collision_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_slide_collision_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_slide_collision(&mut self, slide_idx: i32,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            type CallSig = (Option < Gd < crate::classes::KinematicCollision3D > >, i32);
            let args = (slide_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_slide_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_slide_collision(&mut self,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            type CallSig = (Option < Gd < crate::classes::KinematicCollision3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_last_slide_collision", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CharacterBody3D {
        type Base = crate::classes::PhysicsBody3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CharacterBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CharacterBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody3D > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CharacterBody3D {
        
    }
    impl crate::obj::cap::GodotDefault for CharacterBody3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CharacterBody3D {
        type Target = crate::classes::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CharacterBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CharacterBody3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CharacterBody3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CharacterBody3D > for $Class {
                
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
#[doc = "Default-param extender for [`CharacterBody3D::get_floor_angle_ex`][super::CharacterBody3D::get_floor_angle_ex]."]
#[must_use]
pub struct ExGetFloorAngle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CharacterBody3D, up_direction: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetFloorAngle < 'a > {
    fn new(surround_object: &'a re_export::CharacterBody3D,) -> Self {
        let up_direction = Vector3::new(0 as _, 1 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, up_direction: up_direction,
        }
    }
    #[inline]
    pub fn up_direction(self, up_direction: Vector3) -> Self {
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
        re_export::CharacterBody3D::get_floor_angle_full(surround_object, up_direction,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MotionMode {
    ord: i32
}
impl MotionMode {
    #[doc(alias = "MOTION_MODE_GROUNDED")]
    #[doc = "Godot enumerator name: `MOTION_MODE_GROUNDED`"]
    pub const GROUNDED: MotionMode = MotionMode {
        ord: 0i32
    };
    #[doc(alias = "MOTION_MODE_FLOATING")]
    #[doc = "Godot enumerator name: `MOTION_MODE_FLOATING`"]
    pub const FLOATING: MotionMode = MotionMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MotionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MotionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MotionMode {
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
            Self::GROUNDED => "GROUNDED", Self::FLOATING => "FLOATING", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GROUNDED => "MOTION_MODE_GROUNDED", Self::FLOATING => "MOTION_MODE_FLOATING", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MotionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MotionMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MotionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlatformOnLeave {
    ord: i32
}
impl PlatformOnLeave {
    #[doc(alias = "PLATFORM_ON_LEAVE_ADD_VELOCITY")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_ADD_VELOCITY`"]
    pub const ADD_VELOCITY: PlatformOnLeave = PlatformOnLeave {
        ord: 0i32
    };
    #[doc(alias = "PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY`"]
    pub const ADD_UPWARD_VELOCITY: PlatformOnLeave = PlatformOnLeave {
        ord: 1i32
    };
    #[doc(alias = "PLATFORM_ON_LEAVE_DO_NOTHING")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_DO_NOTHING`"]
    pub const DO_NOTHING: PlatformOnLeave = PlatformOnLeave {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PlatformOnLeave {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlatformOnLeave") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlatformOnLeave {
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
            Self::ADD_VELOCITY => "ADD_VELOCITY", Self::ADD_UPWARD_VELOCITY => "ADD_UPWARD_VELOCITY", Self::DO_NOTHING => "DO_NOTHING", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ADD_VELOCITY => "PLATFORM_ON_LEAVE_ADD_VELOCITY", Self::ADD_UPWARD_VELOCITY => "PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY", Self::DO_NOTHING => "PLATFORM_ON_LEAVE_DO_NOTHING", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PlatformOnLeave {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlatformOnLeave {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlatformOnLeave {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
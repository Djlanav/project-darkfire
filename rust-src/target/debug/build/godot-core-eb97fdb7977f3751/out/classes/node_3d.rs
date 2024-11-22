#![doc = "Sidecar module for class [`Node3D`][crate::classes::Node3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node3D` enums](https://docs.godotengine.org/en/stable/classes/class_node3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node3D.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`node_3d`][crate::classes::node_3d]: sidecar module with related enum/flag types\n* [`INode3D`][crate::classes::INode3D]: virtual methods\n* [`Node3DNotification`][crate::classes::notify::Node3DNotification]: notification type\n\n\nSee also [Godot docs for `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Node3D`][crate::classes::Node3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node3D` methods](https://docs.godotengine.org/en/stable/classes/class_node3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode3D: crate::obj::GodotClass < Base = Node3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Node3D`][crate::classes::Node3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum Node3DNotification {
        TRANSFORM_CHANGED = 2000i32, ENTER_WORLD = 41i32, EXIT_WORLD = 42i32, VISIBILITY_CHANGED = 43i32, LOCAL_TRANSFORM_CHANGED = 44i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for Node3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TRANSFORM_CHANGED, 41i32 => Self::ENTER_WORLD, 42i32 => Self::EXIT_WORLD, 43i32 => Self::VISIBILITY_CHANGED, 44i32 => Self::LOCAL_TRANSFORM_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < Node3DNotification > for i32 {
        fn from(notification: Node3DNotification) -> i32 {
            match notification {
                Node3DNotification::TRANSFORM_CHANGED => 2000i32, Node3DNotification::ENTER_WORLD => 41i32, Node3DNotification::EXIT_WORLD => 42i32, Node3DNotification::VISIBILITY_CHANGED => 43i32, Node3DNotification::LOCAL_TRANSFORM_CHANGED => 44i32, Node3DNotification::ENTER_TREE => 10i32, Node3DNotification::EXIT_TREE => 11i32, Node3DNotification::MOVED_IN_PARENT => 12i32, Node3DNotification::READY => 13i32, Node3DNotification::PAUSED => 14i32, Node3DNotification::UNPAUSED => 15i32, Node3DNotification::PHYSICS_PROCESS => 16i32, Node3DNotification::PROCESS => 17i32, Node3DNotification::PARENTED => 18i32, Node3DNotification::UNPARENTED => 19i32, Node3DNotification::SCENE_INSTANTIATED => 20i32, Node3DNotification::DRAG_BEGIN => 21i32, Node3DNotification::DRAG_END => 22i32, Node3DNotification::PATH_RENAMED => 23i32, Node3DNotification::CHILD_ORDER_CHANGED => 24i32, Node3DNotification::INTERNAL_PROCESS => 25i32, Node3DNotification::INTERNAL_PHYSICS_PROCESS => 26i32, Node3DNotification::POST_ENTER_TREE => 27i32, Node3DNotification::DISABLED => 28i32, Node3DNotification::ENABLED => 29i32, Node3DNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, Node3DNotification::EDITOR_PRE_SAVE => 9001i32, Node3DNotification::EDITOR_POST_SAVE => 9002i32, Node3DNotification::WM_MOUSE_ENTER => 1002i32, Node3DNotification::WM_MOUSE_EXIT => 1003i32, Node3DNotification::WM_WINDOW_FOCUS_IN => 1004i32, Node3DNotification::WM_WINDOW_FOCUS_OUT => 1005i32, Node3DNotification::WM_CLOSE_REQUEST => 1006i32, Node3DNotification::WM_GO_BACK_REQUEST => 1007i32, Node3DNotification::WM_SIZE_CHANGED => 1008i32, Node3DNotification::WM_DPI_CHANGE => 1009i32, Node3DNotification::VP_MOUSE_ENTER => 1010i32, Node3DNotification::VP_MOUSE_EXIT => 1011i32, Node3DNotification::OS_MEMORY_WARNING => 2009i32, Node3DNotification::TRANSLATION_CHANGED => 2010i32, Node3DNotification::WM_ABOUT => 2011i32, Node3DNotification::CRASH => 2012i32, Node3DNotification::OS_IME_UPDATE => 2013i32, Node3DNotification::APPLICATION_RESUMED => 2014i32, Node3DNotification::APPLICATION_PAUSED => 2015i32, Node3DNotification::APPLICATION_FOCUS_IN => 2016i32, Node3DNotification::APPLICATION_FOCUS_OUT => 2017i32, Node3DNotification::TEXT_SERVER_CHANGED => 2018i32, Node3DNotification::POSTINITIALIZE => 0i32, Node3DNotification::PREDELETE => 1i32, Node3DNotification::EXTENSION_RELOADED => 2i32, Node3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Node3D {
        pub fn set_transform(&mut self, local: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (local,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation(&mut self, euler_radians: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, euler_degrees: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (euler_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_order(&mut self, order: crate::global::EulerOrder,) {
            type CallSig = ((), crate::global::EulerOrder);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_order(&self,) -> crate::global::EulerOrder {
            type CallSig = (crate::global::EulerOrder,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_edit_mode(&mut self, edit_mode: crate::classes::node_3d::RotationEditMode,) {
            type CallSig = ((), crate::classes::node_3d::RotationEditMode);
            let args = (edit_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation_edit_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_edit_mode(&self,) -> crate::classes::node_3d::RotationEditMode {
            type CallSig = (crate::classes::node_3d::RotationEditMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation_edit_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_quaternion(&mut self, quaternion: Quaternion,) {
            type CallSig = ((), Quaternion);
            let args = (quaternion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_quaternion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_quaternion(&self,) -> Quaternion {
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_quaternion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_basis(&mut self, basis: Basis,) {
            type CallSig = ((), Basis);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_basis(&self,) -> Basis {
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_transform(&mut self, global: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_transform(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_position(&mut self, position: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_basis(&mut self, basis: Basis,) {
            type CallSig = ((), Basis);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_global_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_basis(&self,) -> Basis {
            type CallSig = (Basis,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_global_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation(&mut self, euler_radians: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation_degrees(&mut self, euler_degrees: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (euler_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation_degrees(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_node_3d(&self,) -> Option < Gd < crate::classes::Node3D > > {
            type CallSig = (Option < Gd < crate::classes::Node3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_parent_node_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ignore_transform_notification(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_ignore_transform_notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_as_top_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_set_as_top_level(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "is_set_as_top_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_scale(&mut self, disable: bool,) {
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_disable_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scale_disabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "is_scale_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_3d(&self,) -> Option < Gd < crate::classes::World3D > > {
            type CallSig = (Option < Gd < crate::classes::World3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_update_transform(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "force_update_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_parent(&mut self, path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_visibility_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_parent(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_visibility_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_gizmos(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "update_gizmos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_gizmo(&mut self, gizmo: impl AsObjectArg < crate::classes::Node3DGizmo >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node3DGizmo >);
            let args = (gizmo.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "add_gizmo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gizmos(&self,) -> Array < Gd < crate::classes::Node3DGizmo > > {
            type CallSig = (Array < Gd < crate::classes::Node3DGizmo > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "get_gizmos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_gizmos(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "clear_gizmos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_subgizmo_selection(&mut self, gizmo: impl AsObjectArg < crate::classes::Node3DGizmo >, id: i32, transform: Transform3D,) {
            type CallSig = ((), ObjectArg < crate::classes::Node3DGizmo >, i32, Transform3D);
            let args = (gizmo.as_object_arg(), id, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_subgizmo_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_subgizmo_selection(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "clear_subgizmo_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "is_visible_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn show(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "show", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn hide(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "hide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_notify_local_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_local_transform_notification_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "is_local_transform_notification_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_notify_transform(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_notify_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_transform_notification_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "is_transform_notification_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate(&mut self, axis: Vector3, angle: f32,) {
            type CallSig = ((), Vector3, f32);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "rotate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_rotate(&mut self, axis: Vector3, angle: f32,) {
            type CallSig = ((), Vector3, f32);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "global_rotate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_scale(&mut self, scale: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "global_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_translate(&mut self, offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "global_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_object_local(&mut self, axis: Vector3, angle: f32,) {
            type CallSig = ((), Vector3, f32);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "rotate_object_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scale_object_local(&mut self, scale: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "scale_object_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn translate_object_local(&mut self, offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "translate_object_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_x(&mut self, angle: f32,) {
            type CallSig = ((), f32);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "rotate_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_y(&mut self, angle: f32,) {
            type CallSig = ((), f32);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "rotate_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate_z(&mut self, angle: f32,) {
            type CallSig = ((), f32);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "rotate_z", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn translate(&mut self, offset: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn orthonormalize(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "orthonormalize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_identity(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "set_identity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn look_at_full(&mut self, target: Vector3, up: Vector3, use_model_front: bool,) {
            type CallSig = ((), Vector3, Vector3, bool);
            let args = (target, up, use_model_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "look_at", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::look_at_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn look_at(&mut self, target: Vector3,) {
            self.look_at_ex(target,) . done()
        }
        #[inline]
        pub fn look_at_ex < 'a > (&'a mut self, target: Vector3,) -> ExLookAt < 'a > {
            ExLookAt::new(self, target,)
        }
        pub(crate) fn look_at_from_position_full(&mut self, position: Vector3, target: Vector3, up: Vector3, use_model_front: bool,) {
            type CallSig = ((), Vector3, Vector3, Vector3, bool);
            let args = (position, target, up, use_model_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "look_at_from_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::look_at_from_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn look_at_from_position(&mut self, position: Vector3, target: Vector3,) {
            self.look_at_from_position_ex(position, target,) . done()
        }
        #[inline]
        pub fn look_at_from_position_ex < 'a > (&'a mut self, position: Vector3, target: Vector3,) -> ExLookAtFromPosition < 'a > {
            ExLookAtFromPosition::new(self, position, target,)
        }
        pub fn to_local(&self, global_point: Vector3,) -> Vector3 {
            type CallSig = (Vector3, Vector3);
            let args = (global_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn to_global(&self, local_point: Vector3,) -> Vector3 {
            type CallSig = (Vector3, Vector3);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node3D", "to_global", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_ENTER_WORLD: i32 = 41i32;
        pub(crate) const NOTIFICATION_EXIT_WORLD: i32 = 42i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 43i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 44i32;
        
    }
    impl crate::obj::GodotClass for Node3D {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Node3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Node3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node3D {
        
    }
    impl crate::obj::cap::GodotDefault for Node3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node3D {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Node3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node3D::look_at_ex`][super::Node3D::look_at_ex]."]
#[must_use]
pub struct ExLookAt < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node3D, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLookAt < 'a > {
    fn new(surround_object: &'a mut re_export::Node3D, target: Vector3,) -> Self {
        let up = Vector3::new(0 as _, 1 as _, 0 as _);
        let use_model_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, target: target, up: up, use_model_front: use_model_front,
        }
    }
    #[inline]
    pub fn up(self, up: Vector3) -> Self {
        Self {
            up: up, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, use_model_front: bool) -> Self {
        Self {
            use_model_front: use_model_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, target, up, use_model_front,
        }
        = self;
        re_export::Node3D::look_at_full(surround_object, target, up, use_model_front,)
    }
}
#[doc = "Default-param extender for [`Node3D::look_at_from_position_ex`][super::Node3D::look_at_from_position_ex]."]
#[must_use]
pub struct ExLookAtFromPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node3D, position: Vector3, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLookAtFromPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Node3D, position: Vector3, target: Vector3,) -> Self {
        let up = Vector3::new(0 as _, 1 as _, 0 as _);
        let use_model_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, target: target, up: up, use_model_front: use_model_front,
        }
    }
    #[inline]
    pub fn up(self, up: Vector3) -> Self {
        Self {
            up: up, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, use_model_front: bool) -> Self {
        Self {
            use_model_front: use_model_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, target, up, use_model_front,
        }
        = self;
        re_export::Node3D::look_at_from_position_full(surround_object, position, target, up, use_model_front,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RotationEditMode {
    ord: i32
}
impl RotationEditMode {
    #[doc(alias = "ROTATION_EDIT_MODE_EULER")]
    #[doc = "Godot enumerator name: `ROTATION_EDIT_MODE_EULER`"]
    pub const EULER: RotationEditMode = RotationEditMode {
        ord: 0i32
    };
    #[doc(alias = "ROTATION_EDIT_MODE_QUATERNION")]
    #[doc = "Godot enumerator name: `ROTATION_EDIT_MODE_QUATERNION`"]
    pub const QUATERNION: RotationEditMode = RotationEditMode {
        ord: 1i32
    };
    #[doc(alias = "ROTATION_EDIT_MODE_BASIS")]
    #[doc = "Godot enumerator name: `ROTATION_EDIT_MODE_BASIS`"]
    pub const BASIS: RotationEditMode = RotationEditMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RotationEditMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RotationEditMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RotationEditMode {
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
            Self::EULER => "EULER", Self::QUATERNION => "QUATERNION", Self::BASIS => "BASIS", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::EULER => "ROTATION_EDIT_MODE_EULER", Self::QUATERNION => "ROTATION_EDIT_MODE_QUATERNION", Self::BASIS => "ROTATION_EDIT_MODE_BASIS", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for RotationEditMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RotationEditMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RotationEditMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
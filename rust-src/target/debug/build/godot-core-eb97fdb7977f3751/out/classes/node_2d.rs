#![doc = "Sidecar module for class [`Node2D`][crate::classes::Node2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node2D` enums](https://docs.godotengine.org/en/stable/classes/class_node2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node2D.`\n\nInherits [`CanvasItem`][crate::classes::CanvasItem].\n\nRelated symbols:\n\n* [`node_2d`][crate::classes::node_2d]: sidecar module with related enum/flag types\n* [`INode2D`][crate::classes::INode2D]: virtual methods\n\n\nSee also [Godot docs for `Node2D`](https://docs.godotengine.org/en/stable/classes/class_node2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Node2D`][crate::classes::Node2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node2D` methods](https://docs.godotengine.org/en/stable/classes/class_node2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode2D: crate::obj::GodotClass < Base = Node2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
        fn draw(&mut self,) {
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
    impl Node2D {
        pub fn set_position(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skew(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skew(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "rotate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn move_local_x_full(&mut self, delta: f32, scaled: bool,) {
            type CallSig = ((), f32, bool);
            let args = (delta, scaled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "move_local_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_local_x_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_local_x(&mut self, delta: f32,) {
            self.move_local_x_ex(delta,) . done()
        }
        #[inline]
        pub fn move_local_x_ex < 'a > (&'a mut self, delta: f32,) -> ExMoveLocalX < 'a > {
            ExMoveLocalX::new(self, delta,)
        }
        pub(crate) fn move_local_y_full(&mut self, delta: f32, scaled: bool,) {
            type CallSig = ((), f32, bool);
            let args = (delta, scaled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "move_local_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_local_y_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_local_y(&mut self, delta: f32,) {
            self.move_local_y_ex(delta,) . done()
        }
        #[inline]
        pub fn move_local_y_ex < 'a > (&'a mut self, delta: f32,) -> ExMoveLocalY < 'a > {
            ExMoveLocalY::new(self, delta,)
        }
        pub fn translate(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_translate(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "global_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_scale(&mut self, ratio: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "apply_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_position(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation_degrees(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation_degrees(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_skew(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_global_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_skew(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_global_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_scale(&mut self, scale: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_global_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_scale(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_global_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, xform: Transform2D,) {
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_transform(&mut self, xform: Transform2D,) {
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "set_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn look_at(&mut self, point: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "look_at", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angle_to(&self, point: Vector2,) -> f32 {
            type CallSig = (f32, Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_angle_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn to_local(&self, global_point: Vector2,) -> Vector2 {
            type CallSig = (Vector2, Vector2);
            let args = (global_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn to_global(&self, local_point: Vector2,) -> Vector2 {
            type CallSig = (Vector2, Vector2);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "to_global", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_relative_transform_to_parent(&self, parent: impl AsObjectArg < crate::classes::Node >,) -> Transform2D {
            type CallSig = (Transform2D, ObjectArg < crate::classes::Node >);
            let args = (parent.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node2D", "get_relative_transform_to_parent", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Node2D {
        type Base = crate::classes::CanvasItem;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Node2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Node2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Node2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node2D {
        
    }
    impl crate::obj::cap::GodotDefault for Node2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node2D {
        type Target = crate::classes::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Node2D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node2D::move_local_x_ex`][super::Node2D::move_local_x_ex]."]
#[must_use]
pub struct ExMoveLocalX < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node2D, delta: f32, scaled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveLocalX < 'a > {
    fn new(surround_object: &'a mut re_export::Node2D, delta: f32,) -> Self {
        let scaled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delta: delta, scaled: scaled,
        }
    }
    #[inline]
    pub fn scaled(self, scaled: bool) -> Self {
        Self {
            scaled: scaled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, delta, scaled,
        }
        = self;
        re_export::Node2D::move_local_x_full(surround_object, delta, scaled,)
    }
}
#[doc = "Default-param extender for [`Node2D::move_local_y_ex`][super::Node2D::move_local_y_ex]."]
#[must_use]
pub struct ExMoveLocalY < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node2D, delta: f32, scaled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveLocalY < 'a > {
    fn new(surround_object: &'a mut re_export::Node2D, delta: f32,) -> Self {
        let scaled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delta: delta, scaled: scaled,
        }
    }
    #[inline]
    pub fn scaled(self, scaled: bool) -> Self {
        Self {
            scaled: scaled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, delta, scaled,
        }
        = self;
        re_export::Node2D::move_local_y_full(surround_object, delta, scaled,)
    }
}
#![doc = "Sidecar module for class [`VisualShader`][crate::classes::VisualShader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShader` enums](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShader.`\n\nInherits [`Shader`][crate::classes::Shader].\n\nRelated symbols:\n\n* [`visual_shader`][crate::classes::visual_shader]: sidecar module with related enum/flag types\n* [`IVisualShader`][crate::classes::IVisualShader]: virtual methods\n\n\nSee also [Godot docs for `VisualShader`](https://docs.godotengine.org/en/stable/classes/class_visualshader.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShader::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShader`][crate::classes::VisualShader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShader` methods](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShader: crate::obj::GodotClass < Base = VisualShader > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl VisualShader {
        pub fn set_mode(&mut self, mode: crate::classes::shader::Mode,) {
            type CallSig = ((), crate::classes::shader::Mode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_node(&mut self, type_: crate::classes::visual_shader::Type, node: impl AsObjectArg < crate::classes::VisualShaderNode >, position: Vector2, id: i32,) {
            type CallSig = ((), crate::classes::visual_shader::Type, ObjectArg < crate::classes::VisualShaderNode >, Vector2, i32);
            let args = (type_, node.as_object_arg(), position, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "add_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node(&self, type_: crate::classes::visual_shader::Type, id: i32,) -> Option < Gd < crate::classes::VisualShaderNode > > {
            type CallSig = (Option < Gd < crate::classes::VisualShaderNode > >, crate::classes::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "get_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_node_position(&mut self, type_: crate::classes::visual_shader::Type, id: i32, position: Vector2,) {
            type CallSig = ((), crate::classes::visual_shader::Type, i32, Vector2);
            let args = (type_, id, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "set_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_position(&self, type_: crate::classes::visual_shader::Type, id: i32,) -> Vector2 {
            type CallSig = (Vector2, crate::classes::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "get_node_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_list(&self, type_: crate::classes::visual_shader::Type,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array, crate::classes::visual_shader::Type);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "get_node_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_valid_node_id(&self, type_: crate::classes::visual_shader::Type,) -> i32 {
            type CallSig = (i32, crate::classes::visual_shader::Type);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "get_valid_node_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_node(&mut self, type_: crate::classes::visual_shader::Type, id: i32,) {
            type CallSig = ((), crate::classes::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "remove_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn replace_node(&mut self, type_: crate::classes::visual_shader::Type, id: i32, new_class: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), crate::classes::visual_shader::Type, i32, CowArg < 'a0, StringName >);
            let args = (type_, id, new_class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "replace_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_node_connection(&self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> bool {
            type CallSig = (bool, crate::classes::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "is_node_connection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_connect_nodes(&self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> bool {
            type CallSig = (bool, crate::classes::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "can_connect_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_nodes(&mut self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, crate::classes::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9958usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "connect_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn disconnect_nodes(&mut self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) {
            type CallSig = ((), crate::classes::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9959usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "disconnect_nodes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_nodes_forced(&mut self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) {
            type CallSig = ((), crate::classes::visual_shader::Type, i32, i32, i32, i32);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9960usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "connect_nodes_forced", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_connections(&self, type_: crate::classes::visual_shader::Type,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >, crate::classes::visual_shader::Type);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9961usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "get_node_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9962usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "set_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_graph_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9963usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "get_graph_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn attach_node_to_frame(&mut self, type_: crate::classes::visual_shader::Type, id: i32, frame: i32,) {
            type CallSig = ((), crate::classes::visual_shader::Type, i32, i32);
            let args = (type_, id, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9964usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "attach_node_to_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn detach_node_from_frame(&mut self, type_: crate::classes::visual_shader::Type, id: i32,) {
            type CallSig = ((), crate::classes::visual_shader::Type, i32);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9965usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "detach_node_from_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_varying(&mut self, name: impl AsArg < GString >, mode: crate::classes::visual_shader::VaryingMode, type_: crate::classes::visual_shader::VaryingType,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, crate::classes::visual_shader::VaryingMode, crate::classes::visual_shader::VaryingType);
            let args = (name.into_arg(), mode, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9966usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "add_varying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_varying(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "remove_varying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_varying(&self, name: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShader", "has_varying", self.object_ptr, self.__checked_id(), args,)
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
        pub const NODE_ID_INVALID: i32 = - 1i32;
        pub const NODE_ID_OUTPUT: i32 = 0i32;
        
    }
    impl crate::obj::GodotClass for VisualShader {
        type Base = crate::classes::Shader;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VisualShader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Shader > for VisualShader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShader {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShader {
        type Target = crate::classes::Shader;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShader`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VisualShader {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShader > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Shader > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    ord: i32
}
impl Type {
    #[doc(alias = "TYPE_VERTEX")]
    #[doc = "Godot enumerator name: `TYPE_VERTEX`"]
    pub const VERTEX: Type = Type {
        ord: 0i32
    };
    #[doc(alias = "TYPE_FRAGMENT")]
    #[doc = "Godot enumerator name: `TYPE_FRAGMENT`"]
    pub const FRAGMENT: Type = Type {
        ord: 1i32
    };
    #[doc(alias = "TYPE_LIGHT")]
    #[doc = "Godot enumerator name: `TYPE_LIGHT`"]
    pub const LIGHT: Type = Type {
        ord: 2i32
    };
    #[doc(alias = "TYPE_START")]
    #[doc = "Godot enumerator name: `TYPE_START`"]
    pub const START: Type = Type {
        ord: 3i32
    };
    #[doc(alias = "TYPE_PROCESS")]
    #[doc = "Godot enumerator name: `TYPE_PROCESS`"]
    pub const PROCESS: Type = Type {
        ord: 4i32
    };
    #[doc(alias = "TYPE_COLLIDE")]
    #[doc = "Godot enumerator name: `TYPE_COLLIDE`"]
    pub const COLLIDE: Type = Type {
        ord: 5i32
    };
    #[doc(alias = "TYPE_START_CUSTOM")]
    #[doc = "Godot enumerator name: `TYPE_START_CUSTOM`"]
    pub const START_CUSTOM: Type = Type {
        ord: 6i32
    };
    #[doc(alias = "TYPE_PROCESS_CUSTOM")]
    #[doc = "Godot enumerator name: `TYPE_PROCESS_CUSTOM`"]
    pub const PROCESS_CUSTOM: Type = Type {
        ord: 7i32
    };
    #[doc(alias = "TYPE_SKY")]
    #[doc = "Godot enumerator name: `TYPE_SKY`"]
    pub const SKY: Type = Type {
        ord: 8i32
    };
    #[doc(alias = "TYPE_FOG")]
    #[doc = "Godot enumerator name: `TYPE_FOG`"]
    pub const FOG: Type = Type {
        ord: 9i32
    };
    #[doc(alias = "TYPE_MAX")]
    #[doc = "Godot enumerator name: `TYPE_MAX`"]
    pub const MAX: Type = Type {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Type") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Type {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::FRAGMENT => "FRAGMENT", Self::LIGHT => "LIGHT", Self::START => "START", Self::PROCESS => "PROCESS", Self::COLLIDE => "COLLIDE", Self::START_CUSTOM => "START_CUSTOM", Self::PROCESS_CUSTOM => "PROCESS_CUSTOM", Self::SKY => "SKY", Self::FOG => "FOG", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VERTEX => "TYPE_VERTEX", Self::FRAGMENT => "TYPE_FRAGMENT", Self::LIGHT => "TYPE_LIGHT", Self::START => "TYPE_START", Self::PROCESS => "TYPE_PROCESS", Self::COLLIDE => "TYPE_COLLIDE", Self::START_CUSTOM => "TYPE_START_CUSTOM", Self::PROCESS_CUSTOM => "TYPE_PROCESS_CUSTOM", Self::SKY => "TYPE_SKY", Self::FOG => "TYPE_FOG", Self::MAX => "TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Type {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for Type {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Type {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VaryingMode {
    ord: i32
}
impl VaryingMode {
    #[doc(alias = "VARYING_MODE_VERTEX_TO_FRAG_LIGHT")]
    #[doc = "Godot enumerator name: `VARYING_MODE_VERTEX_TO_FRAG_LIGHT`"]
    pub const VERTEX_TO_FRAG_LIGHT: VaryingMode = VaryingMode {
        ord: 0i32
    };
    #[doc(alias = "VARYING_MODE_FRAG_TO_LIGHT")]
    #[doc = "Godot enumerator name: `VARYING_MODE_FRAG_TO_LIGHT`"]
    pub const FRAG_TO_LIGHT: VaryingMode = VaryingMode {
        ord: 1i32
    };
    #[doc(alias = "VARYING_MODE_MAX")]
    #[doc = "Godot enumerator name: `VARYING_MODE_MAX`"]
    pub const MAX: VaryingMode = VaryingMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VaryingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VaryingMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VaryingMode {
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
            Self::VERTEX_TO_FRAG_LIGHT => "VERTEX_TO_FRAG_LIGHT", Self::FRAG_TO_LIGHT => "FRAG_TO_LIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VERTEX_TO_FRAG_LIGHT => "VARYING_MODE_VERTEX_TO_FRAG_LIGHT", Self::FRAG_TO_LIGHT => "VARYING_MODE_FRAG_TO_LIGHT", Self::MAX => "VARYING_MODE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for VaryingMode {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for VaryingMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VaryingMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VaryingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VaryingType {
    ord: i32
}
impl VaryingType {
    #[doc(alias = "VARYING_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_FLOAT`"]
    pub const FLOAT: VaryingType = VaryingType {
        ord: 0i32
    };
    #[doc(alias = "VARYING_TYPE_INT")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_INT`"]
    pub const INT: VaryingType = VaryingType {
        ord: 1i32
    };
    #[doc(alias = "VARYING_TYPE_UINT")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_UINT`"]
    pub const UINT: VaryingType = VaryingType {
        ord: 2i32
    };
    #[doc(alias = "VARYING_TYPE_VECTOR_2D")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_VECTOR_2D`"]
    pub const VECTOR_2D: VaryingType = VaryingType {
        ord: 3i32
    };
    #[doc(alias = "VARYING_TYPE_VECTOR_3D")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_VECTOR_3D`"]
    pub const VECTOR_3D: VaryingType = VaryingType {
        ord: 4i32
    };
    #[doc(alias = "VARYING_TYPE_VECTOR_4D")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_VECTOR_4D`"]
    pub const VECTOR_4D: VaryingType = VaryingType {
        ord: 5i32
    };
    #[doc(alias = "VARYING_TYPE_BOOLEAN")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_BOOLEAN`"]
    pub const BOOLEAN: VaryingType = VaryingType {
        ord: 6i32
    };
    #[doc(alias = "VARYING_TYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_TRANSFORM`"]
    pub const TRANSFORM: VaryingType = VaryingType {
        ord: 7i32
    };
    #[doc(alias = "VARYING_TYPE_MAX")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_MAX`"]
    pub const MAX: VaryingType = VaryingType {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for VaryingType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VaryingType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VaryingType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::FLOAT => "FLOAT", Self::INT => "INT", Self::UINT => "UINT", Self::VECTOR_2D => "VECTOR_2D", Self::VECTOR_3D => "VECTOR_3D", Self::VECTOR_4D => "VECTOR_4D", Self::BOOLEAN => "BOOLEAN", Self::TRANSFORM => "TRANSFORM", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::FLOAT => "VARYING_TYPE_FLOAT", Self::INT => "VARYING_TYPE_INT", Self::UINT => "VARYING_TYPE_UINT", Self::VECTOR_2D => "VARYING_TYPE_VECTOR_2D", Self::VECTOR_3D => "VARYING_TYPE_VECTOR_3D", Self::VECTOR_4D => "VARYING_TYPE_VECTOR_4D", Self::BOOLEAN => "VARYING_TYPE_BOOLEAN", Self::TRANSFORM => "VARYING_TYPE_TRANSFORM", Self::MAX => "VARYING_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for VaryingType {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for VaryingType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VaryingType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VaryingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
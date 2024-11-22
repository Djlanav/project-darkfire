#![doc = "Sidecar module for class [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNode` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNode.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`visual_shader_node`][crate::classes::visual_shader_node]: sidecar module with related enum/flag types\n* [`IVisualShaderNode`][crate::classes::IVisualShaderNode]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNode`](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNode>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNode {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNode` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNode: crate::obj::GodotClass < Base = VisualShaderNode > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNode {
        pub fn get_default_input_port(&self, type_: crate::classes::visual_shader_node::PortType,) -> i32 {
            type CallSig = (i32, crate::classes::visual_shader_node::PortType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_default_input_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_output_port_for_preview(&mut self, port: i32,) {
            type CallSig = ((), i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_output_port_for_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_port_for_preview(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_output_port_for_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_input_port_default_value_full(&mut self, port: i32, value: RefArg < Variant >, prev_value: RefArg < Variant >,) {
            type CallSig < 'a0, 'a1, > = ((), i32, RefArg < 'a0, Variant >, RefArg < 'a1, Variant >);
            let args = (port, value, prev_value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_input_port_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_input_port_default_value_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_input_port_default_value(&mut self, port: i32, value: &Variant,) {
            self.set_input_port_default_value_ex(port, value,) . done()
        }
        #[inline]
        pub fn set_input_port_default_value_ex < 'a > (&'a mut self, port: i32, value: &'a Variant,) -> ExSetInputPortDefaultValue < 'a > {
            ExSetInputPortDefaultValue::new(self, port, value,)
        }
        pub fn get_input_port_default_value(&self, port: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_input_port_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_input_port_default_value(&mut self, port: i32,) {
            type CallSig = ((), i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "remove_input_port_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_default_input_values(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "clear_default_input_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_input_values(&mut self, values: &VariantArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, VariantArray >);
            let args = (RefArg::new(values),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_default_input_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_input_values(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_default_input_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type CallSig = ((), i32);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_frame", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNode {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VisualShaderNode"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNode {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNode {
        
    }
    impl std::ops::Deref for VisualShaderNode {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNode`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VisualShaderNode {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
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
#[doc = "Default-param extender for [`VisualShaderNode::set_input_port_default_value_ex`][super::VisualShaderNode::set_input_port_default_value_ex]."]
#[must_use]
pub struct ExSetInputPortDefaultValue < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::VisualShaderNode, port: i32, value: CowArg < 'a, Variant >, prev_value: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetInputPortDefaultValue < 'a > {
    fn new(surround_object: &'a mut re_export::VisualShaderNode, port: i32, value: &'a Variant,) -> Self {
        let prev_value = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, value: CowArg::Borrowed(value), prev_value: CowArg::Owned(prev_value),
        }
    }
    #[inline]
    pub fn prev_value(self, prev_value: &'a Variant) -> Self {
        Self {
            prev_value: CowArg::Borrowed(prev_value), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, port, value, prev_value,
        }
        = self;
        re_export::VisualShaderNode::set_input_port_default_value_full(surround_object, port, value.cow_as_arg(), prev_value.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PortType {
    ord: i32
}
impl PortType {
    #[doc(alias = "PORT_TYPE_SCALAR")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SCALAR`"]
    pub const SCALAR: PortType = PortType {
        ord: 0i32
    };
    #[doc(alias = "PORT_TYPE_SCALAR_INT")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SCALAR_INT`"]
    pub const SCALAR_INT: PortType = PortType {
        ord: 1i32
    };
    #[doc(alias = "PORT_TYPE_SCALAR_UINT")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SCALAR_UINT`"]
    pub const SCALAR_UINT: PortType = PortType {
        ord: 2i32
    };
    #[doc(alias = "PORT_TYPE_VECTOR_2D")]
    #[doc = "Godot enumerator name: `PORT_TYPE_VECTOR_2D`"]
    pub const VECTOR_2D: PortType = PortType {
        ord: 3i32
    };
    #[doc(alias = "PORT_TYPE_VECTOR_3D")]
    #[doc = "Godot enumerator name: `PORT_TYPE_VECTOR_3D`"]
    pub const VECTOR_3D: PortType = PortType {
        ord: 4i32
    };
    #[doc(alias = "PORT_TYPE_VECTOR_4D")]
    #[doc = "Godot enumerator name: `PORT_TYPE_VECTOR_4D`"]
    pub const VECTOR_4D: PortType = PortType {
        ord: 5i32
    };
    #[doc(alias = "PORT_TYPE_BOOLEAN")]
    #[doc = "Godot enumerator name: `PORT_TYPE_BOOLEAN`"]
    pub const BOOLEAN: PortType = PortType {
        ord: 6i32
    };
    #[doc(alias = "PORT_TYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `PORT_TYPE_TRANSFORM`"]
    pub const TRANSFORM: PortType = PortType {
        ord: 7i32
    };
    #[doc(alias = "PORT_TYPE_SAMPLER")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SAMPLER`"]
    pub const SAMPLER: PortType = PortType {
        ord: 8i32
    };
    #[doc(alias = "PORT_TYPE_MAX")]
    #[doc = "Godot enumerator name: `PORT_TYPE_MAX`"]
    pub const MAX: PortType = PortType {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for PortType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PortType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PortType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::SCALAR => "SCALAR", Self::SCALAR_INT => "SCALAR_INT", Self::SCALAR_UINT => "SCALAR_UINT", Self::VECTOR_2D => "VECTOR_2D", Self::VECTOR_3D => "VECTOR_3D", Self::VECTOR_4D => "VECTOR_4D", Self::BOOLEAN => "BOOLEAN", Self::TRANSFORM => "TRANSFORM", Self::SAMPLER => "SAMPLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SCALAR => "PORT_TYPE_SCALAR", Self::SCALAR_INT => "PORT_TYPE_SCALAR_INT", Self::SCALAR_UINT => "PORT_TYPE_SCALAR_UINT", Self::VECTOR_2D => "PORT_TYPE_VECTOR_2D", Self::VECTOR_3D => "PORT_TYPE_VECTOR_3D", Self::VECTOR_4D => "PORT_TYPE_VECTOR_4D", Self::BOOLEAN => "PORT_TYPE_BOOLEAN", Self::TRANSFORM => "PORT_TYPE_TRANSFORM", Self::SAMPLER => "PORT_TYPE_SAMPLER", Self::MAX => "PORT_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for PortType {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::meta::GodotConvert for PortType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PortType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PortType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
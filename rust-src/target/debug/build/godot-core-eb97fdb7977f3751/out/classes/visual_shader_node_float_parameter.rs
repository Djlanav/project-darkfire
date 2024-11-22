#![doc = "Sidecar module for class [`VisualShaderNodeFloatParameter`][crate::classes::VisualShaderNodeFloatParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeFloatParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodefloatparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeFloatParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::classes::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_float_parameter`][crate::classes::visual_shader_node_float_parameter]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeFloatParameter`][crate::classes::IVisualShaderNodeFloatParameter]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeFloatParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodefloatparameter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeFloatParameter::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeFloatParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeFloatParameter`][crate::classes::VisualShaderNodeFloatParameter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeFloatParameter` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodefloatparameter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeFloatParameter: crate::obj::GodotClass < Base = VisualShaderNodeFloatParameter > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeFloatParameter {
        pub fn set_hint(&mut self, hint: crate::classes::visual_shader_node_float_parameter::Hint,) {
            type CallSig = ((), crate::classes::visual_shader_node_float_parameter::Hint);
            let args = (hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "set_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hint(&self,) -> crate::classes::visual_shader_node_float_parameter::Hint {
            type CallSig = (crate::classes::visual_shader_node_float_parameter::Hint,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "get_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_step(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "set_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_value_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "set_default_value_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_default_value_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "is_default_value_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_value(&mut self, value: f32,) {
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "set_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_value(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatParameter", "get_default_value", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeFloatParameter {
        type Base = crate::classes::VisualShaderNodeParameter;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VisualShaderNodeFloatParameter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeFloatParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeParameter > for VisualShaderNodeFloatParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeFloatParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeFloatParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeFloatParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeFloatParameter {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeFloatParameter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeFloatParameter {
        type Target = crate::classes::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeFloatParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeFloatParameter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VisualShaderNodeFloatParameter {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeFloatParameter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeParameter > for $Class {
                
            }
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Hint {
    ord: i32
}
impl Hint {
    #[doc(alias = "HINT_NONE")]
    #[doc = "Godot enumerator name: `HINT_NONE`"]
    pub const NONE: Hint = Hint {
        ord: 0i32
    };
    #[doc(alias = "HINT_RANGE")]
    #[doc = "Godot enumerator name: `HINT_RANGE`"]
    pub const RANGE: Hint = Hint {
        ord: 1i32
    };
    #[doc(alias = "HINT_RANGE_STEP")]
    #[doc = "Godot enumerator name: `HINT_RANGE_STEP`"]
    pub const RANGE_STEP: Hint = Hint {
        ord: 2i32
    };
    #[doc(alias = "HINT_MAX")]
    #[doc = "Godot enumerator name: `HINT_MAX`"]
    pub const MAX: Hint = Hint {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hint") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hint {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::NONE => "NONE", Self::RANGE => "RANGE", Self::RANGE_STEP => "RANGE_STEP", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "HINT_NONE", Self::RANGE => "HINT_RANGE", Self::RANGE_STEP => "HINT_RANGE_STEP", Self::MAX => "HINT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Hint {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for Hint {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hint {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hint {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
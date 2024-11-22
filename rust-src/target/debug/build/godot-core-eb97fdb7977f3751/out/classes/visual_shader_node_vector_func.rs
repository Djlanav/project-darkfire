#![doc = "Sidecar module for class [`VisualShaderNodeVectorFunc`][crate::classes::VisualShaderNodeVectorFunc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeVectorFunc.`\n\nInherits [`VisualShaderNodeVectorBase`][crate::classes::VisualShaderNodeVectorBase].\n\nRelated symbols:\n\n* [`visual_shader_node_vector_func`][crate::classes::visual_shader_node_vector_func]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeVectorFunc`][crate::classes::IVisualShaderNodeVectorFunc]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeVectorFunc::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeVectorFunc {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeVectorFunc`][crate::classes::VisualShaderNodeVectorFunc].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeVectorFunc: crate::obj::GodotClass < Base = VisualShaderNodeVectorFunc > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeVectorFunc {
        pub fn set_function(&mut self, func: crate::classes::visual_shader_node_vector_func::Function,) {
            type CallSig = ((), crate::classes::visual_shader_node_vector_func::Function);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeVectorFunc", "set_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_function(&self,) -> crate::classes::visual_shader_node_vector_func::Function {
            type CallSig = (crate::classes::visual_shader_node_vector_func::Function,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "VisualShaderNodeVectorFunc", "get_function", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeVectorFunc {
        type Base = crate::classes::VisualShaderNodeVectorBase;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"VisualShaderNodeVectorFunc"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeVectorFunc {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeVectorBase > for VisualShaderNodeVectorFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeVectorFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeVectorFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeVectorFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeVectorFunc {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeVectorFunc {
        type Target = crate::classes::VisualShaderNodeVectorBase;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeVectorFunc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeVectorFunc`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_VisualShaderNodeVectorFunc {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeVectorFunc > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeVectorBase > for $Class {
                
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
pub struct Function {
    ord: i32
}
impl Function {
    #[doc(alias = "FUNC_NORMALIZE")]
    #[doc = "Godot enumerator name: `FUNC_NORMALIZE`"]
    pub const NORMALIZE: Function = Function {
        ord: 0i32
    };
    #[doc(alias = "FUNC_SATURATE")]
    #[doc = "Godot enumerator name: `FUNC_SATURATE`"]
    pub const SATURATE: Function = Function {
        ord: 1i32
    };
    #[doc(alias = "FUNC_NEGATE")]
    #[doc = "Godot enumerator name: `FUNC_NEGATE`"]
    pub const NEGATE: Function = Function {
        ord: 2i32
    };
    #[doc(alias = "FUNC_RECIPROCAL")]
    #[doc = "Godot enumerator name: `FUNC_RECIPROCAL`"]
    pub const RECIPROCAL: Function = Function {
        ord: 3i32
    };
    #[doc(alias = "FUNC_ABS")]
    #[doc = "Godot enumerator name: `FUNC_ABS`"]
    pub const ABS: Function = Function {
        ord: 4i32
    };
    #[doc(alias = "FUNC_ACOS")]
    #[doc = "Godot enumerator name: `FUNC_ACOS`"]
    pub const ACOS: Function = Function {
        ord: 5i32
    };
    #[doc(alias = "FUNC_ACOSH")]
    #[doc = "Godot enumerator name: `FUNC_ACOSH`"]
    pub const ACOSH: Function = Function {
        ord: 6i32
    };
    #[doc(alias = "FUNC_ASIN")]
    #[doc = "Godot enumerator name: `FUNC_ASIN`"]
    pub const ASIN: Function = Function {
        ord: 7i32
    };
    #[doc(alias = "FUNC_ASINH")]
    #[doc = "Godot enumerator name: `FUNC_ASINH`"]
    pub const ASINH: Function = Function {
        ord: 8i32
    };
    #[doc(alias = "FUNC_ATAN")]
    #[doc = "Godot enumerator name: `FUNC_ATAN`"]
    pub const ATAN: Function = Function {
        ord: 9i32
    };
    #[doc(alias = "FUNC_ATANH")]
    #[doc = "Godot enumerator name: `FUNC_ATANH`"]
    pub const ATANH: Function = Function {
        ord: 10i32
    };
    #[doc(alias = "FUNC_CEIL")]
    #[doc = "Godot enumerator name: `FUNC_CEIL`"]
    pub const CEIL: Function = Function {
        ord: 11i32
    };
    #[doc(alias = "FUNC_COS")]
    #[doc = "Godot enumerator name: `FUNC_COS`"]
    pub const COS: Function = Function {
        ord: 12i32
    };
    #[doc(alias = "FUNC_COSH")]
    #[doc = "Godot enumerator name: `FUNC_COSH`"]
    pub const COSH: Function = Function {
        ord: 13i32
    };
    #[doc(alias = "FUNC_DEGREES")]
    #[doc = "Godot enumerator name: `FUNC_DEGREES`"]
    pub const DEGREES: Function = Function {
        ord: 14i32
    };
    #[doc(alias = "FUNC_EXP")]
    #[doc = "Godot enumerator name: `FUNC_EXP`"]
    pub const EXP: Function = Function {
        ord: 15i32
    };
    #[doc(alias = "FUNC_EXP2")]
    #[doc = "Godot enumerator name: `FUNC_EXP2`"]
    pub const EXP2: Function = Function {
        ord: 16i32
    };
    #[doc(alias = "FUNC_FLOOR")]
    #[doc = "Godot enumerator name: `FUNC_FLOOR`"]
    pub const FLOOR: Function = Function {
        ord: 17i32
    };
    #[doc(alias = "FUNC_FRACT")]
    #[doc = "Godot enumerator name: `FUNC_FRACT`"]
    pub const FRACT: Function = Function {
        ord: 18i32
    };
    #[doc(alias = "FUNC_INVERSE_SQRT")]
    #[doc = "Godot enumerator name: `FUNC_INVERSE_SQRT`"]
    pub const INVERSE_SQRT: Function = Function {
        ord: 19i32
    };
    #[doc(alias = "FUNC_LOG")]
    #[doc = "Godot enumerator name: `FUNC_LOG`"]
    pub const LOG: Function = Function {
        ord: 20i32
    };
    #[doc(alias = "FUNC_LOG2")]
    #[doc = "Godot enumerator name: `FUNC_LOG2`"]
    pub const LOG2: Function = Function {
        ord: 21i32
    };
    #[doc(alias = "FUNC_RADIANS")]
    #[doc = "Godot enumerator name: `FUNC_RADIANS`"]
    pub const RADIANS: Function = Function {
        ord: 22i32
    };
    #[doc(alias = "FUNC_ROUND")]
    #[doc = "Godot enumerator name: `FUNC_ROUND`"]
    pub const ROUND: Function = Function {
        ord: 23i32
    };
    #[doc(alias = "FUNC_ROUNDEVEN")]
    #[doc = "Godot enumerator name: `FUNC_ROUNDEVEN`"]
    pub const ROUNDEVEN: Function = Function {
        ord: 24i32
    };
    #[doc(alias = "FUNC_SIGN")]
    #[doc = "Godot enumerator name: `FUNC_SIGN`"]
    pub const SIGN: Function = Function {
        ord: 25i32
    };
    #[doc(alias = "FUNC_SIN")]
    #[doc = "Godot enumerator name: `FUNC_SIN`"]
    pub const SIN: Function = Function {
        ord: 26i32
    };
    #[doc(alias = "FUNC_SINH")]
    #[doc = "Godot enumerator name: `FUNC_SINH`"]
    pub const SINH: Function = Function {
        ord: 27i32
    };
    #[doc(alias = "FUNC_SQRT")]
    #[doc = "Godot enumerator name: `FUNC_SQRT`"]
    pub const SQRT: Function = Function {
        ord: 28i32
    };
    #[doc(alias = "FUNC_TAN")]
    #[doc = "Godot enumerator name: `FUNC_TAN`"]
    pub const TAN: Function = Function {
        ord: 29i32
    };
    #[doc(alias = "FUNC_TANH")]
    #[doc = "Godot enumerator name: `FUNC_TANH`"]
    pub const TANH: Function = Function {
        ord: 30i32
    };
    #[doc(alias = "FUNC_TRUNC")]
    #[doc = "Godot enumerator name: `FUNC_TRUNC`"]
    pub const TRUNC: Function = Function {
        ord: 31i32
    };
    #[doc(alias = "FUNC_ONEMINUS")]
    #[doc = "Godot enumerator name: `FUNC_ONEMINUS`"]
    pub const ONEMINUS: Function = Function {
        ord: 32i32
    };
    #[doc(alias = "FUNC_MAX")]
    #[doc = "Godot enumerator name: `FUNC_MAX`"]
    pub const MAX: Function = Function {
        ord: 33i32
    };
    
}
impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Function") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Function {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 => Some(Self {
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
            Self::NORMALIZE => "NORMALIZE", Self::SATURATE => "SATURATE", Self::NEGATE => "NEGATE", Self::RECIPROCAL => "RECIPROCAL", Self::ABS => "ABS", Self::ACOS => "ACOS", Self::ACOSH => "ACOSH", Self::ASIN => "ASIN", Self::ASINH => "ASINH", Self::ATAN => "ATAN", Self::ATANH => "ATANH", Self::CEIL => "CEIL", Self::COS => "COS", Self::COSH => "COSH", Self::DEGREES => "DEGREES", Self::EXP => "EXP", Self::EXP2 => "EXP2", Self::FLOOR => "FLOOR", Self::FRACT => "FRACT", Self::INVERSE_SQRT => "INVERSE_SQRT", Self::LOG => "LOG", Self::LOG2 => "LOG2", Self::RADIANS => "RADIANS", Self::ROUND => "ROUND", Self::ROUNDEVEN => "ROUNDEVEN", Self::SIGN => "SIGN", Self::SIN => "SIN", Self::SINH => "SINH", Self::SQRT => "SQRT", Self::TAN => "TAN", Self::TANH => "TANH", Self::TRUNC => "TRUNC", Self::ONEMINUS => "ONEMINUS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NORMALIZE => "FUNC_NORMALIZE", Self::SATURATE => "FUNC_SATURATE", Self::NEGATE => "FUNC_NEGATE", Self::RECIPROCAL => "FUNC_RECIPROCAL", Self::ABS => "FUNC_ABS", Self::ACOS => "FUNC_ACOS", Self::ACOSH => "FUNC_ACOSH", Self::ASIN => "FUNC_ASIN", Self::ASINH => "FUNC_ASINH", Self::ATAN => "FUNC_ATAN", Self::ATANH => "FUNC_ATANH", Self::CEIL => "FUNC_CEIL", Self::COS => "FUNC_COS", Self::COSH => "FUNC_COSH", Self::DEGREES => "FUNC_DEGREES", Self::EXP => "FUNC_EXP", Self::EXP2 => "FUNC_EXP2", Self::FLOOR => "FUNC_FLOOR", Self::FRACT => "FUNC_FRACT", Self::INVERSE_SQRT => "FUNC_INVERSE_SQRT", Self::LOG => "FUNC_LOG", Self::LOG2 => "FUNC_LOG2", Self::RADIANS => "FUNC_RADIANS", Self::ROUND => "FUNC_ROUND", Self::ROUNDEVEN => "FUNC_ROUNDEVEN", Self::SIGN => "FUNC_SIGN", Self::SIN => "FUNC_SIN", Self::SINH => "FUNC_SINH", Self::SQRT => "FUNC_SQRT", Self::TAN => "FUNC_TAN", Self::TANH => "FUNC_TANH", Self::TRUNC => "FUNC_TRUNC", Self::ONEMINUS => "FUNC_ONEMINUS", Self::MAX => "FUNC_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Function {
    const ENUMERATOR_COUNT: usize = 33usize;
    
}
impl crate::meta::GodotConvert for Function {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Function {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Function {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
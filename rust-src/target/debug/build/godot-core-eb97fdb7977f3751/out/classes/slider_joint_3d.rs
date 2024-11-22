#![doc = "Sidecar module for class [`SliderJoint3D`][crate::classes::SliderJoint3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SliderJoint3D` enums](https://docs.godotengine.org/en/stable/classes/class_sliderjoint3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SliderJoint3D.`\n\nInherits [`Joint3D`][crate::classes::Joint3D].\n\nRelated symbols:\n\n* [`slider_joint_3d`][crate::classes::slider_joint_3d]: sidecar module with related enum/flag types\n* [`ISliderJoint3D`][crate::classes::ISliderJoint3D]: virtual methods\n\n\nSee also [Godot docs for `SliderJoint3D`](https://docs.godotengine.org/en/stable/classes/class_sliderjoint3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SliderJoint3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SliderJoint3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SliderJoint3D`][crate::classes::SliderJoint3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SliderJoint3D` methods](https://docs.godotengine.org/en/stable/classes/class_sliderjoint3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISliderJoint3D: crate::obj::GodotClass < Base = SliderJoint3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SliderJoint3D {
        pub fn set_param(&mut self, param: crate::classes::slider_joint_3d::Param, value: f32,) {
            type CallSig = ((), crate::classes::slider_joint_3d::Param, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SliderJoint3D", "set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param(&self, param: crate::classes::slider_joint_3d::Param,) -> f32 {
            type CallSig = (f32, crate::classes::slider_joint_3d::Param);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SliderJoint3D", "get_param", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SliderJoint3D {
        type Base = crate::classes::Joint3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SliderJoint3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SliderJoint3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Joint3D > for SliderJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SliderJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SliderJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SliderJoint3D {
        
    }
    impl crate::obj::cap::GodotDefault for SliderJoint3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SliderJoint3D {
        type Target = crate::classes::Joint3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SliderJoint3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SliderJoint3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SliderJoint3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SliderJoint3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Joint3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Param {
    ord: i32
}
impl Param {
    #[doc(alias = "PARAM_LINEAR_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LIMIT_UPPER`"]
    pub const LINEAR_LIMIT_UPPER: Param = Param {
        ord: 0i32
    };
    #[doc(alias = "PARAM_LINEAR_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LIMIT_LOWER`"]
    pub const LINEAR_LIMIT_LOWER: Param = Param {
        ord: 1i32
    };
    #[doc(alias = "PARAM_LINEAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LIMIT_SOFTNESS`"]
    pub const LINEAR_LIMIT_SOFTNESS: Param = Param {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LINEAR_LIMIT_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LIMIT_RESTITUTION`"]
    pub const LINEAR_LIMIT_RESTITUTION: Param = Param {
        ord: 3i32
    };
    #[doc(alias = "PARAM_LINEAR_LIMIT_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LIMIT_DAMPING`"]
    pub const LINEAR_LIMIT_DAMPING: Param = Param {
        ord: 4i32
    };
    #[doc(alias = "PARAM_LINEAR_MOTION_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_MOTION_SOFTNESS`"]
    pub const LINEAR_MOTION_SOFTNESS: Param = Param {
        ord: 5i32
    };
    #[doc(alias = "PARAM_LINEAR_MOTION_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_MOTION_RESTITUTION`"]
    pub const LINEAR_MOTION_RESTITUTION: Param = Param {
        ord: 6i32
    };
    #[doc(alias = "PARAM_LINEAR_MOTION_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_MOTION_DAMPING`"]
    pub const LINEAR_MOTION_DAMPING: Param = Param {
        ord: 7i32
    };
    #[doc(alias = "PARAM_LINEAR_ORTHOGONAL_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ORTHOGONAL_SOFTNESS`"]
    pub const LINEAR_ORTHOGONAL_SOFTNESS: Param = Param {
        ord: 8i32
    };
    #[doc(alias = "PARAM_LINEAR_ORTHOGONAL_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ORTHOGONAL_RESTITUTION`"]
    pub const LINEAR_ORTHOGONAL_RESTITUTION: Param = Param {
        ord: 9i32
    };
    #[doc(alias = "PARAM_LINEAR_ORTHOGONAL_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ORTHOGONAL_DAMPING`"]
    pub const LINEAR_ORTHOGONAL_DAMPING: Param = Param {
        ord: 10i32
    };
    #[doc(alias = "PARAM_ANGULAR_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LIMIT_UPPER`"]
    pub const ANGULAR_LIMIT_UPPER: Param = Param {
        ord: 11i32
    };
    #[doc(alias = "PARAM_ANGULAR_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LIMIT_LOWER`"]
    pub const ANGULAR_LIMIT_LOWER: Param = Param {
        ord: 12i32
    };
    #[doc(alias = "PARAM_ANGULAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LIMIT_SOFTNESS`"]
    pub const ANGULAR_LIMIT_SOFTNESS: Param = Param {
        ord: 13i32
    };
    #[doc(alias = "PARAM_ANGULAR_LIMIT_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LIMIT_RESTITUTION`"]
    pub const ANGULAR_LIMIT_RESTITUTION: Param = Param {
        ord: 14i32
    };
    #[doc(alias = "PARAM_ANGULAR_LIMIT_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LIMIT_DAMPING`"]
    pub const ANGULAR_LIMIT_DAMPING: Param = Param {
        ord: 15i32
    };
    #[doc(alias = "PARAM_ANGULAR_MOTION_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_MOTION_SOFTNESS`"]
    pub const ANGULAR_MOTION_SOFTNESS: Param = Param {
        ord: 16i32
    };
    #[doc(alias = "PARAM_ANGULAR_MOTION_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_MOTION_RESTITUTION`"]
    pub const ANGULAR_MOTION_RESTITUTION: Param = Param {
        ord: 17i32
    };
    #[doc(alias = "PARAM_ANGULAR_MOTION_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_MOTION_DAMPING`"]
    pub const ANGULAR_MOTION_DAMPING: Param = Param {
        ord: 18i32
    };
    #[doc(alias = "PARAM_ANGULAR_ORTHOGONAL_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_ORTHOGONAL_SOFTNESS`"]
    pub const ANGULAR_ORTHOGONAL_SOFTNESS: Param = Param {
        ord: 19i32
    };
    #[doc(alias = "PARAM_ANGULAR_ORTHOGONAL_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_ORTHOGONAL_RESTITUTION`"]
    pub const ANGULAR_ORTHOGONAL_RESTITUTION: Param = Param {
        ord: 20i32
    };
    #[doc(alias = "PARAM_ANGULAR_ORTHOGONAL_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_ORTHOGONAL_DAMPING`"]
    pub const ANGULAR_ORTHOGONAL_DAMPING: Param = Param {
        ord: 21i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Param = Param {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Param") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::LINEAR_LIMIT_UPPER => "LINEAR_LIMIT_UPPER", Self::LINEAR_LIMIT_LOWER => "LINEAR_LIMIT_LOWER", Self::LINEAR_LIMIT_SOFTNESS => "LINEAR_LIMIT_SOFTNESS", Self::LINEAR_LIMIT_RESTITUTION => "LINEAR_LIMIT_RESTITUTION", Self::LINEAR_LIMIT_DAMPING => "LINEAR_LIMIT_DAMPING", Self::LINEAR_MOTION_SOFTNESS => "LINEAR_MOTION_SOFTNESS", Self::LINEAR_MOTION_RESTITUTION => "LINEAR_MOTION_RESTITUTION", Self::LINEAR_MOTION_DAMPING => "LINEAR_MOTION_DAMPING", Self::LINEAR_ORTHOGONAL_SOFTNESS => "LINEAR_ORTHOGONAL_SOFTNESS", Self::LINEAR_ORTHOGONAL_RESTITUTION => "LINEAR_ORTHOGONAL_RESTITUTION", Self::LINEAR_ORTHOGONAL_DAMPING => "LINEAR_ORTHOGONAL_DAMPING", Self::ANGULAR_LIMIT_UPPER => "ANGULAR_LIMIT_UPPER", Self::ANGULAR_LIMIT_LOWER => "ANGULAR_LIMIT_LOWER", Self::ANGULAR_LIMIT_SOFTNESS => "ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_LIMIT_RESTITUTION => "ANGULAR_LIMIT_RESTITUTION", Self::ANGULAR_LIMIT_DAMPING => "ANGULAR_LIMIT_DAMPING", Self::ANGULAR_MOTION_SOFTNESS => "ANGULAR_MOTION_SOFTNESS", Self::ANGULAR_MOTION_RESTITUTION => "ANGULAR_MOTION_RESTITUTION", Self::ANGULAR_MOTION_DAMPING => "ANGULAR_MOTION_DAMPING", Self::ANGULAR_ORTHOGONAL_SOFTNESS => "ANGULAR_ORTHOGONAL_SOFTNESS", Self::ANGULAR_ORTHOGONAL_RESTITUTION => "ANGULAR_ORTHOGONAL_RESTITUTION", Self::ANGULAR_ORTHOGONAL_DAMPING => "ANGULAR_ORTHOGONAL_DAMPING", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LINEAR_LIMIT_UPPER => "PARAM_LINEAR_LIMIT_UPPER", Self::LINEAR_LIMIT_LOWER => "PARAM_LINEAR_LIMIT_LOWER", Self::LINEAR_LIMIT_SOFTNESS => "PARAM_LINEAR_LIMIT_SOFTNESS", Self::LINEAR_LIMIT_RESTITUTION => "PARAM_LINEAR_LIMIT_RESTITUTION", Self::LINEAR_LIMIT_DAMPING => "PARAM_LINEAR_LIMIT_DAMPING", Self::LINEAR_MOTION_SOFTNESS => "PARAM_LINEAR_MOTION_SOFTNESS", Self::LINEAR_MOTION_RESTITUTION => "PARAM_LINEAR_MOTION_RESTITUTION", Self::LINEAR_MOTION_DAMPING => "PARAM_LINEAR_MOTION_DAMPING", Self::LINEAR_ORTHOGONAL_SOFTNESS => "PARAM_LINEAR_ORTHOGONAL_SOFTNESS", Self::LINEAR_ORTHOGONAL_RESTITUTION => "PARAM_LINEAR_ORTHOGONAL_RESTITUTION", Self::LINEAR_ORTHOGONAL_DAMPING => "PARAM_LINEAR_ORTHOGONAL_DAMPING", Self::ANGULAR_LIMIT_UPPER => "PARAM_ANGULAR_LIMIT_UPPER", Self::ANGULAR_LIMIT_LOWER => "PARAM_ANGULAR_LIMIT_LOWER", Self::ANGULAR_LIMIT_SOFTNESS => "PARAM_ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_LIMIT_RESTITUTION => "PARAM_ANGULAR_LIMIT_RESTITUTION", Self::ANGULAR_LIMIT_DAMPING => "PARAM_ANGULAR_LIMIT_DAMPING", Self::ANGULAR_MOTION_SOFTNESS => "PARAM_ANGULAR_MOTION_SOFTNESS", Self::ANGULAR_MOTION_RESTITUTION => "PARAM_ANGULAR_MOTION_RESTITUTION", Self::ANGULAR_MOTION_DAMPING => "PARAM_ANGULAR_MOTION_DAMPING", Self::ANGULAR_ORTHOGONAL_SOFTNESS => "PARAM_ANGULAR_ORTHOGONAL_SOFTNESS", Self::ANGULAR_ORTHOGONAL_RESTITUTION => "PARAM_ANGULAR_ORTHOGONAL_RESTITUTION", Self::ANGULAR_ORTHOGONAL_DAMPING => "PARAM_ANGULAR_ORTHOGONAL_DAMPING", Self::MAX => "PARAM_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Param {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
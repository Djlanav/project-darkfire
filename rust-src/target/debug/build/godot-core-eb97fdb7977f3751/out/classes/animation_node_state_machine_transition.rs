#![doc = "Sidecar module for class [`AnimationNodeStateMachineTransition`][crate::classes::AnimationNodeStateMachineTransition].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachineTransition.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`animation_node_state_machine_transition`][crate::classes::animation_node_state_machine_transition]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachineTransition`][crate::classes::IAnimationNodeStateMachineTransition]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeStateMachineTransition::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachineTransition {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeStateMachineTransition`][crate::classes::AnimationNodeStateMachineTransition].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachineTransition: crate::obj::GodotClass < Base = AnimationNodeStateMachineTransition > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachineTransition {
        pub fn set_switch_mode(&mut self, mode: crate::classes::animation_node_state_machine_transition::SwitchMode,) {
            type CallSig = ((), crate::classes::animation_node_state_machine_transition::SwitchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_switch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_switch_mode(&self,) -> crate::classes::animation_node_state_machine_transition::SwitchMode {
            type CallSig = (crate::classes::animation_node_state_machine_transition::SwitchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_switch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_advance_mode(&mut self, mode: crate::classes::animation_node_state_machine_transition::AdvanceMode,) {
            type CallSig = ((), crate::classes::animation_node_state_machine_transition::AdvanceMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_advance_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_advance_mode(&self,) -> crate::classes::animation_node_state_machine_transition::AdvanceMode {
            type CallSig = (crate::classes::animation_node_state_machine_transition::AdvanceMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_advance_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_advance_condition(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_advance_condition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_advance_condition(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_advance_condition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_xfade_time(&mut self, secs: f32,) {
            type CallSig = ((), f32);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_xfade_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_xfade_time(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_xfade_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_xfade_curve(&mut self, curve: impl AsObjectArg < crate::classes::Curve >,) {
            type CallSig = ((), ObjectArg < crate::classes::Curve >);
            let args = (curve.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_xfade_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_xfade_curve(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallSig = (Option < Gd < crate::classes::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_xfade_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_break_loop_at_end(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_break_loop_at_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_loop_broken_at_end(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "is_loop_broken_at_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reset(&mut self, reset: bool,) {
            type CallSig = ((), bool);
            let args = (reset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_reset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_reset(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "is_reset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_advance_expression(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_advance_expression", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_advance_expression(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_advance_expression", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachineTransition {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AnimationNodeStateMachineTransition"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachineTransition {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeStateMachineTransition {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeStateMachineTransition {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeStateMachineTransition {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachineTransition {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachineTransition {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachineTransition {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeStateMachineTransition`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AnimationNodeStateMachineTransition {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeStateMachineTransition > for $Class {
                
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
pub struct SwitchMode {
    ord: i32
}
impl SwitchMode {
    #[doc(alias = "SWITCH_MODE_IMMEDIATE")]
    #[doc = "Godot enumerator name: `SWITCH_MODE_IMMEDIATE`"]
    pub const IMMEDIATE: SwitchMode = SwitchMode {
        ord: 0i32
    };
    #[doc(alias = "SWITCH_MODE_SYNC")]
    #[doc = "Godot enumerator name: `SWITCH_MODE_SYNC`"]
    pub const SYNC: SwitchMode = SwitchMode {
        ord: 1i32
    };
    #[doc(alias = "SWITCH_MODE_AT_END")]
    #[doc = "Godot enumerator name: `SWITCH_MODE_AT_END`"]
    pub const AT_END: SwitchMode = SwitchMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SwitchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SwitchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SwitchMode {
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
            Self::IMMEDIATE => "IMMEDIATE", Self::SYNC => "SYNC", Self::AT_END => "AT_END", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::IMMEDIATE => "SWITCH_MODE_IMMEDIATE", Self::SYNC => "SWITCH_MODE_SYNC", Self::AT_END => "SWITCH_MODE_AT_END", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SwitchMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SwitchMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SwitchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AdvanceMode {
    ord: i32
}
impl AdvanceMode {
    #[doc(alias = "ADVANCE_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `ADVANCE_MODE_DISABLED`"]
    pub const DISABLED: AdvanceMode = AdvanceMode {
        ord: 0i32
    };
    #[doc(alias = "ADVANCE_MODE_ENABLED")]
    #[doc = "Godot enumerator name: `ADVANCE_MODE_ENABLED`"]
    pub const ENABLED: AdvanceMode = AdvanceMode {
        ord: 1i32
    };
    #[doc(alias = "ADVANCE_MODE_AUTO")]
    #[doc = "Godot enumerator name: `ADVANCE_MODE_AUTO`"]
    pub const AUTO: AdvanceMode = AdvanceMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AdvanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AdvanceMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AdvanceMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::AUTO => "AUTO", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "ADVANCE_MODE_DISABLED", Self::ENABLED => "ADVANCE_MODE_ENABLED", Self::AUTO => "ADVANCE_MODE_AUTO", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AdvanceMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AdvanceMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AdvanceMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
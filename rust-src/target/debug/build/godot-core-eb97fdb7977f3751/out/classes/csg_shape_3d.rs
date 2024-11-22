#![doc = "Sidecar module for class [`CsgShape3D`][crate::classes::CsgShape3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CSGShape3D` enums](https://docs.godotengine.org/en/stable/classes/class_csgshape3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CSGShape3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`csg_shape_3d`][crate::classes::csg_shape_3d]: sidecar module with related enum/flag types\n* [`ICsgShape3D`][crate::classes::ICsgShape3D]: virtual methods\n\n\nSee also [Godot docs for `CSGShape3D`](https://docs.godotengine.org/en/stable/classes/class_csgshape3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CsgShape3D>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CsgShape3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CsgShape3D`][crate::classes::CsgShape3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CSGShape3D` methods](https://docs.godotengine.org/en/stable/classes/class_csgshape3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICsgShape3D: crate::obj::GodotClass < Base = CsgShape3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CsgShape3D {
        pub fn is_root_shape(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "is_root_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_operation(&mut self, operation: crate::classes::csg_shape_3d::Operation,) {
            type CallSig = ((), crate::classes::csg_shape_3d::Operation);
            let args = (operation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_operation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_operation(&self,) -> crate::classes::csg_shape_3d::Operation {
            type CallSig = (crate::classes::csg_shape_3d::Operation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_operation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap(&mut self, snap: f32,) {
            type CallSig = ((), f32);
            let args = (snap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_snap(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_collision(&mut self, operation: bool,) {
            type CallSig = ((), bool);
            let args = (operation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_use_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_collision(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "is_using_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallSig = ((), u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_layer_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallSig = ((), f32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_calculate_tangents(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_calculate_tangents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_calculating_tangents(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "is_calculating_tangents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meshes(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_meshes", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CsgShape3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CSGShape3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CsgShape3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CsgShape3D {
        
    }
    impl std::ops::Deref for CsgShape3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CsgShape3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CsgShape3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CsgShape3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgShape3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Operation {
    ord: i32
}
impl Operation {
    #[doc(alias = "OPERATION_UNION")]
    #[doc = "Godot enumerator name: `OPERATION_UNION`"]
    pub const UNION: Operation = Operation {
        ord: 0i32
    };
    #[doc(alias = "OPERATION_INTERSECTION")]
    #[doc = "Godot enumerator name: `OPERATION_INTERSECTION`"]
    pub const INTERSECTION: Operation = Operation {
        ord: 1i32
    };
    #[doc(alias = "OPERATION_SUBTRACTION")]
    #[doc = "Godot enumerator name: `OPERATION_SUBTRACTION`"]
    pub const SUBTRACTION: Operation = Operation {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Operation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Operation {
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
            Self::UNION => "UNION", Self::INTERSECTION => "INTERSECTION", Self::SUBTRACTION => "SUBTRACTION", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNION => "OPERATION_UNION", Self::INTERSECTION => "OPERATION_INTERSECTION", Self::SUBTRACTION => "OPERATION_SUBTRACTION", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Operation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Operation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Operation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
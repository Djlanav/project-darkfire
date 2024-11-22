#![doc = "Sidecar module for class [`SkeletonProfile`][crate::classes::SkeletonProfile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonProfile` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkeletonProfile.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`skeleton_profile`][crate::classes::skeleton_profile]: sidecar module with related enum/flag types\n* [`ISkeletonProfile`][crate::classes::ISkeletonProfile]: virtual methods\n\n\nSee also [Godot docs for `SkeletonProfile`](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SkeletonProfile::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkeletonProfile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SkeletonProfile`][crate::classes::SkeletonProfile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SkeletonProfile` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeletonProfile: crate::obj::GodotClass < Base = SkeletonProfile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SkeletonProfile {
        pub fn set_root_bone(&mut self, bone_name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7757usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_bone(&mut self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7758usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale_base_bone(&mut self, bone_name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7759usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_scale_base_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale_base_bone(&mut self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7760usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_scale_base_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7761usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_group_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_size(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7762usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_group_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_name(&self, group_idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (group_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7763usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_group_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group_name(&mut self, group_idx: i32, group_name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (group_idx, group_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7764usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_group_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self, group_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (group_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7765usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, group_idx: i32, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (group_idx, texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7766usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7767usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_size(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7768usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_bone(&self, bone_name: impl AsArg < StringName >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, StringName >);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7769usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "find_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_name(&self, bone_idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7770usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_name(&mut self, bone_idx: i32, bone_name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (bone_idx, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_parent(&self, bone_idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_parent(&mut self, bone_idx: i32, bone_parent: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (bone_idx, bone_parent.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tail_direction(&self, bone_idx: i32,) -> crate::classes::skeleton_profile::TailDirection {
            type CallSig = (crate::classes::skeleton_profile::TailDirection, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_tail_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tail_direction(&mut self, bone_idx: i32, tail_direction: crate::classes::skeleton_profile::TailDirection,) {
            type CallSig = ((), i32, crate::classes::skeleton_profile::TailDirection);
            let args = (bone_idx, tail_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_tail_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone_tail(&self, bone_idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_tail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone_tail(&mut self, bone_idx: i32, bone_tail: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (bone_idx, bone_tail.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_tail", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reference_pose(&self, bone_idx: i32,) -> Transform3D {
            type CallSig = (Transform3D, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7778usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_reference_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reference_pose(&mut self, bone_idx: i32, bone_name: Transform3D,) {
            type CallSig = ((), i32, Transform3D);
            let args = (bone_idx, bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7779usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_reference_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_handle_offset(&self, bone_idx: i32,) -> Vector2 {
            type CallSig = (Vector2, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7780usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_handle_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handle_offset(&mut self, bone_idx: i32, handle_offset: Vector2,) {
            type CallSig = ((), i32, Vector2);
            let args = (bone_idx, handle_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7781usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_handle_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group(&self, bone_idx: i32,) -> StringName {
            type CallSig = (StringName, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_group(&mut self, bone_idx: i32, group: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, StringName >);
            let args = (bone_idx, group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_required(&self, bone_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "is_required", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_required(&mut self, bone_idx: i32, required: bool,) {
            type CallSig = ((), i32, bool);
            let args = (bone_idx, required,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_required", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SkeletonProfile {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SkeletonProfile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkeletonProfile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SkeletonProfile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SkeletonProfile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SkeletonProfile {
        
    }
    impl crate::obj::cap::GodotDefault for SkeletonProfile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SkeletonProfile {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkeletonProfile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SkeletonProfile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SkeletonProfile {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SkeletonProfile > for $Class {
                
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
pub struct TailDirection {
    ord: i32
}
impl TailDirection {
    #[doc(alias = "TAIL_DIRECTION_AVERAGE_CHILDREN")]
    #[doc = "Godot enumerator name: `TAIL_DIRECTION_AVERAGE_CHILDREN`"]
    pub const AVERAGE_CHILDREN: TailDirection = TailDirection {
        ord: 0i32
    };
    #[doc(alias = "TAIL_DIRECTION_SPECIFIC_CHILD")]
    #[doc = "Godot enumerator name: `TAIL_DIRECTION_SPECIFIC_CHILD`"]
    pub const SPECIFIC_CHILD: TailDirection = TailDirection {
        ord: 1i32
    };
    #[doc(alias = "TAIL_DIRECTION_END")]
    #[doc = "Godot enumerator name: `TAIL_DIRECTION_END`"]
    pub const END: TailDirection = TailDirection {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TailDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TailDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TailDirection {
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
            Self::AVERAGE_CHILDREN => "AVERAGE_CHILDREN", Self::SPECIFIC_CHILD => "SPECIFIC_CHILD", Self::END => "END", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::AVERAGE_CHILDREN => "TAIL_DIRECTION_AVERAGE_CHILDREN", Self::SPECIFIC_CHILD => "TAIL_DIRECTION_SPECIFIC_CHILD", Self::END => "TAIL_DIRECTION_END", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TailDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TailDirection {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TailDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
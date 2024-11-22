#![doc = "Sidecar module for class [`SkeletonIk3d`][crate::classes::SkeletonIk3d].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonIK3D` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonik3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkeletonIK3D.`\n\nInherits [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D].\n\nRelated symbols:\n\n* [`skeleton_ik_3d`][crate::classes::skeleton_ik_3d]: sidecar module with related enum/flag types\n* [`ISkeletonIk3d`][crate::classes::ISkeletonIk3d]: virtual methods\n\n\nSee also [Godot docs for `SkeletonIK3D`](https://docs.godotengine.org/en/stable/classes/class_skeletonik3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SkeletonIk3d::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkeletonIk3d {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SkeletonIk3d`][crate::classes::SkeletonIk3d].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SkeletonIK3D` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonik3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeletonIk3d: crate::obj::GodotClass < Base = SkeletonIk3d > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn process_modification(&mut self,) {
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
    impl SkeletonIk3d {
        pub fn set_root_bone(&mut self, root_bone: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (root_bone.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_bone(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_root_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tip_bone(&mut self, tip_bone: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (tip_bone.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_tip_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tip_bone(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_tip_bone", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_transform(&mut self, target: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_target_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_transform(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_target_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_node(&mut self, node: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_target_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_node(&mut self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7737usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_target_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_override_tip_basis(&mut self, override_: bool,) {
            type CallSig = ((), bool);
            let args = (override_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7738usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_override_tip_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_override_tip_basis(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "is_override_tip_basis", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_magnet(&mut self, use_: bool,) {
            type CallSig = ((), bool);
            let args = (use_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_use_magnet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_magnet(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "is_using_magnet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_magnet_position(&mut self, local_position: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_magnet_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_magnet_position(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_magnet_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_skeleton(&self,) -> Option < Gd < crate::classes::Skeleton3D > > {
            type CallSig = (Option < Gd < crate::classes::Skeleton3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_parent_skeleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "is_running", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_distance(&mut self, min_distance: f32,) {
            type CallSig = ((), f32);
            let args = (min_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7746usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_min_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_distance(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7747usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_min_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_iterations(&mut self, iterations: i32,) {
            type CallSig = ((), i32);
            let args = (iterations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7748usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "set_max_iterations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_iterations(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7749usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "get_max_iterations", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn start_full(&mut self, one_time: bool,) {
            type CallSig = ((), bool);
            let args = (one_time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7750usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::start_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn start(&mut self,) {
            self.start_ex() . done()
        }
        #[inline]
        pub fn start_ex < 'a > (&'a mut self,) -> ExStart < 'a > {
            ExStart::new(self,)
        }
        pub fn stop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7751usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "SkeletonIk3d", "stop", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SkeletonIk3d {
        type Base = crate::classes::SkeletonModifier3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"SkeletonIK3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkeletonIk3d {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for SkeletonIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SkeletonIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SkeletonIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SkeletonIk3d {
        
    }
    impl crate::obj::cap::GodotDefault for SkeletonIk3d {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SkeletonIk3d {
        type Target = crate::classes::SkeletonModifier3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkeletonIk3d {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SkeletonIk3d`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_SkeletonIk3d {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SkeletonIk3d > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SkeletonModifier3D > for $Class {
                
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
#[doc = "Default-param extender for [`SkeletonIk3d::start_ex`][super::SkeletonIk3d::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SkeletonIk3d, one_time: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::SkeletonIk3d,) -> Self {
        let one_time = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, one_time: one_time,
        }
    }
    #[inline]
    pub fn one_time(self, one_time: bool) -> Self {
        Self {
            one_time: one_time, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, one_time,
        }
        = self;
        re_export::SkeletonIk3d::start_full(surround_object, one_time,)
    }
}
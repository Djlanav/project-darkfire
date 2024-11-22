#![doc = "Sidecar module for class [`ImporterMeshInstance3D`][crate::classes::ImporterMeshInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImporterMeshInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_importermeshinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImporterMeshInstance3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`IImporterMeshInstance3D`][crate::classes::IImporterMeshInstance3D]: virtual methods\n\n\nSee also [Godot docs for `ImporterMeshInstance3D`](https://docs.godotengine.org/en/stable/classes/class_importermeshinstance3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ImporterMeshInstance3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImporterMeshInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ImporterMeshInstance3D`][crate::classes::ImporterMeshInstance3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ImporterMeshInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_importermeshinstance3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImporterMeshInstance3D: crate::obj::GodotClass < Base = ImporterMeshInstance3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ImporterMeshInstance3D {
        pub fn set_mesh(&mut self, mesh: impl AsObjectArg < crate::classes::ImporterMesh >,) {
            type CallSig = ((), ObjectArg < crate::classes::ImporterMesh >);
            let args = (mesh.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::ImporterMesh > > {
            type CallSig = (Option < Gd < crate::classes::ImporterMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skin(&mut self, skin: impl AsObjectArg < crate::classes::Skin >,) {
            type CallSig = ((), ObjectArg < crate::classes::Skin >);
            let args = (skin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::classes::Skin > > {
            type CallSig = (Option < Gd < crate::classes::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skeleton_path(&mut self, skeleton_path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (skeleton_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_skeleton_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skeleton_path(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_skeleton_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layer_mask(&mut self, layer_mask: u32,) {
            type CallSig = ((), u32);
            let args = (layer_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_layer_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_layer_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cast_shadows_setting(&mut self, shadow_casting_setting: crate::classes::geometry_instance_3d::ShadowCastingSetting,) {
            type CallSig = ((), crate::classes::geometry_instance_3d::ShadowCastingSetting);
            let args = (shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cast_shadows_setting(&self,) -> crate::classes::geometry_instance_3d::ShadowCastingSetting {
            type CallSig = (crate::classes::geometry_instance_3d::ShadowCastingSetting,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_end_margin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_visibility_range_end_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_end_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_visibility_range_end_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_end(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_visibility_range_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_end(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_visibility_range_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_begin_margin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_visibility_range_begin_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_begin_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_visibility_range_begin_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_begin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_visibility_range_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_begin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_visibility_range_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_fade_mode(&mut self, mode: crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,) {
            type CallSig = ((), crate::classes::geometry_instance_3d::VisibilityRangeFadeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "set_visibility_range_fade_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_fade_mode(&self,) -> crate::classes::geometry_instance_3d::VisibilityRangeFadeMode {
            type CallSig = (crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ImporterMeshInstance3D", "get_visibility_range_fade_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ImporterMeshInstance3D {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ImporterMeshInstance3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImporterMeshInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for ImporterMeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ImporterMeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ImporterMeshInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for ImporterMeshInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ImporterMeshInstance3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImporterMeshInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ImporterMeshInstance3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ImporterMeshInstance3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ImporterMeshInstance3D > for $Class {
                
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
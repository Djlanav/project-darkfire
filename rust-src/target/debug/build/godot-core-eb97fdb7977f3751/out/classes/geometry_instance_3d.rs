#![doc = "Sidecar module for class [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GeometryInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GeometryInstance3D.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`geometry_instance_3d`][crate::classes::geometry_instance_3d]: sidecar module with related enum/flag types\n* [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D]: virtual methods\n\n\nSee also [Godot docs for `GeometryInstance3D`](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GeometryInstance3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GeometryInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GeometryInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometryInstance3D: crate::obj::GodotClass < Base = GeometryInstance3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GeometryInstance3D {
        pub fn set_material_override(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material_override(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material_overlay(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_material_overlay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material_overlay(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_material_overlay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_cast_shadows_setting(&mut self, shadow_casting_setting: crate::classes::geometry_instance_3d::ShadowCastingSetting,) {
            type CallSig = ((), crate::classes::geometry_instance_3d::ShadowCastingSetting);
            let args = (shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cast_shadows_setting(&self,) -> crate::classes::geometry_instance_3d::ShadowCastingSetting {
            type CallSig = (crate::classes::geometry_instance_3d::ShadowCastingSetting,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_cast_shadows_setting", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lod_bias(&mut self, bias: f32,) {
            type CallSig = ((), f32);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lod_bias(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lod_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: f32,) {
            type CallSig = ((), f32);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transparency(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_transparency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_end_margin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_end_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_end_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_end_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_end(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_end(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_begin_margin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_begin_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_begin_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_begin_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_begin(&mut self, distance: f32,) {
            type CallSig = ((), f32);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_begin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_range_fade_mode(&mut self, mode: crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,) {
            type CallSig = ((), crate::classes::geometry_instance_3d::VisibilityRangeFadeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_fade_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_range_fade_mode(&self,) -> crate::classes::geometry_instance_3d::VisibilityRangeFadeMode {
            type CallSig = (crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_fade_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_instance_shader_parameter(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3937usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_instance_shader_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_instance_shader_parameter(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_instance_shader_parameter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_extra_cull_margin(&mut self, margin: f32,) {
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_extra_cull_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_extra_cull_margin(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_extra_cull_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lightmap_scale(&mut self, scale: crate::classes::geometry_instance_3d::LightmapScale,) {
            type CallSig = ((), crate::classes::geometry_instance_3d::LightmapScale);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lightmap_scale(&self,) -> crate::classes::geometry_instance_3d::LightmapScale {
            type CallSig = (crate::classes::geometry_instance_3d::LightmapScale,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lightmap_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gi_mode(&mut self, mode: crate::classes::geometry_instance_3d::GiMode,) {
            type CallSig = ((), crate::classes::geometry_instance_3d::GiMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_gi_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gi_mode(&self,) -> crate::classes::geometry_instance_3d::GiMode {
            type CallSig = (crate::classes::geometry_instance_3d::GiMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_gi_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ignore_occlusion_culling(&mut self, ignore_culling: bool,) {
            type CallSig = ((), bool);
            let args = (ignore_culling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_ignore_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ignoring_occlusion_culling(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "is_ignoring_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_custom_aabb", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GeometryInstance3D {
        type Base = crate::classes::VisualInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GeometryInstance3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GeometryInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GeometryInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for GeometryInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GeometryInstance3D {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GeometryInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GeometryInstance3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GeometryInstance3D {
        ($Class: ident) => {
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
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    #[doc(alias = "SHADOW_CASTING_SETTING_OFF")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_OFF`"]
    pub const OFF: ShadowCastingSetting = ShadowCastingSetting {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_ON")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_ON`"]
    pub const ON: ShadowCastingSetting = ShadowCastingSetting {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: ShadowCastingSetting = ShadowCastingSetting {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_SHADOWS_ONLY")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_SHADOWS_ONLY`"]
    pub const SHADOWS_ONLY: ShadowCastingSetting = ShadowCastingSetting {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadowCastingSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowCastingSetting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
            Self::OFF => "OFF", Self::ON => "ON", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::SHADOWS_ONLY => "SHADOWS_ONLY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OFF => "SHADOW_CASTING_SETTING_OFF", Self::ON => "SHADOW_CASTING_SETTING_ON", Self::DOUBLE_SIDED => "SHADOW_CASTING_SETTING_DOUBLE_SIDED", Self::SHADOWS_ONLY => "SHADOW_CASTING_SETTING_SHADOWS_ONLY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowCastingSetting {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `GIMode`."]
pub struct GiMode {
    ord: i32
}
impl GiMode {
    #[doc(alias = "GI_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `GI_MODE_DISABLED`"]
    pub const DISABLED: GiMode = GiMode {
        ord: 0i32
    };
    #[doc(alias = "GI_MODE_STATIC")]
    #[doc = "Godot enumerator name: `GI_MODE_STATIC`"]
    pub const STATIC: GiMode = GiMode {
        ord: 1i32
    };
    #[doc(alias = "GI_MODE_DYNAMIC")]
    #[doc = "Godot enumerator name: `GI_MODE_DYNAMIC`"]
    pub const DYNAMIC: GiMode = GiMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GiMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GiMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "GI_MODE_DISABLED", Self::STATIC => "GI_MODE_STATIC", Self::DYNAMIC => "GI_MODE_DYNAMIC", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for GiMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GiMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GiMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightmapScale {
    ord: i32
}
impl LightmapScale {
    #[doc(alias = "LIGHTMAP_SCALE_1X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_1X`"]
    pub const SCALE_1X: LightmapScale = LightmapScale {
        ord: 0i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_2X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_2X`"]
    pub const SCALE_2X: LightmapScale = LightmapScale {
        ord: 1i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_4X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_4X`"]
    pub const SCALE_4X: LightmapScale = LightmapScale {
        ord: 2i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_8X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_8X`"]
    pub const SCALE_8X: LightmapScale = LightmapScale {
        ord: 3i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_MAX")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_MAX`"]
    pub const MAX: LightmapScale = LightmapScale {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for LightmapScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightmapScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightmapScale {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::SCALE_1X => "SCALE_1X", Self::SCALE_2X => "SCALE_2X", Self::SCALE_4X => "SCALE_4X", Self::SCALE_8X => "SCALE_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SCALE_1X => "LIGHTMAP_SCALE_1X", Self::SCALE_2X => "LIGHTMAP_SCALE_2X", Self::SCALE_4X => "LIGHTMAP_SCALE_4X", Self::SCALE_8X => "LIGHTMAP_SCALE_8X", Self::MAX => "LIGHTMAP_SCALE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for LightmapScale {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for LightmapScale {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightmapScale {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightmapScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    #[doc(alias = "VISIBILITY_RANGE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DISABLED`"]
    pub const DISABLED: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_SELF")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_SELF`"]
    pub const SELF: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_DEPENDENCIES")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DEPENDENCIES`"]
    pub const DEPENDENCIES: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityRangeFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityRangeFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
            Self::DISABLED => "DISABLED", Self::SELF => "SELF", Self::DEPENDENCIES => "DEPENDENCIES", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "VISIBILITY_RANGE_FADE_DISABLED", Self::SELF => "VISIBILITY_RANGE_FADE_SELF", Self::DEPENDENCIES => "VISIBILITY_RANGE_FADE_DEPENDENCIES", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibilityRangeFadeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
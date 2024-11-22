#![doc = "Sidecar module for class [`GpuParticlesCollisionSdf3d`][crate::classes::GpuParticlesCollisionSdf3d].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticlesCollisionSDF3D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionsdf3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GPUParticlesCollisionSDF3D.`\n\nInherits [`GpuParticlesCollision3D`][crate::classes::GpuParticlesCollision3D].\n\nRelated symbols:\n\n* [`gpu_particles_collision_sdf_3d`][crate::classes::gpu_particles_collision_sdf_3d]: sidecar module with related enum/flag types\n* [`IGpuParticlesCollisionSdf3d`][crate::classes::IGpuParticlesCollisionSdf3d]: virtual methods\n\n\nSee also [Godot docs for `GPUParticlesCollisionSDF3D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionsdf3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GpuParticlesCollisionSdf3d::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GpuParticlesCollisionSdf3d {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GpuParticlesCollisionSdf3d`][crate::classes::GpuParticlesCollisionSdf3d].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GPUParticlesCollisionSDF3D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionsdf3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGpuParticlesCollisionSdf3d: crate::obj::GodotClass < Base = GpuParticlesCollisionSdf3d > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GpuParticlesCollisionSdf3d {
        pub fn set_size(&mut self, size: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_resolution(&mut self, resolution: crate::classes::gpu_particles_collision_sdf_3d::Resolution,) {
            type CallSig = ((), crate::classes::gpu_particles_collision_sdf_3d::Resolution);
            let args = (resolution,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "set_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resolution(&self,) -> crate::classes::gpu_particles_collision_sdf_3d::Resolution {
            type CallSig = (crate::classes::gpu_particles_collision_sdf_3d::Resolution,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "get_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture3D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture3D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture3D > > {
            type CallSig = (Option < Gd < crate::classes::Texture3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_thickness(&mut self, thickness: f32,) {
            type CallSig = ((), f32);
            let args = (thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "set_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_thickness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "get_thickness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "set_bake_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "get_bake_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bake_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "set_bake_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bake_mask_value(&self, layer_number: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionSdf3d", "get_bake_mask_value", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GpuParticlesCollisionSdf3d {
        type Base = crate::classes::GpuParticlesCollision3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GPUParticlesCollisionSDF3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GpuParticlesCollisionSdf3d {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GpuParticlesCollision3D > for GpuParticlesCollisionSdf3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GpuParticlesCollisionSdf3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GpuParticlesCollisionSdf3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GpuParticlesCollisionSdf3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GpuParticlesCollisionSdf3d {
        
    }
    impl crate::obj::cap::GodotDefault for GpuParticlesCollisionSdf3d {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GpuParticlesCollisionSdf3d {
        type Target = crate::classes::GpuParticlesCollision3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GpuParticlesCollisionSdf3d {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GpuParticlesCollisionSdf3d`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GpuParticlesCollisionSdf3d {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticlesCollisionSdf3d > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticlesCollision3D > for $Class {
                
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
pub struct Resolution {
    ord: i32
}
impl Resolution {
    pub const RESOLUTION_16: Resolution = Resolution {
        ord: 0i32
    };
    pub const RESOLUTION_32: Resolution = Resolution {
        ord: 1i32
    };
    pub const RESOLUTION_64: Resolution = Resolution {
        ord: 2i32
    };
    pub const RESOLUTION_128: Resolution = Resolution {
        ord: 3i32
    };
    pub const RESOLUTION_256: Resolution = Resolution {
        ord: 4i32
    };
    pub const RESOLUTION_512: Resolution = Resolution {
        ord: 5i32
    };
    #[doc(alias = "RESOLUTION_MAX")]
    #[doc = "Godot enumerator name: `RESOLUTION_MAX`"]
    pub const MAX: Resolution = Resolution {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Resolution") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Resolution {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::RESOLUTION_16 => "RESOLUTION_16", Self::RESOLUTION_32 => "RESOLUTION_32", Self::RESOLUTION_64 => "RESOLUTION_64", Self::RESOLUTION_128 => "RESOLUTION_128", Self::RESOLUTION_256 => "RESOLUTION_256", Self::RESOLUTION_512 => "RESOLUTION_512", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::MAX => "RESOLUTION_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Resolution {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for Resolution {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Resolution {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Resolution {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
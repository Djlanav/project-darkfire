#![doc = "Sidecar module for class [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshInstance3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`mesh_instance_3d`][crate::classes::mesh_instance_3d]: sidecar module with related enum/flag types\n* [`IMeshInstance3D`][crate::classes::IMeshInstance3D]: virtual methods\n\n\nSee also [Godot docs for `MeshInstance3D`](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MeshInstance3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MeshInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshInstance3D: crate::obj::GodotClass < Base = MeshInstance3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshInstance3D {
        pub fn set_mesh(&mut self, mesh: impl AsObjectArg < crate::classes::Mesh >,) {
            type CallSig = ((), ObjectArg < crate::classes::Mesh >);
            let args = (mesh.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::Mesh > > {
            type CallSig = (Option < Gd < crate::classes::Mesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skeleton_path(&mut self, skeleton_path: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (skeleton_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_skeleton_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skeleton_path(&mut self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_skeleton_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skin(&mut self, skin: impl AsObjectArg < crate::classes::Skin >,) {
            type CallSig = ((), ObjectArg < crate::classes::Skin >);
            let args = (skin.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::classes::Skin > > {
            type CallSig = (Option < Gd < crate::classes::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin_reference(&self,) -> Option < Gd < crate::classes::SkinReference > > {
            type CallSig = (Option < Gd < crate::classes::SkinReference > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_skin_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_override_material_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_surface_override_material_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_surface_override_material(&mut self, surface: i32, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Material >);
            let args = (surface, material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_surface_override_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_override_material(&self, surface: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >, i32);
            let args = (surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_surface_override_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_active_material(&self, surface: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >, i32);
            let args = (surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_active_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_trimesh_collision(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_trimesh_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_convex_collision_full(&mut self, clean: bool, simplify: bool,) {
            type CallSig = ((), bool, bool);
            let args = (clean, simplify,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_convex_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_convex_collision_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_convex_collision(&mut self,) {
            self.create_convex_collision_ex() . done()
        }
        #[inline]
        pub fn create_convex_collision_ex < 'a > (&'a mut self,) -> ExCreateConvexCollision < 'a > {
            ExCreateConvexCollision::new(self,)
        }
        pub(crate) fn create_multiple_convex_collisions_full(&mut self, settings: ObjectArg < crate::classes::MeshConvexDecompositionSettings >,) {
            type CallSig = ((), ObjectArg < crate::classes::MeshConvexDecompositionSettings >);
            let args = (settings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_multiple_convex_collisions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_multiple_convex_collisions_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_multiple_convex_collisions(&mut self,) {
            self.create_multiple_convex_collisions_ex() . done()
        }
        #[inline]
        pub fn create_multiple_convex_collisions_ex < 'a > (&'a mut self,) -> ExCreateMultipleConvexCollisions < 'a > {
            ExCreateMultipleConvexCollisions::new(self,)
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_blend_shape_by_name(&mut self, name: impl AsArg < StringName >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "find_blend_shape_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_value(&self, blend_shape_idx: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_blend_shape_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_value(&mut self, blend_shape_idx: i32, value: f32,) {
            type CallSig = ((), i32, f32);
            let args = (blend_shape_idx, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_blend_shape_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_debug_tangents(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_debug_tangents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn bake_mesh_from_current_blend_shape_mix_full(&mut self, existing: ObjectArg < crate::classes::ArrayMesh >,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallSig = (Option < Gd < crate::classes::ArrayMesh > >, ObjectArg < crate::classes::ArrayMesh >);
            let args = (existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MeshInstance3D", "bake_mesh_from_current_blend_shape_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bake_mesh_from_current_blend_shape_mix_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bake_mesh_from_current_blend_shape_mix(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.bake_mesh_from_current_blend_shape_mix_ex() . done()
        }
        #[inline]
        pub fn bake_mesh_from_current_blend_shape_mix_ex < 'a > (&'a mut self,) -> ExBakeMeshFromCurrentBlendShapeMix < 'a > {
            ExBakeMeshFromCurrentBlendShapeMix::new(self,)
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
    impl crate::obj::GodotClass for MeshInstance3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"MeshInstance3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for MeshInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshInstance3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshInstance3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_MeshInstance3D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshInstance3D > for $Class {
                
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
#[doc = "Default-param extender for [`MeshInstance3D::create_convex_collision_ex`][super::MeshInstance3D::create_convex_collision_ex]."]
#[must_use]
pub struct ExCreateConvexCollision < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MeshInstance3D, clean: bool, simplify: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateConvexCollision < 'a > {
    fn new(surround_object: &'a mut re_export::MeshInstance3D,) -> Self {
        let clean = true;
        let simplify = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, clean: clean, simplify: simplify,
        }
    }
    #[inline]
    pub fn clean(self, clean: bool) -> Self {
        Self {
            clean: clean, .. self
        }
    }
    #[inline]
    pub fn simplify(self, simplify: bool) -> Self {
        Self {
            simplify: simplify, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, clean, simplify,
        }
        = self;
        re_export::MeshInstance3D::create_convex_collision_full(surround_object, clean, simplify,)
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::create_multiple_convex_collisions_ex`][super::MeshInstance3D::create_multiple_convex_collisions_ex]."]
#[must_use]
pub struct ExCreateMultipleConvexCollisions < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MeshInstance3D, settings: ObjectCow < crate::classes::MeshConvexDecompositionSettings >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateMultipleConvexCollisions < 'a > {
    fn new(surround_object: &'a mut re_export::MeshInstance3D,) -> Self {
        let settings = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, settings: settings.consume_arg(),
        }
    }
    #[inline]
    pub fn settings(self, settings: impl AsObjectArg < crate::classes::MeshConvexDecompositionSettings >) -> Self {
        Self {
            settings: settings.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, settings,
        }
        = self;
        re_export::MeshInstance3D::create_multiple_convex_collisions_full(surround_object, settings.cow_as_object_arg(),)
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::bake_mesh_from_current_blend_shape_mix_ex`][super::MeshInstance3D::bake_mesh_from_current_blend_shape_mix_ex]."]
#[must_use]
pub struct ExBakeMeshFromCurrentBlendShapeMix < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MeshInstance3D, existing: ObjectCow < crate::classes::ArrayMesh >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBakeMeshFromCurrentBlendShapeMix < 'a > {
    fn new(surround_object: &'a mut re_export::MeshInstance3D,) -> Self {
        let existing = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing: existing.consume_arg(),
        }
    }
    #[inline]
    pub fn existing(self, existing: impl AsObjectArg < crate::classes::ArrayMesh >) -> Self {
        Self {
            existing: existing.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, existing,
        }
        = self;
        re_export::MeshInstance3D::bake_mesh_from_current_blend_shape_mix_full(surround_object, existing.cow_as_object_arg(),)
    }
}
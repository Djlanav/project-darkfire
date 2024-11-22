#![doc = "Sidecar module for class [`GltfPhysicsShape`][crate::classes::GltfPhysicsShape].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFPhysicsShape` enums](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFPhysicsShape.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_physics_shape`][crate::classes::gltf_physics_shape]: sidecar module with related enum/flag types\n* [`IGltfPhysicsShape`][crate::classes::IGltfPhysicsShape]: virtual methods\n\n\nSee also [Godot docs for `GLTFPhysicsShape`](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfPhysicsShape::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfPhysicsShape {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfPhysicsShape`][crate::classes::GltfPhysicsShape].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFPhysicsShape` methods](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfPhysicsShape: crate::obj::GodotClass < Base = GltfPhysicsShape > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfPhysicsShape {
        pub fn from_node(shape_node: impl AsObjectArg < crate::classes::CollisionShape3D >,) -> Option < Gd < crate::classes::GltfPhysicsShape > > {
            type CallSig = (Option < Gd < crate::classes::GltfPhysicsShape > >, ObjectArg < crate::classes::CollisionShape3D >);
            let args = (shape_node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "from_node", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn to_node_full(&mut self, cache_shapes: bool,) -> Option < Gd < crate::classes::CollisionShape3D > > {
            type CallSig = (Option < Gd < crate::classes::CollisionShape3D > >, bool);
            let args = (cache_shapes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "to_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::to_node_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn to_node(&mut self,) -> Option < Gd < crate::classes::CollisionShape3D > > {
            self.to_node_ex() . done()
        }
        #[inline]
        pub fn to_node_ex < 'a > (&'a mut self,) -> ExToNode < 'a > {
            ExToNode::new(self,)
        }
        pub fn from_resource(shape_resource: impl AsObjectArg < crate::classes::Shape3D >,) -> Option < Gd < crate::classes::GltfPhysicsShape > > {
            type CallSig = (Option < Gd < crate::classes::GltfPhysicsShape > >, ObjectArg < crate::classes::Shape3D >);
            let args = (shape_resource.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "from_resource", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn to_resource_full(&mut self, cache_shapes: bool,) -> Option < Gd < crate::classes::Shape3D > > {
            type CallSig = (Option < Gd < crate::classes::Shape3D > >, bool);
            let args = (cache_shapes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "to_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::to_resource_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn to_resource(&mut self,) -> Option < Gd < crate::classes::Shape3D > > {
            self.to_resource_ex() . done()
        }
        #[inline]
        pub fn to_resource_ex < 'a > (&'a mut self,) -> ExToResource < 'a > {
            ExToResource::new(self,)
        }
        pub fn from_dictionary(dictionary: &Dictionary,) -> Option < Gd < crate::classes::GltfPhysicsShape > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::GltfPhysicsShape > >, RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(dictionary),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "from_dictionary", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn to_dictionary(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "to_dictionary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape_type(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_shape_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape_type(&mut self, shape_type: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (shape_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_shape_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type CallSig = ((), Vector3);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radius(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_radius(&mut self, radius: f32,) {
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, height: f32,) {
            type CallSig = ((), f32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_is_trigger(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_is_trigger", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_is_trigger(&mut self, is_trigger: bool,) {
            type CallSig = ((), bool);
            let args = (is_trigger,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_is_trigger", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh_index(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_mesh_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh_index(&mut self, mesh_index: i32,) {
            type CallSig = ((), i32);
            let args = (mesh_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_mesh_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_importer_mesh(&self,) -> Option < Gd < crate::classes::ImporterMesh > > {
            type CallSig = (Option < Gd < crate::classes::ImporterMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_importer_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_importer_mesh(&mut self, importer_mesh: impl AsObjectArg < crate::classes::ImporterMesh >,) {
            type CallSig = ((), ObjectArg < crate::classes::ImporterMesh >);
            let args = (importer_mesh.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_importer_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for GltfPhysicsShape {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GLTFPhysicsShape"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfPhysicsShape {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfPhysicsShape {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfPhysicsShape {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfPhysicsShape {
        
    }
    impl crate::obj::cap::GodotDefault for GltfPhysicsShape {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfPhysicsShape {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfPhysicsShape {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfPhysicsShape`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GltfPhysicsShape {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfPhysicsShape > for $Class {
                
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
#[doc = "Default-param extender for [`GltfPhysicsShape::to_node_ex`][super::GltfPhysicsShape::to_node_ex]."]
#[must_use]
pub struct ExToNode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfPhysicsShape, cache_shapes: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExToNode < 'a > {
    fn new(surround_object: &'a mut re_export::GltfPhysicsShape,) -> Self {
        let cache_shapes = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cache_shapes: cache_shapes,
        }
    }
    #[inline]
    pub fn cache_shapes(self, cache_shapes: bool) -> Self {
        Self {
            cache_shapes: cache_shapes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::CollisionShape3D > > {
        let Self {
            _phantom, surround_object, cache_shapes,
        }
        = self;
        re_export::GltfPhysicsShape::to_node_full(surround_object, cache_shapes,)
    }
}
#[doc = "Default-param extender for [`GltfPhysicsShape::to_resource_ex`][super::GltfPhysicsShape::to_resource_ex]."]
#[must_use]
pub struct ExToResource < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfPhysicsShape, cache_shapes: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExToResource < 'a > {
    fn new(surround_object: &'a mut re_export::GltfPhysicsShape,) -> Self {
        let cache_shapes = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cache_shapes: cache_shapes,
        }
    }
    #[inline]
    pub fn cache_shapes(self, cache_shapes: bool) -> Self {
        Self {
            cache_shapes: cache_shapes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Shape3D > > {
        let Self {
            _phantom, surround_object, cache_shapes,
        }
        = self;
        re_export::GltfPhysicsShape::to_resource_full(surround_object, cache_shapes,)
    }
}
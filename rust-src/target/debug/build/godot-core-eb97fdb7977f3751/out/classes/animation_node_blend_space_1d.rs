#![doc = "Sidecar module for class [`AnimationNodeBlendSpace1D`][crate::classes::AnimationNodeBlendSpace1D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeBlendSpace1D` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace1d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeBlendSpace1D.`\n\nInherits [`AnimationRootNode`][crate::classes::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_blend_space_1d`][crate::classes::animation_node_blend_space_1d]: sidecar module with related enum/flag types\n* [`IAnimationNodeBlendSpace1D`][crate::classes::IAnimationNodeBlendSpace1D]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeBlendSpace1D`](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace1d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeBlendSpace1D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeBlendSpace1D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeBlendSpace1D`][crate::classes::AnimationNodeBlendSpace1D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeBlendSpace1D` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace1d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeBlendSpace1D: crate::obj::GodotClass < Base = AnimationNodeBlendSpace1D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_child_nodes(&self,) -> Dictionary {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> VariantArray {
            unimplemented !()
        }
        fn get_child_by_name(&self, name: StringName,) -> Option < Gd < crate::classes::AnimationNode > > {
            unimplemented !()
        }
        fn get_parameter_default_value(&self, parameter: StringName,) -> Variant {
            unimplemented !()
        }
        fn is_parameter_read_only(&self, parameter: StringName,) -> bool {
            unimplemented !()
        }
        fn process(&self, time: f64, seek: bool, is_external_seeking: bool, test_only: bool,) -> f64 {
            unimplemented !()
        }
        fn get_caption(&self,) -> GString {
            unimplemented !()
        }
        fn has_filter(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AnimationNodeBlendSpace1D {
        pub(crate) fn add_blend_point_full(&mut self, node: ObjectArg < crate::classes::AnimationRootNode >, pos: f32, at_index: i32,) {
            type CallSig = ((), ObjectArg < crate::classes::AnimationRootNode >, f32, i32);
            let args = (node, pos, at_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "add_blend_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_blend_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_blend_point(&mut self, node: impl AsObjectArg < crate::classes::AnimationRootNode >, pos: f32,) {
            self.add_blend_point_ex(node, pos,) . done()
        }
        #[inline]
        pub fn add_blend_point_ex < 'a > (&'a mut self, node: impl AsObjectArg < crate::classes::AnimationRootNode >, pos: f32,) -> ExAddBlendPoint < 'a > {
            ExAddBlendPoint::new(self, node, pos,)
        }
        pub fn set_blend_point_position(&mut self, point: i32, pos: f32,) {
            type CallSig = ((), i32, f32);
            let args = (point, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_blend_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_point_position(&self, point: i32,) -> f32 {
            type CallSig = (f32, i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_blend_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_point_node(&mut self, point: i32, node: impl AsObjectArg < crate::classes::AnimationRootNode >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::AnimationRootNode >);
            let args = (point, node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_blend_point_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_point_node(&self, point: i32,) -> Option < Gd < crate::classes::AnimationRootNode > > {
            type CallSig = (Option < Gd < crate::classes::AnimationRootNode > >, i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_blend_point_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_blend_point(&mut self, point: i32,) {
            type CallSig = ((), i32);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "remove_blend_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_point_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_blend_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_space(&mut self, min_space: f32,) {
            type CallSig = ((), f32);
            let args = (min_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_min_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_space(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_min_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_space(&mut self, max_space: f32,) {
            type CallSig = ((), f32);
            let args = (max_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_max_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_space(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_max_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap(&mut self, snap: f32,) {
            type CallSig = ((), f32);
            let args = (snap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_snap(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_snap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_value_label(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_value_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_value_label(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_value_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_mode(&mut self, mode: crate::classes::animation_node_blend_space_1d::BlendMode,) {
            type CallSig = ((), crate::classes::animation_node_blend_space_1d::BlendMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::animation_node_blend_space_1d::BlendMode {
            type CallSig = (crate::classes::animation_node_blend_space_1d::BlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "get_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_sync(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "set_use_sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_sync(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace1D", "is_using_sync", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNodeBlendSpace1D {
        type Base = crate::classes::AnimationRootNode;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AnimationNodeBlendSpace1D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeBlendSpace1D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationRootNode > for AnimationNodeBlendSpace1D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeBlendSpace1D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeBlendSpace1D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeBlendSpace1D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeBlendSpace1D {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeBlendSpace1D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeBlendSpace1D {
        type Target = crate::classes::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeBlendSpace1D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeBlendSpace1D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AnimationNodeBlendSpace1D {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeBlendSpace1D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationRootNode > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNode > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeBlendSpace1D::add_blend_point_ex`][super::AnimationNodeBlendSpace1D::add_blend_point_ex]."]
#[must_use]
pub struct ExAddBlendPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeBlendSpace1D, node: ObjectCow < crate::classes::AnimationRootNode >, pos: f32, at_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBlendPoint < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeBlendSpace1D, node: impl AsObjectArg < crate::classes::AnimationRootNode >, pos: f32,) -> Self {
        let at_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.consume_arg(), pos: pos, at_index: at_index,
        }
    }
    #[inline]
    pub fn at_index(self, at_index: i32) -> Self {
        Self {
            at_index: at_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, pos, at_index,
        }
        = self;
        re_export::AnimationNodeBlendSpace1D::add_blend_point_full(surround_object, node.cow_as_object_arg(), pos, at_index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    #[doc(alias = "BLEND_MODE_INTERPOLATED")]
    #[doc = "Godot enumerator name: `BLEND_MODE_INTERPOLATED`"]
    pub const INTERPOLATED: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_DISCRETE")]
    #[doc = "Godot enumerator name: `BLEND_MODE_DISCRETE`"]
    pub const DISCRETE: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_DISCRETE_CARRY")]
    #[doc = "Godot enumerator name: `BLEND_MODE_DISCRETE_CARRY`"]
    pub const DISCRETE_CARRY: BlendMode = BlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendMode {
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
            Self::INTERPOLATED => "INTERPOLATED", Self::DISCRETE => "DISCRETE", Self::DISCRETE_CARRY => "DISCRETE_CARRY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INTERPOLATED => "BLEND_MODE_INTERPOLATED", Self::DISCRETE => "BLEND_MODE_DISCRETE", Self::DISCRETE_CARRY => "BLEND_MODE_DISCRETE_CARRY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
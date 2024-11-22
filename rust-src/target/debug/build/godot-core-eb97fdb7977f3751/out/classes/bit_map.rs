#![doc = "Sidecar module for class [`BitMap`][crate::classes::BitMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BitMap` enums](https://docs.godotengine.org/en/stable/classes/class_bitmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BitMap.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`bit_map`][crate::classes::bit_map]: sidecar module with related enum/flag types\n* [`IBitMap`][crate::classes::IBitMap]: virtual methods\n\n\nSee also [Godot docs for `BitMap`](https://docs.godotengine.org/en/stable/classes/class_bitmap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`BitMap::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BitMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`BitMap`][crate::classes::BitMap].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BitMap` methods](https://docs.godotengine.org/en/stable/classes/class_bitmap.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBitMap: crate::obj::GodotClass < Base = BitMap > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl BitMap {
        pub fn create(&mut self, size: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_from_image_alpha_full(&mut self, image: ObjectArg < crate::classes::Image >, threshold: f32,) {
            type CallSig = ((), ObjectArg < crate::classes::Image >, f32);
            let args = (image, threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "create_from_image_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_from_image_alpha_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_from_image_alpha(&mut self, image: impl AsObjectArg < crate::classes::Image >,) {
            self.create_from_image_alpha_ex(image,) . done()
        }
        #[inline]
        pub fn create_from_image_alpha_ex < 'a > (&'a mut self, image: impl AsObjectArg < crate::classes::Image >,) -> ExCreateFromImageAlpha < 'a > {
            ExCreateFromImageAlpha::new(self, image,)
        }
        pub fn set_bitv(&mut self, position: Vector2i, bit: bool,) {
            type CallSig = ((), Vector2i, bool);
            let args = (position, bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "set_bitv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bit(&mut self, x: i32, y: i32, bit: bool,) {
            type CallSig = ((), i32, i32, bool);
            let args = (x, y, bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "set_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bitv(&self, position: Vector2i,) -> bool {
            type CallSig = (bool, Vector2i);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "get_bitv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bit(&self, x: i32, y: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "get_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bit_rect(&mut self, rect: Rect2i, bit: bool,) {
            type CallSig = ((), Rect2i, bool);
            let args = (rect, bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "set_bit_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_true_bit_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "get_true_bit_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn resize(&mut self, new_size: Vector2i,) {
            type CallSig = ((), Vector2i);
            let args = (new_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "resize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grow_mask(&mut self, pixels: i32, rect: Rect2i,) {
            type CallSig = ((), i32, Rect2i);
            let args = (pixels, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "grow_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convert_to_image(&self,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "convert_to_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn opaque_to_polygons_full(&self, rect: Rect2i, epsilon: f32,) -> Array < PackedVector2Array > {
            type CallSig = (Array < PackedVector2Array >, Rect2i, f32);
            let args = (rect, epsilon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "BitMap", "opaque_to_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::opaque_to_polygons_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn opaque_to_polygons(&self, rect: Rect2i,) -> Array < PackedVector2Array > {
            self.opaque_to_polygons_ex(rect,) . done()
        }
        #[inline]
        pub fn opaque_to_polygons_ex < 'a > (&'a self, rect: Rect2i,) -> ExOpaqueToPolygons < 'a > {
            ExOpaqueToPolygons::new(self, rect,)
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
    impl crate::obj::GodotClass for BitMap {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"BitMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BitMap {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for BitMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for BitMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for BitMap {
        
    }
    impl crate::obj::cap::GodotDefault for BitMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for BitMap {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BitMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`BitMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_BitMap {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::BitMap > for $Class {
                
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
#[doc = "Default-param extender for [`BitMap::create_from_image_alpha_ex`][super::BitMap::create_from_image_alpha_ex]."]
#[must_use]
pub struct ExCreateFromImageAlpha < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::BitMap, image: ObjectCow < crate::classes::Image >, threshold: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromImageAlpha < 'a > {
    fn new(surround_object: &'a mut re_export::BitMap, image: impl AsObjectArg < crate::classes::Image >,) -> Self {
        let threshold = 0.1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.consume_arg(), threshold: threshold,
        }
    }
    #[inline]
    pub fn threshold(self, threshold: f32) -> Self {
        Self {
            threshold: threshold, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, threshold,
        }
        = self;
        re_export::BitMap::create_from_image_alpha_full(surround_object, image.cow_as_object_arg(), threshold,)
    }
}
#[doc = "Default-param extender for [`BitMap::opaque_to_polygons_ex`][super::BitMap::opaque_to_polygons_ex]."]
#[must_use]
pub struct ExOpaqueToPolygons < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::BitMap, rect: Rect2i, epsilon: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpaqueToPolygons < 'a > {
    fn new(surround_object: &'a re_export::BitMap, rect: Rect2i,) -> Self {
        let epsilon = 2f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rect: rect, epsilon: epsilon,
        }
    }
    #[inline]
    pub fn epsilon(self, epsilon: f32) -> Self {
        Self {
            epsilon: epsilon, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        let Self {
            _phantom, surround_object, rect, epsilon,
        }
        = self;
        re_export::BitMap::opaque_to_polygons_full(surround_object, rect, epsilon,)
    }
}
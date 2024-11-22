#![doc = "Sidecar module for class [`RegEx`][crate::classes::RegEx].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RegEx` enums](https://docs.godotengine.org/en/stable/classes/class_regex.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RegEx.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`reg_ex`][crate::classes::reg_ex]: sidecar module with related enum/flag types\n* [`IRegEx`][crate::classes::IRegEx]: virtual methods\n\n\nSee also [Godot docs for `RegEx`](https://docs.godotengine.org/en/stable/classes/class_regex.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RegEx::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RegEx {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RegEx`][crate::classes::RegEx].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RegEx` methods](https://docs.godotengine.org/en/stable/classes/class_regex.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRegEx: crate::obj::GodotClass < Base = RegEx > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl RegEx {
        pub fn create_from_string(pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::RegEx > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::RegEx > >, CowArg < 'a0, GString >);
            let args = (pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "create_from_string", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compile(&mut self, pattern: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "compile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn search_full(&self, subject: CowArg < GString >, offset: i32, end: i32,) -> Option < Gd < crate::classes::RegExMatch > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::RegExMatch > >, CowArg < 'a0, GString >, i32, i32);
            let args = (subject, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::search_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn search(&self, subject: impl AsArg < GString >,) -> Option < Gd < crate::classes::RegExMatch > > {
            self.search_ex(subject,) . done()
        }
        #[inline]
        pub fn search_ex < 'a > (&'a self, subject: impl AsArg < GString > + 'a,) -> ExSearch < 'a > {
            ExSearch::new(self, subject,)
        }
        pub(crate) fn search_all_full(&self, subject: CowArg < GString >, offset: i32, end: i32,) -> Array < Gd < crate::classes::RegExMatch > > {
            type CallSig < 'a0, > = (Array < Gd < crate::classes::RegExMatch > >, CowArg < 'a0, GString >, i32, i32);
            let args = (subject, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "search_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::search_all_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn search_all(&self, subject: impl AsArg < GString >,) -> Array < Gd < crate::classes::RegExMatch > > {
            self.search_all_ex(subject,) . done()
        }
        #[inline]
        pub fn search_all_ex < 'a > (&'a self, subject: impl AsArg < GString > + 'a,) -> ExSearchAll < 'a > {
            ExSearchAll::new(self, subject,)
        }
        pub(crate) fn sub_full(&self, subject: CowArg < GString >, replacement: CowArg < GString >, all: bool, offset: i32, end: i32,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool, i32, i32);
            let args = (subject, replacement, all, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "sub", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::sub_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn sub(&self, subject: impl AsArg < GString >, replacement: impl AsArg < GString >,) -> GString {
            self.sub_ex(subject, replacement,) . done()
        }
        #[inline]
        pub fn sub_ex < 'a > (&'a self, subject: impl AsArg < GString > + 'a, replacement: impl AsArg < GString > + 'a,) -> ExSub < 'a > {
            ExSub::new(self, subject, replacement,)
        }
        pub fn is_valid(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6933usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pattern(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6934usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "get_pattern", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_group_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6935usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "get_group_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_names(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6936usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RegEx", "get_names", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RegEx {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RegEx"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RegEx {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RegEx {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RegEx {
        
    }
    impl crate::obj::cap::GodotDefault for RegEx {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RegEx {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RegEx {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RegEx`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RegEx {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RegEx > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RegEx::search_ex`][super::RegEx::search_ex]."]
#[must_use]
pub struct ExSearch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegEx, subject: CowArg < 'a, GString >, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSearch < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: impl AsArg < GString > + 'a,) -> Self {
        let offset = 0i32;
        let end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, subject: subject.into_arg(), offset: offset, end: end,
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn end(self, end: i32) -> Self {
        Self {
            end: end, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::RegExMatch > > {
        let Self {
            _phantom, surround_object, subject, offset, end,
        }
        = self;
        re_export::RegEx::search_full(surround_object, subject, offset, end,)
    }
}
#[doc = "Default-param extender for [`RegEx::search_all_ex`][super::RegEx::search_all_ex]."]
#[must_use]
pub struct ExSearchAll < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegEx, subject: CowArg < 'a, GString >, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSearchAll < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: impl AsArg < GString > + 'a,) -> Self {
        let offset = 0i32;
        let end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, subject: subject.into_arg(), offset: offset, end: end,
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn end(self, end: i32) -> Self {
        Self {
            end: end, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::RegExMatch > > {
        let Self {
            _phantom, surround_object, subject, offset, end,
        }
        = self;
        re_export::RegEx::search_all_full(surround_object, subject, offset, end,)
    }
}
#[doc = "Default-param extender for [`RegEx::sub_ex`][super::RegEx::sub_ex]."]
#[must_use]
pub struct ExSub < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegEx, subject: CowArg < 'a, GString >, replacement: CowArg < 'a, GString >, all: bool, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSub < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: impl AsArg < GString > + 'a, replacement: impl AsArg < GString > + 'a,) -> Self {
        let all = false;
        let offset = 0i32;
        let end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, subject: subject.into_arg(), replacement: replacement.into_arg(), all: all, offset: offset, end: end,
        }
    }
    #[inline]
    pub fn all(self, all: bool) -> Self {
        Self {
            all: all, .. self
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn end(self, end: i32) -> Self {
        Self {
            end: end, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, subject, replacement, all, offset, end,
        }
        = self;
        re_export::RegEx::sub_full(surround_object, subject, replacement, all, offset, end,)
    }
}
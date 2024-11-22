#![doc = "Sidecar module for class [`ZipReader`][crate::classes::ZipReader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ZIPReader` enums](https://docs.godotengine.org/en/stable/classes/class_zipreader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ZIPReader.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`zip_reader`][crate::classes::zip_reader]: sidecar module with related enum/flag types\n* [`IZipReader`][crate::classes::IZipReader]: virtual methods\n\n\nSee also [Godot docs for `ZIPReader`](https://docs.godotengine.org/en/stable/classes/class_zipreader.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ZipReader::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ZipReader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ZipReader`][crate::classes::ZipReader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ZIPReader` methods](https://docs.godotengine.org/en/stable/classes/class_zipreader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IZipReader: crate::obj::GodotClass < Base = ZipReader > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ZipReader {
        pub fn open(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ZipReader", "open", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ZipReader", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_files(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ZipReader", "get_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn read_file_full(&mut self, path: CowArg < GString >, case_sensitive: bool,) -> PackedByteArray {
            type CallSig < 'a0, > = (PackedByteArray, CowArg < 'a0, GString >, bool);
            let args = (path, case_sensitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ZipReader", "read_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::read_file_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn read_file(&mut self, path: impl AsArg < GString >,) -> PackedByteArray {
            self.read_file_ex(path,) . done()
        }
        #[inline]
        pub fn read_file_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExReadFile < 'a > {
            ExReadFile::new(self, path,)
        }
        pub(crate) fn file_exists_full(&mut self, path: CowArg < GString >, case_sensitive: bool,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >, bool);
            let args = (path, case_sensitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ZipReader", "file_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::file_exists_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn file_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            self.file_exists_ex(path,) . done()
        }
        #[inline]
        pub fn file_exists_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExFileExists < 'a > {
            ExFileExists::new(self, path,)
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
    impl crate::obj::GodotClass for ZipReader {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ZIPReader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ZipReader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ZipReader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ZipReader {
        
    }
    impl crate::obj::cap::GodotDefault for ZipReader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ZipReader {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ZipReader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ZipReader`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ZipReader {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ZipReader > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ZipReader::read_file_ex`][super::ZipReader::read_file_ex]."]
#[must_use]
pub struct ExReadFile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ZipReader, path: CowArg < 'a, GString >, case_sensitive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReadFile < 'a > {
    fn new(surround_object: &'a mut re_export::ZipReader, path: impl AsArg < GString > + 'a,) -> Self {
        let case_sensitive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), case_sensitive: case_sensitive,
        }
    }
    #[inline]
    pub fn case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            case_sensitive: case_sensitive, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, path, case_sensitive,
        }
        = self;
        re_export::ZipReader::read_file_full(surround_object, path, case_sensitive,)
    }
}
#[doc = "Default-param extender for [`ZipReader::file_exists_ex`][super::ZipReader::file_exists_ex]."]
#[must_use]
pub struct ExFileExists < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ZipReader, path: CowArg < 'a, GString >, case_sensitive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFileExists < 'a > {
    fn new(surround_object: &'a mut re_export::ZipReader, path: impl AsArg < GString > + 'a,) -> Self {
        let case_sensitive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), case_sensitive: case_sensitive,
        }
    }
    #[inline]
    pub fn case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            case_sensitive: case_sensitive, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, path, case_sensitive,
        }
        = self;
        re_export::ZipReader::file_exists_full(surround_object, path, case_sensitive,)
    }
}
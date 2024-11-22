#![doc = "Sidecar module for class [`ConfigFile`][crate::classes::ConfigFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ConfigFile` enums](https://docs.godotengine.org/en/stable/classes/class_configfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ConfigFile.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`config_file`][crate::classes::config_file]: sidecar module with related enum/flag types\n* [`IConfigFile`][crate::classes::IConfigFile]: virtual methods\n\n\nSee also [Godot docs for `ConfigFile`](https://docs.godotengine.org/en/stable/classes/class_configfile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ConfigFile::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ConfigFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ConfigFile`][crate::classes::ConfigFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ConfigFile` methods](https://docs.godotengine.org/en/stable/classes/class_configfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IConfigFile: crate::obj::GodotClass < Base = ConfigFile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ConfigFile {
        pub fn set_value(&mut self, section: impl AsArg < GString >, key: impl AsArg < GString >, value: &Variant,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >);
            let args = (section.into_arg(), key.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "set_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_value_full(&self, section: CowArg < GString >, key: CowArg < GString >, default: RefArg < Variant >,) -> Variant {
            type CallSig < 'a0, 'a1, 'a2, > = (Variant, CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >);
            let args = (section, key, default,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "get_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_value_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_value(&self, section: impl AsArg < GString >, key: impl AsArg < GString >,) -> Variant {
            self.get_value_ex(section, key,) . done()
        }
        #[inline]
        pub fn get_value_ex < 'a > (&'a self, section: impl AsArg < GString > + 'a, key: impl AsArg < GString > + 'a,) -> ExGetValue < 'a > {
            ExGetValue::new(self, section, key,)
        }
        pub fn has_section(&self, section: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (section.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "has_section", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_section_key(&self, section: impl AsArg < GString >, key: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (section.into_arg(), key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "has_section_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sections(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "get_sections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_section_keys(&self, section: impl AsArg < GString >,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >);
            let args = (section.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "get_section_keys", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_section(&mut self, section: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (section.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "erase_section", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_section_key(&mut self, section: impl AsArg < GString >, key: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (section.into_arg(), key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2276usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "erase_section_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2277usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "load", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parse(&mut self, data: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (data.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2278usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "parse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2279usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn encode_to_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2280usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "encode_to_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_encrypted(&mut self, path: impl AsArg < GString >, key: &PackedByteArray,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, RefArg < 'a1, PackedByteArray >);
            let args = (path.into_arg(), RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2281usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "load_encrypted", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_encrypted_pass(&mut self, path: impl AsArg < GString >, password: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (path.into_arg(), password.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2282usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "load_encrypted_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_encrypted(&mut self, path: impl AsArg < GString >, key: &PackedByteArray,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, RefArg < 'a1, PackedByteArray >);
            let args = (path.into_arg(), RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2283usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "save_encrypted", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_encrypted_pass(&mut self, path: impl AsArg < GString >, password: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (path.into_arg(), password.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2284usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "save_encrypted_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2285usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ConfigFile", "clear", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ConfigFile {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ConfigFile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ConfigFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ConfigFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ConfigFile {
        
    }
    impl crate::obj::cap::GodotDefault for ConfigFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ConfigFile {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ConfigFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ConfigFile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ConfigFile {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConfigFile > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ConfigFile::get_value_ex`][super::ConfigFile::get_value_ex]."]
#[must_use]
pub struct ExGetValue < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ConfigFile, section: CowArg < 'a, GString >, key: CowArg < 'a, GString >, default: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetValue < 'a > {
    fn new(surround_object: &'a re_export::ConfigFile, section: impl AsArg < GString > + 'a, key: impl AsArg < GString > + 'a,) -> Self {
        let default = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, section: section.into_arg(), key: key.into_arg(), default: CowArg::Owned(default),
        }
    }
    #[inline]
    pub fn default(self, default: &'a Variant) -> Self {
        Self {
            default: CowArg::Borrowed(default), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, section, key, default,
        }
        = self;
        re_export::ConfigFile::get_value_full(surround_object, section, key, default.cow_as_arg(),)
    }
}
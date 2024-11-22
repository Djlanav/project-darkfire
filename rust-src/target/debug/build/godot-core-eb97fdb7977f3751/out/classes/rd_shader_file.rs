#![doc = "Sidecar module for class [`RdShaderFile`][crate::classes::RdShaderFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDShaderFile` enums](https://docs.godotengine.org/en/stable/classes/class_rdshaderfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDShaderFile.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`rd_shader_file`][crate::classes::rd_shader_file]: sidecar module with related enum/flag types\n* [`IRdShaderFile`][crate::classes::IRdShaderFile]: virtual methods\n\n\nSee also [Godot docs for `RDShaderFile`](https://docs.godotengine.org/en/stable/classes/class_rdshaderfile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RdShaderFile::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdShaderFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdShaderFile`][crate::classes::RdShaderFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDShaderFile` methods](https://docs.godotengine.org/en/stable/classes/class_rdshaderfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdShaderFile: crate::obj::GodotClass < Base = RdShaderFile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdShaderFile {
        pub(crate) fn set_bytecode_full(&mut self, bytecode: ObjectArg < crate::classes::RdShaderSpirv >, version: CowArg < StringName >,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::RdShaderSpirv >, CowArg < 'a0, StringName >);
            let args = (bytecode, version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdShaderFile", "set_bytecode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_bytecode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_bytecode(&mut self, bytecode: impl AsObjectArg < crate::classes::RdShaderSpirv >,) {
            self.set_bytecode_ex(bytecode,) . done()
        }
        #[inline]
        pub fn set_bytecode_ex < 'a > (&'a mut self, bytecode: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> ExSetBytecode < 'a > {
            ExSetBytecode::new(self, bytecode,)
        }
        pub(crate) fn get_spirv_full(&self, version: CowArg < StringName >,) -> Option < Gd < crate::classes::RdShaderSpirv > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::RdShaderSpirv > >, CowArg < 'a0, StringName >);
            let args = (version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdShaderFile", "get_spirv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_spirv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_spirv(&self,) -> Option < Gd < crate::classes::RdShaderSpirv > > {
            self.get_spirv_ex() . done()
        }
        #[inline]
        pub fn get_spirv_ex < 'a > (&'a self,) -> ExGetSpirv < 'a > {
            ExGetSpirv::new(self,)
        }
        pub fn get_version_list(&self,) -> Array < StringName > {
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdShaderFile", "get_version_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_base_error(&mut self, error: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (error.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdShaderFile", "set_base_error", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_error(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RdShaderFile", "get_base_error", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdShaderFile {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RDShaderFile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdShaderFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for RdShaderFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RdShaderFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RdShaderFile {
        
    }
    impl crate::obj::cap::GodotDefault for RdShaderFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdShaderFile {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdShaderFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RdShaderFile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RdShaderFile {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RdShaderFile > for $Class {
                
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
#[doc = "Default-param extender for [`RdShaderFile::set_bytecode_ex`][super::RdShaderFile::set_bytecode_ex]."]
#[must_use]
pub struct ExSetBytecode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RdShaderFile, bytecode: ObjectCow < crate::classes::RdShaderSpirv >, version: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBytecode < 'a > {
    fn new(surround_object: &'a mut re_export::RdShaderFile, bytecode: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> Self {
        let version = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytecode: bytecode.consume_arg(), version: CowArg::Owned(version),
        }
    }
    #[inline]
    pub fn version(self, version: impl AsArg < StringName > + 'a) -> Self {
        Self {
            version: version.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bytecode, version,
        }
        = self;
        re_export::RdShaderFile::set_bytecode_full(surround_object, bytecode.cow_as_object_arg(), version,)
    }
}
#[doc = "Default-param extender for [`RdShaderFile::get_spirv_ex`][super::RdShaderFile::get_spirv_ex]."]
#[must_use]
pub struct ExGetSpirv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RdShaderFile, version: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSpirv < 'a > {
    fn new(surround_object: &'a re_export::RdShaderFile,) -> Self {
        let version = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, version: CowArg::Owned(version),
        }
    }
    #[inline]
    pub fn version(self, version: impl AsArg < StringName > + 'a) -> Self {
        Self {
            version: version.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::RdShaderSpirv > > {
        let Self {
            _phantom, surround_object, version,
        }
        = self;
        re_export::RdShaderFile::get_spirv_full(surround_object, version,)
    }
}
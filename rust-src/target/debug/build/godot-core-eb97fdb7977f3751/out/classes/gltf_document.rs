#![doc = "Sidecar module for class [`GltfDocument`][crate::classes::GltfDocument].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFDocument` enums](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFDocument.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_document`][crate::classes::gltf_document]: sidecar module with related enum/flag types\n* [`IGltfDocument`][crate::classes::IGltfDocument]: virtual methods\n\n\nSee also [Godot docs for `GLTFDocument`](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfDocument::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfDocument {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`GltfDocument`][crate::classes::GltfDocument].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFDocument` methods](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfDocument: crate::obj::GodotClass < Base = GltfDocument > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfDocument {
        pub fn set_image_format(&mut self, image_format: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (image_format.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "set_image_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_image_format(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "get_image_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lossy_quality(&mut self, lossy_quality: f32,) {
            type CallSig = ((), f32);
            let args = (lossy_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "set_lossy_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lossy_quality(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "get_lossy_quality", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_node_mode(&mut self, root_node_mode: crate::classes::gltf_document::RootNodeMode,) {
            type CallSig = ((), crate::classes::gltf_document::RootNodeMode);
            let args = (root_node_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "set_root_node_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_node_mode(&self,) -> crate::classes::gltf_document::RootNodeMode {
            type CallSig = (crate::classes::gltf_document::RootNodeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "get_root_node_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn append_from_file_full(&mut self, path: CowArg < GString >, state: ObjectArg < crate::classes::GltfState >, flags: u32, base_path: CowArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, ObjectArg < crate::classes::GltfState >, u32, CowArg < 'a1, GString >);
            let args = (path, state, flags, base_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "append_from_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::append_from_file_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn append_from_file(&mut self, path: impl AsArg < GString >, state: impl AsObjectArg < crate::classes::GltfState >,) -> crate::global::Error {
            self.append_from_file_ex(path, state,) . done()
        }
        #[inline]
        pub fn append_from_file_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a, state: impl AsObjectArg < crate::classes::GltfState >,) -> ExAppendFromFile < 'a > {
            ExAppendFromFile::new(self, path, state,)
        }
        pub(crate) fn append_from_buffer_full(&mut self, bytes: RefArg < PackedByteArray >, base_path: CowArg < GString >, state: ObjectArg < crate::classes::GltfState >, flags: u32,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, RefArg < 'a0, PackedByteArray >, CowArg < 'a1, GString >, ObjectArg < crate::classes::GltfState >, u32);
            let args = (bytes, base_path, state, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "append_from_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::append_from_buffer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn append_from_buffer(&mut self, bytes: &PackedByteArray, base_path: impl AsArg < GString >, state: impl AsObjectArg < crate::classes::GltfState >,) -> crate::global::Error {
            self.append_from_buffer_ex(bytes, base_path, state,) . done()
        }
        #[inline]
        pub fn append_from_buffer_ex < 'a > (&'a mut self, bytes: &'a PackedByteArray, base_path: impl AsArg < GString > + 'a, state: impl AsObjectArg < crate::classes::GltfState >,) -> ExAppendFromBuffer < 'a > {
            ExAppendFromBuffer::new(self, bytes, base_path, state,)
        }
        pub(crate) fn append_from_scene_full(&mut self, node: ObjectArg < crate::classes::Node >, state: ObjectArg < crate::classes::GltfState >, flags: u32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, ObjectArg < crate::classes::Node >, ObjectArg < crate::classes::GltfState >, u32);
            let args = (node, state, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "append_from_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::append_from_scene_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn append_from_scene(&mut self, node: impl AsObjectArg < crate::classes::Node >, state: impl AsObjectArg < crate::classes::GltfState >,) -> crate::global::Error {
            self.append_from_scene_ex(node, state,) . done()
        }
        #[inline]
        pub fn append_from_scene_ex < 'a > (&'a mut self, node: impl AsObjectArg < crate::classes::Node >, state: impl AsObjectArg < crate::classes::GltfState >,) -> ExAppendFromScene < 'a > {
            ExAppendFromScene::new(self, node, state,)
        }
        pub(crate) fn generate_scene_full(&mut self, state: ObjectArg < crate::classes::GltfState >, bake_fps: f32, trimming: bool, remove_immutable_tracks: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >, ObjectArg < crate::classes::GltfState >, f32, bool, bool);
            let args = (state, bake_fps, trimming, remove_immutable_tracks,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "generate_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::generate_scene_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn generate_scene(&mut self, state: impl AsObjectArg < crate::classes::GltfState >,) -> Option < Gd < crate::classes::Node > > {
            self.generate_scene_ex(state,) . done()
        }
        #[inline]
        pub fn generate_scene_ex < 'a > (&'a mut self, state: impl AsObjectArg < crate::classes::GltfState >,) -> ExGenerateScene < 'a > {
            ExGenerateScene::new(self, state,)
        }
        pub fn generate_buffer(&mut self, state: impl AsObjectArg < crate::classes::GltfState >,) -> PackedByteArray {
            type CallSig = (PackedByteArray, ObjectArg < crate::classes::GltfState >);
            let args = (state.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "generate_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn write_to_filesystem(&mut self, state: impl AsObjectArg < crate::classes::GltfState >, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, ObjectArg < crate::classes::GltfState >, CowArg < 'a0, GString >);
            let args = (state.as_object_arg(), path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "write_to_filesystem", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn register_gltf_document_extension_full(extension: ObjectArg < crate::classes::GltfDocumentExtension >, first_priority: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::GltfDocumentExtension >, bool);
            let args = (extension, first_priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "register_gltf_document_extension", std::ptr::null_mut(), None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::register_gltf_document_extension_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn register_gltf_document_extension(extension: impl AsObjectArg < crate::classes::GltfDocumentExtension >,) {
            Self::register_gltf_document_extension_ex(extension,) . done()
        }
        #[inline]
        pub fn register_gltf_document_extension_ex < 'a > (extension: impl AsObjectArg < crate::classes::GltfDocumentExtension >,) -> ExRegisterGltfDocumentExtension < 'a > {
            ExRegisterGltfDocumentExtension::new(extension,)
        }
        pub fn unregister_gltf_document_extension(extension: impl AsObjectArg < crate::classes::GltfDocumentExtension >,) {
            type CallSig = ((), ObjectArg < crate::classes::GltfDocumentExtension >);
            let args = (extension.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "GltfDocument", "unregister_gltf_document_extension", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for GltfDocument {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"GLTFDocument"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfDocument {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfDocument {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfDocument {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfDocument {
        
    }
    impl crate::obj::cap::GodotDefault for GltfDocument {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfDocument {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfDocument {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfDocument`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_GltfDocument {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfDocument > for $Class {
                
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
#[doc = "Default-param extender for [`GltfDocument::append_from_file_ex`][super::GltfDocument::append_from_file_ex]."]
#[must_use]
pub struct ExAppendFromFile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfDocument, path: CowArg < 'a, GString >, state: ObjectCow < crate::classes::GltfState >, flags: u32, base_path: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendFromFile < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, path: impl AsArg < GString > + 'a, state: impl AsObjectArg < crate::classes::GltfState >,) -> Self {
        let flags = 0u32;
        let base_path = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), state: state.consume_arg(), flags: flags, base_path: CowArg::Owned(base_path),
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn base_path(self, base_path: impl AsArg < GString > + 'a) -> Self {
        Self {
            base_path: base_path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, state, flags, base_path,
        }
        = self;
        re_export::GltfDocument::append_from_file_full(surround_object, path, state.cow_as_object_arg(), flags, base_path,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::append_from_buffer_ex`][super::GltfDocument::append_from_buffer_ex]."]
#[must_use]
pub struct ExAppendFromBuffer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfDocument, bytes: CowArg < 'a, PackedByteArray >, base_path: CowArg < 'a, GString >, state: ObjectCow < crate::classes::GltfState >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendFromBuffer < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, bytes: &'a PackedByteArray, base_path: impl AsArg < GString > + 'a, state: impl AsObjectArg < crate::classes::GltfState >,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: CowArg::Borrowed(bytes), base_path: base_path.into_arg(), state: state.consume_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, bytes, base_path, state, flags,
        }
        = self;
        re_export::GltfDocument::append_from_buffer_full(surround_object, bytes.cow_as_arg(), base_path, state.cow_as_object_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::append_from_scene_ex`][super::GltfDocument::append_from_scene_ex]."]
#[must_use]
pub struct ExAppendFromScene < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfDocument, node: ObjectCow < crate::classes::Node >, state: ObjectCow < crate::classes::GltfState >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendFromScene < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, node: impl AsObjectArg < crate::classes::Node >, state: impl AsObjectArg < crate::classes::GltfState >,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.consume_arg(), state: state.consume_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, node, state, flags,
        }
        = self;
        re_export::GltfDocument::append_from_scene_full(surround_object, node.cow_as_object_arg(), state.cow_as_object_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::generate_scene_ex`][super::GltfDocument::generate_scene_ex]."]
#[must_use]
pub struct ExGenerateScene < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfDocument, state: ObjectCow < crate::classes::GltfState >, bake_fps: f32, trimming: bool, remove_immutable_tracks: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateScene < 'a > {
    fn new(surround_object: &'a mut re_export::GltfDocument, state: impl AsObjectArg < crate::classes::GltfState >,) -> Self {
        let bake_fps = 30f32;
        let trimming = false;
        let remove_immutable_tracks = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, state: state.consume_arg(), bake_fps: bake_fps, trimming: trimming, remove_immutable_tracks: remove_immutable_tracks,
        }
    }
    #[inline]
    pub fn bake_fps(self, bake_fps: f32) -> Self {
        Self {
            bake_fps: bake_fps, .. self
        }
    }
    #[inline]
    pub fn trimming(self, trimming: bool) -> Self {
        Self {
            trimming: trimming, .. self
        }
    }
    #[inline]
    pub fn remove_immutable_tracks(self, remove_immutable_tracks: bool) -> Self {
        Self {
            remove_immutable_tracks: remove_immutable_tracks, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, state, bake_fps, trimming, remove_immutable_tracks,
        }
        = self;
        re_export::GltfDocument::generate_scene_full(surround_object, state.cow_as_object_arg(), bake_fps, trimming, remove_immutable_tracks,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::register_gltf_document_extension_ex`][super::GltfDocument::register_gltf_document_extension_ex]."]
#[must_use]
pub struct ExRegisterGltfDocumentExtension < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, extension: ObjectCow < crate::classes::GltfDocumentExtension >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRegisterGltfDocumentExtension < 'a > {
    fn new(extension: impl AsObjectArg < crate::classes::GltfDocumentExtension >,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, extension: extension.consume_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, extension, first_priority,
        }
        = self;
        re_export::GltfDocument::register_gltf_document_extension_full(extension.cow_as_object_arg(), first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RootNodeMode {
    ord: i32
}
impl RootNodeMode {
    #[doc(alias = "ROOT_NODE_MODE_SINGLE_ROOT")]
    #[doc = "Godot enumerator name: `ROOT_NODE_MODE_SINGLE_ROOT`"]
    pub const SINGLE_ROOT: RootNodeMode = RootNodeMode {
        ord: 0i32
    };
    #[doc(alias = "ROOT_NODE_MODE_KEEP_ROOT")]
    #[doc = "Godot enumerator name: `ROOT_NODE_MODE_KEEP_ROOT`"]
    pub const KEEP_ROOT: RootNodeMode = RootNodeMode {
        ord: 1i32
    };
    #[doc(alias = "ROOT_NODE_MODE_MULTI_ROOT")]
    #[doc = "Godot enumerator name: `ROOT_NODE_MODE_MULTI_ROOT`"]
    pub const MULTI_ROOT: RootNodeMode = RootNodeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RootNodeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RootNodeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RootNodeMode {
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
            Self::SINGLE_ROOT => "SINGLE_ROOT", Self::KEEP_ROOT => "KEEP_ROOT", Self::MULTI_ROOT => "MULTI_ROOT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SINGLE_ROOT => "ROOT_NODE_MODE_SINGLE_ROOT", Self::KEEP_ROOT => "ROOT_NODE_MODE_KEEP_ROOT", Self::MULTI_ROOT => "ROOT_NODE_MODE_MULTI_ROOT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for RootNodeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RootNodeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RootNodeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
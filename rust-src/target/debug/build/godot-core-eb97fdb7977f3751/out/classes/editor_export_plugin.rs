#![doc = "Sidecar module for class [`EditorExportPlugin`][crate::classes::EditorExportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPlugin.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IEditorExportPlugin`][crate::classes::IEditorExportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorExportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorExportPlugin::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorExportPlugin`][crate::classes::EditorExportPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorExportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorExportPlugin: crate::obj::GodotClass < Base = EditorExportPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn export_file(&mut self, path: GString, type_: GString, features: PackedStringArray,) {
            unimplemented !()
        }
        fn export_begin(&mut self, features: PackedStringArray, is_debug: bool, path: GString, flags: u32,) {
            unimplemented !()
        }
        fn export_end(&mut self,) {
            unimplemented !()
        }
        fn begin_customize_resources(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, features: PackedStringArray,) -> bool {
            unimplemented !()
        }
        fn customize_resource(&mut self, resource: Gd < crate::classes::Resource >, path: GString,) -> Option < Gd < crate::classes::Resource > >;
        fn begin_customize_scenes(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, features: PackedStringArray,) -> bool {
            unimplemented !()
        }
        fn customize_scene(&mut self, scene: Gd < crate::classes::Node >, path: GString,) -> Option < Gd < crate::classes::Node > >;
        fn get_customization_configuration_hash(&self,) -> u64;
        fn end_customize_scenes(&mut self,) {
            unimplemented !()
        }
        fn end_customize_resources(&mut self,) {
            unimplemented !()
        }
        fn get_export_options(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_export_options_overrides(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> Dictionary {
            unimplemented !()
        }
        fn should_update_export_options(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> bool {
            unimplemented !()
        }
        fn get_export_option_warning(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, option: GString,) -> GString {
            unimplemented !()
        }
        fn get_export_features(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_name(&self,) -> GString;
        fn supports_platform(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> bool {
            unimplemented !()
        }
        fn get_android_dependencies(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_android_dependencies_maven_repos(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_android_libraries(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_android_manifest_activity_element_contents(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> GString {
            unimplemented !()
        }
        fn get_android_manifest_application_element_contents(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> GString {
            unimplemented !()
        }
        fn get_android_manifest_element_contents(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> GString {
            unimplemented !()
        }
    }
    impl EditorExportPlugin {
        pub fn add_shared_object(&mut self, path: impl AsArg < GString >, tags: &PackedStringArray, target: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, CowArg < 'a2, GString >);
            let args = (path.into_arg(), RefArg::new(tags), target.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(13usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_shared_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_project_static_lib(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(14usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_project_static_lib", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_file(&mut self, path: impl AsArg < GString >, file: &PackedByteArray, remap: bool,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, PackedByteArray >, bool);
            let args = (path.into_arg(), RefArg::new(file), remap,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(15usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_framework(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(16usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_framework", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_embedded_framework(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(17usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_embedded_framework", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_plist_content(&mut self, plist_content: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (plist_content.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(18usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_plist_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_linker_flags(&mut self, flags: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (flags.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(19usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_linker_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_bundle_file(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(20usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_bundle_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ios_cpp_code(&mut self, code: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (code.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(21usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_cpp_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_macos_plugin_file(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(22usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_macos_plugin_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn skip(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(23usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "skip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_option(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(24usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "get_option", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorExportPlugin {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EditorExportPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorExportPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorExportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorExportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorExportPlugin {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorExportPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EditorExportPlugin {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorExportPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#![doc = "Sidecar module for class [`Object`][crate::classes::Object].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Object` enums](https://docs.godotengine.org/en/stable/classes/class_object.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Object.`\n\nThis is the base class for all other classes at the root of the hierarchy. Every instance of `Object` can be stored in a [`Gd`][crate::obj::Gd] smart pointer.\n\nRelated symbols:\n\n* [`object`][crate::classes::object]: sidecar module with related enum/flag types\n* [`IObject`][crate::classes::IObject]: virtual methods\n* [`ObjectNotification`][crate::classes::notify::ObjectNotification]: notification type\n\n\nSee also [Godot docs for `Object`](https://docs.godotengine.org/en/stable/classes/class_object.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Object::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Object {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Object`][crate::classes::Object].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Object` methods](https://docs.godotengine.org/en/stable/classes/class_object.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IObject: crate::obj::GodotClass < Base = Object > + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Object`][crate::classes::Object]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum ObjectNotification {
        POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for ObjectNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ObjectNotification > for i32 {
        fn from(notification: ObjectNotification) -> i32 {
            match notification {
                ObjectNotification::POSTINITIALIZE => 0i32, ObjectNotification::PREDELETE => 1i32, ObjectNotification::EXTENSION_RELOADED => 2i32, ObjectNotification::Unknown(int) => int,
            }
        }
    }
    impl Object {
        pub fn get_class(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(54usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_class(&self, class: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(55usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "is_class", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(56usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get(&self, property: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(57usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indexed(&mut self, property_path: impl AsArg < NodePath >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, NodePath >, RefArg < 'a1, Variant >);
            let args = (property_path.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(58usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set_indexed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_indexed(&self, property_path: impl AsArg < NodePath >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, NodePath >);
            let args = (property_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(59usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_indexed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_property_list(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(60usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_property_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_method_list(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(61usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_method_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_can_revert(&self, property: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(62usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "property_can_revert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn property_get_revert(&self, property: impl AsArg < StringName >,) -> Variant {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(63usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "property_get_revert", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn notification(&mut self, what: i32, reversed: bool,) {
            type CallSig = ((), i32, bool);
            let args = (what, reversed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(64usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn to_string(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(65usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "to_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_script(&mut self, script: &Variant,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >);
            let args = (RefArg::new(script),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(66usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script(&self,) -> Variant {
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(67usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meta(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(68usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_meta(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(69usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "remove_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_meta_full(&self, name: CowArg < StringName >, default: RefArg < Variant >,) -> Variant {
            type CallSig < 'a0, 'a1, > = (Variant, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (name, default,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(70usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_meta_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_meta(&self, name: impl AsArg < StringName >,) -> Variant {
            self.get_meta_ex(name,) . done()
        }
        #[inline]
        pub fn get_meta_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetMeta < 'a > {
            ExGetMeta::new(self, name,)
        }
        pub fn has_meta(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(71usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "has_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_meta_list(&self,) -> Array < StringName > {
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(72usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_meta_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_user_signal_full(&mut self, signal: CowArg < GString >, arguments: RefArg < VariantArray >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, VariantArray >);
            let args = (signal, arguments,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(73usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "add_user_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_user_signal_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_user_signal(&mut self, signal: impl AsArg < GString >,) {
            self.add_user_signal_ex(signal,) . done()
        }
        #[inline]
        pub fn add_user_signal_ex < 'a > (&'a mut self, signal: impl AsArg < GString > + 'a,) -> ExAddUserSignal < 'a > {
            ExAddUserSignal::new(self, signal,)
        }
        pub fn has_user_signal(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(74usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "has_user_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_user_signal(&mut self, signal: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(75usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "remove_user_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn emit_signal(&mut self, signal: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_emit_signal(self, signal, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_emit_signal(&mut self, signal: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, StringName >);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(76usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Object", "emit_signal", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(77usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Object", "call", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_deferred(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_deferred(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_deferred(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(78usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Object", "call_deferred", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn set_deferred(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(79usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set_deferred", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn callv(&mut self, method: impl AsArg < StringName >, arg_array: &VariantArray,) -> Variant {
            type CallSig < 'a0, 'a1, > = (Variant, CowArg < 'a0, StringName >, RefArg < 'a1, VariantArray >);
            let args = (method.into_arg(), RefArg::new(arg_array),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(80usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "callv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_method(&self, method: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(81usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "has_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_method_argument_count(&self, method: impl AsArg < StringName >,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(82usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_method_argument_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_signal(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(83usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "has_signal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_signal_list(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(84usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_signal_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_signal_connection_list(&self, signal: impl AsArg < StringName >,) -> Array < Dictionary > {
            type CallSig < 'a0, > = (Array < Dictionary >, CowArg < 'a0, StringName >);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(85usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_signal_connection_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_incoming_connections(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(86usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "get_incoming_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_full(&mut self, signal: CowArg < StringName >, callable: RefArg < Callable >, flags: u32,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, StringName >, RefArg < 'a1, Callable >, u32);
            let args = (signal, callable, flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(87usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "connect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect(&mut self, signal: impl AsArg < StringName >, callable: &Callable,) -> crate::global::Error {
            self.connect_ex(signal, callable,) . done()
        }
        #[inline]
        pub fn connect_ex < 'a > (&'a mut self, signal: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> ExConnect < 'a > {
            ExConnect::new(self, signal, callable,)
        }
        pub fn disconnect(&mut self, signal: impl AsArg < StringName >, callable: &Callable,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Callable >);
            let args = (signal.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(88usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "disconnect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_connected(&self, signal: impl AsArg < StringName >, callable: &Callable,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, RefArg < 'a1, Callable >);
            let args = (signal.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(89usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "is_connected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_block_signals(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(90usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set_block_signals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_blocking_signals(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(91usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "is_blocking_signals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_property_list_changed(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(92usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "notify_property_list_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_message_translation(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(93usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "set_message_translation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_translate_messages(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(94usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "can_translate_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn tr_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(95usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "tr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tr_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tr(&self, message: impl AsArg < StringName >,) -> GString {
            self.tr_ex(message,) . done()
        }
        #[inline]
        pub fn tr_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a,) -> ExTr < 'a > {
            ExTr::new(self, message,)
        }
        pub(crate) fn tr_n_full(&self, message: CowArg < StringName >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> GString {
            type CallSig < 'a0, 'a1, 'a2, > = (GString, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(96usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "tr_n", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tr_n_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tr_n(&self, message: impl AsArg < StringName >, plural_message: impl AsArg < StringName >, n: i32,) -> GString {
            self.tr_n_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn tr_n_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExTrN < 'a > {
            ExTrN::new(self, message, plural_message, n,)
        }
        pub fn is_queued_for_deletion(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(97usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "is_queued_for_deletion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn cancel_free(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(98usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Object", "cancel_free", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_POSTINITIALIZE: i32 = 0i32;
        pub(crate) const NOTIFICATION_PREDELETE: i32 = 1i32;
        pub(crate) const NOTIFICATION_EXTENSION_RELOADED: i32 = 2i32;
        
    }
    impl crate::obj::GodotClass for Object {
        type Base = crate::obj::NoBase;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Object"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for Object {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemDynamic;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    impl crate::obj::cap::GodotDefault for Object {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Object`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Object {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Object::get_meta_ex`][super::Object::get_meta_ex]."]
#[must_use]
pub struct ExGetMeta < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Object, name: CowArg < 'a, StringName >, default: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMeta < 'a > {
    fn new(surround_object: &'a re_export::Object, name: impl AsArg < StringName > + 'a,) -> Self {
        let default = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), default: CowArg::Owned(default),
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
            _phantom, surround_object, name, default,
        }
        = self;
        re_export::Object::get_meta_full(surround_object, name, default.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Object::add_user_signal_ex`][super::Object::add_user_signal_ex]."]
#[must_use]
pub struct ExAddUserSignal < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Object, signal: CowArg < 'a, GString >, arguments: CowArg < 'a, VariantArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddUserSignal < 'a > {
    fn new(surround_object: &'a mut re_export::Object, signal: impl AsArg < GString > + 'a,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, signal: signal.into_arg(), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VariantArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, signal, arguments,
        }
        = self;
        re_export::Object::add_user_signal_full(surround_object, signal, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Object::connect_ex`][super::Object::connect_ex]."]
#[must_use]
pub struct ExConnect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Object, signal: CowArg < 'a, StringName >, callable: CowArg < 'a, Callable >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnect < 'a > {
    fn new(surround_object: &'a mut re_export::Object, signal: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, signal: signal.into_arg(), callable: CowArg::Borrowed(callable), flags: flags,
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
            _phantom, surround_object, signal, callable, flags,
        }
        = self;
        re_export::Object::connect_full(surround_object, signal, callable.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`Object::tr_ex`][super::Object::tr_ex]."]
#[must_use]
pub struct ExTr < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Object, message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTr < 'a > {
    fn new(surround_object: &'a re_export::Object, message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::Object::tr_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`Object::tr_n_ex`][super::Object::tr_n_ex]."]
#[must_use]
pub struct ExTrN < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Object, message: CowArg < 'a, StringName >, plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrN < 'a > {
    fn new(surround_object: &'a re_export::Object, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::Object::tr_n_full(surround_object, message, plural_message, n, context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConnectFlags {
    ord: i32
}
impl ConnectFlags {
    #[doc(alias = "CONNECT_DEFERRED")]
    #[doc = "Godot enumerator name: `CONNECT_DEFERRED`"]
    pub const DEFERRED: ConnectFlags = ConnectFlags {
        ord: 1i32
    };
    #[doc(alias = "CONNECT_PERSIST")]
    #[doc = "Godot enumerator name: `CONNECT_PERSIST`"]
    pub const PERSIST: ConnectFlags = ConnectFlags {
        ord: 2i32
    };
    #[doc(alias = "CONNECT_ONE_SHOT")]
    #[doc = "Godot enumerator name: `CONNECT_ONE_SHOT`"]
    pub const ONE_SHOT: ConnectFlags = ConnectFlags {
        ord: 4i32
    };
    #[doc(alias = "CONNECT_REFERENCE_COUNTED")]
    #[doc = "Godot enumerator name: `CONNECT_REFERENCE_COUNTED`"]
    pub const REFERENCE_COUNTED: ConnectFlags = ConnectFlags {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ConnectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConnectFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConnectFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 => Some(Self {
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
            Self::DEFERRED => "DEFERRED", Self::PERSIST => "PERSIST", Self::ONE_SHOT => "ONE_SHOT", Self::REFERENCE_COUNTED => "REFERENCE_COUNTED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFERRED => "CONNECT_DEFERRED", Self::PERSIST => "CONNECT_PERSIST", Self::ONE_SHOT => "CONNECT_ONE_SHOT", Self::REFERENCE_COUNTED => "CONNECT_REFERENCE_COUNTED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ConnectFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ConnectFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
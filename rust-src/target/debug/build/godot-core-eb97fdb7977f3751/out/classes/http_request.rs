#![doc = "Sidecar module for class [`HttpRequest`][crate::classes::HttpRequest].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HTTPRequest` enums](https://docs.godotengine.org/en/stable/classes/class_httprequest.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HTTPRequest.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`http_request`][crate::classes::http_request]: sidecar module with related enum/flag types\n* [`IHttpRequest`][crate::classes::IHttpRequest]: virtual methods\n\n\nSee also [Godot docs for `HTTPRequest`](https://docs.godotengine.org/en/stable/classes/class_httprequest.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`HttpRequest::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HttpRequest {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`HttpRequest`][crate::classes::HttpRequest].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `HTTPRequest` methods](https://docs.godotengine.org/en/stable/classes/class_httprequest.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHttpRequest: crate::obj::GodotClass < Base = HttpRequest > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    impl HttpRequest {
        pub(crate) fn request_full(&mut self, url: CowArg < GString >, custom_headers: RefArg < PackedStringArray >, method: crate::classes::http_client::Method, request_data: CowArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, > = (crate::global::Error, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, crate::classes::http_client::Method, CowArg < 'a2, GString >);
            let args = (url, custom_headers, method, request_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::request_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn request(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.request_ex(url,) . done()
        }
        #[inline]
        pub fn request_ex < 'a > (&'a mut self, url: impl AsArg < GString > + 'a,) -> ExRequest < 'a > {
            ExRequest::new(self, url,)
        }
        pub(crate) fn request_raw_full(&mut self, url: CowArg < GString >, custom_headers: RefArg < PackedStringArray >, method: crate::classes::http_client::Method, request_data_raw: RefArg < PackedByteArray >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, > = (crate::global::Error, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, crate::classes::http_client::Method, RefArg < 'a2, PackedByteArray >);
            let args = (url, custom_headers, method, request_data_raw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "request_raw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::request_raw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn request_raw(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.request_raw_ex(url,) . done()
        }
        #[inline]
        pub fn request_raw_ex < 'a > (&'a mut self, url: impl AsArg < GString > + 'a,) -> ExRequestRaw < 'a > {
            ExRequestRaw::new(self, url,)
        }
        pub fn cancel_request(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "cancel_request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tls_options(&mut self, client_options: impl AsObjectArg < crate::classes::TlsOptions >,) {
            type CallSig = ((), ObjectArg < crate::classes::TlsOptions >);
            let args = (client_options.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_tls_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_http_client_status(&self,) -> crate::classes::http_client::Status {
            type CallSig = (crate::classes::http_client::Status,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_http_client_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_threads(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_use_threads", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_threads(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "is_using_threads", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_accept_gzip(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_accept_gzip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_accepting_gzip(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "is_accepting_gzip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_body_size_limit(&mut self, bytes: i32,) {
            type CallSig = ((), i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_body_size_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_body_size_limit(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_body_size_limit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_redirects(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_max_redirects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_redirects(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_max_redirects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_download_file(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_download_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_download_file(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_download_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_downloaded_bytes(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_downloaded_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_body_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_body_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_timeout(&mut self, timeout: f64,) {
            type CallSig = ((), f64);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_timeout(&mut self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_download_chunk_size(&mut self, chunk_size: i32,) {
            type CallSig = ((), i32);
            let args = (chunk_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_download_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_download_chunk_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "get_download_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_http_proxy(&mut self, host: impl AsArg < GString >, port: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_http_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_https_proxy(&mut self, host: impl AsArg < GString >, port: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpRequest", "set_https_proxy", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for HttpRequest {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"HTTPRequest"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HttpRequest {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for HttpRequest {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for HttpRequest {
        
    }
    impl crate::obj::cap::GodotDefault for HttpRequest {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HttpRequest {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HttpRequest {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`HttpRequest`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_HttpRequest {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::HttpRequest > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`HttpRequest::request_ex`][super::HttpRequest::request_ex]."]
#[must_use]
pub struct ExRequest < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::HttpRequest, url: CowArg < 'a, GString >, custom_headers: CowArg < 'a, PackedStringArray >, method: crate::classes::http_client::Method, request_data: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequest < 'a > {
    fn new(surround_object: &'a mut re_export::HttpRequest, url: impl AsArg < GString > + 'a,) -> Self {
        let custom_headers = PackedStringArray::new();
        let method = crate::obj::EngineEnum::from_ord(0);
        let request_data = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, url: url.into_arg(), custom_headers: CowArg::Owned(custom_headers), method: method, request_data: CowArg::Owned(request_data),
        }
    }
    #[inline]
    pub fn custom_headers(self, custom_headers: &'a PackedStringArray) -> Self {
        Self {
            custom_headers: CowArg::Borrowed(custom_headers), .. self
        }
    }
    #[inline]
    pub fn method(self, method: crate::classes::http_client::Method) -> Self {
        Self {
            method: method, .. self
        }
    }
    #[inline]
    pub fn request_data(self, request_data: impl AsArg < GString > + 'a) -> Self {
        Self {
            request_data: request_data.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, url, custom_headers, method, request_data,
        }
        = self;
        re_export::HttpRequest::request_full(surround_object, url, custom_headers.cow_as_arg(), method, request_data,)
    }
}
#[doc = "Default-param extender for [`HttpRequest::request_raw_ex`][super::HttpRequest::request_raw_ex]."]
#[must_use]
pub struct ExRequestRaw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::HttpRequest, url: CowArg < 'a, GString >, custom_headers: CowArg < 'a, PackedStringArray >, method: crate::classes::http_client::Method, request_data_raw: CowArg < 'a, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequestRaw < 'a > {
    fn new(surround_object: &'a mut re_export::HttpRequest, url: impl AsArg < GString > + 'a,) -> Self {
        let custom_headers = PackedStringArray::new();
        let method = crate::obj::EngineEnum::from_ord(0);
        let request_data_raw = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, url: url.into_arg(), custom_headers: CowArg::Owned(custom_headers), method: method, request_data_raw: CowArg::Owned(request_data_raw),
        }
    }
    #[inline]
    pub fn custom_headers(self, custom_headers: &'a PackedStringArray) -> Self {
        Self {
            custom_headers: CowArg::Borrowed(custom_headers), .. self
        }
    }
    #[inline]
    pub fn method(self, method: crate::classes::http_client::Method) -> Self {
        Self {
            method: method, .. self
        }
    }
    #[inline]
    pub fn request_data_raw(self, request_data_raw: &'a PackedByteArray) -> Self {
        Self {
            request_data_raw: CowArg::Borrowed(request_data_raw), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, url, custom_headers, method, request_data_raw,
        }
        = self;
        re_export::HttpRequest::request_raw_full(surround_object, url, custom_headers.cow_as_arg(), method, request_data_raw.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Result {
    ord: i32
}
impl Result {
    #[doc(alias = "RESULT_SUCCESS")]
    #[doc = "Godot enumerator name: `RESULT_SUCCESS`"]
    pub const SUCCESS: Result = Result {
        ord: 0i32
    };
    #[doc(alias = "RESULT_CHUNKED_BODY_SIZE_MISMATCH")]
    #[doc = "Godot enumerator name: `RESULT_CHUNKED_BODY_SIZE_MISMATCH`"]
    pub const CHUNKED_BODY_SIZE_MISMATCH: Result = Result {
        ord: 1i32
    };
    #[doc(alias = "RESULT_CANT_CONNECT")]
    #[doc = "Godot enumerator name: `RESULT_CANT_CONNECT`"]
    pub const CANT_CONNECT: Result = Result {
        ord: 2i32
    };
    #[doc(alias = "RESULT_CANT_RESOLVE")]
    #[doc = "Godot enumerator name: `RESULT_CANT_RESOLVE`"]
    pub const CANT_RESOLVE: Result = Result {
        ord: 3i32
    };
    #[doc(alias = "RESULT_CONNECTION_ERROR")]
    #[doc = "Godot enumerator name: `RESULT_CONNECTION_ERROR`"]
    pub const CONNECTION_ERROR: Result = Result {
        ord: 4i32
    };
    #[doc(alias = "RESULT_TLS_HANDSHAKE_ERROR")]
    #[doc = "Godot enumerator name: `RESULT_TLS_HANDSHAKE_ERROR`"]
    pub const TLS_HANDSHAKE_ERROR: Result = Result {
        ord: 5i32
    };
    #[doc(alias = "RESULT_NO_RESPONSE")]
    #[doc = "Godot enumerator name: `RESULT_NO_RESPONSE`"]
    pub const NO_RESPONSE: Result = Result {
        ord: 6i32
    };
    #[doc(alias = "RESULT_BODY_SIZE_LIMIT_EXCEEDED")]
    #[doc = "Godot enumerator name: `RESULT_BODY_SIZE_LIMIT_EXCEEDED`"]
    pub const BODY_SIZE_LIMIT_EXCEEDED: Result = Result {
        ord: 7i32
    };
    #[doc(alias = "RESULT_BODY_DECOMPRESS_FAILED")]
    #[doc = "Godot enumerator name: `RESULT_BODY_DECOMPRESS_FAILED`"]
    pub const BODY_DECOMPRESS_FAILED: Result = Result {
        ord: 8i32
    };
    #[doc(alias = "RESULT_REQUEST_FAILED")]
    #[doc = "Godot enumerator name: `RESULT_REQUEST_FAILED`"]
    pub const REQUEST_FAILED: Result = Result {
        ord: 9i32
    };
    #[doc(alias = "RESULT_DOWNLOAD_FILE_CANT_OPEN")]
    #[doc = "Godot enumerator name: `RESULT_DOWNLOAD_FILE_CANT_OPEN`"]
    pub const DOWNLOAD_FILE_CANT_OPEN: Result = Result {
        ord: 10i32
    };
    #[doc(alias = "RESULT_DOWNLOAD_FILE_WRITE_ERROR")]
    #[doc = "Godot enumerator name: `RESULT_DOWNLOAD_FILE_WRITE_ERROR`"]
    pub const DOWNLOAD_FILE_WRITE_ERROR: Result = Result {
        ord: 11i32
    };
    #[doc(alias = "RESULT_REDIRECT_LIMIT_REACHED")]
    #[doc = "Godot enumerator name: `RESULT_REDIRECT_LIMIT_REACHED`"]
    pub const REDIRECT_LIMIT_REACHED: Result = Result {
        ord: 12i32
    };
    #[doc(alias = "RESULT_TIMEOUT")]
    #[doc = "Godot enumerator name: `RESULT_TIMEOUT`"]
    pub const TIMEOUT: Result = Result {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Result") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Result {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::SUCCESS => "SUCCESS", Self::CHUNKED_BODY_SIZE_MISMATCH => "CHUNKED_BODY_SIZE_MISMATCH", Self::CANT_CONNECT => "CANT_CONNECT", Self::CANT_RESOLVE => "CANT_RESOLVE", Self::CONNECTION_ERROR => "CONNECTION_ERROR", Self::TLS_HANDSHAKE_ERROR => "TLS_HANDSHAKE_ERROR", Self::NO_RESPONSE => "NO_RESPONSE", Self::BODY_SIZE_LIMIT_EXCEEDED => "BODY_SIZE_LIMIT_EXCEEDED", Self::BODY_DECOMPRESS_FAILED => "BODY_DECOMPRESS_FAILED", Self::REQUEST_FAILED => "REQUEST_FAILED", Self::DOWNLOAD_FILE_CANT_OPEN => "DOWNLOAD_FILE_CANT_OPEN", Self::DOWNLOAD_FILE_WRITE_ERROR => "DOWNLOAD_FILE_WRITE_ERROR", Self::REDIRECT_LIMIT_REACHED => "REDIRECT_LIMIT_REACHED", Self::TIMEOUT => "TIMEOUT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SUCCESS => "RESULT_SUCCESS", Self::CHUNKED_BODY_SIZE_MISMATCH => "RESULT_CHUNKED_BODY_SIZE_MISMATCH", Self::CANT_CONNECT => "RESULT_CANT_CONNECT", Self::CANT_RESOLVE => "RESULT_CANT_RESOLVE", Self::CONNECTION_ERROR => "RESULT_CONNECTION_ERROR", Self::TLS_HANDSHAKE_ERROR => "RESULT_TLS_HANDSHAKE_ERROR", Self::NO_RESPONSE => "RESULT_NO_RESPONSE", Self::BODY_SIZE_LIMIT_EXCEEDED => "RESULT_BODY_SIZE_LIMIT_EXCEEDED", Self::BODY_DECOMPRESS_FAILED => "RESULT_BODY_DECOMPRESS_FAILED", Self::REQUEST_FAILED => "RESULT_REQUEST_FAILED", Self::DOWNLOAD_FILE_CANT_OPEN => "RESULT_DOWNLOAD_FILE_CANT_OPEN", Self::DOWNLOAD_FILE_WRITE_ERROR => "RESULT_DOWNLOAD_FILE_WRITE_ERROR", Self::REDIRECT_LIMIT_REACHED => "RESULT_REDIRECT_LIMIT_REACHED", Self::TIMEOUT => "RESULT_TIMEOUT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Result {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Result {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Result {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
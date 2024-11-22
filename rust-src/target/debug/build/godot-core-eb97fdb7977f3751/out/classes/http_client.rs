#![doc = "Sidecar module for class [`HttpClient`][crate::classes::HttpClient].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HTTPClient` enums](https://docs.godotengine.org/en/stable/classes/class_httpclient.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HTTPClient.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`http_client`][crate::classes::http_client]: sidecar module with related enum/flag types\n* [`IHttpClient`][crate::classes::IHttpClient]: virtual methods\n\n\nSee also [Godot docs for `HTTPClient`](https://docs.godotengine.org/en/stable/classes/class_httpclient.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`HttpClient::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HttpClient {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`HttpClient`][crate::classes::HttpClient].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `HTTPClient` methods](https://docs.godotengine.org/en/stable/classes/class_httpclient.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHttpClient: crate::obj::GodotClass < Base = HttpClient > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl HttpClient {
        pub(crate) fn connect_to_host_full(&mut self, host: CowArg < GString >, port: i32, tls_options: ObjectArg < crate::classes::TlsOptions >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, i32, ObjectArg < crate::classes::TlsOptions >);
            let args = (host, port, tls_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "connect_to_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_to_host_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect_to_host(&mut self, host: impl AsArg < GString >,) -> crate::global::Error {
            self.connect_to_host_ex(host,) . done()
        }
        #[inline]
        pub fn connect_to_host_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExConnectToHost < 'a > {
            ExConnectToHost::new(self, host,)
        }
        pub fn set_connection(&mut self, connection: impl AsObjectArg < crate::classes::StreamPeer >,) {
            type CallSig = ((), ObjectArg < crate::classes::StreamPeer >);
            let args = (connection.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "set_connection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection(&self,) -> Option < Gd < crate::classes::StreamPeer > > {
            type CallSig = (Option < Gd < crate::classes::StreamPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_connection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_raw(&mut self, method: crate::classes::http_client::Method, url: impl AsArg < GString >, headers: &PackedStringArray, body: &PackedByteArray,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, > = (crate::global::Error, crate::classes::http_client::Method, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, RefArg < 'a2, PackedByteArray >);
            let args = (method, url.into_arg(), RefArg::new(headers), RefArg::new(body),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "request_raw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn request_full(&mut self, method: crate::classes::http_client::Method, url: CowArg < GString >, headers: RefArg < PackedStringArray >, body: CowArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, 'a2, > = (crate::global::Error, crate::classes::http_client::Method, CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, CowArg < 'a2, GString >);
            let args = (method, url, headers, body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "request", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::request_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn request(&mut self, method: crate::classes::http_client::Method, url: impl AsArg < GString >, headers: &PackedStringArray,) -> crate::global::Error {
            self.request_ex(method, url, headers,) . done()
        }
        #[inline]
        pub fn request_ex < 'a > (&'a mut self, method: crate::classes::http_client::Method, url: impl AsArg < GString > + 'a, headers: &'a PackedStringArray,) -> ExRequest < 'a > {
            ExRequest::new(self, method, url, headers,)
        }
        pub fn close(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_response(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "has_response", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_response_chunked(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "is_response_chunked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_code(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_response_code", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_headers(&mut self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_response_headers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_headers_as_dictionary(&mut self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_response_headers_as_dictionary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_response_body_length(&self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_response_body_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn read_response_body_chunk(&mut self,) -> PackedByteArray {
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "read_response_body_chunk", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_read_chunk_size(&mut self, bytes: i32,) {
            type CallSig = ((), i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "set_read_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_read_chunk_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_read_chunk_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blocking_mode(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "set_blocking_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_blocking_mode_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "is_blocking_mode_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_status(&self,) -> crate::classes::http_client::Status {
            type CallSig = (crate::classes::http_client::Status,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "get_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_http_proxy(&mut self, host: impl AsArg < GString >, port: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "set_http_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_https_proxy(&mut self, host: impl AsArg < GString >, port: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "set_https_proxy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn query_string_from_dict(&mut self, fields: &Dictionary,) -> GString {
            type CallSig < 'a0, > = (GString, RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(fields),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "HttpClient", "query_string_from_dict", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for HttpClient {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"HTTPClient"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HttpClient {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for HttpClient {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for HttpClient {
        
    }
    impl crate::obj::cap::GodotDefault for HttpClient {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HttpClient {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HttpClient {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`HttpClient`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_HttpClient {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::HttpClient > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`HttpClient::connect_to_host_ex`][super::HttpClient::connect_to_host_ex]."]
#[must_use]
pub struct ExConnectToHost < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::HttpClient, host: CowArg < 'a, GString >, port: i32, tls_options: ObjectCow < crate::classes::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToHost < 'a > {
    fn new(surround_object: &'a mut re_export::HttpClient, host: impl AsArg < GString > + 'a,) -> Self {
        let port = - 1i32;
        let tls_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), port: port, tls_options: tls_options.consume_arg(),
        }
    }
    #[inline]
    pub fn port(self, port: i32) -> Self {
        Self {
            port: port, .. self
        }
    }
    #[inline]
    pub fn tls_options(self, tls_options: impl AsObjectArg < crate::classes::TlsOptions >) -> Self {
        Self {
            tls_options: tls_options.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, host, port, tls_options,
        }
        = self;
        re_export::HttpClient::connect_to_host_full(surround_object, host, port, tls_options.cow_as_object_arg(),)
    }
}
#[doc = "Default-param extender for [`HttpClient::request_ex`][super::HttpClient::request_ex]."]
#[must_use]
pub struct ExRequest < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::HttpClient, method: crate::classes::http_client::Method, url: CowArg < 'a, GString >, headers: CowArg < 'a, PackedStringArray >, body: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequest < 'a > {
    fn new(surround_object: &'a mut re_export::HttpClient, method: crate::classes::http_client::Method, url: impl AsArg < GString > + 'a, headers: &'a PackedStringArray,) -> Self {
        let body = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, method: method, url: url.into_arg(), headers: CowArg::Borrowed(headers), body: CowArg::Owned(body),
        }
    }
    #[inline]
    pub fn body(self, body: impl AsArg < GString > + 'a) -> Self {
        Self {
            body: body.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, method, url, headers, body,
        }
        = self;
        re_export::HttpClient::request_full(surround_object, method, url, headers.cow_as_arg(), body,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Method {
    ord: i32
}
impl Method {
    #[doc(alias = "METHOD_GET")]
    #[doc = "Godot enumerator name: `METHOD_GET`"]
    pub const GET: Method = Method {
        ord: 0i32
    };
    #[doc(alias = "METHOD_HEAD")]
    #[doc = "Godot enumerator name: `METHOD_HEAD`"]
    pub const HEAD: Method = Method {
        ord: 1i32
    };
    #[doc(alias = "METHOD_POST")]
    #[doc = "Godot enumerator name: `METHOD_POST`"]
    pub const POST: Method = Method {
        ord: 2i32
    };
    #[doc(alias = "METHOD_PUT")]
    #[doc = "Godot enumerator name: `METHOD_PUT`"]
    pub const PUT: Method = Method {
        ord: 3i32
    };
    #[doc(alias = "METHOD_DELETE")]
    #[doc = "Godot enumerator name: `METHOD_DELETE`"]
    pub const DELETE: Method = Method {
        ord: 4i32
    };
    #[doc(alias = "METHOD_OPTIONS")]
    #[doc = "Godot enumerator name: `METHOD_OPTIONS`"]
    pub const OPTIONS: Method = Method {
        ord: 5i32
    };
    #[doc(alias = "METHOD_TRACE")]
    #[doc = "Godot enumerator name: `METHOD_TRACE`"]
    pub const TRACE: Method = Method {
        ord: 6i32
    };
    #[doc(alias = "METHOD_CONNECT")]
    #[doc = "Godot enumerator name: `METHOD_CONNECT`"]
    pub const CONNECT: Method = Method {
        ord: 7i32
    };
    #[doc(alias = "METHOD_PATCH")]
    #[doc = "Godot enumerator name: `METHOD_PATCH`"]
    pub const PATCH: Method = Method {
        ord: 8i32
    };
    #[doc(alias = "METHOD_MAX")]
    #[doc = "Godot enumerator name: `METHOD_MAX`"]
    pub const MAX: Method = Method {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Method") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Method {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::GET => "GET", Self::HEAD => "HEAD", Self::POST => "POST", Self::PUT => "PUT", Self::DELETE => "DELETE", Self::OPTIONS => "OPTIONS", Self::TRACE => "TRACE", Self::CONNECT => "CONNECT", Self::PATCH => "PATCH", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GET => "METHOD_GET", Self::HEAD => "METHOD_HEAD", Self::POST => "METHOD_POST", Self::PUT => "METHOD_PUT", Self::DELETE => "METHOD_DELETE", Self::OPTIONS => "METHOD_OPTIONS", Self::TRACE => "METHOD_TRACE", Self::CONNECT => "METHOD_CONNECT", Self::PATCH => "METHOD_PATCH", Self::MAX => "METHOD_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Method {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::meta::GodotConvert for Method {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Method {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Method {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Status {
    ord: i32
}
impl Status {
    #[doc(alias = "STATUS_DISCONNECTED")]
    #[doc = "Godot enumerator name: `STATUS_DISCONNECTED`"]
    pub const DISCONNECTED: Status = Status {
        ord: 0i32
    };
    #[doc(alias = "STATUS_RESOLVING")]
    #[doc = "Godot enumerator name: `STATUS_RESOLVING`"]
    pub const RESOLVING: Status = Status {
        ord: 1i32
    };
    #[doc(alias = "STATUS_CANT_RESOLVE")]
    #[doc = "Godot enumerator name: `STATUS_CANT_RESOLVE`"]
    pub const CANT_RESOLVE: Status = Status {
        ord: 2i32
    };
    #[doc(alias = "STATUS_CONNECTING")]
    #[doc = "Godot enumerator name: `STATUS_CONNECTING`"]
    pub const CONNECTING: Status = Status {
        ord: 3i32
    };
    #[doc(alias = "STATUS_CANT_CONNECT")]
    #[doc = "Godot enumerator name: `STATUS_CANT_CONNECT`"]
    pub const CANT_CONNECT: Status = Status {
        ord: 4i32
    };
    #[doc(alias = "STATUS_CONNECTED")]
    #[doc = "Godot enumerator name: `STATUS_CONNECTED`"]
    pub const CONNECTED: Status = Status {
        ord: 5i32
    };
    #[doc(alias = "STATUS_REQUESTING")]
    #[doc = "Godot enumerator name: `STATUS_REQUESTING`"]
    pub const REQUESTING: Status = Status {
        ord: 6i32
    };
    #[doc(alias = "STATUS_BODY")]
    #[doc = "Godot enumerator name: `STATUS_BODY`"]
    pub const BODY: Status = Status {
        ord: 7i32
    };
    #[doc(alias = "STATUS_CONNECTION_ERROR")]
    #[doc = "Godot enumerator name: `STATUS_CONNECTION_ERROR`"]
    pub const CONNECTION_ERROR: Status = Status {
        ord: 8i32
    };
    #[doc(alias = "STATUS_TLS_HANDSHAKE_ERROR")]
    #[doc = "Godot enumerator name: `STATUS_TLS_HANDSHAKE_ERROR`"]
    pub const TLS_HANDSHAKE_ERROR: Status = Status {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Status") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Status {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::RESOLVING => "RESOLVING", Self::CANT_RESOLVE => "CANT_RESOLVE", Self::CONNECTING => "CONNECTING", Self::CANT_CONNECT => "CANT_CONNECT", Self::CONNECTED => "CONNECTED", Self::REQUESTING => "REQUESTING", Self::BODY => "BODY", Self::CONNECTION_ERROR => "CONNECTION_ERROR", Self::TLS_HANDSHAKE_ERROR => "TLS_HANDSHAKE_ERROR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISCONNECTED => "STATUS_DISCONNECTED", Self::RESOLVING => "STATUS_RESOLVING", Self::CANT_RESOLVE => "STATUS_CANT_RESOLVE", Self::CONNECTING => "STATUS_CONNECTING", Self::CANT_CONNECT => "STATUS_CANT_CONNECT", Self::CONNECTED => "STATUS_CONNECTED", Self::REQUESTING => "STATUS_REQUESTING", Self::BODY => "STATUS_BODY", Self::CONNECTION_ERROR => "STATUS_CONNECTION_ERROR", Self::TLS_HANDSHAKE_ERROR => "STATUS_TLS_HANDSHAKE_ERROR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Status {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Status {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Status {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ResponseCode {
    ord: i32
}
impl ResponseCode {
    #[doc(alias = "RESPONSE_CONTINUE")]
    #[doc = "Godot enumerator name: `RESPONSE_CONTINUE`"]
    pub const CONTINUE: ResponseCode = ResponseCode {
        ord: 100i32
    };
    #[doc(alias = "RESPONSE_SWITCHING_PROTOCOLS")]
    #[doc = "Godot enumerator name: `RESPONSE_SWITCHING_PROTOCOLS`"]
    pub const SWITCHING_PROTOCOLS: ResponseCode = ResponseCode {
        ord: 101i32
    };
    #[doc(alias = "RESPONSE_PROCESSING")]
    #[doc = "Godot enumerator name: `RESPONSE_PROCESSING`"]
    pub const PROCESSING: ResponseCode = ResponseCode {
        ord: 102i32
    };
    #[doc(alias = "RESPONSE_OK")]
    #[doc = "Godot enumerator name: `RESPONSE_OK`"]
    pub const OK: ResponseCode = ResponseCode {
        ord: 200i32
    };
    #[doc(alias = "RESPONSE_CREATED")]
    #[doc = "Godot enumerator name: `RESPONSE_CREATED`"]
    pub const CREATED: ResponseCode = ResponseCode {
        ord: 201i32
    };
    #[doc(alias = "RESPONSE_ACCEPTED")]
    #[doc = "Godot enumerator name: `RESPONSE_ACCEPTED`"]
    pub const ACCEPTED: ResponseCode = ResponseCode {
        ord: 202i32
    };
    #[doc(alias = "RESPONSE_NON_AUTHORITATIVE_INFORMATION")]
    #[doc = "Godot enumerator name: `RESPONSE_NON_AUTHORITATIVE_INFORMATION`"]
    pub const NON_AUTHORITATIVE_INFORMATION: ResponseCode = ResponseCode {
        ord: 203i32
    };
    #[doc(alias = "RESPONSE_NO_CONTENT")]
    #[doc = "Godot enumerator name: `RESPONSE_NO_CONTENT`"]
    pub const NO_CONTENT: ResponseCode = ResponseCode {
        ord: 204i32
    };
    #[doc(alias = "RESPONSE_RESET_CONTENT")]
    #[doc = "Godot enumerator name: `RESPONSE_RESET_CONTENT`"]
    pub const RESET_CONTENT: ResponseCode = ResponseCode {
        ord: 205i32
    };
    #[doc(alias = "RESPONSE_PARTIAL_CONTENT")]
    #[doc = "Godot enumerator name: `RESPONSE_PARTIAL_CONTENT`"]
    pub const PARTIAL_CONTENT: ResponseCode = ResponseCode {
        ord: 206i32
    };
    #[doc(alias = "RESPONSE_MULTI_STATUS")]
    #[doc = "Godot enumerator name: `RESPONSE_MULTI_STATUS`"]
    pub const MULTI_STATUS: ResponseCode = ResponseCode {
        ord: 207i32
    };
    #[doc(alias = "RESPONSE_ALREADY_REPORTED")]
    #[doc = "Godot enumerator name: `RESPONSE_ALREADY_REPORTED`"]
    pub const ALREADY_REPORTED: ResponseCode = ResponseCode {
        ord: 208i32
    };
    #[doc(alias = "RESPONSE_IM_USED")]
    #[doc = "Godot enumerator name: `RESPONSE_IM_USED`"]
    pub const IM_USED: ResponseCode = ResponseCode {
        ord: 226i32
    };
    #[doc(alias = "RESPONSE_MULTIPLE_CHOICES")]
    #[doc = "Godot enumerator name: `RESPONSE_MULTIPLE_CHOICES`"]
    pub const MULTIPLE_CHOICES: ResponseCode = ResponseCode {
        ord: 300i32
    };
    #[doc(alias = "RESPONSE_MOVED_PERMANENTLY")]
    #[doc = "Godot enumerator name: `RESPONSE_MOVED_PERMANENTLY`"]
    pub const MOVED_PERMANENTLY: ResponseCode = ResponseCode {
        ord: 301i32
    };
    #[doc(alias = "RESPONSE_FOUND")]
    #[doc = "Godot enumerator name: `RESPONSE_FOUND`"]
    pub const FOUND: ResponseCode = ResponseCode {
        ord: 302i32
    };
    #[doc(alias = "RESPONSE_SEE_OTHER")]
    #[doc = "Godot enumerator name: `RESPONSE_SEE_OTHER`"]
    pub const SEE_OTHER: ResponseCode = ResponseCode {
        ord: 303i32
    };
    #[doc(alias = "RESPONSE_NOT_MODIFIED")]
    #[doc = "Godot enumerator name: `RESPONSE_NOT_MODIFIED`"]
    pub const NOT_MODIFIED: ResponseCode = ResponseCode {
        ord: 304i32
    };
    #[doc(alias = "RESPONSE_USE_PROXY")]
    #[doc = "Godot enumerator name: `RESPONSE_USE_PROXY`"]
    pub const USE_PROXY: ResponseCode = ResponseCode {
        ord: 305i32
    };
    #[doc(alias = "RESPONSE_SWITCH_PROXY")]
    #[doc = "Godot enumerator name: `RESPONSE_SWITCH_PROXY`"]
    pub const SWITCH_PROXY: ResponseCode = ResponseCode {
        ord: 306i32
    };
    #[doc(alias = "RESPONSE_TEMPORARY_REDIRECT")]
    #[doc = "Godot enumerator name: `RESPONSE_TEMPORARY_REDIRECT`"]
    pub const TEMPORARY_REDIRECT: ResponseCode = ResponseCode {
        ord: 307i32
    };
    #[doc(alias = "RESPONSE_PERMANENT_REDIRECT")]
    #[doc = "Godot enumerator name: `RESPONSE_PERMANENT_REDIRECT`"]
    pub const PERMANENT_REDIRECT: ResponseCode = ResponseCode {
        ord: 308i32
    };
    #[doc(alias = "RESPONSE_BAD_REQUEST")]
    #[doc = "Godot enumerator name: `RESPONSE_BAD_REQUEST`"]
    pub const BAD_REQUEST: ResponseCode = ResponseCode {
        ord: 400i32
    };
    #[doc(alias = "RESPONSE_UNAUTHORIZED")]
    #[doc = "Godot enumerator name: `RESPONSE_UNAUTHORIZED`"]
    pub const UNAUTHORIZED: ResponseCode = ResponseCode {
        ord: 401i32
    };
    #[doc(alias = "RESPONSE_PAYMENT_REQUIRED")]
    #[doc = "Godot enumerator name: `RESPONSE_PAYMENT_REQUIRED`"]
    pub const PAYMENT_REQUIRED: ResponseCode = ResponseCode {
        ord: 402i32
    };
    #[doc(alias = "RESPONSE_FORBIDDEN")]
    #[doc = "Godot enumerator name: `RESPONSE_FORBIDDEN`"]
    pub const FORBIDDEN: ResponseCode = ResponseCode {
        ord: 403i32
    };
    #[doc(alias = "RESPONSE_NOT_FOUND")]
    #[doc = "Godot enumerator name: `RESPONSE_NOT_FOUND`"]
    pub const NOT_FOUND: ResponseCode = ResponseCode {
        ord: 404i32
    };
    #[doc(alias = "RESPONSE_METHOD_NOT_ALLOWED")]
    #[doc = "Godot enumerator name: `RESPONSE_METHOD_NOT_ALLOWED`"]
    pub const METHOD_NOT_ALLOWED: ResponseCode = ResponseCode {
        ord: 405i32
    };
    #[doc(alias = "RESPONSE_NOT_ACCEPTABLE")]
    #[doc = "Godot enumerator name: `RESPONSE_NOT_ACCEPTABLE`"]
    pub const NOT_ACCEPTABLE: ResponseCode = ResponseCode {
        ord: 406i32
    };
    #[doc(alias = "RESPONSE_PROXY_AUTHENTICATION_REQUIRED")]
    #[doc = "Godot enumerator name: `RESPONSE_PROXY_AUTHENTICATION_REQUIRED`"]
    pub const PROXY_AUTHENTICATION_REQUIRED: ResponseCode = ResponseCode {
        ord: 407i32
    };
    #[doc(alias = "RESPONSE_REQUEST_TIMEOUT")]
    #[doc = "Godot enumerator name: `RESPONSE_REQUEST_TIMEOUT`"]
    pub const REQUEST_TIMEOUT: ResponseCode = ResponseCode {
        ord: 408i32
    };
    #[doc(alias = "RESPONSE_CONFLICT")]
    #[doc = "Godot enumerator name: `RESPONSE_CONFLICT`"]
    pub const CONFLICT: ResponseCode = ResponseCode {
        ord: 409i32
    };
    #[doc(alias = "RESPONSE_GONE")]
    #[doc = "Godot enumerator name: `RESPONSE_GONE`"]
    pub const GONE: ResponseCode = ResponseCode {
        ord: 410i32
    };
    #[doc(alias = "RESPONSE_LENGTH_REQUIRED")]
    #[doc = "Godot enumerator name: `RESPONSE_LENGTH_REQUIRED`"]
    pub const LENGTH_REQUIRED: ResponseCode = ResponseCode {
        ord: 411i32
    };
    #[doc(alias = "RESPONSE_PRECONDITION_FAILED")]
    #[doc = "Godot enumerator name: `RESPONSE_PRECONDITION_FAILED`"]
    pub const PRECONDITION_FAILED: ResponseCode = ResponseCode {
        ord: 412i32
    };
    #[doc(alias = "RESPONSE_REQUEST_ENTITY_TOO_LARGE")]
    #[doc = "Godot enumerator name: `RESPONSE_REQUEST_ENTITY_TOO_LARGE`"]
    pub const REQUEST_ENTITY_TOO_LARGE: ResponseCode = ResponseCode {
        ord: 413i32
    };
    #[doc(alias = "RESPONSE_REQUEST_URI_TOO_LONG")]
    #[doc = "Godot enumerator name: `RESPONSE_REQUEST_URI_TOO_LONG`"]
    pub const REQUEST_URI_TOO_LONG: ResponseCode = ResponseCode {
        ord: 414i32
    };
    #[doc(alias = "RESPONSE_UNSUPPORTED_MEDIA_TYPE")]
    #[doc = "Godot enumerator name: `RESPONSE_UNSUPPORTED_MEDIA_TYPE`"]
    pub const UNSUPPORTED_MEDIA_TYPE: ResponseCode = ResponseCode {
        ord: 415i32
    };
    #[doc(alias = "RESPONSE_REQUESTED_RANGE_NOT_SATISFIABLE")]
    #[doc = "Godot enumerator name: `RESPONSE_REQUESTED_RANGE_NOT_SATISFIABLE`"]
    pub const REQUESTED_RANGE_NOT_SATISFIABLE: ResponseCode = ResponseCode {
        ord: 416i32
    };
    #[doc(alias = "RESPONSE_EXPECTATION_FAILED")]
    #[doc = "Godot enumerator name: `RESPONSE_EXPECTATION_FAILED`"]
    pub const EXPECTATION_FAILED: ResponseCode = ResponseCode {
        ord: 417i32
    };
    #[doc(alias = "RESPONSE_IM_A_TEAPOT")]
    #[doc = "Godot enumerator name: `RESPONSE_IM_A_TEAPOT`"]
    pub const IM_A_TEAPOT: ResponseCode = ResponseCode {
        ord: 418i32
    };
    #[doc(alias = "RESPONSE_MISDIRECTED_REQUEST")]
    #[doc = "Godot enumerator name: `RESPONSE_MISDIRECTED_REQUEST`"]
    pub const MISDIRECTED_REQUEST: ResponseCode = ResponseCode {
        ord: 421i32
    };
    #[doc(alias = "RESPONSE_UNPROCESSABLE_ENTITY")]
    #[doc = "Godot enumerator name: `RESPONSE_UNPROCESSABLE_ENTITY`"]
    pub const UNPROCESSABLE_ENTITY: ResponseCode = ResponseCode {
        ord: 422i32
    };
    #[doc(alias = "RESPONSE_LOCKED")]
    #[doc = "Godot enumerator name: `RESPONSE_LOCKED`"]
    pub const LOCKED: ResponseCode = ResponseCode {
        ord: 423i32
    };
    #[doc(alias = "RESPONSE_FAILED_DEPENDENCY")]
    #[doc = "Godot enumerator name: `RESPONSE_FAILED_DEPENDENCY`"]
    pub const FAILED_DEPENDENCY: ResponseCode = ResponseCode {
        ord: 424i32
    };
    #[doc(alias = "RESPONSE_UPGRADE_REQUIRED")]
    #[doc = "Godot enumerator name: `RESPONSE_UPGRADE_REQUIRED`"]
    pub const UPGRADE_REQUIRED: ResponseCode = ResponseCode {
        ord: 426i32
    };
    #[doc(alias = "RESPONSE_PRECONDITION_REQUIRED")]
    #[doc = "Godot enumerator name: `RESPONSE_PRECONDITION_REQUIRED`"]
    pub const PRECONDITION_REQUIRED: ResponseCode = ResponseCode {
        ord: 428i32
    };
    #[doc(alias = "RESPONSE_TOO_MANY_REQUESTS")]
    #[doc = "Godot enumerator name: `RESPONSE_TOO_MANY_REQUESTS`"]
    pub const TOO_MANY_REQUESTS: ResponseCode = ResponseCode {
        ord: 429i32
    };
    #[doc(alias = "RESPONSE_REQUEST_HEADER_FIELDS_TOO_LARGE")]
    #[doc = "Godot enumerator name: `RESPONSE_REQUEST_HEADER_FIELDS_TOO_LARGE`"]
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: ResponseCode = ResponseCode {
        ord: 431i32
    };
    #[doc(alias = "RESPONSE_UNAVAILABLE_FOR_LEGAL_REASONS")]
    #[doc = "Godot enumerator name: `RESPONSE_UNAVAILABLE_FOR_LEGAL_REASONS`"]
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: ResponseCode = ResponseCode {
        ord: 451i32
    };
    #[doc(alias = "RESPONSE_INTERNAL_SERVER_ERROR")]
    #[doc = "Godot enumerator name: `RESPONSE_INTERNAL_SERVER_ERROR`"]
    pub const INTERNAL_SERVER_ERROR: ResponseCode = ResponseCode {
        ord: 500i32
    };
    #[doc(alias = "RESPONSE_NOT_IMPLEMENTED")]
    #[doc = "Godot enumerator name: `RESPONSE_NOT_IMPLEMENTED`"]
    pub const NOT_IMPLEMENTED: ResponseCode = ResponseCode {
        ord: 501i32
    };
    #[doc(alias = "RESPONSE_BAD_GATEWAY")]
    #[doc = "Godot enumerator name: `RESPONSE_BAD_GATEWAY`"]
    pub const BAD_GATEWAY: ResponseCode = ResponseCode {
        ord: 502i32
    };
    #[doc(alias = "RESPONSE_SERVICE_UNAVAILABLE")]
    #[doc = "Godot enumerator name: `RESPONSE_SERVICE_UNAVAILABLE`"]
    pub const SERVICE_UNAVAILABLE: ResponseCode = ResponseCode {
        ord: 503i32
    };
    #[doc(alias = "RESPONSE_GATEWAY_TIMEOUT")]
    #[doc = "Godot enumerator name: `RESPONSE_GATEWAY_TIMEOUT`"]
    pub const GATEWAY_TIMEOUT: ResponseCode = ResponseCode {
        ord: 504i32
    };
    #[doc(alias = "RESPONSE_HTTP_VERSION_NOT_SUPPORTED")]
    #[doc = "Godot enumerator name: `RESPONSE_HTTP_VERSION_NOT_SUPPORTED`"]
    pub const HTTP_VERSION_NOT_SUPPORTED: ResponseCode = ResponseCode {
        ord: 505i32
    };
    #[doc(alias = "RESPONSE_VARIANT_ALSO_NEGOTIATES")]
    #[doc = "Godot enumerator name: `RESPONSE_VARIANT_ALSO_NEGOTIATES`"]
    pub const VARIANT_ALSO_NEGOTIATES: ResponseCode = ResponseCode {
        ord: 506i32
    };
    #[doc(alias = "RESPONSE_INSUFFICIENT_STORAGE")]
    #[doc = "Godot enumerator name: `RESPONSE_INSUFFICIENT_STORAGE`"]
    pub const INSUFFICIENT_STORAGE: ResponseCode = ResponseCode {
        ord: 507i32
    };
    #[doc(alias = "RESPONSE_LOOP_DETECTED")]
    #[doc = "Godot enumerator name: `RESPONSE_LOOP_DETECTED`"]
    pub const LOOP_DETECTED: ResponseCode = ResponseCode {
        ord: 508i32
    };
    #[doc(alias = "RESPONSE_NOT_EXTENDED")]
    #[doc = "Godot enumerator name: `RESPONSE_NOT_EXTENDED`"]
    pub const NOT_EXTENDED: ResponseCode = ResponseCode {
        ord: 510i32
    };
    #[doc(alias = "RESPONSE_NETWORK_AUTH_REQUIRED")]
    #[doc = "Godot enumerator name: `RESPONSE_NETWORK_AUTH_REQUIRED`"]
    pub const NETWORK_AUTH_REQUIRED: ResponseCode = ResponseCode {
        ord: 511i32
    };
    
}
impl std::fmt::Debug for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ResponseCode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ResponseCode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 100i32 | ord @ 101i32 | ord @ 102i32 | ord @ 200i32 | ord @ 201i32 | ord @ 202i32 | ord @ 203i32 | ord @ 204i32 | ord @ 205i32 | ord @ 206i32 | ord @ 207i32 | ord @ 208i32 | ord @ 226i32 | ord @ 300i32 | ord @ 301i32 | ord @ 302i32 | ord @ 303i32 | ord @ 304i32 | ord @ 305i32 | ord @ 306i32 | ord @ 307i32 | ord @ 308i32 | ord @ 400i32 | ord @ 401i32 | ord @ 402i32 | ord @ 403i32 | ord @ 404i32 | ord @ 405i32 | ord @ 406i32 | ord @ 407i32 | ord @ 408i32 | ord @ 409i32 | ord @ 410i32 | ord @ 411i32 | ord @ 412i32 | ord @ 413i32 | ord @ 414i32 | ord @ 415i32 | ord @ 416i32 | ord @ 417i32 | ord @ 418i32 | ord @ 421i32 | ord @ 422i32 | ord @ 423i32 | ord @ 424i32 | ord @ 426i32 | ord @ 428i32 | ord @ 429i32 | ord @ 431i32 | ord @ 451i32 | ord @ 500i32 | ord @ 501i32 | ord @ 502i32 | ord @ 503i32 | ord @ 504i32 | ord @ 505i32 | ord @ 506i32 | ord @ 507i32 | ord @ 508i32 | ord @ 510i32 | ord @ 511i32 => Some(Self {
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
            Self::CONTINUE => "CONTINUE", Self::SWITCHING_PROTOCOLS => "SWITCHING_PROTOCOLS", Self::PROCESSING => "PROCESSING", Self::OK => "OK", Self::CREATED => "CREATED", Self::ACCEPTED => "ACCEPTED", Self::NON_AUTHORITATIVE_INFORMATION => "NON_AUTHORITATIVE_INFORMATION", Self::NO_CONTENT => "NO_CONTENT", Self::RESET_CONTENT => "RESET_CONTENT", Self::PARTIAL_CONTENT => "PARTIAL_CONTENT", Self::MULTI_STATUS => "MULTI_STATUS", Self::ALREADY_REPORTED => "ALREADY_REPORTED", Self::IM_USED => "IM_USED", Self::MULTIPLE_CHOICES => "MULTIPLE_CHOICES", Self::MOVED_PERMANENTLY => "MOVED_PERMANENTLY", Self::FOUND => "FOUND", Self::SEE_OTHER => "SEE_OTHER", Self::NOT_MODIFIED => "NOT_MODIFIED", Self::USE_PROXY => "USE_PROXY", Self::SWITCH_PROXY => "SWITCH_PROXY", Self::TEMPORARY_REDIRECT => "TEMPORARY_REDIRECT", Self::PERMANENT_REDIRECT => "PERMANENT_REDIRECT", Self::BAD_REQUEST => "BAD_REQUEST", Self::UNAUTHORIZED => "UNAUTHORIZED", Self::PAYMENT_REQUIRED => "PAYMENT_REQUIRED", Self::FORBIDDEN => "FORBIDDEN", Self::NOT_FOUND => "NOT_FOUND", Self::METHOD_NOT_ALLOWED => "METHOD_NOT_ALLOWED", Self::NOT_ACCEPTABLE => "NOT_ACCEPTABLE", Self::PROXY_AUTHENTICATION_REQUIRED => "PROXY_AUTHENTICATION_REQUIRED", Self::REQUEST_TIMEOUT => "REQUEST_TIMEOUT", Self::CONFLICT => "CONFLICT", Self::GONE => "GONE", Self::LENGTH_REQUIRED => "LENGTH_REQUIRED", Self::PRECONDITION_FAILED => "PRECONDITION_FAILED", Self::REQUEST_ENTITY_TOO_LARGE => "REQUEST_ENTITY_TOO_LARGE", Self::REQUEST_URI_TOO_LONG => "REQUEST_URI_TOO_LONG", Self::UNSUPPORTED_MEDIA_TYPE => "UNSUPPORTED_MEDIA_TYPE", Self::REQUESTED_RANGE_NOT_SATISFIABLE => "REQUESTED_RANGE_NOT_SATISFIABLE", Self::EXPECTATION_FAILED => "EXPECTATION_FAILED", Self::IM_A_TEAPOT => "IM_A_TEAPOT", Self::MISDIRECTED_REQUEST => "MISDIRECTED_REQUEST", Self::UNPROCESSABLE_ENTITY => "UNPROCESSABLE_ENTITY", Self::LOCKED => "LOCKED", Self::FAILED_DEPENDENCY => "FAILED_DEPENDENCY", Self::UPGRADE_REQUIRED => "UPGRADE_REQUIRED", Self::PRECONDITION_REQUIRED => "PRECONDITION_REQUIRED", Self::TOO_MANY_REQUESTS => "TOO_MANY_REQUESTS", Self::REQUEST_HEADER_FIELDS_TOO_LARGE => "REQUEST_HEADER_FIELDS_TOO_LARGE", Self::UNAVAILABLE_FOR_LEGAL_REASONS => "UNAVAILABLE_FOR_LEGAL_REASONS", Self::INTERNAL_SERVER_ERROR => "INTERNAL_SERVER_ERROR", Self::NOT_IMPLEMENTED => "NOT_IMPLEMENTED", Self::BAD_GATEWAY => "BAD_GATEWAY", Self::SERVICE_UNAVAILABLE => "SERVICE_UNAVAILABLE", Self::GATEWAY_TIMEOUT => "GATEWAY_TIMEOUT", Self::HTTP_VERSION_NOT_SUPPORTED => "HTTP_VERSION_NOT_SUPPORTED", Self::VARIANT_ALSO_NEGOTIATES => "VARIANT_ALSO_NEGOTIATES", Self::INSUFFICIENT_STORAGE => "INSUFFICIENT_STORAGE", Self::LOOP_DETECTED => "LOOP_DETECTED", Self::NOT_EXTENDED => "NOT_EXTENDED", Self::NETWORK_AUTH_REQUIRED => "NETWORK_AUTH_REQUIRED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CONTINUE => "RESPONSE_CONTINUE", Self::SWITCHING_PROTOCOLS => "RESPONSE_SWITCHING_PROTOCOLS", Self::PROCESSING => "RESPONSE_PROCESSING", Self::OK => "RESPONSE_OK", Self::CREATED => "RESPONSE_CREATED", Self::ACCEPTED => "RESPONSE_ACCEPTED", Self::NON_AUTHORITATIVE_INFORMATION => "RESPONSE_NON_AUTHORITATIVE_INFORMATION", Self::NO_CONTENT => "RESPONSE_NO_CONTENT", Self::RESET_CONTENT => "RESPONSE_RESET_CONTENT", Self::PARTIAL_CONTENT => "RESPONSE_PARTIAL_CONTENT", Self::MULTI_STATUS => "RESPONSE_MULTI_STATUS", Self::ALREADY_REPORTED => "RESPONSE_ALREADY_REPORTED", Self::IM_USED => "RESPONSE_IM_USED", Self::MULTIPLE_CHOICES => "RESPONSE_MULTIPLE_CHOICES", Self::MOVED_PERMANENTLY => "RESPONSE_MOVED_PERMANENTLY", Self::FOUND => "RESPONSE_FOUND", Self::SEE_OTHER => "RESPONSE_SEE_OTHER", Self::NOT_MODIFIED => "RESPONSE_NOT_MODIFIED", Self::USE_PROXY => "RESPONSE_USE_PROXY", Self::SWITCH_PROXY => "RESPONSE_SWITCH_PROXY", Self::TEMPORARY_REDIRECT => "RESPONSE_TEMPORARY_REDIRECT", Self::PERMANENT_REDIRECT => "RESPONSE_PERMANENT_REDIRECT", Self::BAD_REQUEST => "RESPONSE_BAD_REQUEST", Self::UNAUTHORIZED => "RESPONSE_UNAUTHORIZED", Self::PAYMENT_REQUIRED => "RESPONSE_PAYMENT_REQUIRED", Self::FORBIDDEN => "RESPONSE_FORBIDDEN", Self::NOT_FOUND => "RESPONSE_NOT_FOUND", Self::METHOD_NOT_ALLOWED => "RESPONSE_METHOD_NOT_ALLOWED", Self::NOT_ACCEPTABLE => "RESPONSE_NOT_ACCEPTABLE", Self::PROXY_AUTHENTICATION_REQUIRED => "RESPONSE_PROXY_AUTHENTICATION_REQUIRED", Self::REQUEST_TIMEOUT => "RESPONSE_REQUEST_TIMEOUT", Self::CONFLICT => "RESPONSE_CONFLICT", Self::GONE => "RESPONSE_GONE", Self::LENGTH_REQUIRED => "RESPONSE_LENGTH_REQUIRED", Self::PRECONDITION_FAILED => "RESPONSE_PRECONDITION_FAILED", Self::REQUEST_ENTITY_TOO_LARGE => "RESPONSE_REQUEST_ENTITY_TOO_LARGE", Self::REQUEST_URI_TOO_LONG => "RESPONSE_REQUEST_URI_TOO_LONG", Self::UNSUPPORTED_MEDIA_TYPE => "RESPONSE_UNSUPPORTED_MEDIA_TYPE", Self::REQUESTED_RANGE_NOT_SATISFIABLE => "RESPONSE_REQUESTED_RANGE_NOT_SATISFIABLE", Self::EXPECTATION_FAILED => "RESPONSE_EXPECTATION_FAILED", Self::IM_A_TEAPOT => "RESPONSE_IM_A_TEAPOT", Self::MISDIRECTED_REQUEST => "RESPONSE_MISDIRECTED_REQUEST", Self::UNPROCESSABLE_ENTITY => "RESPONSE_UNPROCESSABLE_ENTITY", Self::LOCKED => "RESPONSE_LOCKED", Self::FAILED_DEPENDENCY => "RESPONSE_FAILED_DEPENDENCY", Self::UPGRADE_REQUIRED => "RESPONSE_UPGRADE_REQUIRED", Self::PRECONDITION_REQUIRED => "RESPONSE_PRECONDITION_REQUIRED", Self::TOO_MANY_REQUESTS => "RESPONSE_TOO_MANY_REQUESTS", Self::REQUEST_HEADER_FIELDS_TOO_LARGE => "RESPONSE_REQUEST_HEADER_FIELDS_TOO_LARGE", Self::UNAVAILABLE_FOR_LEGAL_REASONS => "RESPONSE_UNAVAILABLE_FOR_LEGAL_REASONS", Self::INTERNAL_SERVER_ERROR => "RESPONSE_INTERNAL_SERVER_ERROR", Self::NOT_IMPLEMENTED => "RESPONSE_NOT_IMPLEMENTED", Self::BAD_GATEWAY => "RESPONSE_BAD_GATEWAY", Self::SERVICE_UNAVAILABLE => "RESPONSE_SERVICE_UNAVAILABLE", Self::GATEWAY_TIMEOUT => "RESPONSE_GATEWAY_TIMEOUT", Self::HTTP_VERSION_NOT_SUPPORTED => "RESPONSE_HTTP_VERSION_NOT_SUPPORTED", Self::VARIANT_ALSO_NEGOTIATES => "RESPONSE_VARIANT_ALSO_NEGOTIATES", Self::INSUFFICIENT_STORAGE => "RESPONSE_INSUFFICIENT_STORAGE", Self::LOOP_DETECTED => "RESPONSE_LOOP_DETECTED", Self::NOT_EXTENDED => "RESPONSE_NOT_EXTENDED", Self::NETWORK_AUTH_REQUIRED => "RESPONSE_NETWORK_AUTH_REQUIRED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ResponseCode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ResponseCode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ResponseCode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#![doc = "Sidecar module for class [`StreamPeerTls`][crate::classes::StreamPeerTls].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeerTLS` enums](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeerTLS.`\n\nInherits [`StreamPeer`][crate::classes::StreamPeer].\n\nRelated symbols:\n\n* [`stream_peer_tls`][crate::classes::stream_peer_tls]: sidecar module with related enum/flag types\n* [`IStreamPeerTls`][crate::classes::IStreamPeerTls]: virtual methods\n\n\nSee also [Godot docs for `StreamPeerTLS`](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StreamPeerTls::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeerTls {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StreamPeerTls`][crate::classes::StreamPeerTls].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StreamPeerTLS` methods](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStreamPeerTls: crate::obj::GodotClass < Base = StreamPeerTls > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl StreamPeerTls {
        pub fn poll(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeerTls", "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn accept_stream(&mut self, stream: impl AsObjectArg < crate::classes::StreamPeer >, server_options: impl AsObjectArg < crate::classes::TlsOptions >,) -> crate::global::Error {
            type CallSig = (crate::global::Error, ObjectArg < crate::classes::StreamPeer >, ObjectArg < crate::classes::TlsOptions >);
            let args = (stream.as_object_arg(), server_options.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeerTls", "accept_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_to_stream_full(&mut self, stream: ObjectArg < crate::classes::StreamPeer >, common_name: CowArg < GString >, client_options: ObjectArg < crate::classes::TlsOptions >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, ObjectArg < crate::classes::StreamPeer >, CowArg < 'a0, GString >, ObjectArg < crate::classes::TlsOptions >);
            let args = (stream, common_name, client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeerTls", "connect_to_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_to_stream_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect_to_stream(&mut self, stream: impl AsObjectArg < crate::classes::StreamPeer >, common_name: impl AsArg < GString >,) -> crate::global::Error {
            self.connect_to_stream_ex(stream, common_name,) . done()
        }
        #[inline]
        pub fn connect_to_stream_ex < 'a > (&'a mut self, stream: impl AsObjectArg < crate::classes::StreamPeer >, common_name: impl AsArg < GString > + 'a,) -> ExConnectToStream < 'a > {
            ExConnectToStream::new(self, stream, common_name,)
        }
        pub fn get_status(&self,) -> crate::classes::stream_peer_tls::Status {
            type CallSig = (crate::classes::stream_peer_tls::Status,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeerTls", "get_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::classes::StreamPeer > > {
            type CallSig = (Option < Gd < crate::classes::StreamPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeerTls", "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn disconnect_from_stream(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StreamPeerTls", "disconnect_from_stream", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for StreamPeerTls {
        type Base = crate::classes::StreamPeer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"StreamPeerTLS"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeerTls {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StreamPeer > for StreamPeerTls {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StreamPeerTls {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StreamPeerTls {
        
    }
    impl crate::obj::cap::GodotDefault for StreamPeerTls {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StreamPeerTls {
        type Target = crate::classes::StreamPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeerTls {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StreamPeerTls`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_StreamPeerTls {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeerTls > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`StreamPeerTls::connect_to_stream_ex`][super::StreamPeerTls::connect_to_stream_ex]."]
#[must_use]
pub struct ExConnectToStream < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeerTls, stream: ObjectCow < crate::classes::StreamPeer >, common_name: CowArg < 'a, GString >, client_options: ObjectCow < crate::classes::TlsOptions >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToStream < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeerTls, stream: impl AsObjectArg < crate::classes::StreamPeer >, common_name: impl AsArg < GString > + 'a,) -> Self {
        let client_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, stream: stream.consume_arg(), common_name: common_name.into_arg(), client_options: client_options.consume_arg(),
        }
    }
    #[inline]
    pub fn client_options(self, client_options: impl AsObjectArg < crate::classes::TlsOptions >) -> Self {
        Self {
            client_options: client_options.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, stream, common_name, client_options,
        }
        = self;
        re_export::StreamPeerTls::connect_to_stream_full(surround_object, stream.cow_as_object_arg(), common_name, client_options.cow_as_object_arg(),)
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
    #[doc(alias = "STATUS_HANDSHAKING")]
    #[doc = "Godot enumerator name: `STATUS_HANDSHAKING`"]
    pub const HANDSHAKING: Status = Status {
        ord: 1i32
    };
    #[doc(alias = "STATUS_CONNECTED")]
    #[doc = "Godot enumerator name: `STATUS_CONNECTED`"]
    pub const CONNECTED: Status = Status {
        ord: 2i32
    };
    #[doc(alias = "STATUS_ERROR")]
    #[doc = "Godot enumerator name: `STATUS_ERROR`"]
    pub const ERROR: Status = Status {
        ord: 3i32
    };
    #[doc(alias = "STATUS_ERROR_HOSTNAME_MISMATCH")]
    #[doc = "Godot enumerator name: `STATUS_ERROR_HOSTNAME_MISMATCH`"]
    pub const ERROR_HOSTNAME_MISMATCH: Status = Status {
        ord: 4i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::HANDSHAKING => "HANDSHAKING", Self::CONNECTED => "CONNECTED", Self::ERROR => "ERROR", Self::ERROR_HOSTNAME_MISMATCH => "ERROR_HOSTNAME_MISMATCH", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISCONNECTED => "STATUS_DISCONNECTED", Self::HANDSHAKING => "STATUS_HANDSHAKING", Self::CONNECTED => "STATUS_CONNECTED", Self::ERROR => "STATUS_ERROR", Self::ERROR_HOSTNAME_MISMATCH => "STATUS_ERROR_HOSTNAME_MISMATCH", _ => self.as_str(),
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
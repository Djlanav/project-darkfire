#![doc = "Sidecar module for class [`WebRtcPeerConnection`][crate::classes::WebRtcPeerConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCPeerConnection` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCPeerConnection.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`web_rtc_peer_connection`][crate::classes::web_rtc_peer_connection]: sidecar module with related enum/flag types\n* [`IWebRtcPeerConnection`][crate::classes::IWebRtcPeerConnection]: virtual methods\n\n\nSee also [Godot docs for `WebRTCPeerConnection`](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`WebRtcPeerConnection::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcPeerConnection {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebRtcPeerConnection`][crate::classes::WebRtcPeerConnection].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebRTCPeerConnection` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebRtcPeerConnection: crate::obj::GodotClass < Base = WebRtcPeerConnection > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebRtcPeerConnection {
        pub fn set_default_extension(extension_class: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (extension_class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "set_default_extension", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn initialize_full(&mut self, configuration: RefArg < Dictionary >,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, RefArg < 'a0, Dictionary >);
            let args = (configuration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "initialize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::initialize_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn initialize(&mut self,) -> crate::global::Error {
            self.initialize_ex() . done()
        }
        #[inline]
        pub fn initialize_ex < 'a > (&'a mut self,) -> ExInitialize < 'a > {
            ExInitialize::new(self,)
        }
        pub(crate) fn create_data_channel_full(&mut self, label: CowArg < GString >, options: RefArg < Dictionary >,) -> Option < Gd < crate::classes::WebRtcDataChannel > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::WebRtcDataChannel > >, CowArg < 'a0, GString >, RefArg < 'a1, Dictionary >);
            let args = (label, options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "create_data_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_data_channel_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_data_channel(&mut self, label: impl AsArg < GString >,) -> Option < Gd < crate::classes::WebRtcDataChannel > > {
            self.create_data_channel_ex(label,) . done()
        }
        #[inline]
        pub fn create_data_channel_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a,) -> ExCreateDataChannel < 'a > {
            ExCreateDataChannel::new(self, label,)
        }
        pub fn create_offer(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "create_offer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_local_description(&mut self, type_: impl AsArg < GString >, sdp: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (type_.into_arg(), sdp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "set_local_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_remote_description(&mut self, type_: impl AsArg < GString >, sdp: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (type_.into_arg(), sdp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "set_remote_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_ice_candidate(&mut self, media: impl AsArg < GString >, index: i32, name: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >);
            let args = (media.into_arg(), index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "add_ice_candidate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_connection_state(&self,) -> crate::classes::web_rtc_peer_connection::ConnectionState {
            type CallSig = (crate::classes::web_rtc_peer_connection::ConnectionState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "get_connection_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gathering_state(&self,) -> crate::classes::web_rtc_peer_connection::GatheringState {
            type CallSig = (crate::classes::web_rtc_peer_connection::GatheringState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10274usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "get_gathering_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_signaling_state(&self,) -> crate::classes::web_rtc_peer_connection::SignalingState {
            type CallSig = (crate::classes::web_rtc_peer_connection::SignalingState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10275usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "get_signaling_state", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebRtcPeerConnection {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"WebRTCPeerConnection"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcPeerConnection {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebRtcPeerConnection {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebRtcPeerConnection {
        
    }
    impl crate::obj::cap::GodotDefault for WebRtcPeerConnection {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebRtcPeerConnection {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcPeerConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebRtcPeerConnection`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_WebRtcPeerConnection {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebRtcPeerConnection > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WebRtcPeerConnection::initialize_ex`][super::WebRtcPeerConnection::initialize_ex]."]
#[must_use]
pub struct ExInitialize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebRtcPeerConnection, configuration: CowArg < 'a, Dictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInitialize < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcPeerConnection,) -> Self {
        let configuration = Dictionary::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, configuration: CowArg::Owned(configuration),
        }
    }
    #[inline]
    pub fn configuration(self, configuration: &'a Dictionary) -> Self {
        Self {
            configuration: CowArg::Borrowed(configuration), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, configuration,
        }
        = self;
        re_export::WebRtcPeerConnection::initialize_full(surround_object, configuration.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`WebRtcPeerConnection::create_data_channel_ex`][super::WebRtcPeerConnection::create_data_channel_ex]."]
#[must_use]
pub struct ExCreateDataChannel < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebRtcPeerConnection, label: CowArg < 'a, GString >, options: CowArg < 'a, Dictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateDataChannel < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcPeerConnection, label: impl AsArg < GString > + 'a,) -> Self {
        let options = Dictionary::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), options: CowArg::Owned(options),
        }
    }
    #[inline]
    pub fn options(self, options: &'a Dictionary) -> Self {
        Self {
            options: CowArg::Borrowed(options), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::WebRtcDataChannel > > {
        let Self {
            _phantom, surround_object, label, options,
        }
        = self;
        re_export::WebRtcPeerConnection::create_data_channel_full(surround_object, label, options.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConnectionState {
    ord: i32
}
impl ConnectionState {
    #[doc(alias = "STATE_NEW")]
    #[doc = "Godot enumerator name: `STATE_NEW`"]
    pub const NEW: ConnectionState = ConnectionState {
        ord: 0i32
    };
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: ConnectionState = ConnectionState {
        ord: 1i32
    };
    #[doc(alias = "STATE_CONNECTED")]
    #[doc = "Godot enumerator name: `STATE_CONNECTED`"]
    pub const CONNECTED: ConnectionState = ConnectionState {
        ord: 2i32
    };
    #[doc(alias = "STATE_DISCONNECTED")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECTED`"]
    pub const DISCONNECTED: ConnectionState = ConnectionState {
        ord: 3i32
    };
    #[doc(alias = "STATE_FAILED")]
    #[doc = "Godot enumerator name: `STATE_FAILED`"]
    pub const FAILED: ConnectionState = ConnectionState {
        ord: 4i32
    };
    #[doc(alias = "STATE_CLOSED")]
    #[doc = "Godot enumerator name: `STATE_CLOSED`"]
    pub const CLOSED: ConnectionState = ConnectionState {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConnectionState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConnectionState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NEW => "NEW", Self::CONNECTING => "CONNECTING", Self::CONNECTED => "CONNECTED", Self::DISCONNECTED => "DISCONNECTED", Self::FAILED => "FAILED", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEW => "STATE_NEW", Self::CONNECTING => "STATE_CONNECTING", Self::CONNECTED => "STATE_CONNECTED", Self::DISCONNECTED => "STATE_DISCONNECTED", Self::FAILED => "STATE_FAILED", Self::CLOSED => "STATE_CLOSED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ConnectionState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ConnectionState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectionState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GatheringState {
    ord: i32
}
impl GatheringState {
    #[doc(alias = "GATHERING_STATE_NEW")]
    #[doc = "Godot enumerator name: `GATHERING_STATE_NEW`"]
    pub const NEW: GatheringState = GatheringState {
        ord: 0i32
    };
    #[doc(alias = "GATHERING_STATE_GATHERING")]
    #[doc = "Godot enumerator name: `GATHERING_STATE_GATHERING`"]
    pub const GATHERING: GatheringState = GatheringState {
        ord: 1i32
    };
    #[doc(alias = "GATHERING_STATE_COMPLETE")]
    #[doc = "Godot enumerator name: `GATHERING_STATE_COMPLETE`"]
    pub const COMPLETE: GatheringState = GatheringState {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GatheringState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GatheringState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GatheringState {
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
            Self::NEW => "NEW", Self::GATHERING => "GATHERING", Self::COMPLETE => "COMPLETE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEW => "GATHERING_STATE_NEW", Self::GATHERING => "GATHERING_STATE_GATHERING", Self::COMPLETE => "GATHERING_STATE_COMPLETE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for GatheringState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GatheringState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GatheringState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SignalingState {
    ord: i32
}
impl SignalingState {
    #[doc(alias = "SIGNALING_STATE_STABLE")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_STABLE`"]
    pub const STABLE: SignalingState = SignalingState {
        ord: 0i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_LOCAL_OFFER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_LOCAL_OFFER`"]
    pub const HAVE_LOCAL_OFFER: SignalingState = SignalingState {
        ord: 1i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_REMOTE_OFFER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_REMOTE_OFFER`"]
    pub const HAVE_REMOTE_OFFER: SignalingState = SignalingState {
        ord: 2i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_LOCAL_PRANSWER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_LOCAL_PRANSWER`"]
    pub const HAVE_LOCAL_PRANSWER: SignalingState = SignalingState {
        ord: 3i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_REMOTE_PRANSWER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_REMOTE_PRANSWER`"]
    pub const HAVE_REMOTE_PRANSWER: SignalingState = SignalingState {
        ord: 4i32
    };
    #[doc(alias = "SIGNALING_STATE_CLOSED")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_CLOSED`"]
    pub const CLOSED: SignalingState = SignalingState {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SignalingState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SignalingState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SignalingState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::STABLE => "STABLE", Self::HAVE_LOCAL_OFFER => "HAVE_LOCAL_OFFER", Self::HAVE_REMOTE_OFFER => "HAVE_REMOTE_OFFER", Self::HAVE_LOCAL_PRANSWER => "HAVE_LOCAL_PRANSWER", Self::HAVE_REMOTE_PRANSWER => "HAVE_REMOTE_PRANSWER", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STABLE => "SIGNALING_STATE_STABLE", Self::HAVE_LOCAL_OFFER => "SIGNALING_STATE_HAVE_LOCAL_OFFER", Self::HAVE_REMOTE_OFFER => "SIGNALING_STATE_HAVE_REMOTE_OFFER", Self::HAVE_LOCAL_PRANSWER => "SIGNALING_STATE_HAVE_LOCAL_PRANSWER", Self::HAVE_REMOTE_PRANSWER => "SIGNALING_STATE_HAVE_REMOTE_PRANSWER", Self::CLOSED => "SIGNALING_STATE_CLOSED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SignalingState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SignalingState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SignalingState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
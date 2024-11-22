#![doc = "Sidecar module for class [`ENetMultiplayerPeer`][crate::classes::ENetMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetMultiplayerPeer.`\n\nInherits [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nRelated symbols:\n\n* [`enet_multiplayer_peer`][crate::classes::enet_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IENetMultiplayerPeer`][crate::classes::IENetMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `ENetMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ENetMultiplayerPeer::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ENetMultiplayerPeer`][crate::classes::ENetMultiplayerPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetMultiplayerPeer: crate::obj::GodotClass < Base = ENetMultiplayerPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ENetMultiplayerPeer {
        pub(crate) fn create_server_full(&mut self, port: i32, max_clients: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, i32, i32, i32, i32, i32);
            let args = (port, max_clients, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "create_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_server_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_server(&mut self, port: i32,) -> crate::global::Error {
            self.create_server_ex(port,) . done()
        }
        #[inline]
        pub fn create_server_ex < 'a > (&'a mut self, port: i32,) -> ExCreateServer < 'a > {
            ExCreateServer::new(self, port,)
        }
        pub(crate) fn create_client_full(&mut self, address: CowArg < GString >, port: i32, channel_count: i32, in_bandwidth: i32, out_bandwidth: i32, local_port: i32,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, i32, i32, i32, i32, i32);
            let args = (address, port, channel_count, in_bandwidth, out_bandwidth, local_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "create_client", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_client_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_client(&mut self, address: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            self.create_client_ex(address, port,) . done()
        }
        #[inline]
        pub fn create_client_ex < 'a > (&'a mut self, address: impl AsArg < GString > + 'a, port: i32,) -> ExCreateClient < 'a > {
            ExCreateClient::new(self, address, port,)
        }
        pub fn create_mesh(&mut self, unique_id: i32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, i32);
            let args = (unique_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "create_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_mesh_peer(&mut self, peer_id: i32, host: impl AsObjectArg < crate::classes::ENetConnection >,) -> crate::global::Error {
            type CallSig = (crate::global::Error, i32, ObjectArg < crate::classes::ENetConnection >);
            let args = (peer_id, host.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "add_mesh_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bind_ip(&mut self, ip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (ip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "set_bind_ip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_host(&self,) -> Option < Gd < crate::classes::ENetConnection > > {
            type CallSig = (Option < Gd < crate::classes::ENetConnection > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "get_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peer(&self, id: i32,) -> Option < Gd < crate::classes::ENetPacketPeer > > {
            type CallSig = (Option < Gd < crate::classes::ENetPacketPeer > >, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "get_peer", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ENetMultiplayerPeer {
        type Base = crate::classes::MultiplayerPeer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ENetMultiplayerPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MultiplayerPeer > for ENetMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for ENetMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ENetMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ENetMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for ENetMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ENetMultiplayerPeer {
        type Target = crate::classes::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ENetMultiplayerPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ENetMultiplayerPeer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ENetMultiplayerPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PacketPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ENetMultiplayerPeer::create_server_ex`][super::ENetMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetMultiplayerPeer, port: i32, max_clients: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateServer < 'a > {
    fn new(surround_object: &'a mut re_export::ENetMultiplayerPeer, port: i32,) -> Self {
        let max_clients = 32i32;
        let max_channels = 0i32;
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, max_clients: max_clients, max_channels: max_channels, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth,
        }
    }
    #[inline]
    pub fn max_clients(self, max_clients: i32) -> Self {
        Self {
            max_clients: max_clients, .. self
        }
    }
    #[inline]
    pub fn max_channels(self, max_channels: i32) -> Self {
        Self {
            max_channels: max_channels, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, max_clients, max_channels, in_bandwidth, out_bandwidth,
        }
        = self;
        re_export::ENetMultiplayerPeer::create_server_full(surround_object, port, max_clients, max_channels, in_bandwidth, out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetMultiplayerPeer::create_client_ex`][super::ENetMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetMultiplayerPeer, address: CowArg < 'a, GString >, port: i32, channel_count: i32, in_bandwidth: i32, out_bandwidth: i32, local_port: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateClient < 'a > {
    fn new(surround_object: &'a mut re_export::ENetMultiplayerPeer, address: impl AsArg < GString > + 'a, port: i32,) -> Self {
        let channel_count = 0i32;
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        let local_port = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, address: address.into_arg(), port: port, channel_count: channel_count, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth, local_port: local_port,
        }
    }
    #[inline]
    pub fn channel_count(self, channel_count: i32) -> Self {
        Self {
            channel_count: channel_count, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn local_port(self, local_port: i32) -> Self {
        Self {
            local_port: local_port, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, address, port, channel_count, in_bandwidth, out_bandwidth, local_port,
        }
        = self;
        re_export::ENetMultiplayerPeer::create_client_full(surround_object, address, port, channel_count, in_bandwidth, out_bandwidth, local_port,)
    }
}
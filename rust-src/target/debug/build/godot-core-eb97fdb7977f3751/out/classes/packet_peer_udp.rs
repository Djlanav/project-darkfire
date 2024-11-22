#![doc = "Sidecar module for class [`PacketPeerUdp`][crate::classes::PacketPeerUdp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PacketPeerUDP` enums](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PacketPeerUDP.`\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`packet_peer_udp`][crate::classes::packet_peer_udp]: sidecar module with related enum/flag types\n* [`IPacketPeerUdp`][crate::classes::IPacketPeerUdp]: virtual methods\n\n\nSee also [Godot docs for `PacketPeerUDP`](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PacketPeerUdp::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PacketPeerUdp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PacketPeerUdp`][crate::classes::PacketPeerUdp].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PacketPeerUDP` methods](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPacketPeerUdp: crate::obj::GodotClass < Base = PacketPeerUdp > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PacketPeerUdp {
        pub(crate) fn bind_full(&mut self, port: i32, bind_address: CowArg < GString >, recv_buf_size: i32,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, i32, CowArg < 'a0, GString >, i32);
            let args = (port, bind_address, recv_buf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "bind", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bind_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bind(&mut self, port: i32,) -> crate::global::Error {
            self.bind_ex(port,) . done()
        }
        #[inline]
        pub fn bind_ex < 'a > (&'a mut self, port: i32,) -> ExBind < 'a > {
            ExBind::new(self, port,)
        }
        pub fn close(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn wait(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "wait", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bound(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "is_bound", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn connect_to_host(&mut self, host: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, i32);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "connect_to_host", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_socket_connected(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "is_socket_connected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_ip(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "get_packet_ip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_port(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "get_packet_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_port(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "get_local_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dest_address(&mut self, host: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, GString >, i32);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "set_dest_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_broadcast_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "set_broadcast_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn join_multicast_group(&mut self, multicast_address: impl AsArg < GString >, interface_name: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (multicast_address.into_arg(), interface_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "join_multicast_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn leave_multicast_group(&mut self, multicast_address: impl AsArg < GString >, interface_name: impl AsArg < GString >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (multicast_address.into_arg(), interface_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "leave_multicast_group", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PacketPeerUdp {
        type Base = crate::classes::PacketPeer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PacketPeerUDP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PacketPeerUdp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for PacketPeerUdp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PacketPeerUdp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PacketPeerUdp {
        
    }
    impl crate::obj::cap::GodotDefault for PacketPeerUdp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PacketPeerUdp {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PacketPeerUdp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PacketPeerUdp`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PacketPeerUdp {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PacketPeerUdp > for $Class {
                
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
#[doc = "Default-param extender for [`PacketPeerUdp::bind_ex`][super::PacketPeerUdp::bind_ex]."]
#[must_use]
pub struct ExBind < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PacketPeerUdp, port: i32, bind_address: CowArg < 'a, GString >, recv_buf_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBind < 'a > {
    fn new(surround_object: &'a mut re_export::PacketPeerUdp, port: i32,) -> Self {
        let bind_address = GString::from("*");
        let recv_buf_size = 65536i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, bind_address: CowArg::Owned(bind_address), recv_buf_size: recv_buf_size,
        }
    }
    #[inline]
    pub fn bind_address(self, bind_address: impl AsArg < GString > + 'a) -> Self {
        Self {
            bind_address: bind_address.into_arg(), .. self
        }
    }
    #[inline]
    pub fn recv_buf_size(self, recv_buf_size: i32) -> Self {
        Self {
            recv_buf_size: recv_buf_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, bind_address, recv_buf_size,
        }
        = self;
        re_export::PacketPeerUdp::bind_full(surround_object, port, bind_address, recv_buf_size,)
    }
}
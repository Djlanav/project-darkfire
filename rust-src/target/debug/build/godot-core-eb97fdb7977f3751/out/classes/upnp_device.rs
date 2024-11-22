#![doc = "Sidecar module for class [`UpnpDevice`][crate::classes::UpnpDevice].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UPNPDevice` enums](https://docs.godotengine.org/en/stable/classes/class_upnpdevice.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UPNPDevice.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`upnp_device`][crate::classes::upnp_device]: sidecar module with related enum/flag types\n* [`IUpnpDevice`][crate::classes::IUpnpDevice]: virtual methods\n\n\nSee also [Godot docs for `UPNPDevice`](https://docs.godotengine.org/en/stable/classes/class_upnpdevice.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`UpnpDevice::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UpnpDevice {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`UpnpDevice`][crate::classes::UpnpDevice].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `UPNPDevice` methods](https://docs.godotengine.org/en/stable/classes/class_upnpdevice.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUpnpDevice: crate::obj::GodotClass < Base = UpnpDevice > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl UpnpDevice {
        pub fn is_valid_gateway(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "is_valid_gateway", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn query_external_address(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "query_external_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_port_mapping_full(&self, port: i32, port_internal: i32, desc: CowArg < GString >, proto: CowArg < GString >, duration: i32,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, i32, i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32);
            let args = (port, port_internal, desc, proto, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "add_port_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_port_mapping_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_port_mapping(&self, port: i32,) -> i32 {
            self.add_port_mapping_ex(port,) . done()
        }
        #[inline]
        pub fn add_port_mapping_ex < 'a > (&'a self, port: i32,) -> ExAddPortMapping < 'a > {
            ExAddPortMapping::new(self, port,)
        }
        pub(crate) fn delete_port_mapping_full(&self, port: i32, proto: CowArg < GString >,) -> i32 {
            type CallSig < 'a0, > = (i32, i32, CowArg < 'a0, GString >);
            let args = (port, proto,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "delete_port_mapping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::delete_port_mapping_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn delete_port_mapping(&self, port: i32,) -> i32 {
            self.delete_port_mapping_ex(port,) . done()
        }
        #[inline]
        pub fn delete_port_mapping_ex < 'a > (&'a self, port: i32,) -> ExDeletePortMapping < 'a > {
            ExDeletePortMapping::new(self, port,)
        }
        pub fn set_description_url(&mut self, url: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (url.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "set_description_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_description_url(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "get_description_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_service_type(&mut self, type_: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "set_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_service_type(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "get_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_control_url(&mut self, url: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (url.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "set_igd_control_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_control_url(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "get_igd_control_url", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_service_type(&mut self, type_: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "set_igd_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_service_type(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "get_igd_service_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_our_addr(&mut self, addr: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (addr.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "set_igd_our_addr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_our_addr(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "get_igd_our_addr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_igd_status(&mut self, status: crate::classes::upnp_device::IgdStatus,) {
            type CallSig = ((), crate::classes::upnp_device::IgdStatus);
            let args = (status,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "set_igd_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_igd_status(&self,) -> crate::classes::upnp_device::IgdStatus {
            type CallSig = (crate::classes::upnp_device::IgdStatus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UpnpDevice", "get_igd_status", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for UpnpDevice {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"UPNPDevice"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UpnpDevice {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for UpnpDevice {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for UpnpDevice {
        
    }
    impl crate::obj::cap::GodotDefault for UpnpDevice {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UpnpDevice {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UpnpDevice {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`UpnpDevice`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_UpnpDevice {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::UpnpDevice > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UpnpDevice::add_port_mapping_ex`][super::UpnpDevice::add_port_mapping_ex]."]
#[must_use]
pub struct ExAddPortMapping < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::UpnpDevice, port: i32, port_internal: i32, desc: CowArg < 'a, GString >, proto: CowArg < 'a, GString >, duration: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPortMapping < 'a > {
    fn new(surround_object: &'a re_export::UpnpDevice, port: i32,) -> Self {
        let port_internal = 0i32;
        let desc = GString::from("");
        let proto = GString::from("UDP");
        let duration = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, port_internal: port_internal, desc: CowArg::Owned(desc), proto: CowArg::Owned(proto), duration: duration,
        }
    }
    #[inline]
    pub fn port_internal(self, port_internal: i32) -> Self {
        Self {
            port_internal: port_internal, .. self
        }
    }
    #[inline]
    pub fn desc(self, desc: impl AsArg < GString > + 'a) -> Self {
        Self {
            desc: desc.into_arg(), .. self
        }
    }
    #[inline]
    pub fn proto(self, proto: impl AsArg < GString > + 'a) -> Self {
        Self {
            proto: proto.into_arg(), .. self
        }
    }
    #[inline]
    pub fn duration(self, duration: i32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, port, port_internal, desc, proto, duration,
        }
        = self;
        re_export::UpnpDevice::add_port_mapping_full(surround_object, port, port_internal, desc, proto, duration,)
    }
}
#[doc = "Default-param extender for [`UpnpDevice::delete_port_mapping_ex`][super::UpnpDevice::delete_port_mapping_ex]."]
#[must_use]
pub struct ExDeletePortMapping < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::UpnpDevice, port: i32, proto: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeletePortMapping < 'a > {
    fn new(surround_object: &'a re_export::UpnpDevice, port: i32,) -> Self {
        let proto = GString::from("UDP");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, proto: CowArg::Owned(proto),
        }
    }
    #[inline]
    pub fn proto(self, proto: impl AsArg < GString > + 'a) -> Self {
        Self {
            proto: proto.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, port, proto,
        }
        = self;
        re_export::UpnpDevice::delete_port_mapping_full(surround_object, port, proto,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `IGDStatus`."]
pub struct IgdStatus {
    ord: i32
}
impl IgdStatus {
    #[doc(alias = "IGD_STATUS_OK")]
    #[doc = "Godot enumerator name: `IGD_STATUS_OK`"]
    pub const OK: IgdStatus = IgdStatus {
        ord: 0i32
    };
    #[doc(alias = "IGD_STATUS_HTTP_ERROR")]
    #[doc = "Godot enumerator name: `IGD_STATUS_HTTP_ERROR`"]
    pub const HTTP_ERROR: IgdStatus = IgdStatus {
        ord: 1i32
    };
    #[doc(alias = "IGD_STATUS_HTTP_EMPTY")]
    #[doc = "Godot enumerator name: `IGD_STATUS_HTTP_EMPTY`"]
    pub const HTTP_EMPTY: IgdStatus = IgdStatus {
        ord: 2i32
    };
    #[doc(alias = "IGD_STATUS_NO_URLS")]
    #[doc = "Godot enumerator name: `IGD_STATUS_NO_URLS`"]
    pub const NO_URLS: IgdStatus = IgdStatus {
        ord: 3i32
    };
    #[doc(alias = "IGD_STATUS_NO_IGD")]
    #[doc = "Godot enumerator name: `IGD_STATUS_NO_IGD`"]
    pub const NO_IGD: IgdStatus = IgdStatus {
        ord: 4i32
    };
    #[doc(alias = "IGD_STATUS_DISCONNECTED")]
    #[doc = "Godot enumerator name: `IGD_STATUS_DISCONNECTED`"]
    pub const DISCONNECTED: IgdStatus = IgdStatus {
        ord: 5i32
    };
    #[doc(alias = "IGD_STATUS_UNKNOWN_DEVICE")]
    #[doc = "Godot enumerator name: `IGD_STATUS_UNKNOWN_DEVICE`"]
    pub const UNKNOWN_DEVICE: IgdStatus = IgdStatus {
        ord: 6i32
    };
    #[doc(alias = "IGD_STATUS_INVALID_CONTROL")]
    #[doc = "Godot enumerator name: `IGD_STATUS_INVALID_CONTROL`"]
    pub const INVALID_CONTROL: IgdStatus = IgdStatus {
        ord: 7i32
    };
    #[doc(alias = "IGD_STATUS_MALLOC_ERROR")]
    #[doc = "Godot enumerator name: `IGD_STATUS_MALLOC_ERROR`"]
    pub const MALLOC_ERROR: IgdStatus = IgdStatus {
        ord: 8i32
    };
    #[doc(alias = "IGD_STATUS_UNKNOWN_ERROR")]
    #[doc = "Godot enumerator name: `IGD_STATUS_UNKNOWN_ERROR`"]
    pub const UNKNOWN_ERROR: IgdStatus = IgdStatus {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for IgdStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("IgdStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for IgdStatus {
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
            Self::OK => "OK", Self::HTTP_ERROR => "HTTP_ERROR", Self::HTTP_EMPTY => "HTTP_EMPTY", Self::NO_URLS => "NO_URLS", Self::NO_IGD => "NO_IGD", Self::DISCONNECTED => "DISCONNECTED", Self::UNKNOWN_DEVICE => "UNKNOWN_DEVICE", Self::INVALID_CONTROL => "INVALID_CONTROL", Self::MALLOC_ERROR => "MALLOC_ERROR", Self::UNKNOWN_ERROR => "UNKNOWN_ERROR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OK => "IGD_STATUS_OK", Self::HTTP_ERROR => "IGD_STATUS_HTTP_ERROR", Self::HTTP_EMPTY => "IGD_STATUS_HTTP_EMPTY", Self::NO_URLS => "IGD_STATUS_NO_URLS", Self::NO_IGD => "IGD_STATUS_NO_IGD", Self::DISCONNECTED => "IGD_STATUS_DISCONNECTED", Self::UNKNOWN_DEVICE => "IGD_STATUS_UNKNOWN_DEVICE", Self::INVALID_CONTROL => "IGD_STATUS_INVALID_CONTROL", Self::MALLOC_ERROR => "IGD_STATUS_MALLOC_ERROR", Self::UNKNOWN_ERROR => "IGD_STATUS_UNKNOWN_ERROR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for IgdStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for IgdStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for IgdStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
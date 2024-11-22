#![doc = "Sidecar module for class [`CameraServer`][crate::classes::CameraServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CameraServer` enums](https://docs.godotengine.org/en/stable/classes/class_cameraserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CameraServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`camera_server`][crate::classes::camera_server]: sidecar module with related enum/flag types\n* [`ICameraServer`][crate::classes::ICameraServer]: virtual methods\n\n\nSee also [Godot docs for `CameraServer`](https://docs.godotengine.org/en/stable/classes/class_cameraserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`CameraServer::singleton()`][CameraServer::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CameraServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CameraServer`][crate::classes::CameraServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CameraServer` methods](https://docs.godotengine.org/en/stable/classes/class_cameraserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICameraServer: crate::obj::GodotClass < Base = CameraServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CameraServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"CameraServer");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_feed(&mut self, index: i32,) -> Option < Gd < crate::classes::CameraFeed > > {
            type CallSig = (Option < Gd < crate::classes::CameraFeed > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(49usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CameraServer", "get_feed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_feed_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(50usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CameraServer", "get_feed_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn feeds(&mut self,) -> Array < Gd < crate::classes::CameraFeed > > {
            type CallSig = (Array < Gd < crate::classes::CameraFeed > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(51usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CameraServer", "feeds", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_feed(&mut self, feed: impl AsObjectArg < crate::classes::CameraFeed >,) {
            type CallSig = ((), ObjectArg < crate::classes::CameraFeed >);
            let args = (feed.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(52usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CameraServer", "add_feed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_feed(&mut self, feed: impl AsObjectArg < crate::classes::CameraFeed >,) {
            type CallSig = ((), ObjectArg < crate::classes::CameraFeed >);
            let args = (feed.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(53usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CameraServer", "remove_feed", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CameraServer {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CameraServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for CameraServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CameraServer {
        
    }
    impl std::ops::Deref for CameraServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CameraServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CameraServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CameraServer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CameraServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FeedImage {
    ord: i32
}
impl FeedImage {
    #[doc(alias = "FEED_RGBA_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_RGBA_IMAGE`"]
    pub const RGBA_IMAGE: FeedImage = FeedImage {
        ord: 0i32
    };
    #[doc(alias = "FEED_YCBCR_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_YCBCR_IMAGE`"]
    pub const YCBCR_IMAGE: FeedImage = FeedImage {
        ord: 0i32
    };
    #[doc(alias = "FEED_Y_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_Y_IMAGE`"]
    pub const Y_IMAGE: FeedImage = FeedImage {
        ord: 0i32
    };
    #[doc(alias = "FEED_CBCR_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_CBCR_IMAGE`"]
    pub const CBCR_IMAGE: FeedImage = FeedImage {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for FeedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FeedImage") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FeedImage {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::RGBA_IMAGE => "RGBA_IMAGE", Self::YCBCR_IMAGE => "YCBCR_IMAGE", Self::Y_IMAGE => "Y_IMAGE", Self::CBCR_IMAGE => "CBCR_IMAGE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RGBA_IMAGE => "FEED_RGBA_IMAGE", Self::YCBCR_IMAGE => "FEED_YCBCR_IMAGE", Self::Y_IMAGE => "FEED_Y_IMAGE", Self::CBCR_IMAGE => "FEED_CBCR_IMAGE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FeedImage {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FeedImage {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FeedImage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#![doc = "Sidecar module for class [`Sky`][crate::classes::Sky].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Sky` enums](https://docs.godotengine.org/en/stable/classes/class_sky.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Sky.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`sky`][crate::classes::sky]: sidecar module with related enum/flag types\n* [`ISky`][crate::classes::ISky]: virtual methods\n\n\nSee also [Godot docs for `Sky`](https://docs.godotengine.org/en/stable/classes/class_sky.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Sky::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Sky {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Sky`][crate::classes::Sky].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Sky` methods](https://docs.godotengine.org/en/stable/classes/class_sky.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISky: crate::obj::GodotClass < Base = Sky > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Sky {
        pub fn set_radiance_size(&mut self, size: crate::classes::sky::RadianceSize,) {
            type CallSig = ((), crate::classes::sky::RadianceSize);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Sky", "set_radiance_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radiance_size(&self,) -> crate::classes::sky::RadianceSize {
            type CallSig = (crate::classes::sky::RadianceSize,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Sky", "get_radiance_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::classes::sky::ProcessMode,) {
            type CallSig = ((), crate::classes::sky::ProcessMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Sky", "set_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_mode(&self,) -> crate::classes::sky::ProcessMode {
            type CallSig = (crate::classes::sky::ProcessMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Sky", "get_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsObjectArg < crate::classes::Material >,) {
            type CallSig = ((), ObjectArg < crate::classes::Material >);
            let args = (material.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7803usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Sky", "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallSig = (Option < Gd < crate::classes::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7804usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Sky", "get_material", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Sky {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Sky"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Sky {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Sky {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Sky {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Sky {
        
    }
    impl crate::obj::cap::GodotDefault for Sky {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Sky {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Sky {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Sky`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Sky {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Sky > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RadianceSize {
    ord: i32
}
impl RadianceSize {
    #[doc(alias = "RADIANCE_SIZE_32")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_32`"]
    pub const SIZE_32: RadianceSize = RadianceSize {
        ord: 0i32
    };
    #[doc(alias = "RADIANCE_SIZE_64")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_64`"]
    pub const SIZE_64: RadianceSize = RadianceSize {
        ord: 1i32
    };
    #[doc(alias = "RADIANCE_SIZE_128")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_128`"]
    pub const SIZE_128: RadianceSize = RadianceSize {
        ord: 2i32
    };
    #[doc(alias = "RADIANCE_SIZE_256")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_256`"]
    pub const SIZE_256: RadianceSize = RadianceSize {
        ord: 3i32
    };
    #[doc(alias = "RADIANCE_SIZE_512")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_512`"]
    pub const SIZE_512: RadianceSize = RadianceSize {
        ord: 4i32
    };
    #[doc(alias = "RADIANCE_SIZE_1024")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_1024`"]
    pub const SIZE_1024: RadianceSize = RadianceSize {
        ord: 5i32
    };
    #[doc(alias = "RADIANCE_SIZE_2048")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_2048`"]
    pub const SIZE_2048: RadianceSize = RadianceSize {
        ord: 6i32
    };
    #[doc(alias = "RADIANCE_SIZE_MAX")]
    #[doc = "Godot enumerator name: `RADIANCE_SIZE_MAX`"]
    pub const MAX: RadianceSize = RadianceSize {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for RadianceSize {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RadianceSize") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RadianceSize {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::SIZE_32 => "SIZE_32", Self::SIZE_64 => "SIZE_64", Self::SIZE_128 => "SIZE_128", Self::SIZE_256 => "SIZE_256", Self::SIZE_512 => "SIZE_512", Self::SIZE_1024 => "SIZE_1024", Self::SIZE_2048 => "SIZE_2048", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SIZE_32 => "RADIANCE_SIZE_32", Self::SIZE_64 => "RADIANCE_SIZE_64", Self::SIZE_128 => "RADIANCE_SIZE_128", Self::SIZE_256 => "RADIANCE_SIZE_256", Self::SIZE_512 => "RADIANCE_SIZE_512", Self::SIZE_1024 => "RADIANCE_SIZE_1024", Self::SIZE_2048 => "RADIANCE_SIZE_2048", Self::MAX => "RADIANCE_SIZE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for RadianceSize {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for RadianceSize {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RadianceSize {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RadianceSize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessMode {
    ord: i32
}
impl ProcessMode {
    #[doc(alias = "PROCESS_MODE_AUTOMATIC")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_AUTOMATIC`"]
    pub const AUTOMATIC: ProcessMode = ProcessMode {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_MODE_QUALITY")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_QUALITY`"]
    pub const QUALITY: ProcessMode = ProcessMode {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_MODE_INCREMENTAL")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_INCREMENTAL`"]
    pub const INCREMENTAL: ProcessMode = ProcessMode {
        ord: 2i32
    };
    #[doc(alias = "PROCESS_MODE_REALTIME")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_REALTIME`"]
    pub const REALTIME: ProcessMode = ProcessMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::AUTOMATIC => "AUTOMATIC", Self::QUALITY => "QUALITY", Self::INCREMENTAL => "INCREMENTAL", Self::REALTIME => "REALTIME", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::AUTOMATIC => "PROCESS_MODE_AUTOMATIC", Self::QUALITY => "PROCESS_MODE_QUALITY", Self::INCREMENTAL => "PROCESS_MODE_INCREMENTAL", Self::REALTIME => "PROCESS_MODE_REALTIME", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ProcessMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProcessMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
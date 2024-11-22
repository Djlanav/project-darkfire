#![doc = "Sidecar module for class [`AspectRatioContainer`][crate::classes::AspectRatioContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AspectRatioContainer` enums](https://docs.godotengine.org/en/stable/classes/class_aspectratiocontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AspectRatioContainer.`\n\nInherits [`Container`][crate::classes::Container].\n\nRelated symbols:\n\n* [`aspect_ratio_container`][crate::classes::aspect_ratio_container]: sidecar module with related enum/flag types\n* [`IAspectRatioContainer`][crate::classes::IAspectRatioContainer]: virtual methods\n\n\nSee also [Godot docs for `AspectRatioContainer`](https://docs.godotengine.org/en/stable/classes/class_aspectratiocontainer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AspectRatioContainer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AspectRatioContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AspectRatioContainer`][crate::classes::AspectRatioContainer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AspectRatioContainer` methods](https://docs.godotengine.org/en/stable/classes/class_aspectratiocontainer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAspectRatioContainer: crate::obj::GodotClass < Base = AspectRatioContainer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl AspectRatioContainer {
        pub fn set_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "set_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "get_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stretch_mode(&mut self, stretch_mode: crate::classes::aspect_ratio_container::StretchMode,) {
            type CallSig = ((), crate::classes::aspect_ratio_container::StretchMode);
            let args = (stretch_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "set_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stretch_mode(&self,) -> crate::classes::aspect_ratio_container::StretchMode {
            type CallSig = (crate::classes::aspect_ratio_container::StretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "get_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alignment_horizontal(&mut self, alignment_horizontal: crate::classes::aspect_ratio_container::AlignmentMode,) {
            type CallSig = ((), crate::classes::aspect_ratio_container::AlignmentMode);
            let args = (alignment_horizontal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "set_alignment_horizontal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alignment_horizontal(&self,) -> crate::classes::aspect_ratio_container::AlignmentMode {
            type CallSig = (crate::classes::aspect_ratio_container::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "get_alignment_horizontal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alignment_vertical(&mut self, alignment_vertical: crate::classes::aspect_ratio_container::AlignmentMode,) {
            type CallSig = ((), crate::classes::aspect_ratio_container::AlignmentMode);
            let args = (alignment_vertical,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "set_alignment_vertical", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alignment_vertical(&self,) -> crate::classes::aspect_ratio_container::AlignmentMode {
            type CallSig = (crate::classes::aspect_ratio_container::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AspectRatioContainer", "get_alignment_vertical", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AspectRatioContainer {
        type Base = crate::classes::Container;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AspectRatioContainer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AspectRatioContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for AspectRatioContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for AspectRatioContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for AspectRatioContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AspectRatioContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AspectRatioContainer {
        
    }
    impl crate::obj::cap::GodotDefault for AspectRatioContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AspectRatioContainer {
        type Target = crate::classes::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AspectRatioContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AspectRatioContainer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AspectRatioContainer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AspectRatioContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StretchMode {
    ord: i32
}
impl StretchMode {
    #[doc(alias = "STRETCH_WIDTH_CONTROLS_HEIGHT")]
    #[doc = "Godot enumerator name: `STRETCH_WIDTH_CONTROLS_HEIGHT`"]
    pub const WIDTH_CONTROLS_HEIGHT: StretchMode = StretchMode {
        ord: 0i32
    };
    #[doc(alias = "STRETCH_HEIGHT_CONTROLS_WIDTH")]
    #[doc = "Godot enumerator name: `STRETCH_HEIGHT_CONTROLS_WIDTH`"]
    pub const HEIGHT_CONTROLS_WIDTH: StretchMode = StretchMode {
        ord: 1i32
    };
    #[doc(alias = "STRETCH_FIT")]
    #[doc = "Godot enumerator name: `STRETCH_FIT`"]
    pub const FIT: StretchMode = StretchMode {
        ord: 2i32
    };
    #[doc(alias = "STRETCH_COVER")]
    #[doc = "Godot enumerator name: `STRETCH_COVER`"]
    pub const COVER: StretchMode = StretchMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for StretchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StretchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StretchMode {
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
            Self::WIDTH_CONTROLS_HEIGHT => "WIDTH_CONTROLS_HEIGHT", Self::HEIGHT_CONTROLS_WIDTH => "HEIGHT_CONTROLS_WIDTH", Self::FIT => "FIT", Self::COVER => "COVER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::WIDTH_CONTROLS_HEIGHT => "STRETCH_WIDTH_CONTROLS_HEIGHT", Self::HEIGHT_CONTROLS_WIDTH => "STRETCH_HEIGHT_CONTROLS_WIDTH", Self::FIT => "STRETCH_FIT", Self::COVER => "STRETCH_COVER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for StretchMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StretchMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlignmentMode {
    ord: i32
}
impl AlignmentMode {
    #[doc(alias = "ALIGNMENT_BEGIN")]
    #[doc = "Godot enumerator name: `ALIGNMENT_BEGIN`"]
    pub const BEGIN: AlignmentMode = AlignmentMode {
        ord: 0i32
    };
    #[doc(alias = "ALIGNMENT_CENTER")]
    #[doc = "Godot enumerator name: `ALIGNMENT_CENTER`"]
    pub const CENTER: AlignmentMode = AlignmentMode {
        ord: 1i32
    };
    #[doc(alias = "ALIGNMENT_END")]
    #[doc = "Godot enumerator name: `ALIGNMENT_END`"]
    pub const END: AlignmentMode = AlignmentMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AlignmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlignmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlignmentMode {
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
            Self::BEGIN => "BEGIN", Self::CENTER => "CENTER", Self::END => "END", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BEGIN => "ALIGNMENT_BEGIN", Self::CENTER => "ALIGNMENT_CENTER", Self::END => "ALIGNMENT_END", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AlignmentMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlignmentMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlignmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
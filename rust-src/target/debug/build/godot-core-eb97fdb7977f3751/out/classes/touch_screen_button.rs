#![doc = "Sidecar module for class [`TouchScreenButton`][crate::classes::TouchScreenButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TouchScreenButton` enums](https://docs.godotengine.org/en/stable/classes/class_touchscreenbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TouchScreenButton.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`touch_screen_button`][crate::classes::touch_screen_button]: sidecar module with related enum/flag types\n* [`ITouchScreenButton`][crate::classes::ITouchScreenButton]: virtual methods\n\n\nSee also [Godot docs for `TouchScreenButton`](https://docs.godotengine.org/en/stable/classes/class_touchscreenbutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TouchScreenButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TouchScreenButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TouchScreenButton`][crate::classes::TouchScreenButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TouchScreenButton` methods](https://docs.godotengine.org/en/stable/classes/class_touchscreenbutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITouchScreenButton: crate::obj::GodotClass < Base = TouchScreenButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
    impl TouchScreenButton {
        pub fn set_texture_normal(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_texture_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_normal(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "get_texture_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_pressed(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_texture_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_pressed(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "get_texture_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bitmask(&mut self, bitmask: impl AsObjectArg < crate::classes::BitMap >,) {
            type CallSig = ((), ObjectArg < crate::classes::BitMap >);
            let args = (bitmask.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_bitmask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bitmask(&self,) -> Option < Gd < crate::classes::BitMap > > {
            type CallSig = (Option < Gd < crate::classes::BitMap > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "get_bitmask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape(&mut self, shape: impl AsObjectArg < crate::classes::Shape2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Shape2D >);
            let args = (shape.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape(&self,) -> Option < Gd < crate::classes::Shape2D > > {
            type CallSig = (Option < Gd < crate::classes::Shape2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape_centered(&mut self, bool: bool,) {
            type CallSig = ((), bool);
            let args = (bool,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_shape_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_centered(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "is_shape_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape_visible(&mut self, bool: bool,) {
            type CallSig = ((), bool);
            let args = (bool,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_shape_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shape_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "is_shape_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_action(&mut self, action: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "get_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_visibility_mode(&mut self, mode: crate::classes::touch_screen_button::VisibilityMode,) {
            type CallSig = ((), crate::classes::touch_screen_button::VisibilityMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_mode(&self,) -> crate::classes::touch_screen_button::VisibilityMode {
            type CallSig = (crate::classes::touch_screen_button::VisibilityMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "get_visibility_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_passby_press(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "set_passby_press", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_passby_press_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "is_passby_press_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TouchScreenButton", "is_pressed", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TouchScreenButton {
        type Base = crate::classes::Node2D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TouchScreenButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TouchScreenButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for TouchScreenButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TouchScreenButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TouchScreenButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TouchScreenButton {
        
    }
    impl crate::obj::cap::GodotDefault for TouchScreenButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TouchScreenButton {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TouchScreenButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TouchScreenButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TouchScreenButton {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TouchScreenButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
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
pub struct VisibilityMode {
    ord: i32
}
impl VisibilityMode {
    #[doc(alias = "VISIBILITY_ALWAYS")]
    #[doc = "Godot enumerator name: `VISIBILITY_ALWAYS`"]
    pub const ALWAYS: VisibilityMode = VisibilityMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_TOUCHSCREEN_ONLY")]
    #[doc = "Godot enumerator name: `VISIBILITY_TOUCHSCREEN_ONLY`"]
    pub const TOUCHSCREEN_ONLY: VisibilityMode = VisibilityMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VisibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityMode {
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
            Self::ALWAYS => "ALWAYS", Self::TOUCHSCREEN_ONLY => "TOUCHSCREEN_ONLY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ALWAYS => "VISIBILITY_ALWAYS", Self::TOUCHSCREEN_ONLY => "VISIBILITY_TOUCHSCREEN_ONLY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for VisibilityMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibilityMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#![doc = "Sidecar module for class [`TextureButton`][crate::classes::TextureButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextureButton` enums](https://docs.godotengine.org/en/stable/classes/class_texturebutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextureButton.`\n\nInherits [`BaseButton`][crate::classes::BaseButton].\n\nRelated symbols:\n\n* [`texture_button`][crate::classes::texture_button]: sidecar module with related enum/flag types\n* [`ITextureButton`][crate::classes::ITextureButton]: virtual methods\n\n\nSee also [Godot docs for `TextureButton`](https://docs.godotengine.org/en/stable/classes/class_texturebutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TextureButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextureButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextureButton`][crate::classes::TextureButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextureButton` methods](https://docs.godotengine.org/en/stable/classes/class_texturebutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextureButton: crate::obj::GodotClass < Base = TextureButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl TextureButton {
        pub fn set_texture_normal(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8938usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_texture_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_pressed(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8939usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_texture_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_hover(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8940usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_texture_hover", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_disabled(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8941usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_texture_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_focused(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8942usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_texture_focused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_click_mask(&mut self, mask: impl AsObjectArg < crate::classes::BitMap >,) {
            type CallSig = ((), ObjectArg < crate::classes::BitMap >);
            let args = (mask.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8943usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_click_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ignore_texture_size(&mut self, ignore: bool,) {
            type CallSig = ((), bool);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8944usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_ignore_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stretch_mode(&mut self, mode: crate::classes::texture_button::StretchMode,) {
            type CallSig = ((), crate::classes::texture_button::StretchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8945usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_h(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8946usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_flip_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8947usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "is_flipped_h", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_flip_v(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "set_flip_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "is_flipped_v", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_normal(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_texture_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_pressed(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_texture_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_hover(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_texture_hover", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_disabled(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_texture_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_focused(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_texture_focused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_click_mask(&self,) -> Option < Gd < crate::classes::BitMap > > {
            type CallSig = (Option < Gd < crate::classes::BitMap > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_click_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ignore_texture_size(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_ignore_texture_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stretch_mode(&self,) -> crate::classes::texture_button::StretchMode {
            type CallSig = (crate::classes::texture_button::StretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureButton", "get_stretch_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextureButton {
        type Base = crate::classes::BaseButton;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TextureButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextureButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BaseButton > for TextureButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TextureButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TextureButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TextureButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextureButton {
        
    }
    impl crate::obj::cap::GodotDefault for TextureButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextureButton {
        type Target = crate::classes::BaseButton;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextureButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextureButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TextureButton {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextureButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
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
    #[doc(alias = "STRETCH_SCALE")]
    #[doc = "Godot enumerator name: `STRETCH_SCALE`"]
    pub const SCALE: StretchMode = StretchMode {
        ord: 0i32
    };
    #[doc(alias = "STRETCH_TILE")]
    #[doc = "Godot enumerator name: `STRETCH_TILE`"]
    pub const TILE: StretchMode = StretchMode {
        ord: 1i32
    };
    #[doc(alias = "STRETCH_KEEP")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP`"]
    pub const KEEP: StretchMode = StretchMode {
        ord: 2i32
    };
    #[doc(alias = "STRETCH_KEEP_CENTERED")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_CENTERED`"]
    pub const KEEP_CENTERED: StretchMode = StretchMode {
        ord: 3i32
    };
    #[doc(alias = "STRETCH_KEEP_ASPECT")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_ASPECT`"]
    pub const KEEP_ASPECT: StretchMode = StretchMode {
        ord: 4i32
    };
    #[doc(alias = "STRETCH_KEEP_ASPECT_CENTERED")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_ASPECT_CENTERED`"]
    pub const KEEP_ASPECT_CENTERED: StretchMode = StretchMode {
        ord: 5i32
    };
    #[doc(alias = "STRETCH_KEEP_ASPECT_COVERED")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_ASPECT_COVERED`"]
    pub const KEEP_ASPECT_COVERED: StretchMode = StretchMode {
        ord: 6i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::SCALE => "SCALE", Self::TILE => "TILE", Self::KEEP => "KEEP", Self::KEEP_CENTERED => "KEEP_CENTERED", Self::KEEP_ASPECT => "KEEP_ASPECT", Self::KEEP_ASPECT_CENTERED => "KEEP_ASPECT_CENTERED", Self::KEEP_ASPECT_COVERED => "KEEP_ASPECT_COVERED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SCALE => "STRETCH_SCALE", Self::TILE => "STRETCH_TILE", Self::KEEP => "STRETCH_KEEP", Self::KEEP_CENTERED => "STRETCH_KEEP_CENTERED", Self::KEEP_ASPECT => "STRETCH_KEEP_ASPECT", Self::KEEP_ASPECT_CENTERED => "STRETCH_KEEP_ASPECT_CENTERED", Self::KEEP_ASPECT_COVERED => "STRETCH_KEEP_ASPECT_COVERED", _ => self.as_str(),
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
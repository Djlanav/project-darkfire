#![doc = "Sidecar module for class [`StyleBoxTexture`][crate::classes::StyleBoxTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StyleBoxTexture` enums](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StyleBoxTexture.`\n\nInherits [`StyleBox`][crate::classes::StyleBox].\n\nRelated symbols:\n\n* [`style_box_texture`][crate::classes::style_box_texture]: sidecar module with related enum/flag types\n* [`IStyleBoxTexture`][crate::classes::IStyleBoxTexture]: virtual methods\n\n\nSee also [Godot docs for `StyleBoxTexture`](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StyleBoxTexture::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StyleBoxTexture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StyleBoxTexture`][crate::classes::StyleBoxTexture].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StyleBoxTexture` methods](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStyleBoxTexture: crate::obj::GodotClass < Base = StyleBoxTexture > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn draw(&self, to_canvas_item: Rid, rect: Rect2,);
        fn get_draw_rect(&self, rect: Rect2,) -> Rect2 {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn test_mask(&self, point: Vector2, rect: Rect2,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl StyleBoxTexture {
        pub fn set_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_margin(&mut self, margin: crate::global::Side, size: f32,) {
            type CallSig = ((), crate::global::Side, f32);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_texture_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_margin_all(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_texture_margin_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_margin(&self, margin: crate::global::Side,) -> f32 {
            type CallSig = (f32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_texture_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_margin(&mut self, margin: crate::global::Side, size: f32,) {
            type CallSig = ((), crate::global::Side, f32);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_expand_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_margin_all(&mut self, size: f32,) {
            type CallSig = ((), f32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_expand_margin_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_expand_margin(&self, margin: crate::global::Side,) -> f32 {
            type CallSig = (f32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_expand_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_region_rect(&mut self, region: Rect2,) {
            type CallSig = ((), Rect2);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_region_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_region_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_region_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_center(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_draw_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_center_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "is_draw_center_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modulate(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_axis_stretch_mode(&mut self, mode: crate::classes::style_box_texture::AxisStretchMode,) {
            type CallSig = ((), crate::classes::style_box_texture::AxisStretchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_h_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_axis_stretch_mode(&self,) -> crate::classes::style_box_texture::AxisStretchMode {
            type CallSig = (crate::classes::style_box_texture::AxisStretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_h_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_axis_stretch_mode(&mut self, mode: crate::classes::style_box_texture::AxisStretchMode,) {
            type CallSig = ((), crate::classes::style_box_texture::AxisStretchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_v_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_axis_stretch_mode(&self,) -> crate::classes::style_box_texture::AxisStretchMode {
            type CallSig = (crate::classes::style_box_texture::AxisStretchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_v_axis_stretch_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for StyleBoxTexture {
        type Base = crate::classes::StyleBox;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"StyleBoxTexture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StyleBoxTexture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StyleBox > for StyleBoxTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for StyleBoxTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StyleBoxTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StyleBoxTexture {
        
    }
    impl crate::obj::cap::GodotDefault for StyleBoxTexture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StyleBoxTexture {
        type Target = crate::classes::StyleBox;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StyleBoxTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StyleBoxTexture`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_StyleBoxTexture {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBoxTexture > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBox > for $Class {
                
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
pub struct AxisStretchMode {
    ord: i32
}
impl AxisStretchMode {
    #[doc(alias = "AXIS_STRETCH_MODE_STRETCH")]
    #[doc = "Godot enumerator name: `AXIS_STRETCH_MODE_STRETCH`"]
    pub const STRETCH: AxisStretchMode = AxisStretchMode {
        ord: 0i32
    };
    #[doc(alias = "AXIS_STRETCH_MODE_TILE")]
    #[doc = "Godot enumerator name: `AXIS_STRETCH_MODE_TILE`"]
    pub const TILE: AxisStretchMode = AxisStretchMode {
        ord: 1i32
    };
    #[doc(alias = "AXIS_STRETCH_MODE_TILE_FIT")]
    #[doc = "Godot enumerator name: `AXIS_STRETCH_MODE_TILE_FIT`"]
    pub const TILE_FIT: AxisStretchMode = AxisStretchMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AxisStretchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AxisStretchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AxisStretchMode {
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
            Self::STRETCH => "STRETCH", Self::TILE => "TILE", Self::TILE_FIT => "TILE_FIT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STRETCH => "AXIS_STRETCH_MODE_STRETCH", Self::TILE => "AXIS_STRETCH_MODE_TILE", Self::TILE_FIT => "AXIS_STRETCH_MODE_TILE_FIT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AxisStretchMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AxisStretchMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AxisStretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
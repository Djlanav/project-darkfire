#![doc = "Sidecar module for class [`TextureProgressBar`][crate::classes::TextureProgressBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextureProgressBar` enums](https://docs.godotengine.org/en/stable/classes/class_textureprogressbar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextureProgressBar.`\n\nInherits [`Range`][crate::classes::Range].\n\nRelated symbols:\n\n* [`texture_progress_bar`][crate::classes::texture_progress_bar]: sidecar module with related enum/flag types\n* [`ITextureProgressBar`][crate::classes::ITextureProgressBar]: virtual methods\n\n\nSee also [Godot docs for `TextureProgressBar`](https://docs.godotengine.org/en/stable/classes/class_textureprogressbar.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TextureProgressBar::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextureProgressBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TextureProgressBar`][crate::classes::TextureProgressBar].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextureProgressBar` methods](https://docs.godotengine.org/en/stable/classes/class_textureprogressbar.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextureProgressBar: crate::obj::GodotClass < Base = TextureProgressBar > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn value_changed(&mut self, new_value: f64,) {
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
    impl TextureProgressBar {
        pub fn set_under_texture(&mut self, tex: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (tex.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8967usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_under_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_under_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8968usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_under_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_progress_texture(&mut self, tex: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (tex.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8969usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_progress_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_progress_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8970usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_progress_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_over_texture(&mut self, tex: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (tex.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8971usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_over_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_over_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8972usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_over_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill_mode(&mut self, mode: i32,) {
            type CallSig = ((), i32);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8973usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_fill_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill_mode(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_fill_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tint_under(&mut self, tint: Color,) {
            type CallSig = ((), Color);
            let args = (tint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_tint_under", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tint_under(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_tint_under", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tint_progress(&mut self, tint: Color,) {
            type CallSig = ((), Color);
            let args = (tint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_tint_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tint_progress(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_tint_progress", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tint_over(&mut self, tint: Color,) {
            type CallSig = ((), Color);
            let args = (tint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_tint_over", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tint_over(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_tint_over", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_progress_offset(&mut self, offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_texture_progress_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_progress_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_texture_progress_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_radial_initial_angle(&mut self, mode: f32,) {
            type CallSig = ((), f32);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_radial_initial_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radial_initial_angle(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_radial_initial_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_radial_center_offset(&mut self, mode: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_radial_center_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_radial_center_offset(&mut self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_radial_center_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fill_degrees(&mut self, mode: f32,) {
            type CallSig = ((), f32);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_fill_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fill_degrees(&mut self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_fill_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stretch_margin(&mut self, margin: crate::global::Side, value: i32,) {
            type CallSig = ((), crate::global::Side, i32);
            let args = (margin, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_stretch_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stretch_margin(&self, margin: crate::global::Side,) -> i32 {
            type CallSig = (i32, crate::global::Side);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_stretch_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_nine_patch_stretch(&mut self, stretch: bool,) {
            type CallSig = ((), bool);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "set_nine_patch_stretch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_nine_patch_stretch(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TextureProgressBar", "get_nine_patch_stretch", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TextureProgressBar {
        type Base = crate::classes::Range;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TextureProgressBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextureProgressBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Range > for TextureProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TextureProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TextureProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TextureProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextureProgressBar {
        
    }
    impl crate::obj::cap::GodotDefault for TextureProgressBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextureProgressBar {
        type Target = crate::classes::Range;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextureProgressBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextureProgressBar`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TextureProgressBar {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextureProgressBar > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Range > for $Class {
                
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
pub struct FillMode {
    ord: i32
}
impl FillMode {
    #[doc(alias = "FILL_LEFT_TO_RIGHT")]
    #[doc = "Godot enumerator name: `FILL_LEFT_TO_RIGHT`"]
    pub const LEFT_TO_RIGHT: FillMode = FillMode {
        ord: 0i32
    };
    #[doc(alias = "FILL_RIGHT_TO_LEFT")]
    #[doc = "Godot enumerator name: `FILL_RIGHT_TO_LEFT`"]
    pub const RIGHT_TO_LEFT: FillMode = FillMode {
        ord: 1i32
    };
    #[doc(alias = "FILL_TOP_TO_BOTTOM")]
    #[doc = "Godot enumerator name: `FILL_TOP_TO_BOTTOM`"]
    pub const TOP_TO_BOTTOM: FillMode = FillMode {
        ord: 2i32
    };
    #[doc(alias = "FILL_BOTTOM_TO_TOP")]
    #[doc = "Godot enumerator name: `FILL_BOTTOM_TO_TOP`"]
    pub const BOTTOM_TO_TOP: FillMode = FillMode {
        ord: 3i32
    };
    #[doc(alias = "FILL_CLOCKWISE")]
    #[doc = "Godot enumerator name: `FILL_CLOCKWISE`"]
    pub const CLOCKWISE: FillMode = FillMode {
        ord: 4i32
    };
    #[doc(alias = "FILL_COUNTER_CLOCKWISE")]
    #[doc = "Godot enumerator name: `FILL_COUNTER_CLOCKWISE`"]
    pub const COUNTER_CLOCKWISE: FillMode = FillMode {
        ord: 5i32
    };
    #[doc(alias = "FILL_BILINEAR_LEFT_AND_RIGHT")]
    #[doc = "Godot enumerator name: `FILL_BILINEAR_LEFT_AND_RIGHT`"]
    pub const BILINEAR_LEFT_AND_RIGHT: FillMode = FillMode {
        ord: 6i32
    };
    #[doc(alias = "FILL_BILINEAR_TOP_AND_BOTTOM")]
    #[doc = "Godot enumerator name: `FILL_BILINEAR_TOP_AND_BOTTOM`"]
    pub const BILINEAR_TOP_AND_BOTTOM: FillMode = FillMode {
        ord: 7i32
    };
    #[doc(alias = "FILL_CLOCKWISE_AND_COUNTER_CLOCKWISE")]
    #[doc = "Godot enumerator name: `FILL_CLOCKWISE_AND_COUNTER_CLOCKWISE`"]
    pub const CLOCKWISE_AND_COUNTER_CLOCKWISE: FillMode = FillMode {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for FillMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FillMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FillMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::LEFT_TO_RIGHT => "LEFT_TO_RIGHT", Self::RIGHT_TO_LEFT => "RIGHT_TO_LEFT", Self::TOP_TO_BOTTOM => "TOP_TO_BOTTOM", Self::BOTTOM_TO_TOP => "BOTTOM_TO_TOP", Self::CLOCKWISE => "CLOCKWISE", Self::COUNTER_CLOCKWISE => "COUNTER_CLOCKWISE", Self::BILINEAR_LEFT_AND_RIGHT => "BILINEAR_LEFT_AND_RIGHT", Self::BILINEAR_TOP_AND_BOTTOM => "BILINEAR_TOP_AND_BOTTOM", Self::CLOCKWISE_AND_COUNTER_CLOCKWISE => "CLOCKWISE_AND_COUNTER_CLOCKWISE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LEFT_TO_RIGHT => "FILL_LEFT_TO_RIGHT", Self::RIGHT_TO_LEFT => "FILL_RIGHT_TO_LEFT", Self::TOP_TO_BOTTOM => "FILL_TOP_TO_BOTTOM", Self::BOTTOM_TO_TOP => "FILL_BOTTOM_TO_TOP", Self::CLOCKWISE => "FILL_CLOCKWISE", Self::COUNTER_CLOCKWISE => "FILL_COUNTER_CLOCKWISE", Self::BILINEAR_LEFT_AND_RIGHT => "FILL_BILINEAR_LEFT_AND_RIGHT", Self::BILINEAR_TOP_AND_BOTTOM => "FILL_BILINEAR_TOP_AND_BOTTOM", Self::CLOCKWISE_AND_COUNTER_CLOCKWISE => "FILL_CLOCKWISE_AND_COUNTER_CLOCKWISE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FillMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FillMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FillMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
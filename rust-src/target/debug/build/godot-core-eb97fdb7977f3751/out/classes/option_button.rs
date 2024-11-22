#![doc = "Sidecar module for class [`OptionButton`][crate::classes::OptionButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OptionButton` enums](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OptionButton.`\n\nInherits [`Button`][crate::classes::Button].\n\nRelated symbols:\n\n* [`option_button`][crate::classes::option_button]: sidecar module with related enum/flag types\n* [`IOptionButton`][crate::classes::IOptionButton]: virtual methods\n\n\nSee also [Godot docs for `OptionButton`](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OptionButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OptionButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OptionButton`][crate::classes::OptionButton].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OptionButton` methods](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOptionButton: crate::obj::GodotClass < Base = OptionButton > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OptionButton {
        pub(crate) fn add_item_full(&mut self, label: CowArg < GString >, id: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "add_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_item(&mut self, label: impl AsArg < GString >,) {
            self.add_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a,) -> ExAddItem < 'a > {
            ExAddItem::new(self, label,)
        }
        pub(crate) fn add_icon_item_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, id: i32,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, i32);
            let args = (texture, label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "add_icon_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_item(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) {
            self.add_icon_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex < 'a > (&'a mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExAddIconItem < 'a > {
            ExAddIconItem::new(self, texture, label,)
        }
        pub fn set_item_text(&mut self, idx: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, idx: i32, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (idx, texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_id(&mut self, idx: i32, id: i32,) {
            type CallSig = ((), i32, i32);
            let args = (idx, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_metadata(&mut self, idx: i32, metadata: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_id(&self, idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_separator(&self, idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "is_item_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_separator_full(&mut self, text: CowArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "add_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_separator_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_separator(&mut self,) {
            self.add_separator_ex() . done()
        }
        #[inline]
        pub fn add_separator_ex < 'a > (&'a mut self,) -> ExAddSeparator < 'a > {
            ExAddSeparator::new(self,)
        }
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select(&mut self, idx: i32,) {
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_id(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_metadata(&self,) -> Variant {
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, idx: i32,) {
            type CallSig = ((), i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallSig = (Option < Gd < crate::classes::PopupMenu > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn show_popup(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "show_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_selectable_items(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "has_selectable_items", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_selectable_item_full(&self, from_last: bool,) -> i32 {
            type CallSig = (i32, bool);
            let args = (from_last,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_selectable_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selectable_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selectable_item(&self,) -> i32 {
            self.get_selectable_item_ex() . done()
        }
        #[inline]
        pub fn get_selectable_item_ex < 'a > (&'a self,) -> ExGetSelectableItem < 'a > {
            ExGetSelectableItem::new(self,)
        }
        pub fn set_fit_to_longest_item(&mut self, fit: bool,) {
            type CallSig = ((), bool);
            let args = (fit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_fit_to_longest_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_fit_to_longest_item(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "is_fit_to_longest_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "get_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_shortcuts(&mut self, disabled: bool,) {
            type CallSig = ((), bool);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "OptionButton", "set_disable_shortcuts", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OptionButton {
        type Base = crate::classes::Button;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OptionButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OptionButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Button > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BaseButton > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OptionButton {
        
    }
    impl crate::obj::cap::GodotDefault for OptionButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OptionButton {
        type Target = crate::classes::Button;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OptionButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OptionButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_OptionButton {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OptionButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Button > for $Class {
                
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
#[doc = "Default-param extender for [`OptionButton::add_item_ex`][super::OptionButton::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OptionButton, label: CowArg < 'a, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id,
        }
        = self;
        re_export::OptionButton::add_item_full(surround_object, label, id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_icon_item_ex`][super::OptionButton::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OptionButton, texture: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), label: label.into_arg(), id: id,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id,
        }
        = self;
        re_export::OptionButton::add_icon_item_full(surround_object, texture.cow_as_object_arg(), label, id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_separator_ex`][super::OptionButton::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OptionButton, text: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton,) -> Self {
        let text = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: CowArg::Owned(text),
        }
    }
    #[inline]
    pub fn text(self, text: impl AsArg < GString > + 'a) -> Self {
        Self {
            text: text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text,
        }
        = self;
        re_export::OptionButton::add_separator_full(surround_object, text,)
    }
}
#[doc = "Default-param extender for [`OptionButton::get_selectable_item_ex`][super::OptionButton::get_selectable_item_ex]."]
#[must_use]
pub struct ExGetSelectableItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::OptionButton, from_last: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectableItem < 'a > {
    fn new(surround_object: &'a re_export::OptionButton,) -> Self {
        let from_last = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_last: from_last,
        }
    }
    #[inline]
    pub fn from_last(self, from_last: bool) -> Self {
        Self {
            from_last: from_last, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, from_last,
        }
        = self;
        re_export::OptionButton::get_selectable_item_full(surround_object, from_last,)
    }
}
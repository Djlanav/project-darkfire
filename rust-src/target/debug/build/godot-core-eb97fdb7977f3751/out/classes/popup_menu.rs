#![doc = "Sidecar module for class [`PopupMenu`][crate::classes::PopupMenu].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PopupMenu` enums](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PopupMenu.`\n\nInherits [`Popup`][crate::classes::Popup].\n\nRelated symbols:\n\n* [`popup_menu`][crate::classes::popup_menu]: sidecar module with related enum/flag types\n* [`IPopupMenu`][crate::classes::IPopupMenu]: virtual methods\n\n\nSee also [Godot docs for `PopupMenu`](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PopupMenu::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PopupMenu {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PopupMenu`][crate::classes::PopupMenu].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PopupMenu` methods](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPopupMenu: crate::obj::GodotClass < Base = PopupMenu > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
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
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl PopupMenu {
        pub(crate) fn activate_item_by_event_full(&mut self, event: ObjectArg < crate::classes::InputEvent >, for_global_only: bool,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::InputEvent >, bool);
            let args = (event, for_global_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "activate_item_by_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::activate_item_by_event_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn activate_item_by_event(&mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) -> bool {
            self.activate_item_by_event_ex(event,) . done()
        }
        #[inline]
        pub fn activate_item_by_event_ex < 'a > (&'a mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) -> ExActivateItemByEvent < 'a > {
            ExActivateItemByEvent::new(self, event,)
        }
        pub fn set_prefer_native_menu(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_prefer_native_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_prefer_native_menu(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_prefer_native_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_native_menu(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_native_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_item_full(&mut self, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_item", self.object_ptr, self.__checked_id(), args,)
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
        pub(crate) fn add_icon_item_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_item", self.object_ptr, self.__checked_id(), args,)
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
        pub(crate) fn add_check_item_full(&mut self, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_check_item(&mut self, label: impl AsArg < GString >,) {
            self.add_check_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_check_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a,) -> ExAddCheckItem < 'a > {
            ExAddCheckItem::new(self, label,)
        }
        pub(crate) fn add_icon_check_item_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_check_item(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) {
            self.add_icon_check_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_check_item_ex < 'a > (&'a mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExAddIconCheckItem < 'a > {
            ExAddIconCheckItem::new(self, texture, label,)
        }
        pub(crate) fn add_radio_check_item_full(&mut self, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_radio_check_item(&mut self, label: impl AsArg < GString >,) {
            self.add_radio_check_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_radio_check_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a,) -> ExAddRadioCheckItem < 'a > {
            ExAddRadioCheckItem::new(self, label,)
        }
        pub(crate) fn add_icon_radio_check_item_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Texture2D >, CowArg < 'a0, GString >, i32, crate::global::Key);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_radio_check_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_radio_check_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_radio_check_item(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString >,) {
            self.add_icon_radio_check_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_radio_check_item_ex < 'a > (&'a mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> ExAddIconRadioCheckItem < 'a > {
            ExAddIconRadioCheckItem::new(self, texture, label,)
        }
        pub(crate) fn add_multistate_item_full(&mut self, label: CowArg < GString >, max_states: i32, default_state: i32, id: i32, accel: crate::global::Key,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32, i32, i32, crate::global::Key);
            let args = (label, max_states, default_state, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_multistate_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_multistate_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_multistate_item(&mut self, label: impl AsArg < GString >, max_states: i32,) {
            self.add_multistate_item_ex(label, max_states,) . done()
        }
        #[inline]
        pub fn add_multistate_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a, max_states: i32,) -> ExAddMultistateItem < 'a > {
            ExAddMultistateItem::new(self, label, max_states,)
        }
        pub(crate) fn add_shortcut_full(&mut self, shortcut: ObjectArg < crate::classes::Shortcut >, id: i32, global: bool, allow_echo: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Shortcut >, i32, bool, bool);
            let args = (shortcut, id, global, allow_echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_shortcut(&mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.add_shortcut_ex(shortcut,) . done()
        }
        #[inline]
        pub fn add_shortcut_ex < 'a > (&'a mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExAddShortcut < 'a > {
            ExAddShortcut::new(self, shortcut,)
        }
        pub(crate) fn add_icon_shortcut_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, shortcut: ObjectArg < crate::classes::Shortcut >, id: i32, global: bool, allow_echo: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >, ObjectArg < crate::classes::Shortcut >, i32, bool, bool);
            let args = (texture, shortcut, id, global, allow_echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_shortcut(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.add_icon_shortcut_ex(texture, shortcut,) . done()
        }
        #[inline]
        pub fn add_icon_shortcut_ex < 'a > (&'a mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExAddIconShortcut < 'a > {
            ExAddIconShortcut::new(self, texture, shortcut,)
        }
        pub(crate) fn add_check_shortcut_full(&mut self, shortcut: ObjectArg < crate::classes::Shortcut >, id: i32, global: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Shortcut >, i32, bool);
            let args = (shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_check_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_check_shortcut(&mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.add_check_shortcut_ex(shortcut,) . done()
        }
        #[inline]
        pub fn add_check_shortcut_ex < 'a > (&'a mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExAddCheckShortcut < 'a > {
            ExAddCheckShortcut::new(self, shortcut,)
        }
        pub(crate) fn add_icon_check_shortcut_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, shortcut: ObjectArg < crate::classes::Shortcut >, id: i32, global: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >, ObjectArg < crate::classes::Shortcut >, i32, bool);
            let args = (texture, shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_check_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_check_shortcut(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.add_icon_check_shortcut_ex(texture, shortcut,) . done()
        }
        #[inline]
        pub fn add_icon_check_shortcut_ex < 'a > (&'a mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExAddIconCheckShortcut < 'a > {
            ExAddIconCheckShortcut::new(self, texture, shortcut,)
        }
        pub(crate) fn add_radio_check_shortcut_full(&mut self, shortcut: ObjectArg < crate::classes::Shortcut >, id: i32, global: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Shortcut >, i32, bool);
            let args = (shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_radio_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_radio_check_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_radio_check_shortcut(&mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.add_radio_check_shortcut_ex(shortcut,) . done()
        }
        #[inline]
        pub fn add_radio_check_shortcut_ex < 'a > (&'a mut self, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExAddRadioCheckShortcut < 'a > {
            ExAddRadioCheckShortcut::new(self, shortcut,)
        }
        pub(crate) fn add_icon_radio_check_shortcut_full(&mut self, texture: ObjectArg < crate::classes::Texture2D >, shortcut: ObjectArg < crate::classes::Shortcut >, id: i32, global: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >, ObjectArg < crate::classes::Shortcut >, i32, bool);
            let args = (texture, shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_radio_check_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_radio_check_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_radio_check_shortcut(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.add_icon_radio_check_shortcut_ex(texture, shortcut,) . done()
        }
        #[inline]
        pub fn add_icon_radio_check_shortcut_ex < 'a > (&'a mut self, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExAddIconRadioCheckShortcut < 'a > {
            ExAddIconRadioCheckShortcut::new(self, texture, shortcut,)
        }
        pub(crate) fn add_submenu_item_full(&mut self, label: CowArg < GString >, submenu: CowArg < GString >, id: i32,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32);
            let args = (label, submenu, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_submenu_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_submenu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_submenu_item(&mut self, label: impl AsArg < GString >, submenu: impl AsArg < GString >,) {
            self.add_submenu_item_ex(label, submenu,) . done()
        }
        #[inline]
        pub fn add_submenu_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a, submenu: impl AsArg < GString > + 'a,) -> ExAddSubmenuItem < 'a > {
            ExAddSubmenuItem::new(self, label, submenu,)
        }
        pub(crate) fn add_submenu_node_item_full(&mut self, label: CowArg < GString >, submenu: ObjectArg < crate::classes::PopupMenu >, id: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, ObjectArg < crate::classes::PopupMenu >, i32);
            let args = (label, submenu, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_submenu_node_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_submenu_node_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_submenu_node_item(&mut self, label: impl AsArg < GString >, submenu: impl AsObjectArg < crate::classes::PopupMenu >,) {
            self.add_submenu_node_item_ex(label, submenu,) . done()
        }
        #[inline]
        pub fn add_submenu_node_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a, submenu: impl AsObjectArg < crate::classes::PopupMenu >,) -> ExAddSubmenuNodeItem < 'a > {
            ExAddSubmenuNodeItem::new(self, label, submenu,)
        }
        pub fn set_item_text(&mut self, index: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (index, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_text_direction(&mut self, index: i32, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), i32, crate::classes::control::TextDirection);
            let args = (index, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_language(&mut self, index: i32, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (index, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon(&mut self, index: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (index, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_max_width(&mut self, index: i32, width: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_icon_modulate(&mut self, index: i32, modulate: Color,) {
            type CallSig = ((), i32, Color);
            let args = (index, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_checked(&mut self, index: i32, checked: bool,) {
            type CallSig = ((), i32, bool);
            let args = (index, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_id(&mut self, index: i32, id: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_accelerator(&mut self, index: i32, accel: crate::global::Key,) {
            type CallSig = ((), i32, crate::global::Key);
            let args = (index, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_metadata(&mut self, index: i32, metadata: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (index, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_disabled(&mut self, index: i32, disabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_submenu(&mut self, index: i32, submenu: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (index, submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_submenu_node(&mut self, index: i32, submenu: impl AsObjectArg < crate::classes::PopupMenu >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::PopupMenu >);
            let args = (index, submenu.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_submenu_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_as_separator(&mut self, index: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_as_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_as_checkable(&mut self, index: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_as_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_as_radio_checkable(&mut self, index: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_as_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, index: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (index, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_item_shortcut_full(&mut self, index: i32, shortcut: ObjectArg < crate::classes::Shortcut >, global: bool,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Shortcut >, bool);
            let args = (index, shortcut, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_item_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_item_shortcut(&mut self, index: i32, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) {
            self.set_item_shortcut_ex(index, shortcut,) . done()
        }
        #[inline]
        pub fn set_item_shortcut_ex < 'a > (&'a mut self, index: i32, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> ExSetItemShortcut < 'a > {
            ExSetItemShortcut::new(self, index, shortcut,)
        }
        pub fn set_item_indent(&mut self, index: i32, indent: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index, indent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_multistate(&mut self, index: i32, state: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_multistate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_multistate_max(&mut self, index: i32, max_states: i32,) {
            type CallSig = ((), i32, i32);
            let args = (index, max_states,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_multistate_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_shortcut_disabled(&mut self, index: i32, disabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_shortcut_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_item_checked(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "toggle_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_item_multistate(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "toggle_item_multistate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text(&self, index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_text_direction(&self, index: i32,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_language(&self, index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon(&self, index: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_max_width(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_icon_modulate(&self, index: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_checked(&self, index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_id(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_accelerator(&self, index: i32,) -> crate::global::Key {
            type CallSig = (crate::global::Key, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_accelerator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_metadata(&self, index: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_disabled(&self, index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_submenu(&self, index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_submenu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_submenu_node(&self, index: i32,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallSig = (Option < Gd < crate::classes::PopupMenu > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_submenu_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_separator(&self, index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_separator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_checkable(&self, index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_radio_checkable(&self, index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_radio_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_item_shortcut_disabled(&self, index: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_shortcut_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_tooltip(&self, index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_shortcut(&self, index: i32,) -> Option < Gd < crate::classes::Shortcut > > {
            type CallSig = (Option < Gd < crate::classes::Shortcut > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_shortcut", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_indent(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_multistate_max(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_multistate_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_multistate(&self, index: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_multistate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focused_item(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_focused_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focused_item(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_focused_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn scroll_to_item(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "scroll_to_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_item(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "remove_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_separator_full(&mut self, label: CowArg < GString >, id: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, i32);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "add_separator", self.object_ptr, self.__checked_id(), args,)
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
        pub(crate) fn clear_full(&mut self, free_submenus: bool,) {
            type CallSig = ((), bool);
            let args = (free_submenus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear(&mut self,) {
            self.clear_ex() . done()
        }
        #[inline]
        pub fn clear_ex < 'a > (&'a mut self,) -> ExClear < 'a > {
            ExClear::new(self,)
        }
        pub fn set_hide_on_item_selection(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_hide_on_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hide_on_item_selection(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_hide_on_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_on_checkable_item_selection(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_hide_on_checkable_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hide_on_checkable_item_selection(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_hide_on_checkable_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_on_state_item_selection(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_hide_on_state_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hide_on_state_item_selection(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_hide_on_state_item_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_submenu_popup_delay(&mut self, seconds: f32,) {
            type CallSig = ((), f32);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_submenu_popup_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_submenu_popup_delay(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_submenu_popup_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_system_menu(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "is_system_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_system_menu(&mut self, system_menu_id: crate::classes::native_menu::SystemMenus,) {
            type CallSig = ((), crate::classes::native_menu::SystemMenus);
            let args = (system_menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "set_system_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_menu(&self,) -> crate::classes::native_menu::SystemMenus {
            type CallSig = (crate::classes::native_menu::SystemMenus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "PopupMenu", "get_system_menu", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PopupMenu {
        type Base = crate::classes::Popup;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"PopupMenu"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PopupMenu {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Popup > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PopupMenu {
        
    }
    impl crate::obj::cap::GodotDefault for PopupMenu {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PopupMenu {
        type Target = crate::classes::Popup;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PopupMenu {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PopupMenu`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_PopupMenu {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PopupMenu > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Popup > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PopupMenu::activate_item_by_event_ex`][super::PopupMenu::activate_item_by_event_ex]."]
#[must_use]
pub struct ExActivateItemByEvent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, event: ObjectCow < crate::classes::InputEvent >, for_global_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExActivateItemByEvent < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, event: impl AsObjectArg < crate::classes::InputEvent >,) -> Self {
        let for_global_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.consume_arg(), for_global_only: for_global_only,
        }
    }
    #[inline]
    pub fn for_global_only(self, for_global_only: bool) -> Self {
        Self {
            for_global_only: for_global_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, event, for_global_only,
        }
        = self;
        re_export::PopupMenu::activate_item_by_event_full(surround_object, event.cow_as_object_arg(), for_global_only,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_item_ex`][super::PopupMenu::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_item_full(surround_object, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_item_ex`][super::PopupMenu::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, texture: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_icon_item_full(surround_object, texture.cow_as_object_arg(), label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_check_item_ex`][super::PopupMenu::add_check_item_ex]."]
#[must_use]
pub struct ExAddCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_check_item_full(surround_object, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_check_item_ex`][super::PopupMenu::add_icon_check_item_ex]."]
#[must_use]
pub struct ExAddIconCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, texture: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_icon_check_item_full(surround_object, texture.cow_as_object_arg(), label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_radio_check_item_ex`][super::PopupMenu::add_radio_check_item_ex]."]
#[must_use]
pub struct ExAddRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_radio_check_item_full(surround_object, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_radio_check_item_ex`][super::PopupMenu::add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, texture: ObjectCow < crate::classes::Texture2D >, label: CowArg < 'a, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconRadioCheckItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: impl AsObjectArg < crate::classes::Texture2D >, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_icon_radio_check_item_full(surround_object, texture.cow_as_object_arg(), label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_multistate_item_ex`][super::PopupMenu::add_multistate_item_ex]."]
#[must_use]
pub struct ExAddMultistateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, max_states: i32, default_state: i32, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMultistateItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: impl AsArg < GString > + 'a, max_states: i32,) -> Self {
        let default_state = 0i32;
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), max_states: max_states, default_state: default_state, id: id, accel: accel,
        }
    }
    #[inline]
    pub fn default_state(self, default_state: i32) -> Self {
        Self {
            default_state: default_state, .. self
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, max_states, default_state, id, accel,
        }
        = self;
        re_export::PopupMenu::add_multistate_item_full(surround_object, label, max_states, default_state, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_shortcut_ex`][super::PopupMenu::add_shortcut_ex]."]
#[must_use]
pub struct ExAddShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, shortcut: ObjectCow < crate::classes::Shortcut >, id: i32, global: bool, allow_echo: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let id = - 1i32;
        let global = false;
        let allow_echo = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shortcut: shortcut.consume_arg(), id: id, global: global, allow_echo: allow_echo,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shortcut, id, global, allow_echo,
        }
        = self;
        re_export::PopupMenu::add_shortcut_full(surround_object, shortcut.cow_as_object_arg(), id, global, allow_echo,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_shortcut_ex`][super::PopupMenu::add_icon_shortcut_ex]."]
#[must_use]
pub struct ExAddIconShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, texture: ObjectCow < crate::classes::Texture2D >, shortcut: ObjectCow < crate::classes::Shortcut >, id: i32, global: bool, allow_echo: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let id = - 1i32;
        let global = false;
        let allow_echo = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), shortcut: shortcut.consume_arg(), id: id, global: global, allow_echo: allow_echo,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, shortcut, id, global, allow_echo,
        }
        = self;
        re_export::PopupMenu::add_icon_shortcut_full(surround_object, texture.cow_as_object_arg(), shortcut.cow_as_object_arg(), id, global, allow_echo,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_check_shortcut_ex`][super::PopupMenu::add_check_shortcut_ex]."]
#[must_use]
pub struct ExAddCheckShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, shortcut: ObjectCow < crate::classes::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shortcut: shortcut.consume_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_check_shortcut_full(surround_object, shortcut.cow_as_object_arg(), id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_check_shortcut_ex`][super::PopupMenu::add_icon_check_shortcut_ex]."]
#[must_use]
pub struct ExAddIconCheckShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, texture: ObjectCow < crate::classes::Texture2D >, shortcut: ObjectCow < crate::classes::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), shortcut: shortcut.consume_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_icon_check_shortcut_full(surround_object, texture.cow_as_object_arg(), shortcut.cow_as_object_arg(), id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_radio_check_shortcut_ex`][super::PopupMenu::add_radio_check_shortcut_ex]."]
#[must_use]
pub struct ExAddRadioCheckShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, shortcut: ObjectCow < crate::classes::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddRadioCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shortcut: shortcut.consume_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_radio_check_shortcut_full(surround_object, shortcut.cow_as_object_arg(), id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_radio_check_shortcut_ex`][super::PopupMenu::add_icon_radio_check_shortcut_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, texture: ObjectCow < crate::classes::Texture2D >, shortcut: ObjectCow < crate::classes::Shortcut >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconRadioCheckShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, texture: impl AsObjectArg < crate::classes::Texture2D >, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.consume_arg(), shortcut: shortcut.consume_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_icon_radio_check_shortcut_full(surround_object, texture.cow_as_object_arg(), shortcut.cow_as_object_arg(), id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_submenu_item_ex`][super::PopupMenu::add_submenu_item_ex]."]
#[must_use]
pub struct ExAddSubmenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, submenu: CowArg < 'a, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: impl AsArg < GString > + 'a, submenu: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), submenu: submenu.into_arg(), id: id,
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
            _phantom, surround_object, label, submenu, id,
        }
        = self;
        re_export::PopupMenu::add_submenu_item_full(surround_object, label, submenu, id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_submenu_node_item_ex`][super::PopupMenu::add_submenu_node_item_ex]."]
#[must_use]
pub struct ExAddSubmenuNodeItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, submenu: ObjectCow < crate::classes::PopupMenu >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSubmenuNodeItem < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, label: impl AsArg < GString > + 'a, submenu: impl AsObjectArg < crate::classes::PopupMenu >,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), submenu: submenu.consume_arg(), id: id,
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
            _phantom, surround_object, label, submenu, id,
        }
        = self;
        re_export::PopupMenu::add_submenu_node_item_full(surround_object, label, submenu.cow_as_object_arg(), id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::set_item_shortcut_ex`][super::PopupMenu::set_item_shortcut_ex]."]
#[must_use]
pub struct ExSetItemShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, index: i32, shortcut: ObjectCow < crate::classes::Shortcut >, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetItemShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu, index: i32, shortcut: impl AsObjectArg < crate::classes::Shortcut >,) -> Self {
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index, shortcut: shortcut.consume_arg(), global: global,
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index, shortcut, global,
        }
        = self;
        re_export::PopupMenu::set_item_shortcut_full(surround_object, index, shortcut.cow_as_object_arg(), global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_separator_ex`][super::PopupMenu::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, label: CowArg < 'a, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu,) -> Self {
        let label = GString::from("");
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: CowArg::Owned(label), id: id,
        }
    }
    #[inline]
    pub fn label(self, label: impl AsArg < GString > + 'a) -> Self {
        Self {
            label: label.into_arg(), .. self
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
        re_export::PopupMenu::add_separator_full(surround_object, label, id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::clear_ex`][super::PopupMenu::clear_ex]."]
#[must_use]
pub struct ExClear < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PopupMenu, free_submenus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClear < 'a > {
    fn new(surround_object: &'a mut re_export::PopupMenu,) -> Self {
        let free_submenus = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, free_submenus: free_submenus,
        }
    }
    #[inline]
    pub fn free_submenus(self, free_submenus: bool) -> Self {
        Self {
            free_submenus: free_submenus, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, free_submenus,
        }
        = self;
        re_export::PopupMenu::clear_full(surround_object, free_submenus,)
    }
}
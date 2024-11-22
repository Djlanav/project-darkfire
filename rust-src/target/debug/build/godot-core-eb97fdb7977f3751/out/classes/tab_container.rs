#![doc = "Sidecar module for class [`TabContainer`][crate::classes::TabContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TabContainer` enums](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TabContainer.`\n\nInherits [`Container`][crate::classes::Container].\n\nRelated symbols:\n\n* [`tab_container`][crate::classes::tab_container]: sidecar module with related enum/flag types\n* [`ITabContainer`][crate::classes::ITabContainer]: virtual methods\n\n\nSee also [Godot docs for `TabContainer`](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TabContainer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TabContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TabContainer`][crate::classes::TabContainer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TabContainer` methods](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITabContainer: crate::obj::GodotClass < Base = TabContainer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TabContainer {
        pub fn get_tab_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8286usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_tab(&mut self, tab_idx: i32,) {
            type CallSig = ((), i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8287usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_tab(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8288usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_current_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_previous_tab(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8289usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_previous_tab", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_previous_available(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8290usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "select_previous_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_next_available(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "select_next_available", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_tab_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_current_tab_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_bar(&self,) -> Option < Gd < crate::classes::TabBar > > {
            type CallSig = (Option < Gd < crate::classes::TabBar > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_bar", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_control(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_alignment(&mut self, alignment: crate::classes::tab_bar::AlignmentMode,) {
            type CallSig = ((), crate::classes::tab_bar::AlignmentMode);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_alignment(&self,) -> crate::classes::tab_bar::AlignmentMode {
            type CallSig = (crate::classes::tab_bar::AlignmentMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_position(&mut self, tabs_position: crate::classes::tab_container::TabPosition,) {
            type CallSig = ((), crate::classes::tab_container::TabPosition);
            let args = (tabs_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tabs_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tabs_position(&self,) -> crate::classes::tab_container::TabPosition {
            type CallSig = (crate::classes::tab_container::TabPosition,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tabs_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_tabs(&mut self, clip_tabs: bool,) {
            type CallSig = ((), bool);
            let args = (clip_tabs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_clip_tabs(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_clip_tabs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tabs_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_tabs_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "are_tabs_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_all_tabs_in_front(&mut self, is_front: bool,) {
            type CallSig = ((), bool);
            let args = (is_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_all_tabs_in_front", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_all_tabs_in_front(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "is_all_tabs_in_front", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_title(&mut self, tab_idx: i32, title: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (tab_idx, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_title(&self, tab_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_tooltip(&mut self, tab_idx: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (tab_idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_tooltip(&self, tab_idx: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon(&mut self, tab_idx: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (tab_idx, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_icon_max_width(&mut self, tab_idx: i32, width: i32,) {
            type CallSig = ((), i32, i32);
            let args = (tab_idx, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_icon_max_width(&self, tab_idx: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_disabled(&mut self, tab_idx: i32, disabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (tab_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_disabled(&self, tab_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "is_tab_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_hidden(&mut self, tab_idx: i32, hidden: bool,) {
            type CallSig = ((), i32, bool);
            let args = (tab_idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_tab_hidden(&self, tab_idx: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "is_tab_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_metadata(&mut self, tab_idx: i32, metadata: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (tab_idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_metadata(&self, tab_idx: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_button_icon(&mut self, tab_idx: i32, icon: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (tab_idx, icon.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_button_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_button_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_idx_at_point(&self, point: Vector2,) -> i32 {
            type CallSig = (i32, Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_idx_at_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_idx_from_control(&self, control: impl AsObjectArg < crate::classes::Control >,) -> i32 {
            type CallSig = (i32, ObjectArg < crate::classes::Control >);
            let args = (control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_idx_from_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_popup(&mut self, popup: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (popup.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_popup(&self,) -> Option < Gd < crate::classes::Popup > > {
            type CallSig = (Option < Gd < crate::classes::Popup > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_popup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_to_rearrange_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drag_to_rearrange_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_drag_to_rearrange_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tabs_rearrange_group(&mut self, group_id: i32,) {
            type CallSig = ((), i32);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tabs_rearrange_group(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tabs_rearrange_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_hidden_tabs_for_min_size(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_use_hidden_tabs_for_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_hidden_tabs_for_min_size(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_use_hidden_tabs_for_min_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tab_focus_mode(&mut self, focus_mode: crate::classes::control::FocusMode,) {
            type CallSig = ((), crate::classes::control::FocusMode);
            let args = (focus_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tab_focus_mode(&self,) -> crate::classes::control::FocusMode {
            type CallSig = (crate::classes::control::FocusMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deselect_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "set_deselect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_deselect_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TabContainer", "get_deselect_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TabContainer {
        type Base = crate::classes::Container;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TabContainer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TabContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TabContainer {
        
    }
    impl crate::obj::cap::GodotDefault for TabContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TabContainer {
        type Target = crate::classes::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TabContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TabContainer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TabContainer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TabContainer > for $Class {
                
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
pub struct TabPosition {
    ord: i32
}
impl TabPosition {
    #[doc(alias = "POSITION_TOP")]
    #[doc = "Godot enumerator name: `POSITION_TOP`"]
    pub const TOP: TabPosition = TabPosition {
        ord: 0i32
    };
    #[doc(alias = "POSITION_BOTTOM")]
    #[doc = "Godot enumerator name: `POSITION_BOTTOM`"]
    pub const BOTTOM: TabPosition = TabPosition {
        ord: 1i32
    };
    #[doc(alias = "POSITION_MAX")]
    #[doc = "Godot enumerator name: `POSITION_MAX`"]
    pub const MAX: TabPosition = TabPosition {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TabPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TabPosition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TabPosition {
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
            Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TOP => "POSITION_TOP", Self::BOTTOM => "POSITION_BOTTOM", Self::MAX => "POSITION_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TabPosition {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for TabPosition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TabPosition {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TabPosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#![doc = "Sidecar module for class [`AcceptDialog`][crate::classes::AcceptDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AcceptDialog` enums](https://docs.godotengine.org/en/stable/classes/class_acceptdialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AcceptDialog.`\n\nInherits [`Window`][crate::classes::Window].\n\nRelated symbols:\n\n* [`accept_dialog`][crate::classes::accept_dialog]: sidecar module with related enum/flag types\n* [`IAcceptDialog`][crate::classes::IAcceptDialog]: virtual methods\n\n\nSee also [Godot docs for `AcceptDialog`](https://docs.godotengine.org/en/stable/classes/class_acceptdialog.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AcceptDialog::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AcceptDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AcceptDialog`][crate::classes::AcceptDialog].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AcceptDialog` methods](https://docs.godotengine.org/en/stable/classes/class_acceptdialog.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAcceptDialog: crate::obj::GodotClass < Base = AcceptDialog > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AcceptDialog {
        pub fn get_ok_button(&mut self,) -> Option < Gd < crate::classes::Button > > {
            type CallSig = (Option < Gd < crate::classes::Button > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(82usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_ok_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_label(&mut self,) -> Option < Gd < crate::classes::Label > > {
            type CallSig = (Option < Gd < crate::classes::Label > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(83usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_on_ok(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(84usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_hide_on_ok", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hide_on_ok(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(85usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_hide_on_ok", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_close_on_escape(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(86usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_close_on_escape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_close_on_escape(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(87usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_close_on_escape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_button_full(&mut self, text: CowArg < GString >, right: bool, action: CowArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::Button > >, CowArg < 'a0, GString >, bool, CowArg < 'a1, GString >);
            let args = (text, right, action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(88usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "add_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_button_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_button(&mut self, text: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            self.add_button_ex(text,) . done()
        }
        #[inline]
        pub fn add_button_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExAddButton < 'a > {
            ExAddButton::new(self, text,)
        }
        pub fn add_cancel_button(&mut self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Button > >, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(89usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "add_cancel_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_button(&mut self, button: impl AsObjectArg < crate::classes::Button >,) {
            type CallSig = ((), ObjectArg < crate::classes::Button >);
            let args = (button.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(90usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "remove_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_text_enter(&mut self, line_edit: impl AsObjectArg < crate::classes::LineEdit >,) {
            type CallSig = ((), ObjectArg < crate::classes::LineEdit >);
            let args = (line_edit.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(91usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "register_text_enter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(92usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(93usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap(&mut self, autowrap: bool,) {
            type CallSig = ((), bool);
            let args = (autowrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(94usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_autowrap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_autowrap(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(95usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "has_autowrap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ok_button_text(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(96usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_ok_button_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ok_button_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(97usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_ok_button_text", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AcceptDialog {
        type Base = crate::classes::Window;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AcceptDialog"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AcceptDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for AcceptDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for AcceptDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AcceptDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AcceptDialog {
        
    }
    impl crate::obj::cap::GodotDefault for AcceptDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AcceptDialog {
        type Target = crate::classes::Window;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AcceptDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AcceptDialog`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AcceptDialog {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AcceptDialog > for $Class {
                
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
#[doc = "Default-param extender for [`AcceptDialog::add_button_ex`][super::AcceptDialog::add_button_ex]."]
#[must_use]
pub struct ExAddButton < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AcceptDialog, text: CowArg < 'a, GString >, right: bool, action: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddButton < 'a > {
    fn new(surround_object: &'a mut re_export::AcceptDialog, text: impl AsArg < GString > + 'a,) -> Self {
        let right = false;
        let action = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), right: right, action: CowArg::Owned(action),
        }
    }
    #[inline]
    pub fn right(self, right: bool) -> Self {
        Self {
            right: right, .. self
        }
    }
    #[inline]
    pub fn action(self, action: impl AsArg < GString > + 'a) -> Self {
        Self {
            action: action.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Button > > {
        let Self {
            _phantom, surround_object, text, right, action,
        }
        = self;
        re_export::AcceptDialog::add_button_full(surround_object, text, right, action,)
    }
}
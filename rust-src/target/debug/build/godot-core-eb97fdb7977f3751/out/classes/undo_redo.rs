#![doc = "Sidecar module for class [`UndoRedo`][crate::classes::UndoRedo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UndoRedo` enums](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UndoRedo.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`undo_redo`][crate::classes::undo_redo]: sidecar module with related enum/flag types\n* [`IUndoRedo`][crate::classes::IUndoRedo]: virtual methods\n\n\nSee also [Godot docs for `UndoRedo`](https://docs.godotengine.org/en/stable/classes/class_undoredo.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`UndoRedo::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UndoRedo {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`UndoRedo`][crate::classes::UndoRedo].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `UndoRedo` methods](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUndoRedo: crate::obj::GodotClass < Base = UndoRedo > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl UndoRedo {
        pub(crate) fn create_action_full(&mut self, name: CowArg < GString >, merge_mode: crate::classes::undo_redo::MergeMode, backward_undo_ops: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, crate::classes::undo_redo::MergeMode, bool);
            let args = (name, merge_mode, backward_undo_ops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "create_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_action(&mut self, name: impl AsArg < GString >,) {
            self.create_action_ex(name,) . done()
        }
        #[inline]
        pub fn create_action_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a,) -> ExCreateAction < 'a > {
            ExCreateAction::new(self, name,)
        }
        pub(crate) fn commit_action_full(&mut self, execute: bool,) {
            type CallSig = ((), bool);
            let args = (execute,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "commit_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit_action(&mut self,) {
            self.commit_action_ex() . done()
        }
        #[inline]
        pub fn commit_action_ex < 'a > (&'a mut self,) -> ExCommitAction < 'a > {
            ExCommitAction::new(self,)
        }
        pub fn is_committing_action(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "is_committing_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_method(&mut self, callable: &Callable,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_method(&mut self, callable: &Callable,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Callable >);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_property(&mut self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (object.as_object_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_property(&mut self, object: impl AsObjectArg < crate::classes::Object >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (object.as_object_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_do_reference(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig = ((), ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_undo_reference(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            type CallSig = ((), ObjectArg < crate::classes::Object >);
            let args = (object.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_reference", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn start_force_keep_in_merge_ends(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "start_force_keep_in_merge_ends", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_force_keep_in_merge_ends(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "end_force_keep_in_merge_ends", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_history_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "get_history_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_action(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "get_current_action", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_name(&mut self, id: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9727usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "get_action_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn clear_history_full(&mut self, increase_version: bool,) {
            type CallSig = ((), bool);
            let args = (increase_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9728usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "clear_history", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_history_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear_history(&mut self,) {
            self.clear_history_ex() . done()
        }
        #[inline]
        pub fn clear_history_ex < 'a > (&'a mut self,) -> ExClearHistory < 'a > {
            ExClearHistory::new(self,)
        }
        pub fn get_current_action_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9729usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "get_current_action_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_undo(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9730usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "has_undo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_redo(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9731usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "has_redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9732usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "get_version", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_steps(&mut self, max_steps: i32,) {
            type CallSig = ((), i32);
            let args = (max_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9733usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "set_max_steps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_steps(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9734usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "get_max_steps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn redo(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9735usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "redo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn undo(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9736usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "UndoRedo", "undo", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for UndoRedo {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"UndoRedo"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UndoRedo {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for UndoRedo {
        
    }
    impl crate::obj::cap::GodotDefault for UndoRedo {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UndoRedo {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UndoRedo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`UndoRedo`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_UndoRedo {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::UndoRedo > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UndoRedo::create_action_ex`][super::UndoRedo::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::UndoRedo, name: CowArg < 'a, GString >, merge_mode: crate::classes::undo_redo::MergeMode, backward_undo_ops: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAction < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo, name: impl AsArg < GString > + 'a,) -> Self {
        let merge_mode = crate::obj::EngineEnum::from_ord(0);
        let backward_undo_ops = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), merge_mode: merge_mode, backward_undo_ops: backward_undo_ops,
        }
    }
    #[inline]
    pub fn merge_mode(self, merge_mode: crate::classes::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: merge_mode, .. self
        }
    }
    #[inline]
    pub fn backward_undo_ops(self, backward_undo_ops: bool) -> Self {
        Self {
            backward_undo_ops: backward_undo_ops, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, merge_mode, backward_undo_ops,
        }
        = self;
        re_export::UndoRedo::create_action_full(surround_object, name, merge_mode, backward_undo_ops,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::commit_action_ex`][super::UndoRedo::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::UndoRedo, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitAction < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo,) -> Self {
        let execute = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, execute: execute,
        }
    }
    #[inline]
    pub fn execute(self, execute: bool) -> Self {
        Self {
            execute: execute, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, execute,
        }
        = self;
        re_export::UndoRedo::commit_action_full(surround_object, execute,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::clear_history_ex`][super::UndoRedo::clear_history_ex]."]
#[must_use]
pub struct ExClearHistory < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::UndoRedo, increase_version: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearHistory < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo,) -> Self {
        let increase_version = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, increase_version: increase_version,
        }
    }
    #[inline]
    pub fn increase_version(self, increase_version: bool) -> Self {
        Self {
            increase_version: increase_version, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, increase_version,
        }
        = self;
        re_export::UndoRedo::clear_history_full(surround_object, increase_version,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MergeMode {
    ord: i32
}
impl MergeMode {
    #[doc(alias = "MERGE_DISABLE")]
    #[doc = "Godot enumerator name: `MERGE_DISABLE`"]
    pub const DISABLE: MergeMode = MergeMode {
        ord: 0i32
    };
    #[doc(alias = "MERGE_ENDS")]
    #[doc = "Godot enumerator name: `MERGE_ENDS`"]
    pub const ENDS: MergeMode = MergeMode {
        ord: 1i32
    };
    #[doc(alias = "MERGE_ALL")]
    #[doc = "Godot enumerator name: `MERGE_ALL`"]
    pub const ALL: MergeMode = MergeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MergeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MergeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MergeMode {
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
            Self::DISABLE => "DISABLE", Self::ENDS => "ENDS", Self::ALL => "ALL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLE => "MERGE_DISABLE", Self::ENDS => "MERGE_ENDS", Self::ALL => "MERGE_ALL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MergeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MergeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MergeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
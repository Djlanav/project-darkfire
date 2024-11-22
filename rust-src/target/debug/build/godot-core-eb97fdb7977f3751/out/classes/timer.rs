#![doc = "Sidecar module for class [`Timer`][crate::classes::Timer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Timer` enums](https://docs.godotengine.org/en/stable/classes/class_timer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Timer.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`timer`][crate::classes::timer]: sidecar module with related enum/flag types\n* [`ITimer`][crate::classes::ITimer]: virtual methods\n\n\nSee also [Godot docs for `Timer`](https://docs.godotengine.org/en/stable/classes/class_timer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Timer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Timer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Timer`][crate::classes::Timer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Timer` methods](https://docs.godotengine.org/en/stable/classes/class_timer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITimer: crate::obj::GodotClass < Base = Timer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
    impl Timer {
        pub fn set_wait_time(&mut self, time_sec: f64,) {
            type CallSig = ((), f64);
            let args = (time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "set_wait_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wait_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "get_wait_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "set_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_one_shot(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "is_one_shot", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autostart(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "set_autostart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_autostart(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "has_autostart", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn start_full(&mut self, time_sec: f64,) {
            type CallSig = ((), f64);
            let args = (time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::start_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn start(&mut self,) {
            self.start_ex() . done()
        }
        #[inline]
        pub fn start_ex < 'a > (&'a mut self,) -> ExStart < 'a > {
            ExStart::new(self,)
        }
        pub fn stop(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_paused(&mut self, paused: bool,) {
            type CallSig = ((), bool);
            let args = (paused,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "set_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_paused(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "is_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stopped(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "is_stopped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_left(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "get_time_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_timer_process_callback(&mut self, callback: crate::classes::timer::TimerProcessCallback,) {
            type CallSig = ((), crate::classes::timer::TimerProcessCallback);
            let args = (callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "set_timer_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_timer_process_callback(&self,) -> crate::classes::timer::TimerProcessCallback {
            type CallSig = (crate::classes::timer::TimerProcessCallback,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Timer", "get_timer_process_callback", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Timer {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Timer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Timer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Timer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Timer {
        
    }
    impl crate::obj::cap::GodotDefault for Timer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Timer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Timer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Timer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Timer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Timer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Timer::start_ex`][super::Timer::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Timer, time_sec: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::Timer,) -> Self {
        let time_sec = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, time_sec: time_sec,
        }
    }
    #[inline]
    pub fn time_sec(self, time_sec: f64) -> Self {
        Self {
            time_sec: time_sec, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, time_sec,
        }
        = self;
        re_export::Timer::start_full(surround_object, time_sec,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TimerProcessCallback {
    ord: i32
}
impl TimerProcessCallback {
    #[doc(alias = "TIMER_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `TIMER_PROCESS_PHYSICS`"]
    pub const PHYSICS: TimerProcessCallback = TimerProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "TIMER_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `TIMER_PROCESS_IDLE`"]
    pub const IDLE: TimerProcessCallback = TimerProcessCallback {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TimerProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TimerProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TimerProcessCallback {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PHYSICS => "TIMER_PROCESS_PHYSICS", Self::IDLE => "TIMER_PROCESS_IDLE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TimerProcessCallback {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TimerProcessCallback {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TimerProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
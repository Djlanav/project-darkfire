use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, ObjectArg, ObjectCow, PtrcallSignatureTuple, RefArg, VarcallSignatureTuple
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerQuaternion < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerQuaternion < 'a > {
    pub fn from_outer(outer: &Quaternion) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn length(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(344usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(345usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Quaternion {
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(346usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(347usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Quaternion,) -> bool {
        type CallSig = (bool, Quaternion);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(348usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(349usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Quaternion {
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(350usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "inverse", self.sys_ptr, args)
        }
    }
    pub fn log(&self,) -> Quaternion {
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(351usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "log", self.sys_ptr, args)
        }
    }
    pub fn exp(&self,) -> Quaternion {
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(352usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "exp", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Quaternion,) -> f64 {
        type CallSig = (f64, Quaternion);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(353usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "angle_to", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Quaternion,) -> f64 {
        type CallSig = (f64, Quaternion);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(354usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "dot", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type CallSig = (Quaternion, Quaternion, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(355usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "slerp", self.sys_ptr, args)
        }
    }
    pub fn slerpni(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type CallSig = (Quaternion, Quaternion, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(356usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "slerpni", self.sys_ptr, args)
        }
    }
    pub fn spherical_cubic_interpolate(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64,) -> Quaternion {
        type CallSig = (Quaternion, Quaternion, Quaternion, Quaternion, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(357usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "spherical_cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn spherical_cubic_interpolate_in_time(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Quaternion {
        type CallSig = (Quaternion, Quaternion, Quaternion, Quaternion, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(358usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "spherical_cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type CallSig = (Vector3, i64);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(359usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_euler", self.sys_ptr, args)
        }
    }
    pub fn from_euler(euler: Vector3,) -> Quaternion {
        type CallSig = (Quaternion, Vector3);
        let args = (euler,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(360usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "from_euler", std::ptr::null_mut(), args)
        }
    }
    pub fn get_axis(&self,) -> Vector3 {
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(361usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_axis", self.sys_ptr, args)
        }
    }
    pub fn get_angle(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(362usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_angle", self.sys_ptr, args)
        }
    }
}
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
pub struct InnerStringName < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerStringName < 'a > {
    pub fn from_outer(outer: &StringName) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn casecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(471usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "casecmp_to", self.sys_ptr, args)
        }
    }
    pub fn nocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(472usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "nocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalcasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(473usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "naturalcasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalnocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(474usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "naturalnocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filecasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(475usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "filecasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filenocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(476usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "filenocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(477usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "length", self.sys_ptr, args)
        }
    }
    pub fn substr(&self, from: i64, len: i64,) -> GString {
        type CallSig = (GString, i64, i64);
        let args = (from, len,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(478usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "substr", self.sys_ptr, args)
        }
    }
    pub fn get_slice(&self, delimiter: impl AsArg < GString >, slice: i64,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, i64);
        let args = (delimiter.into_arg(), slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(479usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_slice", self.sys_ptr, args)
        }
    }
    pub fn get_slicec(&self, delimiter: i64, slice: i64,) -> GString {
        type CallSig = (GString, i64, i64);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(480usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_slicec", self.sys_ptr, args)
        }
    }
    pub fn get_slice_count(&self, delimiter: impl AsArg < GString >,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >);
        let args = (delimiter.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(481usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_slice_count", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(482usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "find", self.sys_ptr, args)
        }
    }
    pub fn findn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(483usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "findn", self.sys_ptr, args)
        }
    }
    pub fn count(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64, i64);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(484usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "count", self.sys_ptr, args)
        }
    }
    pub fn countn(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64, i64);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(485usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "countn", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(486usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "rfind", self.sys_ptr, args)
        }
    }
    pub fn rfindn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallSig < 'a0, > = (i64, CowArg < 'a0, GString >, i64);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(487usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "rfindn", self.sys_ptr, args)
        }
    }
    pub fn match_(&self, expr: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(488usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "match", self.sys_ptr, args)
        }
    }
    pub fn matchn(&self, expr: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(489usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "matchn", self.sys_ptr, args)
        }
    }
    pub fn begins_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(490usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "begins_with", self.sys_ptr, args)
        }
    }
    pub fn ends_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(491usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "ends_with", self.sys_ptr, args)
        }
    }
    pub fn is_subsequence_of(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(492usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_subsequence_of", self.sys_ptr, args)
        }
    }
    pub fn is_subsequence_ofn(&self, text: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(493usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_subsequence_ofn", self.sys_ptr, args)
        }
    }
    pub fn bigrams(&self,) -> PackedStringArray {
        type CallSig = (PackedStringArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(494usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "bigrams", self.sys_ptr, args)
        }
    }
    pub fn similarity(&self, text: impl AsArg < GString >,) -> f64 {
        type CallSig < 'a0, > = (f64, CowArg < 'a0, GString >);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(495usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "similarity", self.sys_ptr, args)
        }
    }
    pub fn format(&self, values: &Variant, placeholder: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, 'a1, > = (GString, RefArg < 'a0, Variant >, CowArg < 'a1, GString >);
        let args = (RefArg::new(values), placeholder.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(496usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "format", self.sys_ptr, args)
        }
    }
    pub fn replace(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(497usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "replace", self.sys_ptr, args)
        }
    }
    pub fn replacen(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, GString >);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(498usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "replacen", self.sys_ptr, args)
        }
    }
    pub fn repeat(&self, count: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(499usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "repeat", self.sys_ptr, args)
        }
    }
    pub fn reverse(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(500usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "reverse", self.sys_ptr, args)
        }
    }
    pub fn insert(&self, position: i64, what: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, i64, CowArg < 'a0, GString >);
        let args = (position, what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(501usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "insert", self.sys_ptr, args)
        }
    }
    pub fn erase(&self, position: i64, chars: i64,) -> GString {
        type CallSig = (GString, i64, i64);
        let args = (position, chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(502usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "erase", self.sys_ptr, args)
        }
    }
    pub fn capitalize(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(503usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "capitalize", self.sys_ptr, args)
        }
    }
    pub fn to_camel_case(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(504usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_camel_case", self.sys_ptr, args)
        }
    }
    pub fn to_pascal_case(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(505usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_pascal_case", self.sys_ptr, args)
        }
    }
    pub fn to_snake_case(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(506usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_snake_case", self.sys_ptr, args)
        }
    }
    pub fn split(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >, bool, i64);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(507usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "split", self.sys_ptr, args)
        }
    }
    pub fn rsplit(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >, bool, i64);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(508usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "rsplit", self.sys_ptr, args)
        }
    }
    pub fn split_floats(&self, delimiter: impl AsArg < GString >, allow_empty: bool,) -> PackedFloat64Array {
        type CallSig < 'a0, > = (PackedFloat64Array, CowArg < 'a0, GString >, bool);
        let args = (delimiter.into_arg(), allow_empty,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(509usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "split_floats", self.sys_ptr, args)
        }
    }
    pub fn join(&self, parts: &PackedStringArray,) -> GString {
        type CallSig < 'a0, > = (GString, RefArg < 'a0, PackedStringArray >);
        let args = (RefArg::new(parts),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(510usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "join", self.sys_ptr, args)
        }
    }
    pub fn to_upper(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(511usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_upper", self.sys_ptr, args)
        }
    }
    pub fn to_lower(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(512usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_lower", self.sys_ptr, args)
        }
    }
    pub fn left(&self, length: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(513usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "left", self.sys_ptr, args)
        }
    }
    pub fn right(&self, length: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(514usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "right", self.sys_ptr, args)
        }
    }
    pub fn strip_edges(&self, left: bool, right: bool,) -> GString {
        type CallSig = (GString, bool, bool);
        let args = (left, right,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(515usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "strip_edges", self.sys_ptr, args)
        }
    }
    pub fn strip_escapes(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(516usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "strip_escapes", self.sys_ptr, args)
        }
    }
    pub fn lstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(517usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "lstrip", self.sys_ptr, args)
        }
    }
    pub fn rstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(518usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "rstrip", self.sys_ptr, args)
        }
    }
    pub fn get_extension(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(519usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_extension", self.sys_ptr, args)
        }
    }
    pub fn get_basename(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(520usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_basename", self.sys_ptr, args)
        }
    }
    pub fn path_join(&self, file: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (file.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(521usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "path_join", self.sys_ptr, args)
        }
    }
    pub fn unicode_at(&self, at: i64,) -> i64 {
        type CallSig = (i64, i64);
        let args = (at,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(522usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "unicode_at", self.sys_ptr, args)
        }
    }
    pub fn indent(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(523usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "indent", self.sys_ptr, args)
        }
    }
    pub fn dedent(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(524usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "dedent", self.sys_ptr, args)
        }
    }
    pub fn md5_text(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(525usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "md5_text", self.sys_ptr, args)
        }
    }
    pub fn sha1_text(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(526usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "sha1_text", self.sys_ptr, args)
        }
    }
    pub fn sha256_text(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(527usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "sha256_text", self.sys_ptr, args)
        }
    }
    pub fn md5_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(528usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "md5_buffer", self.sys_ptr, args)
        }
    }
    pub fn sha1_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(529usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "sha1_buffer", self.sys_ptr, args)
        }
    }
    pub fn sha256_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(530usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "sha256_buffer", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(531usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn contains(&self, what: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(532usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "contains", self.sys_ptr, args)
        }
    }
    pub fn containsn(&self, what: impl AsArg < GString >,) -> bool {
        type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(533usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "containsn", self.sys_ptr, args)
        }
    }
    pub fn is_absolute_path(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(534usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_absolute_path", self.sys_ptr, args)
        }
    }
    pub fn is_relative_path(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(535usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_relative_path", self.sys_ptr, args)
        }
    }
    pub fn simplify_path(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(536usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "simplify_path", self.sys_ptr, args)
        }
    }
    pub fn get_base_dir(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(537usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_base_dir", self.sys_ptr, args)
        }
    }
    pub fn get_file(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(538usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "get_file", self.sys_ptr, args)
        }
    }
    pub fn xml_escape(&self, escape_quotes: bool,) -> GString {
        type CallSig = (GString, bool);
        let args = (escape_quotes,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(539usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "xml_escape", self.sys_ptr, args)
        }
    }
    pub fn xml_unescape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(540usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "xml_unescape", self.sys_ptr, args)
        }
    }
    pub fn uri_encode(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(541usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "uri_encode", self.sys_ptr, args)
        }
    }
    pub fn uri_decode(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(542usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "uri_decode", self.sys_ptr, args)
        }
    }
    pub fn c_escape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(543usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "c_escape", self.sys_ptr, args)
        }
    }
    pub fn c_unescape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(544usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "c_unescape", self.sys_ptr, args)
        }
    }
    pub fn json_escape(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(545usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "json_escape", self.sys_ptr, args)
        }
    }
    pub fn validate_node_name(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(546usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "validate_node_name", self.sys_ptr, args)
        }
    }
    pub fn validate_filename(&self,) -> GString {
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(547usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "validate_filename", self.sys_ptr, args)
        }
    }
    pub fn is_valid_identifier(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(548usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_identifier", self.sys_ptr, args)
        }
    }
    pub fn is_valid_int(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(549usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_int", self.sys_ptr, args)
        }
    }
    pub fn is_valid_float(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(550usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_float", self.sys_ptr, args)
        }
    }
    pub fn is_valid_hex_number(&self, with_prefix: bool,) -> bool {
        type CallSig = (bool, bool);
        let args = (with_prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(551usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_hex_number", self.sys_ptr, args)
        }
    }
    pub fn is_valid_html_color(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(552usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_html_color", self.sys_ptr, args)
        }
    }
    pub fn is_valid_ip_address(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(553usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_ip_address", self.sys_ptr, args)
        }
    }
    pub fn is_valid_filename(&self,) -> bool {
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(554usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_filename", self.sys_ptr, args)
        }
    }
    pub fn to_int(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(555usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_int", self.sys_ptr, args)
        }
    }
    pub fn to_float(&self,) -> f64 {
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(556usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_float", self.sys_ptr, args)
        }
    }
    pub fn hex_to_int(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(557usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "hex_to_int", self.sys_ptr, args)
        }
    }
    pub fn bin_to_int(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(558usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "bin_to_int", self.sys_ptr, args)
        }
    }
    pub fn lpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, i64, CowArg < 'a0, GString >);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(559usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "lpad", self.sys_ptr, args)
        }
    }
    pub fn rpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, i64, CowArg < 'a0, GString >);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(560usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "rpad", self.sys_ptr, args)
        }
    }
    pub fn pad_decimals(&self, digits: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(561usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "pad_decimals", self.sys_ptr, args)
        }
    }
    pub fn pad_zeros(&self, digits: i64,) -> GString {
        type CallSig = (GString, i64);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(562usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "pad_zeros", self.sys_ptr, args)
        }
    }
    pub fn trim_prefix(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(563usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "trim_prefix", self.sys_ptr, args)
        }
    }
    pub fn trim_suffix(&self, suffix: impl AsArg < GString >,) -> GString {
        type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
        let args = (suffix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(564usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "trim_suffix", self.sys_ptr, args)
        }
    }
    pub fn to_ascii_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(565usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_ascii_buffer", self.sys_ptr, args)
        }
    }
    pub fn to_utf8_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(566usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf8_buffer", self.sys_ptr, args)
        }
    }
    pub fn to_utf16_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(567usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf16_buffer", self.sys_ptr, args)
        }
    }
    pub fn to_utf32_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(568usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf32_buffer", self.sys_ptr, args)
        }
    }
    pub fn hex_decode(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(569usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "hex_decode", self.sys_ptr, args)
        }
    }
    pub fn to_wchar_buffer(&self,) -> PackedByteArray {
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(570usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "to_wchar_buffer", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(571usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall(method_bind, "StringName", "hash", self.sys_ptr, args)
        }
    }
}
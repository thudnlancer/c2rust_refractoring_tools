use std::ffi::{CString, CStr};
use std::ptr;
use std::os::raw::{c_char, c_uint, c_ulong, c_void};
use c2rust_bitfields::BitfieldStruct;

type size_t = c_ulong;
type uintmax_t = c_ulong;
type gmk_func_ptr = Option<extern "C" fn(*const c_char, c_uint, *mut *mut c_char) -> *mut c_char>;
type hash_func_t = Option<extern "C" fn(*const c_void) -> c_ulong>;
type hash_cmp_func_t = Option<extern "C" fn(*const c_void, *const c_void) -> c_int>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmk_floc {
    pub filenm: *const c_char,
    pub lineno: c_ulong,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const c_char,
    pub lineno: c_ulong,
    pub offset: c_ulong,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const c_char,
    pub hname: *const c_char,
    pub vpath: *const c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: c_uint,
    pub command_flags: c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: c_ulong,
    pub ht_capacity: c_ulong,
    pub ht_fill: c_ulong,
    pub ht_empty_slots: c_ulong,
    pub ht_collisions: c_ulong,
    pub ht_lookups: c_ulong,
    pub ht_rehashes: c_uint,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const c_char,
    #[bitfield(name = "flags", ty = "c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}

pub enum commands {}

static mut reading_file: *const floc = ptr::null();

pub fn gmk_alloc(len: c_uint) -> *mut c_char {
    unsafe { xmalloc(len as size_t) as *mut c_char }
}

pub fn gmk_free(s: *mut c_char) {
    unsafe { free(s as *mut c_void) }
}

pub fn gmk_eval(buffer: &str, gfloc: Option<&gmk_floc>) {
    let c_buffer = CString::new(buffer).unwrap();
    let mut fl = floc {
        filenm: ptr::null(),
        lineno: 0,
        offset: 0,
    };
    
    let flp = if let Some(floc) = gfloc {
        fl.filenm = floc.filenm;
        fl.lineno = floc.lineno;
        &mut fl as *mut floc
    } else {
        ptr::null_mut()
    };

    let mut pbuf: *mut c_char = ptr::null_mut();
    let mut plen: size_t = 0;
    
    unsafe {
        install_variable_buffer(&mut pbuf, &mut plen);
        let s = xstrdup(c_buffer.as_ptr());
        eval_buffer(s, flp);
        free(s as *mut c_void);
        restore_variable_buffer(pbuf, plen);
    }
}

pub fn gmk_expand(reference: &str) -> *mut c_char {
    let c_ref = CString::new(reference).unwrap();
    unsafe { allocated_variable_expand_for_file(c_ref.as_ptr(), ptr::null_mut()) }
}

pub fn gmk_add_function(
    name: &str,
    func: gmk_func_ptr,
    min: c_uint,
    max: c_uint,
    flags: c_uint,
) {
    let c_name = CString::new(name).unwrap();
    unsafe {
        define_new_function(reading_file, c_name.as_ptr(), min, max, flags, func);
    }
}

extern "C" {
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn xmalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn allocated_variable_expand_for_file(line: *const c_char, file: *mut file) -> *mut c_char;
    fn install_variable_buffer(bufp: *mut *mut c_char, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut c_char, len: size_t);
    fn define_new_function(
        flocp: *const floc,
        name: *const c_char,
        min: c_uint,
        max: c_uint,
        flags: c_uint,
        func: gmk_func_ptr,
    );
    fn eval_buffer(buffer: *mut c_char, floc: *const floc);
}
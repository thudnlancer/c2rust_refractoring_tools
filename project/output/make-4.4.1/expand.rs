#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type dep;
    static mut environ: *mut *mut i8;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn xstrndup(_: *const i8, _: size_t) -> *mut i8;
    fn lindex(_: *const i8, _: *const i8, _: i32) -> *mut i8;
    fn find_percent(_: *mut i8) -> *mut i8;
    static mut reading_file: *const floc;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut db_level: i32;
    static mut env_recursion: libc::c_ulonglong;
    static mut current_variable_set_list: *mut variable_set_list;
    fn handle_function(op: *mut *mut i8, stringp: *mut *const i8) -> i32;
    fn patsubst_expand_pat(
        o: *mut i8,
        text: *const i8,
        pattern: *const i8,
        replace: *const i8,
        pattern_percent: *const i8,
        replace_percent: *const i8,
    ) -> *mut i8;
    fn lookup_variable(name: *const i8, length: size_t) -> *mut variable;
    fn lookup_variable_in_set(
        name: *const i8,
        length: size_t,
        set: *const variable_set,
    ) -> *mut variable;
    fn warn_undefined(name: *const i8, length: size_t);
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const i8,
    pub hname: *const i8,
    pub vpath: *const i8,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const i8,
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
    pub considered: u32,
    pub command_flags: i32,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cmd_state {
    cs_finished = 3,
    cs_running = 2,
    cs_deps_running = 1,
    cs_not_started = 0,
}
impl cmd_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cmd_state::cs_finished => 3,
            cmd_state::cs_running => 2,
            cmd_state::cs_deps_running => 1,
            cmd_state::cs_not_started => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> cmd_state {
        match value {
            3 => cmd_state::cs_finished,
            2 => cmd_state::cs_running,
            1 => cmd_state::cs_deps_running,
            0 => cmd_state::cs_not_started,
            _ => panic!("Invalid value for cmd_state: {}", value),
        }
    }
}
impl AddAssign<u32> for cmd_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cmd_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cmd_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cmd_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cmd_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cmd_state {
    type Output = cmd_state;
    fn add(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cmd_state {
    type Output = cmd_state;
    fn sub(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cmd_state {
    type Output = cmd_state;
    fn mul(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cmd_state {
    type Output = cmd_state;
    fn div(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cmd_state {
    type Output = cmd_state;
    fn rem(self, rhs: u32) -> cmd_state {
        cmd_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum update_status {
    us_failed = 3,
    us_question = 2,
    us_none = 1,
    us_success = 0,
}
impl update_status {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            update_status::us_failed => 3,
            update_status::us_question => 2,
            update_status::us_none => 1,
            update_status::us_success => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> update_status {
        match value {
            3 => update_status::us_failed,
            2 => update_status::us_question,
            1 => update_status::us_none,
            0 => update_status::us_success,
            _ => panic!("Invalid value for update_status: {}", value),
        }
    }
}
impl AddAssign<u32> for update_status {
    fn add_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for update_status {
    fn sub_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for update_status {
    fn mul_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for update_status {
    fn div_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for update_status {
    fn rem_assign(&mut self, rhs: u32) {
        *self = update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for update_status {
    type Output = update_status;
    fn add(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for update_status {
    type Output = update_status;
    fn sub(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for update_status {
    type Output = update_status;
    fn mul(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for update_status {
    type Output = update_status;
    fn div(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for update_status {
    type Output = update_status;
    fn rem(self, rhs: u32) -> update_status {
        update_status::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: u64,
    pub ht_capacity: u64,
    pub ht_fill: u64,
    pub ht_empty_slots: u64,
    pub ht_collisions: u64,
    pub ht_lookups: u64,
    pub ht_rehashes: u32,
}
pub type hash_cmp_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type hash_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> u64>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
    pub commands: *mut i8,
    pub command_lines: *mut *mut i8,
    pub lines_flags: *mut u8,
    pub ncommand_lines: libc::c_ushort,
    pub recipe_prefix: i8,
    #[bitfield(name = "any_recurse", ty = "libc::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_origin {
    o_default,
    o_env,
    o_file,
    o_env_override,
    o_command,
    o_override,
    o_automatic,
    o_invalid,
}
impl variable_origin {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_origin::o_default => 0,
            variable_origin::o_env => 1,
            variable_origin::o_file => 2,
            variable_origin::o_env_override => 3,
            variable_origin::o_command => 4,
            variable_origin::o_override => 5,
            variable_origin::o_automatic => 6,
            variable_origin::o_invalid => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_origin {
        match value {
            0 => variable_origin::o_default,
            1 => variable_origin::o_env,
            2 => variable_origin::o_file,
            3 => variable_origin::o_env_override,
            4 => variable_origin::o_command,
            5 => variable_origin::o_override,
            6 => variable_origin::o_automatic,
            7 => variable_origin::o_invalid,
            _ => panic!("Invalid value for variable_origin: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_origin {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_origin {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_origin {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_origin {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_origin {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_origin::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_origin {
    type Output = variable_origin;
    fn add(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_origin {
    type Output = variable_origin;
    fn sub(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_origin {
    type Output = variable_origin;
    fn mul(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_origin {
    type Output = variable_origin;
    fn div(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_origin {
    type Output = variable_origin;
    fn rem(self, rhs: u32) -> variable_origin {
        variable_origin::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut i8,
    pub value: *mut i8,
    pub fileinfo: floc,
    pub length: u32,
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "libc::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_export {
    v_default = 0,
    v_export,
    v_noexport,
    v_ifset,
}
impl variable_export {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_export::v_default => 0,
            variable_export::v_export => 1,
            variable_export::v_noexport => 2,
            variable_export::v_ifset => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_export {
        match value {
            0 => variable_export::v_default,
            1 => variable_export::v_export,
            2 => variable_export::v_noexport,
            3 => variable_export::v_ifset,
            _ => panic!("Invalid value for variable_export: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_export {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_export {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_export {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_export {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_export {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_export::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_export {
    type Output = variable_export;
    fn add(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_export {
    type Output = variable_export;
    fn sub(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_export {
    type Output = variable_export;
    fn mul(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_export {
    type Output = variable_export;
    fn div(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_export {
    type Output = variable_export;
    fn rem(self, rhs: u32) -> variable_export {
        variable_export::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum variable_flavor {
    f_bogus,
    f_simple,
    f_recursive,
    f_expand,
    f_append,
    f_conditional,
    f_shell,
    f_append_value,
}
impl variable_flavor {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            variable_flavor::f_bogus => 0,
            variable_flavor::f_simple => 1,
            variable_flavor::f_recursive => 2,
            variable_flavor::f_expand => 3,
            variable_flavor::f_append => 4,
            variable_flavor::f_conditional => 5,
            variable_flavor::f_shell => 6,
            variable_flavor::f_append_value => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> variable_flavor {
        match value {
            0 => variable_flavor::f_bogus,
            1 => variable_flavor::f_simple,
            2 => variable_flavor::f_recursive,
            3 => variable_flavor::f_expand,
            4 => variable_flavor::f_append,
            5 => variable_flavor::f_conditional,
            6 => variable_flavor::f_shell,
            7 => variable_flavor::f_append_value,
            _ => panic!("Invalid value for variable_flavor: {}", value),
        }
    }
}
impl AddAssign<u32> for variable_flavor {
    fn add_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for variable_flavor {
    fn sub_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for variable_flavor {
    fn mul_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for variable_flavor {
    fn div_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for variable_flavor {
    fn rem_assign(&mut self, rhs: u32) {
        *self = variable_flavor::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for variable_flavor {
    type Output = variable_flavor;
    fn add(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for variable_flavor {
    type Output = variable_flavor;
    fn sub(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for variable_flavor {
    type Output = variable_flavor;
    fn mul(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for variable_flavor {
    type Output = variable_flavor;
    fn div(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for variable_flavor {
    type Output = variable_flavor;
    fn rem(self, rhs: u32) -> variable_flavor {
        variable_flavor::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub static mut expanding_var: *mut *const floc = unsafe {
    &reading_file as *const *const floc as *mut *const floc
};
static mut variable_buffer_length: size_t = 0;
#[no_mangle]
pub static mut variable_buffer: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn variable_buffer_output(
    mut ptr: *mut i8,
    mut string: *const i8,
    mut length: size_t,
) -> *mut i8 {
    let mut newlen: size_t = length
        .wrapping_add(ptr.offset_from(variable_buffer) as i64 as u64);
    if newlen.wrapping_add(5 as i32 as u64) > variable_buffer_length {
        let mut offset: size_t = ptr.offset_from(variable_buffer) as i64 as size_t;
        variable_buffer_length = if newlen.wrapping_add(100 as i32 as u64)
            > (2 as i32 as u64).wrapping_mul(variable_buffer_length)
        {
            newlen.wrapping_add(100 as i32 as u64)
        } else {
            (2 as i32 as u64).wrapping_mul(variable_buffer_length)
        };
        variable_buffer = xrealloc(
            variable_buffer as *mut libc::c_void,
            variable_buffer_length,
        ) as *mut i8;
        ptr = variable_buffer.offset(offset as isize);
    }
    return mempcpy(ptr as *mut libc::c_void, string as *const libc::c_void, length)
        as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn initialize_variable_output() -> *mut i8 {
    if variable_buffer.is_null() {
        variable_buffer_length = 200 as i32 as size_t;
        variable_buffer = xmalloc(variable_buffer_length) as *mut i8;
        *variable_buffer.offset(0 as i32 as isize) = '\0' as i32 as i8;
    }
    return variable_buffer;
}
#[no_mangle]
pub unsafe extern "C" fn recursively_expand_for_file(
    mut v: *mut variable,
    mut file: *mut file,
) -> *mut i8 {
    let mut value: *mut i8 = 0 as *mut i8;
    let mut this_var: *const floc = 0 as *const floc;
    let mut saved_varp: *mut *const floc = 0 as *mut *const floc;
    let mut save: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut set_reading: i32 = 0 as i32;
    if (*v).expanding() as i32 != 0 && env_recursion != 0 {
        let mut nl: size_t = strlen((*v).name);
        let mut ep: *mut *mut i8 = 0 as *mut *mut i8;
        if 0x2 as i32 & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"%s:%lu: not recursively expanding %s to export to shell function\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*v).fileinfo.filenm,
                (*v).fileinfo.lineno,
                (*v).name,
            );
            fflush(stdout);
        }
        ep = environ;
        while !(*ep).is_null() {
            if *(*ep).offset(nl as isize) as i32 == '=' as i32
                && strncmp(*ep, (*v).name, nl) == 0 as i32
            {
                return xstrdup((*ep).offset(nl as isize).offset(1 as i32 as isize));
            }
            ep = ep.offset(1);
            ep;
        }
        return xstrdup(b"\0" as *const u8 as *const i8);
    }
    saved_varp = expanding_var;
    if !((*v).fileinfo.filenm).is_null() {
        this_var = &mut (*v).fileinfo;
        expanding_var = &mut this_var;
    }
    if reading_file.is_null() {
        set_reading = 1 as i32;
        reading_file = &mut (*v).fileinfo;
    }
    if (*v).expanding() != 0 {
        if (*v).exp_count() == 0 {
            fatal(
                *expanding_var,
                strlen((*v).name),
                dcgettext(
                    0 as *const i8,
                    b"Recursive variable '%s' references itself (eventually)\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*v).name,
            );
        }
        (*v).set_exp_count((*v).exp_count() - 1);
        (*v).exp_count();
    }
    if !file.is_null() {
        save = current_variable_set_list;
        current_variable_set_list = (*file).variables;
    }
    (*v).set_expanding(1 as i32 as u32);
    if (*v).append() != 0 {
        value = allocated_variable_append(v);
    } else {
        value = allocated_variable_expand_for_file((*v).value, 0 as *mut file);
    }
    (*v).set_expanding(0 as i32 as u32);
    if set_reading != 0 {
        reading_file = 0 as *const floc;
    }
    if !file.is_null() {
        current_variable_set_list = save;
    }
    expanding_var = saved_varp;
    return value;
}
#[inline]
unsafe extern "C" fn reference_variable(
    mut o: *mut i8,
    mut name: *const i8,
    mut length: size_t,
) -> *mut i8 {
    let mut v: *mut variable = 0 as *mut variable;
    let mut value: *mut i8 = 0 as *mut i8;
    v = lookup_variable(name, length);
    if v.is_null() {
        warn_undefined(name, length);
    }
    if v.is_null() || *(*v).value as i32 == '\0' as i32 && (*v).append() == 0 {
        return o;
    }
    value = if (*v).recursive() as i32 != 0 {
        recursively_expand_for_file(v, 0 as *mut file)
    } else {
        (*v).value
    };
    o = variable_buffer_output(o, value, strlen(value));
    if (*v).recursive() != 0 {
        free(value as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn variable_expand_string(
    mut line: *mut i8,
    mut string: *const i8,
    mut length: size_t,
) -> *mut i8 {
    let mut v: *mut variable = 0 as *mut variable;
    let mut p: *const i8 = 0 as *const i8;
    let mut p1: *const i8 = 0 as *const i8;
    let mut save: *mut i8 = 0 as *mut i8;
    let mut o: *mut i8 = 0 as *mut i8;
    let mut line_offset: size_t = 0;
    if line.is_null() {
        line = initialize_variable_output();
    }
    o = line;
    line_offset = line.offset_from(variable_buffer) as i64 as size_t;
    if length == 0 as i32 as u64 {
        variable_buffer_output(o, b"\0" as *const u8 as *const i8, 1 as i32 as size_t);
        return variable_buffer;
    }
    save = if length == 18446744073709551615 as u64 {
        xstrdup(string)
    } else {
        xstrndup(string, length)
    };
    p = save;
    loop {
        p1 = strchr(p, '$' as i32);
        o = variable_buffer_output(
            o,
            p,
            if !p1.is_null() {
                p1.offset_from(p) as i64 as size_t
            } else {
                (strlen(p)).wrapping_add(1 as i32 as u64)
            },
        );
        if p1.is_null() {
            break;
        }
        p = p1.offset(1 as i32 as isize);
        match *p as i32 {
            36 | 0 => {
                o = variable_buffer_output(o, p1, 1 as i32 as size_t);
            }
            40 | 123 => {
                let mut openparen: i8 = *p;
                let mut closeparen: i8 = (if openparen as i32 == '(' as i32 {
                    ')' as i32
                } else {
                    '}' as i32
                }) as i8;
                let mut begp: *const i8 = 0 as *const i8;
                let mut beg: *const i8 = p.offset(1 as i32 as isize);
                let mut op: *mut i8 = 0 as *mut i8;
                let mut abeg: *mut i8 = 0 as *mut i8;
                let mut end: *const i8 = 0 as *const i8;
                let mut colon: *const i8 = 0 as *const i8;
                op = o;
                begp = p;
                if handle_function(&mut op, &mut begp) != 0 {
                    o = op;
                    p = begp;
                } else {
                    end = strchr(beg, closeparen as i32);
                    if end.is_null() {
                        fatal(
                            *expanding_var,
                            0 as i32 as size_t,
                            dcgettext(
                                0 as *const i8,
                                b"unterminated variable reference\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    p1 = lindex(beg, end, '$' as i32);
                    if !p1.is_null() {
                        let mut count: i32 = 0 as i32;
                        p = beg;
                        while *p as i32 != '\0' as i32 {
                            if *p as i32 == openparen as i32 {
                                count += 1;
                                count;
                            } else if *p as i32 == closeparen as i32
                                && {
                                    count -= 1;
                                    count < 0 as i32
                                }
                            {
                                break;
                            }
                            p = p.offset(1);
                            p;
                        }
                        if count < 0 as i32 {
                            abeg = expand_argument(beg, p);
                            beg = abeg;
                            end = strchr(beg, '\0' as i32);
                        }
                    } else {
                        p = end;
                    }
                    colon = lindex(beg, end, ':' as i32);
                    if !colon.is_null() {
                        let mut subst_beg: *const i8 = colon.offset(1 as i32 as isize);
                        let mut subst_end: *const i8 = lindex(
                            subst_beg,
                            end,
                            '=' as i32,
                        );
                        if subst_end.is_null() {
                            colon = 0 as *const i8;
                        } else {
                            let mut replace_beg: *const i8 = subst_end
                                .offset(1 as i32 as isize);
                            let mut replace_end: *const i8 = end;
                            v = lookup_variable(
                                beg,
                                colon.offset_from(beg) as i64 as size_t,
                            );
                            if v.is_null() {
                                warn_undefined(
                                    beg,
                                    colon.offset_from(beg) as i64 as size_t,
                                );
                            }
                            if !v.is_null() && *(*v).value as i32 != '\0' as i32 {
                                let mut pattern: *mut i8 = 0 as *mut i8;
                                let mut replace: *mut i8 = 0 as *mut i8;
                                let mut ppercent: *mut i8 = 0 as *mut i8;
                                let mut rpercent: *mut i8 = 0 as *mut i8;
                                let mut value: *mut i8 = if (*v).recursive() as i32 != 0 {
                                    recursively_expand_for_file(v, 0 as *mut file)
                                } else {
                                    (*v).value
                                };
                                let mut fresh0 = ::std::vec::from_elem(
                                    0,
                                    (subst_end.offset_from(subst_beg) as i64 + 2 as i32 as i64)
                                        as u64 as usize,
                                );
                                pattern = fresh0.as_mut_ptr() as *mut i8;
                                let fresh1 = pattern;
                                pattern = pattern.offset(1);
                                *fresh1 = '%' as i32 as i8;
                                memcpy(
                                    pattern as *mut libc::c_void,
                                    subst_beg as *const libc::c_void,
                                    subst_end.offset_from(subst_beg) as i64 as u64,
                                );
                                *pattern
                                    .offset(subst_end.offset_from(subst_beg) as i64 as isize) = '\0'
                                    as i32 as i8;
                                let mut fresh2 = ::std::vec::from_elem(
                                    0,
                                    (replace_end.offset_from(replace_beg) as i64
                                        + 2 as i32 as i64) as u64 as usize,
                                );
                                replace = fresh2.as_mut_ptr() as *mut i8;
                                let fresh3 = replace;
                                replace = replace.offset(1);
                                *fresh3 = '%' as i32 as i8;
                                memcpy(
                                    replace as *mut libc::c_void,
                                    replace_beg as *const libc::c_void,
                                    replace_end.offset_from(replace_beg) as i64 as u64,
                                );
                                *replace
                                    .offset(
                                        replace_end.offset_from(replace_beg) as i64 as isize,
                                    ) = '\0' as i32 as i8;
                                ppercent = find_percent(pattern);
                                if !ppercent.is_null() {
                                    ppercent = ppercent.offset(1);
                                    ppercent;
                                    rpercent = find_percent(replace);
                                    if !rpercent.is_null() {
                                        rpercent = rpercent.offset(1);
                                        rpercent;
                                    }
                                } else {
                                    ppercent = pattern;
                                    rpercent = replace;
                                    pattern = pattern.offset(-1);
                                    pattern;
                                    replace = replace.offset(-1);
                                    replace;
                                }
                                o = patsubst_expand_pat(
                                    o,
                                    value,
                                    pattern,
                                    replace,
                                    ppercent,
                                    rpercent,
                                );
                                if (*v).recursive() != 0 {
                                    free(value as *mut libc::c_void);
                                }
                            }
                        }
                    }
                    if colon.is_null() {
                        o = reference_variable(
                            o,
                            beg,
                            end.offset_from(beg) as i64 as size_t,
                        );
                    }
                    free(abeg as *mut libc::c_void);
                }
            }
            _ => {
                if !(*stopchar_map
                    .as_mut_ptr()
                    .offset(*p.offset(-(1 as i32) as isize) as u8 as isize) as i32
                    & (0x2 as i32 | 0x4 as i32) != 0 as i32)
                {
                    o = reference_variable(o, p, 1 as i32 as size_t);
                }
            }
        }
        if *p as i32 == '\0' as i32 {
            break;
        }
        p = p.offset(1);
        p;
    }
    free(save as *mut libc::c_void);
    variable_buffer_output(o, b"\0" as *const u8 as *const i8, 1 as i32 as size_t);
    return variable_buffer.offset(line_offset as isize);
}
#[no_mangle]
pub unsafe extern "C" fn variable_expand(mut line: *const i8) -> *mut i8 {
    return variable_expand_string(0 as *mut i8, line, 18446744073709551615 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn expand_argument(
    mut str: *const i8,
    mut end: *const i8,
) -> *mut i8 {
    let mut tmp: *mut i8 = 0 as *mut i8;
    let mut alloc: *mut i8 = 0 as *mut i8;
    let mut r: *mut i8 = 0 as *mut i8;
    if str == end {
        return xstrdup(b"\0" as *const u8 as *const i8);
    }
    if end.is_null() || *end as i32 == '\0' as i32 {
        return allocated_variable_expand_for_file(str, 0 as *mut file);
    }
    if end.offset_from(str) as i64 + 1 as i32 as i64 > 1000 as i32 as i64 {
        alloc = xmalloc((end.offset_from(str) as i64 + 1 as i32 as i64) as size_t)
            as *mut i8;
        tmp = alloc;
    } else {
        let mut fresh4 = ::std::vec::from_elem(
            0,
            (end.offset_from(str) as i64 + 1 as i32 as i64) as u64 as usize,
        );
        tmp = fresh4.as_mut_ptr() as *mut i8;
    }
    memcpy(
        tmp as *mut libc::c_void,
        str as *const libc::c_void,
        end.offset_from(str) as i64 as u64,
    );
    *tmp.offset(end.offset_from(str) as i64 as isize) = '\0' as i32 as i8;
    r = allocated_variable_expand_for_file(tmp, 0 as *mut file);
    free(alloc as *mut libc::c_void);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn variable_expand_for_file(
    mut line: *const i8,
    mut file: *mut file,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut savev: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut savef: *const floc = 0 as *const floc;
    if file.is_null() {
        return variable_expand(line);
    }
    savev = current_variable_set_list;
    current_variable_set_list = (*file).variables;
    savef = reading_file;
    if !((*file).cmds).is_null() && !((*(*file).cmds).fileinfo.filenm).is_null() {
        reading_file = &mut (*(*file).cmds).fileinfo;
    } else {
        reading_file = 0 as *const floc;
    }
    result = variable_expand(line);
    current_variable_set_list = savev;
    reading_file = savef;
    return result;
}
unsafe extern "C" fn variable_append(
    mut name: *const i8,
    mut length: size_t,
    mut set: *const variable_set_list,
    mut local: i32,
) -> *mut i8 {
    let mut v: *const variable = 0 as *const variable;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut nextlocal: i32 = 0;
    if set.is_null() {
        return initialize_variable_output();
    }
    nextlocal = (local != 0 && (*set).next_is_parent == 0 as i32) as i32;
    v = lookup_variable_in_set(name, length, (*set).set);
    if v.is_null() || local == 0 && (*v).private_var() as i32 != 0 {
        return variable_append(name, length, (*set).next, nextlocal);
    }
    if (*v).append() != 0 {
        buf = variable_append(name, length, (*set).next, nextlocal);
    } else {
        buf = initialize_variable_output();
    }
    if buf > variable_buffer {
        buf = variable_buffer_output(
            buf,
            b" \0" as *const u8 as *const i8,
            1 as i32 as size_t,
        );
    }
    if (*v).recursive() == 0 {
        return variable_buffer_output(buf, (*v).value, strlen((*v).value));
    }
    buf = variable_expand_string(buf, (*v).value, strlen((*v).value));
    return buf.offset(strlen(buf) as isize);
}
unsafe extern "C" fn allocated_variable_append(mut v: *const variable) -> *mut i8 {
    let mut val: *mut i8 = 0 as *mut i8;
    let mut obuf: *mut i8 = variable_buffer;
    let mut olen: size_t = variable_buffer_length;
    variable_buffer = 0 as *mut i8;
    val = variable_append(
        (*v).name,
        strlen((*v).name),
        current_variable_set_list,
        1 as i32,
    );
    variable_buffer_output(val, b"\0" as *const u8 as *const i8, 1 as i32 as size_t);
    val = variable_buffer;
    variable_buffer = obuf;
    variable_buffer_length = olen;
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn allocated_variable_expand_for_file(
    mut line: *const i8,
    mut file: *mut file,
) -> *mut i8 {
    let mut value: *mut i8 = 0 as *mut i8;
    let mut obuf: *mut i8 = variable_buffer;
    let mut olen: size_t = variable_buffer_length;
    variable_buffer = 0 as *mut i8;
    value = variable_expand_for_file(line, file);
    variable_buffer = obuf;
    variable_buffer_length = olen;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn install_variable_buffer(
    mut bufp: *mut *mut i8,
    mut lenp: *mut size_t,
) {
    *bufp = variable_buffer;
    *lenp = variable_buffer_length;
    variable_buffer = 0 as *mut i8;
    initialize_variable_output();
}
#[no_mangle]
pub unsafe extern "C" fn restore_variable_buffer(mut buf: *mut i8, mut len: size_t) {
    free(variable_buffer as *mut libc::c_void);
    variable_buffer = buf;
    variable_buffer_length = len;
}
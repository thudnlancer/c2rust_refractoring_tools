#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
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
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn find_percent_cached(_: *mut *const i8) -> *const i8;
    fn dir_file_exists_p(_: *const i8, _: *const i8) -> i32;
    fn strcache_add_len(str: *const i8, len: size_t) -> *const i8;
    static mut posix_pedantic: i32;
    fn lookup_file(name: *const i8) -> *mut file;
    fn expand_extra_prereqs(extra: *const variable) -> *mut dep;
    fn parse_file_seq(
        stringp: *mut *mut i8,
        size: size_t,
        stopmap: i32,
        prefix: *const i8,
        flags: i32,
    ) -> *mut libc::c_void;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn print_commands(cmds: *const commands);
    fn lookup_variable(name: *const i8, length: size_t) -> *mut variable;
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
pub struct dep {
    pub next: *mut dep,
    pub name: *const i8,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const i8,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub next: *mut rule,
    pub targets: *mut *const i8,
    pub lens: *mut u32,
    pub suffixes: *mut *const i8,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub _defn: *mut i8,
    pub num: libc::c_ushort,
    pub terminal: i8,
    pub in_use: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const i8,
    pub dep: *const i8,
    pub commands: *const i8,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[no_mangle]
pub static mut pattern_rules: *mut rule = 0 as *const rule as *mut rule;
#[no_mangle]
pub static mut last_pattern_rule: *mut rule = 0 as *const rule as *mut rule;
#[no_mangle]
pub static mut num_pattern_rules: u32 = 0;
#[no_mangle]
pub static mut max_pattern_targets: u32 = 0;
#[no_mangle]
pub static mut max_pattern_deps: u32 = 0;
#[no_mangle]
pub static mut max_pattern_dep_length: size_t = 0;
#[no_mangle]
pub static mut suffix_file: *mut file = 0 as *const file as *mut file;
static mut maxsuffix: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn get_rule_defn(mut r: *mut rule) -> *const i8 {
    if ((*r)._defn).is_null() {
        let mut len: size_t = 8 as i32 as size_t;
        let mut k: u32 = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut sep: *const i8 = b"\0" as *const u8 as *const i8;
        let mut dep: *const dep = 0 as *const dep;
        let mut ood: *const dep = 0 as *const dep;
        k = 0 as i32 as u32;
        while k < (*r).num as u32 {
            len = (len as u64)
                .wrapping_add(
                    (*((*r).lens).offset(k as isize)).wrapping_add(1 as i32 as u32)
                        as u64,
                ) as size_t as size_t;
            k = k.wrapping_add(1);
            k;
        }
        dep = (*r).deps;
        while !dep.is_null() {
            len = (len as u64)
                .wrapping_add(
                    (strlen(
                        (if !((*dep).name).is_null() {
                            (*dep).name
                        } else {
                            (*(*dep).file).name
                        }),
                    ))
                        .wrapping_add(
                            (if (*dep).wait_here() as i32 != 0 {
                                (::core::mem::size_of::<[i8; 7]>() as u64)
                                    .wrapping_sub(1 as i32 as u64)
                            } else {
                                0 as i32 as u64
                            }),
                        )
                        .wrapping_add(1 as i32 as u64),
                ) as size_t as size_t;
            dep = (*dep).next;
        }
        (*r)._defn = xmalloc(len) as *mut i8;
        p = (*r)._defn;
        k = 0 as i32 as u32;
        while k < (*r).num as u32 {
            p = mempcpy(
                mempcpy(p as *mut libc::c_void, sep as *const libc::c_void, strlen(sep)),
                *((*r).targets).offset(k as isize) as *const libc::c_void,
                *((*r).lens).offset(k as isize) as size_t,
            ) as *mut i8;
            k = k.wrapping_add(1);
            k;
            sep = b" \0" as *const u8 as *const i8;
        }
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = ':' as i32 as i8;
        if (*r).terminal != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = ':' as i32 as i8;
        }
        dep = (*r).deps;
        while !dep.is_null() {
            if (*dep).ignore_mtime() as i32 == 0 as i32 {
                if (*dep).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut libc::c_void,
                        b" .WAIT\0" as *const u8 as *const i8 as *const libc::c_void,
                        (::core::mem::size_of::<[i8; 7]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) as *mut i8;
                }
                p = mempcpy(
                    mempcpy(
                        p as *mut libc::c_void,
                        b" \0" as *const u8 as *const i8 as *const libc::c_void,
                        1 as i32 as size_t,
                    ),
                    (if !((*dep).name).is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }) as *const libc::c_void,
                    strlen(
                        if !((*dep).name).is_null() {
                            (*dep).name
                        } else {
                            (*(*dep).file).name
                        },
                    ),
                ) as *mut i8;
            } else if ood.is_null() {
                ood = dep;
            }
            dep = (*dep).next;
        }
        sep = b" | \0" as *const u8 as *const i8;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                p = mempcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    strlen(sep),
                ) as *mut i8;
                if (*ood).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut libc::c_void,
                        b".WAIT \0" as *const u8 as *const i8 as *const libc::c_void,
                        (::core::mem::size_of::<[i8; 7]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) as *mut i8;
                }
                p = mempcpy(
                    p as *mut libc::c_void,
                    (if !((*ood).name).is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    }) as *const libc::c_void,
                    strlen(
                        if !((*ood).name).is_null() {
                            (*ood).name
                        } else {
                            (*(*ood).file).name
                        },
                    ),
                ) as *mut i8;
            }
            ood = (*ood).next;
            sep = b" \0" as *const u8 as *const i8;
        }
        *p = '\0' as i32 as i8;
    }
    return (*r)._defn;
}
#[no_mangle]
pub unsafe extern "C" fn snap_implicit_rules() {
    let mut name: *mut i8 = 0 as *mut i8;
    let mut namelen: size_t = 0 as i32 as size_t;
    let mut rule: *mut rule = 0 as *mut rule;
    let mut dep: *mut dep = 0 as *mut dep;
    let mut prereqs: *mut dep = expand_extra_prereqs(
        lookup_variable(
            b".EXTRA_PREREQS\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 15]>() as u64).wrapping_sub(1 as i32 as u64),
        ),
    );
    let mut pre_deps: u32 = 0 as i32 as u32;
    max_pattern_dep_length = 0 as i32 as size_t;
    dep = prereqs;
    while !dep.is_null() {
        let mut d: *const i8 = if !((*dep).name).is_null() {
            (*dep).name
        } else {
            (*(*dep).file).name
        };
        let mut l: size_t = strlen(d);
        if (*dep).need_2nd_expansion() != 0 {
            loop {
                d = strchr(d, '%' as i32);
                if d.is_null() {
                    break;
                }
                l = (l as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
                d = d.offset(1);
                d;
            }
        }
        if l > max_pattern_dep_length {
            max_pattern_dep_length = l;
        }
        pre_deps = pre_deps.wrapping_add(1);
        pre_deps;
        dep = (*dep).next;
    }
    max_pattern_deps = 0 as i32 as u32;
    max_pattern_targets = max_pattern_deps;
    num_pattern_rules = max_pattern_targets;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ndeps: u32 = pre_deps;
        let mut lastdep: *mut dep = 0 as *mut dep;
        num_pattern_rules = num_pattern_rules.wrapping_add(1);
        num_pattern_rules;
        if (*rule).num as u32 > max_pattern_targets {
            max_pattern_targets = (*rule).num as u32;
        }
        dep = (*rule).deps;
        while !dep.is_null() {
            let mut dname: *const i8 = if !((*dep).name).is_null() {
                (*dep).name
            } else {
                (*(*dep).file).name
            };
            let mut len: size_t = strlen(dname);
            let mut p: *const i8 = strrchr(dname, '/' as i32);
            let mut p2: *const i8 = if !p.is_null() {
                strchr(p, '%' as i32)
            } else {
                0 as *mut i8
            };
            ndeps = ndeps.wrapping_add(1);
            ndeps;
            if len > max_pattern_dep_length {
                max_pattern_dep_length = len;
            }
            if ((*dep).next).is_null() {
                lastdep = dep;
            }
            if !p2.is_null() {
                if p == dname {
                    p = p.offset(1);
                    p;
                }
                if p.offset_from(dname) as i64 as size_t > namelen {
                    namelen = p.offset_from(dname) as i64 as size_t;
                    name = xrealloc(
                        name as *mut libc::c_void,
                        namelen.wrapping_add(1 as i32 as u64),
                    ) as *mut i8;
                }
                memcpy(
                    name as *mut libc::c_void,
                    dname as *const libc::c_void,
                    p.offset_from(dname) as i64 as u64,
                );
                *name.offset(p.offset_from(dname) as i64 as isize) = '\0' as i32 as i8;
                (*dep)
                    .set_changed(
                        (dir_file_exists_p(name, b"\0" as *const u8 as *const i8) == 0)
                            as i32 as u32,
                    );
            } else {
                (*dep).set_changed(0 as i32 as u32);
            }
            dep = (*dep).next;
        }
        if !prereqs.is_null() {
            if !lastdep.is_null() {
                (*lastdep).next = copy_dep_chain(prereqs);
            } else {
                (*rule).deps = copy_dep_chain(prereqs);
            }
        }
        if ndeps > max_pattern_deps {
            max_pattern_deps = ndeps;
        }
        rule = (*rule).next;
    }
    free(name as *mut libc::c_void);
    free_ns_chain(prereqs as *mut nameseq);
}
unsafe extern "C" fn convert_suffix_rule(
    mut target: *const i8,
    mut source: *const i8,
    mut cmds: *mut commands,
) {
    let mut names: *mut *const i8 = 0 as *mut *const i8;
    let mut percents: *mut *const i8 = 0 as *mut *const i8;
    let mut deps: *mut dep = 0 as *mut dep;
    names = xmalloc(::core::mem::size_of::<*const i8>() as u64) as *mut *const i8;
    percents = xmalloc(::core::mem::size_of::<*const i8>() as u64) as *mut *const i8;
    if target.is_null() {
        *names = strcache_add_len(
            b"(%.o)\0" as *const u8 as *const i8,
            5 as i32 as size_t,
        );
        *percents = (*names).offset(1 as i32 as isize);
    } else {
        let mut len: size_t = strlen(target);
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (1 as i32 as u64).wrapping_add(len).wrapping_add(1 as i32 as u64) as usize,
        );
        let mut p: *mut i8 = fresh2.as_mut_ptr() as *mut i8;
        *p.offset(0 as i32 as isize) = '%' as i32 as i8;
        memcpy(
            p.offset(1 as i32 as isize) as *mut libc::c_void,
            target as *const libc::c_void,
            len.wrapping_add(1 as i32 as u64),
        );
        *names = strcache_add_len(p, len.wrapping_add(1 as i32 as u64));
        *percents = *names;
    }
    if source.is_null() {
        deps = 0 as *mut dep;
    } else {
        let mut len_0: size_t = strlen(source);
        let mut fresh3 = ::std::vec::from_elem(
            0,
            (1 as i32 as u64).wrapping_add(len_0).wrapping_add(1 as i32 as u64) as usize,
        );
        let mut p_0: *mut i8 = fresh3.as_mut_ptr() as *mut i8;
        *p_0.offset(0 as i32 as isize) = '%' as i32 as i8;
        memcpy(
            p_0.offset(1 as i32 as isize) as *mut libc::c_void,
            source as *const libc::c_void,
            len_0.wrapping_add(1 as i32 as u64),
        );
        deps = xcalloc(::core::mem::size_of::<dep>() as u64) as *mut dep;
        (*deps).name = strcache_add_len(p_0, len_0.wrapping_add(1 as i32 as u64));
    }
    create_pattern_rule(
        names,
        percents,
        1 as i32 as libc::c_ushort,
        0 as i32,
        deps,
        cmds,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn convert_to_pattern() {
    let mut d: *mut dep = 0 as *mut dep;
    let mut d2: *mut dep = 0 as *mut dep;
    let mut rulename: *mut i8 = 0 as *mut i8;
    maxsuffix = 0 as i32 as size_t;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut l: size_t = strlen(
            if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
        );
        if l > maxsuffix {
            maxsuffix = l;
        }
        d = (*d).next;
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        maxsuffix.wrapping_mul(2 as i32 as u64).wrapping_add(1 as i32 as u64) as usize,
    );
    rulename = fresh4.as_mut_ptr() as *mut i8;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut slen: size_t = 0;
        convert_suffix_rule(
            if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
            0 as *const i8,
            0 as *mut commands,
        );
        if !((*(*d).file).cmds).is_null() {
            convert_suffix_rule(
                b"\0" as *const u8 as *const i8,
                if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
                (*(*d).file).cmds,
            );
        }
        slen = strlen(
            if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
        );
        memcpy(
            rulename as *mut libc::c_void,
            (if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name })
                as *const libc::c_void,
            slen,
        );
        let mut current_block_20: u64;
        d2 = (*suffix_file).deps;
        while !d2.is_null() {
            let mut f: *mut file = 0 as *mut file;
            let mut s2len: size_t = 0;
            s2len = strlen(
                if !((*d2).name).is_null() { (*d2).name } else { (*(*d2).file).name },
            );
            if !(slen == s2len
                && ((if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name })
                    == (if !((*d2).name).is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    })
                    || *(if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as i32
                        == *(if !((*d2).name).is_null() {
                            (*d2).name
                        } else {
                            (*(*d2).file).name
                        }) as i32
                        && (*(if !((*d).name).is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) as i32 == '\0' as i32
                            || strcmp(
                                (if !((*d).name).is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                })
                                    .offset(1 as i32 as isize),
                                (if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                })
                                    .offset(1 as i32 as isize),
                            ) == 0)))
            {
                memcpy(
                    rulename.offset(slen as isize) as *mut libc::c_void,
                    (if !((*d2).name).is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    }) as *const libc::c_void,
                    s2len.wrapping_add(1 as i32 as u64),
                );
                f = lookup_file(rulename);
                if !(f.is_null() || ((*f).cmds).is_null()) {
                    if !((*f).deps).is_null() {
                        if posix_pedantic != 0 {
                            current_block_20 = 13586036798005543211;
                        } else {
                            error(
                                &mut (*(*f).cmds).fileinfo as *mut floc,
                                0 as i32 as size_t,
                                dcgettext(
                                    0 as *const i8,
                                    b"warning: ignoring prerequisites on suffix rule definition\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            current_block_20 = 5783071609795492627;
                        }
                    } else {
                        current_block_20 = 5783071609795492627;
                    }
                    match current_block_20 {
                        13586036798005543211 => {}
                        _ => {
                            if s2len == 2 as i32 as u64
                                && *rulename.offset(slen as isize) as i32 == '.' as i32
                                && *rulename
                                    .offset(slen.wrapping_add(1 as i32 as u64) as isize) as i32
                                    == 'a' as i32
                            {
                                convert_suffix_rule(
                                    0 as *const i8,
                                    if !((*d).name).is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    },
                                    (*f).cmds,
                                );
                            }
                            convert_suffix_rule(
                                if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                                if !((*d).name).is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                },
                                (*f).cmds,
                            );
                        }
                    }
                }
            }
            d2 = (*d2).next;
        }
        d = (*d).next;
    }
}
unsafe extern "C" fn new_pattern_rule(mut rule: *mut rule, mut override_0: i32) -> i32 {
    let mut r: *mut rule = 0 as *mut rule;
    let mut lastrule: *mut rule = 0 as *mut rule;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    (*rule).in_use = 0 as i32 as i8;
    (*rule).terminal = 0 as i32 as i8;
    (*rule).next = 0 as *mut rule;
    lastrule = 0 as *mut rule;
    r = pattern_rules;
    's_18: while !r.is_null() {
        i = 0 as i32 as u32;
        while i < (*rule).num as u32 {
            j = 0 as i32 as u32;
            while j < (*r).num as u32 {
                if !(*((*rule).targets).offset(i as isize)
                    == *((*r).targets).offset(j as isize)
                    || **((*rule).targets).offset(i as isize) as i32
                        == **((*r).targets).offset(j as isize) as i32
                        && (**((*rule).targets).offset(i as isize) as i32 == '\0' as i32
                            || strcmp(
                                (*((*rule).targets).offset(i as isize))
                                    .offset(1 as i32 as isize),
                                (*((*r).targets).offset(j as isize))
                                    .offset(1 as i32 as isize),
                            ) == 0))
                {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
            if j == (*r).num as u32 {
                let mut d: *mut dep = 0 as *mut dep;
                let mut d2: *mut dep = 0 as *mut dep;
                d = (*rule).deps;
                d2 = (*r).deps;
                while !d.is_null() && !d2.is_null() {
                    if !((if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    })
                        == (if !((*d2).name).is_null() {
                            (*d2).name
                        } else {
                            (*(*d2).file).name
                        })
                        || *(if !((*d).name).is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) as i32
                            == *(if !((*d2).name).is_null() {
                                (*d2).name
                            } else {
                                (*(*d2).file).name
                            }) as i32
                            && (*(if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }) as i32 == '\0' as i32
                                || strcmp(
                                    (if !((*d).name).is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    })
                                        .offset(1 as i32 as isize),
                                    (if !((*d2).name).is_null() {
                                        (*d2).name
                                    } else {
                                        (*(*d2).file).name
                                    })
                                        .offset(1 as i32 as isize),
                                ) == 0))
                    {
                        break;
                    }
                    d = (*d).next;
                    d2 = (*d2).next;
                }
                if d.is_null() && d2.is_null() {
                    if override_0 != 0 {
                        freerule(r, lastrule);
                        if pattern_rules.is_null() {
                            pattern_rules = rule;
                        } else {
                            (*last_pattern_rule).next = rule;
                        }
                        last_pattern_rule = rule;
                        break 's_18;
                    } else {
                        freerule(rule, 0 as *mut rule);
                        return 0 as i32;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        lastrule = r;
        r = (*r).next;
    }
    if r.is_null() {
        if pattern_rules.is_null() {
            pattern_rules = rule;
        } else {
            (*last_pattern_rule).next = rule;
        }
        last_pattern_rule = rule;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn install_pattern_rule(mut p: *mut pspec, mut terminal: i32) {
    let mut r: *mut rule = 0 as *mut rule;
    let mut ptr: *const i8 = 0 as *const i8;
    r = xmalloc(::core::mem::size_of::<rule>() as u64) as *mut rule;
    (*r).num = 1 as i32 as libc::c_ushort;
    (*r).targets = xmalloc(::core::mem::size_of::<*const i8>() as u64) as *mut *const i8;
    (*r).suffixes = xmalloc(::core::mem::size_of::<*const i8>() as u64)
        as *mut *const i8;
    (*r).lens = xmalloc(::core::mem::size_of::<u32>() as u64) as *mut u32;
    (*r)._defn = 0 as *mut i8;
    *((*r).lens).offset(0 as i32 as isize) = strlen((*p).target) as u32;
    let ref mut fresh5 = *((*r).targets).offset(0 as i32 as isize);
    *fresh5 = (*p).target;
    let ref mut fresh6 = *((*r).suffixes).offset(0 as i32 as isize);
    *fresh6 = find_percent_cached(&mut *((*r).targets).offset(0 as i32 as isize));
    let ref mut fresh7 = *((*r).suffixes).offset(0 as i32 as isize);
    *fresh7 = (*fresh7).offset(1);
    *fresh7;
    ptr = (*p).dep;
    (*r).deps = parse_file_seq(
        &mut ptr as *mut *const i8 as *mut *mut i8,
        ::core::mem::size_of::<dep>() as u64,
        0x1 as i32,
        0 as *const i8,
        0 as i32,
    ) as *mut dep;
    if new_pattern_rule(r, 0 as i32) != 0 {
        (*r).terminal = (if terminal != 0 { 1 as i32 } else { 0 as i32 }) as i8;
        (*r).cmds = xmalloc(::core::mem::size_of::<commands>() as u64) as *mut commands;
        (*(*r).cmds).fileinfo.filenm = 0 as *const i8;
        (*(*r).cmds).fileinfo.lineno = 0 as i32 as u64;
        (*(*r).cmds).fileinfo.offset = 0 as i32 as u64;
        (*(*r).cmds).commands = xstrdup((*p).commands);
        (*(*r).cmds).command_lines = 0 as *mut *mut i8;
        (*(*r).cmds).recipe_prefix = '\t' as i32 as i8;
    }
}
unsafe extern "C" fn freerule(mut rule: *mut rule, mut lastrule: *mut rule) {
    let mut next: *mut rule = (*rule).next;
    free_ns_chain((*rule).deps as *mut nameseq);
    free((*rule).targets as *mut libc::c_void);
    free((*rule).suffixes as *mut libc::c_void);
    free((*rule).lens as *mut libc::c_void);
    free((*rule)._defn as *mut libc::c_void);
    free(rule as *mut libc::c_void);
    if pattern_rules == rule {
        if !lastrule.is_null() {
            abort();
        } else {
            pattern_rules = next;
        }
    } else if !lastrule.is_null() {
        (*lastrule).next = next;
    }
    if last_pattern_rule == rule {
        last_pattern_rule = lastrule;
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_pattern_rule(
    mut targets: *mut *const i8,
    mut target_percents: *mut *const i8,
    mut n: libc::c_ushort,
    mut terminal: i32,
    mut deps: *mut dep,
    mut commands: *mut commands,
    mut override_0: i32,
) {
    let mut i: u32 = 0;
    let mut r: *mut rule = xmalloc(::core::mem::size_of::<rule>() as u64) as *mut rule;
    (*r).num = n;
    (*r).cmds = commands;
    (*r).deps = deps;
    (*r).targets = targets;
    (*r).suffixes = target_percents;
    (*r).lens = xmalloc((n as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64))
        as *mut u32;
    (*r)._defn = 0 as *mut i8;
    i = 0 as i32 as u32;
    while i < n as u32 {
        *((*r).lens).offset(i as isize) = strlen(*targets.offset(i as isize)) as u32;
        let ref mut fresh8 = *((*r).suffixes).offset(i as isize);
        *fresh8 = (*fresh8).offset(1);
        *fresh8;
        i = i.wrapping_add(1);
        i;
    }
    if new_pattern_rule(r, override_0) != 0 {
        (*r).terminal = (if terminal != 0 { 1 as i32 } else { 0 as i32 }) as i8;
    }
}
unsafe extern "C" fn print_rule(mut r: *mut rule) {
    fputs(get_rule_defn(r), stdout);
    putchar('\n' as i32);
    if !((*r).cmds).is_null() {
        print_commands((*r).cmds);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_rule_data_base() {
    let mut rules: u32 = 0;
    let mut terminal: u32 = 0;
    let mut r: *mut rule = 0 as *mut rule;
    puts(
        dcgettext(
            0 as *const i8,
            b"\n# Implicit Rules\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    terminal = 0 as i32 as u32;
    rules = terminal;
    r = pattern_rules;
    while !r.is_null() {
        rules = rules.wrapping_add(1);
        rules;
        putchar('\n' as i32);
        print_rule(r);
        if (*r).terminal != 0 {
            terminal = terminal.wrapping_add(1);
            terminal;
        }
        r = (*r).next;
    }
    if rules == 0 as i32 as u32 {
        puts(
            dcgettext(
                0 as *const i8,
                b"\n# No implicit rules.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const i8,
                b"\n# %u implicit rules, %u (%.1f%%) terminal.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            rules,
            terminal,
            terminal as libc::c_double / rules as libc::c_double * 100.0f64,
        );
    }
    if num_pattern_rules != rules {
        if num_pattern_rules != 0 as i32 as u32 {
            fatal(
                0 as *mut floc,
                (53 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                    .wrapping_div(22 as i32 as u64)
                    .wrapping_add(3 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64),
                dcgettext(
                    0 as *const i8,
                    b"BUG: num_pattern_rules is wrong!  %u != %u\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                num_pattern_rules,
                rules,
            );
        }
    }
}
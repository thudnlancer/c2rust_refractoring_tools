use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn lindex(_: *const i8, _: *const i8, _: i32) -> *mut i8;
    fn print_spaces(_: u32);
    fn ar_name(_: *const i8) -> i32;
    fn file_exists_p(_: *const i8) -> i32;
    fn file_impossible_p(_: *const i8) -> i32;
    fn file_impossible(_: *const i8);
    fn vpath_search(
        file: *const i8,
        mtime_ptr: *mut uintmax_t,
        vpath_index: *mut u32,
        path_index: *mut u32,
    ) -> *const i8;
    fn strcache_add(str: *const i8) -> *const i8;
    fn strcache_add_len(str: *const i8, len: size_t) -> *const i8;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut no_intermediates: u32;
    fn lookup_file(name: *const i8) -> *mut file;
    fn enter_file(name: *const i8) -> *mut file;
    static mut pattern_rules: *mut rule;
    static mut num_pattern_rules: u32;
    static mut max_pattern_deps: u32;
    static mut max_pattern_targets: u32;
    static mut max_pattern_dep_length: size_t;
    fn get_rule_defn(rule: *mut rule) -> *const i8;
    fn parse_file_seq(
        stringp: *mut *mut i8,
        size: size_t,
        stopmap: i32,
        prefix: *const i8,
        flags: i32,
    ) -> *mut libc::c_void;
    fn free_ns_chain(n: *mut nameseq);
    static mut db_level: i32;
    fn variable_expand_for_file(line: *const i8, file: *mut file) -> *mut i8;
    fn free_variable_set(_: *mut variable_set_list);
    fn initialize_file_variables(file: *mut file, reading: i32);
    fn merge_variable_set_lists(
        to_list: *mut *mut variable_set_list,
        from_list: *mut variable_set_list,
    );
    fn define_variable_in_set(
        name: *const i8,
        length: size_t,
        value: *const i8,
        origin: variable_origin,
        recursive: i32,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    fn set_file_variables(file: *mut file, stem: *const i8);
    fn shuffle_deps_recursive(g: *mut dep);
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct patdeps {
    pub name: *const i8,
    pub pattern: *const i8,
    pub file: *mut file,
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "3..=3")]
    pub ignore_mtime_ignore_automatic_vars_is_explicit_wait_here: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tryrule {
    pub rule: *mut rule,
    pub stemlen: size_t,
    pub matches: u32,
    pub order: u32,
    pub checked_lastslash: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[no_mangle]
pub unsafe extern "C" fn try_implicit_rule(mut file: *mut file, mut depth: u32) -> i32 {
    if 0x8 as i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const i8,
                b"Looking for an implicit rule for '%s'.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if pattern_search(file, 0 as i32, depth, 0 as i32 as u32, 0 as i32) != 0 {
        return 1 as i32;
    }
    if ar_name((*file).name) != 0 {
        if 0x8 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Looking for archive-member implicit rule for '%s'.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        if pattern_search(file, 1 as i32, depth, 0 as i32 as u32, 0 as i32) != 0 {
            return 1 as i32;
        }
        if 0x8 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"No archive-member implicit rule found for '%s'.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn get_next_word(
    mut buffer: *const i8,
    mut length: *mut size_t,
) -> *const i8 {
    let mut current_block: u64;
    let mut p: *const i8 = buffer;
    let mut beg: *const i8 = 0 as *const i8;
    let mut c: i8 = 0;
    while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        p = p.offset(1);
        p;
    }
    beg = p;
    let fresh0 = p;
    p = p.offset(1);
    c = *fresh0;
    if c as i32 == '\0' as i32 {
        return 0 as *const i8;
    }
    loop {
        let mut closeparen: i8 = 0;
        let mut count: i32 = 0;
        match c as i32 {
            0 | 32 | 9 => {
                current_block = 14504264955210041902;
                break;
            }
            36 => {
                let fresh1 = p;
                p = p.offset(1);
                c = *fresh1;
                if !(c as i32 == '$' as i32) {
                    if c as i32 == '(' as i32 {
                        closeparen = ')' as i32 as i8;
                        current_block = 10599921512955367680;
                    } else if c as i32 == '{' as i32 {
                        closeparen = '}' as i32 as i8;
                        current_block = 10599921512955367680;
                    } else {
                        current_block = 2668756484064249700;
                    }
                    match current_block {
                        2668756484064249700 => {}
                        _ => {
                            count = 0 as i32;
                            while *p as i32 != '\0' as i32 {
                                if *p as i32 == c as i32 {
                                    count += 1;
                                    count;
                                } else if *p as i32 == closeparen as i32
                                    && {
                                        count -= 1;
                                        count < 0 as i32
                                    }
                                {
                                    p = p.offset(1);
                                    p;
                                    break;
                                }
                                p = p.offset(1);
                                p;
                            }
                        }
                    }
                }
            }
            124 => {
                current_block = 7724970128870605577;
                break;
            }
            _ => {}
        }
        let fresh2 = p;
        p = p.offset(1);
        c = *fresh2;
    }
    match current_block {
        14504264955210041902 => {
            p = p.offset(-1);
            p;
        }
        _ => {}
    }
    if !length.is_null() {
        *length = p.offset_from(beg) as i64 as size_t;
    }
    return beg;
}
#[no_mangle]
pub unsafe extern "C" fn stemlen_compare(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> i32 {
    let mut r1: *const tryrule = v1 as *const tryrule;
    let mut r2: *const tryrule = v2 as *const tryrule;
    let mut r: i32 = ((*r1).stemlen).wrapping_sub((*r2).stemlen) as i32;
    return if r != 0 as i32 {
        r
    } else {
        ((*r1).order).wrapping_sub((*r2).order) as i32
    };
}
unsafe extern "C" fn pattern_search(
    mut file: *mut file,
    mut archive: i32,
    mut depth: u32,
    mut recursions: u32,
    mut allow_compat_rules: i32,
) -> i32 {
    let mut filename: *const i8 = if archive != 0 {
        strchr((*file).name, '(' as i32)
    } else {
        (*file).name
    };
    let mut namelen: size_t = strlen(filename);
    let mut lastslash: *const i8 = 0 as *const i8;
    let mut int_file: *mut file = 0 as *mut file;
    let mut max_deps: u32 = max_pattern_deps;
    let mut deplist: *mut patdeps = xmalloc(
        (max_deps as u64).wrapping_mul(::core::mem::size_of::<patdeps>() as u64),
    ) as *mut patdeps;
    let mut pat: *mut patdeps = deplist;
    let mut deplen: size_t = namelen
        .wrapping_add(max_pattern_dep_length)
        .wrapping_add(4 as i32 as u64);
    let mut fresh3 = ::std::vec::from_elem(0, deplen as usize);
    let mut depname: *mut i8 = fresh3.as_mut_ptr() as *mut i8;
    let mut stem: *const i8 = 0 as *const i8;
    let mut stemlen: size_t = 0 as i32 as size_t;
    let mut fullstemlen: size_t = 0 as i32 as size_t;
    let mut tryrules: *mut tryrule = xmalloc(
        (num_pattern_rules.wrapping_mul(max_pattern_targets) as u64)
            .wrapping_mul(::core::mem::size_of::<tryrule>() as u64),
    ) as *mut tryrule;
    let mut nrules: u32 = 0;
    let mut foundrule: u32 = 0;
    let mut intermed_ok: i32 = 0;
    let mut file_vars_initialized: i32 = 0 as i32;
    let mut specific_rule_matched: i32 = 0 as i32;
    let mut ri: u32 = 0;
    let mut found_compat_rule: i32 = 0 as i32;
    let mut rule: *mut rule = 0 as *mut rule;
    let mut pathdir: *mut i8 = 0 as *mut i8;
    let mut pathlen: size_t = 0;
    let mut stem_str: [i8; 4097] = [0; 4097];
    depth = depth.wrapping_add(1);
    depth;
    if archive != 0 || ar_name(filename) != 0 {
        lastslash = 0 as *const i8;
    } else {
        lastslash = memrchr(
            filename as *const libc::c_void,
            '/' as i32,
            namelen.wrapping_sub(1 as i32 as u64),
        ) as *const i8;
    }
    pathlen = (if !lastslash.is_null() {
        lastslash.offset_from(filename) as i64 + 1 as i32 as i64
    } else {
        0 as i32 as i64
    }) as size_t;
    nrules = 0 as i32 as u32;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ti: u32 = 0;
        if !(!((*rule).deps).is_null() && ((*rule).cmds).is_null()) {
            if (*rule).in_use != 0 {
                if 0x8 as i32 & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"Avoiding implicit rule recursion for rule '%s'.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        get_rule_defn(rule),
                    );
                    fflush(stdout);
                }
            } else {
                let mut current_block_31: u64;
                ti = 0 as i32 as u32;
                while ti < (*rule).num as u32 {
                    let mut target: *const i8 = *((*rule).targets).offset(ti as isize);
                    let mut suffix: *const i8 = *((*rule).suffixes).offset(ti as isize);
                    let mut check_lastslash: i8 = 0;
                    if !(recursions > 0 as i32 as u32
                        && *target.offset(1 as i32 as isize) as i32 == '\0' as i32
                        && (*rule).terminal == 0)
                    {
                        if !(*((*rule).lens).offset(ti as isize) as u64 > namelen) {
                            stem = filename
                                .offset(
                                    (suffix.offset_from(target) as i64 - 1 as i32 as i64)
                                        as isize,
                                );
                            stemlen = namelen
                                .wrapping_sub(*((*rule).lens).offset(ti as isize) as u64)
                                .wrapping_add(1 as i32 as u64);
                            check_lastslash = 0 as i32 as i8;
                            if !lastslash.is_null() {
                                check_lastslash = (strchr(target, '/' as i32)
                                    == 0 as *mut i8) as i32 as i8;
                            }
                            if check_lastslash != 0 {
                                if pathlen > stemlen {
                                    current_block_31 = 16203760046146113240;
                                } else {
                                    stemlen = (stemlen as u64).wrapping_sub(pathlen) as size_t
                                        as size_t;
                                    stem = stem.offset(pathlen as isize);
                                    current_block_31 = 8845338526596852646;
                                }
                            } else {
                                current_block_31 = 8845338526596852646;
                            }
                            match current_block_31 {
                                16203760046146113240 => {}
                                _ => {
                                    if check_lastslash != 0 {
                                        if stem > lastslash.offset(1 as i32 as isize)
                                            && !(strncmp(
                                                target,
                                                lastslash.offset(1 as i32 as isize),
                                                (stem.offset_from(lastslash) as i64 - 1 as i32 as i64)
                                                    as u64,
                                            ) == 0 as i32)
                                        {
                                            current_block_31 = 16203760046146113240;
                                        } else {
                                            current_block_31 = 10891380440665537214;
                                        }
                                    } else if stem > filename
                                        && !(strncmp(
                                            target,
                                            filename,
                                            stem.offset_from(filename) as i64 as u64,
                                        ) == 0 as i32)
                                    {
                                        current_block_31 = 16203760046146113240;
                                    } else {
                                        current_block_31 = 10891380440665537214;
                                    }
                                    match current_block_31 {
                                        16203760046146113240 => {}
                                        _ => {
                                            if !(*suffix as i32 != *stem.offset(stemlen as isize) as i32
                                                || *suffix as i32 != '\0' as i32
                                                    && !(&*suffix.offset(1 as i32 as isize) as *const i8
                                                        == &*stem
                                                            .offset(stemlen.wrapping_add(1 as i32 as u64) as isize)
                                                            as *const i8
                                                        || *suffix.offset(1 as i32 as isize) as i32
                                                            == *stem
                                                                .offset(stemlen.wrapping_add(1 as i32 as u64) as isize)
                                                                as i32
                                                            && (*suffix.offset(1 as i32 as isize) as i32 == '\0' as i32
                                                                || strcmp(
                                                                    (&*suffix.offset(1 as i32 as isize) as *const i8)
                                                                        .offset(1 as i32 as isize),
                                                                    (&*stem
                                                                        .offset(stemlen.wrapping_add(1 as i32 as u64) as isize)
                                                                        as *const i8)
                                                                        .offset(1 as i32 as isize),
                                                                ) == 0)))
                                            {
                                                if *target.offset(1 as i32 as isize) as i32 != '\0' as i32 {
                                                    specific_rule_matched = 1 as i32;
                                                }
                                                if !(((*rule).deps).is_null() && ((*rule).cmds).is_null()) {
                                                    let ref mut fresh4 = (*tryrules.offset(nrules as isize))
                                                        .rule;
                                                    *fresh4 = rule;
                                                    (*tryrules.offset(nrules as isize)).matches = ti;
                                                    (*tryrules.offset(nrules as isize)).stemlen = stemlen
                                                        .wrapping_add(
                                                            (if check_lastslash as i32 != 0 {
                                                                pathlen
                                                            } else {
                                                                0 as i32 as u64
                                                            }),
                                                        );
                                                    (*tryrules.offset(nrules as isize)).order = nrules;
                                                    (*tryrules.offset(nrules as isize)).checked_lastslash = check_lastslash;
                                                    nrules = nrules.wrapping_add(1);
                                                    nrules;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ti = ti.wrapping_add(1);
                    ti;
                }
            }
        }
        rule = (*rule).next;
    }
    if !(nrules == 0 as i32 as u32) {
        if nrules > 1 as i32 as u32 {
            qsort(
                tryrules as *mut libc::c_void,
                nrules as size_t,
                ::core::mem::size_of::<tryrule>() as u64,
                Some(
                    stemlen_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> i32,
                ),
            );
        }
        if specific_rule_matched != 0 {
            ri = 0 as i32 as u32;
            while ri < nrules {
                if (*(*tryrules.offset(ri as isize)).rule).terminal == 0 {
                    let mut j: u32 = 0;
                    j = 0 as i32 as u32;
                    while j < (*(*tryrules.offset(ri as isize)).rule).num as u32 {
                        if *(*((*(*tryrules.offset(ri as isize)).rule).targets)
                            .offset(j as isize))
                            .offset(1 as i32 as isize) as i32 == '\0' as i32
                        {
                            let ref mut fresh5 = (*tryrules.offset(ri as isize)).rule;
                            *fresh5 = 0 as *mut rule;
                            break;
                        } else {
                            j = j.wrapping_add(1);
                            j;
                        }
                    }
                }
                ri = ri.wrapping_add(1);
                ri;
            }
        }
        intermed_ok = 0 as i32;
        while intermed_ok < 2 as i32 {
            pat = deplist;
            if intermed_ok != 0 {
                if 0x8 as i32 & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"Trying harder.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    fflush(stdout);
                }
            }
            ri = 0 as i32 as u32;
            while ri < nrules {
                let mut dep: *mut dep = 0 as *mut dep;
                let mut check_lastslash_0: i8 = 0;
                let mut failed: u32 = 0 as i32 as u32;
                let mut file_variables_set: i32 = 0 as i32;
                let mut deps_found: u32 = 0 as i32 as u32;
                let mut nptr: *const i8 = 0 as *const i8;
                let mut order_only: i32 = 0 as i32;
                let mut matches: u32 = 0;
                rule = (*tryrules.offset(ri as isize)).rule;
                if !rule.is_null() {
                    if !(intermed_ok != 0 && (*rule).terminal as i32 != 0) {
                        matches = (*tryrules.offset(ri as isize)).matches;
                        stem = filename
                            .offset(
                                (*((*rule).suffixes).offset(matches as isize))
                                    .offset_from(*((*rule).targets).offset(matches as isize))
                                    as i64 as isize,
                            )
                            .offset(-(1 as i32 as isize));
                        stemlen = namelen
                            .wrapping_sub(
                                *((*rule).lens).offset(matches as isize) as u64,
                            )
                            .wrapping_add(1 as i32 as u64);
                        check_lastslash_0 = (*tryrules.offset(ri as isize))
                            .checked_lastslash;
                        if check_lastslash_0 != 0 {
                            stem = stem.offset(pathlen as isize);
                            stemlen = (stemlen as u64).wrapping_sub(pathlen) as size_t
                                as size_t;
                            if pathdir.is_null() {
                                let mut fresh6 = ::std::vec::from_elem(
                                    0,
                                    pathlen.wrapping_add(1 as i32 as u64) as usize,
                                );
                                pathdir = fresh6.as_mut_ptr() as *mut i8;
                                memcpy(
                                    pathdir as *mut libc::c_void,
                                    filename as *const libc::c_void,
                                    pathlen,
                                );
                                *pathdir.offset(pathlen as isize) = '\0' as i32 as i8;
                            }
                        }
                        if 0x8 as i32 & db_level != 0 {
                            print_spaces(depth);
                            printf(
                                dcgettext(
                                    0 as *const i8,
                                    b"Trying pattern rule '%s' with stem '%.*s'.\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                get_rule_defn(rule),
                                stemlen as i32,
                                stem,
                            );
                            fflush(stdout);
                        }
                        if stemlen
                            .wrapping_add(
                                (if check_lastslash_0 as i32 != 0 {
                                    pathlen
                                } else {
                                    0 as i32 as u64
                                }),
                            ) > 4096 as i32 as u64
                        {
                            if 0x8 as i32 & db_level != 0 {
                                print_spaces(depth);
                                printf(
                                    dcgettext(
                                        0 as *const i8,
                                        b"Stem too long: '%s%.*s'.\n\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    if check_lastslash_0 as i32 != 0 {
                                        pathdir
                                    } else {
                                        b"\0" as *const u8 as *const i8
                                    },
                                    stemlen as i32,
                                    stem,
                                );
                                fflush(stdout);
                            }
                        } else {
                            if check_lastslash_0 == 0 {
                                memcpy(
                                    stem_str.as_mut_ptr() as *mut libc::c_void,
                                    stem as *const libc::c_void,
                                    stemlen,
                                );
                                stem_str[stemlen as usize] = '\0' as i32 as i8;
                            } else {
                                memcpy(
                                    stem_str.as_mut_ptr() as *mut libc::c_void,
                                    filename as *const libc::c_void,
                                    pathlen,
                                );
                                memcpy(
                                    stem_str.as_mut_ptr().offset(pathlen as isize)
                                        as *mut libc::c_void,
                                    stem as *const libc::c_void,
                                    stemlen,
                                );
                                stem_str[pathlen.wrapping_add(stemlen) as usize] = '\0'
                                    as i32 as i8;
                            }
                            if ((*rule).deps).is_null() {
                                break;
                            }
                            (*rule).in_use = 1 as i32 as i8;
                            pat = deplist;
                            dep = (*rule).deps;
                            nptr = if !((*dep).name).is_null() {
                                (*dep).name
                            } else {
                                (*(*dep).file).name
                            };
                            loop {
                                let mut dl: *mut dep = 0 as *mut dep;
                                let mut d: *mut dep = 0 as *mut dep;
                                if nptr.is_null() {
                                    dep = (*dep).next;
                                    if dep.is_null() {
                                        break;
                                    }
                                    nptr = if !((*dep).name).is_null() {
                                        (*dep).name
                                    } else {
                                        (*(*dep).file).name
                                    };
                                }
                                if (*dep).need_2nd_expansion() == 0 {
                                    let mut p: *mut i8 = 0 as *mut i8;
                                    let mut is_explicit: i32 = 1 as i32;
                                    let mut cp: *const i8 = strchr(nptr, '%' as i32);
                                    if cp.is_null() {
                                        strcpy(depname, nptr);
                                    } else {
                                        let mut o: *mut i8 = depname;
                                        if check_lastslash_0 != 0 {
                                            o = mempcpy(
                                                o as *mut libc::c_void,
                                                filename as *const libc::c_void,
                                                pathlen,
                                            ) as *mut i8;
                                        }
                                        o = mempcpy(
                                            o as *mut libc::c_void,
                                            nptr as *const libc::c_void,
                                            cp.offset_from(nptr) as i64 as size_t,
                                        ) as *mut i8;
                                        o = mempcpy(
                                            o as *mut libc::c_void,
                                            stem as *const libc::c_void,
                                            stemlen,
                                        ) as *mut i8;
                                        strcpy(o, cp.offset(1 as i32 as isize));
                                        is_explicit = 0 as i32;
                                    }
                                    p = depname;
                                    dl = parse_file_seq(
                                        &mut p,
                                        ::core::mem::size_of::<dep>() as u64,
                                        0x1 as i32,
                                        0 as *const i8,
                                        0x20 as i32 | 0x40 as i32,
                                    ) as *mut dep;
                                    d = dl;
                                    while !d.is_null() {
                                        deps_found = deps_found.wrapping_add(1);
                                        deps_found;
                                        (*d).set_ignore_mtime((*dep).ignore_mtime());
                                        (*d)
                                            .set_ignore_automatic_vars((*dep).ignore_automatic_vars());
                                        (*d)
                                            .set_wait_here(
                                                (*d).wait_here() | (*dep).wait_here() as i32 as u32,
                                            );
                                        (*d).set_is_explicit(is_explicit as u32);
                                        d = (*d).next;
                                    }
                                    nptr = 0 as *const i8;
                                } else {
                                    let mut add_dir: i32 = 0 as i32;
                                    let mut len: size_t = 0;
                                    let mut end: *const i8 = 0 as *const i8;
                                    let mut dptr: *mut *mut dep = 0 as *mut *mut dep;
                                    let mut is_explicit_0: i32 = 0;
                                    let mut cp_0: *const i8 = 0 as *const i8;
                                    let mut p_0: *mut i8 = 0 as *mut i8;
                                    nptr = get_next_word(nptr, &mut len);
                                    if nptr.is_null() {
                                        continue;
                                    }
                                    end = nptr.offset(len as isize);
                                    if order_only == 0 && len == 1 as i32 as u64
                                        && *nptr.offset(0 as i32 as isize) as i32 == '|' as i32
                                    {
                                        order_only = 1 as i32;
                                        nptr = end;
                                        continue;
                                    } else {
                                        cp_0 = lindex(nptr, end, '%' as i32);
                                        if cp_0.is_null() {
                                            memcpy(
                                                depname as *mut libc::c_void,
                                                nptr as *const libc::c_void,
                                                len,
                                            );
                                            *depname.offset(len as isize) = '\0' as i32 as i8;
                                            is_explicit_0 = 1 as i32;
                                        } else {
                                            let mut o_0: *mut i8 = depname;
                                            is_explicit_0 = 0 as i32;
                                            loop {
                                                let mut i: size_t = cp_0.offset_from(nptr) as i64 as size_t;
                                                o_0 = mempcpy(
                                                    o_0 as *mut libc::c_void,
                                                    nptr as *const libc::c_void,
                                                    i,
                                                ) as *mut i8;
                                                if check_lastslash_0 != 0 {
                                                    add_dir = 1 as i32;
                                                    o_0 = mempcpy(
                                                        o_0 as *mut libc::c_void,
                                                        b"$(*F)\0" as *const u8 as *const i8 as *const libc::c_void,
                                                        5 as i32 as size_t,
                                                    ) as *mut i8;
                                                } else {
                                                    o_0 = mempcpy(
                                                        o_0 as *mut libc::c_void,
                                                        b"$*\0" as *const u8 as *const i8 as *const libc::c_void,
                                                        2 as i32 as size_t,
                                                    ) as *mut i8;
                                                }
                                                cp_0 = cp_0.offset(1);
                                                cp_0;
                                                nptr = cp_0;
                                                if nptr == end {
                                                    break;
                                                }
                                                while cp_0 < end
                                                    && !(*stopchar_map.as_mut_ptr().offset(*cp_0 as u8 as isize)
                                                        as i32 & (0x2 as i32 | 0x4 as i32 | 0x1 as i32) != 0 as i32)
                                                {
                                                    cp_0 = cp_0.offset(1);
                                                    cp_0;
                                                }
                                                cp_0 = lindex(cp_0, end, '%' as i32);
                                                if cp_0.is_null() {
                                                    break;
                                                }
                                            }
                                            len = end.offset_from(nptr) as i64 as size_t;
                                            memcpy(
                                                o_0 as *mut libc::c_void,
                                                nptr as *const libc::c_void,
                                                len,
                                            );
                                            *o_0.offset(len as isize) = '\0' as i32 as i8;
                                        }
                                        nptr = end;
                                        if file_vars_initialized == 0 {
                                            initialize_file_variables(file, 0 as i32);
                                            set_file_variables(file, stem_str.as_mut_ptr());
                                            file_vars_initialized = 1 as i32;
                                        } else if file_variables_set == 0 {
                                            define_variable_in_set(
                                                b"*\0" as *const u8 as *const i8,
                                                1 as i32 as size_t,
                                                stem_str.as_mut_ptr(),
                                                variable_origin::o_automatic,
                                                0 as i32,
                                                (*(*file).variables).set,
                                                0 as *mut floc,
                                            );
                                            file_variables_set = 1 as i32;
                                        }
                                        p_0 = variable_expand_for_file(depname, file);
                                        dptr = &mut dl;
                                        loop {
                                            let mut dp: *mut dep = parse_file_seq(
                                                &mut p_0,
                                                ::core::mem::size_of::<dep>() as u64,
                                                if order_only != 0 { 0x1 as i32 } else { 0x100 as i32 },
                                                if add_dir != 0 { pathdir } else { 0 as *mut i8 },
                                                0x40 as i32,
                                            ) as *mut dep;
                                            *dptr = dp;
                                            d = dp;
                                            while !d.is_null() {
                                                deps_found = deps_found.wrapping_add(1);
                                                deps_found;
                                                if order_only != 0 {
                                                    (*d).set_ignore_mtime(1 as i32 as u32);
                                                }
                                                (*d).set_is_explicit(is_explicit_0 as u32);
                                                dptr = &mut (*d).next;
                                                d = (*d).next;
                                            }
                                            if *p_0 as i32 == '|' as i32 {
                                                order_only = 1 as i32;
                                                p_0 = p_0.offset(1);
                                                p_0;
                                            }
                                            if !(*p_0 as i32 != '\0' as i32) {
                                                break;
                                            }
                                        }
                                    }
                                }
                                if deps_found > max_deps {
                                    let mut l: size_t = pat.offset_from(deplist) as i64
                                        as size_t;
                                    max_pattern_deps = if max_pattern_deps > deps_found {
                                        max_pattern_deps
                                    } else {
                                        deps_found
                                    };
                                    max_deps = max_pattern_deps;
                                    deplist = xrealloc(
                                        deplist as *mut libc::c_void,
                                        (max_deps as u64)
                                            .wrapping_mul(::core::mem::size_of::<patdeps>() as u64),
                                    ) as *mut patdeps;
                                    pat = deplist.offset(l as isize);
                                }
                                let mut current_block_294: u64;
                                d = dl;
                                while !d.is_null() {
                                    let mut df: *mut file = 0 as *mut file;
                                    let mut is_rule: i32 = ((*d).name
                                        == (if !((*dep).name).is_null() {
                                            (*dep).name
                                        } else {
                                            (*(*dep).file).name
                                        })) as i32;
                                    let mut explicit: i32 = 0 as i32;
                                    let mut dp_0: *mut dep = 0 as *mut dep;
                                    if file_impossible_p((*d).name) != 0 {
                                        if 0x8 as i32 & db_level != 0 {
                                            print_spaces(depth);
                                            printf(
                                                if is_rule != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Rejecting rule '%s' due to impossible rule prerequisite '%s'.\n\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Rejecting rule '%s' due to impossible implicit prerequisite '%s'.\n\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                get_rule_defn(rule),
                                                (*d).name,
                                            );
                                            fflush(stdout);
                                        }
                                        let ref mut fresh7 = (*tryrules.offset(ri as isize)).rule;
                                        *fresh7 = 0 as *mut rule;
                                        failed = 1 as i32 as u32;
                                        break;
                                    } else {
                                        memset(
                                            pat as *mut libc::c_void,
                                            '\0' as i32,
                                            ::core::mem::size_of::<patdeps>() as u64,
                                        );
                                        (*pat).set_ignore_mtime((*d).ignore_mtime());
                                        (*pat)
                                            .set_ignore_automatic_vars((*d).ignore_automatic_vars());
                                        (*pat).set_wait_here((*d).wait_here());
                                        (*pat).set_is_explicit((*d).is_explicit());
                                        if 0x8 as i32 & db_level != 0 {
                                            print_spaces(depth);
                                            printf(
                                                if is_rule != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Trying rule prerequisite '%s'.\n\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Trying implicit prerequisite '%s'.\n\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                (*d).name,
                                            );
                                            fflush(stdout);
                                        }
                                        df = lookup_file((*d).name);
                                        if !df.is_null() && (*df).is_explicit() as i32 != 0 {
                                            (*pat).set_is_explicit(1 as i32 as u32);
                                        }
                                        if !df.is_null() && (*df).is_explicit() == 0
                                            && (*d).is_explicit() == 0
                                        {
                                            (*df).set_intermediate(1 as i32 as u32);
                                        }
                                        if !df.is_null() && (*df).is_target() as i32 != 0 {
                                            explicit = 1 as i32;
                                        } else {
                                            dp_0 = (*file).deps;
                                            while !dp_0.is_null() {
                                                if (*d).name
                                                    == (if !((*dp_0).name).is_null() {
                                                        (*dp_0).name
                                                    } else {
                                                        (*(*dp_0).file).name
                                                    })
                                                    || *(*d).name as i32
                                                        == *(if !((*dp_0).name).is_null() {
                                                            (*dp_0).name
                                                        } else {
                                                            (*(*dp_0).file).name
                                                        }) as i32
                                                        && (*(*d).name as i32 == '\0' as i32
                                                            || strcmp(
                                                                ((*d).name).offset(1 as i32 as isize),
                                                                (if !((*dp_0).name).is_null() {
                                                                    (*dp_0).name
                                                                } else {
                                                                    (*(*dp_0).file).name
                                                                })
                                                                    .offset(1 as i32 as isize),
                                                            ) == 0)
                                                {
                                                    break;
                                                }
                                                dp_0 = (*dp_0).next;
                                            }
                                        }
                                        if explicit != 0 || !dp_0.is_null() {
                                            let fresh8 = pat;
                                            pat = pat.offset(1);
                                            (*fresh8).name = (*d).name;
                                            if 0x8 as i32 & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"'%s' ought to exist.\n\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else if file_exists_p((*d).name) != 0 {
                                            let fresh9 = pat;
                                            pat = pat.offset(1);
                                            (*fresh9).name = (*d).name;
                                            if 0x8 as i32 & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Found '%s'.\n\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else if !df.is_null() && allow_compat_rules != 0 {
                                            let fresh10 = pat;
                                            pat = pat.offset(1);
                                            (*fresh10).name = (*d).name;
                                            if 0x8 as i32 & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Using compatibility rule '%s' due to '%s'.\n\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    get_rule_defn(rule),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else {
                                            if !df.is_null() {
                                                if 0x8 as i32 & db_level != 0 {
                                                    print_spaces(depth);
                                                    printf(
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"Prerequisite '%s' of rule '%s' does not qualify as ought to exist.\n\0"
                                                                as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                        (*d).name,
                                                        get_rule_defn(rule),
                                                    );
                                                    fflush(stdout);
                                                }
                                                found_compat_rule = 1 as i32;
                                            }
                                            let mut vname: *const i8 = vpath_search(
                                                (*d).name,
                                                0 as *mut uintmax_t,
                                                0 as *mut u32,
                                                0 as *mut u32,
                                            );
                                            if !vname.is_null() {
                                                if 0x8 as i32 & db_level != 0 {
                                                    print_spaces(depth);
                                                    printf(
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"Found prerequisite '%s' as VPATH '%s'.\n\0" as *const u8
                                                                as *const i8,
                                                            5 as i32,
                                                        ),
                                                        (*d).name,
                                                        vname,
                                                    );
                                                    fflush(stdout);
                                                }
                                                let fresh11 = pat;
                                                pat = pat.offset(1);
                                                (*fresh11).name = (*d).name;
                                            } else {
                                                if intermed_ok != 0 {
                                                    if 0x8 as i32 & db_level != 0 {
                                                        print_spaces(depth);
                                                        printf(
                                                            if (*d).is_explicit() as i32 != 0
                                                                || !df.is_null() && (*df).is_explicit() as i32 != 0
                                                            {
                                                                dcgettext(
                                                                    0 as *const i8,
                                                                    b"Looking for a rule with explicit file '%s'.\n\0"
                                                                        as *const u8 as *const i8,
                                                                    5 as i32,
                                                                )
                                                            } else {
                                                                dcgettext(
                                                                    0 as *const i8,
                                                                    b"Looking for a rule with intermediate file '%s'.\n\0"
                                                                        as *const u8 as *const i8,
                                                                    5 as i32,
                                                                )
                                                            },
                                                            (*d).name,
                                                        );
                                                        fflush(stdout);
                                                    }
                                                    if int_file.is_null() {
                                                        let mut fresh12 = ::std::vec::from_elem(
                                                            0,
                                                            ::core::mem::size_of::<file>() as u64 as usize,
                                                        );
                                                        int_file = fresh12.as_mut_ptr() as *mut file;
                                                    }
                                                    memset(
                                                        int_file as *mut libc::c_void,
                                                        '\0' as i32,
                                                        ::core::mem::size_of::<file>() as u64,
                                                    );
                                                    (*int_file).name = (*d).name;
                                                    if pattern_search(
                                                        int_file,
                                                        0 as i32,
                                                        depth,
                                                        recursions.wrapping_add(1 as i32 as u32),
                                                        allow_compat_rules,
                                                    ) != 0
                                                    {
                                                        (*pat).pattern = (*int_file).name;
                                                        (*int_file).name = (*d).name;
                                                        (*pat).file = int_file;
                                                        int_file = 0 as *mut file;
                                                        let fresh13 = pat;
                                                        pat = pat.offset(1);
                                                        (*fresh13).name = (*d).name;
                                                        current_block_294 = 2277602629737488951;
                                                    } else {
                                                        if !((*int_file).variables).is_null() {
                                                            free_variable_set((*int_file).variables);
                                                        }
                                                        if !((*int_file).pat_variables).is_null() {
                                                            free_variable_set((*int_file).pat_variables);
                                                        }
                                                        if df.is_null() {
                                                            file_impossible((*d).name);
                                                        }
                                                        current_block_294 = 7344615536999694015;
                                                    }
                                                } else {
                                                    current_block_294 = 7344615536999694015;
                                                }
                                                match current_block_294 {
                                                    2277602629737488951 => {}
                                                    _ => {
                                                        if intermed_ok != 0 {
                                                            if 0x8 as i32 & db_level != 0 {
                                                                print_spaces(depth);
                                                                printf(
                                                                    dcgettext(
                                                                        0 as *const i8,
                                                                        b"Rejecting rule '%s' due to impossible prerequisite '%s'.\n\0"
                                                                            as *const u8 as *const i8,
                                                                        5 as i32,
                                                                    ),
                                                                    get_rule_defn(rule),
                                                                    (*d).name,
                                                                );
                                                                fflush(stdout);
                                                            }
                                                        } else if 0x8 as i32 & db_level != 0 {
                                                            print_spaces(depth);
                                                            printf(
                                                                dcgettext(
                                                                    0 as *const i8,
                                                                    b"Not found '%s'.\n\0" as *const u8 as *const i8,
                                                                    5 as i32,
                                                                ),
                                                                (*d).name,
                                                            );
                                                            fflush(stdout);
                                                        }
                                                        failed = 1 as i32 as u32;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        d = (*d).next;
                                    }
                                }
                                free_ns_chain(dl as *mut nameseq);
                                if failed != 0 {
                                    break;
                                }
                            }
                            (*rule).in_use = 0 as i32 as i8;
                            if failed == 0 {
                                break;
                            }
                        }
                    }
                }
                ri = ri.wrapping_add(1);
                ri;
            }
            if ri < nrules {
                break;
            }
            rule = 0 as *mut rule;
            intermed_ok += 1;
            intermed_ok;
        }
        if !rule.is_null() {
            foundrule = ri;
            if recursions > 0 as i32 as u32 {
                (*file).name = *((*rule).targets)
                    .offset((*tryrules.offset(foundrule as isize)).matches as isize);
            }
            loop {
                let fresh14 = pat;
                pat = pat.offset(-1);
                if !(fresh14 > deplist) {
                    break;
                }
                let mut dep_0: *mut dep = 0 as *mut dep;
                let mut s: *const i8 = 0 as *const i8;
                if !((*pat).file).is_null() {
                    let mut imf: *mut file = (*pat).file;
                    let mut f: *mut file = lookup_file((*imf).name);
                    if f.is_null() {
                        f = enter_file((*imf).name);
                    }
                    (*f).deps = (*imf).deps;
                    (*f).cmds = (*imf).cmds;
                    (*f).stem = (*imf).stem;
                    merge_variable_set_lists(&mut (*f).variables, (*imf).variables);
                    (*f).pat_variables = (*imf).pat_variables;
                    (*f).set_pat_searched((*imf).pat_searched());
                    (*f).also_make = (*imf).also_make;
                    (*f).set_is_target(1 as i32 as u32);
                    (*f)
                        .set_is_explicit(
                            (*f).is_explicit()
                                | ((*imf).is_explicit() as i32 != 0
                                    || (*pat).is_explicit() as i32 != 0) as i32 as u32,
                        );
                    (*f)
                        .set_notintermediate(
                            (*f).notintermediate()
                                | ((*imf).notintermediate() as i32 != 0
                                    || no_intermediates != 0) as i32 as u32,
                        );
                    (*f)
                        .set_intermediate(
                            (*f).intermediate()
                                | ((*f).is_explicit() == 0 && (*f).notintermediate() == 0)
                                    as i32 as u32,
                        );
                    (*f).set_tried_implicit(1 as i32 as u32);
                    imf = lookup_file((*pat).pattern);
                    if !imf.is_null() && (*imf).precious() as i32 != 0 {
                        (*f).set_precious(1 as i32 as u32);
                    }
                    dep_0 = (*f).deps;
                    while !dep_0.is_null() {
                        (*dep_0).file = enter_file((*dep_0).name);
                        (*dep_0).name = 0 as *const i8;
                        (*(*dep_0).file)
                            .set_tried_implicit(
                                (*(*dep_0).file).tried_implicit()
                                    | (*dep_0).changed() as i32 as u32,
                            );
                        dep_0 = (*dep_0).next;
                    }
                }
                dep_0 = xcalloc(::core::mem::size_of::<dep>() as u64) as *mut dep;
                (*dep_0).set_ignore_mtime((*pat).ignore_mtime());
                (*dep_0).set_is_explicit((*pat).is_explicit());
                (*dep_0).set_ignore_automatic_vars((*pat).ignore_automatic_vars());
                (*dep_0).set_wait_here((*pat).wait_here());
                s = strcache_add((*pat).name);
                if recursions != 0 {
                    (*dep_0).name = s;
                } else {
                    (*dep_0).file = lookup_file(s);
                    if ((*dep_0).file).is_null() {
                        (*dep_0).file = enter_file(s);
                    }
                }
                if ((*pat).file).is_null()
                    && (*(*tryrules.offset(foundrule as isize)).rule).terminal as i32
                        != 0
                {
                    if ((*dep_0).file).is_null() {
                        (*dep_0).set_changed(1 as i32 as u32);
                    } else {
                        (*(*dep_0).file).set_tried_implicit(1 as i32 as u32);
                    }
                }
                (*dep_0).next = (*file).deps;
                (*file).deps = dep_0;
                (*file).set_was_shuffled(0 as i32 as u32);
            }
            if (*file).was_shuffled() == 0 {
                shuffle_deps_recursive((*file).deps);
            }
            if (*tryrules.offset(foundrule as isize)).checked_lastslash == 0 {
                (*file).stem = strcache_add_len(stem, stemlen);
                fullstemlen = stemlen;
            } else {
                fullstemlen = pathlen.wrapping_add(stemlen);
                memcpy(
                    stem_str.as_mut_ptr() as *mut libc::c_void,
                    filename as *const libc::c_void,
                    pathlen,
                );
                memcpy(
                    stem_str.as_mut_ptr().offset(pathlen as isize) as *mut libc::c_void,
                    stem as *const libc::c_void,
                    stemlen,
                );
                stem_str[fullstemlen as usize] = '\0' as i32 as i8;
                (*file).stem = strcache_add(stem_str.as_mut_ptr());
            }
            (*file).cmds = (*rule).cmds;
            (*file).set_is_target(1 as i32 as u32);
            let mut f_0: *mut file = lookup_file(
                *((*rule).targets)
                    .offset((*tryrules.offset(foundrule as isize)).matches as isize),
            );
            if !f_0.is_null() {
                if (*f_0).precious() != 0 {
                    (*file).set_precious(1 as i32 as u32);
                }
                if (*f_0).notintermediate() as i32 != 0 || no_intermediates != 0 {
                    (*file).set_notintermediate(1 as i32 as u32);
                }
            }
            if (*rule).num as i32 > 1 as i32 {
                ri = 0 as i32 as u32;
                while ri < (*rule).num as u32 {
                    if ri != (*tryrules.offset(foundrule as isize)).matches {
                        let mut fresh15 = ::std::vec::from_elem(
                            0,
                            (*((*rule).lens).offset(ri as isize) as u64)
                                .wrapping_add(fullstemlen)
                                .wrapping_add(1 as i32 as u64) as usize,
                        );
                        let mut nm: *mut i8 = fresh15.as_mut_ptr() as *mut i8;
                        let mut p_1: *mut i8 = nm;
                        let mut f_1: *mut file = 0 as *mut file;
                        let mut new: *mut dep = xcalloc(
                            ::core::mem::size_of::<dep>() as u64,
                        ) as *mut dep;
                        p_1 = mempcpy(
                            p_1 as *mut libc::c_void,
                            *((*rule).targets).offset(ri as isize)
                                as *const libc::c_void,
                            ((*((*rule).suffixes).offset(ri as isize))
                                .offset_from(*((*rule).targets).offset(ri as isize)) as i64
                                - 1 as i32 as i64) as size_t,
                        ) as *mut i8;
                        p_1 = mempcpy(
                            p_1 as *mut libc::c_void,
                            (*file).stem as *const libc::c_void,
                            fullstemlen,
                        ) as *mut i8;
                        memcpy(
                            p_1 as *mut libc::c_void,
                            *((*rule).suffixes).offset(ri as isize)
                                as *const libc::c_void,
                            (*((*rule).lens).offset(ri as isize) as i64
                                - (*((*rule).suffixes).offset(ri as isize))
                                    .offset_from(*((*rule).targets).offset(ri as isize)) as i64
                                + 1 as i32 as i64) as u64,
                        );
                        (*new).name = strcache_add(nm);
                        (*new).file = enter_file((*new).name);
                        (*new).next = (*file).also_make;
                        f_1 = lookup_file(*((*rule).targets).offset(ri as isize));
                        if !f_1.is_null() {
                            if (*f_1).precious() != 0 {
                                (*(*new).file).set_precious(1 as i32 as u32);
                            }
                            if (*f_1).notintermediate() as i32 != 0
                                || no_intermediates != 0
                            {
                                (*(*new).file).set_notintermediate(1 as i32 as u32);
                            }
                        }
                        (*(*new).file).set_is_target(1 as i32 as u32);
                        (*file).also_make = new;
                    }
                    ri = ri.wrapping_add(1);
                    ri;
                }
            }
        }
    }
    free(tryrules as *mut libc::c_void);
    free(deplist as *mut libc::c_void);
    depth = depth.wrapping_sub(1);
    depth;
    if !rule.is_null() {
        if 0x8 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Found implicit rule '%s' for '%s'.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                get_rule_defn(rule),
                filename,
            );
            fflush(stdout);
        }
        return 1 as i32;
    }
    if found_compat_rule != 0 {
        if 0x8 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Searching for a compatibility rule for '%s'.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                filename,
            );
            fflush(stdout);
        }
        return pattern_search(file, archive, depth, recursions, 1 as i32);
    }
    if 0x8 as i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const i8,
                b"No implicit rule found for '%s'.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
        );
        fflush(stdout);
    }
    return 0 as i32;
}
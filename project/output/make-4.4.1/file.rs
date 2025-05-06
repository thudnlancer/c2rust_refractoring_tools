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
    fn unlink(__name: *const i8) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> i32;
    fn __errno_location() -> *mut i32;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
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
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn end_of_token(_: *const i8) -> *mut i8;
    fn find_percent(_: *mut i8) -> *mut i8;
    fn strcache_iscached(str: *const i8) -> i32;
    fn strcache_add_len(str: *const i8, len: size_t) -> *const i8;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut just_print_flag: i32;
    static mut run_silent: i32;
    static mut ignore_errors_flag: i32;
    static mut question_flag: i32;
    static mut touch_flag: i32;
    static mut no_builtin_rules_flag: i32;
    static mut not_parallel: i32;
    static mut second_expansion: i32;
    static mut cmd_prefix: i8;
    static mut no_intermediates: u32;
    fn hash_init(
        ht: *mut hash_table,
        size: u64,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_delete(ht: *mut hash_table, item: *const libc::c_void) -> *mut libc::c_void;
    fn hash_map(ht: *mut hash_table, map: hash_map_func_t);
    fn hash_map_arg(
        ht: *mut hash_table,
        map: hash_map_arg_func_t,
        arg: *mut libc::c_void,
    );
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn jhash_string(key: *const u8) -> u32;
    static mut hash_deleted_item: *mut libc::c_void;
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
    fn set_file_variables(file: *mut file, stem: *const i8);
    static mut variable_buffer: *mut i8;
    fn variable_buffer_output(
        ptr: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn variable_expand(line: *const i8) -> *mut i8;
    fn variable_expand_for_file(line: *const i8, file: *mut file) -> *mut i8;
    fn patsubst_expand_pat(
        o: *mut i8,
        text: *const i8,
        pattern: *const i8,
        replace: *const i8,
        pattern_percent: *const i8,
        replace_percent: *const i8,
    ) -> *mut i8;
    fn initialize_file_variables(file: *mut file, reading: i32);
    fn print_file_variables(file: *const file);
    fn print_target_variables(file: *const file);
    fn merge_variable_set_lists(
        to_list: *mut *mut variable_set_list,
        from_list: *mut variable_set_list,
    );
    fn lookup_variable(name: *const i8, length: size_t) -> *mut variable;
    fn lookup_variable_in_set(
        name: *const i8,
        length: size_t,
        set: *const variable_set,
    ) -> *mut variable;
    static mut export_all_variables: i32;
    static mut db_level: i32;
    fn shuffle_deps_recursive(g: *mut dep);
}
pub type size_t = u64;
pub type __intmax_t = i64;
pub type __uintmax_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __clockid_t = i32;
pub type __syscall_slong_t = i64;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
pub type intmax_t = __intmax_t;
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
pub type hash_map_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> ()>;
pub type hash_map_arg_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[no_mangle]
pub static mut snapped_deps: i32 = 0 as i32;
unsafe extern "C" fn file_hash_1(mut key: *const libc::c_void) -> u64 {
    let mut _result_: u64 = 0 as i32 as u64;
    let mut _key_: *const u8 = (*(key as *const file)).hname as *const u8;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as u64);
    return _result_;
}
unsafe extern "C" fn file_hash_2(mut key: *const libc::c_void) -> u64 {
    let mut _result_: u64 = 0 as i32 as u64;
    return _result_;
}
unsafe extern "C" fn file_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> i32 {
    return if (*(x as *const file)).hname == (*(y as *const file)).hname {
        0 as i32
    } else {
        strcmp((*(x as *const file)).hname, (*(y as *const file)).hname)
    };
}
static mut files: hash_table = hash_table {
    ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
};
static mut all_secondary: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn lookup_file(mut name: *const i8) -> *mut file {
    let mut f: *mut file = 0 as *mut file;
    let mut file_key: file = file {
        name: 0 as *const i8,
        hname: 0 as *const i8,
        vpath: 0 as *const i8,
        deps: 0 as *mut dep,
        cmds: 0 as *mut commands,
        stem: 0 as *const i8,
        also_make: 0 as *mut dep,
        prev: 0 as *mut file,
        last: 0 as *mut file,
        renamed: 0 as *mut file,
        variables: 0 as *mut variable_set_list,
        pat_variables: 0 as *mut variable_set_list,
        parent: 0 as *mut file,
        double_colon: 0 as *mut file,
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
        c2rust_padding: [0; 4],
    };
    while *name.offset(0 as i32 as isize) as i32 == '.' as i32
        && *stopchar_map
            .as_mut_ptr()
            .offset(*name.offset(1 as i32 as isize) as u8 as isize) as i32
            & 0x8000 as i32 != 0 as i32
        && *name.offset(2 as i32 as isize) as i32 != '\0' as i32
    {
        name = name.offset(2 as i32 as isize);
        while *stopchar_map.as_mut_ptr().offset(*name as u8 as isize) as i32
            & 0x8000 as i32 != 0 as i32
        {
            name = name.offset(1);
            name;
        }
    }
    if *name as i32 == '\0' as i32 {
        name = b"./\0" as *const u8 as *const i8;
    }
    file_key.hname = name;
    f = hash_find_item(&mut files, &mut file_key as *mut file as *const libc::c_void)
        as *mut file;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn enter_file(mut name: *const i8) -> *mut file {
    let mut f: *mut file = 0 as *mut file;
    let mut new: *mut file = 0 as *mut file;
    let mut file_slot: *mut *mut file = 0 as *mut *mut file;
    let mut file_key: file = file {
        name: 0 as *const i8,
        hname: 0 as *const i8,
        vpath: 0 as *const i8,
        deps: 0 as *mut dep,
        cmds: 0 as *mut commands,
        stem: 0 as *const i8,
        also_make: 0 as *mut dep,
        prev: 0 as *mut file,
        last: 0 as *mut file,
        renamed: 0 as *mut file,
        variables: 0 as *mut variable_set_list,
        pat_variables: 0 as *mut variable_set_list,
        parent: 0 as *mut file,
        double_colon: 0 as *mut file,
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
        c2rust_padding: [0; 4],
    };
    file_key.hname = name;
    file_slot = hash_find_slot(
        &mut files,
        &mut file_key as *mut file as *const libc::c_void,
    ) as *mut *mut file;
    f = *file_slot;
    if !(f.is_null() || f as *mut libc::c_void == hash_deleted_item)
        && ((*f).double_colon).is_null()
    {
        (*f).set_builtin(0 as i32 as u32);
        return f;
    }
    new = xcalloc(::core::mem::size_of::<file>() as u64) as *mut file;
    (*new).hname = name;
    (*new).name = (*new).hname;
    (*new).set_update_status(update_status::us_none);
    if f.is_null() || f as *mut libc::c_void == hash_deleted_item {
        (*new).last = new;
        hash_insert_at(
            &mut files,
            new as *const libc::c_void,
            file_slot as *const libc::c_void,
        );
    } else {
        (*new).double_colon = f;
        (*(*f).last).prev = new;
        (*f).last = new;
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn rehash_file(mut from_file: *mut file, mut to_hname: *const i8) {
    let mut file_key: file = file {
        name: 0 as *const i8,
        hname: 0 as *const i8,
        vpath: 0 as *const i8,
        deps: 0 as *mut dep,
        cmds: 0 as *mut commands,
        stem: 0 as *const i8,
        also_make: 0 as *mut dep,
        prev: 0 as *mut file,
        last: 0 as *mut file,
        renamed: 0 as *mut file,
        variables: 0 as *mut variable_set_list,
        pat_variables: 0 as *mut variable_set_list,
        parent: 0 as *mut file,
        double_colon: 0 as *mut file,
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut file_slot: *mut *mut file = 0 as *mut *mut file;
    let mut to_file: *mut file = 0 as *mut file;
    let mut deleted_file: *mut file = 0 as *mut file;
    let mut f: *mut file = 0 as *mut file;
    (*from_file).set_builtin(0 as i32 as u32);
    file_key.hname = to_hname;
    if file_hash_cmp(
        from_file as *const libc::c_void,
        &mut file_key as *mut file as *const libc::c_void,
    ) == 0
    {
        return;
    }
    file_key.hname = (*from_file).hname;
    while !((*from_file).renamed).is_null() {
        from_file = (*from_file).renamed;
    }
    if file_hash_cmp(
        from_file as *const libc::c_void,
        &mut file_key as *mut file as *const libc::c_void,
    ) != 0
    {
        abort();
    }
    deleted_file = hash_delete(&mut files, from_file as *const libc::c_void)
        as *mut file;
    if deleted_file != from_file {
        abort();
    }
    file_key.hname = to_hname;
    file_slot = hash_find_slot(
        &mut files,
        &mut file_key as *mut file as *const libc::c_void,
    ) as *mut *mut file;
    to_file = *file_slot;
    (*from_file).hname = to_hname;
    f = (*from_file).double_colon;
    while !f.is_null() {
        (*f).hname = to_hname;
        f = (*f).prev;
    }
    if to_file.is_null() || to_file as *mut libc::c_void == hash_deleted_item {
        hash_insert_at(
            &mut files,
            from_file as *const libc::c_void,
            file_slot as *const libc::c_void,
        );
        return;
    }
    if !((*from_file).cmds).is_null() {
        if ((*to_file).cmds).is_null() {
            (*to_file).cmds = (*from_file).cmds;
        } else if (*from_file).cmds != (*to_file).cmds {
            let mut l: size_t = strlen((*from_file).name);
            if !((*(*to_file).cmds).fileinfo.filenm).is_null() {
                error(
                    &mut (*(*from_file).cmds).fileinfo as *mut floc,
                    l
                        .wrapping_add(strlen((*(*to_file).cmds).fileinfo.filenm))
                        .wrapping_add(
                            (53 as i32 as u64)
                                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                                .wrapping_div(22 as i32 as u64)
                                .wrapping_add(3 as i32 as u64),
                        ),
                    dcgettext(
                        0 as *const i8,
                        b"Recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*from_file).name,
                    (*(*from_file).cmds).fileinfo.filenm,
                    (*(*from_file).cmds).fileinfo.lineno,
                );
            } else {
                error(
                    &mut (*(*from_file).cmds).fileinfo as *mut floc,
                    l,
                    dcgettext(
                        0 as *const i8,
                        b"Recipe for file '%s' was found by implicit rule search,\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*from_file).name,
                );
            }
            l = (l as u64).wrapping_add(strlen(to_hname)) as size_t as size_t;
            error(
                &mut (*(*from_file).cmds).fileinfo as *mut floc,
                l,
                dcgettext(
                    0 as *const i8,
                    b"but '%s' is now considered the same file as '%s'.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*from_file).name,
                to_hname,
            );
            error(
                &mut (*(*from_file).cmds).fileinfo as *mut floc,
                l,
                dcgettext(
                    0 as *const i8,
                    b"Recipe for '%s' will be ignored in favor of the one for '%s'.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*from_file).name,
                to_hname,
            );
        }
    }
    if ((*to_file).deps).is_null() {
        (*to_file).deps = (*from_file).deps;
    } else {
        let mut deps: *mut dep = (*to_file).deps;
        while !((*deps).next).is_null() {
            deps = (*deps).next;
        }
        (*deps).next = (*from_file).deps;
    }
    merge_variable_set_lists(&mut (*to_file).variables, (*from_file).variables);
    if !((*to_file).double_colon).is_null() && (*from_file).is_target() as i32 != 0
        && ((*from_file).double_colon).is_null()
    {
        fatal(
            0 as *mut floc,
            (strlen((*from_file).name)).wrapping_add(strlen(to_hname)),
            dcgettext(
                0 as *const i8,
                b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*from_file).name,
            to_hname,
        );
    }
    if ((*to_file).double_colon).is_null() && !((*from_file).double_colon).is_null() {
        if (*to_file).is_target() != 0 {
            fatal(
                0 as *mut floc,
                (strlen((*from_file).name)).wrapping_add(strlen(to_hname)),
                dcgettext(
                    0 as *const i8,
                    b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*from_file).name,
                to_hname,
            );
        } else {
            (*to_file).double_colon = (*from_file).double_colon;
        }
    }
    if (*from_file).last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = (*from_file).last_mtime;
    }
    (*to_file).mtime_before_update = (*from_file).mtime_before_update;
    (*to_file)
        .set_precious((*to_file).precious() | (*from_file).precious() as i32 as u32);
    (*to_file).set_loaded((*to_file).loaded() | (*from_file).loaded() as i32 as u32);
    (*to_file)
        .set_tried_implicit(
            (*to_file).tried_implicit() | (*from_file).tried_implicit() as i32 as u32,
        );
    (*to_file)
        .set_updating((*to_file).updating() | (*from_file).updating() as i32 as u32);
    (*to_file).set_updated((*to_file).updated() | (*from_file).updated() as i32 as u32);
    (*to_file)
        .set_is_target((*to_file).is_target() | (*from_file).is_target() as i32 as u32);
    (*to_file)
        .set_cmd_target(
            (*to_file).cmd_target() | (*from_file).cmd_target() as i32 as u32,
        );
    (*to_file).set_phony((*to_file).phony() | (*from_file).phony() as i32 as u32);
    (*to_file)
        .set_is_explicit(
            (*to_file).is_explicit() | (*from_file).is_explicit() as i32 as u32,
        );
    (*to_file)
        .set_secondary((*to_file).secondary() | (*from_file).secondary() as i32 as u32);
    (*to_file)
        .set_notintermediate(
            (*to_file).notintermediate() | (*from_file).notintermediate() as i32 as u32,
        );
    (*to_file)
        .set_ignore_vpath(
            (*to_file).ignore_vpath() | (*from_file).ignore_vpath() as i32 as u32,
        );
    (*to_file).set_snapped((*to_file).snapped() | (*from_file).snapped() as i32 as u32);
    (*to_file).set_builtin(0 as i32 as u32);
    (*from_file).renamed = to_file;
}
#[no_mangle]
pub unsafe extern "C" fn rename_file(mut from_file: *mut file, mut to_hname: *const i8) {
    rehash_file(from_file, to_hname);
    while !from_file.is_null() {
        (*from_file).name = (*from_file).hname;
        from_file = (*from_file).prev;
    }
}
#[no_mangle]
pub unsafe extern "C" fn remove_intermediates(mut sig: i32) {
    let mut file_slot: *mut *mut file = 0 as *mut *mut file;
    let mut file_end: *mut *mut file = 0 as *mut *mut file;
    let mut doneany: i32 = 0 as i32;
    if question_flag != 0 || touch_flag != 0 || all_secondary != 0
        || no_intermediates != 0
    {
        return;
    }
    if sig != 0 && just_print_flag != 0 {
        return;
    }
    file_slot = files.ht_vec as *mut *mut file;
    file_end = file_slot.offset(files.ht_size as isize);
    let mut current_block_32: u64;
    while file_slot < file_end {
        if !((*file_slot).is_null()
            || *file_slot as *mut libc::c_void == hash_deleted_item)
        {
            let mut f: *mut file = *file_slot;
            if (*f).intermediate() as i32 != 0
                && ((*f).dontcare() as i32 != 0 || (*f).precious() == 0)
                && (*f).secondary() == 0 && (*f).notintermediate() == 0
                && (*f).cmd_target() == 0
            {
                let mut status: i32 = 0;
                if !((*f).update_status() as i32 == update_status::us_none as i32) {
                    if just_print_flag != 0 {
                        status = 0 as i32;
                        current_block_32 = 2979737022853876585;
                    } else {
                        status = unlink((*f).name);
                        if status < 0 as i32 && *__errno_location() == 2 as i32 {
                            current_block_32 = 6873731126896040597;
                        } else {
                            current_block_32 = 2979737022853876585;
                        }
                    }
                    match current_block_32 {
                        6873731126896040597 => {}
                        _ => {
                            if (*f).dontcare() == 0 {
                                if sig != 0 {
                                    error(
                                        0 as *mut floc,
                                        strlen((*f).name),
                                        dcgettext(
                                            0 as *const i8,
                                            b"*** Deleting intermediate file '%s'\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                        (*f).name,
                                    );
                                } else {
                                    if doneany == 0 {
                                        if 0x1 as i32 & db_level != 0 {
                                            printf(
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Removing intermediate files...\n\0" as *const u8
                                                        as *const i8,
                                                    5 as i32,
                                                ),
                                            );
                                            fflush(stdout);
                                        }
                                    }
                                    if run_silent == 0 {
                                        if doneany == 0 {
                                            fputs(b"rm \0" as *const u8 as *const i8, stdout);
                                            doneany = 1 as i32;
                                        } else {
                                            putchar(' ' as i32);
                                        }
                                        fputs((*f).name, stdout);
                                        fflush(stdout);
                                    }
                                }
                                if status < 0 as i32 {
                                    perror_with_name(
                                        b"\nunlink: \0" as *const u8 as *const i8,
                                        (*f).name,
                                    );
                                    doneany = 0 as i32;
                                }
                            }
                        }
                    }
                }
            }
        }
        file_slot = file_slot.offset(1);
        file_slot;
    }
    if doneany != 0 && sig == 0 {
        putchar('\n' as i32);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn split_prereqs(mut p: *mut i8) -> *mut dep {
    let mut new: *mut dep = parse_file_seq(
        &mut p,
        ::core::mem::size_of::<dep>() as u64,
        0x100 as i32,
        0 as *const i8,
        0x40 as i32,
    ) as *mut dep;
    if *p != 0 {
        let mut ood: *mut dep = 0 as *mut dep;
        p = p.offset(1);
        p;
        ood = parse_file_seq(
            &mut p,
            ::core::mem::size_of::<dep>() as u64,
            0x1 as i32,
            0 as *const i8,
            0x40 as i32,
        ) as *mut dep;
        if new.is_null() {
            new = ood;
        } else {
            let mut dp: *mut dep = 0 as *mut dep;
            dp = new;
            while !((*dp).next).is_null() {
                dp = (*dp).next;
            }
            (*dp).next = ood;
        }
        while !ood.is_null() {
            (*ood).set_ignore_mtime(1 as i32 as u32);
            ood = (*ood).next;
        }
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn enter_prereqs(
    mut deps: *mut dep,
    mut stem: *const i8,
) -> *mut dep {
    let mut d1: *mut dep = 0 as *mut dep;
    if deps.is_null() {
        return 0 as *mut dep;
    }
    if !stem.is_null() {
        let mut pattern: *const i8 = b"%\0" as *const u8 as *const i8;
        let mut dp: *mut dep = deps;
        let mut dl: *mut dep = 0 as *mut dep;
        while !dp.is_null() {
            let mut percent: *mut i8 = 0 as *mut i8;
            let mut nl: size_t = (strlen((*dp).name)).wrapping_add(1 as i32 as u64);
            let mut fresh0 = ::std::vec::from_elem(0, nl as usize);
            let mut nm: *mut i8 = fresh0.as_mut_ptr() as *mut i8;
            memcpy(nm as *mut libc::c_void, (*dp).name as *const libc::c_void, nl);
            percent = find_percent(nm);
            if !percent.is_null() {
                let mut o: *mut i8 = 0 as *mut i8;
                if *stem.offset(0 as i32 as isize) as i32 == '\0' as i32 {
                    memmove(
                        percent as *mut libc::c_void,
                        percent.offset(1 as i32 as isize) as *const libc::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        variable_buffer,
                        nm,
                        (strlen(nm)).wrapping_add(1 as i32 as u64),
                    );
                } else {
                    o = patsubst_expand_pat(
                        variable_buffer,
                        stem,
                        pattern,
                        nm,
                        pattern.offset(1 as i32 as isize),
                        percent.offset(1 as i32 as isize),
                    );
                }
                if *variable_buffer.offset(0 as i32 as isize) as i32 == '\0' as i32 {
                    let mut df: *mut dep = dp;
                    if dp == deps {
                        deps = (*deps).next;
                        dp = deps;
                    } else {
                        (*dl).next = (*dp).next;
                        dp = (*dl).next;
                    }
                    free(df as *mut libc::c_void);
                    continue;
                } else {
                    (*dp).name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as i64 as size_t,
                    );
                }
            }
            (*dp).stem = stem;
            (*dp).set_staticpattern(1 as i32 as u32);
            dl = dp;
            dp = (*dp).next;
        }
    }
    d1 = deps;
    while !d1.is_null() {
        if !((*d1).need_2nd_expansion() != 0) {
            (*d1).file = lookup_file((*d1).name);
            if ((*d1).file).is_null() {
                (*d1).file = enter_file((*d1).name);
            }
            (*d1).set_staticpattern(0 as i32 as u32);
            (*d1).name = 0 as *const i8;
            if stem.is_null() {
                (*(*d1).file).set_is_explicit(1 as i32 as u32);
            }
        }
        d1 = (*d1).next;
    }
    return deps;
}
#[no_mangle]
pub unsafe extern "C" fn expand_deps(mut f: *mut file) {
    let mut d: *mut dep = 0 as *mut dep;
    let mut dp: *mut *mut dep = 0 as *mut *mut dep;
    let mut fstem: *const i8 = 0 as *const i8;
    let mut initialized: i32 = 0 as i32;
    let mut changed_dep: i32 = 0 as i32;
    if (*f).snapped() != 0 {
        return;
    }
    (*f).set_snapped(1 as i32 as u32);
    dp = &mut (*f).deps;
    d = (*f).deps;
    while !d.is_null() {
        let mut p: *mut i8 = 0 as *mut i8;
        let mut new: *mut dep = 0 as *mut dep;
        let mut next: *mut dep = 0 as *mut dep;
        if ((*d).name).is_null() || (*d).need_2nd_expansion() == 0 {
            dp = &mut (*d).next;
            d = (*d).next;
        } else {
            if (*d).staticpattern() != 0 {
                let mut cs: *const i8 = (*d).name;
                let mut nperc: size_t = 0 as i32 as size_t;
                loop {
                    cs = strchr(cs, '%' as i32);
                    if cs.is_null() {
                        break;
                    }
                    nperc = nperc.wrapping_add(1);
                    nperc;
                    cs = cs.offset(1);
                    cs;
                }
                if nperc != 0 {
                    let mut slen: size_t = (strlen((*d).name))
                        .wrapping_add(nperc)
                        .wrapping_add(1 as i32 as u64);
                    let mut pcs: *const i8 = (*d).name;
                    let mut name: *mut i8 = xmalloc(slen) as *mut i8;
                    let mut s: *mut i8 = name;
                    cs = strchr(pcs, '%' as i32);
                    while !cs.is_null() {
                        s = mempcpy(
                            s as *mut libc::c_void,
                            pcs as *const libc::c_void,
                            cs.offset_from(pcs) as i64 as size_t,
                        ) as *mut i8;
                        let fresh1 = s;
                        s = s.offset(1);
                        *fresh1 = '$' as i32 as i8;
                        let fresh2 = s;
                        s = s.offset(1);
                        *fresh2 = '*' as i32 as i8;
                        cs = cs.offset(1);
                        pcs = cs;
                        cs = strchr(end_of_token(cs), '%' as i32);
                    }
                    strcpy(s, pcs);
                    free((*d).name as *mut i8 as *mut libc::c_void);
                    (*d).name = name;
                }
            }
            if initialized == 0 {
                initialize_file_variables(f, 0 as i32);
                initialized = 1 as i32;
            }
            set_file_variables(
                f,
                if !((*d).stem).is_null() { (*d).stem } else { (*f).stem },
            );
            p = variable_expand_for_file((*d).name, f);
            free((*d).name as *mut i8 as *mut libc::c_void);
            new = split_prereqs(p);
            if new.is_null() {
                *dp = (*d).next;
                changed_dep = 1 as i32;
                free(d as *mut libc::c_void);
                d = *dp;
            } else {
                fstem = (*d).stem;
                next = (*d).next;
                changed_dep = 1 as i32;
                free(d as *mut libc::c_void);
                *dp = new;
                dp = &mut new;
                d = new;
                while !d.is_null() {
                    (*d).file = lookup_file((*d).name);
                    if ((*d).file).is_null() {
                        (*d).file = enter_file((*d).name);
                    }
                    (*d).name = 0 as *const i8;
                    (*d).stem = fstem;
                    if fstem.is_null() {
                        (*(*d).file).set_is_explicit(1 as i32 as u32);
                    }
                    dp = &mut (*d).next;
                    d = (*d).next;
                }
                *dp = next;
                d = *dp;
            }
        }
    }
    if changed_dep != 0 {
        shuffle_deps_recursive((*f).deps);
    }
}
#[no_mangle]
pub unsafe extern "C" fn expand_extra_prereqs(mut extra: *const variable) -> *mut dep {
    let mut d: *mut dep = 0 as *mut dep;
    let mut prereqs: *mut dep = if !extra.is_null() {
        split_prereqs(variable_expand((*extra).value))
    } else {
        0 as *mut dep
    };
    d = prereqs;
    while !d.is_null() {
        (*d).file = lookup_file((*d).name);
        if ((*d).file).is_null() {
            (*d).file = enter_file((*d).name);
        }
        (*d).name = 0 as *const i8;
        (*d).set_ignore_automatic_vars(1 as i32 as u32);
        d = (*d).next;
    }
    return prereqs;
}
unsafe extern "C" fn snap_file(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut f: *mut file = item as *mut file;
    let mut prereqs: *mut dep = 0 as *mut dep;
    if second_expansion == 0 {
        (*f).set_updating(0 as i32 as u32);
    }
    if all_secondary != 0 && (*f).notintermediate() == 0 {
        (*f).set_intermediate(1 as i32 as u32);
    }
    if no_intermediates != 0 && (*f).intermediate() == 0 && (*f).secondary() == 0 {
        (*f).set_notintermediate(1 as i32 as u32);
    }
    if !((*f).variables).is_null() {
        prereqs = expand_extra_prereqs(
            lookup_variable_in_set(
                b".EXTRA_PREREQS\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 15]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                (*(*f).variables).set,
            ),
        );
    } else if (*f).is_target() != 0 {
        prereqs = copy_dep_chain(arg as *const dep);
    }
    if !prereqs.is_null() {
        let mut d: *mut dep = 0 as *mut dep;
        d = prereqs;
        while !d.is_null() {
            if (*f).name
                == (if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name })
                || *(*f).name as i32
                    == *(if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as i32
                    && (*(*f).name as i32 == '\0' as i32
                        || strcmp(
                            ((*f).name).offset(1 as i32 as isize),
                            (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            })
                                .offset(1 as i32 as isize),
                        ) == 0)
            {
                break;
            }
            d = (*d).next;
        }
        if !d.is_null() {
            free_ns_chain(prereqs as *mut nameseq);
        } else if ((*f).deps).is_null() {
            (*f).deps = prereqs;
        } else {
            d = (*f).deps;
            while !((*d).next).is_null() {
                d = (*d).next;
            }
            (*d).next = prereqs;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn snap_deps() {
    let mut f: *mut file = 0 as *mut file;
    let mut f2: *mut file = 0 as *mut file;
    let mut d: *mut dep = 0 as *mut dep;
    snapped_deps = 1 as i32;
    f = lookup_file(b".PRECIOUS\0" as *const u8 as *const i8);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_precious(1 as i32 as u32);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".LOW_RESOLUTION_TIME\0" as *const u8 as *const i8);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_low_resolution_time(1 as i32 as u32);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".PHONY\0" as *const u8 as *const i8);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_phony(1 as i32 as u32);
                (*f2).set_is_target(1 as i32 as u32);
                (*f2).last_mtime = 1 as i32 as uintmax_t;
                (*f2).mtime_before_update = 1 as i32 as uintmax_t;
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".NOTINTERMEDIATE\0" as *const u8 as *const i8);
    while !f.is_null() {
        if !((*f).deps).is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).set_notintermediate(1 as i32 as u32);
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            no_intermediates = 1 as i32 as u32;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".INTERMEDIATE\0" as *const u8 as *const i8);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                if (*f2).notintermediate() != 0 {
                    fatal(
                        0 as *mut floc,
                        strlen((*f2).name),
                        dcgettext(
                            0 as *const i8,
                            b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*f2).name,
                    );
                } else {
                    (*f2).set_intermediate(1 as i32 as u32);
                }
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".SECONDARY\0" as *const u8 as *const i8);
    while !f.is_null() {
        if !((*f).deps).is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if (*f2).notintermediate() != 0 {
                        fatal(
                            0 as *mut floc,
                            strlen((*f2).name),
                            dcgettext(
                                0 as *const i8,
                                b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*f2).name,
                        );
                    } else {
                        (*f2).set_secondary(1 as i32 as u32);
                        (*f2).set_intermediate((*f2).secondary());
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            all_secondary = 1 as i32;
        }
        f = (*f).prev;
    }
    if no_intermediates != 0 && all_secondary != 0 {
        fatal(
            0 as *mut floc,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    f = lookup_file(b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const i8);
    if !f.is_null() && (*f).is_target() as i32 != 0 {
        export_all_variables = 1 as i32;
    }
    f = lookup_file(b".IGNORE\0" as *const u8 as *const i8);
    if !f.is_null() && (*f).is_target() as i32 != 0 {
        if ((*f).deps).is_null() {
            ignore_errors_flag = 1 as i32;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= 4 as i32;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".SILENT\0" as *const u8 as *const i8);
    if !f.is_null() && (*f).is_target() as i32 != 0 {
        if ((*f).deps).is_null() {
            run_silent = 1 as i32;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= 2 as i32;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".NOTPARALLEL\0" as *const u8 as *const i8);
    if !f.is_null() && (*f).is_target() as i32 != 0 {
        let mut d2: *mut dep = 0 as *mut dep;
        if ((*f).deps).is_null() {
            not_parallel = 1 as i32;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if !((*f2).deps).is_null() {
                        d2 = (*(*f2).deps).next;
                        while !d2.is_null() {
                            (*d2).set_wait_here(1 as i32 as u32);
                            d2 = (*d2).next;
                        }
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    let mut prereqs: *mut dep = expand_extra_prereqs(
        lookup_variable(
            b".EXTRA_PREREQS\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 15]>() as u64).wrapping_sub(1 as i32 as u64),
        ),
    );
    hash_map_arg(
        &mut files,
        Some(
            snap_file
                as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
        ),
        prereqs as *mut libc::c_void,
    );
    free_ns_chain(prereqs as *mut nameseq);
}
#[no_mangle]
pub unsafe extern "C" fn set_command_state(mut file: *mut file, mut state: cmd_state) {
    let mut d: *mut dep = 0 as *mut dep;
    (*file).set_command_state(state);
    d = (*file).also_make;
    while !d.is_null() {
        if state as u32 > (*(*d).file).command_state() as u32 {
            (*(*d).file).set_command_state(state);
        }
        d = (*d).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn file_timestamp_cons(
    mut fname: *const i8,
    mut stamp: time_t,
    mut ns: i64,
) -> uintmax_t {
    let mut offset: i32 = ((2 as i32 + 1 as i32) as i64
        + (if 1 as i32 != 0 { ns } else { 0 as i32 as i64 })) as i32;
    let mut s: uintmax_t = stamp as uintmax_t;
    let mut product: uintmax_t = s << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 });
    let mut ts: uintmax_t = product.wrapping_add(offset as u64);
    if !(s
        <= ((!(0 as i32 as uintmax_t))
            .wrapping_sub(
                (if !(-(1 as i32) as uintmax_t <= 0 as i32 as u64) {
                    0 as i32 as uintmax_t
                } else {
                    !(0 as i32 as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                }),
            )
            .wrapping_sub((2 as i32 + 1 as i32) as u64)
            >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 })
            << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
            .wrapping_add((2 as i32 + 1 as i32) as u64)
            .wrapping_add(
                (if 1 as i32 != 0 { 1000000000 as i32 } else { 1 as i32 }) as u64,
            )
            .wrapping_sub(1 as i32 as u64)
            .wrapping_sub((2 as i32 + 1 as i32) as u64)
            >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }) && product <= ts
        && ts
            <= ((!(0 as i32 as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as i32) as uintmax_t <= 0 as i32 as u64) {
                        0 as i32 as uintmax_t
                    } else {
                        !(0 as i32 as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    }),
                )
                .wrapping_sub((2 as i32 + 1 as i32) as u64)
                >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 })
                << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                .wrapping_add((2 as i32 + 1 as i32) as u64)
                .wrapping_add(
                    (if 1 as i32 != 0 { 1000000000 as i32 } else { 1 as i32 }) as u64,
                )
                .wrapping_sub(1 as i32 as u64))
    {
        let mut buf: [i8; 43] = [0; 43];
        let mut f: *const i8 = if !fname.is_null() {
            fname
        } else {
            dcgettext(
                0 as *const i8,
                b"Current time\0" as *const u8 as *const i8,
                5 as i32,
            )
        };
        ts = if s <= 2 as i32 as u64 {
            (2 as i32 + 1 as i32) as u64
        } else {
            ((!(0 as i32 as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as i32) as uintmax_t <= 0 as i32 as u64) {
                        0 as i32 as uintmax_t
                    } else {
                        !(0 as i32 as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    }),
                )
                .wrapping_sub((2 as i32 + 1 as i32) as u64)
                >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 })
                << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                .wrapping_add((2 as i32 + 1 as i32) as u64)
                .wrapping_add(
                    (if 1 as i32 != 0 { 1000000000 as i32 } else { 1 as i32 }) as u64,
                )
                .wrapping_sub(1 as i32 as u64)
        };
        file_timestamp_sprintf(buf.as_mut_ptr(), ts);
        error(
            0 as *mut floc,
            (strlen(f)).wrapping_add(strlen(buf.as_mut_ptr())),
            dcgettext(
                0 as *const i8,
                b"%s: Timestamp out of range; substituting %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            f,
            buf.as_mut_ptr(),
        );
    }
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn file_timestamp_now(mut resolution: *mut i32) -> uintmax_t {
    let mut r: i32 = 0;
    let mut s: time_t = 0;
    let mut ns: i32 = 0;
    let mut timespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(0 as i32, &mut timespec) == 0 as i32 {
        r = 1 as i32;
        s = timespec.tv_sec;
        ns = timespec.tv_nsec as i32;
    } else {
        let mut timeval: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        if gettimeofday(&mut timeval, 0 as *mut timezone) == 0 as i32 {
            r = 1000 as i32;
            s = timeval.tv_sec;
            ns = (timeval.tv_usec * 1000 as i32 as i64) as i32;
        } else {
            r = 1000000000 as i32;
            s = time(0 as *mut time_t);
            ns = 0 as i32;
        }
    }
    *resolution = r;
    return file_timestamp_cons(0 as *const i8, s, ns as i64);
}
#[no_mangle]
pub unsafe extern "C" fn file_timestamp_sprintf(mut p: *mut i8, mut ts: uintmax_t) {
    let mut t: time_t = (ts.wrapping_sub((2 as i32 + 1 as i32) as u64)
        >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 })) as time_t;
    let mut tm: *mut tm = localtime(&mut t);
    if !tm.is_null() {
        let mut year: intmax_t = (*tm).tm_year as intmax_t;
        sprintf(
            p,
            b"%04ld-%02d-%02d %02d:%02d:%02d\0" as *const u8 as *const i8,
            year + 1900 as i32 as i64,
            (*tm).tm_mon + 1 as i32,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
        );
    } else if t < 0 as i32 as i64 {
        sprintf(p, b"%ld\0" as *const u8 as *const i8, t);
    } else {
        sprintf(p, b"%lu\0" as *const u8 as *const i8, t as uintmax_t);
    }
    p = p.offset(strlen(p) as isize);
    sprintf(
        p,
        b".%09d\0" as *const u8 as *const i8,
        (ts.wrapping_sub((2 as i32 + 1 as i32) as u64)
            & (((1 as i32) << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                - 1 as i32) as u64) as i32,
    );
    p = p.offset((strlen(p)).wrapping_sub(1 as i32 as u64) as isize);
    while *p as i32 == '0' as i32 {
        p = p.offset(-1);
        p;
    }
    p = p.offset((*p as i32 != '.' as i32) as i32 as isize);
    *p = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn print_prereqs(mut deps: *const dep) {
    let mut ood: *const dep = 0 as *const dep;
    while !deps.is_null() {
        if (*deps).ignore_mtime() == 0 {
            printf(
                b" %s%s\0" as *const u8 as *const i8,
                if (*deps).wait_here() as i32 != 0 {
                    b".WAIT \0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
                if !((*deps).name).is_null() {
                    (*deps).name
                } else {
                    (*(*deps).file).name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if !ood.is_null() {
        printf(
            b" | %s%s\0" as *const u8 as *const i8,
            if (*ood).wait_here() as i32 != 0 {
                b".WAIT \0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if !((*ood).name).is_null() { (*ood).name } else { (*(*ood).file).name },
        );
        ood = (*ood).next;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                printf(
                    b" %s%s\0" as *const u8 as *const i8,
                    if (*ood).wait_here() as i32 != 0 {
                        b".WAIT \0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                    if !((*ood).name).is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    },
                );
            }
            ood = (*ood).next;
        }
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn print_file(mut item: *const libc::c_void) {
    let mut f: *const file = item as *const file;
    if no_builtin_rules_flag != 0 && (*f).builtin() as i32 != 0 {
        return;
    }
    putchar('\n' as i32);
    if !((*f).cmds).is_null() && (*(*f).cmds).recipe_prefix as i32 != cmd_prefix as i32 {
        fputs(b".RECIPEPREFIX = \0" as *const u8 as *const i8, stdout);
        cmd_prefix = (*(*f).cmds).recipe_prefix;
        if cmd_prefix as i32 != '\t' as i32 {
            putchar(cmd_prefix as i32);
        }
        putchar('\n' as i32);
    }
    if !((*f).variables).is_null() {
        print_target_variables(f);
    }
    if (*f).is_target() == 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"# Not a target:\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    printf(
        b"%s:%s\0" as *const u8 as *const i8,
        (*f).name,
        if !((*f).double_colon).is_null() {
            b":\0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    print_prereqs((*f).deps);
    if (*f).precious() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  Precious file (prerequisite of .PRECIOUS).\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    if (*f).phony() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  Phony target (prerequisite of .PHONY).\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if (*f).cmd_target() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  Command line target.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if (*f).dontcare() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  A default, MAKEFILES, or -include/sinclude makefile.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    if (*f).builtin() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  Builtin rule\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    puts(
        if (*f).tried_implicit() as i32 != 0 {
            dcgettext(
                0 as *const i8,
                b"#  Implicit rule search has been done.\0" as *const u8 as *const i8,
                5 as i32,
            )
        } else {
            dcgettext(
                0 as *const i8,
                b"#  Implicit rule search has not been done.\0" as *const u8
                    as *const i8,
                5 as i32,
            )
        },
    );
    if !((*f).stem).is_null() {
        printf(
            dcgettext(
                0 as *const i8,
                b"#  Implicit/static pattern stem: '%s'\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*f).stem,
        );
    }
    if (*f).intermediate() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  File is an intermediate prerequisite.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if (*f).notintermediate() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  File is a prerequisite of .NOTINTERMEDIATE.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    if (*f).secondary() != 0 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  File is secondary (prerequisite of .SECONDARY).\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    if !((*f).also_make).is_null() {
        let mut d: *const dep = 0 as *const dep;
        fputs(
            dcgettext(
                0 as *const i8,
                b"#  Also makes:\0" as *const u8 as *const i8,
                5 as i32,
            ),
            stdout,
        );
        d = (*f).also_make;
        while !d.is_null() {
            printf(
                b" %s\0" as *const u8 as *const i8,
                if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
            );
            d = (*d).next;
        }
        putchar('\n' as i32);
    }
    if (*f).last_mtime == 0 as i32 as u64 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  Modification time never checked.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else if (*f).last_mtime == 1 as i32 as u64 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  File does not exist.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else if (*f).last_mtime == 2 as i32 as u64 {
        puts(
            dcgettext(
                0 as *const i8,
                b"#  File is very old.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        let mut buf: [i8; 43] = [0; 43];
        file_timestamp_sprintf(buf.as_mut_ptr(), (*f).last_mtime);
        printf(
            dcgettext(
                0 as *const i8,
                b"#  Last modified %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            buf.as_mut_ptr(),
        );
    }
    puts(
        if (*f).updated() as i32 != 0 {
            dcgettext(
                0 as *const i8,
                b"#  File has been updated.\0" as *const u8 as *const i8,
                5 as i32,
            )
        } else {
            dcgettext(
                0 as *const i8,
                b"#  File has not been updated.\0" as *const u8 as *const i8,
                5 as i32,
            )
        },
    );
    match (*f).command_state() as i32 {
        2 => {
            puts(
                dcgettext(
                    0 as *const i8,
                    b"#  Recipe currently running (THIS IS A BUG).\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        1 => {
            puts(
                dcgettext(
                    0 as *const i8,
                    b"#  Dependencies recipe running (THIS IS A BUG).\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        0 | 3 => {
            match (*f).update_status() as i32 {
                0 => {
                    puts(
                        dcgettext(
                            0 as *const i8,
                            b"#  Successfully updated.\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                2 => {
                    puts(
                        dcgettext(
                            0 as *const i8,
                            b"#  Needs to be updated (-q is set).\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                3 => {
                    puts(
                        dcgettext(
                            0 as *const i8,
                            b"#  Failed to be updated.\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                1 | _ => {}
            }
        }
        _ => {
            puts(
                dcgettext(
                    0 as *const i8,
                    b"#  Invalid value in 'command_state' member!\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            fflush(stdout);
            fflush(stderr);
            abort();
        }
    }
    if !((*f).variables).is_null() {
        print_file_variables(f);
    }
    if !((*f).cmds).is_null() {
        print_commands((*f).cmds);
    }
    if !((*f).prev).is_null() {
        print_file((*f).prev as *const libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_file_data_base() {
    puts(dcgettext(0 as *const i8, b"\n# Files\0" as *const u8 as *const i8, 5 as i32));
    hash_map(
        &mut files,
        Some(print_file as unsafe extern "C" fn(*const libc::c_void) -> ()),
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\n# files hash-table stats:\n# \0" as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    hash_print_stats(&mut files, stdout);
}
unsafe extern "C" fn verify_file(mut item: *const libc::c_void) {
    let mut f: *const file = item as *const file;
    let mut d: *const dep = 0 as *const dep;
    if !((*f).name).is_null() && *((*f).name).offset(0 as i32 as isize) as i32 != 0
        && strcache_iscached((*f).name) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(strlen((*f).name)),
            dcgettext(
                0 as *const i8,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*f).name,
            b"name\0" as *const u8 as *const i8,
            (*f).name,
        );
    }
    if !((*f).hname).is_null() && *((*f).hname).offset(0 as i32 as isize) as i32 != 0
        && strcache_iscached((*f).hname) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(strlen((*f).hname)),
            dcgettext(
                0 as *const i8,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*f).name,
            b"hname\0" as *const u8 as *const i8,
            (*f).hname,
        );
    }
    if !((*f).vpath).is_null() && *((*f).vpath).offset(0 as i32 as isize) as i32 != 0
        && strcache_iscached((*f).vpath) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(strlen((*f).vpath)),
            dcgettext(
                0 as *const i8,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*f).name,
            b"vpath\0" as *const u8 as *const i8,
            (*f).vpath,
        );
    }
    if !((*f).stem).is_null() && *((*f).stem).offset(0 as i32 as isize) as i32 != 0
        && strcache_iscached((*f).stem) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(strlen((*f).stem)),
            dcgettext(
                0 as *const i8,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*f).name,
            b"stem\0" as *const u8 as *const i8,
            (*f).stem,
        );
    }
    d = (*f).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 {
            if !((*d).name).is_null()
                && *((*d).name).offset(0 as i32 as isize) as i32 != 0
                && strcache_iscached((*d).name) == 0
            {
                error(
                    0 as *const floc,
                    (strlen((*d).name))
                        .wrapping_add(
                            (::core::mem::size_of::<[i8; 5]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        )
                        .wrapping_add(strlen((*d).name)),
                    dcgettext(
                        0 as *const i8,
                        b"%s: Field '%s' not cached: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*d).name,
                    b"name\0" as *const u8 as *const i8,
                    (*d).name,
                );
            }
        }
        if !((*d).stem).is_null() && *((*d).stem).offset(0 as i32 as isize) as i32 != 0
            && strcache_iscached((*d).stem) == 0
        {
            error(
                0 as *const floc,
                (strlen((*d).name))
                    .wrapping_add(
                        (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    )
                    .wrapping_add(strlen((*d).stem)),
                dcgettext(
                    0 as *const i8,
                    b"%s: Field '%s' not cached: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*d).name,
                b"stem\0" as *const u8 as *const i8,
                (*d).stem,
            );
        }
        d = (*d).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn verify_file_data_base() {
    hash_map(
        &mut files,
        Some(verify_file as unsafe extern "C" fn(*const libc::c_void) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn build_target_list(mut value: *mut i8) -> *mut i8 {
    static mut last_targ_count: u64 = 0 as i32 as u64;
    if files.ht_fill != last_targ_count {
        let mut max: size_t = (strlen(value))
            .wrapping_div(500 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(500 as i32 as u64);
        let mut len: size_t = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut fp: *mut *mut file = files.ht_vec as *mut *mut file;
        let mut end: *mut *mut file = &mut *fp.offset(files.ht_size as isize)
            as *mut *mut file;
        value = xrealloc(value as *mut libc::c_void, max) as *mut i8;
        p = value;
        len = 0 as i32 as size_t;
        while fp < end {
            if !((*fp).is_null() || *fp as *mut libc::c_void == hash_deleted_item)
                && (**fp).is_target() as i32 != 0
            {
                let mut f: *mut file = *fp;
                let mut l: size_t = strlen((*f).name);
                len = (len as u64).wrapping_add(l.wrapping_add(1 as i32 as u64))
                    as size_t as size_t;
                if len > max {
                    let mut off: size_t = p.offset_from(value) as i64 as size_t;
                    max = (max as u64)
                        .wrapping_add(
                            l
                                .wrapping_add(1 as i32 as u64)
                                .wrapping_div(500 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                                .wrapping_mul(500 as i32 as u64),
                        ) as size_t as size_t;
                    value = xrealloc(value as *mut libc::c_void, max) as *mut i8;
                    p = &mut *value.offset(off as isize) as *mut i8;
                }
                p = mempcpy(p as *mut libc::c_void, (*f).name as *const libc::c_void, l)
                    as *mut i8;
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = ' ' as i32 as i8;
            }
            fp = fp.offset(1);
            fp;
        }
        *p.offset(-(1 as i32 as isize)) = '\0' as i32 as i8;
        last_targ_count = files.ht_fill;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn init_hash_files() {
    hash_init(
        &mut files,
        1000 as i32 as u64,
        Some(file_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(file_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            file_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
}
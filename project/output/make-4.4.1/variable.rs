use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
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
    static mut make_host: *mut i8;
    static mut remote_description: *mut i8;
    static mut version_string: *mut i8;
    fn concat(_: u32, _: ...) -> *const i8;
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    static mut makelevel: u32;
    fn reset_makeflags(origin: variable_origin);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn xstrndup(_: *const i8, _: size_t) -> *mut i8;
    fn next_token(_: *const i8) -> *mut i8;
    static mut reading_file: *const floc;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut env_overrides: i32;
    static mut warn_undefined_variables_flag: i32;
    static mut default_shell: *const i8;
    static mut cmd_prefix: i8;
    static mut jobserver_auth: *mut i8;
    static mut hash_deleted_item: *mut libc::c_void;
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn hash_map_arg(
        ht: *mut hash_table,
        map: hash_map_arg_func_t,
        arg: *mut libc::c_void,
    );
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
    fn hash_delete_at(
        ht: *mut hash_table,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(ht: *mut hash_table, free_items: i32);
    fn hash_map(ht: *mut hash_table, map: hash_map_func_t);
    fn jhash(key: *const u8, n: i32) -> u32;
    static mut variable_buffer: *mut i8;
    static mut shell_var: variable;
    fn variable_buffer_output(
        ptr: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn allocated_variable_expand_for_file(line: *const i8, file: *mut file) -> *mut i8;
    fn install_variable_buffer(bufp: *mut *mut i8, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut i8, len: size_t);
    fn func_shell_base(o: *mut i8, argv: *mut *mut i8, trim_newlines: i32) -> *mut i8;
    fn recursively_expand_for_file(v: *mut variable, file: *mut file) -> *mut i8;
    fn jobserver_get_invalid_auth() -> *const i8;
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
pub struct pattern_var {
    pub next: *mut pattern_var,
    pub suffix: *const i8,
    pub target: *const i8,
    pub len: size_t,
    pub variable: variable,
}
pub type hash_map_arg_func_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
>;
pub type hash_map_func_t = Option<unsafe extern "C" fn(*const libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defined_vars {
    pub name: *const i8,
    pub len: size_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[no_mangle]
pub static mut env_recursion: libc::c_ulonglong = 0 as i32 as libc::c_ulonglong;
static mut variable_changenum: u64 = 0 as i32 as u64;
static mut pattern_vars: *mut pattern_var = 0 as *const pattern_var as *mut pattern_var;
static mut last_pattern_vars: [*mut pattern_var; 256] = [0 as *const pattern_var
    as *mut pattern_var; 256];
#[no_mangle]
pub unsafe extern "C" fn create_pattern_var(
    mut target: *const i8,
    mut suffix: *const i8,
) -> *mut pattern_var {
    let mut len: size_t = strlen(target);
    let mut p: *mut pattern_var = xcalloc(::core::mem::size_of::<pattern_var>() as u64)
        as *mut pattern_var;
    if !pattern_vars.is_null() {
        if len < 256 as i32 as u64 && !(last_pattern_vars[len as usize]).is_null() {
            (*p).next = (*last_pattern_vars[len as usize]).next;
            (*last_pattern_vars[len as usize]).next = p;
        } else {
            let mut v: *mut *mut pattern_var = 0 as *mut *mut pattern_var;
            v = &mut pattern_vars;
            loop {
                if (*v).is_null() || (**v).len > len {
                    (*p).next = *v;
                    *v = p;
                    break;
                } else {
                    v = &mut (**v).next;
                }
            }
        }
    } else {
        pattern_vars = p;
        (*p).next = 0 as *mut pattern_var;
    }
    (*p).target = target;
    (*p).len = len;
    (*p).suffix = suffix.offset(1 as i32 as isize);
    if len < 256 as i32 as u64 {
        last_pattern_vars[len as usize] = p;
    }
    return p;
}
unsafe extern "C" fn lookup_pattern_var(
    mut start: *mut pattern_var,
    mut target: *const i8,
    mut targlen: size_t,
) -> *mut pattern_var {
    let mut p: *mut pattern_var = 0 as *mut pattern_var;
    p = if !start.is_null() { (*start).next } else { pattern_vars };
    while !p.is_null() {
        let mut stem: *const i8 = 0 as *const i8;
        let mut stemlen: size_t = 0;
        if !((*p).len > targlen) {
            stem = target
                .offset(
                    (((*p).suffix).offset_from((*p).target) as i64 - 1 as i32 as i64)
                        as isize,
                );
            stemlen = targlen.wrapping_sub((*p).len).wrapping_add(1 as i32 as u64);
            if !(stem > target
                && !(strncmp((*p).target, target, stem.offset_from(target) as i64 as u64)
                    == 0 as i32))
            {
                if *(*p).suffix as i32 == *stem.offset(stemlen as isize) as i32
                    && (*(*p).suffix as i32 == '\0' as i32
                        || (&*((*p).suffix).offset(1 as i32 as isize) as *const i8
                            == &*stem
                                .offset(stemlen.wrapping_add(1 as i32 as u64) as isize)
                                as *const i8
                            || *((*p).suffix).offset(1 as i32 as isize) as i32
                                == *stem
                                    .offset(stemlen.wrapping_add(1 as i32 as u64) as isize)
                                    as i32
                                && (*((*p).suffix).offset(1 as i32 as isize) as i32
                                    == '\0' as i32
                                    || strcmp(
                                        (&*((*p).suffix).offset(1 as i32 as isize) as *const i8)
                                            .offset(1 as i32 as isize),
                                        (&*stem
                                            .offset(stemlen.wrapping_add(1 as i32 as u64) as isize)
                                            as *const i8)
                                            .offset(1 as i32 as isize),
                                    ) == 0)))
                {
                    break;
                }
            }
        }
        p = (*p).next;
    }
    return p;
}
unsafe extern "C" fn variable_hash_1(mut keyv: *const libc::c_void) -> u64 {
    let mut key: *const variable = keyv as *const variable;
    let mut _result_: u64 = 0 as i32 as u64;
    let mut _key_: *const u8 = (*key).name as *const u8;
    _result_ = _result_.wrapping_add(jhash(_key_, (*key).length as i32) as u64);
    return _result_;
}
unsafe extern "C" fn variable_hash_2(mut keyv: *const libc::c_void) -> u64 {
    let mut key: *const variable = keyv as *const variable;
    let mut _result_: u64 = 0 as i32 as u64;
    return _result_;
}
unsafe extern "C" fn variable_hash_cmp(
    mut xv: *const libc::c_void,
    mut yv: *const libc::c_void,
) -> i32 {
    let mut x: *const variable = xv as *const variable;
    let mut y: *const variable = yv as *const variable;
    let mut result: i32 = ((*x).length).wrapping_sub((*y).length) as i32;
    if result != 0 {
        return result;
    }
    return if (*x).name == (*y).name {
        0 as i32
    } else {
        memcmp(
            (*x).name as *const libc::c_void,
            (*y).name as *const libc::c_void,
            (*x).length as u64,
        )
    };
}
static mut global_variable_set: variable_set = variable_set {
    table: hash_table {
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
    },
};
static mut global_setlist: variable_set_list = unsafe {
    {
        let mut init = variable_set_list {
            next: 0 as *const variable_set_list as *mut variable_set_list,
            set: &global_variable_set as *const variable_set as *mut variable_set,
            next_is_parent: 0 as i32,
        };
        init
    }
};
#[no_mangle]
pub static mut current_variable_set_list: *mut variable_set_list = unsafe {
    &global_setlist as *const variable_set_list as *mut variable_set_list
};
#[no_mangle]
pub unsafe extern "C" fn init_hash_global_variable_set() {
    hash_init(
        &mut global_variable_set.table,
        523 as i32 as u64,
        Some(variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn define_variable_in_set(
    mut name: *const i8,
    mut length: size_t,
    mut value: *const i8,
    mut origin: variable_origin,
    mut recursive: i32,
    mut set: *mut variable_set,
    mut flocp: *const floc,
) -> *mut variable {
    let mut v: *mut variable = 0 as *mut variable;
    let mut var_slot: *mut *mut variable = 0 as *mut *mut variable;
    let mut var_key: variable = variable {
        name: 0 as *const i8 as *mut i8,
        value: 0 as *const i8 as *mut i8,
        fileinfo: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    if set.is_null() {
        set = &mut global_variable_set;
    }
    var_key.name = name as *mut i8;
    var_key.length = length as u32;
    var_slot = hash_find_slot(
        &mut (*set).table,
        &mut var_key as *mut variable as *const libc::c_void,
    ) as *mut *mut variable;
    v = *var_slot;
    if env_overrides != 0 && origin as u32 == variable_origin::o_env as i32 as u32 {
        origin = variable_origin::o_env_override;
    }
    if !(v.is_null() || v as *mut libc::c_void == hash_deleted_item) {
        if env_overrides != 0 && (*v).origin() as i32 == variable_origin::o_env as i32 {
            (*v).set_origin(variable_origin::o_env_override);
        }
        if origin as i32 >= (*v).origin() as i32 {
            free((*v).value as *mut libc::c_void);
            (*v).value = xstrdup(value);
            if !flocp.is_null() {
                (*v).fileinfo = *flocp;
            } else {
                (*v).fileinfo.filenm = 0 as *const i8;
            }
            (*v).set_origin(origin);
            (*v).set_recursive(recursive as u32);
        }
        return v;
    }
    v = xcalloc(::core::mem::size_of::<variable>() as u64) as *mut variable;
    (*v).name = xstrndup(name, length);
    (*v).length = length as u32;
    hash_insert_at(
        &mut (*set).table,
        v as *const libc::c_void,
        var_slot as *const libc::c_void,
    );
    if set == &mut global_variable_set as *mut variable_set {
        variable_changenum = variable_changenum.wrapping_add(1);
        variable_changenum;
    }
    (*v).value = xstrdup(value);
    if !flocp.is_null() {
        (*v).fileinfo = *flocp;
    }
    (*v).set_origin(origin);
    (*v).set_recursive(recursive as u32);
    (*v).set_export(variable_export::v_default);
    (*v).set_exportable(1 as i32 as u32);
    name = (*v).name;
    if *name as i32 != '_' as i32
        && ((*name as i32) < 'A' as i32 || *name as i32 > 'Z' as i32)
        && ((*name as i32) < 'a' as i32 || *name as i32 > 'z' as i32)
    {
        (*v).set_exportable(0 as i32 as u32);
    } else {
        name = name.offset(1);
        name;
        while *name as i32 != '\0' as i32 {
            if *name as i32 != '_' as i32
                && ((*name as i32) < 'a' as i32 || *name as i32 > 'z' as i32)
                && ((*name as i32) < 'A' as i32 || *name as i32 > 'Z' as i32)
                && !((*name as u32).wrapping_sub('0' as i32 as u32) <= 9 as i32 as u32)
            {
                break;
            }
            name = name.offset(1);
            name;
        }
        if *name as i32 != '\0' as i32 {
            (*v).set_exportable(0 as i32 as u32);
        }
    }
    return v;
}
unsafe extern "C" fn free_variable_name_and_value(mut item: *const libc::c_void) {
    let mut v: *mut variable = item as *mut variable;
    free((*v).name as *mut libc::c_void);
    free((*v).value as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_variable_set(mut list: *mut variable_set_list) {
    hash_map(
        &mut (*(*list).set).table,
        Some(
            free_variable_name_and_value
                as unsafe extern "C" fn(*const libc::c_void) -> (),
        ),
    );
    hash_free(&mut (*(*list).set).table, 1 as i32);
    free((*list).set as *mut libc::c_void);
    free(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn undefine_variable_in_set(
    mut name: *const i8,
    mut length: size_t,
    mut origin: variable_origin,
    mut set: *mut variable_set,
) {
    let mut v: *mut variable = 0 as *mut variable;
    let mut var_slot: *mut *mut variable = 0 as *mut *mut variable;
    let mut var_key: variable = variable {
        name: 0 as *const i8 as *mut i8,
        value: 0 as *const i8 as *mut i8,
        fileinfo: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    if set.is_null() {
        set = &mut global_variable_set;
    }
    var_key.name = name as *mut i8;
    var_key.length = length as u32;
    var_slot = hash_find_slot(
        &mut (*set).table,
        &mut var_key as *mut variable as *const libc::c_void,
    ) as *mut *mut variable;
    if env_overrides != 0 && origin as u32 == variable_origin::o_env as i32 as u32 {
        origin = variable_origin::o_env_override;
    }
    v = *var_slot;
    if !(v.is_null() || v as *mut libc::c_void == hash_deleted_item) {
        if env_overrides != 0 && (*v).origin() as i32 == variable_origin::o_env as i32 {
            (*v).set_origin(variable_origin::o_env_override);
        }
        if origin as i32 >= (*v).origin() as i32 {
            hash_delete_at(&mut (*set).table, var_slot as *const libc::c_void);
            free_variable_name_and_value(v as *const libc::c_void);
            free(v as *mut libc::c_void);
            if set == &mut global_variable_set as *mut variable_set {
                variable_changenum = variable_changenum.wrapping_add(1);
                variable_changenum;
            }
        }
    }
}
unsafe extern "C" fn lookup_special_var(mut var: *mut variable) -> *mut variable {
    static mut last_changenum: u64 = 0 as i32 as u64;
    if variable_changenum != last_changenum
        && ((*var).name == b".VARIABLES\0" as *const u8 as *const i8 as *mut i8
            || *(*var).name as i32 == *(b".VARIABLES\0" as *const u8 as *const i8) as i32
                && (*(*var).name as i32 == '\0' as i32
                    || strcmp(
                        ((*var).name).offset(1 as i32 as isize),
                        (b".VARIABLES\0" as *const u8 as *const i8)
                            .offset(1 as i32 as isize),
                    ) == 0))
    {
        let mut max: size_t = (strlen((*var).value))
            .wrapping_div(500 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(500 as i32 as u64);
        let mut len: size_t = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut vp: *mut *mut variable = global_variable_set.table.ht_vec
            as *mut *mut variable;
        let mut end: *mut *mut variable = &mut *vp
            .offset(global_variable_set.table.ht_size as isize) as *mut *mut variable;
        (*var).value = xrealloc((*var).value as *mut libc::c_void, max) as *mut i8;
        p = (*var).value;
        len = 0 as i32 as size_t;
        while vp < end {
            if !((*vp).is_null() || *vp as *mut libc::c_void == hash_deleted_item) {
                let mut v: *mut variable = *vp;
                let mut l: i32 = (*v).length as i32;
                len = (len as u64).wrapping_add((l + 1 as i32) as u64) as size_t
                    as size_t;
                if len > max {
                    let mut off: size_t = p.offset_from((*var).value) as i64 as size_t;
                    max = (max as u64)
                        .wrapping_add(
                            (((l + 1 as i32) / 500 as i32 + 1 as i32) * 500 as i32)
                                as u64,
                        ) as size_t as size_t;
                    (*var).value = xrealloc((*var).value as *mut libc::c_void, max)
                        as *mut i8;
                    p = &mut *((*var).value).offset(off as isize) as *mut i8;
                }
                p = mempcpy(
                    p as *mut libc::c_void,
                    (*v).name as *const libc::c_void,
                    l as size_t,
                ) as *mut i8;
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = ' ' as i32 as i8;
            }
            vp = vp.offset(1);
            vp;
        }
        *p.offset(-(1 as i32 as isize)) = '\0' as i32 as i8;
        last_changenum = variable_changenum;
    }
    return var;
}
#[no_mangle]
pub unsafe extern "C" fn lookup_variable(
    mut name: *const i8,
    mut length: size_t,
) -> *mut variable {
    let mut setlist: *const variable_set_list = 0 as *const variable_set_list;
    let mut var_key: variable = variable {
        name: 0 as *const i8 as *mut i8,
        value: 0 as *const i8 as *mut i8,
        fileinfo: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut is_parent: i32 = 0 as i32;
    var_key.name = name as *mut i8;
    var_key.length = length as u32;
    setlist = current_variable_set_list;
    while !setlist.is_null() {
        let mut set: *const variable_set = (*setlist).set;
        let mut v: *mut variable = 0 as *mut variable;
        v = hash_find_item(
            &(*set).table as *const hash_table as *mut hash_table,
            &mut var_key as *mut variable as *const libc::c_void,
        ) as *mut variable;
        if !v.is_null() && (is_parent == 0 || (*v).private_var() == 0) {
            return if (*v).special() as i32 != 0 { lookup_special_var(v) } else { v };
        }
        is_parent |= (*setlist).next_is_parent;
        setlist = (*setlist).next;
    }
    return 0 as *mut variable;
}
#[no_mangle]
pub unsafe extern "C" fn lookup_variable_for_file(
    mut name: *const i8,
    mut length: size_t,
    mut file: *mut file,
) -> *mut variable {
    let mut var: *mut variable = 0 as *mut variable;
    let mut savev: *mut variable_set_list = 0 as *mut variable_set_list;
    if file.is_null() {
        return lookup_variable(name, length);
    }
    savev = current_variable_set_list;
    current_variable_set_list = (*file).variables;
    var = lookup_variable(name, length);
    current_variable_set_list = savev;
    return var;
}
#[no_mangle]
pub unsafe extern "C" fn lookup_variable_in_set(
    mut name: *const i8,
    mut length: size_t,
    mut set: *const variable_set,
) -> *mut variable {
    let mut var_key: variable = variable {
        name: 0 as *const i8 as *mut i8,
        value: 0 as *const i8 as *mut i8,
        fileinfo: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    var_key.name = name as *mut i8;
    var_key.length = length as u32;
    return hash_find_item(
        &(*set).table as *const hash_table as *mut hash_table,
        &mut var_key as *mut variable as *const libc::c_void,
    ) as *mut variable;
}
#[no_mangle]
pub unsafe extern "C" fn initialize_file_variables(
    mut file: *mut file,
    mut reading: i32,
) {
    let mut l: *mut variable_set_list = (*file).variables;
    if l.is_null() {
        l = xmalloc(::core::mem::size_of::<variable_set_list>() as u64)
            as *mut variable_set_list;
        (*l).set = xmalloc(::core::mem::size_of::<variable_set>() as u64)
            as *mut variable_set;
        hash_init(
            &mut (*(*l).set).table,
            23 as i32 as u64,
            Some(variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
            Some(variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
            Some(
                variable_hash_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
        (*file).variables = l;
    }
    if !((*file).double_colon).is_null() && (*file).double_colon != file {
        initialize_file_variables((*file).double_colon, reading);
        (*l).next = (*(*file).double_colon).variables;
        (*l).next_is_parent = 0 as i32;
        return;
    }
    if ((*file).parent).is_null() {
        (*l).next = &mut global_setlist;
    } else {
        initialize_file_variables((*file).parent, reading);
        (*l).next = (*(*file).parent).variables;
    }
    (*l).next_is_parent = 1 as i32;
    if reading == 0 && (*file).pat_searched() == 0 {
        let mut p: *mut pattern_var = 0 as *mut pattern_var;
        let targlen: size_t = strlen((*file).name);
        p = lookup_pattern_var(0 as *mut pattern_var, (*file).name, targlen);
        if !p.is_null() {
            let mut global: *mut variable_set_list = current_variable_set_list;
            (*file).pat_variables = create_new_variable_set();
            current_variable_set_list = (*file).pat_variables;
            loop {
                let mut v: *mut variable = 0 as *mut variable;
                if ((*p).variable).flavor() as i32 == variable_flavor::f_simple as i32 {
                    v = define_variable_in_set(
                        (*p).variable.name,
                        strlen((*p).variable.name),
                        (*p).variable.value,
                        ((*p).variable).origin(),
                        0 as i32,
                        (*current_variable_set_list).set,
                        &mut (*p).variable.fileinfo,
                    );
                    (*v).set_flavor(variable_flavor::f_simple);
                } else {
                    v = do_variable_definition(
                        &mut (*p).variable.fileinfo,
                        (*p).variable.name,
                        (*p).variable.value,
                        ((*p).variable).origin(),
                        ((*p).variable).flavor(),
                        1 as i32,
                    );
                }
                (*v).set_per_target(((*p).variable).per_target());
                (*v).set_export(((*p).variable).export());
                (*v).set_private_var(((*p).variable).private_var());
                p = lookup_pattern_var(p, (*file).name, targlen);
                if p.is_null() {
                    break;
                }
            }
            current_variable_set_list = global;
        }
        (*file).set_pat_searched(1 as i32 as u32);
    }
    if !((*file).pat_variables).is_null() {
        (*(*file).pat_variables).next = (*l).next;
        (*(*file).pat_variables).next_is_parent = (*l).next_is_parent;
        (*l).next = (*file).pat_variables;
        (*l).next_is_parent = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_new_variable_set() -> *mut variable_set_list {
    let mut setlist: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut set: *mut variable_set = 0 as *mut variable_set;
    set = xmalloc(::core::mem::size_of::<variable_set>() as u64) as *mut variable_set;
    hash_init(
        &mut (*set).table,
        13 as i32 as u64,
        Some(variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    setlist = xmalloc(::core::mem::size_of::<variable_set_list>() as u64)
        as *mut variable_set_list;
    (*setlist).set = set;
    (*setlist).next = current_variable_set_list;
    (*setlist).next_is_parent = 0 as i32;
    return setlist;
}
#[no_mangle]
pub unsafe extern "C" fn push_new_variable_scope() -> *mut variable_set_list {
    current_variable_set_list = create_new_variable_set();
    if (*current_variable_set_list).next == &mut global_setlist as *mut variable_set_list
    {
        let mut set: *mut variable_set = (*current_variable_set_list).set;
        (*current_variable_set_list).set = global_setlist.set;
        global_setlist.set = set;
        (*current_variable_set_list).next = global_setlist.next;
        global_setlist.next = current_variable_set_list;
        current_variable_set_list = &mut global_setlist;
    }
    return current_variable_set_list;
}
#[no_mangle]
pub unsafe extern "C" fn pop_variable_scope() {
    let mut setlist: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut set: *mut variable_set = 0 as *mut variable_set;
    if current_variable_set_list != &mut global_setlist as *mut variable_set_list {
        setlist = current_variable_set_list;
        set = (*setlist).set;
        current_variable_set_list = (*setlist).next;
    } else {
        setlist = global_setlist.next;
        set = global_setlist.set;
        global_setlist.set = (*setlist).set;
        global_setlist.next = (*setlist).next;
        global_setlist.next_is_parent = (*setlist).next_is_parent;
    }
    free(setlist as *mut libc::c_void);
    hash_map(
        &mut (*set).table,
        Some(
            free_variable_name_and_value
                as unsafe extern "C" fn(*const libc::c_void) -> (),
        ),
    );
    hash_free(&mut (*set).table, 1 as i32);
    free(set as *mut libc::c_void);
}
unsafe extern "C" fn merge_variable_sets(
    mut to_set: *mut variable_set,
    mut from_set: *mut variable_set,
) {
    let mut from_var_slot: *mut *mut variable = (*from_set).table.ht_vec
        as *mut *mut variable;
    let mut from_var_end: *mut *mut variable = from_var_slot
        .offset((*from_set).table.ht_size as isize);
    let mut inc: i32 = if to_set == &mut global_variable_set as *mut variable_set {
        1 as i32
    } else {
        0 as i32
    };
    while from_var_slot < from_var_end {
        if !((*from_var_slot).is_null()
            || *from_var_slot as *mut libc::c_void == hash_deleted_item)
        {
            let mut from_var: *mut variable = *from_var_slot;
            let mut to_var_slot: *mut *mut variable = hash_find_slot(
                &mut (*to_set).table,
                *from_var_slot as *const libc::c_void,
            ) as *mut *mut variable;
            if (*to_var_slot).is_null()
                || *to_var_slot as *mut libc::c_void == hash_deleted_item
            {
                hash_insert_at(
                    &mut (*to_set).table,
                    from_var as *const libc::c_void,
                    to_var_slot as *const libc::c_void,
                );
                variable_changenum = variable_changenum.wrapping_add(inc as u64);
            } else {
                free((*from_var).value as *mut libc::c_void);
                free(from_var as *mut libc::c_void);
            }
        }
        from_var_slot = from_var_slot.offset(1);
        from_var_slot;
    }
}
#[no_mangle]
pub unsafe extern "C" fn merge_variable_set_lists(
    mut setlist0: *mut *mut variable_set_list,
    mut setlist1: *mut variable_set_list,
) {
    let mut to: *mut variable_set_list = *setlist0;
    let mut last0: *mut variable_set_list = 0 as *mut variable_set_list;
    if setlist1.is_null() || setlist1 == &mut global_setlist as *mut variable_set_list {
        return;
    }
    if !to.is_null() {
        while to != &mut global_setlist as *mut variable_set_list {
            if to == setlist1 {
                return;
            }
            to = (*to).next;
        }
        to = *setlist0;
        while setlist1 != &mut global_setlist as *mut variable_set_list
            && to != &mut global_setlist as *mut variable_set_list
        {
            let mut from: *mut variable_set_list = setlist1;
            setlist1 = (*setlist1).next;
            merge_variable_sets((*to).set, (*from).set);
            last0 = to;
            to = (*to).next;
        }
    }
    if setlist1 != &mut global_setlist as *mut variable_set_list {
        if last0.is_null() {
            *setlist0 = setlist1;
        } else {
            (*last0).next = setlist1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_automatic_variables() {
    let mut v: *mut variable = 0 as *mut variable;
    let mut buf: [i8; 200] = [0; 200];
    sprintf(buf.as_mut_ptr(), b"%u\0" as *const u8 as *const i8, makelevel);
    define_variable_in_set(
        b"MAKELEVEL\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64),
        buf.as_mut_ptr(),
        variable_origin::o_env,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    sprintf(
        buf.as_mut_ptr(),
        b"%s%s%s\0" as *const u8 as *const i8,
        version_string,
        if remote_description.is_null()
            || *remote_description.offset(0 as i32 as isize) as i32 == '\0' as i32
        {
            b"\0" as *const u8 as *const i8
        } else {
            b"-\0" as *const u8 as *const i8
        },
        if remote_description.is_null()
            || *remote_description.offset(0 as i32 as isize) as i32 == '\0' as i32
        {
            b"\0" as *const u8 as *const i8
        } else {
            remote_description
        },
    );
    define_variable_in_set(
        b"MAKE_VERSION\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 13]>() as u64).wrapping_sub(1 as i32 as u64),
        buf.as_mut_ptr(),
        variable_origin::o_default,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"MAKE_HOST\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64),
        make_host,
        variable_origin::o_default,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    v = define_variable_in_set(
        b"SHELL\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
        default_shell,
        variable_origin::o_default,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if *(*v).value as i32 == '\0' as i32
        || (*v).origin() as i32 == variable_origin::o_env as i32
        || (*v).origin() as i32 == variable_origin::o_env_override as i32
    {
        free((*v).value as *mut libc::c_void);
        (*v).set_origin(variable_origin::o_file);
        (*v).value = xstrdup(default_shell);
    }
    v = define_variable_in_set(
        b"MAKEFILES\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64),
        b"\0" as *const u8 as *const i8,
        variable_origin::o_default,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    (*v).set_export(variable_export::v_ifset);
    define_variable_in_set(
        b"@D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $@))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"%D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $%))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"*D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $*))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"<D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $<))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"?D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $?))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"^D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $^))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"+D\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(patsubst %/,%,$(dir $+))\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"@F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $@)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"%F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $%)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"*F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $*)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"<F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $<)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"?F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $?)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"^F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $^)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"+F\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
        b"$(notdir $+)\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        1 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
}
#[no_mangle]
pub static mut export_all_variables: i32 = 0;
unsafe extern "C" fn should_export(mut v: *const variable) -> i32 {
    match (*v).export() as i32 {
        2 => return 0 as i32,
        3 => {
            if (*v).origin() as i32 == variable_origin::o_default as i32 {
                return 0 as i32;
            }
        }
        0 => {
            if (*v).origin() as i32 == variable_origin::o_default as i32
                || (*v).origin() as i32 == variable_origin::o_automatic as i32
            {
                return 0 as i32;
            }
            if (*v).exportable() == 0 {
                return 0 as i32;
            }
            if export_all_variables == 0
                && (*v).origin() as i32 != variable_origin::o_command as i32
                && (*v).origin() as i32 != variable_origin::o_env as i32
                && (*v).origin() as i32 != variable_origin::o_env_override as i32
            {
                return 0 as i32;
            }
        }
        1 | _ => {}
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn target_environment(
    mut file: *mut file,
    mut recursive: i32,
) -> *mut *mut i8 {
    let mut set_list: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut s: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut table: hash_table = hash_table {
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
    let mut v_slot: *mut *mut variable = 0 as *mut *mut variable;
    let mut v_end: *mut *mut variable = 0 as *mut *mut variable;
    let mut result_0: *mut *mut i8 = 0 as *mut *mut i8;
    let mut result: *mut *mut i8 = 0 as *mut *mut i8;
    let mut invalid: *const i8 = 0 as *const i8;
    let mut added_SHELL: i32 = (shell_var.value == 0 as *mut i8) as i32;
    let mut found_makelevel: i32 = 0 as i32;
    let mut found_mflags: i32 = 0 as i32;
    let mut found_makeflags: i32 = 0 as i32;
    if file.is_null() {
        env_recursion = env_recursion.wrapping_add(1);
        env_recursion;
    }
    if recursive == 0 && !jobserver_auth.is_null() {
        invalid = jobserver_get_invalid_auth();
    }
    if !file.is_null() {
        set_list = (*file).variables;
    } else {
        set_list = current_variable_set_list;
    }
    hash_init(
        &mut table,
        523 as i32 as u64,
        Some(variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    s = set_list;
    while !s.is_null() {
        let mut set: *mut variable_set = (*s).set;
        let islocal: i32 = (s == set_list) as i32;
        let isglobal: i32 = (set == &mut global_variable_set as *mut variable_set)
            as i32;
        v_slot = (*set).table.ht_vec as *mut *mut variable;
        v_end = v_slot.offset((*set).table.ht_size as isize);
        while v_slot < v_end {
            if !((*v_slot).is_null()
                || *v_slot as *mut libc::c_void == hash_deleted_item)
            {
                let mut evslot: *mut *mut variable = 0 as *mut *mut variable;
                let mut v: *mut variable = *v_slot;
                if !(islocal == 0 && (*v).private_var() as i32 != 0) {
                    evslot = hash_find_slot(&mut table, v as *const libc::c_void)
                        as *mut *mut variable;
                    if (*evslot).is_null()
                        || *evslot as *mut libc::c_void == hash_deleted_item
                    {
                        if isglobal == 0 || should_export(v) != 0 {
                            hash_insert_at(
                                &mut table,
                                v as *const libc::c_void,
                                evslot as *const libc::c_void,
                            );
                        }
                    } else if (**evslot).export() as i32
                        == variable_export::v_default as i32
                    {
                        (**evslot).set_export((*v).export());
                    }
                }
            }
            v_slot = v_slot.offset(1);
            v_slot;
        }
        s = (*s).next;
    }
    result_0 = xmalloc(
        (table.ht_fill)
            .wrapping_add(3 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    result = result_0;
    v_slot = table.ht_vec as *mut *mut variable;
    v_end = v_slot.offset(table.ht_size as isize);
    while v_slot < v_end {
        if !((*v_slot).is_null() || *v_slot as *mut libc::c_void == hash_deleted_item) {
            let mut v_0: *mut variable = *v_slot;
            let mut value: *mut i8 = (*v_0).value;
            let mut cp: *mut i8 = 0 as *mut i8;
            if !(should_export(v_0) == 0) {
                if (*v_0).recursive() as i32 != 0
                    && ((*v_0).origin() as i32 != variable_origin::o_env as i32
                        && (*v_0).origin() as i32
                            != variable_origin::o_env_override as i32
                        || ((*v_0).name
                            == b"MAKEFLAGS\0" as *const u8 as *const i8 as *mut i8
                            || *(*v_0).name as i32
                                == *(b"MAKEFLAGS\0" as *const u8 as *const i8) as i32
                                && (*(*v_0).name as i32 == '\0' as i32
                                    || strcmp(
                                        ((*v_0).name).offset(1 as i32 as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const i8)
                                            .offset(1 as i32 as isize),
                                    ) == 0)))
                {
                    cp = recursively_expand_for_file(v_0, file);
                    value = cp;
                }
                if added_SHELL == 0
                    && ((*v_0).name == b"SHELL\0" as *const u8 as *const i8 as *mut i8
                        || *(*v_0).name as i32
                            == *(b"SHELL\0" as *const u8 as *const i8) as i32
                            && (*(*v_0).name as i32 == '\0' as i32
                                || strcmp(
                                    ((*v_0).name).offset(1 as i32 as isize),
                                    (b"SHELL\0" as *const u8 as *const i8)
                                        .offset(1 as i32 as isize),
                                ) == 0))
                {
                    added_SHELL = 1 as i32;
                } else if found_makelevel == 0
                    && ((*v_0).name
                        == b"MAKELEVEL\0" as *const u8 as *const i8 as *mut i8
                        || *(*v_0).name as i32
                            == *(b"MAKELEVEL\0" as *const u8 as *const i8) as i32
                            && (*(*v_0).name as i32 == '\0' as i32
                                || strcmp(
                                    ((*v_0).name).offset(1 as i32 as isize),
                                    (b"MAKELEVEL\0" as *const u8 as *const i8)
                                        .offset(1 as i32 as isize),
                                ) == 0))
                {
                    let mut val: [i8; 23] = [0; 23];
                    sprintf(
                        val.as_mut_ptr(),
                        b"%u\0" as *const u8 as *const i8,
                        makelevel.wrapping_add(1 as i32 as u32),
                    );
                    free(cp as *mut libc::c_void);
                    cp = xstrdup(val.as_mut_ptr());
                    value = cp;
                    found_makelevel = 1 as i32;
                } else if !invalid.is_null() {
                    if found_makeflags == 0
                        && ((*v_0).name
                            == b"MAKEFLAGS\0" as *const u8 as *const i8 as *mut i8
                            || *(*v_0).name as i32
                                == *(b"MAKEFLAGS\0" as *const u8 as *const i8) as i32
                                && (*(*v_0).name as i32 == '\0' as i32
                                    || strcmp(
                                        ((*v_0).name).offset(1 as i32 as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const i8)
                                            .offset(1 as i32 as isize),
                                    ) == 0))
                    {
                        let mut mf: *mut i8 = 0 as *mut i8;
                        let mut vars: *mut i8 = 0 as *mut i8;
                        found_makeflags = 1 as i32;
                        if !(strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const i8,
                        ))
                            .is_null()
                        {
                            vars = strstr(value, b" -- \0" as *const u8 as *const i8);
                            if vars.is_null() {
                                mf = xstrdup(concat(2 as i32 as u32, value, invalid));
                            } else {
                                let mut lf: size_t = vars.offset_from(value) as i64
                                    as size_t;
                                let mut li: size_t = strlen(invalid);
                                mf = xmalloc(
                                    (strlen(value))
                                        .wrapping_add(li)
                                        .wrapping_add(1 as i32 as u64),
                                ) as *mut i8;
                                strcpy(
                                    mempcpy(
                                        mempcpy(
                                            mf as *mut libc::c_void,
                                            value as *const libc::c_void,
                                            lf,
                                        ),
                                        invalid as *const libc::c_void,
                                        li,
                                    ) as *mut i8,
                                    vars,
                                );
                            }
                            free(cp as *mut libc::c_void);
                            cp = mf;
                            value = cp;
                            if found_mflags != 0 {
                                invalid = 0 as *const i8;
                            }
                        }
                    } else if found_mflags == 0
                        && ((*v_0).name
                            == b"MFLAGS\0" as *const u8 as *const i8 as *mut i8
                            || *(*v_0).name as i32
                                == *(b"MFLAGS\0" as *const u8 as *const i8) as i32
                                && (*(*v_0).name as i32 == '\0' as i32
                                    || strcmp(
                                        ((*v_0).name).offset(1 as i32 as isize),
                                        (b"MFLAGS\0" as *const u8 as *const i8)
                                            .offset(1 as i32 as isize),
                                    ) == 0))
                    {
                        let mut mf_0: *const i8 = 0 as *const i8;
                        found_mflags = 1 as i32;
                        if !(strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const i8,
                        ))
                            .is_null()
                        {
                            if !((*v_0).origin() as i32 != variable_origin::o_env as i32)
                            {
                                mf_0 = concat(2 as i32 as u32, value, invalid);
                                free(cp as *mut libc::c_void);
                                cp = xstrdup(mf_0);
                                value = cp;
                                if found_makeflags != 0 {
                                    invalid = 0 as *const i8;
                                }
                            }
                        }
                    }
                }
                let fresh1 = result;
                result = result.offset(1);
                *fresh1 = xstrdup(
                    concat(
                        3 as i32 as u32,
                        (*v_0).name,
                        b"=\0" as *const u8 as *const i8,
                        value,
                    ),
                );
                free(cp as *mut libc::c_void);
            }
        }
        v_slot = v_slot.offset(1);
        v_slot;
    }
    if added_SHELL == 0 {
        let fresh2 = result;
        result = result.offset(1);
        *fresh2 = xstrdup(
            concat(
                3 as i32 as u32,
                shell_var.name,
                b"=\0" as *const u8 as *const i8,
                shell_var.value,
            ),
        );
    }
    if found_makelevel == 0 {
        let mut val_0: [i8; 33] = [0; 33];
        sprintf(
            val_0.as_mut_ptr(),
            b"%s=%u\0" as *const u8 as *const i8,
            b"MAKELEVEL\0" as *const u8 as *const i8,
            makelevel.wrapping_add(1 as i32 as u32),
        );
        let fresh3 = result;
        result = result.offset(1);
        *fresh3 = xstrdup(val_0.as_mut_ptr());
    }
    *result = 0 as *mut i8;
    hash_free(&mut table, 0 as i32);
    if file.is_null() {
        env_recursion = env_recursion.wrapping_sub(1);
        env_recursion;
    }
    return result_0;
}
unsafe extern "C" fn set_special_var(
    mut var: *mut variable,
    mut origin: variable_origin,
) -> *mut variable {
    if (*var).name == b"MAKEFLAGS\0" as *const u8 as *const i8 as *mut i8
        || *(*var).name as i32 == *(b"MAKEFLAGS\0" as *const u8 as *const i8) as i32
            && (*(*var).name as i32 == '\0' as i32
                || strcmp(
                    ((*var).name).offset(1 as i32 as isize),
                    (b"MAKEFLAGS\0" as *const u8 as *const i8).offset(1 as i32 as isize),
                ) == 0)
    {
        reset_makeflags(origin);
    } else if (*var).name == b".RECIPEPREFIX\0" as *const u8 as *const i8 as *mut i8
        || *(*var).name as i32 == *(b".RECIPEPREFIX\0" as *const u8 as *const i8) as i32
            && (*(*var).name as i32 == '\0' as i32
                || strcmp(
                    ((*var).name).offset(1 as i32 as isize),
                    (b".RECIPEPREFIX\0" as *const u8 as *const i8)
                        .offset(1 as i32 as isize),
                ) == 0)
    {
        cmd_prefix = (if *((*var).value).offset(0 as i32 as isize) as i32 == '\0' as i32
        {
            '\t' as i32
        } else {
            *((*var).value).offset(0 as i32 as isize) as i32
        }) as i8;
    }
    return var;
}
unsafe extern "C" fn shell_result(mut p: *const i8) -> *mut i8 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut args: [*mut i8; 2] = [0 as *mut i8; 2];
    let mut result: *mut i8 = 0 as *mut i8;
    install_variable_buffer(&mut buf, &mut len);
    args[0 as i32 as usize] = p as *mut i8;
    args[1 as i32 as usize] = 0 as *mut i8;
    variable_buffer_output(
        func_shell_base(variable_buffer, args.as_mut_ptr(), 0 as i32),
        b"\0\0" as *const u8 as *const i8,
        1 as i32 as size_t,
    );
    result = strdup(variable_buffer);
    restore_variable_buffer(buf, len);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn do_variable_definition(
    mut flocp: *const floc,
    mut varname: *const i8,
    mut value: *const i8,
    mut origin: variable_origin,
    mut flavor: variable_flavor,
    mut target_var: i32,
) -> *mut variable {
    let mut current_block: u64;
    let mut newval: *const i8 = 0 as *const i8;
    let mut alloc_value: *mut i8 = 0 as *mut i8;
    let mut v: *mut variable = 0 as *mut variable;
    let mut append: i32 = 0 as i32;
    let mut conditional: i32 = 0 as i32;
    match flavor as u32 {
        1 => {
            alloc_value = allocated_variable_expand_for_file(value, 0 as *mut file);
            newval = alloc_value;
            current_block = 1423531122933789233;
        }
        3 => {
            let mut t: *mut i8 = allocated_variable_expand_for_file(
                value,
                0 as *mut file,
            );
            alloc_value = xmalloc(
                (strlen(t)).wrapping_mul(2 as i32 as u64).wrapping_add(1 as i32 as u64),
            ) as *mut i8;
            let mut np: *mut i8 = alloc_value;
            let mut op: *mut i8 = t;
            while *op.offset(0 as i32 as isize) as i32 != '\0' as i32 {
                if *op.offset(0 as i32 as isize) as i32 == '$' as i32 {
                    let fresh4 = np;
                    np = np.offset(1);
                    *fresh4 = '$' as i32 as i8;
                }
                let fresh5 = op;
                op = op.offset(1);
                let fresh6 = np;
                np = np.offset(1);
                *fresh6 = *fresh5;
            }
            *np = '\0' as i32 as i8;
            free(t as *mut libc::c_void);
            newval = alloc_value;
            current_block = 1423531122933789233;
        }
        6 => {
            let mut q: *mut i8 = allocated_variable_expand_for_file(
                value,
                0 as *mut file,
            );
            alloc_value = shell_result(q);
            free(q as *mut libc::c_void);
            flavor = variable_flavor::f_recursive;
            newval = alloc_value;
            current_block = 1423531122933789233;
        }
        5 => {
            v = lookup_variable(varname, strlen(varname));
            if !v.is_null() {
                current_block = 17468364432813189961;
            } else {
                conditional = 1 as i32;
                flavor = variable_flavor::f_recursive;
                current_block = 6076343616746591237;
            }
        }
        2 => {
            current_block = 6076343616746591237;
        }
        4 | 7 => {
            if target_var != 0 {
                append = 1 as i32;
                v = lookup_variable_in_set(
                    varname,
                    strlen(varname),
                    (*current_variable_set_list).set,
                );
                if !v.is_null() && (*v).append() == 0 {
                    append = 0 as i32;
                }
            } else {
                v = lookup_variable(varname, strlen(varname));
            }
            if v.is_null() {
                newval = value;
                flavor = variable_flavor::f_recursive;
                current_block = 1423531122933789233;
            } else {
                let mut oldlen: size_t = 0;
                let mut vallen: size_t = 0;
                let mut alloclen: size_t = 0;
                let mut val: *const i8 = 0 as *const i8;
                let mut cp: *mut i8 = 0 as *mut i8;
                let mut tp: *mut i8 = 0 as *mut i8;
                val = value;
                if (*v).recursive() != 0 {
                    flavor = variable_flavor::f_recursive;
                } else if flavor as u32 != variable_flavor::f_append_value as i32 as u32
                {
                    tp = allocated_variable_expand_for_file(val, 0 as *mut file);
                    val = tp;
                }
                vallen = strlen(val);
                if vallen == 0 {
                    alloc_value = tp;
                    current_block = 17468364432813189961;
                } else {
                    oldlen = strlen((*v).value);
                    alloclen = oldlen
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(vallen)
                        .wrapping_add(1 as i32 as u64);
                    alloc_value = xmalloc(alloclen) as *mut i8;
                    cp = alloc_value;
                    if oldlen != 0 {
                        let mut s: *mut i8 = 0 as *mut i8;
                        if (varname == b"MAKEFLAGS\0" as *const u8 as *const i8
                            || *varname as i32
                                == *(b"MAKEFLAGS\0" as *const u8 as *const i8) as i32
                                && (*varname as i32 == '\0' as i32
                                    || strcmp(
                                        varname.offset(1 as i32 as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const i8)
                                            .offset(1 as i32 as isize),
                                    ) == 0))
                            && {
                                s = strstr((*v).value, b" -- \0" as *const u8 as *const i8);
                                !s.is_null()
                            }
                        {
                            cp = mempcpy(
                                cp as *mut libc::c_void,
                                (*v).value as *const libc::c_void,
                                s.offset_from((*v).value) as i64 as size_t,
                            ) as *mut i8;
                        } else {
                            cp = mempcpy(
                                cp as *mut libc::c_void,
                                (*v).value as *const libc::c_void,
                                oldlen,
                            ) as *mut i8;
                        }
                        let fresh7 = cp;
                        cp = cp.offset(1);
                        *fresh7 = ' ' as i32 as i8;
                    }
                    memcpy(
                        cp as *mut libc::c_void,
                        val as *const libc::c_void,
                        vallen.wrapping_add(1 as i32 as u64),
                    );
                    free(tp as *mut libc::c_void);
                    newval = alloc_value;
                    current_block = 1423531122933789233;
                }
            }
        }
        0 | _ => {
            abort();
        }
    }
    match current_block {
        6076343616746591237 => {
            newval = value;
            current_block = 1423531122933789233;
        }
        _ => {}
    }
    match current_block {
        1423531122933789233 => {
            v = define_variable_in_set(
                varname,
                strlen(varname),
                newval,
                origin,
                (flavor as u32 == variable_flavor::f_recursive as i32 as u32
                    || flavor as u32 == variable_flavor::f_expand as i32 as u32) as i32,
                if target_var != 0 {
                    (*current_variable_set_list).set
                } else {
                    0 as *mut variable_set
                },
                flocp,
            );
            (*v).set_append(append as u32);
            (*v).set_conditional(conditional as u32);
        }
        _ => {}
    }
    free(alloc_value as *mut libc::c_void);
    return if (*v).special() as i32 != 0 { set_special_var(v, origin) } else { v };
}
#[no_mangle]
pub unsafe extern "C" fn parse_variable_definition(
    mut str: *const i8,
    mut var: *mut variable,
) -> *mut i8 {
    let mut p: *const i8 = str;
    let mut end: *const i8 = 0 as *const i8;
    while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        p = p.offset(1);
        p;
    }
    (*var).name = p as *mut i8;
    (*var).length = 0 as i32 as u32;
    let mut current_block_38: u64;
    loop {
        let fresh8 = p;
        p = p.offset(1);
        let mut c: i32 = *fresh8 as i32;
        if *stopchar_map.as_mut_ptr().offset(c as u8 as isize) as i32
            & (0x8 as i32 | 0x1 as i32) != 0 as i32
        {
            return 0 as *mut i8;
        }
        if *stopchar_map.as_mut_ptr().offset(c as u8 as isize) as i32 & 0x2 as i32
            != 0 as i32
        {
            if !end.is_null() {
                return 0 as *mut i8;
            }
            end = p.offset(-(1 as i32 as isize));
            while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
                & (0x2 as i32 | 0x4 as i32) != 0 as i32
            {
                p = p.offset(1);
                p;
            }
        } else if c == '=' as i32 {
            if end.is_null() {
                end = p.offset(-(1 as i32 as isize));
            }
            (*var).set_flavor(variable_flavor::f_recursive);
            break;
        } else if c == ':' as i32 {
            if end.is_null() {
                end = p.offset(-(1 as i32 as isize));
            }
            let fresh9 = p;
            p = p.offset(1);
            c = *fresh9 as i32;
            if c == '=' as i32 {
                (*var).set_flavor(variable_flavor::f_simple);
                break;
            } else {
                if c == ':' as i32 {
                    let fresh10 = p;
                    p = p.offset(1);
                    c = *fresh10 as i32;
                    if c == '=' as i32 {
                        (*var).set_flavor(variable_flavor::f_simple);
                        break;
                    } else if c == ':' as i32
                        && {
                            let fresh11 = p;
                            p = p.offset(1);
                            *fresh11 as i32 == '=' as i32
                        }
                    {
                        (*var).set_flavor(variable_flavor::f_expand);
                        break;
                    }
                }
                return 0 as *mut i8;
            }
        } else {
            if *p as i32 == '=' as i32 {
                match c {
                    43 => {
                        current_block_38 = 16894542219116672231;
                        match current_block_38 {
                            12473531611435473340 => {
                                (*var).set_flavor(variable_flavor::f_shell);
                            }
                            8504108418281322610 => {
                                (*var).set_flavor(variable_flavor::f_conditional);
                            }
                            _ => {
                                (*var).set_flavor(variable_flavor::f_append);
                            }
                        }
                        if end.is_null() {
                            end = p.offset(-(1 as i32 as isize));
                        }
                        p = p.offset(1);
                        p;
                        break;
                    }
                    63 => {
                        current_block_38 = 8504108418281322610;
                        match current_block_38 {
                            12473531611435473340 => {
                                (*var).set_flavor(variable_flavor::f_shell);
                            }
                            8504108418281322610 => {
                                (*var).set_flavor(variable_flavor::f_conditional);
                            }
                            _ => {
                                (*var).set_flavor(variable_flavor::f_append);
                            }
                        }
                        if end.is_null() {
                            end = p.offset(-(1 as i32 as isize));
                        }
                        p = p.offset(1);
                        p;
                        break;
                    }
                    33 => {
                        current_block_38 = 12473531611435473340;
                        match current_block_38 {
                            12473531611435473340 => {
                                (*var).set_flavor(variable_flavor::f_shell);
                            }
                            8504108418281322610 => {
                                (*var).set_flavor(variable_flavor::f_conditional);
                            }
                            _ => {
                                (*var).set_flavor(variable_flavor::f_append);
                            }
                        }
                        if end.is_null() {
                            end = p.offset(-(1 as i32 as isize));
                        }
                        p = p.offset(1);
                        p;
                        break;
                    }
                    _ => {}
                }
            }
            if !end.is_null() {
                return 0 as *mut i8;
            }
            if !(c == '$' as i32) {
                continue;
            }
            let mut closeparen: i8 = 0;
            let mut count: u32 = 0;
            let fresh12 = p;
            p = p.offset(1);
            c = *fresh12 as i32;
            match c {
                40 => {
                    closeparen = ')' as i32 as i8;
                }
                123 => {
                    closeparen = '}' as i32 as i8;
                }
                0 => return 0 as *mut i8,
                _ => {
                    continue;
                }
            }
            count = 1 as i32 as u32;
            while *p as i32 != '\0' as i32 {
                if *p as i32 == closeparen as i32
                    && {
                        count = count.wrapping_sub(1);
                        count == 0 as i32 as u32
                    }
                {
                    p = p.offset(1);
                    p;
                    break;
                } else {
                    if *p as i32 == c {
                        count = count.wrapping_add(1);
                        count;
                    }
                    p = p.offset(1);
                    p;
                }
            }
        }
    }
    (*var).length = end.offset_from((*var).name) as i64 as u32;
    (*var).value = next_token(p);
    return p as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn assign_variable_definition(
    mut v: *mut variable,
    mut line: *const i8,
) -> *mut variable {
    let mut name: *mut i8 = 0 as *mut i8;
    if (parse_variable_definition(line, v)).is_null() {
        return 0 as *mut variable;
    }
    let mut fresh13 = ::std::vec::from_elem(
        0,
        ((*v).length).wrapping_add(1 as i32 as u32) as u64 as usize,
    );
    name = fresh13.as_mut_ptr() as *mut i8;
    memcpy(
        name as *mut libc::c_void,
        (*v).name as *const libc::c_void,
        (*v).length as u64,
    );
    *name.offset((*v).length as isize) = '\0' as i32 as i8;
    (*v).name = allocated_variable_expand_for_file(name, 0 as *mut file);
    if *((*v).name).offset(0 as i32 as isize) as i32 == '\0' as i32 {
        fatal(
            &mut (*v).fileinfo as *mut floc,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"empty variable name\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn try_variable_definition(
    mut flocp: *const floc,
    mut line: *const i8,
    mut origin: variable_origin,
    mut target_var: i32,
) -> *mut variable {
    let mut v: variable = variable {
        name: 0 as *const i8 as *mut i8,
        value: 0 as *const i8 as *mut i8,
        fileinfo: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut vp: *mut variable = 0 as *mut variable;
    if !flocp.is_null() {
        v.fileinfo = *flocp;
    } else {
        v.fileinfo.filenm = 0 as *const i8;
    }
    if (assign_variable_definition(&mut v, line)).is_null() {
        return 0 as *mut variable;
    }
    vp = do_variable_definition(flocp, v.name, v.value, origin, v.flavor(), target_var);
    free(v.name as *mut libc::c_void);
    return vp;
}
static mut defined_vars: [defined_vars; 11] = [defined_vars {
    name: 0 as *const i8,
    len: 0,
}; 11];
#[no_mangle]
pub unsafe extern "C" fn warn_undefined(mut name: *const i8, mut len: size_t) {
    if warn_undefined_variables_flag != 0 {
        let mut dp: *const defined_vars = 0 as *const defined_vars;
        dp = defined_vars.as_ptr();
        while !((*dp).name).is_null() {
            if (*dp).len == len
                && memcmp(
                    (*dp).name as *const libc::c_void,
                    name as *const libc::c_void,
                    len,
                ) == 0 as i32
            {
                return;
            }
            dp = dp.offset(1);
            dp;
        }
        error(
            reading_file,
            len,
            dcgettext(
                0 as *const i8,
                b"warning: undefined variable '%.*s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            len as i32,
            name,
        );
    }
}
unsafe extern "C" fn print_variable(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut v: *const variable = item as *const variable;
    let mut prefix: *const i8 = arg as *const i8;
    let mut origin: *const i8 = 0 as *const i8;
    match (*v).origin() as i32 {
        6 => {
            origin = dcgettext(
                0 as *const i8,
                b"automatic\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        0 => {
            origin = dcgettext(
                0 as *const i8,
                b"default\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        1 => {
            origin = dcgettext(
                0 as *const i8,
                b"environment\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        2 => {
            origin = dcgettext(
                0 as *const i8,
                b"makefile\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        3 => {
            origin = dcgettext(
                0 as *const i8,
                b"environment under -e\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        4 => {
            origin = dcgettext(
                0 as *const i8,
                b"command line\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        5 => {
            origin = dcgettext(
                0 as *const i8,
                b"'override' directive\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
        7 => {
            abort();
        }
        _ => {}
    }
    fputs(b"# \0" as *const u8 as *const i8, stdout);
    fputs(origin, stdout);
    if (*v).private_var() != 0 {
        fputs(b" private\0" as *const u8 as *const i8, stdout);
    }
    if !((*v).fileinfo.filenm).is_null() {
        printf(
            dcgettext(
                0 as *const i8,
                b" (from '%s', line %lu)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*v).fileinfo.filenm,
            ((*v).fileinfo.lineno).wrapping_add((*v).fileinfo.offset),
        );
    }
    putchar('\n' as i32);
    fputs(prefix, stdout);
    if (*v).recursive() as i32 != 0 && !(strchr((*v).value, '\n' as i32)).is_null() {
        printf(
            b"define %s\n%s\nendef\n\0" as *const u8 as *const i8,
            (*v).name,
            (*v).value,
        );
    } else {
        let mut p: *mut i8 = 0 as *mut i8;
        printf(
            b"%s %s= \0" as *const u8 as *const i8,
            (*v).name,
            if (*v).recursive() as i32 != 0 {
                if (*v).append() as i32 != 0 {
                    b"+\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                }
            } else {
                b":\0" as *const u8 as *const i8
            },
        );
        p = next_token((*v).value);
        if p != (*v).value && *p as i32 == '\0' as i32 {
            printf(b"$(subst ,,%s)\0" as *const u8 as *const i8, (*v).value);
        } else if (*v).recursive() != 0 {
            fputs((*v).value, stdout);
        } else {
            p = (*v).value;
            while *p as i32 != '\0' as i32 {
                if *p as i32 == '$' as i32 {
                    putchar('$' as i32);
                }
                putchar(*p as i32);
                p = p.offset(1);
                p;
            }
        }
        putchar('\n' as i32);
    };
}
unsafe extern "C" fn print_auto_variable(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut v: *const variable = item as *const variable;
    if (*v).origin() as i32 == variable_origin::o_automatic as i32 {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_noauto_variable(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut v: *const variable = item as *const variable;
    if (*v).origin() as i32 != variable_origin::o_automatic as i32 {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_variable_set(
    mut set: *mut variable_set,
    mut prefix: *const i8,
    mut pauto: i32,
) {
    hash_map_arg(
        &mut (*set).table,
        if pauto != 0 {
            Some(
                print_auto_variable
                    as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
            )
        } else {
            Some(
                print_variable
                    as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
            )
        },
        prefix as *mut libc::c_void,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"# variable set hash-table stats:\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    fputs(b"# \0" as *const u8 as *const i8, stdout);
    hash_print_stats(&mut (*set).table, stdout);
    _IO_putc('\n' as i32, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn print_variable_data_base() {
    puts(
        dcgettext(
            0 as *const i8,
            b"\n# Variables\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    print_variable_set(
        &mut global_variable_set,
        b"\0" as *const u8 as *const i8,
        0 as i32,
    );
    puts(
        dcgettext(
            0 as *const i8,
            b"\n# Pattern-specific Variable Values\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    let mut p: *mut pattern_var = 0 as *mut pattern_var;
    let mut rules: u32 = 0 as i32 as u32;
    p = pattern_vars;
    while !p.is_null() {
        rules = rules.wrapping_add(1);
        rules;
        printf(b"\n%s :\n\0" as *const u8 as *const i8, (*p).target);
        print_variable(
            &mut (*p).variable as *mut variable as *const libc::c_void,
            b"# \0" as *const u8 as *const i8 as *mut libc::c_void,
        );
        p = (*p).next;
    }
    if rules == 0 as i32 as u32 {
        puts(
            dcgettext(
                0 as *const i8,
                b"\n# No pattern-specific variable values.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const i8,
                b"\n# %u pattern-specific variable values\0" as *const u8 as *const i8,
                5 as i32,
            ),
            rules,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_file_variables(mut file: *const file) {
    if !((*file).variables).is_null() {
        print_variable_set(
            (*(*file).variables).set,
            b"# \0" as *const u8 as *const i8,
            1 as i32,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_target_variables(mut file: *const file) {
    if !((*file).variables).is_null() {
        let mut l: size_t = strlen((*file).name);
        let mut fresh14 = ::std::vec::from_elem(
            0,
            l.wrapping_add(3 as i32 as u64) as usize,
        );
        let mut t: *mut i8 = fresh14.as_mut_ptr() as *mut i8;
        memcpy(t as *mut libc::c_void, (*file).name as *const libc::c_void, l);
        *t.offset(l as isize) = ':' as i32 as i8;
        *t.offset(l.wrapping_add(1 as i32 as u64) as isize) = ' ' as i32 as i8;
        *t.offset(l.wrapping_add(2 as i32 as u64) as isize) = '\0' as i32 as i8;
        hash_map_arg(
            &mut (*(*(*file).variables).set).table,
            Some(
                print_noauto_variable
                    as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
            ),
            t as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn run_static_initializers() {
    defined_vars = [
        {
            let mut init = defined_vars {
                name: b"MAKECMDGOALS\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 13]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKE_RESTARTS\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 14]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKE_TERMOUT\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 13]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKE_TERMERR\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 13]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKEOVERRIDES\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 14]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b".DEFAULT\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"-*-command-variables-*-\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 24]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"-*-eval-flags-*-\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 17]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"VPATH\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"GPATH\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: 0 as *const i8,
                len: 0 as i32 as size_t,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
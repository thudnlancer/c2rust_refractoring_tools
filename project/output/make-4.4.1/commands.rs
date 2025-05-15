use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn unlink(__name: *const i8) -> i32;
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
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
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn temp_stdin_unlink();
    fn pfatal_with_name(_: *const i8) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn make_pid() -> pid_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn xstrndup(_: *const i8, _: size_t) -> *mut i8;
    fn ar_name(_: *const i8) -> i32;
    fn ar_member_date(_: *const i8) -> time_t;
    fn strcache_add(str: *const i8) -> *const i8;
    fn strcache_add_len(str: *const i8, len: size_t) -> *const i8;
    fn unload_file(name: *const i8) -> i32;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut always_make_flag: i32;
    static mut one_shell: i32;
    static mut cmd_prefix: i8;
    fn remote_kill(id: pid_t, sig: i32) -> i32;
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
    fn set_command_state(file: *mut file, state: cmd_state);
    fn notice_finished_file(file: *mut file);
    fn file_timestamp_cons(_: *const i8, _: time_t, _: i64) -> uintmax_t;
    fn hash_free(ht: *mut hash_table, free_items: i32);
    static mut default_file: *mut file;
    fn enter_file(name: *const i8) -> *mut file;
    static mut hash_deleted_item: *mut libc::c_void;
    fn remove_intermediates(sig: i32);
    fn jhash_string(key: *const u8) -> u32;
    fn jobserver_clear();
    fn osync_clear();
    fn initialize_file_variables(file: *mut file, reading: i32);
    fn define_variable_in_set(
        name: *const i8,
        length: size_t,
        value: *const i8,
        origin: variable_origin,
        recursive: i32,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    static mut children: *mut child;
    fn new_job(file: *mut file);
    fn reap_children(block: i32, err: i32);
    static mut job_slots_used: u32;
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type __sig_atomic_t = i32;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: i32,
    pub err: i32,
    #[bitfield(name = "syncout", ty = "libc::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct child {
    pub cmd_name: *mut i8,
    pub environment: *mut *mut i8,
    pub output: output,
    pub next: *mut child,
    pub file: *mut file,
    pub sh_batch_file: *mut i8,
    pub command_lines: *mut *mut i8,
    pub command_ptr: *mut i8,
    pub command_line: u32,
    pub pid: pid_t,
    #[bitfield(name = "remote", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "noerror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "good_stdin", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "deleted", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "jobslot", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "6..=6")]
    pub remote_noerror_good_stdin_deleted_recursive_jobslot_dontcare: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn dep_hash_1(mut key: *const libc::c_void) -> u64 {
    let mut d: *const dep = key as *const dep;
    let mut _result_: u64 = 0 as i32 as u64;
    let mut _key_: *const u8 = (if !((*d).name).is_null() {
        (*d).name
    } else {
        (*(*d).file).name
    }) as *const u8;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as u64);
    return _result_;
}
unsafe extern "C" fn dep_hash_2(mut key: *const libc::c_void) -> u64 {
    let mut d: *const dep = key as *const dep;
    let mut _result_: u64 = 0 as i32 as u64;
    if !((*d).name).is_null() {} else {};
    return _result_;
}
unsafe extern "C" fn dep_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> i32 {
    let mut dx: *const dep = x as *const dep;
    let mut dy: *const dep = y as *const dep;
    return strcmp(
        if !((*dx).name).is_null() { (*dx).name } else { (*(*dx).file).name },
        if !((*dy).name).is_null() { (*dy).name } else { (*(*dy).file).name },
    );
}
#[no_mangle]
pub unsafe extern "C" fn set_file_variables(mut file: *mut file, mut stem: *const i8) {
    let mut d: *mut dep = 0 as *mut dep;
    let mut at: *const i8 = 0 as *const i8;
    let mut percent: *const i8 = 0 as *const i8;
    let mut star: *const i8 = 0 as *const i8;
    let mut less: *const i8 = 0 as *const i8;
    if ar_name((*file).name) != 0 {
        let mut len: size_t = 0;
        let mut cp: *const i8 = 0 as *const i8;
        let mut p: *mut i8 = 0 as *mut i8;
        cp = strchr((*file).name, '(' as i32);
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (cp.offset_from((*file).name) as i64 + 1 as i32 as i64) as u64 as usize,
        );
        p = fresh0.as_mut_ptr() as *mut i8;
        memcpy(
            p as *mut libc::c_void,
            (*file).name as *const libc::c_void,
            cp.offset_from((*file).name) as i64 as u64,
        );
        *p.offset(cp.offset_from((*file).name) as i64 as isize) = '\0' as i32 as i8;
        at = p;
        len = strlen(cp.offset(1 as i32 as isize));
        let mut fresh1 = ::std::vec::from_elem(0, len as usize);
        p = fresh1.as_mut_ptr() as *mut i8;
        memcpy(
            p as *mut libc::c_void,
            cp.offset(1 as i32 as isize) as *const libc::c_void,
            len.wrapping_sub(1 as i32 as u64),
        );
        *p.offset(len.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32 as i8;
        percent = p;
    } else {
        at = (*file).name;
        percent = b"\0" as *const u8 as *const i8;
    }
    if stem.is_null() {
        let mut name: *const i8 = 0 as *const i8;
        let mut len_0: size_t = 0;
        if ar_name((*file).name) != 0 {
            name = (strchr((*file).name, '(' as i32)).offset(1 as i32 as isize);
            len_0 = (strlen(name)).wrapping_sub(1 as i32 as u64);
        } else {
            name = (*file).name;
            len_0 = strlen(name);
        }
        d = (*enter_file(strcache_add(b".SUFFIXES\0" as *const u8 as *const i8))).deps;
        while !d.is_null() {
            let mut dn: *const i8 = if !((*d).name).is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            let mut slen: size_t = strlen(dn);
            if len_0 > slen
                && memcmp(
                    dn as *const libc::c_void,
                    name.offset(len_0.wrapping_sub(slen) as isize)
                        as *const libc::c_void,
                    slen,
                ) == 0 as i32
            {
                stem = strcache_add_len(name, len_0.wrapping_sub(slen));
                (*file).stem = stem;
                break;
            } else {
                d = (*d).next;
            }
        }
        if d.is_null() {
            stem = b"\0" as *const u8 as *const i8;
            (*file).stem = stem;
        }
    }
    star = stem;
    less = b"\0" as *const u8 as *const i8;
    d = (*file).deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0 && (*d).ignore_automatic_vars() == 0
            && (*d).need_2nd_expansion() == 0
        {
            less = if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name };
            break;
        } else {
            d = (*d).next;
        }
    }
    if !((*file).cmds).is_null() && (*file).cmds == (*default_file).cmds {
        less = at;
    }
    define_variable_in_set(
        b"<\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        less,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"*\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        star,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"@\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        at,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"%\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        percent,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    static mut plus_value: *mut i8 = 0 as *const i8 as *mut i8;
    static mut bar_value: *mut i8 = 0 as *const i8 as *mut i8;
    static mut qmark_value: *mut i8 = 0 as *const i8 as *mut i8;
    static mut plus_max: size_t = 0 as i32 as size_t;
    static mut bar_max: size_t = 0 as i32 as size_t;
    static mut qmark_max: size_t = 0 as i32 as size_t;
    let mut qmark_len: size_t = 0;
    let mut plus_len: size_t = 0;
    let mut bar_len: size_t = 0;
    let mut cp_0: *mut i8 = 0 as *mut i8;
    let mut caret_value: *mut i8 = 0 as *mut i8;
    let mut qp: *mut i8 = 0 as *mut i8;
    let mut bp: *mut i8 = 0 as *mut i8;
    let mut len_1: size_t = 0;
    let mut dep_hash: hash_table = hash_table {
        ht_vec: 0 as *mut *mut libc::c_void,
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
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    plus_len = 0 as i32 as size_t;
    bar_len = 0 as i32 as size_t;
    d = (*file).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 && (*d).ignore_automatic_vars() == 0 {
            if (*d).ignore_mtime() != 0 {
                bar_len = (bar_len as u64)
                    .wrapping_add(
                        (strlen(
                            (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }),
                        ))
                            .wrapping_add(1 as i32 as u64),
                    ) as size_t as size_t;
            } else {
                plus_len = (plus_len as u64)
                    .wrapping_add(
                        (strlen(
                            (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }),
                        ))
                            .wrapping_add(1 as i32 as u64),
                    ) as size_t as size_t;
            }
        }
        d = (*d).next;
    }
    if bar_len == 0 as i32 as u64 {
        bar_len = bar_len.wrapping_add(1);
        bar_len;
    }
    if plus_len == 0 as i32 as u64 {
        plus_len = plus_len.wrapping_add(1);
        plus_len;
    }
    if plus_len > plus_max {
        plus_max = plus_len;
        plus_value = xrealloc(plus_value as *mut libc::c_void, plus_max) as *mut i8;
    }
    cp_0 = plus_value;
    qmark_len = plus_len.wrapping_add(1 as i32 as u64);
    d = (*file).deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0 && (*d).need_2nd_expansion() == 0
            && (*d).ignore_automatic_vars() == 0
        {
            let mut c: *const i8 = if !((*d).name).is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            if ar_name(c) != 0 {
                c = (strchr(c, '(' as i32)).offset(1 as i32 as isize);
                len_1 = (strlen(c)).wrapping_sub(1 as i32 as u64);
            } else {
                len_1 = strlen(c);
            }
            cp_0 = mempcpy(cp_0 as *mut libc::c_void, c as *const libc::c_void, len_1)
                as *mut i8;
            let fresh2 = cp_0;
            cp_0 = cp_0.offset(1);
            *fresh2 = ' ' as i32 as i8;
            if !((*d).changed() as i32 != 0 || always_make_flag != 0) {
                qmark_len = (qmark_len as u64)
                    .wrapping_sub(len_1.wrapping_add(1 as i32 as u64)) as size_t
                    as size_t;
            }
        }
        d = (*d).next;
    }
    *cp_0.offset((if cp_0 > plus_value { -(1 as i32) } else { 0 as i32 }) as isize) = '\0'
        as i32 as i8;
    define_variable_in_set(
        b"+\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        plus_value,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    caret_value = plus_value;
    cp_0 = caret_value;
    if qmark_len > qmark_max {
        qmark_max = qmark_len;
        qmark_value = xrealloc(qmark_value as *mut libc::c_void, qmark_max) as *mut i8;
    }
    qp = qmark_value;
    if bar_len > bar_max {
        bar_max = bar_len;
        bar_value = xrealloc(bar_value as *mut libc::c_void, bar_max) as *mut i8;
    }
    bp = bar_value;
    hash_init(
        &mut dep_hash,
        500 as i32 as u64,
        Some(dep_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(dep_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
        Some(
            dep_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    d = (*file).deps;
    while !d.is_null() {
        if !((*d).need_2nd_expansion() as i32 != 0
            || (*d).ignore_automatic_vars() as i32 != 0)
        {
            slot = hash_find_slot(&mut dep_hash, d as *const libc::c_void);
            if (*slot).is_null() || *slot == hash_deleted_item {
                hash_insert_at(
                    &mut dep_hash,
                    d as *const libc::c_void,
                    slot as *const libc::c_void,
                );
            } else {
                let mut hd: *mut dep = *slot as *mut dep;
                if (*d).ignore_mtime() as i32 != (*hd).ignore_mtime() as i32 {
                    (*hd).set_ignore_mtime(0 as i32 as u32);
                    (*d).set_ignore_mtime((*hd).ignore_mtime());
                }
            }
        }
        d = (*d).next;
    }
    d = (*file).deps;
    while !d.is_null() {
        let mut c_0: *const i8 = 0 as *const i8;
        if !((*d).need_2nd_expansion() as i32 != 0
            || (*d).ignore_automatic_vars() as i32 != 0
            || hash_find_item(&mut dep_hash, d as *const libc::c_void)
                != d as *mut libc::c_void)
        {
            c_0 = if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name };
            if ar_name(c_0) != 0 {
                c_0 = (strchr(c_0, '(' as i32)).offset(1 as i32 as isize);
                len_1 = (strlen(c_0)).wrapping_sub(1 as i32 as u64);
            } else {
                len_1 = strlen(c_0);
            }
            if (*d).ignore_mtime() != 0 {
                bp = mempcpy(bp as *mut libc::c_void, c_0 as *const libc::c_void, len_1)
                    as *mut i8;
                let fresh3 = bp;
                bp = bp.offset(1);
                *fresh3 = ' ' as i32 as i8;
            } else {
                cp_0 = mempcpy(
                    cp_0 as *mut libc::c_void,
                    c_0 as *const libc::c_void,
                    len_1,
                ) as *mut i8;
                let fresh4 = cp_0;
                cp_0 = cp_0.offset(1);
                *fresh4 = ' ' as i32 as i8;
                if (*d).changed() as i32 != 0 || always_make_flag != 0 {
                    qp = mempcpy(
                        qp as *mut libc::c_void,
                        c_0 as *const libc::c_void,
                        len_1,
                    ) as *mut i8;
                    let fresh5 = qp;
                    qp = qp.offset(1);
                    *fresh5 = ' ' as i32 as i8;
                }
            }
        }
        d = (*d).next;
    }
    hash_free(&mut dep_hash, 0 as i32);
    *cp_0.offset((if cp_0 > caret_value { -(1 as i32) } else { 0 as i32 }) as isize) = '\0'
        as i32 as i8;
    define_variable_in_set(
        b"^\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        caret_value,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    *qp.offset((if qp > qmark_value { -(1 as i32) } else { 0 as i32 }) as isize) = '\0'
        as i32 as i8;
    define_variable_in_set(
        b"?\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        qmark_value,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    *bp.offset((if bp > bar_value { -(1 as i32) } else { 0 as i32 }) as isize) = '\0'
        as i32 as i8;
    define_variable_in_set(
        b"|\0" as *const u8 as *const i8,
        1 as i32 as size_t,
        bar_value,
        variable_origin::o_automatic,
        0 as i32,
        (*(*file).variables).set,
        0 as *mut floc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn chop_commands(mut cmds: *mut commands) {
    let mut nlines: libc::c_ushort = 0;
    let mut i: libc::c_ushort = 0;
    let mut lines: *mut *mut i8 = 0 as *mut *mut i8;
    if cmds.is_null() || !((*cmds).command_lines).is_null() {
        return;
    }
    if one_shell != 0 {
        let mut l: size_t = strlen((*cmds).commands);
        nlines = 1 as i32 as libc::c_ushort;
        lines = xmalloc(
            (nlines as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        ) as *mut *mut i8;
        let ref mut fresh6 = *lines.offset(0 as i32 as isize);
        *fresh6 = xstrdup((*cmds).commands);
        if l > 0 as i32 as u64
            && *(*lines.offset(0 as i32 as isize))
                .offset(l.wrapping_sub(1 as i32 as u64) as isize) as i32 == '\n' as i32
        {
            *(*lines.offset(0 as i32 as isize))
                .offset(l.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32 as i8;
        }
    } else {
        let mut p: *const i8 = (*cmds).commands;
        let mut max: size_t = 5 as i32 as size_t;
        nlines = 0 as i32 as libc::c_ushort;
        lines = xmalloc(max.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64))
            as *mut *mut i8;
        while *p as i32 != '\0' as i32 {
            let mut end: *const i8 = p;
            loop {
                end = strchr(end, '\n' as i32);
                if end.is_null() {
                    end = p.offset(strlen(p) as isize);
                    break;
                } else {
                    if !(end > p
                        && *end.offset(-(1 as i32) as isize) as i32 == '\\' as i32)
                    {
                        break;
                    }
                    let mut backslash: i32 = 1 as i32;
                    if end > p.offset(1 as i32 as isize) {
                        let mut b: *const i8 = 0 as *const i8;
                        b = end.offset(-(2 as i32 as isize));
                        while b >= p && *b as i32 == '\\' as i32 {
                            backslash = (backslash == 0) as i32;
                            b = b.offset(-1);
                            b;
                        }
                    }
                    if !(backslash != 0) {
                        break;
                    }
                    end = end.offset(1);
                    end;
                }
            }
            if nlines as i32 == 32767 as i32 * 2 as i32 + 1 as i32 {
                fatal(
                    &mut (*cmds).fileinfo as *mut floc,
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                    dcgettext(
                        0 as *const i8,
                        b"Recipe has too many lines (limit %hu)\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    nlines as i32,
                );
            }
            if nlines as u64 == max {
                max = (max as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
                lines = xrealloc(
                    lines as *mut libc::c_void,
                    max.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                ) as *mut *mut i8;
            }
            let fresh7 = nlines;
            nlines = nlines.wrapping_add(1);
            let ref mut fresh8 = *lines.offset(fresh7 as isize);
            *fresh8 = xstrndup(p, end.offset_from(p) as i64 as size_t);
            p = end;
            if *p as i32 != '\0' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    (*cmds).ncommand_lines = nlines;
    (*cmds).command_lines = lines;
    (*cmds).set_any_recurse(0 as i32 as u32);
    (*cmds).lines_flags = xmalloc(nlines as size_t) as *mut u8;
    i = 0 as i32 as libc::c_ushort;
    while (i as i32) < nlines as i32 {
        let mut flags: u8 = 0 as i32 as u8;
        let mut p_0: *const i8 = *lines.offset(i as isize);
        while *stopchar_map.as_mut_ptr().offset(*p_0 as u8 as isize) as i32 & 0x2 as i32
            != 0 as i32 || *p_0 as i32 == '-' as i32 || *p_0 as i32 == '@' as i32
            || *p_0 as i32 == '+' as i32
        {
            let fresh9 = p_0;
            p_0 = p_0.offset(1);
            match *fresh9 as i32 {
                43 => {
                    flags = (flags as i32 | 1 as i32) as u8;
                }
                64 => {
                    flags = (flags as i32 | 2 as i32) as u8;
                }
                45 => {
                    flags = (flags as i32 | 4 as i32) as u8;
                }
                _ => {}
            }
        }
        if !(flags as i32 & 1 as i32 != 0 as i32)
            && (!(strstr(p_0, b"$(MAKE)\0" as *const u8 as *const i8)).is_null()
                || !(strstr(p_0, b"${MAKE}\0" as *const u8 as *const i8)).is_null())
        {
            flags = (flags as i32 | 1 as i32) as u8;
        }
        *((*cmds).lines_flags).offset(i as isize) = flags;
        (*cmds)
            .set_any_recurse(
                (*cmds).any_recurse()
                    | (if flags as i32 & 1 as i32 != 0 as i32 {
                        1 as i32
                    } else {
                        0 as i32
                    }) as u32,
            );
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn execute_file_commands(mut file: *mut file) {
    let mut p: *const i8 = 0 as *const i8;
    p = (*(*file).cmds).commands;
    while *p as i32 != '\0' as i32 {
        if !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32) && *p as i32 != '-' as i32
            && *p as i32 != '@' as i32 && *p as i32 != '+' as i32
        {
            break;
        }
        p = p.offset(1);
        p;
    }
    if *p as i32 == '\0' as i32 {
        set_command_state(file, cmd_state::cs_running);
        (*file).set_update_status(update_status::us_success);
        notice_finished_file(file);
        return;
    }
    initialize_file_variables(file, 0 as i32);
    set_file_variables(file, (*file).stem);
    if (*file).loaded() as i32 != 0 && unload_file((*file).name) == 0 as i32 {
        (*file).set_loaded(0 as i32 as u32);
        (*file).set_unloaded(1 as i32 as u32);
    }
    new_job(file);
}
#[no_mangle]
pub static mut handling_fatal_signal: sig_atomic_t = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn fatal_error_signal(mut sig: i32) {
    ::core::ptr::write_volatile(
        &mut handling_fatal_signal as *mut sig_atomic_t,
        1 as i32,
    );
    signal(sig, None);
    temp_stdin_unlink();
    osync_clear();
    jobserver_clear();
    if sig == 15 as i32 {
        let mut c: *mut child = 0 as *mut child;
        c = children;
        while !c.is_null() {
            if (*c).remote() == 0 && (*c).pid > 0 as i32 {
                kill((*c).pid, 15 as i32);
            }
            c = (*c).next;
        }
    }
    if sig == 15 as i32 || sig == 2 as i32 || sig == 1 as i32 || sig == 3 as i32 {
        let mut c_0: *mut child = 0 as *mut child;
        c_0 = children;
        while !c_0.is_null() {
            if (*c_0).remote() as i32 != 0 && (*c_0).pid > 0 as i32 {
                remote_kill((*c_0).pid, sig);
            }
            c_0 = (*c_0).next;
        }
        c_0 = children;
        while !c_0.is_null() {
            delete_child_targets(c_0);
            c_0 = (*c_0).next;
        }
        while job_slots_used > 0 as i32 as u32 {
            reap_children(1 as i32, 0 as i32);
        }
    } else {
        while job_slots_used > 0 as i32 as u32 {
            reap_children(1 as i32, 1 as i32);
        }
    }
    remove_intermediates(1 as i32);
    if sig == 3 as i32 {
        exit(1 as i32);
    }
    if kill(make_pid(), sig) < 0 as i32 {
        pfatal_with_name(b"kill\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn delete_target(mut file: *mut file, mut on_behalf_of: *const i8) {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut e: i32 = 0;
    if (*file).precious() as i32 != 0 || (*file).phony() as i32 != 0 {
        return;
    }
    if ar_name((*file).name) != 0 {
        let mut file_date: time_t = if (*file).last_mtime == 1 as i32 as u64 {
            -(1 as i32) as time_t
        } else {
            (((*file).last_mtime).wrapping_sub((2 as i32 + 1 as i32) as u64)
                >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 })) as time_t
        };
        if ar_member_date((*file).name) != file_date {
            if !on_behalf_of.is_null() {
                error(
                    0 as *mut floc,
                    (strlen(on_behalf_of)).wrapping_add(strlen((*file).name)),
                    dcgettext(
                        0 as *const i8,
                        b"*** [%s] Archive member '%s' may be bogus; not deleted\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    on_behalf_of,
                    (*file).name,
                );
            } else {
                error(
                    0 as *mut floc,
                    strlen((*file).name),
                    dcgettext(
                        0 as *const i8,
                        b"*** Archive member '%s' may be bogus; not deleted\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
            }
        }
        return;
    }
    loop {
        e = stat((*file).name, &mut st);
        if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if e == 0 as i32 && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
        && file_timestamp_cons((*file).name, st.st_mtim.tv_sec, st.st_mtim.tv_nsec)
            != (*file).last_mtime
    {
        if !on_behalf_of.is_null() {
            error(
                0 as *mut floc,
                (strlen(on_behalf_of)).wrapping_add(strlen((*file).name)),
                dcgettext(
                    0 as *const i8,
                    b"*** [%s] Deleting file '%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                on_behalf_of,
                (*file).name,
            );
        } else {
            error(
                0 as *mut floc,
                strlen((*file).name),
                dcgettext(
                    0 as *const i8,
                    b"*** Deleting file '%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
        }
        if unlink((*file).name) < 0 as i32 && *__errno_location() != 2 as i32 {
            perror_with_name(b"unlink: \0" as *const u8 as *const i8, (*file).name);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete_child_targets(mut child: *mut child) {
    let mut d: *mut dep = 0 as *mut dep;
    if (*child).deleted() as i32 != 0 || (*child).pid < 0 as i32 {
        return;
    }
    delete_target((*child).file, 0 as *const i8);
    d = (*(*child).file).also_make;
    while !d.is_null() {
        delete_target((*d).file, (*(*child).file).name);
        d = (*d).next;
    }
    (*child).set_deleted(1 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn print_commands(mut cmds: *const commands) {
    let mut s: *const i8 = 0 as *const i8;
    fputs(
        dcgettext(
            0 as *const i8,
            b"#  recipe to execute\0" as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    );
    if ((*cmds).fileinfo.filenm).is_null() {
        puts(
            dcgettext(
                0 as *const i8,
                b" (built-in):\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const i8,
                b" (from '%s', line %lu):\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*cmds).fileinfo.filenm,
            (*cmds).fileinfo.lineno,
        );
    }
    s = (*cmds).commands;
    while *s as i32 != '\0' as i32 {
        let mut end: *const i8 = 0 as *const i8;
        let mut bs: i32 = 0;
        end = s;
        bs = 0 as i32;
        while *end as i32 != '\0' as i32 {
            if *end as i32 == '\n' as i32 && bs == 0 {
                break;
            }
            bs = if *end as i32 == '\\' as i32 { (bs == 0) as i32 } else { 0 as i32 };
            end = end.offset(1);
            end;
        }
        printf(
            b"%c%.*s\n\0" as *const u8 as *const i8,
            cmd_prefix as i32,
            end.offset_from(s) as i64 as i32,
            s,
        );
        s = end
            .offset(
                (*end.offset(0 as i32 as isize) as i32 == '\n' as i32) as i32 as isize,
            );
    }
}
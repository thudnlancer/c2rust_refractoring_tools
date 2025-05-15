use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn remove(__filename: *const i8) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    static mut starting_directory: *mut i8;
    static mut command_count: u64;
    fn __errno_location() -> *mut i32;
    fn strtoll(
        __nptr: *const i8,
        __endptr: *mut *mut i8,
        __base: i32,
    ) -> libc::c_longlong;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn realpath(__name: *const i8, __resolved: *mut i8) -> *mut i8;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn make_lltoa(_: libc::c_longlong, _: *mut i8) -> *mut i8;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrndup(_: *const i8, _: size_t) -> *mut i8;
    fn find_next_token(_: *mut *const i8, _: *mut size_t) -> *mut i8;
    fn next_token(_: *const i8) -> *mut i8;
    fn end_of_token(_: *const i8) -> *mut i8;
    fn alpha_compare(_: *const libc::c_void, _: *const libc::c_void) -> i32;
    fn find_percent(_: *mut i8) -> *mut i8;
    static mut expanding_var: *mut *const floc;
    static mut stopchar_map: [libc::c_ushort; 0];
    fn strcache_add(str: *const i8) -> *const i8;
    static mut reading_file: *const floc;
    fn feof(__stream: *mut FILE) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn hash_init(
        ht: *mut hash_table,
        size: u64,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_load(
        ht: *mut hash_table,
        item_table: *mut libc::c_void,
        cardinality: u64,
        size: u64,
    );
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert(ht: *mut hash_table, item: *const libc::c_void) -> *mut libc::c_void;
    fn hash_free(ht: *mut hash_table, free_items: i32);
    fn jhash(key: *const u8, n: i32) -> u32;
    fn jhash_string(key: *const u8) -> u32;
    static mut current_variable_set_list: *mut variable_set_list;
    fn variable_buffer_output(
        ptr: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn allocated_variable_expand_for_file(line: *const i8, file: *mut file) -> *mut i8;
    fn expand_argument(str: *const i8, end: *const i8) -> *mut i8;
    fn variable_expand_string(
        line: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn install_variable_buffer(bufp: *mut *mut i8, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut i8, len: size_t);
    fn push_new_variable_scope() -> *mut variable_set_list;
    fn pop_variable_scope();
    fn lookup_variable(name: *const i8, length: size_t) -> *mut variable;
    fn define_variable_in_set(
        name: *const i8,
        length: size_t,
        value: *const i8,
        origin: variable_origin,
        recursive: i32,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    fn warn_undefined(name: *const i8, length: size_t);
    fn target_environment(file: *mut file, recursive: i32) -> *mut *mut i8;
    fn parse_file_seq(
        stringp: *mut *mut i8,
        size: size_t,
        stopmap: i32,
        prefix: *const i8,
        flags: i32,
    ) -> *mut libc::c_void;
    fn eval_buffer(buffer: *mut i8, floc: *const floc);
    static mut output_context: *mut output;
    fn output_start();
    fn outputs(is_err: i32, msg: *const i8);
    fn free_childbase(child: *mut childbase);
    fn construct_command_argv(
        line: *mut i8,
        restp: *mut *mut i8,
        file: *mut file,
        cmd_flags: i32,
        batch_file: *mut *mut i8,
    ) -> *mut *mut i8;
    fn reap_children(block: i32, err: i32);
    fn child_execute_job(
        child: *mut childbase,
        good_stdin: i32,
        argv: *mut *mut i8,
    ) -> pid_t;
    fn fd_noinherit(_: i32);
    static mut db_level: i32;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type gmk_func_ptr = Option<
    unsafe extern "C" fn(*const i8, u32, *mut *mut i8) -> *mut i8,
>;
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
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct function_table_entry {
    pub fptr: C2RustUnnamed,
    pub name: *const i8,
    pub len: u8,
    pub minimum_args: u8,
    pub maximum_args: u8,
    #[bitfield(name = "expand_args", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "alloc_fn", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "adds_command", ty = "libc::c_uint", bits = "2..=2")]
    pub expand_args_alloc_fn_adds_command: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub func_ptr: Option<
        unsafe extern "C" fn(*mut i8, *mut *mut i8, *const i8) -> *mut i8,
    >,
    pub alloc_func_ptr: gmk_func_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct childbase {
    pub cmd_name: *mut i8,
    pub environment: *mut *mut i8,
    pub output: output,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_word {
    pub chain: *mut a_word,
    pub str_0: *mut i8,
    pub length: size_t,
    pub matched: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_pattern {
    pub str_0: *mut i8,
    pub percent: *mut i8,
    pub length: size_t,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn function_table_entry_hash_1(mut keyv: *const libc::c_void) -> u64 {
    let mut key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: u64 = 0 as i32 as u64;
    let mut _key_: *const u8 = (*key).name as *const u8;
    _result_ = _result_.wrapping_add(jhash(_key_, (*key).len as i32) as u64);
    return _result_;
}
unsafe extern "C" fn function_table_entry_hash_2(mut keyv: *const libc::c_void) -> u64 {
    let mut key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: u64 = 0 as i32 as u64;
    return _result_;
}
unsafe extern "C" fn function_table_entry_hash_cmp(
    mut xv: *const libc::c_void,
    mut yv: *const libc::c_void,
) -> i32 {
    let mut x: *const function_table_entry = xv as *const function_table_entry;
    let mut y: *const function_table_entry = yv as *const function_table_entry;
    let mut result: i32 = (*x).len as i32 - (*y).len as i32;
    if result != 0 {
        return result;
    }
    return if (*x).name == (*y).name {
        0 as i32
    } else {
        memcmp(
            (*x).name as *const libc::c_void,
            (*y).name as *const libc::c_void,
            (*x).len as u64,
        )
    };
}
static mut function_table: hash_table = hash_table {
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
#[no_mangle]
pub unsafe extern "C" fn subst_expand(
    mut o: *mut i8,
    mut text: *const i8,
    mut subst: *const i8,
    mut replace: *const i8,
    mut slen: size_t,
    mut rlen: size_t,
    mut by_word: i32,
) -> *mut i8 {
    let mut t: *const i8 = text;
    let mut p: *const i8 = 0 as *const i8;
    if slen == 0 as i32 as u64 && by_word == 0 {
        o = variable_buffer_output(o, t, strlen(t));
        if rlen > 0 as i32 as u64 {
            o = variable_buffer_output(o, replace, rlen);
        }
        return o;
    }
    loop {
        if by_word != 0 && slen == 0 as i32 as u64 {
            p = end_of_token(next_token(t));
        } else {
            p = strstr(t, subst);
            if p.is_null() {
                o = variable_buffer_output(o, t, strlen(t));
                return o;
            }
        }
        if p > t {
            o = variable_buffer_output(o, t, p.offset_from(t) as i64 as size_t);
        }
        if by_word != 0
            && (p > text
                && !(*stopchar_map
                    .as_mut_ptr()
                    .offset(*p.offset(-(1 as i32) as isize) as u8 as isize) as i32
                    & (0x2 as i32 | 0x4 as i32) != 0 as i32)
                || !(*stopchar_map
                    .as_mut_ptr()
                    .offset(*p.offset(slen as isize) as u8 as isize) as i32
                    & (0x2 as i32 | 0x4 as i32 | 0x1 as i32) != 0 as i32))
        {
            o = variable_buffer_output(o, subst, slen);
        } else if rlen > 0 as i32 as u64 {
            o = variable_buffer_output(o, replace, rlen);
        }
        t = p.offset(slen as isize);
        if !(*t as i32 != '\0' as i32) {
            break;
        }
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn patsubst_expand_pat(
    mut o: *mut i8,
    mut text: *const i8,
    mut pattern: *const i8,
    mut replace: *const i8,
    mut pattern_percent: *const i8,
    mut replace_percent: *const i8,
) -> *mut i8 {
    let mut pattern_prepercent_len: size_t = 0;
    let mut pattern_postpercent_len: size_t = 0;
    let mut replace_prepercent_len: size_t = 0;
    let mut replace_postpercent_len: size_t = 0;
    let mut t: *const i8 = 0 as *const i8;
    let mut len: size_t = 0;
    let mut doneany: i32 = 0 as i32;
    if !replace_percent.is_null() {
        replace_prepercent_len = (replace_percent.offset_from(replace) as i64
            - 1 as i32 as i64) as size_t;
        replace_postpercent_len = strlen(replace_percent);
    } else {
        replace_prepercent_len = strlen(replace);
        replace_postpercent_len = 0 as i32 as size_t;
    }
    if pattern_percent.is_null() {
        return subst_expand(
            o,
            text,
            pattern,
            replace,
            strlen(pattern),
            strlen(replace),
            1 as i32,
        );
    }
    pattern_prepercent_len = (pattern_percent.offset_from(pattern) as i64
        - 1 as i32 as i64) as size_t;
    pattern_postpercent_len = strlen(pattern_percent);
    loop {
        t = find_next_token(&mut text, &mut len);
        if t.is_null() {
            break;
        }
        let mut fail: i32 = 0 as i32;
        if len < pattern_prepercent_len.wrapping_add(pattern_postpercent_len) {
            fail = 1 as i32;
        }
        if fail == 0 && pattern_prepercent_len > 0 as i32 as u64
            && (*t as i32 != *pattern as i32
                || *t
                    .offset(
                        pattern_prepercent_len.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32 != *pattern_percent.offset(-(2 as i32) as isize) as i32
                || !(strncmp(
                    t.offset(1 as i32 as isize),
                    pattern.offset(1 as i32 as isize),
                    pattern_prepercent_len.wrapping_sub(1 as i32 as u64),
                ) == 0 as i32))
        {
            fail = 1 as i32;
        }
        if fail == 0 && pattern_postpercent_len > 0 as i32 as u64
            && (*t.offset(len.wrapping_sub(1 as i32 as u64) as isize) as i32
                != *pattern_percent
                    .offset(
                        pattern_postpercent_len.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32
                || *t.offset(len.wrapping_sub(pattern_postpercent_len) as isize) as i32
                    != *pattern_percent as i32
                || !(strncmp(
                    &*t.offset(len.wrapping_sub(pattern_postpercent_len) as isize),
                    pattern_percent,
                    pattern_postpercent_len.wrapping_sub(1 as i32 as u64),
                ) == 0 as i32))
        {
            fail = 1 as i32;
        }
        if fail != 0 {
            o = variable_buffer_output(o, t, len);
        } else {
            o = variable_buffer_output(o, replace, replace_prepercent_len);
            if !replace_percent.is_null() {
                o = variable_buffer_output(
                    o,
                    t.offset(pattern_prepercent_len as isize),
                    len
                        .wrapping_sub(
                            pattern_prepercent_len.wrapping_add(pattern_postpercent_len),
                        ),
                );
                o = variable_buffer_output(o, replace_percent, replace_postpercent_len);
            }
        }
        if fail != 0 || replace_prepercent_len > 0 as i32 as u64
            || !replace_percent.is_null()
                && len.wrapping_add(replace_postpercent_len) > 0 as i32 as u64
        {
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const i8,
                1 as i32 as size_t,
            );
            doneany = 1 as i32;
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn patsubst_expand(
    mut o: *mut i8,
    mut text: *const i8,
    mut pattern: *mut i8,
    mut replace: *mut i8,
) -> *mut i8 {
    let mut pattern_percent: *const i8 = find_percent(pattern);
    let mut replace_percent: *const i8 = find_percent(replace);
    if !replace_percent.is_null() {
        replace_percent = replace_percent.offset(1);
        replace_percent;
    }
    if !pattern_percent.is_null() {
        pattern_percent = pattern_percent.offset(1);
        pattern_percent;
    }
    return patsubst_expand_pat(
        o,
        text,
        pattern,
        replace,
        pattern_percent,
        replace_percent,
    );
}
unsafe extern "C" fn lookup_function(mut s: *const i8) -> *const function_table_entry {
    let mut function_table_entry_key: function_table_entry = function_table_entry {
        fptr: C2RustUnnamed { func_ptr: None },
        name: 0 as *const i8,
        len: 0,
        minimum_args: 0,
        maximum_args: 0,
        expand_args_alloc_fn_adds_command: [0; 1],
        c2rust_padding: [0; 4],
    };
    let mut e: *const i8 = s;
    while *stopchar_map.as_mut_ptr().offset(*e as u8 as isize) as i32 & 0x2000 as i32
        != 0 as i32
    {
        e = e.offset(1);
        e;
    }
    if e == s
        || !(*stopchar_map.as_mut_ptr().offset(*e as u8 as isize) as i32
            & (0x1 as i32 | (0x2 as i32 | 0x4 as i32)) != 0 as i32)
    {
        return 0 as *const function_table_entry;
    }
    function_table_entry_key.name = s;
    function_table_entry_key.len = e.offset_from(s) as i64 as u8;
    return hash_find_item(
        &mut function_table,
        &mut function_table_entry_key as *mut function_table_entry as *const libc::c_void,
    ) as *const function_table_entry;
}
#[no_mangle]
pub unsafe extern "C" fn pattern_matches(
    mut pattern: *const i8,
    mut percent: *const i8,
    mut str: *const i8,
) -> i32 {
    let mut sfxlen: size_t = 0;
    let mut strlength: size_t = 0;
    if percent.is_null() {
        let mut len: size_t = (strlen(pattern)).wrapping_add(1 as i32 as u64);
        let mut fresh0 = ::std::vec::from_elem(0, len as usize);
        let mut new_chars: *mut i8 = fresh0.as_mut_ptr() as *mut i8;
        memcpy(new_chars as *mut libc::c_void, pattern as *const libc::c_void, len);
        percent = find_percent(new_chars);
        if percent.is_null() {
            return (new_chars == str as *mut i8
                || *new_chars as i32 == *str as i32
                    && (*new_chars as i32 == '\0' as i32
                        || strcmp(
                            new_chars.offset(1 as i32 as isize),
                            str.offset(1 as i32 as isize),
                        ) == 0)) as i32;
        }
        pattern = new_chars;
    }
    sfxlen = strlen(percent.offset(1 as i32 as isize));
    strlength = strlen(str);
    if strlength < (percent.offset_from(pattern) as i64 as u64).wrapping_add(sfxlen)
        || !(strncmp(pattern, str, percent.offset_from(pattern) as i64 as u64)
            == 0 as i32)
    {
        return 0 as i32;
    }
    return (strcmp(
        percent.offset(1 as i32 as isize),
        str.offset(strlength.wrapping_sub(sfxlen) as isize),
    ) == 0) as i32;
}
unsafe extern "C" fn find_next_argument(
    mut startparen: i8,
    mut endparen: i8,
    mut ptr: *const i8,
    mut end: *const i8,
) -> *mut i8 {
    let mut count: i32 = 0 as i32;
    while ptr < end {
        if *stopchar_map.as_mut_ptr().offset(*ptr as u8 as isize) as i32
            & (0x80 as i32 | 0x400 as i32) != 0 as i32
        {
            if *ptr as i32 == startparen as i32 {
                count += 1;
                count;
            } else if *ptr as i32 == endparen as i32 {
                count -= 1;
                count;
                if count < 0 as i32 {
                    return 0 as *mut i8;
                }
            } else if *ptr as i32 == ',' as i32 && count == 0 {
                return ptr as *mut i8
            }
        }
        ptr = ptr.offset(1);
        ptr;
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn string_glob(mut line: *mut i8) -> *mut i8 {
    static mut result: *mut i8 = 0 as *const i8 as *mut i8;
    static mut length: size_t = 0;
    let mut chain: *mut nameseq = 0 as *mut nameseq;
    let mut idx: size_t = 0;
    chain = parse_file_seq(
        &mut line,
        ::core::mem::size_of::<nameseq>() as u64,
        0x1 as i32,
        0 as *const i8,
        0x1 as i32 | 0x10 as i32 | 0x8 as i32,
    ) as *mut nameseq;
    if result.is_null() {
        length = 100 as i32 as size_t;
        result = xmalloc(100 as i32 as size_t) as *mut i8;
    }
    idx = 0 as i32 as size_t;
    while !chain.is_null() {
        let mut next: *mut nameseq = (*chain).next;
        let mut len: size_t = strlen((*chain).name);
        if idx.wrapping_add(len).wrapping_add(1 as i32 as u64) > length {
            length = (length as u64)
                .wrapping_add(
                    len.wrapping_add(1 as i32 as u64).wrapping_mul(2 as i32 as u64),
                ) as size_t as size_t;
            result = xrealloc(result as *mut libc::c_void, length) as *mut i8;
        }
        memcpy(
            &mut *result.offset(idx as isize) as *mut i8 as *mut libc::c_void,
            (*chain).name as *const libc::c_void,
            len,
        );
        idx = (idx as u64).wrapping_add(len) as size_t as size_t;
        let fresh1 = idx;
        idx = idx.wrapping_add(1);
        *result.offset(fresh1 as isize) = ' ' as i32 as i8;
        free((*chain).name as *mut i8 as *mut libc::c_void);
        free(chain as *mut libc::c_void);
        chain = next;
    }
    if idx == 0 as i32 as u64 {
        *result.offset(0 as i32 as isize) = '\0' as i32 as i8;
    } else {
        *result.offset(idx.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32 as i8;
    }
    return result;
}
unsafe extern "C" fn func_patsubst(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    o = patsubst_expand(
        o,
        *argv.offset(2 as i32 as isize),
        *argv.offset(0 as i32 as isize),
        *argv.offset(1 as i32 as isize),
    );
    return o;
}
unsafe extern "C" fn func_join(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut doneany: i32 = 0 as i32;
    let mut tp: *const i8 = 0 as *const i8;
    let mut pp: *const i8 = 0 as *const i8;
    let mut list1_iterator: *const i8 = *argv.offset(0 as i32 as isize);
    let mut list2_iterator: *const i8 = *argv.offset(1 as i32 as isize);
    loop {
        let mut len1: size_t = 0;
        let mut len2: size_t = 0;
        tp = find_next_token(&mut list1_iterator, &mut len1);
        if !tp.is_null() {
            o = variable_buffer_output(o, tp, len1);
        }
        pp = find_next_token(&mut list2_iterator, &mut len2);
        if !pp.is_null() {
            o = variable_buffer_output(o, pp, len2);
        }
        if !tp.is_null() || !pp.is_null() {
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const i8,
                1 as i32 as size_t,
            );
            doneany = 1 as i32;
        }
        if !(!tp.is_null() || !pp.is_null()) {
            break;
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_origin(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut v: *mut variable = lookup_variable(
        *argv.offset(0 as i32 as isize),
        strlen(*argv.offset(0 as i32 as isize)),
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const i8,
            9 as i32 as size_t,
        );
    } else {
        match (*v).origin() as i32 {
            7 => {
                abort();
            }
            0 => {
                o = variable_buffer_output(
                    o,
                    b"default\0" as *const u8 as *const i8,
                    7 as i32 as size_t,
                );
            }
            1 => {
                o = variable_buffer_output(
                    o,
                    b"environment\0" as *const u8 as *const i8,
                    11 as i32 as size_t,
                );
            }
            2 => {
                o = variable_buffer_output(
                    o,
                    b"file\0" as *const u8 as *const i8,
                    4 as i32 as size_t,
                );
            }
            3 => {
                o = variable_buffer_output(
                    o,
                    b"environment override\0" as *const u8 as *const i8,
                    20 as i32 as size_t,
                );
            }
            4 => {
                o = variable_buffer_output(
                    o,
                    b"command line\0" as *const u8 as *const i8,
                    12 as i32 as size_t,
                );
            }
            5 => {
                o = variable_buffer_output(
                    o,
                    b"override\0" as *const u8 as *const i8,
                    8 as i32 as size_t,
                );
            }
            6 => {
                o = variable_buffer_output(
                    o,
                    b"automatic\0" as *const u8 as *const i8,
                    9 as i32 as size_t,
                );
            }
            _ => {}
        }
    }
    return o;
}
unsafe extern "C" fn func_flavor(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut v: *mut variable = lookup_variable(
        *argv.offset(0 as i32 as isize),
        strlen(*argv.offset(0 as i32 as isize)),
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const i8,
            9 as i32 as size_t,
        );
    } else if (*v).recursive() != 0 {
        o = variable_buffer_output(
            o,
            b"recursive\0" as *const u8 as *const i8,
            9 as i32 as size_t,
        );
    } else {
        o = variable_buffer_output(
            o,
            b"simple\0" as *const u8 as *const i8,
            6 as i32 as size_t,
        );
    }
    return o;
}
unsafe extern "C" fn func_notdir_suffix(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut list_iterator: *const i8 = *argv.offset(0 as i32 as isize);
    let mut p2: *const i8 = 0 as *const i8;
    let mut doneany: i32 = 0 as i32;
    let mut len: size_t = 0 as i32 as size_t;
    let mut is_suffix: i32 = (*funcname.offset(0 as i32 as isize) as i32 == 's' as i32)
        as i32;
    let mut is_notdir: i32 = (is_suffix == 0) as i32;
    let mut stop: i32 = 0x8000 as i32
        | (if is_suffix != 0 { 0x200 as i32 } else { 0 as i32 });
    loop {
        p2 = find_next_token(&mut list_iterator, &mut len);
        if p2.is_null() {
            break;
        }
        let mut p: *const i8 = p2.offset(len as isize).offset(-(1 as i32 as isize));
        while p >= p2
            && !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & stop
                != 0 as i32)
        {
            p = p.offset(-1);
            p;
        }
        if p >= p2 {
            if is_notdir != 0 {
                p = p.offset(1);
                p;
            } else if *p as i32 != '.' as i32 {
                continue;
            }
            o = variable_buffer_output(
                o,
                p,
                len.wrapping_sub(p.offset_from(p2) as i64 as u64),
            );
        } else if is_notdir != 0 {
            o = variable_buffer_output(o, p2, len);
        }
        if is_notdir != 0 || p >= p2 {
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const i8,
                1 as i32 as size_t,
            );
            doneany = 1 as i32;
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_basename_dir(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut p3: *const i8 = *argv.offset(0 as i32 as isize);
    let mut p2: *const i8 = 0 as *const i8;
    let mut doneany: i32 = 0 as i32;
    let mut len: size_t = 0 as i32 as size_t;
    let mut is_basename: i32 = (*funcname.offset(0 as i32 as isize) as i32 == 'b' as i32)
        as i32;
    let mut is_dir: i32 = (is_basename == 0) as i32;
    let mut stop: i32 = 0x8000 as i32
        | (if is_basename != 0 { 0x200 as i32 } else { 0 as i32 }) | 0x1 as i32;
    loop {
        p2 = find_next_token(&mut p3, &mut len);
        if p2.is_null() {
            break;
        }
        let mut p: *const i8 = p2.offset(len as isize).offset(-(1 as i32 as isize));
        while p >= p2
            && !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & stop
                != 0 as i32)
        {
            p = p.offset(-1);
            p;
        }
        if p >= p2 && is_dir != 0 {
            p = p.offset(1);
            o = variable_buffer_output(o, p2, p.offset_from(p2) as i64 as size_t);
        } else if p >= p2 && *p as i32 == '.' as i32 {
            o = variable_buffer_output(o, p2, p.offset_from(p2) as i64 as size_t);
        } else if is_dir != 0 {
            o = variable_buffer_output(
                o,
                b"./\0" as *const u8 as *const i8,
                2 as i32 as size_t,
            );
        } else {
            o = variable_buffer_output(o, p2, len);
        }
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const i8,
            1 as i32 as size_t,
        );
        doneany = 1 as i32;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_addsuffix_addprefix(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut fixlen: size_t = strlen(*argv.offset(0 as i32 as isize));
    let mut list_iterator: *const i8 = *argv.offset(1 as i32 as isize);
    let mut is_addprefix: i32 = (*funcname.offset(3 as i32 as isize) as i32
        == 'p' as i32) as i32;
    let mut is_addsuffix: i32 = (is_addprefix == 0) as i32;
    let mut doneany: i32 = 0 as i32;
    let mut p: *const i8 = 0 as *const i8;
    let mut len: size_t = 0;
    loop {
        p = find_next_token(&mut list_iterator, &mut len);
        if p.is_null() {
            break;
        }
        if is_addprefix != 0 {
            o = variable_buffer_output(o, *argv.offset(0 as i32 as isize), fixlen);
        }
        o = variable_buffer_output(o, p, len);
        if is_addsuffix != 0 {
            o = variable_buffer_output(o, *argv.offset(0 as i32 as isize), fixlen);
        }
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const i8,
            1 as i32 as size_t,
        );
        doneany = 1 as i32;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_subst(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    o = subst_expand(
        o,
        *argv.offset(2 as i32 as isize),
        *argv.offset(0 as i32 as isize),
        *argv.offset(1 as i32 as isize),
        strlen(*argv.offset(0 as i32 as isize)),
        strlen(*argv.offset(1 as i32 as isize)),
        0 as i32,
    );
    return o;
}
unsafe extern "C" fn func_firstword(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut i: size_t = 0;
    let mut words: *const i8 = *argv.offset(0 as i32 as isize);
    let mut p: *const i8 = find_next_token(&mut words, &mut i);
    if !p.is_null() {
        o = variable_buffer_output(o, p, i);
    }
    return o;
}
unsafe extern "C" fn func_lastword(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut i: size_t = 0;
    let mut words: *const i8 = *argv.offset(0 as i32 as isize);
    let mut p: *const i8 = 0 as *const i8;
    let mut t: *const i8 = 0 as *const i8;
    loop {
        t = find_next_token(&mut words, &mut i);
        if t.is_null() {
            break;
        }
        p = t;
    }
    if !p.is_null() {
        o = variable_buffer_output(o, p, i);
    }
    return o;
}
unsafe extern "C" fn func_words(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut i: u32 = 0 as i32 as u32;
    let mut word_iterator: *const i8 = *argv.offset(0 as i32 as isize);
    let mut buf: [i8; 22] = [0; 22];
    while !(find_next_token(&mut word_iterator, 0 as *mut size_t)).is_null() {
        i = i.wrapping_add(1);
        i;
    }
    sprintf(buf.as_mut_ptr(), b"%u\0" as *const u8 as *const i8, i);
    o = variable_buffer_output(o, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn strip_whitespace(
    mut begpp: *mut *const i8,
    mut endpp: *mut *const i8,
) -> *mut i8 {
    while *begpp <= *endpp
        && *stopchar_map.as_mut_ptr().offset(**begpp as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        *begpp = (*begpp).offset(1);
        *begpp;
    }
    while *endpp >= *begpp
        && *stopchar_map.as_mut_ptr().offset(**endpp as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        *endpp = (*endpp).offset(-1);
        *endpp;
    }
    return *begpp as *mut i8;
}
unsafe extern "C" fn parse_numeric(
    mut s: *const i8,
    mut msg: *const i8,
) -> libc::c_longlong {
    let mut beg: *const i8 = s;
    let mut end: *const i8 = s.offset(strlen(s) as isize).offset(-(1 as i32 as isize));
    let mut endp: *mut i8 = 0 as *mut i8;
    let mut num: libc::c_longlong = 0;
    strip_whitespace(&mut beg, &mut end);
    if beg > end {
        fatal(
            *expanding_var,
            strlen(msg),
            dcgettext(
                0 as *const i8,
                b"%s: empty value\0" as *const u8 as *const i8,
                5 as i32,
            ),
            msg,
        );
    }
    *__errno_location() = 0 as i32;
    num = strtoll(beg, &mut endp, 10 as i32);
    if *__errno_location() == 34 as i32 {
        fatal(
            *expanding_var,
            (strlen(msg)).wrapping_add(strlen(s)),
            dcgettext(
                0 as *const i8,
                b"%s: '%s' out of range\0" as *const u8 as *const i8,
                5 as i32,
            ),
            msg,
            s,
        );
    } else if endp == beg as *mut i8 || endp <= end as *mut i8 {
        fatal(
            *expanding_var,
            (strlen(msg)).wrapping_add(strlen(s)),
            b"%s: '%s'\0" as *const u8 as *const i8,
            msg,
            s,
        );
    }
    return num;
}
unsafe extern "C" fn func_word(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut end_p: *const i8 = 0 as *const i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut i: libc::c_longlong = 0;
    i = parse_numeric(
        *argv.offset(0 as i32 as isize),
        dcgettext(
            0 as *const i8,
            b"invalid first argument to 'word' function\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    if i < 1 as i32 as libc::c_longlong {
        fatal(
            *expanding_var,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"first argument to 'word' function must be greater than 0\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    end_p = *argv.offset(1 as i32 as isize);
    loop {
        p = find_next_token(&mut end_p, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        i -= 1;
        if i == 0 as i32 as libc::c_longlong {
            break;
        }
    }
    if i == 0 as i32 as libc::c_longlong {
        o = variable_buffer_output(o, p, end_p.offset_from(p) as i64 as size_t);
    }
    return o;
}
unsafe extern "C" fn func_wordlist(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut buf: [i8; 23] = [0; 23];
    let mut start: libc::c_longlong = 0;
    let mut stop: libc::c_longlong = 0;
    let mut count: libc::c_longlong = 0;
    let mut badfirst: *const i8 = dcgettext(
        0 as *const i8,
        b"invalid first argument to 'wordlist' function\0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut badsecond: *const i8 = dcgettext(
        0 as *const i8,
        b"invalid second argument to 'wordlist' function\0" as *const u8 as *const i8,
        5 as i32,
    );
    start = parse_numeric(*argv.offset(0 as i32 as isize), badfirst);
    if start < 1 as i32 as libc::c_longlong {
        fatal(
            *expanding_var,
            (strlen(badfirst)).wrapping_add(strlen(make_lltoa(start, buf.as_mut_ptr()))),
            b"%s: '%s'\0" as *const u8 as *const i8,
            badfirst,
            make_lltoa(start, buf.as_mut_ptr()),
        );
    }
    stop = parse_numeric(*argv.offset(1 as i32 as isize), badsecond);
    if stop < 0 as i32 as libc::c_longlong {
        fatal(
            *expanding_var,
            (strlen(badsecond)).wrapping_add(strlen(make_lltoa(stop, buf.as_mut_ptr()))),
            b"%s: '%s'\0" as *const u8 as *const i8,
            badsecond,
            make_lltoa(stop, buf.as_mut_ptr()),
        );
    }
    count = stop - start + 1 as i32 as libc::c_longlong;
    if count > 0 as i32 as libc::c_longlong {
        let mut p: *const i8 = 0 as *const i8;
        let mut end_p: *const i8 = *argv.offset(2 as i32 as isize);
        loop {
            p = find_next_token(&mut end_p, 0 as *mut size_t);
            if !(!p.is_null()
                && {
                    start -= 1;
                    start != 0
                })
            {
                break;
            }
        }
        if !p.is_null() {
            loop {
                count -= 1;
                if !(count != 0
                    && !(find_next_token(&mut end_p, 0 as *mut size_t)).is_null())
                {
                    break;
                }
            }
            o = variable_buffer_output(o, p, end_p.offset_from(p) as i64 as size_t);
        }
    }
    return o;
}
unsafe extern "C" fn func_findstring(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    if !(strstr(*argv.offset(1 as i32 as isize), *argv.offset(0 as i32 as isize)))
        .is_null()
    {
        o = variable_buffer_output(
            o,
            *argv.offset(0 as i32 as isize),
            strlen(*argv.offset(0 as i32 as isize)),
        );
    }
    return o;
}
unsafe extern "C" fn func_foreach(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut varname: *mut i8 = expand_argument(
        *argv.offset(0 as i32 as isize),
        0 as *const i8,
    );
    let mut list: *mut i8 = expand_argument(
        *argv.offset(1 as i32 as isize),
        0 as *const i8,
    );
    let mut body: *const i8 = *argv.offset(2 as i32 as isize);
    let mut doneany: i32 = 0 as i32;
    let mut list_iterator: *const i8 = list;
    let mut p: *const i8 = 0 as *const i8;
    let mut len: size_t = 0;
    let mut var: *mut variable = 0 as *mut variable;
    let mut vp: *mut i8 = next_token(varname);
    *(end_of_token(vp)).offset(0 as i32 as isize) = '\0' as i32 as i8;
    push_new_variable_scope();
    var = define_variable_in_set(
        vp,
        strlen(vp),
        b"\0" as *const u8 as *const i8,
        variable_origin::o_automatic,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    loop {
        p = find_next_token(&mut list_iterator, &mut len);
        if p.is_null() {
            break;
        }
        let mut result: *mut i8 = 0 as *mut i8;
        free((*var).value as *mut libc::c_void);
        (*var).value = xstrndup(p, len);
        result = allocated_variable_expand_for_file(body, 0 as *mut file);
        o = variable_buffer_output(o, result, strlen(result));
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const i8,
            1 as i32 as size_t,
        );
        doneany = 1 as i32;
        free(result as *mut libc::c_void);
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    pop_variable_scope();
    free(varname as *mut libc::c_void);
    free(list as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn func_let(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut varnames: *mut i8 = expand_argument(
        *argv.offset(0 as i32 as isize),
        0 as *const i8,
    );
    let mut list: *mut i8 = expand_argument(
        *argv.offset(1 as i32 as isize),
        0 as *const i8,
    );
    let mut body: *const i8 = *argv.offset(2 as i32 as isize);
    let mut vp: *const i8 = 0 as *const i8;
    let mut vp_next: *const i8 = varnames;
    let mut list_iterator: *const i8 = list;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut vlen: size_t = 0;
    push_new_variable_scope();
    vp = find_next_token(&mut vp_next, &mut vlen);
    while *stopchar_map.as_mut_ptr().offset(*vp_next as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        vp_next = vp_next.offset(1);
        vp_next;
    }
    while *vp_next as i32 != '\0' as i32 {
        p = find_next_token(&mut list_iterator, &mut len);
        if *list_iterator as i32 != '\0' as i32 {
            list_iterator = list_iterator.offset(1);
            list_iterator;
            *p.offset(len as isize) = '\0' as i32 as i8;
        }
        define_variable_in_set(
            vp,
            vlen,
            if !p.is_null() { p } else { b"\0" as *const u8 as *const i8 },
            variable_origin::o_automatic,
            0 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        vp = find_next_token(&mut vp_next, &mut vlen);
        while *stopchar_map.as_mut_ptr().offset(*vp_next as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32
        {
            vp_next = vp_next.offset(1);
            vp_next;
        }
    }
    if !vp.is_null() {
        define_variable_in_set(
            vp,
            vlen,
            next_token(list_iterator),
            variable_origin::o_automatic,
            0 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
    o = variable_expand_string(o, body, 18446744073709551615 as u64);
    pop_variable_scope();
    free(varnames as *mut libc::c_void);
    free(list as *mut libc::c_void);
    return o.offset(strlen(o) as isize);
}
unsafe extern "C" fn a_word_hash_1(mut key: *const libc::c_void) -> u64 {
    let mut _result_: u64 = 0 as i32 as u64;
    let mut _key_: *const u8 = (*(key as *const a_word)).str_0 as *const u8;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as u64);
    return _result_;
}
unsafe extern "C" fn a_word_hash_2(mut key: *const libc::c_void) -> u64 {
    let mut _result_: u64 = 0 as i32 as u64;
    return _result_;
}
unsafe extern "C" fn a_word_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> i32 {
    let mut ax: *const a_word = x as *const a_word;
    let mut ay: *const a_word = y as *const a_word;
    if (*ax).length != (*ay).length {
        return if (*ax).length > (*ay).length { 1 as i32 } else { -(1 as i32) };
    }
    return if (*ax).str_0 == (*ay).str_0 {
        0 as i32
    } else {
        memcmp(
            (*ax).str_0 as *const libc::c_void,
            (*ay).str_0 as *const libc::c_void,
            (*ax).length,
        )
    };
}
unsafe extern "C" fn func_filter_filterout(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut words: *mut a_word = 0 as *mut a_word;
    let mut word_end: *mut a_word = 0 as *mut a_word;
    let mut wp: *mut a_word = 0 as *mut a_word;
    let mut patterns: *mut a_pattern = 0 as *mut a_pattern;
    let mut pat_end: *mut a_pattern = 0 as *mut a_pattern;
    let mut pp: *mut a_pattern = 0 as *mut a_pattern;
    let mut pat_count: u64 = 0 as i32 as u64;
    let mut word_count: u64 = 0 as i32 as u64;
    let mut a_word_table: hash_table = hash_table {
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
    let mut is_filter: i32 = (*funcname
        .offset(
            (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
                as isize,
        ) as i32 == '\0' as i32) as i32;
    let mut cp: *const i8 = 0 as *const i8;
    let mut literals: i32 = 0 as i32;
    let mut hashing: i32 = 0 as i32;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut doneany: i32 = 0 as i32;
    cp = *argv.offset(1 as i32 as isize);
    loop {
        p = find_next_token(&mut cp, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        word_count = word_count.wrapping_add(1);
        word_count;
    }
    if word_count == 0 {
        return o;
    }
    words = xcalloc(word_count.wrapping_mul(::core::mem::size_of::<a_word>() as u64))
        as *mut a_word;
    word_end = words.offset(word_count as isize);
    cp = *argv.offset(0 as i32 as isize);
    loop {
        p = find_next_token(&mut cp, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        pat_count = pat_count.wrapping_add(1);
        pat_count;
    }
    patterns = xcalloc(
        pat_count.wrapping_mul(::core::mem::size_of::<a_pattern>() as u64),
    ) as *mut a_pattern;
    pat_end = patterns.offset(pat_count as isize);
    cp = *argv.offset(0 as i32 as isize);
    pp = patterns;
    loop {
        p = find_next_token(&mut cp, &mut len);
        if p.is_null() {
            break;
        }
        if *cp as i32 != '\0' as i32 {
            cp = cp.offset(1);
            cp;
        }
        *p.offset(len as isize) = '\0' as i32 as i8;
        (*pp).str_0 = p;
        (*pp).percent = find_percent(p);
        if ((*pp).percent).is_null() {
            literals += 1;
            literals;
        }
        (*pp).length = strlen((*pp).str_0);
        pp = pp.offset(1);
        pp;
    }
    cp = *argv.offset(1 as i32 as isize);
    wp = words;
    loop {
        p = find_next_token(&mut cp, &mut len);
        if p.is_null() {
            break;
        }
        if *cp as i32 != '\0' as i32 {
            cp = cp.offset(1);
            cp;
        }
        *p.offset(len as isize) = '\0' as i32 as i8;
        (*wp).str_0 = p;
        (*wp).length = len;
        wp = wp.offset(1);
        wp;
    }
    hashing = (literals > 1 as i32
        && (literals as u64).wrapping_mul(word_count) >= 10 as i32 as u64) as i32;
    if hashing != 0 {
        hash_init(
            &mut a_word_table,
            word_count,
            Some(a_word_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> u64),
            Some(a_word_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> u64),
            Some(
                a_word_hash_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
        wp = words;
        while wp < word_end {
            let mut owp: *mut a_word = hash_insert(
                &mut a_word_table,
                wp as *const libc::c_void,
            ) as *mut a_word;
            if !owp.is_null() {
                (*wp).chain = owp;
            }
            wp = wp.offset(1);
            wp;
        }
    }
    pp = patterns;
    while pp < pat_end {
        if !((*pp).percent).is_null() {
            wp = words;
            while wp < word_end {
                (*wp).matched
                    |= pattern_matches((*pp).str_0, (*pp).percent, (*wp).str_0);
                wp = wp.offset(1);
                wp;
            }
        } else if hashing != 0 {
            let mut a_word_key: a_word = a_word {
                chain: 0 as *mut a_word,
                str_0: 0 as *mut i8,
                length: 0,
                matched: 0,
            };
            a_word_key.str_0 = (*pp).str_0;
            a_word_key.length = (*pp).length;
            wp = hash_find_item(
                &mut a_word_table,
                &mut a_word_key as *mut a_word as *const libc::c_void,
            ) as *mut a_word;
            while !wp.is_null() {
                (*wp).matched |= 1 as i32;
                wp = (*wp).chain;
            }
        } else {
            wp = words;
            while wp < word_end {
                (*wp).matched
                    |= ((*wp).length == (*pp).length
                        && memcmp(
                            (*pp).str_0 as *const libc::c_void,
                            (*wp).str_0 as *const libc::c_void,
                            (*wp).length,
                        ) == 0 as i32) as i32;
                wp = wp.offset(1);
                wp;
            }
        }
        pp = pp.offset(1);
        pp;
    }
    wp = words;
    while wp < word_end {
        if if is_filter != 0 { (*wp).matched } else { ((*wp).matched == 0) as i32 } != 0
        {
            o = variable_buffer_output(o, (*wp).str_0, strlen((*wp).str_0));
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const i8,
                1 as i32 as size_t,
            );
            doneany = 1 as i32;
        }
        wp = wp.offset(1);
        wp;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    if hashing != 0 {
        hash_free(&mut a_word_table, 0 as i32);
    }
    free(patterns as *mut libc::c_void);
    free(words as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn func_strip(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut p: *const i8 = *argv.offset(0 as i32 as isize);
    let mut doneany: i32 = 0 as i32;
    while *p as i32 != '\0' as i32 {
        let mut i: i32 = 0 as i32;
        let mut word_start: *const i8 = 0 as *const i8;
        while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32
        {
            p = p.offset(1);
            p;
        }
        word_start = p;
        i = 0 as i32;
        while *p as i32 != '\0' as i32
            && !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
                & (0x2 as i32 | 0x4 as i32) != 0 as i32)
        {
            p = p.offset(1);
            p;
            i += 1;
            i;
        }
        if i == 0 {
            break;
        }
        o = variable_buffer_output(o, word_start, i as size_t);
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const i8,
            1 as i32 as size_t,
        );
        doneany = 1 as i32;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_error(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    match *funcname as i32 {
        101 => {
            fatal(
                reading_file,
                strlen(*argv.offset(0 as i32 as isize)),
                b"%s\0" as *const u8 as *const i8,
                *argv.offset(0 as i32 as isize),
            );
        }
        119 => {
            error(
                reading_file,
                strlen(*argv.offset(0 as i32 as isize)),
                b"%s\0" as *const u8 as *const i8,
                *argv.offset(0 as i32 as isize),
            );
        }
        105 => {
            let mut len: size_t = strlen(*argv.offset(0 as i32 as isize));
            let mut fresh2 = ::std::vec::from_elem(
                0,
                len.wrapping_add(2 as i32 as u64) as usize,
            );
            let mut msg: *mut i8 = fresh2.as_mut_ptr() as *mut i8;
            memcpy(
                msg as *mut libc::c_void,
                *argv.offset(0 as i32 as isize) as *const libc::c_void,
                len,
            );
            *msg.offset(len as isize) = '\n' as i32 as i8;
            *msg.offset(len.wrapping_add(1 as i32 as u64) as isize) = '\0' as i32 as i8;
            outputs(0 as i32, msg);
        }
        _ => {
            fatal(
                *expanding_var,
                strlen(funcname),
                b"Internal error: func_error: '%s'\0" as *const u8 as *const i8,
                funcname,
            );
        }
    }
    return o;
}
unsafe extern "C" fn func_sort(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut t: *const i8 = 0 as *const i8;
    let mut words: *mut *mut i8 = 0 as *mut *mut i8;
    let mut wordi: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    t = *argv.offset(0 as i32 as isize);
    wordi = 0 as i32;
    loop {
        p = find_next_token(&mut t, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        t = t.offset(1);
        t;
        wordi += 1;
        wordi;
    }
    words = xmalloc(
        ((if wordi == 0 as i32 { 1 as i32 } else { wordi }) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    t = *argv.offset(0 as i32 as isize);
    wordi = 0 as i32;
    loop {
        p = find_next_token(&mut t, &mut len);
        if p.is_null() {
            break;
        }
        t = t.offset(1);
        t;
        *p.offset(len as isize) = '\0' as i32 as i8;
        let fresh3 = wordi;
        wordi = wordi + 1;
        let ref mut fresh4 = *words.offset(fresh3 as isize);
        *fresh4 = p;
    }
    if wordi != 0 {
        let mut i: i32 = 0;
        qsort(
            words as *mut libc::c_void,
            wordi as size_t,
            ::core::mem::size_of::<*mut i8>() as u64,
            Some(
                alpha_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
        i = 0 as i32;
        while i < wordi {
            len = strlen(*words.offset(i as isize));
            if i == wordi - 1 as i32
                || strlen(*words.offset((i + 1 as i32) as isize)) != len
                || memcmp(
                    *words.offset(i as isize) as *const libc::c_void,
                    *words.offset((i + 1 as i32) as isize) as *const libc::c_void,
                    len,
                ) != 0
            {
                o = variable_buffer_output(o, *words.offset(i as isize), len);
                o = variable_buffer_output(
                    o,
                    b" \0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                );
            }
            i += 1;
            i;
        }
        o = o.offset(-1);
        o;
    }
    free(words as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn parse_textint(
    mut number: *const i8,
    mut msg: *const i8,
    mut sign: *mut i32,
    mut numstart: *mut *const i8,
) -> *const i8 {
    let mut after_sign: *const i8 = 0 as *const i8;
    let mut after_number: *const i8 = 0 as *const i8;
    let mut p: *const i8 = next_token(number);
    let mut negative: i32 = (*p as i32 == '-' as i32) as i32;
    let mut nonzero: i32 = 0;
    if *p as i32 == '\0' as i32 {
        fatal(
            *expanding_var,
            strlen(msg),
            dcgettext(
                0 as *const i8,
                b"%s: empty value\0" as *const u8 as *const i8,
                5 as i32,
            ),
            msg,
        );
    }
    p = p.offset((negative != 0 || *p as i32 == '+' as i32) as i32 as isize);
    after_sign = p;
    while *p as i32 == '0' as i32 {
        p = p.offset(1);
        p;
    }
    *numstart = p;
    while (*p as u32).wrapping_sub('0' as i32 as u32) <= 9 as i32 as u32 {
        p = p.offset(1);
        p;
    }
    after_number = p;
    nonzero = (*numstart != after_number) as i32;
    *sign = if negative != 0 { -nonzero } else { nonzero };
    if after_number == after_sign || *next_token(p) as i32 != '\0' as i32 {
        fatal(
            *expanding_var,
            (strlen(msg)).wrapping_add(strlen(number)),
            b"%s: '%s'\0" as *const u8 as *const i8,
            msg,
            number,
        );
    }
    return after_number;
}
unsafe extern "C" fn func_intcmp(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut lsign: i32 = 0;
    let mut rsign: i32 = 0;
    let mut lnum: *const i8 = 0 as *const i8;
    let mut rnum: *const i8 = 0 as *const i8;
    let mut lhs_str: *mut i8 = expand_argument(
        *argv.offset(0 as i32 as isize),
        0 as *const i8,
    );
    let mut rhs_str: *mut i8 = expand_argument(
        *argv.offset(1 as i32 as isize),
        0 as *const i8,
    );
    let mut llim: *const i8 = parse_textint(
        lhs_str,
        dcgettext(
            0 as *const i8,
            b"non-numeric first argument to 'intcmp' function\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        &mut lsign,
        &mut lnum,
    );
    let mut rlim: *const i8 = parse_textint(
        rhs_str,
        dcgettext(
            0 as *const i8,
            b"non-numeric second argument to 'intcmp' function\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        &mut rsign,
        &mut rnum,
    );
    let mut llen: ptrdiff_t = llim.offset_from(lnum) as i64;
    let mut rlen: ptrdiff_t = rlim.offset_from(rnum) as i64;
    let mut cmp: i32 = lsign - rsign;
    if cmp == 0 as i32 {
        cmp = (llen > rlen) as i32 - (llen < rlen) as i32;
        if cmp == 0 as i32 {
            cmp = memcmp(
                lnum as *const libc::c_void,
                rnum as *const libc::c_void,
                llen as u64,
            );
        }
    }
    argv = argv.offset(2 as i32 as isize);
    if (*argv).is_null() && cmp == 0 as i32 {
        if lsign < 0 as i32 {
            o = variable_buffer_output(
                o,
                b"-\0" as *const u8 as *const i8,
                1 as i32 as size_t,
            );
        }
        o = variable_buffer_output(
            o,
            lnum.offset(-((lsign == 0) as i32 as isize)),
            (llen + (lsign == 0) as i32 as i64) as size_t,
        );
    }
    free(lhs_str as *mut libc::c_void);
    free(rhs_str as *mut libc::c_void);
    if !(*argv).is_null() && cmp >= 0 as i32 {
        argv = argv.offset(1);
        argv;
        if cmp > 0 as i32 && !(*argv).is_null()
            && !(*argv.offset(1 as i32 as isize)).is_null()
        {
            argv = argv.offset(1);
            argv;
        }
    }
    if !(*argv).is_null() {
        let mut expansion: *mut i8 = expand_argument(*argv, 0 as *const i8);
        o = variable_buffer_output(o, expansion, strlen(expansion));
        free(expansion as *mut libc::c_void);
    }
    return o;
}
unsafe extern "C" fn func_if(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut begp: *const i8 = *argv.offset(0 as i32 as isize);
    let mut endp: *const i8 = begp
        .offset(strlen(*argv.offset(0 as i32 as isize)) as isize)
        .offset(-(1 as i32 as isize));
    let mut result: i32 = 0 as i32;
    strip_whitespace(&mut begp, &mut endp);
    if begp <= endp {
        let mut expansion: *mut i8 = expand_argument(
            begp,
            endp.offset(1 as i32 as isize),
        );
        result = (*expansion.offset(0 as i32 as isize) as i32 != '\0' as i32) as i32;
        free(expansion as *mut libc::c_void);
    }
    argv = argv.offset((1 as i32 + (result == 0) as i32) as isize);
    if !(*argv).is_null() {
        let mut expansion_0: *mut i8 = expand_argument(*argv, 0 as *const i8);
        o = variable_buffer_output(o, expansion_0, strlen(expansion_0));
        free(expansion_0 as *mut libc::c_void);
    }
    return o;
}
unsafe extern "C" fn func_or(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    while !(*argv).is_null() {
        let mut begp: *const i8 = *argv;
        let mut endp: *const i8 = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1 as i32 as isize));
        let mut expansion: *mut i8 = 0 as *mut i8;
        let mut result: size_t = 0 as i32 as size_t;
        strip_whitespace(&mut begp, &mut endp);
        if !(begp > endp) {
            expansion = expand_argument(begp, endp.offset(1 as i32 as isize));
            result = strlen(expansion);
            if result == 0 {
                free(expansion as *mut libc::c_void);
            } else {
                o = variable_buffer_output(o, expansion, result);
                free(expansion as *mut libc::c_void);
                break;
            }
        }
        argv = argv.offset(1);
        argv;
    }
    return o;
}
unsafe extern "C" fn func_and(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut expansion: *mut i8 = 0 as *mut i8;
    loop {
        let mut begp: *const i8 = *argv;
        let mut endp: *const i8 = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1 as i32 as isize));
        let mut result: size_t = 0;
        strip_whitespace(&mut begp, &mut endp);
        if begp > endp {
            return o;
        }
        expansion = expand_argument(begp, endp.offset(1 as i32 as isize));
        result = strlen(expansion);
        if result == 0 {
            break;
        }
        argv = argv.offset(1);
        if !(*argv).is_null() {
            free(expansion as *mut libc::c_void);
        } else {
            o = variable_buffer_output(o, expansion, result);
            break;
        }
    }
    free(expansion as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn func_wildcard(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut p: *mut i8 = string_glob(*argv.offset(0 as i32 as isize));
    o = variable_buffer_output(o, p, strlen(p));
    return o;
}
unsafe extern "C" fn func_eval(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    install_variable_buffer(&mut buf, &mut len);
    eval_buffer(*argv.offset(0 as i32 as isize), 0 as *const floc);
    restore_variable_buffer(buf, len);
    return o;
}
unsafe extern "C" fn func_value(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut v: *mut variable = lookup_variable(
        *argv.offset(0 as i32 as isize),
        strlen(*argv.offset(0 as i32 as isize)),
    );
    if !v.is_null() {
        o = variable_buffer_output(o, (*v).value, strlen((*v).value));
    }
    return o;
}
unsafe extern "C" fn fold_newlines(
    mut buffer: *mut i8,
    mut length: *mut size_t,
    mut trim_newlines: i32,
) {
    let mut dst: *mut i8 = buffer;
    let mut src: *mut i8 = buffer;
    let mut last_nonnl: *mut i8 = buffer.offset(-(1 as i32 as isize));
    *src.offset(*length as isize) = 0 as i32 as i8;
    while *src as i32 != '\0' as i32 {
        if !(*src.offset(0 as i32 as isize) as i32 == '\r' as i32
            && *src.offset(1 as i32 as isize) as i32 == '\n' as i32)
        {
            if *src as i32 == '\n' as i32 {
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = ' ' as i32 as i8;
            } else {
                last_nonnl = dst;
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = *src;
            }
        }
        src = src.offset(1);
        src;
    }
    if trim_newlines == 0 && last_nonnl < dst.offset(-(2 as i32 as isize)) {
        last_nonnl = dst.offset(-(2 as i32 as isize));
    }
    last_nonnl = last_nonnl.offset(1);
    *last_nonnl = '\0' as i32 as i8;
    *length = last_nonnl.offset_from(buffer) as i64 as size_t;
}
#[no_mangle]
pub static mut shell_function_pid: pid_t = 0 as i32;
static mut shell_function_completed: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn shell_completed(mut exit_code: i32, mut exit_sig: i32) {
    let mut buf: [i8; 22] = [0; 22];
    shell_function_pid = 0 as i32;
    if exit_sig == 0 as i32 && exit_code == 127 as i32 {
        shell_function_completed = -(1 as i32);
    } else {
        shell_function_completed = 1 as i32;
    }
    if exit_code == 0 as i32 && exit_sig > 0 as i32 {
        exit_code = 128 as i32 + exit_sig;
    }
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const i8, exit_code);
    define_variable_in_set(
        b".SHELLSTATUS\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 13]>() as u64).wrapping_sub(1 as i32 as u64),
        buf.as_mut_ptr(),
        variable_origin::o_override,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn func_shell_base(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut trim_newlines: i32,
) -> *mut i8 {
    let mut child: childbase = {
        let mut init = childbase {
            cmd_name: 0 as *mut i8,
            environment: 0 as *mut *mut i8,
            output: output {
                out: 0,
                err: 0,
                syncout: [0; 1],
                c2rust_padding: [0; 3],
            },
        };
        init
    };
    let mut batch_filename: *mut i8 = 0 as *mut i8;
    let mut errfd: i32 = 0;
    let mut command_argv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut pipedes: [i32; 2] = [0; 2];
    let mut pid: pid_t = 0;
    command_argv = construct_command_argv(
        *argv.offset(0 as i32 as isize),
        0 as *mut *mut i8,
        0 as *mut file,
        0 as i32,
        &mut batch_filename,
    );
    if command_argv.is_null() {
        return o;
    }
    output_start();
    errfd = if !output_context.is_null() && (*output_context).err >= 0 as i32 {
        (*output_context).err
    } else {
        fileno(stderr)
    };
    child.environment = target_environment(0 as *mut file, 0 as i32);
    if pipe(pipedes.as_mut_ptr()) < 0 as i32 {
        error(
            reading_file,
            strlen(strerror(*__errno_location())),
            b"pipe: %s\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
        pid = -(1 as i32);
    } else {
        fd_noinherit(pipedes[1 as i32 as usize]);
        fd_noinherit(pipedes[0 as i32 as usize]);
        (child.output).set_syncout(1 as i32 as u32);
        child.output.out = pipedes[1 as i32 as usize];
        child.output.err = errfd;
        pid = child_execute_job(&mut child, 1 as i32, command_argv);
        if pid < 0 as i32 {
            shell_completed(127 as i32, 0 as i32);
        } else {
            let mut buffer: *mut i8 = 0 as *mut i8;
            let mut maxlen: size_t = 0;
            let mut i: size_t = 0;
            let mut cc: i32 = 0;
            shell_function_pid = pid;
            shell_function_completed = 0 as i32;
            if pipedes[1 as i32 as usize] >= 0 as i32 {
                close(pipedes[1 as i32 as usize]);
            }
            maxlen = 200 as i32 as size_t;
            buffer = xmalloc(maxlen.wrapping_add(1 as i32 as u64)) as *mut i8;
            i = 0 as i32 as size_t;
            loop {
                if i == maxlen {
                    maxlen = (maxlen as u64).wrapping_add(512 as i32 as u64) as size_t
                        as size_t;
                    buffer = xrealloc(
                        buffer as *mut libc::c_void,
                        maxlen.wrapping_add(1 as i32 as u64),
                    ) as *mut i8;
                }
                loop {
                    cc = read(
                        pipedes[0 as i32 as usize],
                        &mut *buffer.offset(i as isize) as *mut i8 as *mut libc::c_void,
                        maxlen.wrapping_sub(i),
                    ) as i32;
                    if !(cc == -(1 as i32) && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if cc <= 0 as i32 {
                    break;
                }
                i = (i as u64).wrapping_add(cc as u64) as size_t as size_t;
            }
            *buffer.offset(i as isize) = '\0' as i32 as i8;
            close(pipedes[0 as i32 as usize]);
            while shell_function_completed == 0 as i32 {
                reap_children(1 as i32, 0 as i32);
            }
            if !batch_filename.is_null() {
                if 0x2 as i32 & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"Cleaning up temporary batch file %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        batch_filename,
                    );
                    fflush(stdout);
                }
                remove(batch_filename);
                free(batch_filename as *mut libc::c_void);
            }
            shell_function_pid = 0 as i32;
            fold_newlines(buffer, &mut i, trim_newlines);
            o = variable_buffer_output(o, buffer, i);
            free(buffer as *mut libc::c_void);
        }
    }
    if !command_argv.is_null() {
        free(*command_argv.offset(0 as i32 as isize) as *mut libc::c_void);
        free(command_argv as *mut libc::c_void);
    }
    free_childbase(&mut child);
    return o;
}
unsafe extern "C" fn func_shell(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    return func_shell_base(o, argv, 1 as i32);
}
unsafe extern "C" fn abspath(mut name: *const i8, mut apath: *mut i8) -> *mut i8 {
    let mut dest: *mut i8 = 0 as *mut i8;
    let mut start: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut apath_limit: *const i8 = 0 as *const i8;
    let mut root_len: u64 = 1 as i32 as u64;
    if *name.offset(0 as i32 as isize) as i32 == '\0' as i32 {
        return 0 as *mut i8;
    }
    apath_limit = apath.offset(4096 as i32 as isize);
    if !(*name.offset(0 as i32 as isize) as i32 == '/' as i32) {
        if starting_directory.is_null() {
            return 0 as *mut i8;
        }
        strcpy(apath, starting_directory);
        dest = strchr(apath, '\0' as i32);
    } else {
        memcpy(apath as *mut libc::c_void, name as *const libc::c_void, root_len);
        *apath.offset(root_len as isize) = '\0' as i32 as i8;
        dest = apath.offset(root_len as isize);
        name = name.offset(root_len as isize);
    }
    end = name;
    start = end;
    while *start as i32 != '\0' as i32 {
        let mut len: size_t = 0;
        while *stopchar_map.as_mut_ptr().offset(*start as u8 as isize) as i32
            & 0x8000 as i32 != 0 as i32
        {
            start = start.offset(1);
            start;
        }
        end = start;
        while !(*stopchar_map.as_mut_ptr().offset(*end as u8 as isize) as i32
            & (0x8000 as i32 | 0x1 as i32) != 0 as i32)
        {
            end = end.offset(1);
            end;
        }
        len = end.offset_from(start) as i64 as size_t;
        if len == 0 as i32 as u64 {
            break;
        }
        if !(len == 1 as i32 as u64
            && *start.offset(0 as i32 as isize) as i32 == '.' as i32)
        {
            if len == 2 as i32 as u64
                && *start.offset(0 as i32 as isize) as i32 == '.' as i32
                && *start.offset(1 as i32 as isize) as i32 == '.' as i32
            {
                if dest > apath.offset(root_len as isize) {
                    dest = dest.offset(-1);
                    dest;
                    while !(*stopchar_map
                        .as_mut_ptr()
                        .offset(*dest.offset(-(1 as i32) as isize) as u8 as isize) as i32
                        & 0x8000 as i32 != 0 as i32)
                    {
                        dest = dest.offset(-1);
                        dest;
                    }
                }
            } else {
                if !(*stopchar_map
                    .as_mut_ptr()
                    .offset(*dest.offset(-(1 as i32) as isize) as u8 as isize) as i32
                    & 0x8000 as i32 != 0 as i32)
                {
                    let fresh7 = dest;
                    dest = dest.offset(1);
                    *fresh7 = '/' as i32 as i8;
                }
                if dest.offset(len as isize) >= apath_limit as *mut i8 {
                    return 0 as *mut i8;
                }
                dest = mempcpy(
                    dest as *mut libc::c_void,
                    start as *const libc::c_void,
                    len,
                ) as *mut i8;
                *dest = '\0' as i32 as i8;
            }
        }
        start = end;
    }
    if dest > apath.offset(root_len as isize)
        && *stopchar_map
            .as_mut_ptr()
            .offset(*dest.offset(-(1 as i32) as isize) as u8 as isize) as i32
            & 0x8000 as i32 != 0 as i32
    {
        dest = dest.offset(-1);
        dest;
    }
    *dest = '\0' as i32 as i8;
    return apath;
}
unsafe extern "C" fn func_realpath(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut p: *const i8 = *argv.offset(0 as i32 as isize);
    let mut path: *const i8 = 0 as *const i8;
    let mut doneany: i32 = 0 as i32;
    let mut len: size_t = 0 as i32 as size_t;
    loop {
        path = find_next_token(&mut p, &mut len);
        if path.is_null() {
            break;
        }
        if len < 4096 as i32 as u64 {
            let mut rp: *mut i8 = 0 as *mut i8;
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
            let mut in_0: [i8; 4097] = [0; 4097];
            let mut out: [i8; 4097] = [0; 4097];
            strncpy(in_0.as_mut_ptr(), path, len);
            in_0[len as usize] = '\0' as i32 as i8;
            loop {
                *__errno_location() = 0 as i32;
                rp = realpath(in_0.as_mut_ptr(), out.as_mut_ptr());
                if !(rp.is_null() && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if !rp.is_null() {
                let mut r: i32 = 0;
                loop {
                    r = stat(out.as_mut_ptr(), &mut st);
                    if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if r == 0 as i32 {
                    o = variable_buffer_output(
                        o,
                        out.as_mut_ptr(),
                        strlen(out.as_mut_ptr()),
                    );
                    o = variable_buffer_output(
                        o,
                        b" \0" as *const u8 as *const i8,
                        1 as i32 as size_t,
                    );
                    doneany = 1 as i32;
                }
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_file(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut fn_0: *mut i8 = *argv.offset(0 as i32 as isize);
    if *fn_0.offset(0 as i32 as isize) as i32 == '>' as i32 {
        let mut len: size_t = 0;
        let mut end: *const i8 = 0 as *const i8;
        let mut start: *const i8 = 0 as *const i8;
        let mut nm: *mut i8 = 0 as *mut i8;
        let mut fp: *mut FILE = 0 as *mut FILE;
        let mut mode: *const i8 = b"w\0" as *const u8 as *const i8;
        fn_0 = fn_0.offset(1);
        fn_0;
        if *fn_0.offset(0 as i32 as isize) as i32 == '>' as i32 {
            mode = b"a\0" as *const u8 as *const i8;
            fn_0 = fn_0.offset(1);
            fn_0;
        }
        start = next_token(fn_0);
        if *start.offset(0 as i32 as isize) as i32 == '\0' as i32 {
            fatal(
                *expanding_var,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"file: missing filename\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        end = end_of_token(start);
        len = end.offset_from(start) as i64 as size_t;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            len.wrapping_add(1 as i32 as u64) as usize,
        );
        nm = fresh8.as_mut_ptr() as *mut i8;
        memcpy(nm as *mut libc::c_void, start as *const libc::c_void, len);
        *nm.offset(len as isize) = '\0' as i32 as i8;
        loop {
            *__errno_location() = 0 as i32;
            fp = fopen(nm, mode);
            if !(fp.is_null() && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if fp.is_null() {
            fatal(
                reading_file,
                (strlen(nm)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"open: %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                nm,
                strerror(*__errno_location()),
            );
        }
        command_count = command_count.wrapping_add(1);
        command_count;
        if !(*argv.offset(1 as i32 as isize)).is_null() {
            let mut l: size_t = strlen(*argv.offset(1 as i32 as isize));
            let mut nl: i32 = (l == 0 as i32 as u64
                || *(*argv.offset(1 as i32 as isize))
                    .offset(l.wrapping_sub(1 as i32 as u64) as isize) as i32
                    != '\n' as i32) as i32;
            if fputs(*argv.offset(1 as i32 as isize), fp) == -(1 as i32)
                || nl != 0 && fputc('\n' as i32, fp) == -(1 as i32)
            {
                fatal(
                    reading_file,
                    (strlen(nm)).wrapping_add(strlen(strerror(*__errno_location()))),
                    dcgettext(
                        0 as *const i8,
                        b"write: %s: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    nm,
                    strerror(*__errno_location()),
                );
            }
        }
        if fclose(fp) != 0 {
            fatal(
                reading_file,
                (strlen(nm)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"close: %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                nm,
                strerror(*__errno_location()),
            );
        }
    } else if *fn_0.offset(0 as i32 as isize) as i32 == '<' as i32 {
        let mut n: size_t = 0 as i32 as size_t;
        let mut len_0: size_t = 0;
        let mut end_0: *const i8 = 0 as *const i8;
        let mut start_0: *const i8 = 0 as *const i8;
        let mut nm_0: *mut i8 = 0 as *mut i8;
        let mut fp_0: *mut FILE = 0 as *mut FILE;
        start_0 = next_token(fn_0.offset(1 as i32 as isize));
        if *start_0.offset(0 as i32 as isize) as i32 == '\0' as i32 {
            fatal(
                *expanding_var,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"file: missing filename\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if !(*argv.offset(1 as i32 as isize)).is_null() {
            fatal(
                *expanding_var,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"file: too many arguments\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        end_0 = end_of_token(start_0);
        len_0 = end_0.offset_from(start_0) as i64 as size_t;
        let mut fresh9 = ::std::vec::from_elem(
            0,
            len_0.wrapping_add(1 as i32 as u64) as usize,
        );
        nm_0 = fresh9.as_mut_ptr() as *mut i8;
        memcpy(nm_0 as *mut libc::c_void, start_0 as *const libc::c_void, len_0);
        *nm_0.offset(len_0 as isize) = '\0' as i32 as i8;
        loop {
            *__errno_location() = 0 as i32;
            fp_0 = fopen(nm_0, b"r\0" as *const u8 as *const i8);
            if !(fp_0.is_null() && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if fp_0.is_null() {
            if *__errno_location() == 2 as i32 {
                if 0x2 as i32 & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const i8,
                            b"file: Failed to open '%s': %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        nm_0,
                        strerror(*__errno_location()),
                    );
                    fflush(stdout);
                }
                return o;
            }
            fatal(
                reading_file,
                (strlen(nm_0)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"open: %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                nm_0,
                strerror(*__errno_location()),
            );
        }
        loop {
            let mut buf: [i8; 1024] = [0; 1024];
            let mut l_0: size_t = fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as i32 as size_t,
                ::core::mem::size_of::<[i8; 1024]>() as u64,
                fp_0,
            );
            if l_0 > 0 as i32 as u64 {
                o = variable_buffer_output(o, buf.as_mut_ptr(), l_0);
                n = (n as u64).wrapping_add(l_0) as size_t as size_t;
            }
            if ferror(fp_0) != 0 {
                if *__errno_location() != 4 as i32 {
                    fatal(
                        reading_file,
                        (strlen(nm_0))
                            .wrapping_add(strlen(strerror(*__errno_location()))),
                        dcgettext(
                            0 as *const i8,
                            b"read: %s: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        nm_0,
                        strerror(*__errno_location()),
                    );
                }
            }
            if feof(fp_0) != 0 {
                break;
            }
        }
        if fclose(fp_0) != 0 {
            fatal(
                reading_file,
                (strlen(nm_0)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"close: %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                nm_0,
                strerror(*__errno_location()),
            );
        }
        if n != 0 && *o.offset(-(1 as i32) as isize) as i32 == '\n' as i32 {
            o = o
                .offset(
                    -((1 as i32
                        + (n > 1 as i32 as u64
                            && *o.offset(-(2 as i32) as isize) as i32 == '\r' as i32)
                            as i32) as isize),
                );
        }
    } else {
        fatal(
            *expanding_var,
            strlen(fn_0),
            dcgettext(
                0 as *const i8,
                b"file: invalid file operation: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fn_0,
        );
    }
    return o;
}
unsafe extern "C" fn func_abspath(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    let mut p: *const i8 = *argv.offset(0 as i32 as isize);
    let mut path: *const i8 = 0 as *const i8;
    let mut doneany: i32 = 0 as i32;
    let mut len: size_t = 0 as i32 as size_t;
    loop {
        path = find_next_token(&mut p, &mut len);
        if path.is_null() {
            break;
        }
        if len < 4096 as i32 as u64 {
            let mut in_0: [i8; 4097] = [0; 4097];
            let mut out: [i8; 4097] = [0; 4097];
            strncpy(in_0.as_mut_ptr(), path, len);
            in_0[len as usize] = '\0' as i32 as i8;
            if !(abspath(in_0.as_mut_ptr(), out.as_mut_ptr())).is_null() {
                o = variable_buffer_output(
                    o,
                    out.as_mut_ptr(),
                    strlen(out.as_mut_ptr()),
                );
                o = variable_buffer_output(
                    o,
                    b" \0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                );
                doneany = 1 as i32;
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
static mut function_table_init: [function_table_entry; 38] = [function_table_entry {
    fptr: C2RustUnnamed { func_ptr: None },
    name: 0 as *const i8,
    len: 0,
    minimum_args: 0,
    maximum_args: 0,
    expand_args_alloc_fn_adds_command: [0; 1],
    c2rust_padding: [0; 4],
}; 38];
unsafe extern "C" fn expand_builtin_function(
    mut o: *mut i8,
    mut argc: u32,
    mut argv: *mut *mut i8,
    mut entry_p: *const function_table_entry,
) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    if argc < (*entry_p).minimum_args as u32 {
        fatal(
            *expanding_var,
            strlen((*entry_p).name),
            dcgettext(
                0 as *const i8,
                b"insufficient number of arguments (%u) to function '%s'\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            argc,
            (*entry_p).name,
        );
    }
    if argc == 0 && (*entry_p).alloc_fn() == 0 {
        return o;
    }
    if ((*entry_p).fptr.func_ptr).is_none() {
        fatal(
            *expanding_var,
            strlen((*entry_p).name),
            dcgettext(
                0 as *const i8,
                b"unimplemented on this platform: function '%s'\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*entry_p).name,
        );
    }
    if (*entry_p).adds_command() != 0 {
        command_count = command_count.wrapping_add(1);
        command_count;
    }
    if (*entry_p).alloc_fn() == 0 {
        return ((*entry_p).fptr.func_ptr)
            .expect("non-null function pointer")(o, argv, (*entry_p).name);
    }
    p = ((*entry_p).fptr.alloc_func_ptr)
        .expect("non-null function pointer")((*entry_p).name, argc, argv);
    if !p.is_null() {
        o = variable_buffer_output(o, p, strlen(p));
        free(p as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn handle_function(
    mut op: *mut *mut i8,
    mut stringp: *mut *const i8,
) -> i32 {
    let mut entry_p: *const function_table_entry = 0 as *const function_table_entry;
    let mut openparen: i8 = *(*stringp).offset(0 as i32 as isize);
    let mut closeparen: i8 = (if openparen as i32 == '(' as i32 {
        ')' as i32
    } else {
        '}' as i32
    }) as i8;
    let mut beg: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut count: i32 = 0 as i32;
    let mut abeg: *mut i8 = 0 as *mut i8;
    let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut argvp: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nargs: u32 = 0;
    beg = (*stringp).offset(1 as i32 as isize);
    entry_p = lookup_function(beg);
    if entry_p.is_null() {
        return 0 as i32;
    }
    beg = beg.offset((*entry_p).len as i32 as isize);
    while *stopchar_map.as_mut_ptr().offset(*beg as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        beg = beg.offset(1);
        beg;
    }
    nargs = 1 as i32 as u32;
    end = beg;
    while *end as i32 != '\0' as i32 {
        if *stopchar_map.as_mut_ptr().offset(*end as u8 as isize) as i32
            & (0x80 as i32 | 0x400 as i32) != 0 as i32
        {
            if *end as i32 == ',' as i32 {
                nargs = nargs.wrapping_add(1);
                nargs;
            } else if *end as i32 == openparen as i32 {
                count += 1;
                count;
            } else if *end as i32 == closeparen as i32
                && {
                    count -= 1;
                    count < 0 as i32
                }
            {
                break;
            }
        }
        end = end.offset(1);
        end;
    }
    if count >= 0 as i32 {
        fatal(
            *expanding_var,
            strlen((*entry_p).name),
            dcgettext(
                0 as *const i8,
                b"unterminated call to function '%s': missing '%c'\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*entry_p).name,
            closeparen as i32,
        );
    }
    *stringp = end;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*mut i8>() as u64)
            .wrapping_mul(nargs.wrapping_add(2 as i32 as u32) as u64) as usize,
    );
    argv = fresh10.as_mut_ptr() as *mut *mut i8;
    argvp = argv;
    if (*entry_p).expand_args() != 0 {
        let mut p: *const i8 = 0 as *const i8;
        p = beg;
        nargs = 0 as i32 as u32;
        while p <= end {
            let mut next: *const i8 = 0 as *const i8;
            nargs = nargs.wrapping_add(1);
            nargs;
            if nargs == (*entry_p).maximum_args as u32
                || {
                    next = find_next_argument(openparen, closeparen, p, end);
                    next.is_null()
                }
            {
                next = end;
            }
            *argvp = expand_argument(p, next);
            p = next.offset(1 as i32 as isize);
            argvp = argvp.offset(1);
            argvp;
        }
    } else {
        let mut len: size_t = end.offset_from(beg) as i64 as size_t;
        let mut p_0: *mut i8 = 0 as *mut i8;
        let mut aend: *mut i8 = 0 as *mut i8;
        abeg = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        aend = mempcpy(abeg as *mut libc::c_void, beg as *const libc::c_void, len)
            as *mut i8;
        *aend = '\0' as i32 as i8;
        p_0 = abeg;
        nargs = 0 as i32 as u32;
        while p_0 <= aend {
            let mut next_0: *mut i8 = 0 as *mut i8;
            nargs = nargs.wrapping_add(1);
            nargs;
            if nargs == (*entry_p).maximum_args as u32
                || {
                    next_0 = find_next_argument(openparen, closeparen, p_0, aend);
                    next_0.is_null()
                }
            {
                next_0 = aend;
            }
            *argvp = p_0;
            *next_0 = '\0' as i32 as i8;
            p_0 = next_0.offset(1 as i32 as isize);
            argvp = argvp.offset(1);
            argvp;
        }
    }
    *argvp = 0 as *mut i8;
    *op = expand_builtin_function(*op, nargs, argv, entry_p);
    if (*entry_p).expand_args() != 0 {
        argvp = argv;
        while !(*argvp).is_null() {
            free(*argvp as *mut libc::c_void);
            argvp = argvp.offset(1);
            argvp;
        }
    } else {
        free(abeg as *mut libc::c_void);
    }
    return 1 as i32;
}
unsafe extern "C" fn func_call(
    mut o: *mut i8,
    mut argv: *mut *mut i8,
    mut funcname: *const i8,
) -> *mut i8 {
    static mut max_args: u32 = 0 as i32 as u32;
    let mut fname: *mut i8 = 0 as *mut i8;
    let mut body: *mut i8 = 0 as *mut i8;
    let mut flen: size_t = 0;
    let mut i: u32 = 0;
    let mut saved_args: i32 = 0;
    let mut entry_p: *const function_table_entry = 0 as *const function_table_entry;
    let mut v: *mut variable = 0 as *mut variable;
    fname = next_token(*argv.offset(0 as i32 as isize));
    *(end_of_token(fname)).offset(0 as i32 as isize) = '\0' as i32 as i8;
    if *fname as i32 == '\0' as i32 {
        return o;
    }
    entry_p = lookup_function(fname);
    if !entry_p.is_null() {
        i = 0 as i32 as u32;
        while !(*argv.offset(i.wrapping_add(1 as i32 as u32) as isize)).is_null() {
            i = i.wrapping_add(1);
            i;
        }
        return expand_builtin_function(o, i, argv.offset(1 as i32 as isize), entry_p);
    }
    flen = strlen(fname);
    v = lookup_variable(fname, flen);
    if v.is_null() {
        warn_undefined(fname, flen);
    }
    if v.is_null() || *(*v).value as i32 == '\0' as i32 {
        return o;
    }
    let mut fresh11 = ::std::vec::from_elem(
        0,
        flen.wrapping_add(4 as i32 as u64) as usize,
    );
    body = fresh11.as_mut_ptr() as *mut i8;
    *body.offset(0 as i32 as isize) = '$' as i32 as i8;
    *body.offset(1 as i32 as isize) = '(' as i32 as i8;
    memcpy(
        body.offset(2 as i32 as isize) as *mut libc::c_void,
        fname as *const libc::c_void,
        flen,
    );
    *body.offset(flen.wrapping_add(2 as i32 as u64) as isize) = ')' as i32 as i8;
    *body.offset(flen.wrapping_add(3 as i32 as u64) as isize) = '\0' as i32 as i8;
    push_new_variable_scope();
    i = 0 as i32 as u32;
    while !(*argv).is_null() {
        let mut num: [i8; 22] = [0; 22];
        sprintf(num.as_mut_ptr(), b"%u\0" as *const u8 as *const i8, i);
        define_variable_in_set(
            num.as_mut_ptr(),
            strlen(num.as_mut_ptr()),
            *argv,
            variable_origin::o_automatic,
            0 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        i = i.wrapping_add(1);
        i;
        argv = argv.offset(1);
        argv;
    }
    while i < max_args {
        let mut num_0: [i8; 22] = [0; 22];
        sprintf(num_0.as_mut_ptr(), b"%u\0" as *const u8 as *const i8, i);
        define_variable_in_set(
            num_0.as_mut_ptr(),
            strlen(num_0.as_mut_ptr()),
            b"\0" as *const u8 as *const i8,
            variable_origin::o_automatic,
            0 as i32,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        i = i.wrapping_add(1);
        i;
    }
    (*v).set_exp_count((((1 as i32) << 15 as i32) - 1 as i32) as u32);
    saved_args = max_args as i32;
    max_args = i;
    o = variable_expand_string(o, body, flen.wrapping_add(3 as i32 as u64));
    max_args = saved_args as u32;
    (*v).set_exp_count(0 as i32 as u32);
    pop_variable_scope();
    return o.offset(strlen(o) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn define_new_function(
    mut flocp: *const floc,
    mut name: *const i8,
    mut min: u32,
    mut max: u32,
    mut flags: u32,
    mut func: gmk_func_ptr,
) {
    let mut e: *const i8 = name;
    let mut ent: *mut function_table_entry = 0 as *mut function_table_entry;
    let mut len: size_t = 0;
    while *stopchar_map.as_mut_ptr().offset(*e as u8 as isize) as i32 & 0x2000 as i32
        != 0 as i32
    {
        e = e.offset(1);
        e;
    }
    len = e.offset_from(name) as i64 as size_t;
    if len == 0 as i32 as u64 {
        fatal(
            flocp,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"Empty function name\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if *name as i32 == '.' as i32 || *e as i32 != '\0' as i32 {
        fatal(
            flocp,
            strlen(name),
            dcgettext(
                0 as *const i8,
                b"Invalid function name: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    }
    if len > 255 as i32 as u64 {
        fatal(
            flocp,
            strlen(name),
            dcgettext(
                0 as *const i8,
                b"Function name too long: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    }
    if min > 255 as i32 as u32 {
        fatal(
            flocp,
            (53 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_div(22 as i32 as u64)
                .wrapping_add(3 as i32 as u64)
                .wrapping_add(strlen(name)),
            dcgettext(
                0 as *const i8,
                b"Invalid minimum argument count (%u) for function %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            min,
            name,
        );
    }
    if max > 255 as i32 as u32 || max != 0 && max < min {
        fatal(
            flocp,
            (53 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_div(22 as i32 as u64)
                .wrapping_add(3 as i32 as u64)
                .wrapping_add(strlen(name)),
            dcgettext(
                0 as *const i8,
                b"Invalid maximum argument count (%u) for function %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            max,
            name,
        );
    }
    ent = xmalloc(::core::mem::size_of::<function_table_entry>() as u64)
        as *mut function_table_entry;
    (*ent).name = strcache_add(name);
    (*ent).len = len as u8;
    (*ent).minimum_args = min as u8;
    (*ent).maximum_args = max as u8;
    (*ent)
        .set_expand_args(
            (if flags & 0x1 as i32 as u32 != 0 as i32 as u32 {
                0 as i32
            } else {
                1 as i32
            }) as u32,
        );
    (*ent).set_alloc_fn(1 as i32 as u32);
    (*ent).set_adds_command(1 as i32 as u32);
    (*ent).fptr.alloc_func_ptr = func;
    ent = hash_insert(&mut function_table, ent as *const libc::c_void)
        as *mut function_table_entry;
    free(ent as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn hash_init_function_table() {
    hash_init(
        &mut function_table,
        (::core::mem::size_of::<[function_table_entry; 38]>() as u64)
            .wrapping_div(::core::mem::size_of::<function_table_entry>() as u64)
            .wrapping_mul(2 as i32 as u64),
        Some(
            function_table_entry_hash_1
                as unsafe extern "C" fn(*const libc::c_void) -> u64,
        ),
        Some(
            function_table_entry_hash_2
                as unsafe extern "C" fn(*const libc::c_void) -> u64,
        ),
        Some(
            function_table_entry_hash_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    hash_load(
        &mut function_table,
        function_table_init.as_mut_ptr() as *mut libc::c_void,
        (::core::mem::size_of::<[function_table_entry; 38]>() as u64)
            .wrapping_div(::core::mem::size_of::<function_table_entry>() as u64),
        ::core::mem::size_of::<function_table_entry>() as u64,
    );
}
unsafe extern "C" fn run_static_initializers() {
    function_table_init = [
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_abspath
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"abspath\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_addsuffix_addprefix
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"addprefix\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 10]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_addsuffix_addprefix
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"addsuffix\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 10]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_basename_dir
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"basename\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_basename_dir
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"dir\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_notdir_suffix
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"notdir\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_subst
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"subst\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 3 as i32 as u8,
                maximum_args: 3 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_notdir_suffix
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"suffix\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_filter_filterout
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"filter\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_filter_filterout
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"filter-out\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 11]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_findstring
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"findstring\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 11]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_firstword
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"firstword\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 10]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_flavor
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"flavor\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_join
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"join\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_lastword
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"lastword\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_patsubst
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"patsubst\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 3 as i32 as u8,
                maximum_args: 3 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_realpath
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"realpath\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_shell
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"shell\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_sort
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"sort\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_strip
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"strip\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_wildcard
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"wildcard\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_word
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"word\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_wordlist
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"wordlist\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 3 as i32 as u8,
                maximum_args: 3 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_words
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"words\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_origin
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"origin\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_foreach
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"foreach\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 3 as i32 as u8,
                maximum_args: 3 as i32 as u8,
            };
            init.set_expand_args(0 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_let
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"let\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 3 as i32 as u8,
                maximum_args: 3 as i32 as u8,
            };
            init.set_expand_args(0 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_call
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"call\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 1 as i32 as u8,
                maximum_args: 0 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_error
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"info\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_error
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"error\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_error
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"warning\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_intcmp
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"intcmp\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 5 as i32 as u8,
            };
            init.set_expand_args(0 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_if
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"if\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 3]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 2 as i32 as u8,
                maximum_args: 3 as i32 as u8,
            };
            init.set_expand_args(0 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_or
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"or\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 3]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 1 as i32 as u8,
                maximum_args: 0 as i32 as u8,
            };
            init.set_expand_args(0 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_and
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"and\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 1 as i32 as u8,
                maximum_args: 0 as i32 as u8,
            };
            init.set_expand_args(0 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_value
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"value\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_eval
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"eval\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 0 as i32 as u8,
                maximum_args: 1 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_file
                            as unsafe extern "C" fn(
                                *mut i8,
                                *mut *mut i8,
                                *const i8,
                            ) -> *mut i8,
                    ),
                },
                name: b"file\0" as *const u8 as *const i8,
                len: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as u8,
                minimum_args: 1 as i32 as u8,
                maximum_args: 2 as i32 as u8,
            };
            init.set_expand_args(1 as i32 as u32);
            init.set_alloc_fn(0 as i32 as u32);
            init.set_adds_command(0 as i32 as u32);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dirent;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    static mut cmd_prefix: i8;
    static mut one_shell: i32;
    static mut second_expansion: i32;
    static mut posix_pedantic: i32;
    static mut stopchar_map: [libc::c_ushort; 0];
    fn load_file(flocp: *const floc, file: *mut file, noerror: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn getlogin() -> *mut i8;
    fn fflush(__stream: *mut FILE) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn glob(
        __pattern: *const i8,
        __flags: i32,
        __errfunc: Option<unsafe extern "C" fn(*const i8, i32) -> i32>,
        __pglob: *mut glob_t,
    ) -> i32;
    fn globfree(__pglob: *mut glob_t);
    fn strip_whitespace(begpp: *mut *const i8, endpp: *mut *const i8) -> *mut i8;
    static mut warn_undefined_variables_flag: i32;
    fn concat(_: u32, _: ...) -> *const i8;
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn out_of_memory() -> !;
    fn strcache_add_len(str: *const i8, len: size_t) -> *const i8;
    fn pfatal_with_name(_: *const i8) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn xstrndup(_: *const i8, _: size_t) -> *mut i8;
    fn find_next_token(_: *mut *const i8, _: *mut size_t) -> *mut i8;
    fn next_token(_: *const i8) -> *mut i8;
    fn end_of_token(_: *const i8) -> *mut i8;
    fn collapse_continuations(_: *mut i8);
    fn construct_vpath_list(pattern: *mut i8, dirpath: *mut i8);
    fn strcache_add(str: *const i8) -> *const i8;
    fn ar_name(_: *const i8) -> i32;
    fn ar_parse_name(_: *const i8, _: *mut *mut i8, _: *mut *mut i8);
    fn file_exists_p(_: *const i8) -> i32;
    fn dir_setup_glob(_: *mut glob_t);
    fn ferror(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn puts(__s: *const i8) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    static mut default_file: *mut file;
    fn lookup_file(name: *const i8) -> *mut file;
    fn enter_file(name: *const i8) -> *mut file;
    fn split_prereqs(prereqstr: *mut i8) -> *mut dep;
    fn enter_prereqs(prereqs: *mut dep, stem: *const i8) -> *mut dep;
    static mut snapped_deps: i32;
    fn ar_glob(
        arname: *const i8,
        member_pattern: *const i8,
        size: size_t,
    ) -> *mut nameseq;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn fd_noinherit(_: i32);
    static mut variable_buffer: *mut i8;
    static mut current_variable_set_list: *mut variable_set_list;
    static mut default_goal_var: *mut variable;
    fn variable_buffer_output(
        ptr: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn variable_expand(line: *const i8) -> *mut i8;
    fn allocated_variable_expand_for_file(line: *const i8, file: *mut file) -> *mut i8;
    fn variable_expand_string(
        line: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn pattern_matches(pattern: *const i8, percent: *const i8, str: *const i8) -> i32;
    fn patsubst_expand_pat(
        o: *mut i8,
        text: *const i8,
        pattern: *const i8,
        replace: *const i8,
        pattern_percent: *const i8,
        replace_percent: *const i8,
    ) -> *mut i8;
    fn initialize_file_variables(file: *mut file, reading: i32);
    fn do_variable_definition(
        flocp: *const floc,
        name: *const i8,
        value: *const i8,
        origin: variable_origin,
        flavor: variable_flavor,
        target_var: i32,
    ) -> *mut variable;
    fn parse_variable_definition(line: *const i8, v: *mut variable) -> *mut i8;
    fn assign_variable_definition(v: *mut variable, line: *const i8) -> *mut variable;
    fn try_variable_definition(
        flocp: *const floc,
        line: *const i8,
        origin: variable_origin,
        target_var: i32,
    ) -> *mut variable;
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
    fn undefine_variable_in_set(
        name: *const i8,
        length: size_t,
        origin: variable_origin,
        set: *mut variable_set,
    );
    fn create_pattern_var(target: *const i8, suffix: *const i8) -> *mut pattern_var;
    static mut export_all_variables: i32;
    static mut suffix_file: *mut file;
    fn create_pattern_rule(
        targets: *mut *const i8,
        target_percents: *mut *const i8,
        num: libc::c_ushort,
        terminal: i32,
        deps: *mut dep,
        commands: *mut commands,
        override_0: i32,
    );
    static mut db_level: i32;
    fn getpwnam(__name: *const i8) -> *mut passwd;
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
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
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
pub type uintmax_t = __uintmax_t;
pub type __size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut i8,
    pub gl_offs: __size_t,
    pub gl_flags: i32,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const i8) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
    pub gl_stat: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32>,
}
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
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct goaldep {
    pub next: *mut goaldep,
    pub name: *const i8,
    pub file: *mut file,
    pub shuf: *mut goaldep,
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
    pub c2rust_padding: [u8; 2],
    pub error: i32,
    pub floc: floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ebuffer {
    pub buffer: *mut i8,
    pub bufnext: *mut i8,
    pub bufstart: *mut i8,
    pub size: size_t,
    pub fp: *mut FILE,
    pub floc: floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conditionals {
    pub if_cmds: u32,
    pub allocated: u32,
    pub ignoring: *mut i8,
    pub seen_else: *mut i8,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct vmodifiers {
    #[bitfield(name = "assign_v", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "define_v", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "undefine_v", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "override_v", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "private_v", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "export_v", ty = "variable_export", bits = "5..=6")]
    pub assign_v_define_v_undefine_v_override_v_private_v_export_v: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum make_word_type {
    w_bogus,
    w_eol,
    w_static,
    w_variable,
    w_colon,
    w_dcolon,
    w_semicolon,
    w_varassign,
    w_ampcolon,
    w_ampdcolon,
}
impl make_word_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            make_word_type::w_bogus => 0,
            make_word_type::w_eol => 1,
            make_word_type::w_static => 2,
            make_word_type::w_variable => 3,
            make_word_type::w_colon => 4,
            make_word_type::w_dcolon => 5,
            make_word_type::w_semicolon => 6,
            make_word_type::w_varassign => 7,
            make_word_type::w_ampcolon => 8,
            make_word_type::w_ampdcolon => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> make_word_type {
        match value {
            0 => make_word_type::w_bogus,
            1 => make_word_type::w_eol,
            2 => make_word_type::w_static,
            3 => make_word_type::w_variable,
            4 => make_word_type::w_colon,
            5 => make_word_type::w_dcolon,
            6 => make_word_type::w_semicolon,
            7 => make_word_type::w_varassign,
            8 => make_word_type::w_ampcolon,
            9 => make_word_type::w_ampdcolon,
            _ => panic!("Invalid value for make_word_type: {}", value),
        }
    }
}
impl AddAssign<u32> for make_word_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = make_word_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for make_word_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = make_word_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for make_word_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = make_word_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for make_word_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = make_word_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for make_word_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = make_word_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for make_word_type {
    type Output = make_word_type;
    fn add(self, rhs: u32) -> make_word_type {
        make_word_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for make_word_type {
    type Output = make_word_type;
    fn sub(self, rhs: u32) -> make_word_type {
        make_word_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for make_word_type {
    type Output = make_word_type;
    fn mul(self, rhs: u32) -> make_word_type {
        make_word_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for make_word_type {
    type Output = make_word_type;
    fn div(self, rhs: u32) -> make_word_type {
        make_word_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for make_word_type {
    type Output = make_word_type;
    fn rem(self, rhs: u32) -> make_word_type {
        make_word_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    c_ifneq = 3,
    c_endif = 5,
    c_else = 4,
    c_ifeq = 2,
    c_ifndef = 1,
    c_ifdef = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::c_ifneq => 3,
            C2RustUnnamed::c_endif => 5,
            C2RustUnnamed::c_else => 4,
            C2RustUnnamed::c_ifeq => 2,
            C2RustUnnamed::c_ifndef => 1,
            C2RustUnnamed::c_ifdef => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            3 => C2RustUnnamed::c_ifneq,
            5 => C2RustUnnamed::c_endif,
            4 => C2RustUnnamed::c_else,
            2 => C2RustUnnamed::c_ifeq,
            1 => C2RustUnnamed::c_ifndef,
            0 => C2RustUnnamed::c_ifdef,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
static mut toplevel_conditionals: conditionals = conditionals {
    if_cmds: 0,
    allocated: 0,
    ignoring: 0 as *const i8 as *mut i8,
    seen_else: 0 as *const i8 as *mut i8,
};
static mut conditionals: *mut conditionals = unsafe {
    &toplevel_conditionals as *const conditionals as *mut conditionals
};
static mut default_include_directories: [*const i8; 4] = [
    b"/usr/gnu/include\0" as *const u8 as *const i8,
    b"/usr/local/include\0" as *const u8 as *const i8,
    b"/usr/include\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut include_directories: *mut *const i8 = 0 as *const *const i8 as *mut *const i8;
static mut max_incl_len: size_t = 0;
#[no_mangle]
pub static mut reading_file: *const floc = 0 as *const floc;
static mut read_files: *mut goaldep = 0 as *const goaldep as *mut goaldep;
#[no_mangle]
pub unsafe extern "C" fn read_all_makefiles(
    mut makefiles: *mut *const i8,
) -> *mut goaldep {
    let mut num_makefiles: u32 = 0 as i32 as u32;
    define_variable_in_set(
        b"MAKEFILE_LIST\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 14]>() as u64).wrapping_sub(1 as i32 as u64),
        b"\0" as *const u8 as *const i8,
        variable_origin::o_file,
        0 as i32,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if 0x1 as i32 & db_level != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"Reading makefiles...\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fflush(stdout);
    }
    let mut value: *mut i8 = 0 as *mut i8;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut length: size_t = 0;
    value = allocated_variable_expand_for_file(
        b"$(MAKEFILES)\0" as *const u8 as *const i8,
        0 as *mut file,
    );
    p = value;
    loop {
        name = find_next_token(&mut p as *mut *mut i8 as *mut *const i8, &mut length);
        if name.is_null() {
            break;
        }
        if *p as i32 != '\0' as i32 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\0' as i32 as i8;
        }
        eval_makefile(
            strcache_add(name),
            ((1 as i32) << 0 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32)
                as libc::c_ushort,
        );
    }
    free(value as *mut libc::c_void);
    if !makefiles.is_null() {
        while !(*makefiles).is_null() {
            let mut d: *mut goaldep = eval_makefile(
                *makefiles,
                0 as i32 as libc::c_ushort,
            );
            if *__errno_location() != 0 {
                perror_with_name(b"\0" as *const u8 as *const i8, *makefiles);
            }
            *makefiles = if !((*d).name).is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            num_makefiles = num_makefiles.wrapping_add(1);
            num_makefiles;
            makefiles = makefiles.offset(1);
            makefiles;
        }
    }
    if num_makefiles == 0 as i32 as u32 {
        static mut default_makefiles: [*const i8; 4] = [
            b"GNUmakefile\0" as *const u8 as *const i8,
            b"makefile\0" as *const u8 as *const i8,
            b"Makefile\0" as *const u8 as *const i8,
            0 as *const i8,
        ];
        let mut p_0: *mut *const i8 = default_makefiles.as_mut_ptr();
        while !(*p_0).is_null() && file_exists_p(*p_0) == 0 {
            p_0 = p_0.offset(1);
            p_0;
        }
        if !(*p_0).is_null() {
            eval_makefile(*p_0, 0 as i32 as libc::c_ushort);
            if *__errno_location() != 0 {
                perror_with_name(b"\0" as *const u8 as *const i8, *p_0);
            }
        } else {
            p_0 = default_makefiles.as_mut_ptr();
            while !(*p_0).is_null() {
                let mut d_0: *mut goaldep = xcalloc(
                    ::core::mem::size_of::<goaldep>() as u64,
                ) as *mut goaldep;
                (*d_0).file = enter_file(strcache_add(*p_0));
                (*d_0).set_flags(((1 as i32) << 2 as i32) as u32);
                (*d_0).next = read_files;
                read_files = d_0;
                p_0 = p_0.offset(1);
                p_0;
            }
        }
    }
    return read_files;
}
unsafe extern "C" fn install_conditionals(
    mut new: *mut conditionals,
) -> *mut conditionals {
    let mut save: *mut conditionals = conditionals;
    memset(
        new as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<conditionals>() as u64,
    );
    conditionals = new;
    return save;
}
unsafe extern "C" fn restore_conditionals(mut saved: *mut conditionals) {
    free((*conditionals).ignoring as *mut libc::c_void);
    free((*conditionals).seen_else as *mut libc::c_void);
    conditionals = saved;
}
unsafe extern "C" fn eval_makefile(
    mut filename: *const i8,
    mut flags: libc::c_ushort,
) -> *mut goaldep {
    let mut deps: *mut goaldep = 0 as *mut goaldep;
    let mut ebuf: ebuffer = ebuffer {
        buffer: 0 as *mut i8,
        bufnext: 0 as *mut i8,
        bufstart: 0 as *mut i8,
        size: 0,
        fp: 0 as *mut FILE,
        floc: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
    };
    let mut curfile: *const floc = 0 as *const floc;
    let mut expanded: *mut i8 = 0 as *mut i8;
    deps = xcalloc(::core::mem::size_of::<goaldep>() as u64) as *mut goaldep;
    (*deps).next = read_files;
    read_files = deps;
    ebuf.floc.filenm = filename;
    ebuf.floc.lineno = 1 as i32 as u64;
    ebuf.floc.offset = 0 as i32 as u64;
    if 0x2 as i32 & db_level != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"Reading makefile '%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
        );
        if flags as i32 & (1 as i32) << 0 as i32 != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b" (no default goal)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if flags as i32 & (1 as i32) << 1 as i32 != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b" (search path)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if flags as i32 & (1 as i32) << 2 as i32 != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b" (don't care)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if flags as i32 & (1 as i32) << 3 as i32 != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b" (no ~ expansion)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        puts(b"...\0" as *const u8 as *const i8);
    }
    if flags as i32 & (1 as i32) << 3 as i32 == 0
        && *filename.offset(0 as i32 as isize) as i32 == '~' as i32
    {
        expanded = tilde_expand(filename);
        if !expanded.is_null() {
            filename = expanded;
        }
    }
    *__errno_location() = 0 as i32;
    loop {
        *__errno_location() = 0 as i32;
        ebuf.fp = fopen(filename, b"r\0" as *const u8 as *const i8);
        if !((ebuf.fp).is_null() && *__errno_location() == 4 as i32) {
            break;
        }
    }
    (*deps).error = *__errno_location();
    match (*deps).error {
        24 | 23 | 12 => {
            let mut err: *const i8 = strerror((*deps).error);
            fatal(reading_file, strlen(err), b"%s\0" as *const u8 as *const i8, err);
        }
        _ => {}
    }
    if (ebuf.fp).is_null() && (*deps).error == 2 as i32 && !include_directories.is_null()
        && flags as i32 & (1 as i32) << 1 as i32 != 0 as i32 && 0 as i32 == 0
        && !(*stopchar_map.as_mut_ptr().offset(*filename as u8 as isize) as i32
            & 0x8000 as i32 != 0 as i32)
    {
        let mut dir: *mut *const i8 = 0 as *mut *const i8;
        dir = include_directories;
        while !(*dir).is_null() {
            let mut included: *const i8 = concat(
                3 as i32 as u32,
                *dir,
                b"/\0" as *const u8 as *const i8,
                filename,
            );
            loop {
                *__errno_location() = 0 as i32;
                ebuf.fp = fopen(included, b"r\0" as *const u8 as *const i8);
                if !((ebuf.fp).is_null() && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if !(ebuf.fp).is_null() {
                filename = included;
                break;
            } else if *__errno_location() != 2 as i32 {
                filename = included;
                (*deps).error = *__errno_location();
                break;
            } else {
                dir = dir.offset(1);
                dir;
            }
        }
    }
    filename = strcache_add(filename);
    (*deps).file = lookup_file(filename);
    if ((*deps).file).is_null() {
        (*deps).file = enter_file(filename);
    }
    filename = (*(*deps).file).name;
    (*deps).set_flags(flags as u32);
    (*(*deps).file).set_is_explicit(1 as i32 as u32);
    free(expanded as *mut libc::c_void);
    if (ebuf.fp).is_null() {
        *__errno_location() = (*deps).error;
        (*(*deps).file).last_mtime = 1 as i32 as uintmax_t;
        return deps;
    }
    (*deps).error = 0 as i32;
    if (*(*deps).file).last_mtime == 1 as i32 as u64 {
        (*(*deps).file).last_mtime = 0 as i32 as uintmax_t;
    }
    fd_noinherit(fileno(ebuf.fp));
    do_variable_definition(
        &mut ebuf.floc,
        b"MAKEFILE_LIST\0" as *const u8 as *const i8,
        filename,
        variable_origin::o_file,
        variable_flavor::f_append_value,
        0 as i32,
    );
    ebuf.size = 200 as i32 as size_t;
    ebuf.bufstart = xmalloc(ebuf.size) as *mut i8;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    curfile = reading_file;
    reading_file = &mut ebuf.floc;
    eval(&mut ebuf, (flags as i32 & (1 as i32) << 0 as i32 == 0) as i32);
    reading_file = curfile;
    fclose(ebuf.fp);
    free(ebuf.bufstart as *mut libc::c_void);
    *__errno_location() = 0 as i32;
    return deps;
}
#[no_mangle]
pub unsafe extern "C" fn eval_buffer(mut buffer: *mut i8, mut flocp: *const floc) {
    let mut ebuf: ebuffer = ebuffer {
        buffer: 0 as *mut i8,
        bufnext: 0 as *mut i8,
        bufstart: 0 as *mut i8,
        size: 0,
        fp: 0 as *mut FILE,
        floc: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
    };
    let mut saved: *mut conditionals = 0 as *mut conditionals;
    let mut new: conditionals = conditionals {
        if_cmds: 0,
        allocated: 0,
        ignoring: 0 as *const i8 as *mut i8,
        seen_else: 0 as *const i8 as *mut i8,
    };
    let mut curfile: *const floc = 0 as *const floc;
    ebuf.size = strlen(buffer);
    ebuf.bufstart = buffer;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    ebuf.fp = 0 as *mut FILE;
    if !flocp.is_null() {
        ebuf.floc = *flocp;
    } else if !reading_file.is_null() {
        ebuf.floc = *reading_file;
    } else {
        ebuf.floc.filenm = 0 as *const i8;
        ebuf.floc.lineno = 1 as i32 as u64;
        ebuf.floc.offset = 0 as i32 as u64;
    }
    curfile = reading_file;
    reading_file = &mut ebuf.floc;
    saved = install_conditionals(&mut new);
    eval(&mut ebuf, 1 as i32);
    restore_conditionals(saved);
    reading_file = curfile;
}
unsafe extern "C" fn parse_var_assignment(
    mut line: *const i8,
    mut targvar: i32,
    mut vmod: *mut vmodifiers,
) -> *mut i8 {
    let mut p: *const i8 = 0 as *const i8;
    memset(
        vmod as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<vmodifiers>() as u64,
    );
    while *stopchar_map.as_mut_ptr().offset(*line as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        line = line.offset(1);
        line;
    }
    if *line as i32 == '\0' as i32 {
        return line as *mut i8;
    }
    p = line;
    loop {
        let mut wlen: size_t = 0;
        let mut p2: *const i8 = 0 as *const i8;
        let mut v: variable = variable {
            name: 0 as *mut i8,
            value: 0 as *mut i8,
            fileinfo: floc {
                filenm: 0 as *const i8,
                lineno: 0,
                offset: 0,
            },
            length: 0,
            recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
        };
        p2 = parse_variable_definition(p, &mut v);
        if !p2.is_null() {
            break;
        }
        p2 = end_of_token(p);
        wlen = p2.offset_from(p) as i64 as size_t;
        if wlen
            == (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                b"export\0" as *const u8 as *const i8 as *const libc::c_void,
                p as *const libc::c_void,
                (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0 as i32
        {
            (*vmod).set_export_v(variable_export::v_export);
        } else if wlen
            == (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                b"unexport\0" as *const u8 as *const i8 as *const libc::c_void,
                p as *const libc::c_void,
                (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0 as i32
        {
            (*vmod).set_export_v(variable_export::v_noexport);
        } else if wlen
            == (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                b"override\0" as *const u8 as *const i8 as *const libc::c_void,
                p as *const libc::c_void,
                (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0 as i32
        {
            (*vmod).set_override_v(1 as i32 as u32);
        } else if wlen
            == (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                b"private\0" as *const u8 as *const i8 as *const libc::c_void,
                p as *const libc::c_void,
                (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0 as i32
        {
            (*vmod).set_private_v(1 as i32 as u32);
        } else if targvar == 0
            && (wlen
                == (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                && memcmp(
                    b"define\0" as *const u8 as *const i8 as *const libc::c_void,
                    p as *const libc::c_void,
                    (::core::mem::size_of::<[i8; 7]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                ) == 0 as i32)
        {
            (*vmod).set_define_v(1 as i32 as u32);
            p = next_token(p2);
            break;
        } else if targvar == 0
            && (wlen
                == (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                && memcmp(
                    b"undefine\0" as *const u8 as *const i8 as *const libc::c_void,
                    p as *const libc::c_void,
                    (::core::mem::size_of::<[i8; 9]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                ) == 0 as i32)
        {
            (*vmod).set_undefine_v(1 as i32 as u32);
            p = next_token(p2);
            break;
        } else {
            return line as *mut i8
        }
        p = next_token(p2);
        if *p as i32 == '\0' as i32 {
            return line as *mut i8;
        }
    }
    (*vmod).set_assign_v(1 as i32 as u32);
    return p as *mut i8;
}
unsafe extern "C" fn eval(mut ebuf: *mut ebuffer, mut set_default: i32) {
    let mut collapsed: *mut i8 = 0 as *mut i8;
    let mut collapsed_length: size_t = 0 as i32 as size_t;
    let mut commands_len: size_t = 200 as i32 as size_t;
    let mut commands: *mut i8 = 0 as *mut i8;
    let mut commands_idx: size_t = 0 as i32 as size_t;
    let mut cmds_started: u32 = 0;
    let mut tgts_started: u32 = 0;
    let mut ignoring: i32 = 0 as i32;
    let mut in_ignored_define: i32 = 0 as i32;
    let mut no_targets: i32 = 0 as i32;
    let mut also_make_targets: i32 = 0 as i32;
    let mut filenames: *mut nameseq = 0 as *mut nameseq;
    let mut depstr: *mut i8 = 0 as *mut i8;
    let mut nlines: i64 = 0 as i32 as i64;
    let mut two_colon: i32 = 0 as i32;
    let mut prefix: i8 = cmd_prefix;
    let mut pattern: *const i8 = 0 as *const i8;
    let mut pattern_percent: *const i8 = 0 as *const i8;
    let mut fstart: *mut floc = 0 as *mut floc;
    let mut fi: floc = floc {
        filenm: 0 as *const i8,
        lineno: 0,
        offset: 0,
    };
    pattern_percent = 0 as *const i8;
    tgts_started = 1 as i32 as u32;
    cmds_started = tgts_started;
    fstart = &mut (*ebuf).floc;
    fi.filenm = (*ebuf).floc.filenm;
    commands = xmalloc(200 as i32 as size_t) as *mut i8;
    loop {
        let mut linelen: size_t = 0;
        let mut line: *mut i8 = 0 as *mut i8;
        let mut wlen: size_t = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut p2: *mut i8 = 0 as *mut i8;
        let mut vmod: vmodifiers = vmodifiers {
            assign_v_define_v_undefine_v_override_v_private_v_export_v: [0; 1],
            c2rust_padding: [0; 3],
        };
        (*ebuf).floc.lineno = ((*ebuf).floc.lineno).wrapping_add(nlines as u64);
        nlines = readline(ebuf);
        if nlines < 0 as i32 as i64 {
            break;
        }
        line = (*ebuf).buffer;
        if (*ebuf).floc.lineno == 1 as i32 as u64 {
            let mut ul: *mut u8 = line as *mut u8;
            if *ul.offset(0 as i32 as isize) as i32 == 0xef as i32
                && *ul.offset(1 as i32 as isize) as i32 == 0xbb as i32
                && *ul.offset(2 as i32 as isize) as i32 == 0xbf as i32
            {
                line = line.offset(3 as i32 as isize);
                if 0x1 as i32 & db_level != 0 {
                    if !((*ebuf).floc.filenm).is_null() {
                        printf(
                            dcgettext(
                                0 as *const i8,
                                b"Skipping UTF-8 BOM in makefile '%s'\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            (*ebuf).floc.filenm,
                        );
                    } else {
                        printf(
                            dcgettext(
                                0 as *const i8,
                                b"Skipping UTF-8 BOM in makefile buffer\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                }
            }
        }
        if *line.offset(0 as i32 as isize) as i32 == '\0' as i32 {
            continue;
        }
        linelen = strlen(line);
        if *line.offset(0 as i32 as isize) as i32 == cmd_prefix as i32 {
            if no_targets != 0 {
                continue;
            } else if !filenames.is_null() {
                if ignoring != 0 {
                    continue;
                } else {
                    if commands_idx == 0 as i32 as u64 {
                        cmds_started = (*ebuf).floc.lineno as u32;
                    }
                    if linelen.wrapping_add(commands_idx) > commands_len {
                        commands_len = linelen
                            .wrapping_add(commands_idx)
                            .wrapping_mul(2 as i32 as u64);
                        commands = xrealloc(commands as *mut libc::c_void, commands_len)
                            as *mut i8;
                    }
                    memcpy(
                        &mut *commands.offset(commands_idx as isize) as *mut i8
                            as *mut libc::c_void,
                        line.offset(1 as i32 as isize) as *const libc::c_void,
                        linelen.wrapping_sub(1 as i32 as u64),
                    );
                    commands_idx = (commands_idx as u64)
                        .wrapping_add(linelen.wrapping_sub(1 as i32 as u64)) as size_t
                        as size_t;
                    let fresh1 = commands_idx;
                    commands_idx = commands_idx.wrapping_add(1);
                    *commands.offset(fresh1 as isize) = '\n' as i32 as i8;
                    continue;
                }
            }
        }
        if collapsed_length < linelen.wrapping_add(1 as i32 as u64) {
            collapsed_length = linelen.wrapping_add(1 as i32 as u64);
            free(collapsed as *mut libc::c_void);
            collapsed = xmalloc(collapsed_length) as *mut i8;
        }
        strcpy(collapsed, line);
        collapse_continuations(collapsed);
        remove_comments(collapsed);
        p = collapsed;
        while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32
        {
            p = p.offset(1);
            p;
        }
        p = parse_var_assignment(p, 0 as i32, &mut vmod);
        if vmod.assign_v() != 0 {
            let mut v: *mut variable = 0 as *mut variable;
            let mut origin: variable_origin = variable_origin::from_libc_c_uint(
                (if vmod.override_v() as i32 != 0 {
                    variable_origin::o_override as i32
                } else {
                    variable_origin::o_file as i32
                }) as u32,
            );
            if ignoring != 0 {
                if vmod.define_v() != 0 {
                    in_ignored_define = 1 as i32;
                }
            } else {
                if !filenames.is_null() {
                    fi.lineno = tgts_started as u64;
                    fi.offset = 0 as i32 as u64;
                    record_files(
                        filenames,
                        also_make_targets,
                        pattern,
                        pattern_percent,
                        depstr,
                        cmds_started,
                        commands,
                        commands_idx,
                        two_colon,
                        prefix,
                        &mut fi,
                    );
                    filenames = 0 as *mut nameseq;
                }
                commands_idx = 0 as i32 as size_t;
                no_targets = 0 as i32;
                pattern = 0 as *const i8;
                also_make_targets = 0 as i32;
                if vmod.undefine_v() != 0 {
                    do_undefine(p, origin, ebuf);
                } else {
                    if vmod.define_v() != 0 {
                        v = do_define(p, origin, ebuf);
                    } else {
                        v = try_variable_definition(fstart, p, origin, 0 as i32);
                    }
                    if vmod.export_v() as i32 != variable_export::v_default as i32 {
                        (*v).set_export(vmod.export_v());
                    }
                    if vmod.private_v() != 0 {
                        (*v).set_private_var(1 as i32 as u32);
                    }
                }
            }
        } else {
            if *p as i32 == '\0' as i32 {
                continue;
            }
            p2 = end_of_token(p);
            wlen = p2.offset_from(p) as i64 as size_t;
            while *stopchar_map.as_mut_ptr().offset(*p2 as u8 as isize) as i32
                & (0x2 as i32 | 0x4 as i32) != 0 as i32
            {
                p2 = p2.offset(1);
                p2;
            }
            if in_ignored_define != 0 {
                if wlen
                    == (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && memcmp(
                        b"endef\0" as *const u8 as *const i8 as *const libc::c_void,
                        p as *const libc::c_void,
                        (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0 as i32
                    && *stopchar_map.as_mut_ptr().offset(*p2 as u8 as isize) as i32
                        & (0x8 as i32 | 0x1 as i32) != 0 as i32
                {
                    in_ignored_define = 0 as i32;
                }
            } else {
                let mut i: i32 = conditional_line(p, wlen, fstart);
                if i != -(2 as i32) {
                    if i == -(1 as i32) {
                        fatal(
                            fstart,
                            0 as i32 as size_t,
                            dcgettext(
                                0 as *const i8,
                                b"invalid syntax in conditional\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    ignoring = i;
                } else {
                    if ignoring != 0 {
                        continue;
                    }
                    if wlen
                        == (::core::mem::size_of::<[i8; 7]>() as u64)
                            .wrapping_sub(1 as i32 as u64)
                        && memcmp(
                            b"export\0" as *const u8 as *const i8 as *const libc::c_void,
                            p as *const libc::c_void,
                            (::core::mem::size_of::<[i8; 7]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        ) == 0 as i32
                        || wlen
                            == (::core::mem::size_of::<[i8; 9]>() as u64)
                                .wrapping_sub(1 as i32 as u64)
                            && memcmp(
                                b"unexport\0" as *const u8 as *const i8
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::core::mem::size_of::<[i8; 9]>() as u64)
                                    .wrapping_sub(1 as i32 as u64),
                            ) == 0 as i32
                    {
                        let mut exporting: i32 = if *p as i32 == 'u' as i32 {
                            0 as i32
                        } else {
                            1 as i32
                        };
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as u64;
                            fi.offset = 0 as i32 as u64;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as i32 as size_t;
                        no_targets = 0 as i32;
                        pattern = 0 as *const i8;
                        also_make_targets = 0 as i32;
                        if *p2 as i32 == '\0' as i32 {
                            export_all_variables = exporting;
                        } else {
                            let mut l: size_t = 0;
                            let mut cp: *const i8 = 0 as *const i8;
                            let mut ap: *mut i8 = 0 as *mut i8;
                            ap = allocated_variable_expand_for_file(p2, 0 as *mut file);
                            cp = ap;
                            p = find_next_token(&mut cp, &mut l);
                            while !p.is_null() {
                                let mut v_0: *mut variable = lookup_variable(p, l);
                                if v_0.is_null() {
                                    v_0 = define_variable_in_set(
                                        p,
                                        l,
                                        b"\0" as *const u8 as *const i8,
                                        variable_origin::o_file,
                                        0 as i32,
                                        0 as *mut variable_set,
                                        fstart,
                                    );
                                }
                                (*v_0)
                                    .set_export(
                                        variable_export::from_libc_c_uint(
                                            (if exporting != 0 {
                                                variable_export::v_export as i32
                                            } else {
                                                variable_export::v_noexport as i32
                                            }) as u32,
                                        ),
                                    );
                                p = find_next_token(&mut cp, &mut l);
                            }
                            free(ap as *mut libc::c_void);
                        }
                    } else if wlen
                        == (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64)
                        && memcmp(
                            b"vpath\0" as *const u8 as *const i8 as *const libc::c_void,
                            p as *const libc::c_void,
                            (::core::mem::size_of::<[i8; 6]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        ) == 0 as i32
                    {
                        let mut cp_0: *const i8 = 0 as *const i8;
                        let mut vpat: *mut i8 = 0 as *mut i8;
                        let mut l_0: size_t = 0;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as u64;
                            fi.offset = 0 as i32 as u64;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as i32 as size_t;
                        no_targets = 0 as i32;
                        pattern = 0 as *const i8;
                        also_make_targets = 0 as i32;
                        cp_0 = variable_expand(p2);
                        p = find_next_token(&mut cp_0, &mut l_0);
                        if !p.is_null() {
                            vpat = xstrndup(p, l_0);
                            p = find_next_token(&mut cp_0, &mut l_0);
                        } else {
                            vpat = 0 as *mut i8;
                        }
                        construct_vpath_list(vpat, p);
                        free(vpat as *mut libc::c_void);
                    } else if wlen
                        == (::core::mem::size_of::<[i8; 8]>() as u64)
                            .wrapping_sub(1 as i32 as u64)
                        && memcmp(
                            b"include\0" as *const u8 as *const i8
                                as *const libc::c_void,
                            p as *const libc::c_void,
                            (::core::mem::size_of::<[i8; 8]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        ) == 0 as i32
                        || wlen
                            == (::core::mem::size_of::<[i8; 9]>() as u64)
                                .wrapping_sub(1 as i32 as u64)
                            && memcmp(
                                b"-include\0" as *const u8 as *const i8
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::core::mem::size_of::<[i8; 9]>() as u64)
                                    .wrapping_sub(1 as i32 as u64),
                            ) == 0 as i32
                        || wlen
                            == (::core::mem::size_of::<[i8; 9]>() as u64)
                                .wrapping_sub(1 as i32 as u64)
                            && memcmp(
                                b"sinclude\0" as *const u8 as *const i8
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::core::mem::size_of::<[i8; 9]>() as u64)
                                    .wrapping_sub(1 as i32 as u64),
                            ) == 0 as i32
                    {
                        let mut save: *mut conditionals = 0 as *mut conditionals;
                        let mut new_conditionals: conditionals = conditionals {
                            if_cmds: 0,
                            allocated: 0,
                            ignoring: 0 as *const i8 as *mut i8,
                            seen_else: 0 as *const i8 as *mut i8,
                        };
                        let mut files: *mut nameseq = 0 as *mut nameseq;
                        let mut noerror: i32 = (*p.offset(0 as i32 as isize) as i32
                            != 'i' as i32) as i32;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as u64;
                            fi.offset = 0 as i32 as u64;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as i32 as size_t;
                        no_targets = 0 as i32;
                        pattern = 0 as *const i8;
                        also_make_targets = 0 as i32;
                        p = allocated_variable_expand_for_file(p2, 0 as *mut file);
                        if *p as i32 == '\0' as i32 {
                            free(p as *mut libc::c_void);
                        } else {
                            p2 = p;
                            files = parse_file_seq(
                                &mut p2,
                                ::core::mem::size_of::<nameseq>() as u64,
                                0x1 as i32,
                                0 as *const i8,
                                0x2 as i32,
                            ) as *mut nameseq;
                            free(p as *mut libc::c_void);
                            save = install_conditionals(&mut new_conditionals);
                            if !filenames.is_null() {
                                fi.lineno = tgts_started as u64;
                                fi.offset = 0 as i32 as u64;
                                record_files(
                                    filenames,
                                    also_make_targets,
                                    pattern,
                                    pattern_percent,
                                    depstr,
                                    cmds_started,
                                    commands,
                                    commands_idx,
                                    two_colon,
                                    prefix,
                                    &mut fi,
                                );
                                filenames = 0 as *mut nameseq;
                            }
                            commands_idx = 0 as i32 as size_t;
                            no_targets = 0 as i32;
                            pattern = 0 as *const i8;
                            also_make_targets = 0 as i32;
                            while !files.is_null() {
                                let mut next: *mut nameseq = (*files).next;
                                let mut flags: libc::c_ushort = ((1 as i32) << 1 as i32
                                    | (1 as i32) << 3 as i32
                                    | (if noerror != 0 {
                                        (1 as i32) << 2 as i32
                                    } else {
                                        0 as i32
                                    })
                                    | (if set_default != 0 {
                                        0 as i32
                                    } else {
                                        (1 as i32) << 0 as i32
                                    })) as libc::c_ushort;
                                let mut d: *mut goaldep = eval_makefile(
                                    (*files).name,
                                    flags,
                                );
                                (*d).floc = *fstart;
                                free(files as *mut libc::c_void);
                                files = next;
                            }
                            restore_conditionals(save);
                        }
                    } else if wlen
                        == (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64)
                        && memcmp(
                            b"load\0" as *const u8 as *const i8 as *const libc::c_void,
                            p as *const libc::c_void,
                            (::core::mem::size_of::<[i8; 5]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        ) == 0 as i32
                        || wlen
                            == (::core::mem::size_of::<[i8; 6]>() as u64)
                                .wrapping_sub(1 as i32 as u64)
                            && memcmp(
                                b"-load\0" as *const u8 as *const i8 as *const libc::c_void,
                                p as *const libc::c_void,
                                (::core::mem::size_of::<[i8; 6]>() as u64)
                                    .wrapping_sub(1 as i32 as u64),
                            ) == 0 as i32
                    {
                        let mut files_0: *mut nameseq = 0 as *mut nameseq;
                        let mut noerror_0: i32 = (*p.offset(0 as i32 as isize) as i32
                            == '-' as i32) as i32;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as u64;
                            fi.offset = 0 as i32 as u64;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as i32 as size_t;
                        no_targets = 0 as i32;
                        pattern = 0 as *const i8;
                        also_make_targets = 0 as i32;
                        p = allocated_variable_expand_for_file(p2, 0 as *mut file);
                        if *p as i32 == '\0' as i32 {
                            free(p as *mut libc::c_void);
                        } else {
                            p2 = p;
                            files_0 = parse_file_seq(
                                &mut p2,
                                ::core::mem::size_of::<nameseq>() as u64,
                                0x1 as i32,
                                0 as *const i8,
                                0x2 as i32,
                            ) as *mut nameseq;
                            free(p as *mut libc::c_void);
                            while !files_0.is_null() {
                                let mut next_0: *mut nameseq = (*files_0).next;
                                let mut name: *const i8 = (*files_0).name;
                                let mut deps: *mut goaldep = 0 as *mut goaldep;
                                let mut f: *mut file = 0 as *mut file;
                                let mut r: i32 = 0;
                                let mut file: file = {
                                    let mut init = file {
                                        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
                                        c2rust_padding: [0; 4],
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
                                    };
                                    init.set_update_status(update_status::us_success);
                                    init.set_command_state(cmd_state::cs_not_started);
                                    init.set_builtin(0);
                                    init.set_precious(0);
                                    init.set_loaded(0);
                                    init.set_unloaded(0);
                                    init.set_low_resolution_time(0);
                                    init.set_tried_implicit(0);
                                    init.set_updating(0);
                                    init.set_updated(0);
                                    init.set_is_target(0);
                                    init.set_cmd_target(0);
                                    init.set_phony(0);
                                    init.set_intermediate(0);
                                    init.set_is_explicit(0);
                                    init.set_secondary(0);
                                    init.set_notintermediate(0);
                                    init.set_dontcare(0);
                                    init.set_ignore_vpath(0);
                                    init.set_pat_searched(0);
                                    init.set_no_diag(0);
                                    init.set_was_shuffled(0);
                                    init.set_snapped(0);
                                    init
                                };
                                file.name = name;
                                r = load_file(&mut (*ebuf).floc, &mut file, noerror_0);
                                if r == 0 && noerror_0 == 0 {
                                    fatal(
                                        &mut (*ebuf).floc as *mut floc,
                                        strlen(name),
                                        dcgettext(
                                            0 as *const i8,
                                            b"%s: failed to load\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                        name,
                                    );
                                }
                                name = file.name;
                                f = lookup_file(name);
                                if f.is_null() {
                                    f = enter_file(name);
                                }
                                (*f).set_loaded(1 as i32 as u32);
                                (*f).set_unloaded(0 as i32 as u32);
                                free(files_0 as *mut libc::c_void);
                                files_0 = next_0;
                                if r == -(1 as i32) {
                                    continue;
                                }
                                deps = xcalloc(::core::mem::size_of::<goaldep>() as u64)
                                    as *mut goaldep;
                                (*deps).next = read_files;
                                (*deps).floc = (*ebuf).floc;
                                read_files = deps;
                                (*deps).file = f;
                            }
                        }
                    } else {
                        if *line.offset(0 as i32 as isize) as i32 == cmd_prefix as i32 {
                            fatal(
                                fstart,
                                0 as i32 as size_t,
                                dcgettext(
                                    0 as *const i8,
                                    b"recipe commences before first target\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        let mut wtype: make_word_type = make_word_type::w_bogus;
                        let mut cmdleft: *mut i8 = 0 as *mut i8;
                        let mut semip: *mut i8 = 0 as *mut i8;
                        let mut lb_next: *mut i8 = 0 as *mut i8;
                        let mut plen: size_t = 0 as i32 as size_t;
                        let mut colonp: *mut i8 = 0 as *mut i8;
                        let mut end: *const i8 = 0 as *const i8;
                        let mut beg: *const i8 = 0 as *const i8;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as u64;
                            fi.offset = 0 as i32 as u64;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as i32 as size_t;
                        no_targets = 0 as i32;
                        pattern = 0 as *const i8;
                        also_make_targets = 0 as i32;
                        tgts_started = (*fstart).lineno as u32;
                        cmdleft = find_map_unquote(
                            line,
                            0x10 as i32 | 0x8 as i32 | 0x4000 as i32,
                        );
                        if !cmdleft.is_null() && *cmdleft as i32 == '#' as i32 {
                            *cmdleft = '\0' as i32 as i8;
                            cmdleft = 0 as *mut i8;
                        } else if !cmdleft.is_null() {
                            let fresh2 = cmdleft;
                            cmdleft = cmdleft.offset(1);
                            semip = fresh2;
                            *semip = '\0' as i32 as i8;
                        }
                        collapse_continuations(line);
                        wtype = get_next_mword(line, &mut lb_next, &mut wlen);
                        match wtype as u32 {
                            1 => {
                                if !cmdleft.is_null() {
                                    fatal(
                                        fstart,
                                        0 as i32 as size_t,
                                        dcgettext(
                                            0 as *const i8,
                                            b"missing rule before recipe\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                }
                            }
                            4 | 5 | 8 | 9 => {
                                no_targets = 1 as i32;
                            }
                            _ => {
                                p2 = variable_expand_string(0 as *mut i8, lb_next, wlen);
                                loop {
                                    lb_next = lb_next.offset(wlen as isize);
                                    if cmdleft.is_null() {
                                        cmdleft = find_char_unquote(p2, ';' as i32);
                                        if !cmdleft.is_null() {
                                            let mut p2_off: size_t = p2.offset_from(variable_buffer)
                                                as i64 as size_t;
                                            let mut cmd_off: size_t = cmdleft
                                                .offset_from(variable_buffer) as i64 as size_t;
                                            let mut pend: *mut i8 = p2.offset(strlen(p2) as isize);
                                            *cmdleft = '\0' as i32 as i8;
                                            variable_expand_string(
                                                pend,
                                                lb_next,
                                                18446744073709551615 as u64,
                                            );
                                            lb_next = lb_next.offset(strlen(lb_next) as isize);
                                            p2 = variable_buffer.offset(p2_off as isize);
                                            cmdleft = variable_buffer
                                                .offset(cmd_off as isize)
                                                .offset(1 as i32 as isize);
                                        }
                                    }
                                    colonp = find_char_unquote(p2, ':' as i32);
                                    if !colonp.is_null() {
                                        if colonp > p2
                                            && *colonp.offset(-(1 as i32) as isize) as i32 == '&' as i32
                                        {
                                            colonp = colonp.offset(-1);
                                            colonp;
                                        }
                                        break;
                                    } else {
                                        wtype = get_next_mword(lb_next, &mut lb_next, &mut wlen);
                                        if wtype as u32 == make_word_type::w_eol as i32 as u32 {
                                            break;
                                        }
                                        p2 = p2.offset(strlen(p2) as isize);
                                        let fresh3 = p2;
                                        p2 = p2.offset(1);
                                        *fresh3 = ' ' as i32 as i8;
                                        p2 = variable_expand_string(p2, lb_next, wlen);
                                    }
                                }
                                p2 = next_token(variable_buffer);
                                if wtype as u32 == make_word_type::w_eol as i32 as u32 {
                                    if *p2 as i32 == '\0' as i32 {
                                        continue;
                                    }
                                    if cmd_prefix as i32 == '\t' as i32
                                        && strncmp(
                                            line,
                                            b"        \0" as *const u8 as *const i8,
                                            8 as i32 as u64,
                                        ) == 0 as i32
                                    {
                                        fatal(
                                            fstart,
                                            0 as i32 as size_t,
                                            dcgettext(
                                                0 as *const i8,
                                                b"missing separator (did you mean TAB instead of 8 spaces?)\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    }
                                    p2 = next_token(line);
                                    if strncmp(
                                        p2,
                                        b"if\0" as *const u8 as *const i8,
                                        2 as i32 as u64,
                                    ) == 0 as i32
                                        && (strncmp(
                                            &mut *p2.offset(2 as i32 as isize),
                                            b"neq\0" as *const u8 as *const i8,
                                            3 as i32 as u64,
                                        ) == 0 as i32
                                            && !(*stopchar_map
                                                .as_mut_ptr()
                                                .offset(*p2.offset(5 as i32 as isize) as u8 as isize) as i32
                                                & 0x2 as i32 != 0 as i32)
                                            || strncmp(
                                                &mut *p2.offset(2 as i32 as isize),
                                                b"eq\0" as *const u8 as *const i8,
                                                2 as i32 as u64,
                                            ) == 0 as i32
                                                && !(*stopchar_map
                                                    .as_mut_ptr()
                                                    .offset(*p2.offset(4 as i32 as isize) as u8 as isize) as i32
                                                    & 0x2 as i32 != 0 as i32))
                                    {
                                        fatal(
                                            fstart,
                                            0 as i32 as size_t,
                                            dcgettext(
                                                0 as *const i8,
                                                b"missing separator (ifeq/ifneq must be followed by whitespace)\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    }
                                    fatal(
                                        fstart,
                                        0 as i32 as size_t,
                                        dcgettext(
                                            0 as *const i8,
                                            b"missing separator\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                } else {
                                    let mut save_0: i8 = *colonp;
                                    if save_0 as i32 == '&' as i32 {
                                        also_make_targets = 1 as i32;
                                    }
                                    *colonp = '\0' as i32 as i8;
                                    filenames = parse_file_seq(
                                        &mut p2,
                                        ::core::mem::size_of::<nameseq>() as u64,
                                        0x1 as i32,
                                        0 as *const i8,
                                        0 as i32,
                                    ) as *mut nameseq;
                                    *colonp = save_0;
                                    p2 = colonp
                                        .offset((save_0 as i32 == '&' as i32) as i32 as isize);
                                    if filenames.is_null() {
                                        no_targets = 1 as i32;
                                    } else {
                                        p2 = p2.offset(1);
                                        p2;
                                        two_colon = (*p2 as i32 == ':' as i32) as i32;
                                        if two_colon != 0 {
                                            p2 = p2.offset(1);
                                            p2;
                                        }
                                        if *lb_next as i32 != '\0' as i32 {
                                            let mut l_1: size_t = p2.offset_from(variable_buffer) as i64
                                                as size_t;
                                            plen = strlen(p2);
                                            variable_buffer_output(
                                                p2.offset(plen as isize),
                                                lb_next,
                                                (strlen(lb_next)).wrapping_add(1 as i32 as u64),
                                            );
                                            p2 = variable_buffer.offset(l_1 as isize);
                                        }
                                        p2 = parse_var_assignment(p2, 1 as i32, &mut vmod);
                                        if vmod.assign_v() != 0 {
                                            if !semip.is_null() {
                                                let mut l_2: size_t = p2.offset_from(variable_buffer) as i64
                                                    as size_t;
                                                *semip = ';' as i32 as i8;
                                                collapse_continuations(semip);
                                                variable_buffer_output(
                                                    p2.offset(strlen(p2) as isize),
                                                    semip,
                                                    (strlen(semip)).wrapping_add(1 as i32 as u64),
                                                );
                                                p2 = variable_buffer.offset(l_2 as isize);
                                            }
                                            record_target_var(
                                                filenames,
                                                p2,
                                                variable_origin::from_libc_c_uint(
                                                    (if vmod.override_v() as i32 != 0 {
                                                        variable_origin::o_override as i32
                                                    } else {
                                                        variable_origin::o_file as i32
                                                    }) as u32,
                                                ),
                                                &mut vmod,
                                                fstart,
                                            );
                                            filenames = 0 as *mut nameseq;
                                        } else {
                                            find_char_unquote(lb_next, '=' as i32);
                                            prefix = cmd_prefix;
                                            no_targets = 0 as i32;
                                            if *lb_next as i32 != '\0' as i32 {
                                                let mut l_3: size_t = p2.offset_from(variable_buffer) as i64
                                                    as size_t;
                                                variable_expand_string(
                                                    p2.offset(plen as isize),
                                                    lb_next,
                                                    18446744073709551615 as u64,
                                                );
                                                p2 = variable_buffer.offset(l_3 as isize);
                                                if cmdleft.is_null() {
                                                    cmdleft = find_char_unquote(p2, ';' as i32);
                                                    if !cmdleft.is_null() {
                                                        let fresh4 = cmdleft;
                                                        cmdleft = cmdleft.offset(1);
                                                        *fresh4 = '\0' as i32 as i8;
                                                    }
                                                }
                                            }
                                            p = strchr(p2, ':' as i32);
                                            while !p.is_null()
                                                && *p.offset(-(1 as i32) as isize) as i32 == '\\' as i32
                                            {
                                                let mut q: *mut i8 = &mut *p.offset(-(1 as i32) as isize)
                                                    as *mut i8;
                                                let mut backslash: i32 = 0 as i32;
                                                loop {
                                                    let fresh5 = q;
                                                    q = q.offset(-1);
                                                    if !(*fresh5 as i32 == '\\' as i32) {
                                                        break;
                                                    }
                                                    backslash = (backslash == 0) as i32;
                                                }
                                                if !(backslash != 0) {
                                                    break;
                                                }
                                                p = strchr(p.offset(1 as i32 as isize), ':' as i32);
                                            }
                                            if !p.is_null() {
                                                let mut target: *mut nameseq = 0 as *mut nameseq;
                                                target = parse_file_seq(
                                                    &mut p2,
                                                    ::core::mem::size_of::<nameseq>() as u64,
                                                    0x40 as i32,
                                                    0 as *const i8,
                                                    0x4 as i32,
                                                ) as *mut nameseq;
                                                p2 = p2.offset(1);
                                                p2;
                                                if target.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0 as i32 as size_t,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"missing target pattern\0" as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                    );
                                                } else if !((*target).next).is_null() {
                                                    fatal(
                                                        fstart,
                                                        0 as i32 as size_t,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"multiple target patterns\0" as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                    );
                                                }
                                                pattern_percent = find_percent_cached(&mut (*target).name);
                                                pattern = (*target).name;
                                                if pattern_percent.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0 as i32 as size_t,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"target pattern contains no '%%'\0" as *const u8
                                                                as *const i8,
                                                            5 as i32,
                                                        ),
                                                    );
                                                }
                                                free(target as *mut libc::c_void);
                                            } else {
                                                pattern = 0 as *const i8;
                                            }
                                            beg = p2;
                                            end = beg
                                                .offset(strlen(beg) as isize)
                                                .offset(-(1 as i32 as isize));
                                            strip_whitespace(&mut beg, &mut end);
                                            if beg <= end && *beg as i32 != '\0' as i32 {
                                                depstr = xstrndup(
                                                    beg,
                                                    (end.offset_from(beg) as i64 + 1 as i32 as i64) as size_t,
                                                );
                                            } else {
                                                depstr = 0 as *mut i8;
                                            }
                                            commands_idx = 0 as i32 as size_t;
                                            if !cmdleft.is_null() {
                                                let mut l_4: size_t = strlen(cmdleft);
                                                cmds_started = (*fstart).lineno as u32;
                                                if l_4.wrapping_add(2 as i32 as u64) > commands_len {
                                                    commands_len = l_4
                                                        .wrapping_add(2 as i32 as u64)
                                                        .wrapping_mul(2 as i32 as u64);
                                                    commands = xrealloc(
                                                        commands as *mut libc::c_void,
                                                        commands_len,
                                                    ) as *mut i8;
                                                }
                                                memcpy(
                                                    commands as *mut libc::c_void,
                                                    cmdleft as *const libc::c_void,
                                                    l_4,
                                                );
                                                commands_idx = (commands_idx as u64).wrapping_add(l_4)
                                                    as size_t as size_t;
                                                let fresh6 = commands_idx;
                                                commands_idx = commands_idx.wrapping_add(1);
                                                *commands.offset(fresh6 as isize) = '\n' as i32 as i8;
                                            }
                                            check_specials(filenames, set_default);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (*conditionals).if_cmds != 0 {
        fatal(
            fstart,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"missing 'endif'\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if !filenames.is_null() {
        fi.lineno = tgts_started as u64;
        fi.offset = 0 as i32 as u64;
        record_files(
            filenames,
            also_make_targets,
            pattern,
            pattern_percent,
            depstr,
            cmds_started,
            commands,
            commands_idx,
            two_colon,
            prefix,
            &mut fi,
        );
        filenames = 0 as *mut nameseq;
    }
    commands_idx = 0 as i32 as size_t;
    no_targets = 0 as i32;
    pattern = 0 as *const i8;
    also_make_targets = 0 as i32;
    free(collapsed as *mut libc::c_void);
    free(commands as *mut libc::c_void);
}
unsafe extern "C" fn remove_comments(mut line: *mut i8) {
    let mut comment: *mut i8 = 0 as *mut i8;
    comment = find_map_unquote(line, 0x8 as i32 | 0x4000 as i32);
    if !comment.is_null() {
        *comment = '\0' as i32 as i8;
    }
}
unsafe extern "C" fn do_undefine(
    mut name: *mut i8,
    mut origin: variable_origin,
    mut ebuf: *mut ebuffer,
) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut var: *mut i8 = 0 as *mut i8;
    var = allocated_variable_expand_for_file(name, 0 as *mut file);
    name = next_token(var);
    if *name as i32 == '\0' as i32 {
        fatal(
            &mut (*ebuf).floc as *mut floc,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"empty variable name\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    p = name.offset(strlen(name) as isize).offset(-(1 as i32 as isize));
    while p > name
        && *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & 0x2 as i32
            != 0 as i32
    {
        p = p.offset(-1);
        p;
    }
    *p.offset(1 as i32 as isize) = '\0' as i32 as i8;
    undefine_variable_in_set(
        name,
        (p.offset_from(name) as i64 + 1 as i32 as i64) as size_t,
        origin,
        0 as *mut variable_set,
    );
    free(var as *mut libc::c_void);
}
unsafe extern "C" fn do_define(
    mut name: *mut i8,
    mut origin: variable_origin,
    mut ebuf: *mut ebuffer,
) -> *mut variable {
    let mut v: *mut variable = 0 as *mut variable;
    let mut var: variable = variable {
        name: 0 as *mut i8,
        value: 0 as *mut i8,
        fileinfo: floc {
            filenm: 0 as *const i8,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut defstart: floc = floc {
        filenm: 0 as *const i8,
        lineno: 0,
        offset: 0,
    };
    let mut nlevels: i32 = 1 as i32;
    let mut length: size_t = 100 as i32 as size_t;
    let mut definition: *mut i8 = xmalloc(length) as *mut i8;
    let mut idx: size_t = 0 as i32 as size_t;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut n: *mut i8 = 0 as *mut i8;
    defstart = (*ebuf).floc;
    p = parse_variable_definition(name, &mut var);
    if p.is_null() {
        var.set_flavor(variable_flavor::f_recursive);
    } else {
        if *(var.value).offset(0 as i32 as isize) as i32 != '\0' as i32 {
            error(
                &mut defstart as *mut floc,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"extraneous text after 'define' directive\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        *(var.name).offset(var.length as isize) = '\0' as i32 as i8;
    }
    n = allocated_variable_expand_for_file(name, 0 as *mut file);
    name = next_token(n);
    if *name.offset(0 as i32 as isize) as i32 == '\0' as i32 {
        fatal(
            &mut defstart as *mut floc,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"empty variable name\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    p = name.offset(strlen(name) as isize).offset(-(1 as i32 as isize));
    while p > name
        && *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & 0x2 as i32
            != 0 as i32
    {
        p = p.offset(-1);
        p;
    }
    *p.offset(1 as i32 as isize) = '\0' as i32 as i8;
    loop {
        let mut len: size_t = 0;
        let mut line: *mut i8 = 0 as *mut i8;
        let mut nlines: i64 = readline(ebuf);
        if nlines < 0 as i32 as i64 {
            fatal(
                &mut defstart as *mut floc,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"missing 'endef', unterminated 'define'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        (*ebuf).floc.lineno = ((*ebuf).floc.lineno).wrapping_add(nlines as u64);
        line = (*ebuf).buffer;
        collapse_continuations(line);
        if *line.offset(0 as i32 as isize) as i32 != cmd_prefix as i32 {
            p = next_token(line);
            len = strlen(p);
            if (len == 6 as i32 as u64
                || len > 6 as i32 as u64
                    && *stopchar_map
                        .as_mut_ptr()
                        .offset(*p.offset(6 as i32 as isize) as u8 as isize) as i32
                        & 0x2 as i32 != 0 as i32)
                && strncmp(p, b"define\0" as *const u8 as *const i8, 6 as i32 as u64)
                    == 0 as i32
            {
                nlevels += 1;
                nlevels;
            } else if (len == 5 as i32 as u64
                || len > 5 as i32 as u64
                    && *stopchar_map
                        .as_mut_ptr()
                        .offset(*p.offset(5 as i32 as isize) as u8 as isize) as i32
                        & 0x2 as i32 != 0 as i32)
                && strncmp(p, b"endef\0" as *const u8 as *const i8, 5 as i32 as u64)
                    == 0 as i32
            {
                p = p.offset(5 as i32 as isize);
                remove_comments(p);
                if *next_token(p) as i32 != '\0' as i32 {
                    error(
                        &mut (*ebuf).floc as *mut floc,
                        0 as i32 as size_t,
                        dcgettext(
                            0 as *const i8,
                            b"extraneous text after 'endef' directive\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                nlevels -= 1;
                if nlevels == 0 as i32 {
                    break;
                }
            }
        }
        len = strlen(line);
        if idx.wrapping_add(len).wrapping_add(1 as i32 as u64) > length {
            length = idx.wrapping_add(len).wrapping_mul(2 as i32 as u64);
            definition = xrealloc(
                definition as *mut libc::c_void,
                length.wrapping_add(1 as i32 as u64),
            ) as *mut i8;
        }
        memcpy(
            &mut *definition.offset(idx as isize) as *mut i8 as *mut libc::c_void,
            line as *const libc::c_void,
            len,
        );
        idx = (idx as u64).wrapping_add(len) as size_t as size_t;
        let fresh7 = idx;
        idx = idx.wrapping_add(1);
        *definition.offset(fresh7 as isize) = '\n' as i32 as i8;
    }
    if idx == 0 as i32 as u64 {
        *definition.offset(0 as i32 as isize) = '\0' as i32 as i8;
    } else {
        *definition.offset(idx.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32
            as i8;
    }
    v = do_variable_definition(
        &mut defstart,
        name,
        definition,
        origin,
        var.flavor(),
        0 as i32,
    );
    free(definition as *mut libc::c_void);
    free(n as *mut libc::c_void);
    return v;
}
unsafe extern "C" fn conditional_line(
    mut line: *mut i8,
    mut len: size_t,
    mut flocp: *const floc,
) -> i32 {
    let mut cmdname: *const i8 = 0 as *const i8;
    let mut cmdtype: C2RustUnnamed = C2RustUnnamed::c_ifdef;
    let mut i: u32 = 0;
    let mut o: u32 = 0;
    if len == (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
        && strncmp(
            b"ifdef\0" as *const u8 as *const i8,
            line,
            (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    {
        cmdtype = C2RustUnnamed::c_ifdef;
        cmdname = b"ifdef\0" as *const u8 as *const i8;
    } else if len
        == (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
        && strncmp(
            b"ifndef\0" as *const u8 as *const i8,
            line,
            (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    {
        cmdtype = C2RustUnnamed::c_ifndef;
        cmdname = b"ifndef\0" as *const u8 as *const i8;
    } else if len
        == (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
        && strncmp(
            b"ifeq\0" as *const u8 as *const i8,
            line,
            (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    {
        cmdtype = C2RustUnnamed::c_ifeq;
        cmdname = b"ifeq\0" as *const u8 as *const i8;
    } else if len
        == (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
        && strncmp(
            b"ifneq\0" as *const u8 as *const i8,
            line,
            (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    {
        cmdtype = C2RustUnnamed::c_ifneq;
        cmdname = b"ifneq\0" as *const u8 as *const i8;
    } else if len
        == (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
        && strncmp(
            b"else\0" as *const u8 as *const i8,
            line,
            (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    {
        cmdtype = C2RustUnnamed::c_else;
        cmdname = b"else\0" as *const u8 as *const i8;
    } else if len
        == (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
        && strncmp(
            b"endif\0" as *const u8 as *const i8,
            line,
            (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
    {
        cmdtype = C2RustUnnamed::c_endif;
        cmdname = b"endif\0" as *const u8 as *const i8;
    } else {
        return -(2 as i32)
    }
    line = line.offset(len as isize);
    while *stopchar_map.as_mut_ptr().offset(*line as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        line = line.offset(1);
        line;
    }
    if cmdtype as u32 == C2RustUnnamed::c_endif as i32 as u32 {
        if *line as i32 != '\0' as i32 {
            error(
                flocp,
                strlen(cmdname),
                dcgettext(
                    0 as *const i8,
                    b"extraneous text after '%s' directive\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                cmdname,
            );
        }
        if (*conditionals).if_cmds == 0 {
            fatal(
                flocp,
                strlen(cmdname),
                dcgettext(
                    0 as *const i8,
                    b"extraneous '%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                cmdname,
            );
        }
        (*conditionals).if_cmds = ((*conditionals).if_cmds).wrapping_sub(1);
        (*conditionals).if_cmds;
    } else if cmdtype as u32 == C2RustUnnamed::c_else as i32 as u32 {
        let mut p: *const i8 = 0 as *const i8;
        if (*conditionals).if_cmds == 0 {
            fatal(
                flocp,
                strlen(cmdname),
                dcgettext(
                    0 as *const i8,
                    b"extraneous '%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                cmdname,
            );
        }
        o = ((*conditionals).if_cmds).wrapping_sub(1 as i32 as u32);
        if *((*conditionals).seen_else).offset(o as isize) != 0 {
            fatal(
                flocp,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"only one 'else' per conditional\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        match *((*conditionals).ignoring).offset(o as isize) as i32 {
            0 => {
                *((*conditionals).ignoring).offset(o as isize) = 2 as i32 as i8;
            }
            1 => {
                *((*conditionals).ignoring).offset(o as isize) = 0 as i32 as i8;
            }
            _ => {}
        }
        if *line as i32 == '\0' as i32 {
            *((*conditionals).seen_else).offset(o as isize) = 1 as i32 as i8;
        } else {
            p = line.offset(1 as i32 as isize);
            while !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
                & (0x2 as i32 | 0x4 as i32 | 0x1 as i32) != 0 as i32)
            {
                p = p.offset(1);
                p;
            }
            len = p.offset_from(line) as i64 as size_t;
            if len
                == (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                && strncmp(
                    b"else\0" as *const u8 as *const i8,
                    line,
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                ) == 0 as i32
                || len
                    == (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && strncmp(
                        b"endif\0" as *const u8 as *const i8,
                        line,
                        (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0 as i32 || conditional_line(line, len, flocp) < 0 as i32
            {
                error(
                    flocp,
                    strlen(cmdname),
                    dcgettext(
                        0 as *const i8,
                        b"extraneous text after '%s' directive\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    cmdname,
                );
            } else {
                if (*((*conditionals).ignoring).offset(o as isize) as i32) < 2 as i32 {
                    *((*conditionals).ignoring).offset(o as isize) = *((*conditionals)
                        .ignoring)
                        .offset(o.wrapping_add(1 as i32 as u32) as isize);
                }
                (*conditionals).if_cmds = ((*conditionals).if_cmds).wrapping_sub(1);
                (*conditionals).if_cmds;
            }
        }
    } else {
        if (*conditionals).allocated == 0 as i32 as u32 {
            (*conditionals).allocated = 5 as i32 as u32;
            (*conditionals).ignoring = xmalloc((*conditionals).allocated as size_t)
                as *mut i8;
            (*conditionals).seen_else = xmalloc((*conditionals).allocated as size_t)
                as *mut i8;
        }
        let fresh8 = (*conditionals).if_cmds;
        (*conditionals).if_cmds = ((*conditionals).if_cmds).wrapping_add(1);
        o = fresh8;
        if (*conditionals).if_cmds > (*conditionals).allocated {
            (*conditionals).allocated = ((*conditionals).allocated)
                .wrapping_add(5 as i32 as u32);
            (*conditionals).ignoring = xrealloc(
                (*conditionals).ignoring as *mut libc::c_void,
                (*conditionals).allocated as size_t,
            ) as *mut i8;
            (*conditionals).seen_else = xrealloc(
                (*conditionals).seen_else as *mut libc::c_void,
                (*conditionals).allocated as size_t,
            ) as *mut i8;
        }
        *((*conditionals).seen_else).offset(o as isize) = 0 as i32 as i8;
        i = 0 as i32 as u32;
        while i < o {
            if *((*conditionals).ignoring).offset(i as isize) != 0 {
                *((*conditionals).ignoring).offset(o as isize) = 1 as i32 as i8;
                return 1 as i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        if cmdtype as u32 == C2RustUnnamed::c_ifdef as i32 as u32
            || cmdtype as u32 == C2RustUnnamed::c_ifndef as i32 as u32
        {
            let mut l: size_t = 0;
            let mut var: *mut i8 = 0 as *mut i8;
            let mut v: *mut variable = 0 as *mut variable;
            let mut p_0: *mut i8 = 0 as *mut i8;
            var = allocated_variable_expand_for_file(line, 0 as *mut file);
            p_0 = end_of_token(var);
            l = p_0.offset_from(var) as i64 as size_t;
            while *stopchar_map.as_mut_ptr().offset(*p_0 as u8 as isize) as i32
                & (0x2 as i32 | 0x4 as i32) != 0 as i32
            {
                p_0 = p_0.offset(1);
                p_0;
            }
            if *p_0 as i32 != '\0' as i32 {
                return -(1 as i32);
            }
            *var.offset(l as isize) = '\0' as i32 as i8;
            v = lookup_variable(var, l);
            *((*conditionals).ignoring).offset(o as isize) = ((!v.is_null()
                && *(*v).value as i32 != '\0' as i32) as i32
                == (cmdtype as u32 == C2RustUnnamed::c_ifndef as i32 as u32) as i32)
                as i32 as i8;
            free(var as *mut libc::c_void);
        } else {
            let mut s1: *mut i8 = 0 as *mut i8;
            let mut s2: *mut i8 = 0 as *mut i8;
            let mut l_0: size_t = 0;
            let mut termin: i8 = (if *line as i32 == '(' as i32 {
                ',' as i32
            } else {
                *line as i32
            }) as i8;
            if termin as i32 != ',' as i32 && termin as i32 != '"' as i32
                && termin as i32 != '\'' as i32
            {
                return -(1 as i32);
            }
            line = line.offset(1);
            s1 = line;
            if termin as i32 == ',' as i32 {
                let mut count: i32 = 0 as i32;
                while *line as i32 != '\0' as i32 {
                    if *line as i32 == '(' as i32 {
                        count += 1;
                        count;
                    } else if *line as i32 == ')' as i32 {
                        count -= 1;
                        count;
                    } else if *line as i32 == ',' as i32 && count <= 0 as i32 {
                        break;
                    }
                    line = line.offset(1);
                    line;
                }
            } else {
                while *line as i32 != '\0' as i32 && *line as i32 != termin as i32 {
                    line = line.offset(1);
                    line;
                }
            }
            if *line as i32 == '\0' as i32 {
                return -(1 as i32);
            }
            if termin as i32 == ',' as i32 {
                let fresh9 = line;
                line = line.offset(1);
                let mut p_1: *mut i8 = fresh9;
                while *stopchar_map
                    .as_mut_ptr()
                    .offset(*p_1.offset(-(1 as i32) as isize) as u8 as isize) as i32
                    & 0x2 as i32 != 0 as i32
                {
                    p_1 = p_1.offset(-1);
                    p_1;
                }
                *p_1 = '\0' as i32 as i8;
            } else {
                let fresh10 = line;
                line = line.offset(1);
                *fresh10 = '\0' as i32 as i8;
            }
            s2 = variable_expand(s1);
            l_0 = strlen(s2);
            let mut fresh11 = ::std::vec::from_elem(
                0,
                l_0.wrapping_add(1 as i32 as u64) as usize,
            );
            s1 = fresh11.as_mut_ptr() as *mut i8;
            memcpy(
                s1 as *mut libc::c_void,
                s2 as *const libc::c_void,
                l_0.wrapping_add(1 as i32 as u64),
            );
            if termin as i32 != ',' as i32 {
                while *stopchar_map.as_mut_ptr().offset(*line as u8 as isize) as i32
                    & (0x2 as i32 | 0x4 as i32) != 0 as i32
                {
                    line = line.offset(1);
                    line;
                }
            }
            termin = (if termin as i32 == ',' as i32 {
                ')' as i32
            } else {
                *line as i32
            }) as i8;
            if termin as i32 != ')' as i32 && termin as i32 != '"' as i32
                && termin as i32 != '\'' as i32
            {
                return -(1 as i32);
            }
            if termin as i32 == ')' as i32 {
                let mut count_0: i32 = 0 as i32;
                s2 = next_token(line);
                line = s2;
                while *line as i32 != '\0' as i32 {
                    if *line as i32 == '(' as i32 {
                        count_0 += 1;
                        count_0;
                    } else if *line as i32 == ')' as i32 {
                        if count_0 <= 0 as i32 {
                            break;
                        }
                        count_0 -= 1;
                        count_0;
                    }
                    line = line.offset(1);
                    line;
                }
            } else {
                line = line.offset(1);
                line;
                s2 = line;
                while *line as i32 != '\0' as i32 && *line as i32 != termin as i32 {
                    line = line.offset(1);
                    line;
                }
            }
            if *line as i32 == '\0' as i32 {
                return -(1 as i32);
            }
            let fresh12 = line;
            line = line.offset(1);
            *fresh12 = '\0' as i32 as i8;
            while *stopchar_map.as_mut_ptr().offset(*line as u8 as isize) as i32
                & (0x2 as i32 | 0x4 as i32) != 0 as i32
            {
                line = line.offset(1);
                line;
            }
            if *line as i32 != '\0' as i32 {
                error(
                    flocp,
                    strlen(cmdname),
                    dcgettext(
                        0 as *const i8,
                        b"extraneous text after '%s' directive\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    cmdname,
                );
            }
            s2 = variable_expand(s2);
            *((*conditionals).ignoring).offset(o as isize) = ((s1 == s2
                || *s1 as i32 == *s2 as i32
                    && (*s1 as i32 == '\0' as i32
                        || strcmp(
                            s1.offset(1 as i32 as isize),
                            s2.offset(1 as i32 as isize),
                        ) == 0)) as i32
                == (cmdtype as u32 == C2RustUnnamed::c_ifneq as i32 as u32) as i32)
                as i32 as i8;
        }
    }
    i = 0 as i32 as u32;
    while i < (*conditionals).if_cmds {
        if *((*conditionals).ignoring).offset(i as isize) != 0 {
            return 1 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn record_target_var(
    mut filenames: *mut nameseq,
    mut defn: *mut i8,
    mut origin: variable_origin,
    mut vmod: *mut vmodifiers,
    mut flocp: *const floc,
) {
    let mut nextf: *mut nameseq = 0 as *mut nameseq;
    let mut global: *mut variable_set_list = 0 as *mut variable_set_list;
    global = current_variable_set_list;
    while !filenames.is_null() {
        let mut v: *mut variable = 0 as *mut variable;
        let mut name: *const i8 = (*filenames).name;
        let mut percent: *const i8 = 0 as *const i8;
        let mut p: *mut pattern_var = 0 as *mut pattern_var;
        nextf = (*filenames).next;
        free(filenames as *mut libc::c_void);
        percent = find_percent_cached(&mut name);
        if !percent.is_null() {
            p = create_pattern_var(name, percent);
            (*p).variable.fileinfo = *flocp;
            v = assign_variable_definition(&mut (*p).variable, defn);
            (*v).set_origin(origin);
            if (*v).flavor() as i32 == variable_flavor::f_simple as i32 {
                (*v).value = allocated_variable_expand_for_file(
                    (*v).value,
                    0 as *mut file,
                );
            } else {
                (*v).value = xstrdup((*v).value);
            }
        } else {
            let mut f: *mut file = 0 as *mut file;
            f = lookup_file(name);
            if f.is_null() {
                f = enter_file(strcache_add(name));
            } else if !((*f).double_colon).is_null() {
                f = (*f).double_colon;
            }
            initialize_file_variables(f, 1 as i32);
            current_variable_set_list = (*f).variables;
            v = try_variable_definition(flocp, defn, origin, 1 as i32);
            if v.is_null() {
                fatal(
                    flocp,
                    0 as i32 as size_t,
                    dcgettext(
                        0 as *const i8,
                        b"Malformed target-specific variable definition\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            current_variable_set_list = global;
        }
        (*v).set_per_target(1 as i32 as u32);
        (*v).set_private_var((*vmod).private_v());
        if (*vmod).export_v() as i32 != variable_export::v_default as i32 {
            (*v).set_export((*vmod).export_v());
        }
        if (*v).origin() as i32 != variable_origin::o_override as i32 {
            let mut gv: *mut variable = 0 as *mut variable;
            let mut len: size_t = strlen((*v).name);
            gv = lookup_variable((*v).name, len);
            if !gv.is_null() && v != gv
                && ((*gv).origin() as i32 == variable_origin::o_env_override as i32
                    || (*gv).origin() as i32 == variable_origin::o_command as i32)
            {
                free((*v).value as *mut libc::c_void);
                (*v).value = xstrdup((*gv).value);
                (*v).set_origin((*gv).origin());
                (*v).set_recursive((*gv).recursive());
                (*v).set_append(0 as i32 as u32);
            }
        }
        filenames = nextf;
    }
}
unsafe extern "C" fn check_specials(mut files: *mut nameseq, mut set_default: i32) {
    let mut t: *mut nameseq = 0 as *mut nameseq;
    t = files;
    while !t.is_null() {
        let mut nm: *const i8 = (*t).name;
        if posix_pedantic == 0
            && (nm == b".POSIX\0" as *const u8 as *const i8
                || *nm as i32 == *(b".POSIX\0" as *const u8 as *const i8) as i32
                    && (*nm as i32 == '\0' as i32
                        || strcmp(
                            nm.offset(1 as i32 as isize),
                            (b".POSIX\0" as *const u8 as *const i8)
                                .offset(1 as i32 as isize),
                        ) == 0))
        {
            posix_pedantic = 1 as i32;
            define_variable_in_set(
                b".SHELLFLAGS\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 12]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                b"-ec\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"CC\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
                b"c99\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"CFLAGS\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64),
                b"-O1\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"FC\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 3]>() as u64).wrapping_sub(1 as i32 as u64),
                b"fort77\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"FFLAGS\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64),
                b"-O1\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"SCCSGETFLAGS\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 13]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                b"-s\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"ARFLAGS\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64),
                b"-rv\0" as *const u8 as *const i8,
                variable_origin::o_default,
                0 as i32,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
        } else if second_expansion == 0
            && (nm == b".SECONDEXPANSION\0" as *const u8 as *const i8
                || *nm as i32
                    == *(b".SECONDEXPANSION\0" as *const u8 as *const i8) as i32
                    && (*nm as i32 == '\0' as i32
                        || strcmp(
                            nm.offset(1 as i32 as isize),
                            (b".SECONDEXPANSION\0" as *const u8 as *const i8)
                                .offset(1 as i32 as isize),
                        ) == 0))
        {
            second_expansion = 1 as i32;
        } else if one_shell == 0
            && (nm == b".ONESHELL\0" as *const u8 as *const i8
                || *nm as i32 == *(b".ONESHELL\0" as *const u8 as *const i8) as i32
                    && (*nm as i32 == '\0' as i32
                        || strcmp(
                            nm.offset(1 as i32 as isize),
                            (b".ONESHELL\0" as *const u8 as *const i8)
                                .offset(1 as i32 as isize),
                        ) == 0))
        {
            one_shell = 1 as i32;
        } else if set_default != 0
            && *((*default_goal_var).value).offset(0 as i32 as isize) as i32
                == '\0' as i32
        {
            let mut d: *mut dep = 0 as *mut dep;
            let mut reject: i32 = 0 as i32;
            if !(strchr(nm, '%' as i32)).is_null() {
                break;
            }
            if !(*nm as i32 == '.' as i32 && (strchr(nm, '/' as i32)).is_null()) {
                d = (*suffix_file).deps;
                while !d.is_null() {
                    let mut d2: *mut dep = 0 as *mut dep;
                    if *(if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as i32 != '.' as i32
                        && (nm
                            == (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            })
                            || *nm as i32
                                == *(if !((*d).name).is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                }) as i32
                                && (*nm as i32 == '\0' as i32
                                    || strcmp(
                                        nm.offset(1 as i32 as isize),
                                        (if !((*d).name).is_null() {
                                            (*d).name
                                        } else {
                                            (*(*d).file).name
                                        })
                                            .offset(1 as i32 as isize),
                                    ) == 0))
                    {
                        reject = 1 as i32;
                        break;
                    } else {
                        d2 = (*suffix_file).deps;
                        while !d2.is_null() {
                            let mut l: size_t = strlen(
                                if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                            );
                            if strncmp(
                                nm,
                                (if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                }),
                                l,
                            ) == 0 as i32
                            {
                                if nm.offset(l as isize)
                                    == (if !((*d).name).is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    })
                                    || *nm.offset(l as isize) as i32
                                        == *(if !((*d).name).is_null() {
                                            (*d).name
                                        } else {
                                            (*(*d).file).name
                                        }) as i32
                                        && (*nm.offset(l as isize) as i32 == '\0' as i32
                                            || strcmp(
                                                nm.offset(l as isize).offset(1 as i32 as isize),
                                                (if !((*d).name).is_null() {
                                                    (*d).name
                                                } else {
                                                    (*(*d).file).name
                                                })
                                                    .offset(1 as i32 as isize),
                                            ) == 0)
                                {
                                    reject = 1 as i32;
                                    break;
                                }
                            }
                            d2 = (*d2).next;
                        }
                        if reject != 0 {
                            break;
                        }
                        d = (*d).next;
                    }
                }
                if reject == 0 {
                    define_variable_in_set(
                        b".DEFAULT_GOAL\0" as *const u8 as *const i8,
                        13 as i32 as size_t,
                        (*t).name,
                        variable_origin::o_file,
                        0 as i32,
                        0 as *mut variable_set,
                        0 as *mut floc,
                    );
                }
            }
        }
        t = (*t).next;
    }
}
unsafe extern "C" fn check_special_file(mut file: *mut file, mut flocp: *const floc) {
    if (*file).name == b".WAIT\0" as *const u8 as *const i8
        || *(*file).name as i32 == *(b".WAIT\0" as *const u8 as *const i8) as i32
            && (*(*file).name as i32 == '\0' as i32
                || strcmp(
                    ((*file).name).offset(1 as i32 as isize),
                    (b".WAIT\0" as *const u8 as *const i8).offset(1 as i32 as isize),
                ) == 0)
    {
        static mut wpre: u32 = 0 as i32 as u32;
        static mut wcmd: u32 = 0 as i32 as u32;
        if wpre == 0 && !((*file).deps).is_null() {
            error(
                flocp,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b".WAIT should not have prerequisites\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            wpre = 1 as i32 as u32;
        }
        if wcmd == 0 && !((*file).cmds).is_null() {
            error(
                flocp,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b".WAIT should not have commands\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            wcmd = 1 as i32 as u32;
        }
        return;
    }
}
unsafe extern "C" fn record_files(
    mut filenames: *mut nameseq,
    mut are_also_makes: i32,
    mut pattern: *const i8,
    mut pattern_percent: *const i8,
    mut depstr: *mut i8,
    mut cmds_started: u32,
    mut commands: *mut i8,
    mut commands_idx: size_t,
    mut two_colon: i32,
    mut prefix: i8,
    mut flocp: *const floc,
) {
    let mut cmds: *mut commands = 0 as *mut commands;
    let mut deps: *mut dep = 0 as *mut dep;
    let mut also_make: *mut dep = 0 as *mut dep;
    let mut implicit_percent: *const i8 = 0 as *const i8;
    let mut name: *const i8 = 0 as *const i8;
    if snapped_deps != 0 {
        fatal(
            flocp,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"prerequisites cannot be defined in recipes\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    name = (*filenames).name;
    implicit_percent = find_percent_cached(&mut name);
    if commands_idx > 0 as i32 as u64 {
        cmds = xmalloc(::core::mem::size_of::<commands>() as u64) as *mut commands;
        (*cmds).fileinfo.filenm = (*flocp).filenm;
        (*cmds).fileinfo.lineno = cmds_started as u64;
        (*cmds).fileinfo.offset = 0 as i32 as u64;
        (*cmds).commands = xstrndup(commands, commands_idx);
        (*cmds).command_lines = 0 as *mut *mut i8;
        (*cmds).recipe_prefix = prefix;
    } else if are_also_makes != 0 {
        fatal(
            flocp,
            0 as i32 as size_t,
            dcgettext(
                0 as *const i8,
                b"grouped targets must provide a recipe\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        cmds = 0 as *mut commands;
    }
    if depstr.is_null() {
        deps = 0 as *mut dep;
    } else {
        depstr = unescape_char(depstr, ':' as i32);
        if second_expansion != 0 && !(strchr(depstr, '$' as i32)).is_null() {
            deps = xcalloc(::core::mem::size_of::<dep>() as u64) as *mut dep;
            (*deps).name = depstr;
            (*deps).set_need_2nd_expansion(1 as i32 as u32);
            (*deps).set_staticpattern((pattern != 0 as *const i8) as i32 as u32);
        } else {
            deps = split_prereqs(depstr);
            free(depstr as *mut libc::c_void);
            if pattern.is_null() && implicit_percent.is_null() {
                deps = enter_prereqs(deps, 0 as *const i8);
            }
        }
    }
    if !implicit_percent.is_null() {
        let mut nextf: *mut nameseq = 0 as *mut nameseq;
        let mut targets: *mut *const i8 = 0 as *mut *const i8;
        let mut target_pats: *mut *const i8 = 0 as *mut *const i8;
        let mut c: libc::c_ushort = 0;
        if !pattern.is_null() {
            fatal(
                flocp,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"mixed implicit and static pattern rules\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        nextf = (*filenames).next;
        free(filenames as *mut libc::c_void);
        filenames = nextf;
        c = 1 as i32 as libc::c_ushort;
        while !nextf.is_null() {
            c = c.wrapping_add(1);
            c;
            nextf = (*nextf).next;
        }
        targets = xmalloc(
            (c as u64).wrapping_mul(::core::mem::size_of::<*const i8>() as u64),
        ) as *mut *const i8;
        target_pats = xmalloc(
            (c as u64).wrapping_mul(::core::mem::size_of::<*const i8>() as u64),
        ) as *mut *const i8;
        let ref mut fresh13 = *targets.offset(0 as i32 as isize);
        *fresh13 = name;
        let ref mut fresh14 = *target_pats.offset(0 as i32 as isize);
        *fresh14 = implicit_percent;
        c = 1 as i32 as libc::c_ushort;
        while !filenames.is_null() {
            name = (*filenames).name;
            implicit_percent = find_percent_cached(&mut name);
            if implicit_percent.is_null() {
                fatal(
                    flocp,
                    0 as i32 as size_t,
                    dcgettext(
                        0 as *const i8,
                        b"mixed implicit and normal rules\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            let ref mut fresh15 = *targets.offset(c as isize);
            *fresh15 = name;
            let ref mut fresh16 = *target_pats.offset(c as isize);
            *fresh16 = implicit_percent;
            c = c.wrapping_add(1);
            c;
            nextf = (*filenames).next;
            free(filenames as *mut libc::c_void);
            filenames = nextf;
        }
        create_pattern_rule(targets, target_pats, c, two_colon, deps, cmds, 1 as i32);
        return;
    }
    loop {
        let mut nextf_0: *mut nameseq = (*filenames).next;
        let mut f: *mut file = 0 as *mut file;
        let mut this: *mut dep = 0 as *mut dep;
        free(filenames as *mut libc::c_void);
        if !pattern.is_null() && pattern_matches(pattern, pattern_percent, name) == 0 {
            error(
                flocp,
                strlen(name),
                dcgettext(
                    0 as *const i8,
                    b"target '%s' doesn't match the target pattern\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                name,
            );
        } else if !deps.is_null() {
            this = if !nextf_0.is_null() { copy_dep_chain(deps) } else { deps };
        }
        if two_colon == 0 {
            f = enter_file(strcache_add(name));
            if !((*f).double_colon).is_null() {
                fatal(
                    flocp,
                    strlen((*f).name),
                    dcgettext(
                        0 as *const i8,
                        b"target file '%s' has both : and :: entries\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*f).name,
                );
            }
            if !cmds.is_null() && cmds == (*f).cmds {
                error(
                    flocp,
                    strlen((*f).name),
                    dcgettext(
                        0 as *const i8,
                        b"target '%s' given more than once in the same rule\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*f).name,
                );
            } else if !cmds.is_null() && !((*f).cmds).is_null()
                && (*f).is_target() as i32 != 0
            {
                let mut l: size_t = strlen((*f).name);
                error(
                    &mut (*cmds).fileinfo as *mut floc,
                    l,
                    dcgettext(
                        0 as *const i8,
                        b"warning: overriding recipe for target '%s'\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*f).name,
                );
                error(
                    &mut (*(*f).cmds).fileinfo as *mut floc,
                    l,
                    dcgettext(
                        0 as *const i8,
                        b"warning: ignoring old recipe for target '%s'\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*f).name,
                );
            }
            if f == default_file && this.is_null() && cmds.is_null() {
                (*f).cmds = 0 as *mut commands;
            }
            if !cmds.is_null() {
                (*f).cmds = cmds;
            }
            if f == suffix_file && this.is_null() {
                free_ns_chain((*f).deps as *mut nameseq);
                (*f).deps = 0 as *mut dep;
            }
            (*f).set_is_explicit(1 as i32 as u32);
        } else {
            f = lookup_file(name);
            if !f.is_null() && (*f).is_target() as i32 != 0
                && ((*f).double_colon).is_null()
            {
                fatal(
                    flocp,
                    strlen((*f).name),
                    dcgettext(
                        0 as *const i8,
                        b"target file '%s' has both : and :: entries\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*f).name,
                );
            }
            f = enter_file(strcache_add(name));
            if ((*f).double_colon).is_null() {
                (*f).double_colon = f;
            }
            (*f).cmds = cmds;
        }
        if are_also_makes != 0 {
            let mut also: *mut dep = xcalloc(::core::mem::size_of::<dep>() as u64)
                as *mut dep;
            (*also).name = (*f).name;
            (*also).file = f;
            (*also).next = also_make;
            also_make = also;
        }
        (*f).set_is_target(1 as i32 as u32);
        if !pattern.is_null() {
            static mut percent: *const i8 = b"%\0" as *const u8 as *const i8;
            let mut o: *mut i8 = patsubst_expand_pat(
                variable_buffer,
                name,
                pattern,
                percent,
                pattern_percent.offset(1 as i32 as isize),
                percent.offset(1 as i32 as isize),
            );
            (*f).stem = strcache_add_len(
                variable_buffer,
                o.offset_from(variable_buffer) as i64 as size_t,
            );
            if !this.is_null() {
                if (*this).need_2nd_expansion() == 0 {
                    this = enter_prereqs(this, (*f).stem);
                } else {
                    (*this).stem = (*f).stem;
                }
            }
        }
        if !this.is_null() {
            if ((*f).deps).is_null() {
                (*f).deps = this;
            } else if !cmds.is_null() {
                let mut d: *mut dep = this;
                while !((*d).next).is_null() {
                    d = (*d).next;
                }
                (*d).next = (*f).deps;
                (*f).deps = this;
            } else {
                let mut d_0: *mut dep = (*f).deps;
                while !((*d_0).next).is_null() {
                    d_0 = (*d_0).next;
                }
                (*d_0).next = this;
            }
        }
        name = (*f).name;
        check_special_file(f, flocp);
        if nextf_0.is_null() {
            break;
        }
        filenames = nextf_0;
        name = (*filenames).name;
        if !(find_percent_cached(&mut name)).is_null() {
            error(
                flocp,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"*** mixed implicit and normal rules: deprecated syntax\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    let mut i: *mut dep = 0 as *mut dep;
    i = also_make;
    while !i.is_null() {
        let mut f_0: *mut file = (*i).file;
        let mut cpy: *mut dep = if !((*i).next).is_null() {
            copy_dep_chain(also_make)
        } else {
            also_make
        };
        if !((*f_0).also_make).is_null() {
            error(
                &mut (*cmds).fileinfo as *mut floc,
                strlen((*f_0).name),
                dcgettext(
                    0 as *const i8,
                    b"warning: overriding group membership for target '%s'\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*f_0).name,
            );
            free_ns_chain((*f_0).also_make as *mut nameseq);
        }
        (*f_0).also_make = cpy;
        i = (*i).next;
    }
}
unsafe extern "C" fn find_map_unquote(mut string: *mut i8, mut stopmap: i32) -> *mut i8 {
    let mut string_len: size_t = 0 as i32 as size_t;
    let mut p: *mut i8 = string;
    stopmap |= 0x1 as i32;
    loop {
        while !(*stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & stopmap
            != 0 as i32)
        {
            p = p.offset(1);
            p;
        }
        if *p as i32 == '\0' as i32 {
            break;
        }
        if *p as i32 == '$' as i32 {
            let mut openparen: i8 = *p.offset(1 as i32 as isize);
            if openparen as i32 == '\0' as i32 {
                break;
            }
            p = p.offset(2 as i32 as isize);
            if openparen as i32 == '(' as i32 || openparen as i32 == '{' as i32 {
                let mut pcount: u32 = 1 as i32 as u32;
                let mut closeparen: i8 = (if openparen as i32 == '(' as i32 {
                    ')' as i32
                } else {
                    '}' as i32
                }) as i8;
                while *p != 0 {
                    if *p as i32 == openparen as i32 {
                        pcount = pcount.wrapping_add(1);
                        pcount;
                    } else if *p as i32 == closeparen as i32 {
                        pcount = pcount.wrapping_sub(1);
                        if pcount == 0 as i32 as u32 {
                            p = p.offset(1);
                            p;
                            break;
                        }
                    }
                    p = p.offset(1);
                    p;
                }
            }
        } else if p > string && *p.offset(-(1 as i32) as isize) as i32 == '\\' as i32 {
            let mut i: i32 = -(2 as i32);
            while &mut *p.offset(i as isize) as *mut i8 >= string
                && *p.offset(i as isize) as i32 == '\\' as i32
            {
                i -= 1;
                i;
            }
            i += 1;
            i;
            if string_len == 0 as i32 as u64 {
                string_len = strlen(string);
            }
            let mut hi: i32 = -(i / 2 as i32);
            memmove(
                &mut *p.offset(i as isize) as *mut i8 as *mut libc::c_void,
                &mut *p.offset((i / 2 as i32) as isize) as *mut i8
                    as *const libc::c_void,
                string_len
                    .wrapping_sub(p.offset_from(string) as i64 as u64)
                    .wrapping_add(hi as u64)
                    .wrapping_add(1 as i32 as u64),
            );
            p = p.offset((i / 2 as i32) as isize);
            if i % 2 as i32 == 0 as i32 {
                return p;
            }
        } else {
            return p
        }
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn find_char_unquote(mut string: *mut i8, mut stop: i32) -> *mut i8 {
    let mut string_len: size_t = 0 as i32 as size_t;
    let mut p: *mut i8 = string;
    loop {
        p = strchr(p, stop);
        if p.is_null() {
            return 0 as *mut i8;
        }
        if p > string && *p.offset(-(1 as i32) as isize) as i32 == '\\' as i32 {
            let mut i: i32 = -(2 as i32);
            while &mut *p.offset(i as isize) as *mut i8 >= string
                && *p.offset(i as isize) as i32 == '\\' as i32
            {
                i -= 1;
                i;
            }
            i += 1;
            i;
            if string_len == 0 as i32 as u64 {
                string_len = strlen(string);
            }
            let mut hi: i32 = -(i / 2 as i32);
            memmove(
                &mut *p.offset(i as isize) as *mut i8 as *mut libc::c_void,
                &mut *p.offset((i / 2 as i32) as isize) as *mut i8
                    as *const libc::c_void,
                string_len
                    .wrapping_sub(p.offset_from(string) as i64 as u64)
                    .wrapping_add(hi as u64)
                    .wrapping_add(1 as i32 as u64),
            );
            p = p.offset((i / 2 as i32) as isize);
            if i % 2 as i32 == 0 as i32 {
                return p;
            }
        } else {
            return p
        }
    };
}
unsafe extern "C" fn unescape_char(mut string: *mut i8, mut c: i32) -> *mut i8 {
    let mut p: *mut i8 = string;
    let mut s: *mut i8 = string;
    while *s as i32 != '\0' as i32 {
        if *s as i32 == '\\' as i32 {
            let mut e: *mut i8 = s;
            let mut l: size_t = 0;
            while *e as i32 == '\\' as i32 {
                e = e.offset(1);
                e;
            }
            l = e.offset_from(s) as i64 as size_t;
            if *e as i32 != c || l.wrapping_rem(2 as i32 as u64) == 0 as i32 as u64 {
                memmove(p as *mut libc::c_void, s as *const libc::c_void, l);
                p = p.offset(l as isize);
                if *e as i32 == '\0' as i32 {
                    break;
                }
            } else if l > 1 as i32 as u64 {
                l = (l as u64).wrapping_div(2 as i32 as u64) as size_t as size_t;
                memmove(p as *mut libc::c_void, s as *const libc::c_void, l);
                p = p.offset(l as isize);
            }
            s = e;
        }
        let fresh17 = s;
        s = s.offset(1);
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = *fresh17;
    }
    *p = '\0' as i32 as i8;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn find_percent(mut pattern: *mut i8) -> *mut i8 {
    return find_char_unquote(pattern, '%' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn find_percent_cached(mut string: *mut *const i8) -> *const i8 {
    let mut p: *const i8 = strchr(*string, '%' as i32);
    let mut new: *mut i8 = 0 as *mut i8;
    let mut np: *mut i8 = 0 as *mut i8;
    let mut slen: size_t = 0;
    if p.is_null() || p == *string
        || *p.offset(-(1 as i32) as isize) as i32 != '\\' as i32
    {
        return p;
    }
    slen = strlen(*string);
    let mut fresh19 = ::std::vec::from_elem(
        0,
        slen.wrapping_add(1 as i32 as u64) as usize,
    );
    new = fresh19.as_mut_ptr() as *mut i8;
    memcpy(
        new as *mut libc::c_void,
        *string as *const libc::c_void,
        slen.wrapping_add(1 as i32 as u64),
    );
    np = new.offset(p.offset_from(*string) as i64 as isize);
    loop {
        let mut pp: *mut i8 = np;
        let mut i: i32 = -(2 as i32);
        while &mut *np.offset(i as isize) as *mut i8 >= new
            && *np.offset(i as isize) as i32 == '\\' as i32
        {
            i -= 1;
            i;
        }
        i += 1;
        i;
        let mut hi: i32 = -(i / 2 as i32);
        memmove(
            &mut *pp.offset(i as isize) as *mut i8 as *mut libc::c_void,
            &mut *pp.offset((i / 2 as i32) as isize) as *mut i8 as *const libc::c_void,
            slen
                .wrapping_sub(pp.offset_from(new) as i64 as u64)
                .wrapping_add(hi as u64)
                .wrapping_add(1 as i32 as u64),
        );
        slen = (slen as u64).wrapping_add((i / 2 as i32 + i % 2 as i32) as u64) as size_t
            as size_t;
        np = np.offset((i / 2 as i32) as isize);
        if i % 2 as i32 == 0 as i32 {
            break;
        }
        np = strchr(np, '%' as i32);
        if !(!np.is_null() && *np.offset(-(1 as i32) as isize) as i32 == '\\' as i32) {
            break;
        }
    }
    *string = strcache_add(new);
    return if !np.is_null() {
        (*string).offset(np.offset_from(new) as i64 as isize)
    } else {
        0 as *const i8
    };
}
unsafe extern "C" fn readstring(mut ebuf: *mut ebuffer) -> i64 {
    let mut eol: *mut i8 = 0 as *mut i8;
    if (*ebuf).bufnext >= ((*ebuf).bufstart).offset((*ebuf).size as isize) {
        return -(1 as i32) as i64;
    }
    (*ebuf).buffer = (*ebuf).bufnext;
    eol = (*ebuf).buffer;
    loop {
        let mut backslash: i32 = 0 as i32;
        let mut bol: *const i8 = eol;
        let mut p: *const i8 = 0 as *const i8;
        eol = strchr(eol, '\n' as i32);
        p = eol;
        if eol.is_null() {
            (*ebuf).bufnext = ((*ebuf).bufstart)
                .offset((*ebuf).size as isize)
                .offset(1 as i32 as isize);
            return 0 as i32 as i64;
        }
        while p > bol
            && {
                p = p.offset(-1);
                *p as i32 == '\\' as i32
            }
        {
            backslash = (backslash == 0) as i32;
        }
        if backslash == 0 {
            break;
        }
        eol = eol.offset(1);
        eol;
    }
    *eol = '\0' as i32 as i8;
    (*ebuf).bufnext = eol.offset(1 as i32 as isize);
    return 0 as i32 as i64;
}
unsafe extern "C" fn readline(mut ebuf: *mut ebuffer) -> i64 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut nlines: i64 = 0 as i32 as i64;
    if ((*ebuf).fp).is_null() {
        return readstring(ebuf);
    }
    start = (*ebuf).bufstart;
    p = start;
    end = p.offset((*ebuf).size as isize);
    *p = '\0' as i32 as i8;
    while !(fgets(p, end.offset_from(p) as i64 as i32, (*ebuf).fp)).is_null() {
        let mut p2: *mut i8 = 0 as *mut i8;
        let mut len: size_t = 0;
        let mut backslash: i32 = 0;
        len = strlen(p);
        if len == 0 as i32 as u64 {
            error(
                &mut (*ebuf).floc as *mut floc,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"warning: NUL character seen; rest of line ignored\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            *p.offset(0 as i32 as isize) = '\n' as i32 as i8;
            len = 1 as i32 as size_t;
        }
        p = p.offset(len as isize);
        if !(*p.offset(-(1 as i32) as isize) as i32 != '\n' as i32) {
            nlines += 1;
            nlines;
            if p.offset_from(start) as i64 > 1 as i32 as i64
                && *p.offset(-(2 as i32) as isize) as i32 == '\r' as i32
            {
                p = p.offset(-1);
                p;
                memmove(
                    p.offset(-(1 as i32 as isize)) as *mut libc::c_void,
                    p as *const libc::c_void,
                    (strlen(p)).wrapping_add(1 as i32 as u64),
                );
            }
            backslash = 0 as i32;
            p2 = p.offset(-(2 as i32 as isize));
            while p2 >= start {
                if *p2 as i32 != '\\' as i32 {
                    break;
                }
                backslash = (backslash == 0) as i32;
                p2 = p2.offset(-1);
                p2;
            }
            if backslash == 0 {
                *p.offset(-(1 as i32) as isize) = '\0' as i32 as i8;
                break;
            } else if end.offset_from(p) as i64 >= 80 as i32 as i64 {
                continue;
            }
        }
        let mut off: size_t = p.offset_from(start) as i64 as size_t;
        (*ebuf).size = ((*ebuf).size as u64).wrapping_mul(2 as i32 as u64) as size_t
            as size_t;
        (*ebuf).bufstart = xrealloc(start as *mut libc::c_void, (*ebuf).size) as *mut i8;
        (*ebuf).buffer = (*ebuf).bufstart;
        start = (*ebuf).buffer;
        p = start.offset(off as isize);
        end = start.offset((*ebuf).size as isize);
        *p = '\0' as i32 as i8;
    }
    if ferror((*ebuf).fp) != 0 {
        pfatal_with_name((*ebuf).floc.filenm);
    }
    return if nlines != 0 {
        nlines
    } else {
        (if p == (*ebuf).bufstart { -(1 as i32) } else { 1 as i32 }) as i64
    };
}
unsafe extern "C" fn get_next_mword(
    mut buffer: *mut i8,
    mut startp: *mut *mut i8,
    mut length: *mut size_t,
) -> make_word_type {
    let mut current_block: u64;
    let mut wtype: make_word_type = make_word_type::w_bogus;
    let mut p: *mut i8 = buffer;
    let mut beg: *mut i8 = 0 as *mut i8;
    let mut c: i8 = 0;
    while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        p = p.offset(1);
        p;
    }
    beg = p;
    let fresh20 = p;
    p = p.offset(1);
    c = *fresh20;
    match c as i32 {
        0 => {
            wtype = make_word_type::w_eol;
            current_block = 11017783287309324887;
        }
        59 => {
            wtype = make_word_type::w_semicolon;
            current_block = 11017783287309324887;
        }
        61 => {
            wtype = make_word_type::w_varassign;
            current_block = 11017783287309324887;
        }
        58 => {
            if *p as i32 == '=' as i32 {
                p = p.offset(1);
                p;
                wtype = make_word_type::w_varassign;
            } else if *p as i32 == ':' as i32 {
                p = p.offset(1);
                p;
                if *p.offset(1 as i32 as isize) as i32 == '=' as i32 {
                    p = p.offset(1);
                    p;
                    wtype = make_word_type::w_varassign;
                } else {
                    wtype = make_word_type::w_dcolon;
                }
            } else {
                wtype = make_word_type::w_colon;
            }
            current_block = 11017783287309324887;
        }
        38 => {
            if *p as i32 == ':' as i32 {
                p = p.offset(1);
                p;
                if *p as i32 != ':' as i32 {
                    wtype = make_word_type::w_ampcolon;
                } else {
                    p = p.offset(1);
                    p;
                    wtype = make_word_type::w_ampdcolon;
                }
                current_block = 11017783287309324887;
            } else {
                current_block = 15897653523371991391;
            }
        }
        43 | 63 | 33 => {
            if *p as i32 == '=' as i32 {
                p = p.offset(1);
                p;
                wtype = make_word_type::w_varassign;
                current_block = 11017783287309324887;
            } else {
                current_block = 15897653523371991391;
            }
        }
        _ => {
            current_block = 15897653523371991391;
        }
    }
    match current_block {
        15897653523371991391 => {
            wtype = make_word_type::w_static;
            loop {
                let mut closeparen: i8 = 0;
                let mut count: i32 = 0;
                if *stopchar_map.as_mut_ptr().offset(c as u8 as isize) as i32
                    & (0x2 as i32 | 0x4 as i32 | 0x1 as i32) != 0 as i32
                {
                    break;
                }
                match c as i32 {
                    61 | 58 => {
                        break;
                    }
                    36 => {
                        let fresh21 = p;
                        p = p.offset(1);
                        c = *fresh21;
                        if !(c as i32 == '$' as i32) {
                            if c as i32 == '\0' as i32 {
                                break;
                            }
                            wtype = make_word_type::w_variable;
                            if c as i32 == '(' as i32 {
                                closeparen = ')' as i32 as i8;
                                current_block = 790185930182612747;
                            } else if c as i32 == '{' as i32 {
                                closeparen = '}' as i32 as i8;
                                current_block = 790185930182612747;
                            } else {
                                current_block = 9512719473022792396;
                            }
                            match current_block {
                                9512719473022792396 => {}
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
                    63 | 43 => {
                        if *p as i32 == '=' as i32 {
                            break;
                        }
                    }
                    92 => {
                        match *p as i32 {
                            58 | 59 | 61 | 92 => {
                                p = p.offset(1);
                                p;
                            }
                            _ => {}
                        }
                    }
                    38 => {
                        if *p as i32 == ':' as i32 {
                            break;
                        }
                    }
                    _ => {}
                }
                let fresh22 = p;
                p = p.offset(1);
                c = *fresh22;
            }
            p = p.offset(-1);
            p;
        }
        _ => {}
    }
    if !startp.is_null() {
        *startp = beg;
    }
    if !length.is_null() {
        *length = p.offset_from(beg) as i64 as size_t;
    }
    return wtype;
}
#[no_mangle]
pub unsafe extern "C" fn construct_include_path(mut arg_dirs: *mut *const i8) {
    let mut stbuf: stat = stat {
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
    let mut dirs: *mut *const i8 = 0 as *mut *const i8;
    let mut cpp: *mut *const i8 = 0 as *mut *const i8;
    let mut idx: size_t = 0;
    let mut disable: i32 = 0 as i32;
    idx = (::core::mem::size_of::<[*const i8; 4]>() as u64)
        .wrapping_div(::core::mem::size_of::<*const i8>() as u64);
    if !arg_dirs.is_null() {
        cpp = arg_dirs;
        while !(*cpp).is_null() {
            idx = idx.wrapping_add(1);
            idx;
            cpp = cpp.offset(1);
            cpp;
        }
    }
    dirs = xmalloc(idx.wrapping_mul(::core::mem::size_of::<*const i8>() as u64))
        as *mut *const i8;
    idx = 0 as i32 as size_t;
    max_incl_len = 0 as i32 as size_t;
    if !arg_dirs.is_null() {
        while !(*arg_dirs).is_null() {
            let fresh23 = arg_dirs;
            arg_dirs = arg_dirs.offset(1);
            let mut dir: *const i8 = *fresh23;
            let mut expanded: *mut i8 = 0 as *mut i8;
            let mut e: i32 = 0;
            if *dir.offset(0 as i32 as isize) as i32 == '-' as i32
                && *dir.offset(1 as i32 as isize) as i32 == '\0' as i32
            {
                disable = 1 as i32;
                idx = 0 as i32 as size_t;
                max_incl_len = 0 as i32 as size_t;
            } else {
                if *dir.offset(0 as i32 as isize) as i32 == '~' as i32 {
                    expanded = tilde_expand(dir);
                    if !expanded.is_null() {
                        dir = expanded;
                    }
                }
                loop {
                    e = stat(dir, &mut stbuf);
                    if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if e == 0 as i32
                    && stbuf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
                {
                    let mut len: size_t = strlen(dir);
                    while len > 1 as i32 as u64
                        && *dir.offset(len.wrapping_sub(1 as i32 as u64) as isize) as i32
                            == '/' as i32
                    {
                        len = len.wrapping_sub(1);
                        len;
                    }
                    if len > max_incl_len {
                        max_incl_len = len;
                    }
                    let fresh24 = idx;
                    idx = idx.wrapping_add(1);
                    let ref mut fresh25 = *dirs.offset(fresh24 as isize);
                    *fresh25 = strcache_add_len(dir, len);
                }
                free(expanded as *mut libc::c_void);
            }
        }
    }
    if disable == 0 {
        cpp = default_include_directories.as_mut_ptr();
        while !(*cpp).is_null() {
            let mut e_0: i32 = 0;
            loop {
                e_0 = stat(*cpp, &mut stbuf);
                if !(e_0 == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if e_0 == 0 as i32
                && stbuf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32
            {
                let mut len_0: size_t = strlen(*cpp);
                while len_0 > 1 as i32 as u64
                    && *(*cpp).offset(len_0.wrapping_sub(1 as i32 as u64) as isize)
                        as i32 == '/' as i32
                {
                    len_0 = len_0.wrapping_sub(1);
                    len_0;
                }
                if len_0 > max_incl_len {
                    max_incl_len = len_0;
                }
                let fresh26 = idx;
                idx = idx.wrapping_add(1);
                let ref mut fresh27 = *dirs.offset(fresh26 as isize);
                *fresh27 = strcache_add_len(*cpp, len_0);
            }
            cpp = cpp.offset(1);
            cpp;
        }
    }
    let ref mut fresh28 = *dirs.offset(idx as isize);
    *fresh28 = 0 as *const i8;
    do_variable_definition(
        0 as *mut floc,
        b".INCLUDE_DIRS\0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        variable_origin::o_default,
        variable_flavor::f_simple,
        0 as i32,
    );
    cpp = dirs;
    while !(*cpp).is_null() {
        do_variable_definition(
            0 as *mut floc,
            b".INCLUDE_DIRS\0" as *const u8 as *const i8,
            *cpp,
            variable_origin::o_default,
            variable_flavor::f_append,
            0 as i32,
        );
        cpp = cpp.offset(1);
        cpp;
    }
    free(include_directories as *mut libc::c_void);
    include_directories = dirs;
}
#[no_mangle]
pub unsafe extern "C" fn tilde_expand(mut name: *const i8) -> *mut i8 {
    if *name.offset(1 as i32 as isize) as i32 == '/' as i32
        || *name.offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        let mut home_dir: *mut i8 = 0 as *mut i8;
        let mut is_variable: i32 = 0;
        let mut save: i32 = warn_undefined_variables_flag;
        warn_undefined_variables_flag = 0 as i32;
        home_dir = allocated_variable_expand_for_file(
            b"$(HOME)\0" as *const u8 as *const i8,
            0 as *mut file,
        );
        warn_undefined_variables_flag = save;
        is_variable = (*home_dir.offset(0 as i32 as isize) as i32 != '\0' as i32) as i32;
        if is_variable == 0 {
            free(home_dir as *mut libc::c_void);
            home_dir = getenv(b"HOME\0" as *const u8 as *const i8);
        }
        if home_dir.is_null()
            || *home_dir.offset(0 as i32 as isize) as i32 == '\0' as i32
        {
            let mut logname: *mut i8 = getlogin();
            home_dir = 0 as *mut i8;
            if !logname.is_null() {
                let mut p: *mut passwd = getpwnam(logname);
                if !p.is_null() {
                    home_dir = (*p).pw_dir;
                }
            }
        }
        if !home_dir.is_null() {
            let mut new: *mut i8 = xstrdup(
                concat(2 as i32 as u32, home_dir, name.offset(1 as i32 as isize)),
            );
            if is_variable != 0 {
                free(home_dir as *mut libc::c_void);
            }
            return new;
        }
    } else {
        let mut pwent: *mut passwd = 0 as *mut passwd;
        let mut userend: *mut i8 = strchr(name.offset(1 as i32 as isize), '/' as i32);
        if !userend.is_null() {
            *userend = '\0' as i32 as i8;
        }
        pwent = getpwnam(name.offset(1 as i32 as isize));
        if !pwent.is_null() {
            if userend.is_null() {
                return xstrdup((*pwent).pw_dir);
            }
            *userend = '/' as i32 as i8;
            return xstrdup(
                concat(
                    3 as i32 as u32,
                    (*pwent).pw_dir,
                    b"/\0" as *const u8 as *const i8,
                    userend.offset(1 as i32 as isize),
                ),
            );
        } else if !userend.is_null() {
            *userend = '/' as i32 as i8;
        }
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn parse_file_seq(
    mut stringp: *mut *mut i8,
    mut size: size_t,
    mut stopmap: i32,
    mut prefix: *const i8,
    mut flags: i32,
) -> *mut libc::c_void {
    static mut tmpbuf: *mut i8 = 0 as *const i8 as *mut i8;
    let mut cachep: i32 = !(flags & 0x10 as i32 != 0 as i32) as i32;
    let mut new: *mut nameseq = 0 as *mut nameseq;
    let mut newp: *mut *mut nameseq = &mut new;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut gl: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut i8,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut tp: *mut i8 = 0 as *mut i8;
    let mut findmap: i32 = stopmap | 0 as i32 | 0x1 as i32;
    let mut found_wait: i32 = 0 as i32;
    if !(flags & 0x20 as i32 != 0 as i32) {
        findmap |= 0x2 as i32;
    }
    stopmap |= 0x1 as i32;
    if size < ::core::mem::size_of::<nameseq>() as u64 {
        size = ::core::mem::size_of::<nameseq>() as u64;
    }
    if !(flags & 0x4 as i32 != 0 as i32) {
        dir_setup_glob(&mut gl);
    }
    static mut tmpbuf_len: size_t = 0 as i32 as size_t;
    let mut l: size_t = (strlen(*stringp)).wrapping_add(1 as i32 as u64);
    if l > tmpbuf_len {
        tmpbuf = xrealloc(tmpbuf as *mut libc::c_void, l) as *mut i8;
        tmpbuf_len = l;
    }
    tp = tmpbuf;
    p = *stringp;
    loop {
        let mut name: *const i8 = 0 as *const i8;
        let mut nlist: *mut *const i8 = 0 as *mut *const i8;
        let mut tildep: *mut i8 = 0 as *mut i8;
        let mut globme: i32 = 1 as i32;
        let mut arname: *mut i8 = 0 as *mut i8;
        let mut memname: *mut i8 = 0 as *mut i8;
        let mut s: *mut i8 = 0 as *mut i8;
        let mut nlen: size_t = 0;
        let mut tot: i32 = 0;
        let mut i: i32 = 0;
        while *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32
            & (0x2 as i32 | 0x4 as i32) != 0 as i32
        {
            p = p.offset(1);
            p;
        }
        if *stopchar_map.as_mut_ptr().offset(*p as u8 as isize) as i32 & stopmap
            != 0 as i32
        {
            break;
        }
        s = p;
        p = find_map_unquote(p, findmap);
        if p.is_null() {
            p = s.offset(strlen(s) as isize);
        }
        if flags & 0x40 as i32 != 0 as i32
            && p.offset_from(s) as i64 as u64
                == (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
            && memcmp(
                s as *const libc::c_void,
                b".WAIT\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0 as i32
        {
            found_wait = 1 as i32;
        } else {
            if !(flags & 0x1 as i32 != 0 as i32) {
                while p.offset_from(s) as i64 > 2 as i32 as i64
                    && *s.offset(0 as i32 as isize) as i32 == '.' as i32
                    && *s.offset(1 as i32 as isize) as i32 == '/' as i32
                {
                    s = s.offset(2 as i32 as isize);
                    while *s as i32 == '/' as i32 {
                        s = s.offset(1);
                        s;
                    }
                }
            }
            if s == p {
                *tp.offset(0 as i32 as isize) = '.' as i32 as i8;
                *tp.offset(1 as i32 as isize) = '/' as i32 as i8;
                *tp.offset(2 as i32 as isize) = '\0' as i32 as i8;
                nlen = 2 as i32 as size_t;
            } else {
                nlen = p.offset_from(s) as i64 as size_t;
                memcpy(tp as *mut libc::c_void, s as *const libc::c_void, nlen);
                *tp.offset(nlen as isize) = '\0' as i32 as i8;
            }
            if !(flags & 0x2 as i32 != 0 as i32) && tp == tmpbuf
                && *tp.offset(0 as i32 as isize) as i32 != '(' as i32
                && *tp.offset(nlen.wrapping_sub(1 as i32 as u64) as isize) as i32
                    != ')' as i32
            {
                let mut n: *mut i8 = strchr(tp, '(' as i32);
                if !n.is_null() {
                    let mut e: *const i8 = p;
                    loop {
                        let mut o: *const i8 = e;
                        while *stopchar_map.as_mut_ptr().offset(*e as u8 as isize) as i32
                            & (0x2 as i32 | 0x4 as i32) != 0 as i32
                        {
                            e = e.offset(1);
                            e;
                        }
                        while !(*stopchar_map.as_mut_ptr().offset(*e as u8 as isize)
                            as i32 & findmap != 0 as i32)
                        {
                            e = e.offset(1);
                            e;
                        }
                        if e == o {
                            break;
                        }
                        if *e.offset(-(1 as i32) as isize) as i32 == ')' as i32 {
                            nlen = (nlen as u64)
                                .wrapping_sub(
                                    n.offset(1 as i32 as isize).offset_from(tp) as i64 as u64,
                                ) as size_t as size_t;
                            tp = n.offset(1 as i32 as isize);
                            break;
                        } else if !(*e as i32 != '\0' as i32) {
                            break;
                        }
                    }
                    if nlen == 0 {
                        continue;
                    }
                }
            }
            if tp > tmpbuf {
                if *tp.offset(nlen.wrapping_sub(1 as i32 as u64) as isize) as i32
                    == ')' as i32
                {
                    tp = tmpbuf;
                    if nlen == 1 as i32 as u64 {
                        continue;
                    }
                } else {
                    let fresh29 = nlen;
                    nlen = nlen.wrapping_add(1);
                    *tp.offset(fresh29 as isize) = ')' as i32 as i8;
                    *tp.offset(nlen as isize) = '\0' as i32 as i8;
                }
            }
            if flags & 0x4 as i32 != 0 as i32 {
                let mut _ns: *mut nameseq = xcalloc(size) as *mut nameseq;
                let mut __n: *const i8 = concat(2 as i32 as u32, prefix, tmpbuf);
                (*_ns).name = if cachep != 0 { strcache_add(__n) } else { xstrdup(__n) };
                if found_wait != 0 {
                    let ref mut fresh30 = *(_ns as *mut dep);
                    (*fresh30).set_wait_here(1 as i32 as u32);
                    found_wait = 0 as i32;
                }
                *newp = _ns;
                newp = &mut (*_ns).next;
            } else {
                name = tmpbuf;
                if *tmpbuf.offset(0 as i32 as isize) as i32 == '~' as i32 {
                    tildep = tilde_expand(tmpbuf);
                    if !tildep.is_null() {
                        name = tildep;
                    }
                }
                if !(flags & 0x2 as i32 != 0 as i32) && ar_name(name) != 0 {
                    ar_parse_name(name, &mut arname, &mut memname);
                    name = arname;
                }
                if !(flags & 0x8 as i32 != 0 as i32)
                    && (strpbrk(name, b"?*[\0" as *const u8 as *const i8)).is_null()
                {
                    globme = 0 as i32;
                    tot = 1 as i32;
                    nlist = &mut name;
                } else {
                    let mut current_block_76: u64;
                    match glob(name, (1 as i32) << 9 as i32, None, &mut gl) {
                        1 => {
                            out_of_memory();
                        }
                        0 => {
                            tot = gl.gl_pathc as i32;
                            nlist = gl.gl_pathv as *mut *const i8;
                            current_block_76 = 15864857819291709765;
                        }
                        3 => {
                            if flags & 0x8 as i32 != 0 as i32 {
                                tot = 0 as i32;
                                current_block_76 = 15864857819291709765;
                            } else {
                                current_block_76 = 3074230126654858580;
                            }
                        }
                        _ => {
                            current_block_76 = 3074230126654858580;
                        }
                    }
                    match current_block_76 {
                        3074230126654858580 => {
                            tot = 1 as i32;
                            nlist = &mut name;
                        }
                        _ => {}
                    }
                }
                i = 0 as i32;
                while i < tot {
                    if !memname.is_null() {
                        let mut found: *mut nameseq = ar_glob(
                            *nlist.offset(i as isize),
                            memname,
                            size,
                        );
                        if found.is_null() {
                            let mut _ns_0: *mut nameseq = xcalloc(size) as *mut nameseq;
                            let mut __n_0: *const i8 = concat(
                                5 as i32 as u32,
                                prefix,
                                *nlist.offset(i as isize),
                                b"(\0" as *const u8 as *const i8,
                                memname,
                                b")\0" as *const u8 as *const i8,
                            );
                            (*_ns_0).name = if cachep != 0 {
                                strcache_add(__n_0)
                            } else {
                                xstrdup(__n_0)
                            };
                            if found_wait != 0 {
                                let ref mut fresh31 = *(_ns_0 as *mut dep);
                                (*fresh31).set_wait_here(1 as i32 as u32);
                                found_wait = 0 as i32;
                            }
                            *newp = _ns_0;
                            newp = &mut (*_ns_0).next;
                        } else {
                            if !(*newp).is_null() {
                                (**newp).next = found;
                            } else {
                                *newp = found;
                            }
                            loop {
                                if cachep == 0 {
                                    (*found).name = xstrdup(
                                        concat(2 as i32 as u32, prefix, name),
                                    );
                                } else if !prefix.is_null() {
                                    (*found).name = strcache_add(
                                        concat(2 as i32 as u32, prefix, name),
                                    );
                                }
                                if ((*found).next).is_null() {
                                    break;
                                }
                                found = (*found).next;
                            }
                            newp = &mut (*found).next;
                        }
                    } else {
                        let mut _ns_1: *mut nameseq = xcalloc(size) as *mut nameseq;
                        let mut __n_1: *const i8 = concat(
                            2 as i32 as u32,
                            prefix,
                            *nlist.offset(i as isize),
                        );
                        (*_ns_1).name = if cachep != 0 {
                            strcache_add(__n_1)
                        } else {
                            xstrdup(__n_1)
                        };
                        if found_wait != 0 {
                            let ref mut fresh32 = *(_ns_1 as *mut dep);
                            (*fresh32).set_wait_here(1 as i32 as u32);
                            found_wait = 0 as i32;
                        }
                        *newp = _ns_1;
                        newp = &mut (*_ns_1).next;
                    }
                    i += 1;
                    i;
                }
                if globme != 0 {
                    globfree(&mut gl);
                }
                free(arname as *mut libc::c_void);
                free(tildep as *mut libc::c_void);
            }
        }
    }
    *stringp = p;
    return new as *mut libc::c_void;
}
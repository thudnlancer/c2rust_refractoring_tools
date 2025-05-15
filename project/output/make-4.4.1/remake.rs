use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn message(prefix: i32, length: size_t, fmt: *const i8, _: ...);
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn find_next_token(_: *mut *const i8, _: *mut size_t) -> *mut i8;
    fn print_spaces(_: u32);
    fn find_percent(_: *mut i8) -> *mut i8;
    fn ar_name(_: *const i8) -> i32;
    fn ar_parse_name(_: *const i8, _: *mut *mut i8, _: *mut *mut i8);
    fn ar_touch(_: *const i8) -> i32;
    fn ar_member_date(_: *const i8) -> time_t;
    fn vpath_search(
        file: *const i8,
        mtime_ptr: *mut uintmax_t,
        vpath_index: *mut u32,
        path_index: *mut u32,
    ) -> *const i8;
    fn gpath_search(file: *const i8, len: size_t) -> i32;
    fn strcache_add(str: *const i8) -> *const i8;
    static mut just_print_flag: i32;
    static mut run_silent: i32;
    static mut keep_going_flag: i32;
    static mut question_flag: i32;
    static mut touch_flag: i32;
    static mut always_make_flag: i32;
    static mut check_symlink_flag: i32;
    static mut second_expansion: i32;
    static mut clock_skew_detected: i32;
    static mut rebuilding_makefiles: i32;
    static mut command_count: u64;
    static mut no_intermediates: u32;
    static mut default_file: *mut file;
    fn lookup_file(name: *const i8) -> *mut file;
    fn enter_file(name: *const i8) -> *mut file;
    fn expand_deps(f: *mut file);
    fn rename_file(file: *mut file, name: *const i8);
    fn set_command_state(file: *mut file, state: cmd_state);
    fn file_timestamp_cons(_: *const i8, _: time_t, _: i64) -> uintmax_t;
    fn rehash_file(file: *mut file, name: *const i8);
    fn file_timestamp_now(_: *mut i32) -> uintmax_t;
    fn try_implicit_rule(file: *mut file, depth: u32) -> i32;
    fn start_waiting_jobs();
    fn reap_children(block: i32, err: i32);
    fn execute_file_commands(file: *mut file);
    fn chop_commands(cmds: *mut commands);
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    static mut variable_buffer: *mut i8;
    fn variable_buffer_output(
        ptr: *mut i8,
        string: *const i8,
        length: size_t,
    ) -> *mut i8;
    fn variable_expand(line: *const i8) -> *mut i8;
    static mut db_level: i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
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
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const i8,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub static mut commands_started: u32 = 0 as i32 as u32;
static mut goal_list: *mut goaldep = 0 as *const goaldep as *mut goaldep;
static mut goal_dep: *mut dep = 0 as *const dep as *mut dep;
static mut considered: u32 = 0 as i32 as u32;
unsafe extern "C" fn check_also_make(mut file: *const file) {
    let mut ad: *mut dep = 0 as *mut dep;
    let mut mtime: uintmax_t = (*file).last_mtime;
    if mtime == 0 as i32 as u64 {
        mtime = name_mtime((*file).name);
    }
    if mtime >= (2 as i32 + 1 as i32) as u64
        && mtime
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
                .wrapping_sub(1 as i32 as u64) && mtime > (*file).mtime_before_update
    {
        ad = (*file).also_make;
        while !ad.is_null() {
            if (*(*ad).file).last_mtime == 1 as i32 as u64 {
                error(
                    if !((*file).cmds).is_null() {
                        &mut (*(*file).cmds).fileinfo as *mut floc
                    } else {
                        0 as *mut floc
                    },
                    strlen((*(*ad).file).name),
                    dcgettext(
                        0 as *const i8,
                        b"warning: pattern recipe did not update peer target '%s'.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*(*ad).file).name,
                );
            }
            ad = (*ad).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_goal_chain(mut goaldeps: *mut goaldep) -> update_status {
    let mut last_cmd_count: u64 = 0 as i32 as u64;
    let mut t: i32 = touch_flag;
    let mut q: i32 = question_flag;
    let mut n: i32 = just_print_flag;
    let mut status: update_status = update_status::us_none;
    let mut goals_orig: *mut dep = copy_dep_chain(goaldeps as *mut dep);
    let mut goals: *mut dep = goals_orig;
    goal_list = if rebuilding_makefiles != 0 { goaldeps } else { 0 as *mut goaldep };
    considered = considered.wrapping_add(1);
    considered;
    while !goals.is_null() {
        let mut gu: *mut dep = 0 as *mut dep;
        let mut g: *mut dep = 0 as *mut dep;
        let mut lastgoal: *mut dep = 0 as *mut dep;
        start_waiting_jobs();
        reap_children((last_cmd_count == command_count) as i32, 0 as i32);
        last_cmd_count = command_count;
        lastgoal = 0 as *mut dep;
        gu = goals;
        while !gu.is_null() {
            let mut file: *mut file = 0 as *mut file;
            let mut stop: i32 = 0 as i32;
            let mut any_not_updated: i32 = 0 as i32;
            g = if !((*gu).shuf).is_null() { (*gu).shuf } else { gu };
            goal_dep = g;
            file = if !((*(*g).file).double_colon).is_null() {
                (*(*g).file).double_colon
            } else {
                (*g).file
            };
            while !file.is_null() {
                let mut ocommands_started: u32 = 0;
                let mut fail: update_status = update_status::us_success;
                (*file)
                    .set_dontcare(
                        ((*g).flags() as i32 & (1 as i32) << 2 as i32 != 0 as i32) as i32
                            as u32,
                    );
                while !((*file).renamed).is_null() {
                    file = (*file).renamed;
                }
                if rebuilding_makefiles != 0 {
                    if (*file).cmd_target() != 0 {
                        touch_flag = t;
                        question_flag = q;
                        just_print_flag = n;
                    } else {
                        just_print_flag = 0 as i32;
                        question_flag = just_print_flag;
                        touch_flag = question_flag;
                    }
                }
                ocommands_started = commands_started;
                fail = update_file(
                    file,
                    (if rebuilding_makefiles != 0 { 1 as i32 } else { 0 as i32 }) as u32,
                );
                while !((*file).renamed).is_null() {
                    file = (*file).renamed;
                }
                if commands_started > ocommands_started {
                    (*g).set_changed(1 as i32 as u32);
                }
                stop = 0 as i32;
                if (fail as u32 != 0 || (*file).updated() as i32 != 0)
                    && (status as u32) < update_status::us_question as i32 as u32
                {
                    if (*file).update_status() as u64 != 0 {
                        status = (*file).update_status();
                        stop = (question_flag != 0 && keep_going_flag == 0
                            && rebuilding_makefiles == 0) as i32;
                    } else {
                        let mut mtime: uintmax_t = if rebuilding_makefiles != 0 {
                            if (*file).last_mtime == 0 as i32 as u64 {
                                f_mtime(file, 0 as i32)
                            } else {
                                (*file).last_mtime
                            }
                        } else if (*file).last_mtime == 0 as i32 as u64 {
                            f_mtime(file, 1 as i32)
                        } else {
                            (*file).last_mtime
                        };
                        while !((*file).renamed).is_null() {
                            file = (*file).renamed;
                        }
                        if (*file).updated() as i32 != 0
                            && mtime != (*file).mtime_before_update
                        {
                            if rebuilding_makefiles == 0
                                || just_print_flag == 0 && question_flag == 0
                            {
                                status = update_status::us_success;
                            }
                            if rebuilding_makefiles != 0
                                && (*file).dontcare() as i32 != 0
                            {
                                stop = 1 as i32;
                            }
                        }
                    }
                }
                any_not_updated |= ((*file).updated() == 0) as i32;
                (*file).set_dontcare(0 as i32 as u32);
                if stop != 0 {
                    break;
                }
                file = (*file).prev;
            }
            file = (*g).file;
            if stop != 0 || any_not_updated == 0 {
                if rebuilding_makefiles == 0
                    && (*file).update_status() as i32 == update_status::us_success as i32
                    && (*g).changed() == 0 && run_silent == 0 && question_flag == 0
                {
                    message(
                        1 as i32,
                        strlen((*file).name),
                        if (*file).phony() as i32 != 0 || ((*file).cmds).is_null() {
                            dcgettext(
                                0 as *const i8,
                                b"Nothing to be done for '%s'.\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        } else {
                            dcgettext(
                                0 as *const i8,
                                b"'%s' is up to date.\0" as *const u8 as *const i8,
                                5 as i32,
                            )
                        },
                        (*file).name,
                    );
                }
                if lastgoal.is_null() {
                    goals = (*gu).next;
                } else {
                    (*lastgoal).next = (*gu).next;
                }
                gu = if lastgoal.is_null() { goals } else { (*lastgoal).next };
                if stop != 0 {
                    break;
                }
            } else {
                lastgoal = gu;
                gu = (*gu).next;
            }
        }
        if gu.is_null() {
            considered = considered.wrapping_add(1);
            considered;
        }
    }
    free_ns_chain(goals_orig as *mut nameseq);
    if rebuilding_makefiles != 0 {
        touch_flag = t;
        question_flag = q;
        just_print_flag = n;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn show_goal_error() {
    let mut goal: *mut goaldep = 0 as *mut goaldep;
    if (*goal_dep).flags() as i32 & ((1 as i32) << 1 as i32 | (1 as i32) << 2 as i32)
        != (1 as i32) << 1 as i32
    {
        return;
    }
    goal = goal_list;
    while !goal.is_null() {
        if (*goal_dep).file == (*goal).file {
            if (*goal).error != 0 {
                error(
                    &mut (*goal).floc as *mut floc,
                    (strlen((*(*goal).file).name))
                        .wrapping_add(strlen(strerror((*goal).error))),
                    b"%s: %s\0" as *const u8 as *const i8,
                    (*(*goal).file).name,
                    strerror((*goal).error),
                );
                (*goal).error = 0 as i32;
            }
            return;
        }
        goal = (*goal).next;
    }
}
unsafe extern "C" fn update_file(mut file: *mut file, mut depth: u32) -> update_status {
    let mut status: update_status = update_status::us_success;
    let mut f: *mut file = 0 as *mut file;
    f = if !((*file).double_colon).is_null() { (*file).double_colon } else { file };
    if (*f).considered == considered {
        if !((*f).updated() as i32 != 0
            && (*f).update_status() as i32 > update_status::us_none as i32
            && (*f).dontcare() == 0 && (*f).no_diag() as i32 != 0)
        {
            if 0x2 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Pruning file '%s'.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            return update_status::from_libc_c_uint(
                (if (*f).command_state() as i32 == cmd_state::cs_finished as i32 {
                    (*f).update_status() as i32
                } else {
                    update_status::us_success as i32
                }) as u32,
            );
        }
    }
    while !f.is_null() {
        let mut new: update_status = update_status::us_success;
        (*f).considered = considered;
        new = update_file_1(f, depth);
        while !((*f).renamed).is_null() {
            f = (*f).renamed;
        }
        if new as u32 != 0 && keep_going_flag == 0 {
            return new;
        }
        if (*f).command_state() as i32 == cmd_state::cs_running as i32
            || (*f).command_state() as i32 == cmd_state::cs_deps_running as i32
        {
            return update_status::us_success;
        }
        if new as u32 > status as u32 {
            status = new;
        }
        f = (*f).prev;
    }
    return status;
}
unsafe extern "C" fn complain(mut file: *mut file) {
    let mut d: *mut dep = 0 as *mut dep;
    d = (*file).deps;
    while !d.is_null() {
        if (*(*d).file).updated() as i32 != 0
            && (*(*d).file).update_status() as i32 > update_status::us_none as i32
            && (*file).no_diag() as i32 != 0
        {
            complain((*d).file);
            break;
        } else {
            d = (*d).next;
        }
    }
    if d.is_null() {
        show_goal_error();
        if !((*file).parent).is_null() {
            let mut l: size_t = (strlen((*file).name))
                .wrapping_add(strlen((*(*file).parent).name))
                .wrapping_add(4 as i32 as u64);
            let mut m: *const i8 = dcgettext(
                0 as *const i8,
                b"%sNo rule to make target '%s', needed by '%s'%s\0" as *const u8
                    as *const i8,
                5 as i32,
            );
            if keep_going_flag == 0 {
                fatal(
                    0 as *mut floc,
                    l,
                    m,
                    b"\0" as *const u8 as *const i8,
                    (*file).name,
                    (*(*file).parent).name,
                    b"\0" as *const u8 as *const i8,
                );
            }
            error(
                0 as *mut floc,
                l,
                m,
                b"*** \0" as *const u8 as *const i8,
                (*file).name,
                (*(*file).parent).name,
                b".\0" as *const u8 as *const i8,
            );
        } else {
            let mut l_0: size_t = (strlen((*file).name)).wrapping_add(4 as i32 as u64);
            let mut m_0: *const i8 = dcgettext(
                0 as *const i8,
                b"%sNo rule to make target '%s'%s\0" as *const u8 as *const i8,
                5 as i32,
            );
            if keep_going_flag == 0 {
                fatal(
                    0 as *mut floc,
                    l_0,
                    m_0,
                    b"\0" as *const u8 as *const i8,
                    (*file).name,
                    b"\0" as *const u8 as *const i8,
                );
            }
            error(
                0 as *mut floc,
                l_0,
                m_0,
                b"*** \0" as *const u8 as *const i8,
                (*file).name,
                b".\0" as *const u8 as *const i8,
            );
        }
        (*file).set_no_diag(0 as i32 as u32);
    }
}
unsafe extern "C" fn update_file_1(
    mut file: *mut file,
    mut depth: u32,
) -> update_status {
    let mut dep_status: update_status = update_status::us_success;
    let mut this_mtime: uintmax_t = 0;
    let mut noexist: i32 = 0;
    let mut must_make: i32 = 0;
    let mut deps_changed: i32 = 0;
    let mut ofile: *mut file = 0 as *mut file;
    let mut du: *mut dep = 0 as *mut dep;
    let mut d: *mut dep = 0 as *mut dep;
    let mut ad: *mut dep = 0 as *mut dep;
    let mut amake: dep = dep {
        next: 0 as *mut dep,
        name: 0 as *const i8,
        file: 0 as *mut file,
        shuf: 0 as *mut dep,
        stem: 0 as *const i8,
        flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [0; 2],
        c2rust_padding: [0; 6],
    };
    let mut running: i32 = 0 as i32;
    if 0x2 as i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const i8,
                b"Considering target file '%s'.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if (*file).updated() != 0 {
        if (*file).update_status() as i32 > update_status::us_none as i32 {
            if 0x2 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Recently tried and failed to update file '%s'.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            if (*file).no_diag() as i32 != 0 && (*file).dontcare() == 0 {
                complain(file);
            }
            return (*file).update_status();
        }
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"File '%s' was considered already.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        return update_status::us_success;
    }
    match (*file).command_state() as i32 {
        0 | 1 => {}
        2 => {
            if 0x2 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Still updating file '%s'.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            return update_status::us_success;
        }
        3 => {
            if 0x2 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Finished updating file '%s'.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            return (*file).update_status();
        }
        _ => {
            abort();
        }
    }
    (*file).set_no_diag((*file).dontcare());
    let ref mut fresh0 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh0).set_updating(1 as i32 as u32);
    ofile = file;
    depth = depth.wrapping_add(1);
    depth;
    this_mtime = if (*file).last_mtime == 0 as i32 as u64 {
        f_mtime(file, 1 as i32)
    } else {
        (*file).last_mtime
    };
    while !((*file).renamed).is_null() {
        file = (*file).renamed;
    }
    noexist = (this_mtime == 1 as i32 as u64) as i32;
    if noexist != 0 {
        if 0x1 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"File '%s' does not exist.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    } else if this_mtime >= (2 as i32 + 1 as i32) as u64
        && this_mtime
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
        && (*file).low_resolution_time() as i32 != 0
    {
        let mut ns: i32 = (this_mtime.wrapping_sub((2 as i32 + 1 as i32) as u64)
            & (((1 as i32) << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                - 1 as i32) as u64) as i32;
        if ns != 0 as i32 {
            error(
                0 as *mut floc,
                strlen((*file).name),
                dcgettext(
                    0 as *const i8,
                    b"*** Warning: .LOW_RESOLUTION_TIME file '%s' has a high resolution time stamp\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
        }
        this_mtime = (this_mtime as u64)
            .wrapping_add(
                ((if 1 as i32 != 0 { 1000000000 as i32 } else { 1 as i32 }) - 1 as i32
                    - ns) as u64,
            ) as uintmax_t as uintmax_t;
    }
    ad = (*file).also_make;
    while !ad.is_null() && noexist == 0 {
        let mut adfile: *mut file = (*ad).file;
        let mut fmtime: uintmax_t = if (*adfile).last_mtime == 0 as i32 as u64 {
            f_mtime(adfile, 1 as i32)
        } else {
            (*adfile).last_mtime
        };
        noexist = (fmtime == 1 as i32 as u64) as i32;
        if noexist != 0 {
            while !((*adfile).renamed).is_null() {
                adfile = (*adfile).renamed;
            }
            if 0x1 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Grouped target peer '%s' of file '%s' does not exist.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*adfile).name,
                    (*file).name,
                );
                fflush(stdout);
            }
        } else if fmtime < this_mtime {
            this_mtime = fmtime;
        }
        ad = (*ad).next;
    }
    must_make = noexist;
    if (*file).phony() == 0 && ((*file).cmds).is_null() && (*file).tried_implicit() == 0
    {
        try_implicit_rule(file, depth);
        (*file).set_tried_implicit(1 as i32 as u32);
    }
    if ((*file).cmds).is_null() && (*file).is_target() == 0 && !default_file.is_null()
        && !((*default_file).cmds).is_null()
    {
        if 0x8 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Using default recipe for '%s'.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        (*file).cmds = (*default_file).cmds;
    }
    amake.file = file;
    amake.next = (*file).also_make;
    ad = &mut amake;
    while !ad.is_null() {
        let mut lastd: *mut dep = 0 as *mut dep;
        if second_expansion != 0 {
            expand_deps((*ad).file);
        }
        du = (*(*ad).file).deps;
        ad = (*ad).next;
        while !du.is_null() {
            let mut new: update_status = update_status::us_success;
            let mut mtime: uintmax_t = 0;
            let mut maybe_make: i32 = 0;
            let mut dontcare: i32 = 0 as i32;
            d = if !((*du).shuf).is_null() { (*du).shuf } else { du };
            if (*d).wait_here() as i32 != 0 && running != 0 {
                break;
            }
            while !((*(*d).file).renamed).is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            mtime = if (*(*d).file).last_mtime == 0 as i32 as u64 {
                f_mtime((*d).file, 1 as i32)
            } else {
                (*(*d).file).last_mtime
            };
            while !((*(*d).file).renamed).is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            if (*if !((*(*d).file).double_colon).is_null() {
                (*(*d).file).double_colon
            } else {
                (*d).file
            })
                .updating() != 0
            {
                error(
                    0 as *mut floc,
                    (strlen((*file).name)).wrapping_add(strlen((*(*d).file).name)),
                    dcgettext(
                        0 as *const i8,
                        b"Circular %s <- %s dependency dropped.\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                    (*(*d).file).name,
                );
                if lastd.is_null() {
                    (*file).deps = (*du).next;
                } else {
                    (*lastd).next = (*du).next;
                }
                du = (*du).next;
            } else {
                (*(*d).file).parent = file;
                maybe_make = must_make;
                if rebuilding_makefiles != 0 {
                    dontcare = (*(*d).file).dontcare() as i32;
                    (*(*d).file).set_dontcare((*file).dontcare());
                }
                new = check_dep((*d).file, depth, this_mtime, &mut maybe_make);
                if new as u32 > dep_status as u32 {
                    dep_status = new;
                }
                if rebuilding_makefiles != 0 {
                    (*(*d).file).set_dontcare(dontcare as u32);
                }
                if (*d).ignore_mtime() == 0 {
                    must_make = maybe_make;
                }
                while !((*(*d).file).renamed).is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f: *mut file = (*d).file;
                if !((*f).double_colon).is_null() {
                    f = (*f).double_colon;
                }
                loop {
                    running
                        |= ((*f).command_state() as i32 == cmd_state::cs_running as i32
                            || (*f).command_state() as i32
                                == cmd_state::cs_deps_running as i32) as i32;
                    f = (*f).prev;
                    if f.is_null() {
                        break;
                    }
                }
                if dep_status as u32 != 0 && keep_going_flag == 0 {
                    break;
                }
                if running == 0 {
                    (*d)
                        .set_changed(
                            ((if (*(*d).file).last_mtime == 0 as i32 as u64 {
                                f_mtime((*d).file, 1 as i32)
                            } else {
                                (*(*d).file).last_mtime
                            }) != mtime || mtime == 1 as i32 as u64) as i32 as u32,
                        );
                }
                lastd = du;
                du = (*du).next;
            }
        }
    }
    if must_make != 0 || always_make_flag != 0 {
        du = (*file).deps;
        while !du.is_null() {
            d = if !((*du).shuf).is_null() { (*du).shuf } else { du };
            if (*d).wait_here() as i32 != 0 && running != 0 {
                break;
            }
            if (*(*d).file).intermediate() != 0 {
                let mut new_0: update_status = update_status::us_success;
                let mut dontcare_0: i32 = 0 as i32;
                let mut mtime_0: uintmax_t = if (*(*d).file).last_mtime
                    == 0 as i32 as u64
                {
                    f_mtime((*d).file, 1 as i32)
                } else {
                    (*(*d).file).last_mtime
                };
                while !((*(*d).file).renamed).is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                (*(*d).file).parent = file;
                if rebuilding_makefiles != 0 {
                    dontcare_0 = (*(*d).file).dontcare() as i32;
                    (*(*d).file).set_dontcare((*file).dontcare());
                }
                (*(*d).file).considered = 0 as i32 as u32;
                new_0 = update_file((*d).file, depth);
                if new_0 as u32 > dep_status as u32 {
                    dep_status = new_0;
                }
                if rebuilding_makefiles != 0 {
                    (*(*d).file).set_dontcare(dontcare_0 as u32);
                }
                while !((*(*d).file).renamed).is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f_0: *mut file = (*d).file;
                if !((*f_0).double_colon).is_null() {
                    f_0 = (*f_0).double_colon;
                }
                loop {
                    running
                        |= ((*f_0).command_state() as i32 == cmd_state::cs_running as i32
                            || (*f_0).command_state() as i32
                                == cmd_state::cs_deps_running as i32) as i32;
                    f_0 = (*f_0).prev;
                    if f_0.is_null() {
                        break;
                    }
                }
                if dep_status as u32 != 0 && keep_going_flag == 0 {
                    break;
                }
                if running == 0 {
                    (*d)
                        .set_changed(
                            ((*file).phony() as i32 != 0 && !((*file).cmds).is_null()
                                || (if (*(*d).file).last_mtime == 0 as i32 as u64 {
                                    f_mtime((*d).file, 1 as i32)
                                } else {
                                    (*(*d).file).last_mtime
                                }) != mtime_0) as i32 as u32,
                        );
                }
            }
            du = (*du).next;
        }
    }
    let ref mut fresh1 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh1).set_updating(0 as i32 as u32);
    let ref mut fresh2 = *if !((*ofile).double_colon).is_null() {
        (*ofile).double_colon
    } else {
        ofile
    };
    (*fresh2).set_updating(0 as i32 as u32);
    depth = depth.wrapping_sub(1);
    depth;
    if running != 0 {
        set_command_state(file, cmd_state::cs_deps_running);
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"The prerequisites of '%s' are being made.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        return update_status::us_success;
    }
    if 0x2 as i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const i8,
                b"Finished prerequisites of target file '%s'.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if dep_status as u64 != 0 {
        (*file)
            .set_update_status(
                update_status::from_libc_c_uint(
                    (if dep_status as u32 == update_status::us_none as i32 as u32 {
                        update_status::us_failed as i32 as u32
                    } else {
                        dep_status as u32
                    }) as u32,
                ),
            );
        notice_finished_file(file);
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Giving up on target file '%s'.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        if depth == 0 as i32 as u32 && keep_going_flag != 0 && just_print_flag == 0
            && question_flag == 0
        {
            error(
                0 as *mut floc,
                strlen((*file).name),
                dcgettext(
                    0 as *const i8,
                    b"Target '%s' not remade because of errors.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
        }
        return dep_status;
    }
    if (*file).command_state() as i32 == cmd_state::cs_deps_running as i32 {
        set_command_state(file, cmd_state::cs_not_started);
    }
    deps_changed = 0 as i32;
    d = (*file).deps;
    while !d.is_null() {
        let mut d_mtime: uintmax_t = if (*(*d).file).last_mtime == 0 as i32 as u64 {
            f_mtime((*d).file, 1 as i32)
        } else {
            (*(*d).file).last_mtime
        };
        while !((*(*d).file).renamed).is_null() {
            (*d).file = (*(*d).file).renamed;
        }
        if (*d).ignore_mtime() == 0 {
            if d_mtime == 1 as i32 as u64 && (*(*d).file).intermediate() == 0 {
                must_make = 1 as i32;
            }
            deps_changed |= (*d).changed() as i32;
        }
        (*d)
            .set_changed(
                (*d).changed() | (noexist != 0 || d_mtime > this_mtime) as i32 as u32,
            );
        if noexist == 0 && (0x1 as i32 | 0x2 as i32) & db_level != 0 {
            let mut fmt: *const i8 = 0 as *const i8;
            if (*d).ignore_mtime() != 0 {
                if 0x2 as i32 & db_level != 0 {
                    fmt = dcgettext(
                        0 as *const i8,
                        b"Prerequisite '%s' is order-only for target '%s'.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    );
                }
            } else if d_mtime == 1 as i32 as u64 {
                if 0x1 as i32 & db_level != 0 {
                    fmt = dcgettext(
                        0 as *const i8,
                        b"Prerequisite '%s' of target '%s' does not exist.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    );
                }
            } else if (*d).changed() != 0 {
                if 0x1 as i32 & db_level != 0 {
                    fmt = dcgettext(
                        0 as *const i8,
                        b"Prerequisite '%s' is newer than target '%s'.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    );
                }
            } else if 0x2 as i32 & db_level != 0 {
                fmt = dcgettext(
                    0 as *const i8,
                    b"Prerequisite '%s' is older than target '%s'.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                );
            }
            if !fmt.is_null() {
                print_spaces(depth.wrapping_add(1 as i32 as u32));
                printf(
                    fmt,
                    if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        d = (*d).next;
    }
    if !((*file).double_colon).is_null() && ((*file).deps).is_null() {
        must_make = 1 as i32;
        if 0x1 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Target '%s' is double-colon and has no prerequisites.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    } else if noexist == 0 && (*file).is_target() as i32 != 0 && deps_changed == 0
        && ((*file).cmds).is_null() && always_make_flag == 0
    {
        must_make = 0 as i32;
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"No recipe for '%s' and no prerequisites actually changed.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    } else if must_make == 0 && !((*file).cmds).is_null() && always_make_flag != 0 {
        must_make = 1 as i32;
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Making '%s' due to always-make flag.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    }
    if must_make == 0 {
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"No need to remake target '%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            if !((*file).name == (*file).hname
                || *(*file).name as i32 == *(*file).hname as i32
                    && (*(*file).name as i32 == '\0' as i32
                        || strcmp(
                            ((*file).name).offset(1 as i32 as isize),
                            ((*file).hname).offset(1 as i32 as isize),
                        ) == 0))
            {
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"; using VPATH name '%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).hname,
                );
            }
            puts(b".\0" as *const u8 as *const i8);
            fflush(stdout);
        }
        if (*file).notintermediate() == 0 && no_intermediates == 0 as i32 as u32 {
            (*file).set_secondary(1 as i32 as u32);
        }
        notice_finished_file(file);
        while !file.is_null() {
            (*file).name = (*file).hname;
            file = (*file).prev;
        }
        return update_status::us_success;
    }
    if 0x1 as i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const i8,
                b"Must remake target '%s'.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if !((*file).name == (*file).hname
        || *(*file).name as i32 == *(*file).hname as i32
            && (*(*file).name as i32 == '\0' as i32
                || strcmp(
                    ((*file).name).offset(1 as i32 as isize),
                    ((*file).hname).offset(1 as i32 as isize),
                ) == 0))
    {
        if 0x1 as i32 & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"  Ignoring VPATH name '%s'.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).hname,
            );
            fflush(stdout);
        }
        (*file).set_ignore_vpath(1 as i32 as u32);
    }
    remake_file(file);
    if (*file).command_state() as i32 != cmd_state::cs_finished as i32 {
        if 0x2 as i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Recipe of '%s' is being run.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        return update_status::us_success;
    }
    match (*file).update_status() as i32 {
        3 => {
            if 0x1 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Failed to remake target file '%s'.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        0 => {
            if 0x1 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Successfully remade target file '%s'.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        2 => {
            if 0x1 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Target file '%s' needs to be remade under -q.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        1 | _ => {}
    }
    (*file).set_updated(1 as i32 as u32);
    return (*file).update_status();
}
#[no_mangle]
pub unsafe extern "C" fn notice_finished_file(mut file: *mut file) {
    let mut d: *mut dep = 0 as *mut dep;
    let mut ran: i32 = ((*file).command_state() as i32 == cmd_state::cs_running as i32)
        as i32;
    let mut touched: i32 = 0 as i32;
    (*file).set_command_state(cmd_state::cs_finished);
    (*file).set_updated(1 as i32 as u32);
    if touch_flag != 0
        && (*file).update_status() as i32 == update_status::us_success as i32
    {
        let mut current_block_9: u64;
        if !((*file).cmds).is_null() && (*(*file).cmds).any_recurse() as i32 != 0 {
            let mut i: u32 = 0;
            i = 0 as i32 as u32;
            loop {
                if !(i < (*(*file).cmds).ncommand_lines as u32) {
                    current_block_9 = 3512920355445576850;
                    break;
                }
                if !(*((*(*file).cmds).lines_flags).offset(i as isize) as i32 & 1 as i32
                    != 0 as i32)
                {
                    current_block_9 = 4046062769010062641;
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            current_block_9 = 4046062769010062641;
        }
        match current_block_9 {
            4046062769010062641 => {
                if (*file).phony() != 0 {
                    (*file).set_update_status(update_status::us_success);
                } else if !((*file).cmds).is_null() {
                    (*file).set_update_status(touch_file(file));
                    commands_started = commands_started.wrapping_add(1);
                    commands_started;
                    touched = 1 as i32;
                }
            }
            _ => {}
        }
    }
    if (*file).mtime_before_update == 0 as i32 as u64 {
        (*file).mtime_before_update = (*file).last_mtime;
    }
    if ran != 0 && (*file).phony() == 0 || touched != 0 {
        let mut i_0: i32 = 0 as i32;
        if (question_flag != 0 || just_print_flag != 0 || touch_flag != 0)
            && !((*file).cmds).is_null()
        {
            i_0 = (*(*file).cmds).ncommand_lines as i32;
            while i_0 > 0 as i32 {
                if !(*((*(*file).cmds).lines_flags).offset((i_0 - 1 as i32) as isize)
                    as i32 & 1 as i32 != 0 as i32)
                {
                    break;
                }
                i_0 -= 1;
                i_0;
            }
        } else if (*file).is_target() as i32 != 0 && ((*file).cmds).is_null() {
            i_0 = 1 as i32;
        }
        (*file).last_mtime = if i_0 == 0 as i32 {
            0 as i32 as u64
        } else {
            (!(0 as i32 as uintmax_t))
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
        };
    }
    if !((*file).double_colon).is_null() {
        let mut f: *mut file = 0 as *mut file;
        let mut max_mtime: uintmax_t = (*file).last_mtime;
        f = (*file).double_colon;
        while !f.is_null() && (*f).updated() as i32 != 0 {
            if max_mtime != 0 as i32 as u64
                && ((*f).last_mtime == 0 as i32 as u64 || (*f).last_mtime > max_mtime)
            {
                max_mtime = (*f).last_mtime;
            }
            f = (*f).prev;
        }
        if f.is_null() {
            f = (*file).double_colon;
            while !f.is_null() {
                (*f).last_mtime = max_mtime;
                f = (*f).prev;
            }
        }
    }
    if ran != 0 && (*file).update_status() as i32 != update_status::us_none as i32 {
        d = (*file).also_make;
        while !d.is_null() {
            (*(*d).file).set_command_state(cmd_state::cs_finished);
            (*(*d).file).set_updated(1 as i32 as u32);
            (*(*d).file).set_update_status((*file).update_status());
            if ran != 0 && (*(*d).file).phony() == 0 {
                f_mtime((*d).file, 0 as i32);
            }
            d = (*d).next;
        }
        if (*file).tried_implicit() as i32 != 0 && !((*file).also_make).is_null() {
            check_also_make(file);
        }
    } else if (*file).update_status() as i32 == update_status::us_none as i32 {
        (*file).set_update_status(update_status::us_success);
    }
}
unsafe extern "C" fn check_dep(
    mut file: *mut file,
    mut depth: u32,
    mut this_mtime: uintmax_t,
    mut must_make_ptr: *mut i32,
) -> update_status {
    let mut ofile: *mut file = 0 as *mut file;
    let mut d: *mut dep = 0 as *mut dep;
    let mut dep_status: update_status = update_status::us_success;
    let ref mut fresh3 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh3).set_updating(1 as i32 as u32);
    ofile = file;
    if (*file).phony() as i32 != 0 || (*file).intermediate() == 0 {
        let mut mtime: uintmax_t = 0;
        dep_status = update_file(file, depth);
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        mtime = if (*file).last_mtime == 0 as i32 as u64 {
            f_mtime(file, 1 as i32)
        } else {
            (*file).last_mtime
        };
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        if mtime == 1 as i32 as u64 || mtime > this_mtime {
            *must_make_ptr = 1 as i32;
        }
    } else {
        let mut mtime_0: uintmax_t = 0;
        if (*file).phony() == 0 && ((*file).cmds).is_null()
            && (*file).tried_implicit() == 0
        {
            try_implicit_rule(file, depth);
            (*file).set_tried_implicit(1 as i32 as u32);
        }
        if ((*file).cmds).is_null() && (*file).is_target() == 0
            && !default_file.is_null() && !((*default_file).cmds).is_null()
        {
            if 0x8 as i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const i8,
                        b"Using default commands for '%s'.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            (*file).cmds = (*default_file).cmds;
        }
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        mtime_0 = if (*file).last_mtime == 0 as i32 as u64 {
            f_mtime(file, 1 as i32)
        } else {
            (*file).last_mtime
        };
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        if mtime_0 != 1 as i32 as u64 && mtime_0 > this_mtime {
            *must_make_ptr = 1 as i32;
        } else {
            let mut ld: *mut dep = 0 as *mut dep;
            let mut deps_running: i32 = 0 as i32;
            if (*file).command_state() as i32 != cmd_state::cs_running as i32 {
                if (*file).command_state() as i32 == cmd_state::cs_deps_running as i32 {
                    (*file).considered = 0 as i32 as u32;
                }
                set_command_state(file, cmd_state::cs_not_started);
            }
            ld = 0 as *mut dep;
            if second_expansion != 0 {
                expand_deps(file);
            }
            d = (*file).deps;
            while !d.is_null() {
                let mut new: update_status = update_status::us_success;
                let mut maybe_make: i32 = 0;
                if (*if !((*(*d).file).double_colon).is_null() {
                    (*(*d).file).double_colon
                } else {
                    (*d).file
                })
                    .updating() != 0
                {
                    error(
                        0 as *mut floc,
                        (strlen((*file).name)).wrapping_add(strlen((*(*d).file).name)),
                        dcgettext(
                            0 as *const i8,
                            b"Circular %s <- %s dependency dropped.\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*file).name,
                        (*(*d).file).name,
                    );
                    if ld.is_null() {
                        (*file).deps = (*d).next;
                        free(d as *mut libc::c_void);
                        d = (*file).deps;
                    } else {
                        (*ld).next = (*d).next;
                        free(d as *mut libc::c_void);
                        d = (*ld).next;
                    }
                } else {
                    (*(*d).file).parent = file;
                    maybe_make = *must_make_ptr;
                    new = check_dep(
                        (*d).file,
                        depth.wrapping_add(1 as i32 as u32),
                        this_mtime,
                        &mut maybe_make,
                    );
                    if new as u32 > dep_status as u32 {
                        dep_status = new;
                    }
                    if (*d).ignore_mtime() == 0 {
                        *must_make_ptr = maybe_make;
                    }
                    while !((*(*d).file).renamed).is_null() {
                        (*d).file = (*(*d).file).renamed;
                    }
                    if dep_status as u32 != 0 && keep_going_flag == 0 {
                        break;
                    }
                    if (*(*d).file).command_state() as i32
                        == cmd_state::cs_running as i32
                        || (*(*d).file).command_state() as i32
                            == cmd_state::cs_deps_running as i32
                    {
                        deps_running = 1 as i32;
                    }
                    ld = d;
                    d = (*d).next;
                }
            }
            if deps_running != 0 {
                set_command_state(file, cmd_state::cs_deps_running);
            }
        }
    }
    let ref mut fresh4 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh4).set_updating(0 as i32 as u32);
    let ref mut fresh5 = *if !((*ofile).double_colon).is_null() {
        (*ofile).double_colon
    } else {
        ofile
    };
    (*fresh5).set_updating(0 as i32 as u32);
    return dep_status;
}
unsafe extern "C" fn touch_file(mut file: *mut file) -> update_status {
    if run_silent == 0 {
        message(
            0 as i32,
            strlen((*file).name),
            b"touch %s\0" as *const u8 as *const i8,
            (*file).name,
        );
    }
    if just_print_flag != 0 {
        return update_status::us_success;
    }
    if ar_name((*file).name) != 0 {
        return update_status::from_libc_c_uint(
            (if ar_touch((*file).name) != 0 {
                update_status::us_failed as i32
            } else {
                update_status::us_success as i32
            }) as u32,
        )
    } else {
        let mut fd: i32 = 0;
        loop {
            fd = open((*file).name, 0o2 as i32 | 0o100 as i32, 0o666 as i32);
            if !(fd == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if fd < 0 as i32 {
            perror_with_name(b"touch: open: \0" as *const u8 as *const i8, (*file).name);
            return update_status::us_failed;
        } else {
            let mut statbuf: stat = stat {
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
            let mut buf: i8 = 'x' as i32 as i8;
            let mut e: i32 = 0;
            loop {
                e = fstat(fd, &mut statbuf);
                if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if e < 0 as i32 {
                perror_with_name(
                    b"touch: fstat: \0" as *const u8 as *const i8,
                    (*file).name,
                );
                return update_status::us_failed;
            }
            loop {
                e = read(
                    fd,
                    &mut buf as *mut i8 as *mut libc::c_void,
                    1 as i32 as size_t,
                ) as i32;
                if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if e < 0 as i32 {
                perror_with_name(
                    b"touch: read: \0" as *const u8 as *const i8,
                    (*file).name,
                );
                return update_status::us_failed;
            }
            let mut o: off_t = 0;
            loop {
                o = lseek(fd, 0 as i64, 0 as i32);
                if !(o == -(1 as i32) as i64 && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if o < 0 as i64 {
                perror_with_name(
                    b"touch: lseek: \0" as *const u8 as *const i8,
                    (*file).name,
                );
                return update_status::us_failed;
            }
            loop {
                e = write(
                    fd,
                    &mut buf as *mut i8 as *const libc::c_void,
                    1 as i32 as size_t,
                ) as i32;
                if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if e < 0 as i32 {
                perror_with_name(
                    b"touch: write: \0" as *const u8 as *const i8,
                    (*file).name,
                );
                return update_status::us_failed;
            }
            if statbuf.st_size == 0 as i32 as i64 {
                close(fd);
                loop {
                    fd = open((*file).name, 0o2 as i32 | 0o1000 as i32, 0o666 as i32);
                    if !(fd == -(1 as i32) && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if fd < 0 as i32 {
                    perror_with_name(
                        b"touch: open: \0" as *const u8 as *const i8,
                        (*file).name,
                    );
                    return update_status::us_failed;
                }
            }
            close(fd);
        }
    }
    return update_status::us_success;
}
unsafe extern "C" fn remake_file(mut file: *mut file) {
    if ((*file).cmds).is_null() {
        if (*file).phony() != 0 {
            (*file).set_update_status(update_status::us_success);
        } else if (*file).is_target() != 0 {
            (*file).set_update_status(update_status::us_success);
        } else {
            if rebuilding_makefiles == 0 || (*file).dontcare() == 0 {
                complain(file);
            }
            (*file).set_update_status(update_status::us_failed);
        }
    } else {
        chop_commands((*file).cmds);
        if touch_flag == 0 || (*(*file).cmds).any_recurse() as i32 != 0 {
            execute_file_commands(file);
            return;
        }
        (*file).set_update_status(update_status::us_success);
    }
    notice_finished_file(file);
}
#[no_mangle]
pub unsafe extern "C" fn f_mtime(mut file: *mut file, mut search: i32) -> uintmax_t {
    let mut mtime: uintmax_t = 0;
    let mut propagate_timestamp: u32 = 0;
    if ar_name((*file).name) != 0 {
        let mut memmtime: uintmax_t = 0;
        let mut arname: *mut i8 = 0 as *mut i8;
        let mut memname: *mut i8 = 0 as *mut i8;
        let mut arfile: *mut file = 0 as *mut file;
        let mut member_date: time_t = 0;
        ar_parse_name((*file).name, &mut arname, &mut memname);
        memmtime = name_mtime(memname);
        arfile = lookup_file(arname);
        if arfile.is_null() {
            arfile = enter_file(strcache_add(arname));
        }
        mtime = f_mtime(arfile, search);
        while !((*arfile).renamed).is_null() {
            arfile = (*arfile).renamed;
        }
        if search != 0 && strcmp((*arfile).hname, arname) != 0 {
            let mut name: *mut i8 = 0 as *mut i8;
            let mut arlen: size_t = 0;
            let mut memlen: size_t = 0;
            arlen = strlen((*arfile).hname);
            memlen = strlen(memname);
            let mut fresh6 = ::std::vec::from_elem(
                0,
                arlen
                    .wrapping_add(1 as i32 as u64)
                    .wrapping_add(memlen)
                    .wrapping_add(2 as i32 as u64) as usize,
            );
            name = fresh6.as_mut_ptr() as *mut i8;
            memcpy(
                name as *mut libc::c_void,
                (*arfile).hname as *const libc::c_void,
                arlen,
            );
            *name.offset(arlen as isize) = '(' as i32 as i8;
            memcpy(
                name.offset(arlen as isize).offset(1 as i32 as isize)
                    as *mut libc::c_void,
                memname as *const libc::c_void,
                memlen,
            );
            *name
                .offset(
                    arlen.wrapping_add(1 as i32 as u64).wrapping_add(memlen) as isize,
                ) = ')' as i32 as i8;
            *name
                .offset(
                    arlen
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(memlen)
                        .wrapping_add(1 as i32 as u64) as isize,
                ) = '\0' as i32 as i8;
            if (*arfile).name == (*arfile).hname {
                rename_file(file, strcache_add(name));
            } else {
                rehash_file(file, strcache_add(name));
            }
            while !((*file).renamed).is_null() {
                file = (*file).renamed;
            }
        }
        free(arname as *mut libc::c_void);
        (*file).set_low_resolution_time(1 as i32 as u32);
        if mtime == 1 as i32 as u64 {
            return 1 as i32 as uintmax_t;
        }
        member_date = ar_member_date((*file).hname);
        if member_date == -(1 as i32) as time_t
            || memmtime != 1 as i32 as u64
                && (memmtime.wrapping_sub((2 as i32 + 1 as i32) as u64)
                    >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 })) as time_t
                    > member_date
        {
            mtime = 1 as i32 as uintmax_t;
        } else {
            mtime = file_timestamp_cons((*file).hname, member_date, 0 as i32 as i64);
        }
    } else {
        mtime = name_mtime((*file).name);
        if mtime == 1 as i32 as u64 && search != 0 && (*file).ignore_vpath() == 0 {
            let mut name_0: *const i8 = vpath_search(
                (*file).name,
                &mut mtime,
                0 as *mut u32,
                0 as *mut u32,
            );
            if !name_0.is_null()
                || *((*file).name).offset(0 as i32 as isize) as i32 == '-' as i32
                    && *((*file).name).offset(1 as i32 as isize) as i32 == 'l' as i32
                    && {
                        name_0 = library_search((*file).name, &mut mtime);
                        !name_0.is_null()
                    }
            {
                let mut name_len: size_t = 0;
                if mtime != 0 as i32 as u64 {
                    (*file).last_mtime = mtime;
                }
                name_len = (strlen(name_0))
                    .wrapping_sub(strlen((*file).name))
                    .wrapping_sub(1 as i32 as u64);
                if gpath_search(name_0, name_len) != 0 {
                    rename_file(file, name_0);
                    while !((*file).renamed).is_null() {
                        file = (*file).renamed;
                    }
                    return if (*file).last_mtime == 0 as i32 as u64 {
                        f_mtime(file, 1 as i32)
                    } else {
                        (*file).last_mtime
                    };
                }
                rehash_file(file, name_0);
                while !((*file).renamed).is_null() {
                    file = (*file).renamed;
                }
                if mtime != 2 as i32 as u64
                    && mtime
                        != (!(0 as i32 as uintmax_t))
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
                {
                    mtime = name_mtime(name_0);
                }
            }
        }
    }
    if clock_skew_detected == 0 && mtime != 1 as i32 as u64
        && mtime
            != (!(0 as i32 as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as i32) as uintmax_t <= 0 as i32 as u64) {
                        0 as i32 as uintmax_t
                    } else {
                        !(0 as i32 as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    }),
                ) && (*file).updated() == 0
    {
        static mut adjusted_now: uintmax_t = 0;
        let mut adjusted_mtime: uintmax_t = mtime;
        if adjusted_now < adjusted_mtime {
            let mut resolution: i32 = 0;
            let mut now: uintmax_t = file_timestamp_now(&mut resolution);
            adjusted_now = now.wrapping_add((resolution - 1 as i32) as u64);
            if adjusted_now < adjusted_mtime {
                let mut from_now: libc::c_double = (mtime
                    .wrapping_sub((2 as i32 + 1 as i32) as u64)
                    >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                    .wrapping_sub(
                        now.wrapping_sub((2 as i32 + 1 as i32) as u64)
                            >> (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }),
                    ) as libc::c_double
                    + ((mtime.wrapping_sub((2 as i32 + 1 as i32) as u64)
                        & (((1 as i32)
                            << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                            - 1 as i32) as u64) as i32
                        - (now.wrapping_sub((2 as i32 + 1 as i32) as u64)
                            & (((1 as i32)
                                << (if 1 as i32 != 0 { 30 as i32 } else { 0 as i32 }))
                                - 1 as i32) as u64) as i32) as libc::c_double / 1e9f64;
                let mut from_now_string: [i8; 100] = [0; 100];
                if from_now >= 100.0f64
                    && from_now
                        < (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64) as libc::c_double
                {
                    sprintf(
                        from_now_string.as_mut_ptr(),
                        b"%lu\0" as *const u8 as *const i8,
                        from_now as u64,
                    );
                } else {
                    sprintf(
                        from_now_string.as_mut_ptr(),
                        b"%.2g\0" as *const u8 as *const i8,
                        from_now,
                    );
                }
                error(
                    0 as *mut floc,
                    (strlen((*file).name))
                        .wrapping_add(strlen(from_now_string.as_mut_ptr())),
                    dcgettext(
                        0 as *const i8,
                        b"Warning: File '%s' has modification time %s s in the future\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*file).name,
                    from_now_string.as_mut_ptr(),
                );
                clock_skew_detected = 1 as i32;
            }
        }
    }
    if !((*file).double_colon).is_null() {
        file = (*file).double_colon;
    }
    propagate_timestamp = (*file).updated();
    loop {
        if mtime != 1 as i32 as u64
            && (*file).command_state() as i32 == cmd_state::cs_not_started as i32
            && (*file).tried_implicit() == 0 && (*file).intermediate() as i32 != 0
        {
            (*file).set_intermediate(0 as i32 as u32);
        }
        if (*file).updated() == propagate_timestamp {
            (*file).last_mtime = mtime;
        }
        file = (*file).prev;
        if file.is_null() {
            break;
        }
    }
    return mtime;
}
unsafe extern "C" fn name_mtime(mut name: *const i8) -> uintmax_t {
    let mut mtime: uintmax_t = 0;
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
    loop {
        e = stat(name, &mut st);
        if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if e == 0 as i32 {
        mtime = file_timestamp_cons(name, st.st_mtim.tv_sec, st.st_mtim.tv_nsec);
    } else if *__errno_location() == 2 as i32 || *__errno_location() == 20 as i32 {
        mtime = 1 as i32 as uintmax_t;
    } else {
        perror_with_name(b"stat: \0" as *const u8 as *const i8, name);
        return 1 as i32 as uintmax_t;
    }
    if check_symlink_flag != 0 && strlen(name) <= 4096 as i32 as u64 {
        let mut lpath: [i8; 4097] = [0; 4097];
        strcpy(lpath.as_mut_ptr(), name);
        loop {
            let mut ltime: uintmax_t = 0;
            let mut lbuf: [i8; 4097] = [0; 4097];
            let mut llen: i64 = 0;
            let mut p: *mut i8 = 0 as *mut i8;
            loop {
                e = lstat(lpath.as_mut_ptr(), &mut st);
                if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if e != 0 {
                if *__errno_location() != 2 as i32 && *__errno_location() != 20 as i32 {
                    perror_with_name(
                        b"lstat: \0" as *const u8 as *const i8,
                        lpath.as_mut_ptr(),
                    );
                }
                break;
            } else {
                if !(st.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32) {
                    break;
                }
                ltime = file_timestamp_cons(
                    lpath.as_mut_ptr(),
                    st.st_mtim.tv_sec,
                    st.st_mtim.tv_nsec,
                );
                if ltime > mtime {
                    mtime = ltime;
                }
                loop {
                    llen = readlink(
                        lpath.as_mut_ptr(),
                        lbuf.as_mut_ptr(),
                        (4096 as i32 - 1 as i32) as size_t,
                    );
                    if !(llen == -(1 as i32) as i64 && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if llen < 0 as i32 as i64 {
                    perror_with_name(
                        b"readlink: \0" as *const u8 as *const i8,
                        lpath.as_mut_ptr(),
                    );
                    break;
                } else {
                    lbuf[llen as usize] = '\0' as i32 as i8;
                    if lbuf[0 as i32 as usize] as i32 == '/' as i32
                        || {
                            p = strrchr(lpath.as_mut_ptr(), '/' as i32);
                            p.is_null()
                        }
                    {
                        strcpy(lpath.as_mut_ptr(), lbuf.as_mut_ptr());
                    } else {
                        if p.offset_from(lpath.as_mut_ptr()) as i64 + llen
                            + 2 as i32 as i64 > 4096 as i32 as i64
                        {
                            break;
                        }
                        strcpy(p.offset(1 as i32 as isize), lbuf.as_mut_ptr());
                    }
                }
            }
        }
    }
    return mtime;
}
unsafe extern "C" fn library_search(
    mut lib: *const i8,
    mut mtime_ptr: *mut uintmax_t,
) -> *const i8 {
    static mut dirs: [*const i8; 4] = [
        b"/lib\0" as *const u8 as *const i8,
        b"/usr/lib\0" as *const u8 as *const i8,
        b"/usr/local/lib\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut file: *const i8 = 0 as *const i8;
    let mut libpatterns: *mut i8 = 0 as *mut i8;
    let mut mtime: uintmax_t = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut p2: *const i8 = 0 as *const i8;
    let mut len: size_t = 0;
    let mut liblen: size_t = 0;
    let mut best_vpath: u32 = 0 as i32 as u32;
    let mut best_path: u32 = 0 as i32 as u32;
    let mut dp: *mut *const i8 = 0 as *mut *const i8;
    libpatterns = xstrdup(
        variable_expand(b"$(.LIBPATTERNS)\0" as *const u8 as *const i8),
    );
    lib = lib.offset(2 as i32 as isize);
    liblen = strlen(lib);
    p2 = libpatterns;
    loop {
        p = find_next_token(&mut p2, &mut len);
        if p.is_null() {
            break;
        }
        static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
        static mut buflen: size_t = 0 as i32 as size_t;
        static mut libdir_maxlen: size_t = 0 as i32 as size_t;
        static mut std_dirs: u32 = 0 as i32 as u32;
        let mut libbuf: *mut i8 = 0 as *mut i8;
        let mut c: i8 = *p.offset(len as isize);
        let mut p3: *mut i8 = 0 as *mut i8;
        let mut p4: *mut i8 = 0 as *mut i8;
        *p.offset(len as isize) = '\0' as i32 as i8;
        p3 = find_percent(p);
        if p3.is_null() {
            error(
                0 as *mut floc,
                strlen(p),
                dcgettext(
                    0 as *const i8,
                    b".LIBPATTERNS element '%s' is not a pattern\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                p,
            );
            *p.offset(len as isize) = c;
        } else {
            p4 = variable_buffer_output(
                variable_buffer,
                p,
                p3.offset_from(p) as i64 as size_t,
            );
            p4 = variable_buffer_output(p4, lib, liblen);
            p4 = variable_buffer_output(
                p4,
                p3.offset(1 as i32 as isize),
                len.wrapping_sub(p3.offset_from(p) as i64 as u64),
            );
            *p.offset(len as isize) = c;
            libbuf = variable_buffer;
            mtime = name_mtime(libbuf);
            if mtime != 1 as i32 as u64 {
                if !mtime_ptr.is_null() {
                    *mtime_ptr = mtime;
                }
                file = strcache_add(libbuf);
                break;
            } else {
                let mut vpath_index: u32 = 0;
                let mut path_index: u32 = 0;
                let mut f: *const i8 = vpath_search(
                    libbuf,
                    if !mtime_ptr.is_null() { &mut mtime } else { 0 as *mut uintmax_t },
                    &mut vpath_index,
                    &mut path_index,
                );
                if !f.is_null() {
                    if file.is_null() || vpath_index < best_vpath
                        || vpath_index == best_vpath && path_index < best_path
                    {
                        file = f;
                        best_vpath = vpath_index;
                        best_path = path_index;
                        if !mtime_ptr.is_null() {
                            *mtime_ptr = mtime;
                        }
                    }
                }
                if buflen == 0 {
                    dp = dirs.as_mut_ptr();
                    while !(*dp).is_null() {
                        let mut l: size_t = strlen(*dp);
                        if l > libdir_maxlen {
                            libdir_maxlen = l;
                        }
                        std_dirs = std_dirs.wrapping_add(1);
                        std_dirs;
                        dp = dp.offset(1);
                        dp;
                    }
                    buflen = strlen(libbuf);
                    buf = xmalloc(
                        libdir_maxlen.wrapping_add(buflen).wrapping_add(2 as i32 as u64),
                    ) as *mut i8;
                } else if buflen < strlen(libbuf) {
                    buflen = strlen(libbuf);
                    buf = xrealloc(
                        buf as *mut libc::c_void,
                        libdir_maxlen.wrapping_add(buflen).wrapping_add(2 as i32 as u64),
                    ) as *mut i8;
                }
                let mut vpath_index_0: u32 = (!(0 as i32 as u32)).wrapping_sub(std_dirs);
                dp = dirs.as_mut_ptr();
                while !(*dp).is_null() {
                    sprintf(buf, b"%s/%s\0" as *const u8 as *const i8, *dp, libbuf);
                    mtime = name_mtime(buf);
                    if mtime != 1 as i32 as u64 {
                        if file.is_null() || vpath_index_0 < best_vpath {
                            file = strcache_add(buf);
                            best_vpath = vpath_index_0;
                            if !mtime_ptr.is_null() {
                                *mtime_ptr = mtime;
                            }
                        }
                    }
                    vpath_index_0 = vpath_index_0.wrapping_add(1);
                    vpath_index_0;
                    dp = dp.offset(1);
                    dp;
                }
            }
        }
    }
    free(libpatterns as *mut libc::c_void);
    return file;
}
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type variable_set_list;
    pub type commands;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut i32;
    fn unlink(__name: *const i8) -> i32;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const i8) -> *mut i8;
    fn mkstemp(__template: *mut i8) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn stpcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn out_of_memory() -> !;
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut posix_pedantic: i32;
    fn os_anontmp() -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
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
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
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
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn make_toui(
    mut str: *const i8,
    mut error_0: *mut *const i8,
) -> u32 {
    let mut end: *mut i8 = 0 as *mut i8;
    let mut val: u64 = strtoul(str, &mut end, 10 as i32);
    if !error_0.is_null() {
        if *str.offset(0 as i32 as isize) as i32 == '\0' as i32 {
            *error_0 = b"Missing value\0" as *const u8 as *const i8;
        } else if *end as i32 != '\0' as i32 {
            *error_0 = b"Invalid value\0" as *const u8 as *const i8;
        } else {
            *error_0 = 0 as *const i8;
        }
    }
    return val as u32;
}
#[no_mangle]
pub unsafe extern "C" fn make_lltoa(
    mut val: libc::c_longlong,
    mut buf: *mut i8,
) -> *mut i8 {
    sprintf(buf, b"%lld\0" as *const u8 as *const i8, val);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn make_ulltoa(
    mut val: libc::c_ulonglong,
    mut buf: *mut i8,
) -> *mut i8 {
    sprintf(buf, b"%llu\0" as *const u8 as *const i8, val);
    return buf;
}
static mut mk_state: u32 = 0 as i32 as u32;
#[no_mangle]
pub unsafe extern "C" fn make_seed(mut seed: u32) {
    mk_state = seed;
}
#[no_mangle]
pub unsafe extern "C" fn make_rand() -> u32 {
    if mk_state == 0 as i32 as u32 {
        mk_state = ((time(0 as *mut time_t) ^ make_pid() as i64) as u32)
            .wrapping_add(1 as i32 as u32);
    }
    mk_state ^= mk_state << 13 as i32;
    mk_state ^= mk_state >> 17 as i32;
    mk_state ^= mk_state << 5 as i32;
    return mk_state;
}
#[no_mangle]
pub unsafe extern "C" fn alpha_compare(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> i32 {
    let mut s1: *const i8 = *(v1 as *mut *mut i8);
    let mut s2: *const i8 = *(v2 as *mut *mut i8);
    if *s1 as i32 != *s2 as i32 {
        return *s1 as i32 - *s2 as i32;
    }
    return strcmp(s1, s2);
}
#[no_mangle]
pub unsafe extern "C" fn collapse_continuations(mut line: *mut i8) {
    let mut out: *mut i8 = line;
    let mut in_0: *mut i8 = line;
    let mut q: *mut i8 = 0 as *mut i8;
    q = strchr(in_0, '\n' as i32);
    if q.is_null() {
        return;
    }
    loop {
        let mut p: *mut i8 = q;
        let mut i: i32 = 0;
        let mut out_line_length: size_t = 0;
        if q > line && *q.offset(-(1 as i32) as isize) as i32 == '\\' as i32 {
            i = -(2 as i32);
            while &mut *p.offset(i as isize) as *mut i8 >= line
                && *p.offset(i as isize) as i32 == '\\' as i32
            {
                i -= 1;
                i;
            }
            i += 1;
            i;
        } else {
            i = 0 as i32;
        }
        out_line_length = (p.offset_from(in_0) as i64 + i as i64 - (i / 2 as i32) as i64)
            as size_t;
        if out != in_0 {
            memmove(
                out as *mut libc::c_void,
                in_0 as *const libc::c_void,
                out_line_length,
            );
        }
        out = out.offset(out_line_length as isize);
        in_0 = q.offset(1 as i32 as isize);
        if i & 1 as i32 != 0 {
            while *stopchar_map.as_mut_ptr().offset(*in_0 as u8 as isize) as i32
                & 0x2 as i32 != 0 as i32
            {
                in_0 = in_0.offset(1);
                in_0;
            }
            if posix_pedantic == 0 {
                while out > line
                    && *stopchar_map
                        .as_mut_ptr()
                        .offset(*out.offset(-(1 as i32) as isize) as u8 as isize) as i32
                        & 0x2 as i32 != 0 as i32
                {
                    out = out.offset(-1);
                    out;
                }
            }
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = ' ' as i32 as i8;
        } else {
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = '\n' as i32 as i8;
        }
        q = strchr(in_0, '\n' as i32);
        if q.is_null() {
            break;
        }
    }
    memmove(
        out as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (strlen(in_0)).wrapping_add(1 as i32 as u64),
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_spaces(mut n: u32) {
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 > 0 as i32 as u32) {
            break;
        }
        putchar(' ' as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn concat(mut num: u32, mut args: ...) -> *const i8 {
    static mut rlen: size_t = 0 as i32 as size_t;
    static mut result: *mut i8 = 0 as *const i8 as *mut i8;
    let mut ri: size_t = 0 as i32 as size_t;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    loop {
        let fresh3 = num;
        num = num.wrapping_sub(1);
        if !(fresh3 > 0 as i32 as u32) {
            break;
        }
        let mut s: *const i8 = args_0.arg::<*const i8>();
        let mut l: size_t = if s.is_null() { 0 as i32 as u64 } else { strlen(s) };
        if l == 0 as i32 as u64 {
            continue;
        }
        if ri.wrapping_add(l) > rlen {
            rlen = (if rlen != 0 { rlen } else { 60 as i32 as u64 })
                .wrapping_add(l)
                .wrapping_mul(2 as i32 as u64);
            result = xrealloc(result as *mut libc::c_void, rlen) as *mut i8;
        }
        memcpy(
            result.offset(ri as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            l,
        );
        ri = (ri as u64).wrapping_add(l) as size_t as size_t;
    }
    if ri == rlen {
        rlen = (if rlen != 0 { rlen } else { 60 as i32 as u64 })
            .wrapping_mul(2 as i32 as u64);
        result = xrealloc(result as *mut libc::c_void, rlen) as *mut i8;
    }
    *result.offset(ri as isize) = '\0' as i32 as i8;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn make_pid() -> pid_t {
    return getpid();
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = malloc(
        if size != 0 { size } else { 1 as i32 as u64 },
    );
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = calloc(
        if size != 0 { size } else { 1 as i32 as u64 },
        1 as i32 as u64,
    );
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 {
        size = 1 as i32 as size_t;
    }
    result = if !ptr.is_null() { realloc(ptr, size) } else { malloc(size) };
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut ptr: *const i8) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    result = strdup(ptr);
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xstrndup(mut str: *const i8, mut length: size_t) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    result = strndup(str, length);
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn lindex(
    mut s: *const i8,
    mut limit: *const i8,
    mut c: i32,
) -> *mut i8 {
    while s < limit {
        let fresh4 = s;
        s = s.offset(1);
        if *fresh4 as i32 == c {
            return s.offset(-(1 as i32 as isize)) as *mut i8;
        }
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn end_of_token(mut s: *const i8) -> *mut i8 {
    while !(*stopchar_map.as_mut_ptr().offset(*s as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32 | 0x1 as i32) != 0 as i32)
    {
        s = s.offset(1);
        s;
    }
    return s as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn next_token(mut s: *const i8) -> *mut i8 {
    while *stopchar_map.as_mut_ptr().offset(*s as u8 as isize) as i32
        & (0x2 as i32 | 0x4 as i32) != 0 as i32
    {
        s = s.offset(1);
        s;
    }
    return s as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn find_next_token(
    mut ptr: *mut *const i8,
    mut lengthptr: *mut size_t,
) -> *mut i8 {
    let mut p: *const i8 = next_token(*ptr);
    if *p as i32 == '\0' as i32 {
        return 0 as *mut i8;
    }
    *ptr = end_of_token(p);
    if !lengthptr.is_null() {
        *lengthptr = (*ptr).offset_from(p) as i64 as size_t;
    }
    return p as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn writebuf(
    mut fd: i32,
    mut buffer: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut msg: *const i8 = buffer as *const i8;
    let mut l: size_t = len;
    while l != 0 {
        let mut r: ssize_t = 0;
        loop {
            r = write(fd, msg as *const libc::c_void, l);
            if !(r == -(1 as i32) as i64 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 as i64 {
            return r;
        }
        l = (l as u64).wrapping_sub(r as u64) as size_t as size_t;
        msg = msg.offset(r as isize);
    }
    return len as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn readbuf(
    mut fd: i32,
    mut buffer: *mut libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut msg: *mut i8 = buffer as *mut i8;
    while len != 0 {
        let mut r: ssize_t = 0;
        loop {
            r = read(fd, msg as *mut libc::c_void, len);
            if !(r == -(1 as i32) as i64 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 as i64 {
            return r;
        }
        if r == 0 as i32 as i64 {
            break;
        }
        len = (len as u64).wrapping_sub(r as u64) as size_t as size_t;
        msg = msg.offset(r as isize);
    }
    return msg.offset_from(buffer as *mut i8) as i64;
}
#[no_mangle]
pub unsafe extern "C" fn copy_dep_chain(mut d: *const dep) -> *mut dep {
    let mut firstnew: *mut dep = 0 as *mut dep;
    let mut lastnew: *mut dep = 0 as *mut dep;
    while !d.is_null() {
        let mut c: *mut dep = xmalloc(::core::mem::size_of::<dep>() as u64) as *mut dep;
        memcpy(
            c as *mut libc::c_void,
            d as *const libc::c_void,
            ::core::mem::size_of::<dep>() as u64,
        );
        if (*c).need_2nd_expansion() != 0 {
            (*c).name = xstrdup((*c).name);
        }
        (*c).next = 0 as *mut dep;
        if firstnew.is_null() {
            lastnew = c;
            firstnew = lastnew;
        } else {
            (*lastnew).next = c;
            lastnew = (*lastnew).next;
        }
        d = (*d).next;
    }
    return firstnew;
}
#[no_mangle]
pub unsafe extern "C" fn free_ns_chain(mut ns: *mut nameseq) {
    while !ns.is_null() {
        let mut t: *mut nameseq = ns;
        ns = (*ns).next;
        free(t as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_tmpdir() -> *const i8 {
    static mut tmpdir: *const i8 = 0 as *const i8;
    if tmpdir.is_null() {
        let mut tlist: [*const i8; 3] = [
            b"MAKE_TMPDIR\0" as *const u8 as *const i8,
            b"TMPDIR\0" as *const u8 as *const i8,
            0 as *const i8,
        ];
        let mut tp: *mut *const i8 = 0 as *mut *const i8;
        let mut found: u32 = 0 as i32 as u32;
        tp = tlist.as_mut_ptr();
        while !(*tp).is_null() {
            tmpdir = getenv(*tp);
            if !tmpdir.is_null() && *tmpdir as i32 != '\0' as i32 {
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
                let mut r: i32 = 0;
                found = 1 as i32 as u32;
                loop {
                    r = stat(tmpdir, &mut st);
                    if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                        break;
                    }
                }
                if r < 0 as i32 {
                    error(
                        0 as *mut floc,
                        (strlen(*tp))
                            .wrapping_add(strlen(tmpdir))
                            .wrapping_add(strlen(strerror(*__errno_location()))),
                        dcgettext(
                            0 as *const i8,
                            b"%s value %s: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *tp,
                        tmpdir,
                        strerror(*__errno_location()),
                    );
                } else if !(st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
                {
                    error(
                        0 as *mut floc,
                        (strlen(*tp)).wrapping_add(strlen(tmpdir)),
                        dcgettext(
                            0 as *const i8,
                            b"%s value %s: not a directory\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *tp,
                        tmpdir,
                    );
                } else {
                    return tmpdir
                }
            }
            tp = tp.offset(1);
            tp;
        }
        tmpdir = b"/tmp\0" as *const u8 as *const i8;
        if found != 0 {
            error(
                0 as *mut floc,
                strlen(tmpdir),
                dcgettext(
                    0 as *const i8,
                    b"using default temporary directory '%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                tmpdir,
            );
        }
    }
    return tmpdir;
}
unsafe extern "C" fn get_tmptemplate() -> *mut i8 {
    let mut tmpdir: *const i8 = get_tmpdir();
    let mut template: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    template = xmalloc(
        (strlen(tmpdir))
            .wrapping_add(
                (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
            )
            .wrapping_add(2 as i32 as u64),
    ) as *mut i8;
    cp = stpcpy(template, tmpdir);
    if !(*stopchar_map
        .as_mut_ptr()
        .offset(*cp.offset(-(1 as i32) as isize) as u8 as isize) as i32 & 0x8000 as i32
        != 0 as i32)
    {
        let fresh5 = cp;
        cp = cp.offset(1);
        *fresh5 = '/' as i32 as i8;
    }
    strcpy(cp, b"GmXXXXXX\0" as *const u8 as *const i8);
    return template;
}
#[no_mangle]
pub unsafe extern "C" fn get_tmpfd(mut name: *mut *mut i8) -> i32 {
    let mut fd: i32 = -(1 as i32);
    let mut tmpnm: *mut i8 = 0 as *mut i8;
    let mut mask: mode_t = 0;
    if !name.is_null() {
        *name = 0 as *mut i8;
    } else {
        fd = os_anontmp();
        if fd >= 0 as i32 {
            return fd;
        }
    }
    mask = umask(0o77 as i32 as __mode_t);
    tmpnm = get_tmptemplate();
    loop {
        fd = mkstemp(tmpnm);
        if !(fd == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if fd < 0 as i32 {
        error(
            0 as *mut floc,
            (strlen(tmpnm)).wrapping_add(strlen(strerror(*__errno_location()))),
            dcgettext(
                0 as *const i8,
                b"cannot create temporary file %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            tmpnm,
            strerror(*__errno_location()),
        );
        free(tmpnm as *mut libc::c_void);
        return -(1 as i32);
    }
    if !name.is_null() {
        *name = tmpnm;
    } else {
        let mut r: i32 = 0;
        loop {
            r = unlink(tmpnm);
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 {
            error(
                0 as *mut floc,
                (strlen(tmpnm)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"cannot unlink temporary file %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                tmpnm,
                strerror(*__errno_location()),
            );
        }
        free(tmpnm as *mut libc::c_void);
    }
    umask(mask);
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn get_tmpfile(mut name: *mut *mut i8) -> *mut FILE {
    let mut tmpfile_mode: *const i8 = b"wb+\0" as *const u8 as *const i8;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut fd: i32 = 0;
    fd = get_tmpfd(name);
    if fd < 0 as i32 {
        return 0 as *mut FILE;
    }
    loop {
        *__errno_location() = 0 as i32;
        file = fdopen(fd, tmpfile_mode);
        if !(file.is_null() && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if file.is_null() {
        error(
            0 as *mut floc,
            (strlen(*name)).wrapping_add(strlen(strerror(*__errno_location()))),
            dcgettext(
                0 as *const i8,
                b"fdopen: temporary file %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            *name,
            strerror(*__errno_location()),
        );
    }
    return file;
}
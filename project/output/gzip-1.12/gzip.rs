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
    pub type __dirstream;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: i32) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigismember(__set: *const sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn fchmod(__fd: i32, __mode: __mode_t) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __errno_location() -> *mut i32;
    static mut header_bytes: off_t;
    fn zip(in_0: i32, out: i32) -> i32;
    static mut unzip_crc: ulg;
    fn unzip(in_0: i32, out: i32) -> i32;
    fn check_zipfile(in_0: i32) -> i32;
    fn unpack(in_0: i32, out: i32) -> i32;
    fn unlzh(in_0: i32, out: i32) -> i32;
    fn perror(__s: *const i8);
    fn rpl_fclose(stream: *mut FILE) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn rpl_fprintf(fp: *mut FILE, format: *const i8, _: ...) -> i32;
    fn __printf__(format: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    fn xunlink(fname: *mut i8) -> i32;
    fn copy(in_0: i32, out: i32) -> i32;
    fn updcrc(s: *const uch, n: u32) -> ulg;
    fn clear_bufs();
    fn fill_inbuf(eof_ok: i32) -> i32;
    fn write_buf(fd: i32, buf: voidp, cnt: u32);
    fn strlwr(s: *mut i8) -> *mut i8;
    fn gzip_base_name(fname: *mut i8) -> *mut i8;
    fn add_envopt(
        argcp: *mut i32,
        argvp: *mut *mut *mut i8,
        env_0: *const i8,
    ) -> *mut i8;
    fn gzip_error(m: *const i8);
    fn read_error();
    fn write_error();
    fn display_ratio(num: off_t, den: off_t, file: *mut FILE);
    fn fprint_off(_: *mut FILE, _: off_t, _: i32);
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn unlzw(in_0: i32, out: i32) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    fn last_component(filename: *const i8) -> *mut i8;
    fn openat_safer(_: i32, _: *const i8, _: i32, _: ...) -> i32;
    fn open_safer(_: *const i8, _: i32, _: ...) -> i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    static mut Version: *const i8;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn yesno() -> bool;
    fn close(__fd: i32) -> i32;
    fn fchown(__fd: i32, __owner: __uid_t, __group: __gid_t) -> i32;
    fn _exit(_: i32) -> !;
    fn isatty(__fd: i32) -> i32;
    fn unlinkat(__fd: i32, __name: *const i8, __flag: i32) -> i32;
    fn fsync(__fd: i32) -> i32;
    fn fdatasync(__fildes: i32) -> i32;
    fn fdopendir(__fd: i32) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn streamsavedir(_: *mut DIR, _: savedir_option) -> *mut i8;
    fn fdutimens(_: i32, _: *const i8, _: *const timespec) -> i32;
}
pub type __uint32_t = u32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type ptrdiff_t = i64;
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
pub type voidp = *mut libc::c_void;
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
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type ulg = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_10 {
    TIMESPEC_RESOLUTION = 1000000000,
}
impl C2RustUnnamed_10 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_10::TIMESPEC_RESOLUTION => 1000000000,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_10 {
        match value {
            1000000000 => C2RustUnnamed_10::TIMESPEC_RESOLUTION,
            _ => panic!("Invalid value for C2RustUnnamed_10: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_10 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_10 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_10 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_10 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_10 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn add(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn sub(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn mul(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn div(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn rem(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
pub type DIR = __dirstream;
pub type savedir_option = u32;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 0;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
    ENV_OPTION = 131,
    SYNCHRONOUS_OPTION = 130,
    RSYNCABLE_OPTION = 129,
    PRESUME_INPUT_TTY_OPTION = 128,
}
impl C2RustUnnamed_11 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_11::ENV_OPTION => 131,
            C2RustUnnamed_11::SYNCHRONOUS_OPTION => 130,
            C2RustUnnamed_11::RSYNCABLE_OPTION => 129,
            C2RustUnnamed_11::PRESUME_INPUT_TTY_OPTION => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_11 {
        match value {
            131 => C2RustUnnamed_11::ENV_OPTION,
            130 => C2RustUnnamed_11::SYNCHRONOUS_OPTION,
            129 => C2RustUnnamed_11::RSYNCABLE_OPTION,
            128 => C2RustUnnamed_11::PRESUME_INPUT_TTY_OPTION,
            _ => panic!("Invalid value for C2RustUnnamed_11: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_11 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_11 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_11 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_11 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_11 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn add(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn sub(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn mul(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn div(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn rem(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_12 {
    try_opening_directories = 1,
}
impl C2RustUnnamed_12 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_12::try_opening_directories => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_12 {
        match value {
            1 => C2RustUnnamed_12::try_opening_directories,
            _ => panic!("Invalid value for C2RustUnnamed_12: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_12 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_12 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_12 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_12 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_12 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn add(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn sub(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn mul(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn div(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn rem(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
static mut license_msg: [*const i8; 6] = [
    b"Copyright (C) 2018 Free Software Foundation, Inc.\0" as *const u8 as *const i8,
    b"Copyright (C) 1993 Jean-loup Gailly.\0" as *const u8 as *const i8,
    b"This is free software.  You may redistribute copies of it under the terms of\0"
        as *const u8 as *const i8,
    b"the GNU General Public License <https://www.gnu.org/licenses/gpl.html>.\0"
        as *const u8 as *const i8,
    b"There is NO WARRANTY, to the extent permitted by law.\0" as *const u8 as *const i8,
    0 as *const i8,
];
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[no_mangle]
pub static mut inbuf: [uch; 262208] = [0; 262208];
#[no_mangle]
pub static mut outbuf: [uch; 264192] = [0; 264192];
#[no_mangle]
pub static mut d_buf: [ush; 32768] = [0; 32768];
#[no_mangle]
pub static mut window: [uch; 65536] = [0; 65536];
#[no_mangle]
pub static mut prev: [ush; 65536] = [0; 65536];
static mut presume_input_tty: bool = false;
static mut synchronous: bool = false;
static mut ascii: i32 = 0 as i32;
#[no_mangle]
pub static mut to_stdout: i32 = 0 as i32;
static mut decompress: i32 = 0 as i32;
static mut force: i32 = 0 as i32;
static mut keep: i32 = 0 as i32;
static mut no_name: i32 = -(1 as i32);
static mut no_time: i32 = -(1 as i32);
static mut recursive: i32 = 0 as i32;
static mut list: i32 = 0 as i32;
static mut verbose: i32 = 0 as i32;
#[no_mangle]
pub static mut quiet: i32 = 0 as i32;
#[no_mangle]
pub static mut test: i32 = 0 as i32;
static mut foreground: i32 = 0 as i32;
#[no_mangle]
pub static mut program_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut maxbits: i32 = 16 as i32;
#[no_mangle]
pub static mut method: i32 = 8 as i32;
#[no_mangle]
pub static mut level: i32 = 6 as i32;
#[no_mangle]
pub static mut exit_code: i32 = 0 as i32;
#[no_mangle]
pub static mut save_orig_name: i32 = 0;
static mut last_member: i32 = 0;
static mut part_nb: i32 = 0;
#[no_mangle]
pub static mut ifile_size: off_t = 0;
static mut env: *mut i8 = 0 as *const i8 as *mut i8;
static mut z_suffix: *const i8 = 0 as *const i8;
static mut z_len: size_t = 0;
#[no_mangle]
pub static mut time_stamp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut exiting_signal: i32 = 0;
static mut remove_ofname_fd: i32 = -(1 as i32);
static mut remove_ofname: [i8; 1024] = [0; 1024];
static mut stdin_was_read: bool = false;
#[no_mangle]
pub static mut bytes_in: off_t = 0;
#[no_mangle]
pub static mut bytes_out: off_t = 0;
static mut total_in: off_t = 0;
static mut total_out: off_t = 0;
#[no_mangle]
pub static mut ifname: [i8; 1024] = [0; 1024];
#[no_mangle]
pub static mut ofname: [i8; 1024] = [0; 1024];
static mut dfname: [i8; 1024] = [0; 1024];
static mut istat: stat = stat {
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
#[no_mangle]
pub static mut ifd: i32 = 0;
#[no_mangle]
pub static mut ofd: i32 = 0;
static mut dfd: i32 = -(1 as i32);
#[no_mangle]
pub static mut insize: u32 = 0;
#[no_mangle]
pub static mut inptr: u32 = 0;
#[no_mangle]
pub static mut outcnt: u32 = 0;
#[no_mangle]
pub static mut rsync: i32 = 0 as i32;
static mut handled_sig: [i32; 6] = [
    2 as i32,
    1 as i32,
    13 as i32,
    15 as i32,
    24 as i32,
    25 as i32,
];
static mut shortopts: [i8; 34] = unsafe {
    *::core::mem::transmute::<
        &[u8; 34],
        &[i8; 34],
    >(b"ab:cdfhH?klLmMnNqrS:tvVZ123456789\0")
};
static mut longopts: [option; 27] = [
    {
        let mut init = option {
            name: b"ascii\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"to-stdout\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdout\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"decompress\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"uncompress\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"license\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-name\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"name\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-input-tty\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: C2RustUnnamed_11::PRESUME_INPUT_TTY_OPTION as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"synchronous\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: C2RustUnnamed_11::SYNCHRONOUS_OPTION as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"test\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"fast\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: '1' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"best\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: '9' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lzw\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bits\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"rsyncable\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: C2RustUnnamed_11::RSYNCABLE_OPTION as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 0 as i32,
        };
        init
    },
];
static mut work: Option<unsafe extern "C" fn(i32, i32) -> i32> = unsafe {
    Some(zip as unsafe extern "C" fn(i32, i32) -> i32)
};
unsafe extern "C" fn try_help() {
    rpl_fprintf(
        stderr,
        b"Try `%s --help' for more information.\n\0" as *const u8 as *const i8,
        program_name,
    );
    do_exit(1 as i32);
}
unsafe extern "C" fn help() {
    static mut help_msg: [*const i8; 28] = [
        b"Compress or uncompress FILEs (by default, compress FILES in-place).\0"
            as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"Mandatory arguments to long options are mandatory for short options too.\0"
            as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"  -c, --stdout      write on standard output, keep original files unchanged\0"
            as *const u8 as *const i8,
        b"  -d, --decompress  decompress\0" as *const u8 as *const i8,
        b"  -f, --force       force overwrite of output file and compress links\0"
            as *const u8 as *const i8,
        b"  -h, --help        give this help\0" as *const u8 as *const i8,
        b"  -k, --keep        keep (don't delete) input files\0" as *const u8
            as *const i8,
        b"  -l, --list        list compressed file contents\0" as *const u8 as *const i8,
        b"  -L, --license     display software license\0" as *const u8 as *const i8,
        b"  -n, --no-name     do not save or restore the original name and timestamp\0"
            as *const u8 as *const i8,
        b"  -N, --name        save or restore the original name and timestamp\0"
            as *const u8 as *const i8,
        b"  -q, --quiet       suppress all warnings\0" as *const u8 as *const i8,
        b"  -r, --recursive   operate recursively on directories\0" as *const u8
            as *const i8,
        b"      --rsyncable   make rsync-friendly archive\0" as *const u8 as *const i8,
        b"  -S, --suffix=SUF  use suffix SUF on compressed files\0" as *const u8
            as *const i8,
        b"      --synchronous synchronous output (safer if system crashes, but slower)\0"
            as *const u8 as *const i8,
        b"  -t, --test        test compressed file integrity\0" as *const u8
            as *const i8,
        b"  -v, --verbose     verbose mode\0" as *const u8 as *const i8,
        b"  -V, --version     display version number\0" as *const u8 as *const i8,
        b"  -1, --fast        compress faster\0" as *const u8 as *const i8,
        b"  -9, --best        compress better\0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"With no FILE, or when FILE is -, read standard input.\0" as *const u8
            as *const i8,
        b"\0" as *const u8 as *const i8,
        b"Report bugs to <bug-gzip@gnu.org>.\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut p: *const *const i8 = help_msg.as_ptr();
    __printf__(
        b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8 as *const i8,
        program_name,
    );
    while !(*p).is_null() {
        let fresh0 = p;
        p = p.offset(1);
        __printf__(b"%s\n\0" as *const u8 as *const i8, *fresh0);
    }
}
unsafe extern "C" fn license() {
    let mut p: *const *const i8 = license_msg.as_ptr();
    __printf__(b"%s %s\n\0" as *const u8 as *const i8, program_name, Version);
    while !(*p).is_null() {
        let fresh1 = p;
        p = p.offset(1);
        __printf__(b"%s\n\0" as *const u8 as *const i8, *fresh1);
    }
}
unsafe extern "C" fn version() {
    license();
    __printf__(b"\n\0" as *const u8 as *const i8);
    __printf__(b"Written by Jean-loup Gailly.\n\0" as *const u8 as *const i8);
}
unsafe extern "C" fn progerror(mut string: *const i8) {
    let mut e: i32 = *__errno_location();
    rpl_fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program_name);
    *__errno_location() = e;
    perror(string);
    exit_code = 1 as i32;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut file_count: i32 = 0;
    let mut proglen: size_t = 0;
    let mut argv_copy: *mut *mut i8 = 0 as *mut *mut i8;
    let mut env_argc: i32 = 0;
    let mut env_argv: *mut *mut i8 = 0 as *mut *mut i8;
    program_name = gzip_base_name(*argv.offset(0 as i32 as isize));
    proglen = strlen(program_name);
    if (4 as i32 as u64) < proglen
        && strcmp(
            program_name.offset(proglen as isize).offset(-(4 as i32 as isize)),
            b".exe\0" as *const u8 as *const i8,
        ) == 0 as i32
    {
        *program_name.offset(proglen.wrapping_sub(4 as i32 as u64) as isize) = '\0'
            as i32 as i8;
    }
    argv_copy = argv;
    env = add_envopt(&mut env_argc, &mut argv_copy, b"GZIP\0" as *const u8 as *const i8);
    env_argv = if !env.is_null() { argv_copy } else { 0 as *mut *mut i8 };
    z_suffix = b".gz\0" as *const u8 as *const i8;
    z_len = strlen(z_suffix);
    loop {
        let mut optc: i32 = 0;
        let mut longind: i32 = -(1 as i32);
        if !env_argv.is_null() {
            if !(*env_argv.offset(optind as isize)).is_null()
                && strcmp(
                    *env_argv.offset(optind as isize),
                    b"--\0" as *const u8 as *const i8,
                ) == 0 as i32
            {
                optc = C2RustUnnamed_11::ENV_OPTION as i32 + '-' as i32;
            } else {
                optc = getopt_long(
                    env_argc,
                    env_argv,
                    shortopts.as_ptr(),
                    longopts.as_ptr(),
                    &mut longind,
                );
                if 0 as i32 <= optc {
                    optc += C2RustUnnamed_11::ENV_OPTION as i32;
                } else {
                    if optind != env_argc {
                        rpl_fprintf(
                            stderr,
                            b"%s: %s: non-option in GZIP environment variable\n\0"
                                as *const u8 as *const i8,
                            program_name,
                            *env_argv.offset(optind as isize),
                        );
                        try_help();
                    }
                    if env_argc != 1 as i32 && quiet == 0 {
                        rpl_fprintf(
                            stderr,
                            b"%s: warning: GZIP environment variable is deprecated; use an alias or script\n\0"
                                as *const u8 as *const i8,
                            program_name,
                        );
                    }
                    rpl_free(env_argv as *mut libc::c_void);
                    env_argv = 0 as *mut *mut i8;
                    optind = 1 as i32;
                    longind = -(1 as i32);
                }
            }
        }
        if env_argv.is_null() {
            optc = getopt_long(
                argc,
                argv,
                shortopts.as_ptr(),
                longopts.as_ptr(),
                &mut longind,
            );
        }
        if optc < 0 as i32 {
            break;
        }
        let mut current_block_76: u64;
        match optc {
            97 => {
                ascii = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            98 => {
                maxbits = atoi(optarg);
                while *optarg != 0 {
                    if !('0' as i32 <= *optarg as i32 && *optarg as i32 <= '9' as i32) {
                        rpl_fprintf(
                            stderr,
                            b"%s: -b operand is not an integer\n\0" as *const u8
                                as *const i8,
                            program_name,
                        );
                        try_help();
                    }
                    optarg = optarg.offset(1);
                    optarg;
                }
                current_block_76 = 15864857819291709765;
            }
            99 => {
                to_stdout = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            100 => {
                decompress = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            102 => {
                force += 1;
                force;
                current_block_76 = 15864857819291709765;
            }
            104 | 72 => {
                help();
                finish_out();
                current_block_76 = 15864857819291709765;
            }
            107 => {
                keep = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            108 => {
                to_stdout = 1 as i32;
                test = to_stdout;
                decompress = test;
                list = decompress;
                current_block_76 = 15864857819291709765;
            }
            76 => {
                license();
                finish_out();
                current_block_76 = 15864857819291709765;
            }
            109 => {
                no_time = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            77 => {
                no_time = 0 as i32;
                current_block_76 = 15864857819291709765;
            }
            110 | 241 => {
                no_time = 1 as i32;
                no_name = no_time;
                current_block_76 = 15864857819291709765;
            }
            78 | 209 => {
                no_time = 0 as i32;
                no_name = no_time;
                current_block_76 = 15864857819291709765;
            }
            128 => {
                presume_input_tty = 1 as i32 != 0;
                current_block_76 = 15864857819291709765;
            }
            113 | 244 => {
                quiet = 1 as i32;
                verbose = 0 as i32;
                current_block_76 = 15864857819291709765;
            }
            114 => {
                recursive = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            129 | 260 => {
                rsync = 1 as i32;
                current_block_76 = 15864857819291709765;
            }
            83 => {
                z_len = strlen(optarg);
                z_suffix = optarg;
                current_block_76 = 15864857819291709765;
            }
            130 => {
                synchronous = 1 as i32 != 0;
                current_block_76 = 15864857819291709765;
            }
            116 => {
                to_stdout = 1 as i32;
                decompress = to_stdout;
                test = decompress;
                current_block_76 = 15864857819291709765;
            }
            118 | 249 => {
                verbose += 1;
                verbose;
                quiet = 0 as i32;
                current_block_76 = 15864857819291709765;
            }
            86 => {
                version();
                finish_out();
                current_block_76 = 15864857819291709765;
            }
            90 => {
                rpl_fprintf(
                    stderr,
                    b"%s: -Z not supported in this version\n\0" as *const u8
                        as *const i8,
                    program_name,
                );
                try_help();
                current_block_76 = 15864857819291709765;
            }
            180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 => {
                optc -= C2RustUnnamed_11::ENV_OPTION as i32;
                current_block_76 = 4611508176128573306;
            }
            49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                current_block_76 = 4611508176128573306;
            }
            _ => {
                if C2RustUnnamed_11::ENV_OPTION as i32 <= optc
                    && optc != C2RustUnnamed_11::ENV_OPTION as i32 + '?' as i32
                {
                    rpl_fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const i8,
                        program_name,
                    );
                    if longind < 0 as i32 {
                        rpl_fprintf(
                            stderr,
                            b"-%c: \0" as *const u8 as *const i8,
                            optc - C2RustUnnamed_11::ENV_OPTION as i32,
                        );
                    } else {
                        rpl_fprintf(
                            stderr,
                            b"--%s: \0" as *const u8 as *const i8,
                            longopts[longind as usize].name,
                        );
                    }
                    rpl_fprintf(
                        stderr,
                        b"option not valid in GZIP environment variable\n\0" as *const u8
                            as *const i8,
                    );
                }
                try_help();
                current_block_76 = 15864857819291709765;
            }
        }
        match current_block_76 {
            4611508176128573306 => {
                level = optc - '0' as i32;
            }
            _ => {}
        }
    }
    if no_time < 0 as i32 {
        no_time = decompress;
    }
    if no_name < 0 as i32 {
        no_name = decompress;
    }
    file_count = argc - optind;
    if ascii != 0 && quiet == 0 {
        rpl_fprintf(
            stderr,
            b"%s: option --ascii ignored on this system\n\0" as *const u8 as *const i8,
            program_name,
        );
    }
    if z_len == 0 as i32 as u64 || z_len > 30 as i32 as u64 {
        rpl_fprintf(
            stderr,
            b"%s: invalid suffix '%s'\n\0" as *const u8 as *const i8,
            program_name,
            z_suffix,
        );
        do_exit(1 as i32);
    }
    ::core::ptr::write_volatile(
        &mut exiting_signal as *mut i32,
        if quiet != 0 { 13 as i32 } else { 0 as i32 },
    );
    install_signal_handlers();
    if file_count != 0 as i32 {
        to_stdout != 0 && test == 0 && (decompress == 0 || ascii == 0);
        while optind < argc {
            let fresh2 = optind;
            optind = optind + 1;
            treat_file(*argv.offset(fresh2 as isize));
        }
    } else {
        treat_stdin();
    }
    if stdin_was_read as i32 != 0 && close(0 as i32) != 0 as i32 {
        strcpy(ifname.as_mut_ptr(), b"stdin\0" as *const u8 as *const i8);
        read_error();
    }
    if list != 0 {
        if quiet == 0 && (1 as i32) < file_count {
            do_list(-(1 as i32));
        }
        if rpl_fflush(stdout) != 0 as i32 {
            write_error();
        }
    }
    if to_stdout != 0
        && (synchronous as i32 != 0 && fdatasync(1 as i32) != 0 as i32
            && *__errno_location() != 22 as i32 || close(1 as i32) != 0 as i32)
        && *__errno_location() != 9 as i32
    {
        write_error();
    }
    do_exit(exit_code);
    return 0;
}
unsafe extern "C" fn input_eof() -> i32 {
    if decompress == 0 || last_member != 0 {
        return 1 as i32;
    }
    if inptr == insize {
        if insize != 0x40000 as i32 as u32 || fill_inbuf(1 as i32) == -(1 as i32) {
            return 1 as i32;
        }
        inptr = 0 as i32 as u32;
    }
    return 0 as i32;
}
unsafe extern "C" fn get_input_size_and_time() {
    ifile_size = -(1 as i32) as off_t;
    time_stamp.tv_nsec = -(1 as i32) as __syscall_slong_t;
    if istat.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
        ifile_size = istat.st_size;
        if no_time == 0 || list != 0 {
            time_stamp = get_stat_mtime(&mut istat);
        }
    }
}
unsafe extern "C" fn treat_stdin() {
    if force == 0 && list == 0
        && (presume_input_tty as i32 != 0
            || isatty((if decompress != 0 { 0 as i32 } else { 1 as i32 })) != 0)
    {
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n\0"
                    as *const u8 as *const i8,
                program_name,
                if decompress != 0 {
                    b"read from\0" as *const u8 as *const i8
                } else {
                    b"written to\0" as *const u8 as *const i8
                },
                if decompress != 0 {
                    b"de\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
                program_name,
            );
        }
        do_exit(1 as i32);
    }
    decompress != 0 || ascii == 0;
    test == 0 && (decompress == 0 || ascii == 0);
    strcpy(ifname.as_mut_ptr(), b"stdin\0" as *const u8 as *const i8);
    strcpy(ofname.as_mut_ptr(), b"stdout\0" as *const u8 as *const i8);
    if fstat(0 as i32, &mut istat) != 0 as i32 {
        progerror(b"standard input\0" as *const u8 as *const i8);
        do_exit(1 as i32);
    }
    get_input_size_and_time();
    clear_bufs();
    to_stdout = 1 as i32;
    part_nb = 0 as i32;
    ifd = 0 as i32;
    stdin_was_read = 1 as i32 != 0;
    if decompress != 0 {
        method = get_method(ifd);
        if method < 0 as i32 {
            do_exit(exit_code);
        }
    }
    loop {
        if work.expect("non-null function pointer")(0 as i32, 1 as i32) != 0 as i32 {
            return;
        }
        if input_eof() != 0 {
            break;
        }
        method = get_method(ifd);
        if method < 0 as i32 {
            return;
        }
        bytes_out = 0 as i32 as off_t;
    }
    if list != 0 {
        do_list(method);
        return;
    }
    if verbose != 0 {
        if test != 0 {
            rpl_fprintf(stderr, b" OK\n\0" as *const u8 as *const i8);
        } else if decompress == 0 {
            display_ratio(bytes_in - (bytes_out - header_bytes), bytes_in, stderr);
            rpl_fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        }
    }
}
static mut dot: i8 = '.' as i32 as i8;
unsafe extern "C" fn atdir_eq(mut dir: *const i8, mut dirlen: ptrdiff_t) -> bool {
    if dirlen == 0 as i32 as i64 {
        dir = &dot;
        dirlen = 1 as i32 as ptrdiff_t;
    }
    return memcmp(
        dfname.as_mut_ptr() as *const libc::c_void,
        dir as *const libc::c_void,
        dirlen as u64,
    ) == 0 as i32 && dfname[dirlen as usize] == 0;
}
unsafe extern "C" fn atdir_set(mut dir: *const i8, mut dirlen: ptrdiff_t) -> i32 {
    if C2RustUnnamed_12::try_opening_directories as i32 != 0 && !atdir_eq(dir, dirlen) {
        if 0 as i32 <= dfd {
            close(dfd);
        }
        if dirlen == 0 as i32 as i64 {
            dir = &dot;
            dirlen = 1 as i32 as ptrdiff_t;
        }
        memcpy(
            dfname.as_mut_ptr() as *mut libc::c_void,
            dir as *const libc::c_void,
            dirlen as u64,
        );
        dfname[dirlen as usize] = '\0' as i32 as i8;
        dfd = open_safer(dfname.as_mut_ptr(), 0 as i32 | 0o200000 as i32);
    }
    return dfd;
}
unsafe extern "C" fn treat_file(mut iname: *mut i8) {
    if strcmp(iname, b"-\0" as *const u8 as *const i8) == 0 as i32 {
        let mut cflag: i32 = to_stdout;
        treat_stdin();
        to_stdout = cflag;
        return;
    }
    ifd = open_input_file(iname, &mut istat);
    if ifd < 0 as i32 {
        return;
    }
    if istat.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
        if recursive != 0 {
            treat_dir(ifd, iname);
            return;
        }
        close(ifd);
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s is a directory -- ignored\n\0" as *const u8 as *const i8,
                program_name,
                ifname.as_mut_ptr(),
            );
        }
        if exit_code == 0 as i32 {
            exit_code = 2 as i32;
        }
        return;
    }
    if to_stdout == 0 {
        if !(istat.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s is not a directory or a regular file - ignored\n\0"
                        as *const u8 as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as i32 {
                exit_code = 2 as i32;
            }
            close(ifd);
            return;
        }
        if istat.st_mode & 0o4000 as i32 as u32 != 0 {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s is set-user-ID on execution - ignored\n\0" as *const u8
                        as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as i32 {
                exit_code = 2 as i32;
            }
            close(ifd);
            return;
        }
        if istat.st_mode & 0o2000 as i32 as u32 != 0 {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s is set-group-ID on execution - ignored\n\0" as *const u8
                        as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as i32 {
                exit_code = 2 as i32;
            }
            close(ifd);
            return;
        }
        if force == 0 {
            if istat.st_mode & 0o1000 as i32 as u32 != 0 {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s has the sticky bit set - file ignored\n\0" as *const u8
                            as *const i8,
                        program_name,
                        ifname.as_mut_ptr(),
                    );
                }
                if exit_code == 0 as i32 {
                    exit_code = 2 as i32;
                }
                close(ifd);
                return;
            }
            if 2 as i32 as u64 <= istat.st_nlink {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s has %lu other link%s -- file ignored\n\0" as *const u8
                            as *const i8,
                        program_name,
                        ifname.as_mut_ptr(),
                        (istat.st_nlink).wrapping_sub(1 as i32 as u64),
                        if istat.st_nlink == 2 as i32 as u64 {
                            b"\0" as *const u8 as *const i8
                        } else {
                            b"s\0" as *const u8 as *const i8
                        },
                    );
                }
                if exit_code == 0 as i32 {
                    exit_code = 2 as i32;
                }
                close(ifd);
                return;
            }
        }
    }
    get_input_size_and_time();
    if to_stdout != 0 && test == 0 {
        strcpy(ofname.as_mut_ptr(), b"stdout\0" as *const u8 as *const i8);
    } else if make_ofname() != 0 as i32 {
        close(ifd);
        return;
    }
    clear_bufs();
    part_nb = 0 as i32;
    if decompress != 0 {
        method = get_method(ifd);
        if method < 0 as i32 {
            close(ifd);
            return;
        }
    }
    if to_stdout != 0 {
        ofd = 1 as i32;
    } else {
        if create_outfile() != 0 as i32 {
            return;
        }
        if decompress == 0 && save_orig_name != 0 && verbose == 0 && quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s compressed to %s\n\0" as *const u8 as *const i8,
                program_name,
                ifname.as_mut_ptr(),
                ofname.as_mut_ptr(),
            );
        }
    }
    if save_orig_name == 0 {
        save_orig_name = (no_name == 0) as i32;
    }
    if verbose != 0 && list == 0 {
        rpl_fprintf(stderr, b"%s:\t\0" as *const u8 as *const i8, ifname.as_mut_ptr());
    }
    loop {
        if (Some(work.expect("non-null function pointer")))
            .expect("non-null function pointer")(ifd, ofd) != 0 as i32
        {
            method = -(1 as i32);
            break;
        } else {
            if input_eof() != 0 {
                break;
            }
            method = get_method(ifd);
            if method < 0 as i32 {
                break;
            }
            bytes_out = 0 as i32 as off_t;
        }
    }
    if close(ifd) != 0 as i32 {
        read_error();
    }
    if list != 0 {
        do_list(method);
        return;
    }
    if to_stdout == 0 {
        copy_stat(&mut istat);
        if synchronous as i32 != 0
            && (0 as i32 <= dfd && fdatasync(dfd) != 0 as i32
                && *__errno_location() != 22 as i32
                || fsync(ofd) != 0 as i32 && *__errno_location() != 22 as i32)
            || close(ofd) != 0 as i32
        {
            write_error();
        }
        if keep == 0 {
            let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
            let mut unlink_errno: i32 = 0;
            let mut ifbase: *mut i8 = last_component(ifname.as_mut_ptr());
            let mut ufd: i32 = if atdir_eq(
                ifname.as_mut_ptr(),
                ifbase.offset_from(ifname.as_mut_ptr()) as i64,
            ) as i32 != 0
            {
                dfd
            } else {
                -(1 as i32)
            };
            let mut res: i32 = 0;
            sigprocmask(0 as i32, &mut caught_signals, &mut oldset);
            ::core::ptr::write_volatile(&mut remove_ofname_fd as *mut i32, -(1 as i32));
            res = if ufd < 0 as i32 {
                xunlink(ifname.as_mut_ptr())
            } else {
                unlinkat(ufd, ifbase, 0 as i32)
            };
            unlink_errno = if res == 0 as i32 { 0 as i32 } else { *__errno_location() };
            sigprocmask(2 as i32, &mut oldset, 0 as *mut sigset_t);
            if unlink_errno != 0 {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const i8,
                        program_name,
                    );
                }
                if exit_code == 0 as i32 {
                    exit_code = 2 as i32;
                }
                if quiet == 0 {
                    *__errno_location() = unlink_errno;
                    perror(ifname.as_mut_ptr());
                }
            }
        }
    }
    if method == -(1 as i32) {
        if to_stdout == 0 {
            remove_output_file(0 as i32 != 0);
        }
        return;
    }
    if verbose != 0 {
        if test != 0 {
            rpl_fprintf(stderr, b" OK\0" as *const u8 as *const i8);
        } else if decompress != 0 {
            display_ratio(bytes_out - (bytes_in - header_bytes), bytes_out, stderr);
        } else {
            display_ratio(bytes_in - (bytes_out - header_bytes), bytes_in, stderr);
        }
        if test == 0 {
            rpl_fprintf(
                stderr,
                b" -- %s %s\0" as *const u8 as *const i8,
                if keep != 0 {
                    b"created\0" as *const u8 as *const i8
                } else {
                    b"replaced with\0" as *const u8 as *const i8
                },
                ofname.as_mut_ptr(),
            );
        }
        rpl_fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn volatile_strcpy(mut dst: *mut i8, mut src: *const i8) {
    loop {
        let fresh3 = src;
        src = src.offset(1);
        let fresh4 = dst;
        dst = dst.offset(1);
        ::core::ptr::write_volatile(fresh4, *fresh3);
        if !(::core::ptr::read_volatile::<i8>(fresh4 as *const i8) != 0) {
            break;
        }
    };
}
unsafe extern "C" fn create_outfile() -> i32 {
    let mut name_shortened: i32 = 0 as i32;
    let mut flags: i32 = 0o1 as i32 | 0o100 as i32 | 0o200 as i32
        | (if ascii != 0 && decompress != 0 { 0 as i32 } else { 0 as i32 });
    let mut base: *const i8 = ofname.as_mut_ptr();
    let mut atfd: i32 = -(100 as i32);
    if keep == 0 {
        let mut b: *const i8 = last_component(ofname.as_mut_ptr());
        let mut f: i32 = atdir_set(
            ofname.as_mut_ptr(),
            b.offset_from(ofname.as_mut_ptr()) as i64,
        );
        if 0 as i32 <= f {
            base = b;
            atfd = f;
        }
    }
    loop {
        let mut open_errno: i32 = 0;
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        volatile_strcpy(remove_ofname.as_mut_ptr(), ofname.as_mut_ptr());
        sigprocmask(0 as i32, &mut caught_signals, &mut oldset);
        ofd = openat_safer(atfd, base, flags, 0o400 as i32 | 0o200 as i32);
        ::core::ptr::write_volatile(&mut remove_ofname_fd as *mut i32, ofd);
        open_errno = *__errno_location();
        sigprocmask(2 as i32, &mut oldset, 0 as *mut sigset_t);
        if 0 as i32 <= ofd {
            break;
        }
        match open_errno {
            36 => {
                shorten_name(ofname.as_mut_ptr());
                name_shortened = 1 as i32;
            }
            17 => {
                if check_ofname() != 0 as i32 {
                    close(ifd);
                    return 1 as i32;
                }
            }
            _ => {
                progerror(ofname.as_mut_ptr());
                close(ifd);
                return 1 as i32;
            }
        }
    }
    if name_shortened != 0 && decompress != 0 {
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s: warning, name truncated\n\0" as *const u8 as *const i8,
                program_name,
                ofname.as_mut_ptr(),
            );
        }
        if exit_code == 0 as i32 {
            exit_code = 2 as i32;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn get_suffix(mut name: *mut i8) -> *mut i8 {
    let mut nlen: i32 = 0;
    let mut slen: i32 = 0;
    let mut suffix: [i8; 33] = [0; 33];
    static mut known_suffixes: [*const i8; 10] = [
        0 as *const i8,
        b".gz\0" as *const u8 as *const i8,
        b".z\0" as *const u8 as *const i8,
        b".taz\0" as *const u8 as *const i8,
        b".tgz\0" as *const u8 as *const i8,
        b"-gz\0" as *const u8 as *const i8,
        b"-z\0" as *const u8 as *const i8,
        b"_z\0" as *const u8 as *const i8,
        0 as *const i8,
        0 as *const i8,
    ];
    let mut suf: *mut *const i8 = 0 as *mut *const i8;
    let mut suffix_of_builtin: bool = 0 as i32 != 0;
    suf = known_suffixes.as_mut_ptr().offset(1 as i32 as isize);
    while !(*suf).is_null() {
        let mut suflen: size_t = strlen(*suf);
        if z_len < suflen
            && strcmp(z_suffix, (*suf).offset(suflen as isize).offset(-(z_len as isize)))
                == 0 as i32
        {
            suffix_of_builtin = 1 as i32 != 0;
            break;
        } else {
            suf = suf.offset(1);
            suf;
        }
    }
    let mut z_lower: *mut i8 = xstrdup(z_suffix);
    strlwr(z_lower);
    known_suffixes[(if suffix_of_builtin as i32 != 0 {
        (::core::mem::size_of::<[*const i8; 10]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
            .wrapping_sub(2 as i32 as u64)
    } else {
        0 as i32 as u64
    }) as usize] = z_lower;
    suf = known_suffixes.as_mut_ptr().offset(suffix_of_builtin as i32 as isize);
    nlen = strlen(name) as i32;
    if nlen <= 30 as i32 + 2 as i32 {
        strcpy(suffix.as_mut_ptr(), name);
    } else {
        strcpy(
            suffix.as_mut_ptr(),
            name
                .offset(nlen as isize)
                .offset(-(30 as i32 as isize))
                .offset(-(2 as i32 as isize)),
        );
    }
    strlwr(suffix.as_mut_ptr());
    slen = strlen(suffix.as_mut_ptr()) as i32;
    let mut match_0: *mut i8 = 0 as *mut i8;
    loop {
        let mut s: i32 = strlen(*suf) as i32;
        if slen > s && !(suffix[(slen - s - 1 as i32) as usize] as i32 == '/' as i32)
            && strcmp(
                suffix.as_mut_ptr().offset(slen as isize).offset(-(s as isize)),
                *suf,
            ) == 0 as i32
        {
            match_0 = name.offset(nlen as isize).offset(-(s as isize));
            break;
        } else {
            suf = suf.offset(1);
            if (*suf).is_null() {
                break;
            }
        }
    }
    rpl_free(z_lower as *mut libc::c_void);
    return match_0;
}
unsafe extern "C" fn open_and_stat(
    mut name: *mut i8,
    mut flags: i32,
    mut st: *mut stat,
) -> i32 {
    let mut fd: i32 = 0;
    let mut atfd: i32 = -(100 as i32);
    let mut base: *const i8 = name;
    if to_stdout == 0 && force == 0 {
        flags |= 0o400000 as i32;
    }
    if keep == 0 {
        let mut b: *const i8 = last_component(name);
        let mut f: i32 = atdir_set(name, b.offset_from(name) as i64);
        if 0 as i32 <= f {
            base = b;
            atfd = f;
        }
    }
    fd = openat_safer(atfd, base, flags);
    if 0 as i32 <= fd && fstat(fd, st) != 0 as i32 {
        let mut e: i32 = *__errno_location();
        close(fd);
        *__errno_location() = e;
        return -(1 as i32);
    }
    return fd;
}
unsafe extern "C" fn open_input_file(mut iname: *mut i8, mut sbuf: *mut stat) -> i32 {
    let mut current_block: u64;
    let mut ilen: i32 = 0;
    let mut z_suffix_errno: i32 = 0 as i32;
    static mut suffixes: [*const i8; 6] = [
        0 as *const i8,
        b".gz\0" as *const u8 as *const i8,
        b".z\0" as *const u8 as *const i8,
        b"-z\0" as *const u8 as *const i8,
        b".Z\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut suf: *mut *const i8 = suffixes.as_mut_ptr();
    let mut s: *const i8 = 0 as *const i8;
    let mut fd: i32 = 0;
    let mut open_flags: i32 = 0 as i32 | 0o4000 as i32 | 0o400 as i32
        | (if ascii != 0 && decompress == 0 { 0 as i32 } else { 0 as i32 });
    *suf = z_suffix;
    if !((::core::mem::size_of::<[i8; 1024]>() as u64).wrapping_sub(1 as i32 as u64)
        <= strlen(iname))
    {
        strcpy(ifname.as_mut_ptr(), iname);
        fd = open_and_stat(ifname.as_mut_ptr(), open_flags, sbuf);
        if 0 as i32 <= fd {
            return fd;
        }
        if decompress == 0 || *__errno_location() != 2 as i32 {
            progerror(ifname.as_mut_ptr());
            return -(1 as i32);
        }
        s = get_suffix(ifname.as_mut_ptr());
        if !s.is_null() {
            progerror(ifname.as_mut_ptr());
            return -(1 as i32);
        }
        ilen = strlen(ifname.as_mut_ptr()) as i32;
        if strcmp(z_suffix, b".gz\0" as *const u8 as *const i8) == 0 as i32 {
            suf = suf.offset(1);
            suf;
        }
        loop {
            s = *suf;
            let mut s0: *const i8 = s;
            strcpy(ifname.as_mut_ptr(), iname);
            if ::core::mem::size_of::<[i8; 1024]>() as u64
                <= (ilen as u64).wrapping_add(strlen(s))
            {
                current_block = 14215106989711722809;
                break;
            }
            strcat(ifname.as_mut_ptr(), s);
            fd = open_and_stat(ifname.as_mut_ptr(), open_flags, sbuf);
            if 0 as i32 <= fd {
                return fd;
            }
            if *__errno_location() != 2 as i32 {
                progerror(ifname.as_mut_ptr());
                return -(1 as i32);
            }
            if strcmp(s0, z_suffix) == 0 as i32 {
                z_suffix_errno = *__errno_location();
            }
            suf = suf.offset(1);
            if (*suf).is_null() {
                current_block = 9828876828309294594;
                break;
            }
        }
        match current_block {
            14215106989711722809 => {}
            _ => {
                strcpy(ifname.as_mut_ptr(), iname);
                strcat(ifname.as_mut_ptr(), z_suffix);
                *__errno_location() = z_suffix_errno;
                progerror(ifname.as_mut_ptr());
                return -(1 as i32);
            }
        }
    }
    rpl_fprintf(
        stderr,
        b"%s: %s: file name too long\n\0" as *const u8 as *const i8,
        program_name,
        iname,
    );
    exit_code = 1 as i32;
    return -(1 as i32);
}
unsafe extern "C" fn make_ofname() -> i32 {
    let mut suff: *mut i8 = 0 as *mut i8;
    strcpy(ofname.as_mut_ptr(), ifname.as_mut_ptr());
    suff = get_suffix(ofname.as_mut_ptr());
    if decompress != 0 {
        if suff.is_null() {
            if recursive == 0 && test != 0 {
                return 0 as i32;
            }
            if verbose != 0 || recursive == 0 && quiet == 0 {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s: unknown suffix -- ignored\n\0" as *const u8
                            as *const i8,
                        program_name,
                        ifname.as_mut_ptr(),
                    );
                }
                if exit_code == 0 as i32 {
                    exit_code = 2 as i32;
                }
            }
            return 2 as i32;
        }
        strlwr(suff);
        if strcmp(suff, b".tgz\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(suff, b".taz\0" as *const u8 as *const i8) == 0 as i32
        {
            strcpy(suff, b".tar\0" as *const u8 as *const i8);
        } else {
            *suff = '\0' as i32 as i8;
        }
    } else if !suff.is_null() && force == 0 {
        if verbose != 0 || recursive == 0 && quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s already has %s suffix -- unchanged\n\0" as *const u8
                    as *const i8,
                program_name,
                ifname.as_mut_ptr(),
                suff,
            );
        }
        return 2 as i32;
    } else {
        save_orig_name = 0 as i32;
        if ::core::mem::size_of::<[i8; 1024]>() as u64
            <= (strlen(ofname.as_mut_ptr())).wrapping_add(z_len)
        {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s: file name too long\n\0" as *const u8 as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as i32 {
                exit_code = 2 as i32;
            }
            return 2 as i32;
        } else {
            strcat(ofname.as_mut_ptr(), z_suffix);
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn discard_input_bytes(mut nbytes: size_t, mut flags: u32) {
    while nbytes != 0 as i32 as u64 {
        let mut c: uch = (if inptr < insize {
            let fresh5 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh5 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        }) as uch;
        if flags & 0x2 as i32 as u32 != 0 {
            updcrc(&mut c, 1 as i32 as u32);
        }
        if nbytes != -(1 as i32) as size_t {
            nbytes = nbytes.wrapping_sub(1);
            nbytes;
        } else if c == 0 {
            break;
        }
    }
}
unsafe extern "C" fn get_method(mut in_0: i32) -> i32 {
    let mut flags: uch = 0;
    let mut magic: [uch; 10] = [0; 10];
    let mut imagic0: i32 = 0;
    let mut imagic1: i32 = 0;
    let mut stamp: ulg = 0;
    if force != 0 && to_stdout != 0 {
        imagic0 = if inptr < insize {
            let fresh6 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh6 as usize] as i32
        } else {
            fill_inbuf(1 as i32)
        };
        magic[0 as i32 as usize] = imagic0 as uch;
        imagic1 = if inptr < insize {
            let fresh7 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh7 as usize] as i32
        } else {
            fill_inbuf(1 as i32)
        };
        magic[1 as i32 as usize] = imagic1 as uch;
    } else {
        magic[0 as i32 as usize] = (if inptr < insize {
            let fresh8 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh8 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        }) as uch;
        imagic0 = 0 as i32;
        if magic[0 as i32 as usize] != 0 {
            magic[1 as i32 as usize] = (if inptr < insize {
                let fresh9 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh9 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as uch;
            imagic1 = 0 as i32;
        } else {
            imagic1 = if inptr < insize {
                let fresh10 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh10 as usize] as i32
            } else {
                fill_inbuf(1 as i32)
            };
            magic[1 as i32 as usize] = imagic1 as uch;
        }
    }
    method = -(1 as i32);
    part_nb += 1;
    part_nb;
    header_bytes = 0 as i32 as off_t;
    last_member = 0 as i32;
    if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\x8B\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    ) == 0 as i32
        || memcmp(
            magic.as_mut_ptr() as *const libc::c_void,
            b"\x1F\x9E\0" as *const u8 as *const i8 as *const libc::c_void,
            2 as i32 as u64,
        ) == 0 as i32
    {
        method = if inptr < insize {
            let fresh11 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh11 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        };
        if method != 8 as i32 {
            rpl_fprintf(
                stderr,
                b"%s: %s: unknown method %d -- not supported\n\0" as *const u8
                    as *const i8,
                program_name,
                ifname.as_mut_ptr(),
                method,
            );
            exit_code = 1 as i32;
            return -(1 as i32);
        }
        work = Some(unzip as unsafe extern "C" fn(i32, i32) -> i32);
        flags = (if inptr < insize {
            let fresh12 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh12 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        }) as uch;
        if flags as i32 & 0x20 as i32 != 0 as i32 {
            rpl_fprintf(
                stderr,
                b"%s: %s is encrypted -- not supported\n\0" as *const u8 as *const i8,
                program_name,
                ifname.as_mut_ptr(),
            );
            exit_code = 1 as i32;
            return -(1 as i32);
        }
        if flags as i32 & 0xc0 as i32 != 0 as i32 {
            rpl_fprintf(
                stderr,
                b"%s: %s has flags 0x%x -- not supported\n\0" as *const u8 as *const i8,
                program_name,
                ifname.as_mut_ptr(),
                flags as i32,
            );
            exit_code = 1 as i32;
            if force <= 1 as i32 {
                return -(1 as i32);
            }
        }
        stamp = (if inptr < insize {
            let fresh13 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh13 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        }) as ulg;
        stamp
            |= ((if inptr < insize {
                let fresh14 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh14 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as ulg) << 8 as i32;
        stamp
            |= ((if inptr < insize {
                let fresh15 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh15 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as ulg) << 16 as i32;
        stamp
            |= ((if inptr < insize {
                let fresh16 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh16 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as ulg) << 24 as i32;
        if stamp != 0 as i32 as u64 && no_time == 0 {
            if stamp
                <= (if (0 as i32 as time_t) < -(1 as i32) as time_t {
                    -(1 as i32) as time_t
                } else {
                    (((1 as i32 as time_t)
                        << (::core::mem::size_of::<time_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                }) as u64
            {
                time_stamp.tv_sec = stamp as __time_t;
                time_stamp.tv_nsec = 0 as i32 as __syscall_slong_t;
            } else {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s: MTIME %lu out of range for this platform\n\0"
                            as *const u8 as *const i8,
                        program_name,
                        ifname.as_mut_ptr(),
                        stamp,
                    );
                }
                if exit_code == 0 as i32 {
                    exit_code = 2 as i32;
                }
                time_stamp.tv_sec = if (0 as i32 as time_t) < -(1 as i32) as time_t {
                    -(1 as i32) as time_t
                } else {
                    (((1 as i32 as time_t)
                        << (::core::mem::size_of::<time_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                };
                time_stamp.tv_nsec = (C2RustUnnamed_10::TIMESPEC_RESOLUTION as i32
                    - 1 as i32) as __syscall_slong_t;
            }
        }
        magic[8 as i32 as usize] = (if inptr < insize {
            let fresh17 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh17 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        }) as uch;
        magic[9 as i32 as usize] = (if inptr < insize {
            let fresh18 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh18 as usize] as i32
        } else {
            fill_inbuf(0 as i32)
        }) as uch;
        if flags as i32 & 0x2 as i32 != 0 {
            magic[2 as i32 as usize] = 8 as i32 as uch;
            magic[3 as i32 as usize] = flags;
            magic[4 as i32 as usize] = (stamp & 0xff as i32 as u64) as uch;
            magic[5 as i32 as usize] = (stamp >> 8 as i32 & 0xff as i32 as u64) as uch;
            magic[6 as i32 as usize] = (stamp >> 16 as i32 & 0xff as i32 as u64) as uch;
            magic[7 as i32 as usize] = (stamp >> 24 as i32) as uch;
            updcrc(0 as *const uch, 0 as i32 as u32);
            updcrc(magic.as_mut_ptr(), 10 as i32 as u32);
        }
        if flags as i32 & 0x4 as i32 != 0 as i32 {
            let mut lenbuf: [uch; 2] = [0; 2];
            lenbuf[0 as i32 as usize] = (if inptr < insize {
                let fresh19 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh19 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as uch;
            let mut len: u32 = lenbuf[0 as i32 as usize] as u32;
            lenbuf[1 as i32 as usize] = (if inptr < insize {
                let fresh20 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh20 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as uch;
            len |= ((lenbuf[1 as i32 as usize] as i32) << 8 as i32) as u32;
            if verbose != 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s: extra field of %u bytes ignored\n\0" as *const u8
                        as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                    len,
                );
            }
            if flags as i32 & 0x2 as i32 != 0 {
                updcrc(lenbuf.as_mut_ptr(), 2 as i32 as u32);
            }
            discard_input_bytes(len as size_t, flags as u32);
        }
        if flags as i32 & 0x8 as i32 != 0 as i32 {
            if no_name != 0 || to_stdout != 0 && list == 0 || part_nb > 1 as i32 {
                discard_input_bytes(-(1 as i32) as size_t, flags as u32);
            } else {
                let mut p: *mut i8 = gzip_base_name(ofname.as_mut_ptr());
                let mut base: *mut i8 = p;
                loop {
                    *p = (if inptr < insize {
                        let fresh21 = inptr;
                        inptr = inptr.wrapping_add(1);
                        inbuf[fresh21 as usize] as i32
                    } else {
                        fill_inbuf(0 as i32)
                    }) as i8;
                    let fresh22 = p;
                    p = p.offset(1);
                    if *fresh22 as i32 == '\0' as i32 {
                        break;
                    }
                    if p
                        >= ofname
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<[i8; 1024]>() as u64 as isize)
                    {
                        gzip_error(
                            b"corrupted input -- file name too large\0" as *const u8
                                as *const i8,
                        );
                    }
                }
                if flags as i32 & 0x2 as i32 != 0 {
                    updcrc(base as *mut uch, p.offset_from(base) as i64 as u32);
                }
                p = gzip_base_name(base);
                memmove(
                    base as *mut libc::c_void,
                    p as *const libc::c_void,
                    (strlen(p)).wrapping_add(1 as i32 as u64),
                );
                if list == 0 {
                    if !base.is_null() {
                        list = 0 as i32;
                    }
                }
            }
        }
        if flags as i32 & 0x10 as i32 != 0 as i32 {
            discard_input_bytes(-(1 as i32) as size_t, flags as u32);
        }
        if flags as i32 & 0x2 as i32 != 0 {
            let mut crc16: u32 = (updcrc(magic.as_mut_ptr(), 0 as i32 as u32)
                & 0xffff as i32 as u64) as u32;
            let mut header16: u32 = (if inptr < insize {
                let fresh23 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh23 as usize] as i32
            } else {
                fill_inbuf(0 as i32)
            }) as u32;
            header16
                |= ((if inptr < insize {
                    let fresh24 = inptr;
                    inptr = inptr.wrapping_add(1);
                    inbuf[fresh24 as usize] as i32
                } else {
                    fill_inbuf(0 as i32)
                }) as u32) << 8 as i32;
            if header16 != crc16 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n\0"
                        as *const u8 as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                    header16,
                    crc16,
                );
                exit_code = 1 as i32;
                if force <= 1 as i32 {
                    return -(1 as i32);
                }
            }
        }
        if part_nb == 1 as i32 {
            header_bytes = inptr.wrapping_add((2 as i32 * 4 as i32) as u32) as off_t;
        }
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"PK\x03\x04\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    ) == 0 as i32 && inptr == 2 as i32 as u32
        && memcmp(
            inbuf.as_mut_ptr() as *mut i8 as *const libc::c_void,
            b"PK\x03\x04\0" as *const u8 as *const i8 as *const libc::c_void,
            4 as i32 as u64,
        ) == 0 as i32
    {
        inptr = 0 as i32 as u32;
        work = Some(unzip as unsafe extern "C" fn(i32, i32) -> i32);
        if check_zipfile(in_0) != 0 as i32 {
            return -(1 as i32);
        }
        last_member = 1 as i32;
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\x1E\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    ) == 0 as i32
    {
        work = Some(unpack as unsafe extern "C" fn(i32, i32) -> i32);
        method = 2 as i32;
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\x9D\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    ) == 0 as i32
    {
        work = Some(unlzw as unsafe extern "C" fn(i32, i32) -> i32);
        method = 1 as i32;
        last_member = 1 as i32;
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\xA0\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    ) == 0 as i32
    {
        work = Some(unlzh as unsafe extern "C" fn(i32, i32) -> i32);
        method = 3 as i32;
        last_member = 1 as i32;
    } else if force != 0 && to_stdout != 0 && list == 0 {
        method = 0 as i32;
        work = Some(copy as unsafe extern "C" fn(i32, i32) -> i32);
        if imagic1 != -(1 as i32) {
            inptr = inptr.wrapping_sub(1);
            inptr;
        }
        last_member = 1 as i32;
        if imagic0 != -(1 as i32) {
            write_buf(1 as i32, magic.as_mut_ptr() as voidp, 1 as i32 as u32);
        }
    }
    if method >= 0 as i32 {
        return method;
    }
    if part_nb == 1 as i32 {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: not in gzip format\n\0" as *const u8 as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as i32;
        return -(1 as i32);
    } else {
        if magic[0 as i32 as usize] as i32 == 0 as i32 {
            let mut inbyte: i32 = 0;
            inbyte = imagic1;
            while inbyte == 0 as i32 {
                inbyte = if inptr < insize {
                    let fresh25 = inptr;
                    inptr = inptr.wrapping_add(1);
                    inbuf[fresh25 as usize] as i32
                } else {
                    fill_inbuf(1 as i32)
                };
            }
            if inbyte == -(1 as i32) {
                if verbose != 0 {
                    if quiet == 0 {
                        rpl_fprintf(
                            stderr,
                            b"\n%s: %s: decompression OK, trailing zero bytes ignored\n\0"
                                as *const u8 as *const i8,
                            program_name,
                            ifname.as_mut_ptr(),
                        );
                    }
                    if exit_code == 0 as i32 {
                        exit_code = 2 as i32;
                    }
                }
                return -(3 as i32);
            }
        }
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"\n%s: %s: decompression OK, trailing garbage ignored\n\0" as *const u8
                    as *const i8,
                program_name,
                ifname.as_mut_ptr(),
            );
        }
        if exit_code == 0 as i32 {
            exit_code = 2 as i32;
        }
        return -(2 as i32);
    };
}
unsafe extern "C" fn do_list(mut method_0: i32) {
    let mut crc: ulg = 0;
    static mut first_time: i32 = 1 as i32;
    static mut methods: [*const i8; 9] = [
        b"store\0" as *const u8 as *const i8,
        b"compr\0" as *const u8 as *const i8,
        b"pack \0" as *const u8 as *const i8,
        b"lzh  \0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"\0" as *const u8 as *const i8,
        b"defla\0" as *const u8 as *const i8,
    ];
    let mut positive_off_t_width: i32 = (::core::mem::size_of::<off_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(!((0 as i32 as off_t) < -(1 as i32) as off_t) as i32 as u64)
        .wrapping_mul(146 as i32 as u64)
        .wrapping_add(484 as i32 as u64)
        .wrapping_div(485 as i32 as u64)
        .wrapping_add(!((0 as i32 as off_t) < -(1 as i32) as off_t) as i32 as u64)
        .wrapping_sub(1 as i32 as u64) as i32;
    if first_time != 0 && method_0 >= 0 as i32 {
        first_time = 0 as i32;
        if verbose != 0 {
            __printf__(b"method  crc     date  time  \0" as *const u8 as *const i8);
        }
        if quiet == 0 {
            __printf__(
                b"%*.*s %*.*s  ratio uncompressed_name\n\0" as *const u8 as *const i8,
                positive_off_t_width,
                positive_off_t_width,
                b"compressed\0" as *const u8 as *const i8,
                positive_off_t_width,
                positive_off_t_width,
                b"uncompressed\0" as *const u8 as *const i8,
            );
        }
    } else if method_0 < 0 as i32 {
        if total_in <= 0 as i32 as i64 || total_out <= 0 as i32 as i64 {
            return;
        }
        if verbose != 0 {
            __printf__(b"                            \0" as *const u8 as *const i8);
        }
        if verbose != 0 || quiet == 0 {
            fprint_off(stdout, total_in, positive_off_t_width);
            __printf__(b" \0" as *const u8 as *const i8);
            fprint_off(stdout, total_out, positive_off_t_width);
            __printf__(b" \0" as *const u8 as *const i8);
        }
        display_ratio(total_out - (total_in - header_bytes), total_out, stdout);
        __printf__(b" (totals)\n\0" as *const u8 as *const i8);
        return;
    }
    crc = !(0 as i32) as ulg;
    if method_0 == 8 as i32 && last_member == 0 {
        crc = unzip_crc;
    }
    if verbose != 0 {
        static mut month_abbr: [[i8; 4]; 12] = unsafe {
            [
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Jan\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Feb\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Mar\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Apr\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"May\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Jun\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Jul\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Aug\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Sep\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Oct\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Nov\0"),
                *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"Dec\0"),
            ]
        };
        let mut tm: *mut tm = localtime(&mut time_stamp.tv_sec);
        __printf__(
            b"%5s %08lx \0" as *const u8 as *const i8,
            methods[method_0 as usize],
            crc,
        );
        if !tm.is_null() {
            __printf__(
                b"%s%3d %02d:%02d \0" as *const u8 as *const i8,
                (month_abbr[(*tm).tm_mon as usize]).as_ptr(),
                (*tm).tm_mday,
                (*tm).tm_hour,
                (*tm).tm_min,
            );
        } else {
            __printf__(b"??? ?? ??:?? \0" as *const u8 as *const i8);
        }
    }
    fprint_off(stdout, bytes_in, positive_off_t_width);
    __printf__(b" \0" as *const u8 as *const i8);
    fprint_off(stdout, bytes_out, positive_off_t_width);
    __printf__(b" \0" as *const u8 as *const i8);
    if bytes_in == -(1 as i64) {
        total_in = -(1 as i64);
        header_bytes = 0 as i32 as off_t;
        bytes_out = header_bytes;
        bytes_in = bytes_out;
    } else if total_in >= 0 as i32 as i64 {
        total_in += bytes_in;
    }
    if bytes_out == -(1 as i64) {
        total_out = -(1 as i64);
        header_bytes = 0 as i32 as off_t;
        bytes_out = header_bytes;
        bytes_in = bytes_out;
    } else if total_out >= 0 as i32 as i64 {
        total_out += bytes_out;
    }
    display_ratio(bytes_out - (bytes_in - header_bytes), bytes_out, stdout);
    __printf__(b" %s\n\0" as *const u8 as *const i8, ofname.as_mut_ptr());
}
unsafe extern "C" fn shorten_name(mut name: *mut i8) {
    let mut len: i32 = 0;
    let mut trunc: *mut i8 = 0 as *mut i8;
    let mut plen: i32 = 0;
    let mut min_part: i32 = 3 as i32;
    let mut p: *mut i8 = 0 as *mut i8;
    len = strlen(name) as i32;
    if decompress != 0 {
        if len <= 1 as i32 {
            gzip_error(b"name too short\0" as *const u8 as *const i8);
        }
        *name.offset((len - 1 as i32) as isize) = '\0' as i32 as i8;
        return;
    }
    p = get_suffix(name);
    if p.is_null() {
        gzip_error(b"can't recover suffix\n\0" as *const u8 as *const i8);
    }
    *p = '\0' as i32 as i8;
    save_orig_name = 1 as i32;
    if len > 4 as i32
        && strcmp(p.offset(-(4 as i32 as isize)), b".tar\0" as *const u8 as *const i8)
            == 0 as i32
    {
        strcpy(p.offset(-(4 as i32 as isize)), b".tgz\0" as *const u8 as *const i8);
        return;
    }
    loop {
        p = last_component(name);
        while *p != 0 {
            plen = strcspn(p, b".\0" as *const u8 as *const i8) as i32;
            p = p.offset(plen as isize);
            if plen > min_part {
                trunc = p.offset(-(1 as i32 as isize));
            }
            if *p != 0 {
                p = p.offset(1);
                p;
            }
        }
        if !(trunc.is_null()
            && {
                min_part -= 1;
                min_part != 0 as i32
            })
        {
            break;
        }
    }
    if !trunc.is_null() {
        loop {
            *trunc.offset(0 as i32 as isize) = *trunc.offset(1 as i32 as isize);
            let fresh26 = trunc;
            trunc = trunc.offset(1);
            if !(*fresh26 != 0) {
                break;
            }
        }
        trunc = trunc.offset(-1);
        trunc;
    } else {
        trunc = strrchr(
            name,
            (*::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b".\0"))[0 as i32 as usize]
                as i32,
        );
        if trunc.is_null() {
            gzip_error(b"internal error in shorten_name\0" as *const u8 as *const i8);
        }
        if *trunc.offset(1 as i32 as isize) as i32 == '\0' as i32 {
            trunc = trunc.offset(-1);
            trunc;
        }
    }
    strcpy(trunc, z_suffix);
}
unsafe extern "C" fn check_ofname() -> i32 {
    if force == 0 {
        let mut ok: i32 = 0 as i32;
        rpl_fprintf(
            stderr,
            b"%s: %s already exists;\0" as *const u8 as *const i8,
            program_name,
            ofname.as_mut_ptr(),
        );
        if foreground != 0 && (presume_input_tty as i32 != 0 || isatty(0 as i32) != 0) {
            rpl_fprintf(
                stderr,
                b" do you wish to overwrite (y or n)? \0" as *const u8 as *const i8,
            );
            rpl_fflush(stderr);
            ok = yesno() as i32;
        }
        if ok == 0 {
            rpl_fprintf(stderr, b"\tnot overwritten\n\0" as *const u8 as *const i8);
            if exit_code == 0 as i32 {
                exit_code = 2 as i32;
            }
            return 1 as i32;
        }
    }
    if xunlink(ofname.as_mut_ptr()) != 0 {
        progerror(ofname.as_mut_ptr());
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn do_chown(
    mut fd: i32,
    mut name: *const i8,
    mut uid: uid_t,
    mut gid: gid_t,
) {
    fchown(fd, uid, gid);
}
unsafe extern "C" fn copy_stat(mut ifstat: *mut stat) {
    let mut mode: mode_t = (*ifstat).st_mode
        & (0o400 as i32 | 0o200 as i32 | 0o100 as i32
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32 >> 3 as i32)
            as u32;
    let mut r: i32 = 0;
    let mut restoring: bool = false;
    let mut timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    timespec[0 as i32 as usize] = get_stat_atime(ifstat);
    timespec[1 as i32 as usize] = get_stat_mtime(ifstat);
    restoring = decompress != 0 && 0 as i32 as i64 <= time_stamp.tv_nsec
        && !(timespec[1 as i32 as usize].tv_sec == time_stamp.tv_sec
            && timespec[1 as i32 as usize].tv_nsec == time_stamp.tv_nsec);
    if restoring {
        timespec[1 as i32 as usize] = time_stamp;
    }
    if fdutimens(ofd, ofname.as_mut_ptr(), timespec.as_mut_ptr() as *const timespec)
        == 0 as i32
    {
        if restoring as i32 != 0 && (1 as i32) < verbose {
            rpl_fprintf(
                stderr,
                b"%s: timestamp restored\n\0" as *const u8 as *const i8,
                ofname.as_mut_ptr(),
            );
        }
    } else {
        let mut e: i32 = *__errno_location();
        if quiet == 0 {
            rpl_fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program_name);
        }
        if exit_code == 0 as i32 {
            exit_code = 2 as i32;
        }
        if quiet == 0 {
            *__errno_location() = e;
            perror(ofname.as_mut_ptr());
        }
    }
    do_chown(ofd, ofname.as_mut_ptr(), -(1 as i32) as uid_t, (*ifstat).st_gid);
    r = fchmod(ofd, mode);
    if r != 0 as i32 {
        let mut e_0: i32 = *__errno_location();
        if quiet == 0 {
            rpl_fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program_name);
        }
        if exit_code == 0 as i32 {
            exit_code = 2 as i32;
        }
        if quiet == 0 {
            *__errno_location() = e_0;
            perror(ofname.as_mut_ptr());
        }
    }
    do_chown(ofd, ofname.as_mut_ptr(), (*ifstat).st_uid, -(1 as i32) as gid_t);
}
unsafe extern "C" fn treat_dir(mut fd: i32, mut dir: *mut i8) {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut nbuf: [i8; 1024] = [0; 1024];
    let mut entries: *mut i8 = 0 as *mut i8;
    let mut entry: *const i8 = 0 as *const i8;
    let mut entrylen: size_t = 0;
    dirp = fdopendir(fd);
    if dirp.is_null() {
        progerror(dir);
        close(fd);
        return;
    }
    entries = streamsavedir(dirp, SAVEDIR_SORT_NONE);
    if entries.is_null() {
        progerror(dir);
    }
    if closedir(dirp) != 0 as i32 {
        progerror(dir);
    }
    if entries.is_null() {
        return;
    }
    entry = entries;
    while *entry != 0 {
        let mut len: size_t = strlen(dir);
        entrylen = strlen(entry);
        if !(strcmp(entry, b".\0" as *const u8 as *const i8) == 0 as i32
            || strcmp(entry, b"..\0" as *const u8 as *const i8) == 0 as i32)
        {
            if len.wrapping_add(entrylen) < (1024 as i32 - 2 as i32) as u64 {
                strcpy(nbuf.as_mut_ptr(), dir);
                if *last_component(nbuf.as_mut_ptr()) as i32 != 0
                    && !(nbuf[len.wrapping_sub(1 as i32 as u64) as usize] as i32
                        == '/' as i32)
                {
                    let fresh27 = len;
                    len = len.wrapping_add(1);
                    nbuf[fresh27 as usize] = '/' as i32 as i8;
                }
                strcpy(nbuf.as_mut_ptr().offset(len as isize), entry);
                treat_file(nbuf.as_mut_ptr());
            } else {
                rpl_fprintf(
                    stderr,
                    b"%s: %s/%s: pathname too long\n\0" as *const u8 as *const i8,
                    program_name,
                    dir,
                    entry,
                );
                exit_code = 1 as i32;
            }
        }
        entry = entry.offset(entrylen.wrapping_add(1 as i32 as u64) as isize);
    }
    rpl_free(entries as *mut libc::c_void);
}
unsafe extern "C" fn install_signal_handlers() {
    let mut nsigs: i32 = (::core::mem::size_of::<[i32; 6]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    let mut i: i32 = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut caught_signals);
    i = 0 as i32;
    while i < nsigs {
        sigaction(handled_sig[i as usize], 0 as *const sigaction, &mut act);
        if act.__sigaction_handler.sa_handler
            != ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as i32 as libc::intptr_t)
        {
            sigaddset(&mut caught_signals, handled_sig[i as usize]);
        }
        i += 1;
        i;
    }
    act.__sigaction_handler.sa_handler = Some(
        abort_gzip_signal as unsafe extern "C" fn(i32) -> (),
    );
    act.sa_mask = caught_signals;
    act.sa_flags = 0 as i32;
    i = 0 as i32;
    while i < nsigs {
        if sigismember(&mut caught_signals, handled_sig[i as usize]) != 0 {
            if i == 0 as i32 {
                foreground = 1 as i32;
            }
            sigaction(handled_sig[i as usize], &mut act, 0 as *mut sigaction);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn do_exit(mut exitcode: i32) {
    static mut in_exit: i32 = 0 as i32;
    if in_exit != 0 {
        exit(exitcode);
    }
    in_exit = 1 as i32;
    rpl_free(env as *mut libc::c_void);
    env = 0 as *mut i8;
    exit(exitcode);
}
unsafe extern "C" fn finish_out() {
    if rpl_fclose(stdout) != 0 as i32 {
        write_error();
    }
    do_exit(0 as i32);
}
unsafe extern "C" fn remove_output_file(mut signals_already_blocked: bool) {
    let mut fd: i32 = 0;
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    if !signals_already_blocked {
        sigprocmask(0 as i32, &mut caught_signals, &mut oldset);
    }
    fd = remove_ofname_fd;
    if 0 as i32 <= fd {
        let mut fname: [i8; 1024] = [0; 1024];
        ::core::ptr::write_volatile(&mut remove_ofname_fd as *mut i32, -(1 as i32));
        close(fd);
        volatile_strcpy(fname.as_mut_ptr() as *mut i8, remove_ofname.as_mut_ptr());
        xunlink(fname.as_mut_ptr());
    }
    if !signals_already_blocked {
        sigprocmask(2 as i32, &mut oldset, 0 as *mut sigset_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn abort_gzip() {
    remove_output_file(0 as i32 != 0);
    do_exit(1 as i32);
}
unsafe extern "C" fn abort_gzip_signal(mut sig: i32) {
    remove_output_file(1 as i32 != 0);
    if sig == exiting_signal {
        _exit(2 as i32);
    }
    signal(sig, None);
    raise(sig);
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}
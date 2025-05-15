use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn fchmod(__fd: i32, __mode: __mode_t) -> i32;
    static mut top: *mut top;
    fn thank_you_and_goodnight(how: i32);
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
    fn dup_safer(_: i32) -> i32;
    fn fatal_sys(who: *const i8);
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
pub type size_t = u64;
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
pub type off_t = __off_t;
pub type mode_t = __mode_t;
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
pub struct cbuf {
    pub string: *const i8,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const i8,
    pub date: *const i8,
    pub author: *const i8,
    pub lockedby: *const i8,
    pub state: *const i8,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const i8,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const i8,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const i8,
    pub name: *const i8,
    pub desc: *const i8,
    pub help: *const i8,
    pub tyag: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum maker {
    effective = 2,
    real = 1,
    notmade = 0,
}
impl maker {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            maker::effective => 2,
            maker::real => 1,
            maker::notmade => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> maker {
        match value {
            2 => maker::effective,
            1 => maker::real,
            0 => maker::notmade,
            _ => panic!("Invalid value for maker: {}", value),
        }
    }
}
impl AddAssign<u32> for maker {
    fn add_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for maker {
    fn sub_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for maker {
    fn mul_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for maker {
    fn div_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for maker {
    fn rem_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for maker {
    type Output = maker;
    fn add(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for maker {
    type Output = maker;
    fn sub(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for maker {
    type Output = maker;
    fn mul(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for maker {
    type Output = maker;
    fn div(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for maker {
    type Output = maker;
    fn rem(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const i8,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const i8,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: i32,
    pub stick_with_euid: bool,
    pub ruid: i32,
    pub euid: i32,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: i32,
    pub pe: *const i8,
    pub zone_offset: zone_offset,
    pub username: *mut i8,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut i8,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut i8,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub valid: bool,
    pub author: *mut i8,
    pub date: *mut i8,
    pub name: *mut i8,
    pub rev: *mut i8,
    pub state: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const i8,
    pub branch: *const i8,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: i32,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const i8,
    pub fd_lock: i32,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const i8,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
#[no_mangle]
pub unsafe extern "C" fn change_mode(mut fd: i32, mut mode: mode_t) -> i32 {
    return fchmod(fd, mode);
}
#[no_mangle]
pub unsafe extern "C" fn Ierror() {
    fatal_sys(b"input error\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn testIerror(mut f: *mut FILE) {
    if ferror(f) != 0 {
        Ierror();
    }
}
#[no_mangle]
pub unsafe extern "C" fn Oerror() {
    if (*top).behavior.Oerrloop {
        thank_you_and_goodnight((*(*top).program).tyag);
    }
    (*top).behavior.Oerrloop = 1 as i32 != 0;
    fatal_sys(b"output error\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn testOerror(mut o: *mut FILE) {
    if ferror(o) != 0 {
        Oerror();
    }
}
#[no_mangle]
pub unsafe extern "C" fn fopen_safer(
    mut filename: *const i8,
    mut type_0: *const i8,
) -> *mut FILE {
    let mut stream: *mut FILE = fopen(filename, type_0);
    if !stream.is_null() {
        let mut fd: i32 = fileno(stream);
        if 0 as i32 <= fd && fd <= 2 as i32 {
            let mut f: i32 = dup_safer(fd);
            if 0 as i32 > f {
                let mut e: i32 = *__errno_location();
                fclose(stream);
                *__errno_location() = e;
                return 0 as *mut FILE;
            }
            if 0 as i32 > fclose(stream) {
                let mut e_0: i32 = *__errno_location();
                close(f);
                *__errno_location() = e_0;
                return 0 as *mut FILE;
            }
            stream = fdopen(f, type_0);
        }
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn Ozclose(mut p: *mut *mut FILE) {
    if !(*p).is_null() && -(1 as i32) == fclose(*p) {
        Oerror();
    }
    *p = 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn aflush(mut f: *mut FILE) {
    if 0 as i32 > fflush(f) {
        Oerror();
    }
}
#[no_mangle]
pub unsafe extern "C" fn oflush() {
    let mut mstdout: *mut FILE = (*top).manifestation.standard_output;
    if 0 as i32 > fflush((if !mstdout.is_null() { mstdout } else { stdout }))
        && !(*top).behavior.Oerrloop
    {
        Oerror();
    }
}
#[no_mangle]
pub unsafe extern "C" fn afputc(mut c: i32, mut f: *mut FILE) {
    if _IO_putc(c, f) == -(1 as i32) {
        testOerror(f);
    }
}
#[no_mangle]
pub unsafe extern "C" fn newline(mut f: *mut FILE) {
    if _IO_putc('\n' as i32, f) == -(1 as i32) {
        testOerror(f);
    }
}
#[no_mangle]
pub unsafe extern "C" fn aputs(mut s: *const i8, mut iop: *mut FILE) {
    if 0 as i32 > fputs(s, iop) {
        Oerror();
    }
}
#[no_mangle]
pub unsafe extern "C" fn aprintf(mut iop: *mut FILE, mut fmt: *const i8, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if 0 as i32 > vfprintf(iop, fmt, ap.as_va_list()) {
        Oerror();
    }
}
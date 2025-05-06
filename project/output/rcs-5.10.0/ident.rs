#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
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
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    static ctab: [tokens; 0];
    fn thank_you_and_goodnight(how: i32);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn __errno_location() -> *mut i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn display_version(prog: *const program, flags: i32);
    fn complain(fmt: *const i8, _: ...);
    fn syserror(e: i32, who: *const i8);
    static mut exit_failure: i32;
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
pub type off_t = __off_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum tokens {
    DELIM,
    DIGIT,
    IDCHAR,
    NEWLN,
    LETTER,
    Letter,
    PERIOD,
    SBEGIN,
    SPACE,
    UNKN,
    COLON,
    ID,
    NUM,
    SEMI,
    STRING,
}
impl tokens {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            tokens::DELIM => 0,
            tokens::DIGIT => 1,
            tokens::IDCHAR => 2,
            tokens::NEWLN => 3,
            tokens::LETTER => 4,
            tokens::Letter => 5,
            tokens::PERIOD => 6,
            tokens::SBEGIN => 7,
            tokens::SPACE => 8,
            tokens::UNKN => 9,
            tokens::COLON => 10,
            tokens::ID => 11,
            tokens::NUM => 12,
            tokens::SEMI => 13,
            tokens::STRING => 14,
        }
    }
    fn from_libc_c_uint(value: u32) -> tokens {
        match value {
            0 => tokens::DELIM,
            1 => tokens::DIGIT,
            2 => tokens::IDCHAR,
            3 => tokens::NEWLN,
            4 => tokens::LETTER,
            5 => tokens::Letter,
            6 => tokens::PERIOD,
            7 => tokens::SBEGIN,
            8 => tokens::SPACE,
            9 => tokens::UNKN,
            10 => tokens::COLON,
            11 => tokens::ID,
            12 => tokens::NUM,
            13 => tokens::SEMI,
            14 => tokens::STRING,
            _ => panic!("Invalid value for tokens: {}", value),
        }
    }
}
impl AddAssign<u32> for tokens {
    fn add_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for tokens {
    fn sub_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for tokens {
    fn mul_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for tokens {
    fn div_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for tokens {
    fn rem_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for tokens {
    type Output = tokens;
    fn add(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for tokens {
    type Output = tokens;
    fn sub(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for tokens {
    type Output = tokens;
    fn mul(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for tokens {
    type Output = tokens;
    fn div(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for tokens {
    type Output = tokens;
    fn rem(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
    notmade,
    real,
    effective,
}
impl maker {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            maker::notmade => 0,
            maker::real => 1,
            maker::effective => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> maker {
        match value {
            0 => maker::notmade,
            1 => maker::real,
            2 => maker::effective,
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
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
static mut ident_help: [i8; 176] = unsafe {
    *::core::mem::transmute::<
        &[u8; 176],
        &[i8; 176],
    >(
        b"[options] [file ...]\nOptions:\n  -q            Suppress warnings if no patterns are found.\n  -V            Obsolete; do not use.\n\nIf no FILE is specified, scan standard input.\n\0",
    )
};
static mut ident_blurb: [i8; 39] = unsafe {
    *::core::mem::transmute::<
        &[u8; 39],
        &[i8; 39],
    >(b"Identify RCS keyword strings in files.\0")
};
#[no_mangle]
pub static mut top: *mut top = 0 as *const top as *mut top;
unsafe extern "C" fn match_0(mut fp: *mut FILE) -> i32 {
    let mut line: [i8; 8192] = [0; 8192];
    let mut c: i32 = 0;
    let mut tp: *mut i8 = 0 as *mut i8;
    let mut subversion_style: bool = 0 as i32 != 0;
    tp = line.as_mut_ptr();
    loop {
        c = _IO_getc(fp);
        if !(c != ':' as i32) {
            break;
        }
        if c == -(1 as i32) && feof(fp) | ferror(fp) != 0 {
            return c;
        }
        's_43: {
            match *ctab.as_ptr().offset(c as isize) as u32 {
                4 | 5 => {
                    let fresh0 = tp;
                    tp = tp.offset(1);
                    *fresh0 = c as i8;
                    if tp
                        < line
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<[i8; 8192]>() as u64 as isize)
                            .offset(-(4 as i32 as isize))
                    {
                        break 's_43;
                    }
                }
                _ => {}
            }
            return if c != 0 { c } else { '\n' as i32 };
        }
    }
    if tp == line.as_mut_ptr() {
        return c;
    }
    let fresh1 = tp;
    tp = tp.offset(1);
    *fresh1 = c as i8;
    c = _IO_getc(fp);
    if ':' as i32 == c {
        subversion_style = 1 as i32 != 0;
        let fresh2 = tp;
        tp = tp.offset(1);
        *fresh2 = c as i8;
        c = _IO_getc(fp);
    }
    if c != ' ' as i32 {
        return if c != 0 { c } else { '\n' as i32 };
    }
    let fresh3 = tp;
    tp = tp.offset(1);
    *fresh3 = c as i8;
    loop {
        c = _IO_getc(fp);
        if !(c != '$' as i32) {
            break;
        }
        if c == -(1 as i32) && feof(fp) | ferror(fp) != 0 {
            return c;
        }
        's_119: {
            match *ctab.as_ptr().offset(c as isize) as u32 {
                3 | 9 => {}
                _ => {
                    let fresh4 = tp;
                    tp = tp.offset(1);
                    *fresh4 = c as i8;
                    if tp
                        < line
                            .as_mut_ptr()
                            .offset(::core::mem::size_of::<[i8; 8192]>() as u64 as isize)
                            .offset(-(2 as i32 as isize))
                    {
                        break 's_119;
                    }
                }
            }
            return if c != 0 { c } else { '\n' as i32 };
        }
    }
    if !(' ' as i32 == *tp.offset(-(1 as i32) as isize) as i32
        || subversion_style as i32 != 0
            && '#' as i32 == *tp.offset(-(1 as i32) as isize) as i32)
    {
        return c;
    }
    let fresh5 = tp;
    tp = tp.offset(1);
    *fresh5 = c as i8;
    *tp = '\0' as i32 as i8;
    printf(b"     %c%s\n\0" as *const u8 as *const i8, '$' as i32, line.as_mut_ptr());
    return 0 as i32;
}
unsafe extern "C" fn scanfile(mut file: *mut FILE, mut name: *const i8) -> i32 {
    let mut c: i32 = 0;
    if !name.is_null() {
        printf(b"%s:\n\0" as *const u8 as *const i8, name);
        if ferror(stdout) != 0 {
            return -(1 as i32);
        }
    } else {
        name = b"standard input\0" as *const u8 as *const i8;
    }
    c = 0 as i32;
    while c != -(1 as i32) || feof(file) | ferror(file) == 0 {
        if c == '$' as i32 {
            c = match_0(file);
            if c != 0 {
                continue;
            }
            if ferror(stdout) != 0 {
                return -(1 as i32);
            }
            (*top).behavior.quiet = 1 as i32 != 0;
        }
        c = _IO_getc(file);
    }
    if ferror(file) != 0 || 0 as i32 > fclose(file) {
        syserror(*__errno_location(), name);
        fflush(stdout);
        thank_you_and_goodnight((*(*top).program).tyag);
    }
    if !(*top).behavior.quiet {
        complain(
            b"%s warning: no id keywords in %s\n\0" as *const u8 as *const i8,
            (*(*top).program).name,
            name,
        );
    }
    return 0 as i32;
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: ident_blurb.as_ptr(),
            help: ident_help.as_ptr(),
            tyag: 0 as i32,
        };
        init
    }
};
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut status: i32 = 0 as i32;
    let mut a: *const i8 = 0 as *const i8;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = b"ident\0" as *const u8 as *const i8;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    loop {
        argv = argv.offset(1);
        a = *argv;
        if !(!a.is_null() && *a as i32 == '-' as i32) {
            break;
        }
        loop {
            a = a.offset(1);
            if !(*a != 0) {
                break;
            }
            's_77: {
                match *a as i32 {
                    113 => {
                        (*top).behavior.quiet = 1 as i32 != 0;
                        break 's_77;
                    }
                    86 => {
                        if *a.offset(1 as i32 as isize) == 0 {
                            display_version(&mut program, 1 as i32);
                            gnurcs_goodbye();
                            return 0 as i32;
                        }
                    }
                    _ => {}
                }
                bad_option(a.offset(-(1 as i32 as isize)));
                gnurcs_goodbye();
                return exit_failure;
            }
        }
    }
    if a.is_null() {
        scanfile(stdin, 0 as *const i8);
    } else {
        loop {
            fp = fopen(a, b"r\0" as *const u8 as *const i8);
            if fp.is_null() {
                syserror(*__errno_location(), a);
                status = exit_failure;
            } else if 0 as i32 > scanfile(fp, a)
                || !(*argv.offset(1 as i32 as isize)).is_null()
                    && putchar('\n' as i32) == -(1 as i32)
            {
                break;
            }
            argv = argv.offset(1);
            a = *argv;
            if a.is_null() {
                break;
            }
        }
    }
    if ferror(stdout) != 0 || 0 as i32 > fclose(stdout) {
        syserror(*__errno_location(), b"standard output\0" as *const u8 as *const i8);
        status = exit_failure;
    }
    gnurcs_goodbye();
    return status;
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
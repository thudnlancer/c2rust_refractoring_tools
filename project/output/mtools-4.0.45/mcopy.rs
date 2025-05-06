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
    pub type dos_name_t;
    pub type doscp_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn access(__name: *const i8, __type: i32) -> i32;
    fn unlink(__name: *const i8) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn mkdir(__path: *const i8, __mode: __mode_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncat(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn mk_entry(
        filename: *const dos_name_t,
        attr: u8,
        fat: u32,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn copyfile(Source: *mut Stream_t, Target: *mut Stream_t) -> mt_off_t;
    fn open_dos2unix(Next: *mut Stream_t, convertCharset: i32) -> *mut Stream_t;
    fn open_unix2dos(Next: *mut Stream_t, convertCharset: i32) -> *mut Stream_t;
    static mut got_signal: i32;
    static mut batchmode: i32;
    fn set_cmd_line_image(img: *mut i8);
    static mut progname: *const i8;
    fn getTimeNow(now: *mut time_t) -> time_t;
    fn printOom();
    fn getfreeMinBytes(Stream: *mut Stream_t, ref_0: mt_off_t) -> i32;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    fn ask_confirmation(_: *const i8, _: ...) -> i32;
    fn fileTooBig(off: mt_off_t) -> i32;
    fn mpPrintFilename(file: *mut FILE, mp: *mut MainParam_t);
    fn dos_target_lookup(mp: *mut MainParam_t, arg: *const i8) -> i32;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(MainParam: *mut MainParam_t, argv: *mut *mut i8, argc: i32) -> i32;
    fn mpGetBasename(mp: *mut MainParam_t) -> *const i8;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn vfat_lookup_zt(
        entry: *mut direntry_t,
        filename: *const i8,
        flags: i32,
        shortname: *mut i8,
        shortname_len: size_t,
        longname: *mut i8,
        longname_len: size_t,
    ) -> i32;
    fn mpPickTargetName(mp: *mut MainParam_t) -> *const i8;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: i32);
    fn isSubdirOf(inside: *mut Stream_t, outside: *mut Stream_t) -> i32;
    fn SimpleFileOpen(
        dev: *mut device,
        orig_dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        mode2: i32,
        locked: i32,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn get_fd(Stream: *mut Stream_t) -> i32;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const i8,
        shortname: *const i8,
        cb: Option<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> i32;
    fn handle_clash_options(ch: *mut ClashHandling_t, c: i32) -> i32;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn createDir(
        Dir: *mut Stream_t,
        filename: *const i8,
        ch: *mut ClashHandling_t,
        attr: u8,
        mtime: time_t,
    ) -> *mut Stream_t;
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn fat_free(Dir: *mut Stream_t, fat: u32) -> i32;
    fn utimes(__file: *const i8, __tvp: *const timeval) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = i32;
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
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [i8; 8],
    pub ext: [i8; 3],
    pub attr: u8,
    pub Case: u8,
    pub ctime_ms: u8,
    pub ctime: [u8; 2],
    pub cdate: [u8; 2],
    pub adate: [u8; 2],
    pub startHi: [u8; 2],
    pub time: [u8; 2],
    pub date: [u8; 2],
    pub start: [u8; 2],
    pub size: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MainParam_t {
    pub loop_0: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut MainParam_t, *const i8) -> i32,
    >,
    pub dirCallback: Option<
        unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    >,
    pub callback: Option<unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32>,
    pub unixcallback: Option<unsafe extern "C" fn(*mut MainParam_t) -> i32>,
    pub arg: *mut libc::c_void,
    pub openflags: i32,
    pub lookupflags: i32,
    pub fast_quit: i32,
    pub shortname: bounded_string,
    pub longname: bounded_string,
    pub File: *mut Stream_t,
    pub direntry: *mut direntry_t,
    pub unixSourceName: *mut i8,
    pub targetDir: *mut Stream_t,
    pub targetName: *const i8,
    pub originalArg: *mut i8,
    pub basenameHasWildcard: i32,
    pub mcwd: [i8; 132],
    pub targetBuffer: [i8; 1021],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: i32,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: u32,
    pub endSlot: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bounded_string {
    pub data: *mut i8,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_t {
    pub recursive: i32,
    pub preserveAttributes: i32,
    pub preserveTime: i32,
    pub attr: u8,
    pub path: *mut i8,
    pub textmode: i32,
    pub needfilter: i32,
    pub nowarn: i32,
    pub verbose: i32,
    pub type_0: i32,
    pub convertCharset: i32,
    pub mp: MainParam_t,
    pub ch: ClashHandling_t,
    pub noClobber: i32,
    pub unixTarget: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClashHandling_t {
    pub action: [clash_action; 2],
    pub namematch_default: [clash_action; 2],
    pub nowarn: i32,
    pub got_slots: i32,
    pub mod_time: i32,
    pub myname: *mut i8,
    pub dosname: *mut u8,
    pub single: i32,
    pub use_longname: i32,
    pub ignore_entry: i32,
    pub source: i32,
    pub source_entry: i32,
    pub name_converter: Option<
        unsafe extern "C" fn(
            *mut doscp_t,
            *const i8,
            i32,
            *mut i32,
            *mut dos_name_t,
        ) -> (),
    >,
    pub is_label: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum clash_action {
    NAMEMATCH_GREW = 9,
    NAMEMATCH_SUCCESS = 8,
    NAMEMATCH_ERROR = 7,
    NAMEMATCH_OVERWRITE = 6,
    NAMEMATCH_PRENAME = 5,
    NAMEMATCH_RENAME = 4,
    NAMEMATCH_SKIP = 3,
    NAMEMATCH_QUIT = 2,
    NAMEMATCH_AUTORENAME = 1,
    NAMEMATCH_NONE = 0,
}
impl clash_action {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            clash_action::NAMEMATCH_GREW => 9,
            clash_action::NAMEMATCH_SUCCESS => 8,
            clash_action::NAMEMATCH_ERROR => 7,
            clash_action::NAMEMATCH_OVERWRITE => 6,
            clash_action::NAMEMATCH_PRENAME => 5,
            clash_action::NAMEMATCH_RENAME => 4,
            clash_action::NAMEMATCH_SKIP => 3,
            clash_action::NAMEMATCH_QUIT => 2,
            clash_action::NAMEMATCH_AUTORENAME => 1,
            clash_action::NAMEMATCH_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> clash_action {
        match value {
            9 => clash_action::NAMEMATCH_GREW,
            8 => clash_action::NAMEMATCH_SUCCESS,
            7 => clash_action::NAMEMATCH_ERROR,
            6 => clash_action::NAMEMATCH_OVERWRITE,
            5 => clash_action::NAMEMATCH_PRENAME,
            4 => clash_action::NAMEMATCH_RENAME,
            3 => clash_action::NAMEMATCH_SKIP,
            2 => clash_action::NAMEMATCH_QUIT,
            1 => clash_action::NAMEMATCH_AUTORENAME,
            0 => clash_action::NAMEMATCH_NONE,
            _ => panic!("Invalid value for clash_action: {}", value),
        }
    }
}
impl AddAssign<u32> for clash_action {
    fn add_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for clash_action {
    fn sub_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for clash_action {
    fn mul_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for clash_action {
    fn div_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for clash_action {
    fn rem_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for clash_action {
    type Output = clash_action;
    fn add(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for clash_action {
    type Output = clash_action;
    fn sub(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for clash_action {
    type Output = clash_action;
    fn mul(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for clash_action {
    type Output = clash_action;
    fn div(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for clash_action {
    type Output = clash_action;
    fn rem(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut i8,
    *mut libc::c_void,
    *mut direntry_t,
) -> i32;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn ptrdiff(mut end: *const i8, mut begin: *const i8) -> size_t {
    return end.offset_from(begin) as i64 as size_t;
}
unsafe extern "C" fn set_mtime(mut target: *const i8, mut mtime: time_t) {
    if !target.is_null() && strcmp(target, b"-\0" as *const u8 as *const i8) != 0
        && mtime != 0 as i64
    {
        let mut tv: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
        tv[0 as i32 as usize].tv_sec = mtime;
        tv[0 as i32 as usize].tv_usec = 0 as i32 as __suseconds_t;
        tv[1 as i32 as usize].tv_sec = mtime;
        tv[1 as i32 as usize].tv_usec = 0 as i32 as __suseconds_t;
        utimes(target, tv.as_mut_ptr() as *const timeval);
    }
}
unsafe extern "C" fn buildUnixFilename(mut arg: *mut Arg_t) -> *mut i8 {
    let mut target: *const i8 = 0 as *const i8;
    let mut ret: *mut i8 = 0 as *mut i8;
    let mut tmp: *mut i8 = 0 as *mut i8;
    target = mpPickTargetName(&mut (*arg).mp);
    ret = malloc(
        (strlen((*arg).unixTarget))
            .wrapping_add(2 as i32 as u64)
            .wrapping_add(strlen(target)),
    ) as *mut i8;
    if ret.is_null() {
        return 0 as *mut i8;
    }
    strcpy(ret, (*arg).unixTarget);
    strcat(ret, b"/\0" as *const u8 as *const i8);
    if *target != 0 {
        if strcmp(target, b".\0" as *const u8 as *const i8) == 0 {
            target = b"DOT\0" as *const u8 as *const i8;
        } else if strcmp(target, b"..\0" as *const u8 as *const i8) == 0 {
            target = b"DOTDOT\0" as *const u8 as *const i8;
        }
        loop {
            tmp = strchr(target, '/' as i32);
            if tmp.is_null() {
                break;
            }
            strncat(ret, target, ptrdiff(tmp, target));
            strcat(ret, b"\\\0" as *const u8 as *const i8);
            target = tmp.offset(1 as i32 as isize);
        }
        strcat(ret, target);
    }
    return ret;
}
unsafe extern "C" fn unix_is_dir(mut name: *const i8) -> i32 {
    let mut buf: stat = stat {
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
    if stat(name, &mut buf) < 0 as i32 {
        return -(1 as i32)
    } else {
        return (1 as i32 != 0
            && buf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) as i32
    };
}
unsafe extern "C" fn unix_target_lookup(
    mut arg: *mut Arg_t,
    mut in_0: *const i8,
) -> i32 {
    let mut ptr: *mut i8 = 0 as *mut i8;
    (*arg).unixTarget = strdup(in_0);
    if access((*arg).unixTarget, 0 as i32) == 0 as i32 {
        match unix_is_dir((*arg).unixTarget) {
            -1 => return 16 as i32,
            0 => {}
            _ => return 4 as i32,
        }
    } else if *__errno_location() != 2 as i32 {
        return 16 as i32
    }
    ptr = strrchr((*arg).unixTarget, '/' as i32);
    if ptr.is_null() {
        (*arg).mp.targetName = (*arg).unixTarget;
        (*arg).unixTarget = strdup(b".\0" as *const u8 as *const i8);
        return 4 as i32;
    } else {
        *ptr = '\0' as i32 as i8;
        (*arg).mp.targetName = ptr.offset(1 as i32 as isize);
        return 4 as i32;
    };
}
unsafe extern "C" fn target_lookup(mut arg: *mut Arg_t, mut in_0: *const i8) -> i32 {
    if *in_0.offset(0 as i32 as isize) as i32 != 0
        && *in_0.offset(1 as i32 as isize) as i32 == ':' as i32
    {
        return dos_target_lookup(&mut (*arg).mp, in_0)
    } else {
        return unix_target_lookup(arg, in_0)
    };
}
unsafe extern "C" fn unix_write(mut mp: *mut MainParam_t, mut needfilter: i32) -> i32 {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if (*arg).type_0 != 0 {
        return mt_unix_write(mp, needfilter, b"-\0" as *const u8 as *const i8)
    } else {
        let mut unixFile: *mut i8 = buildUnixFilename(arg);
        let mut ret: i32 = 0;
        if unixFile.is_null() {
            printOom();
            return 16 as i32;
        }
        ret = mt_unix_write(mp, needfilter, unixFile);
        free(unixFile as *mut libc::c_void);
        return ret;
    };
}
unsafe extern "C" fn mt_unix_write(
    mut mp: *mut MainParam_t,
    mut needfilter: i32,
    mut unixFile: *const i8,
) -> i32 {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut mtime: time_t = 0;
    let mut File: *mut Stream_t = (*mp).File;
    let mut Target: *mut Stream_t = 0 as *mut Stream_t;
    let mut Source: *mut Stream_t = 0 as *mut Stream_t;
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
    let mut errmsg: [i8; 200] = [0; 200];
    ((*(*File).Class).get_data)
        .expect(
            "non-null function pointer",
        )(File, &mut mtime, 0 as *mut mt_off_t, 0 as *mut i32, 0 as *mut uint32_t);
    if (*arg).preserveTime == 0 {
        mtime = 0 as i64;
    }
    if (*arg).type_0 == 0 {
        if (*arg).nowarn == 0 && access(unixFile, 0 as i32) == 0 {
            if (*arg).noClobber != 0 {
                fprintf(
                    stderr,
                    b"File \"%s\" exists. To overwrite, try again, and explicitly specify target directory\n\0"
                        as *const u8 as *const i8,
                    unixFile,
                );
                return 16 as i32;
            }
            if stat(unixFile, &mut stbuf) == 0 {
                let mut srcStbuf: stat = stat {
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
                let mut sFd: i32 = 0;
                if !(stbuf.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
                    fprintf(
                        stderr,
                        b"\"%s\" is not a regular file\n\0" as *const u8 as *const i8,
                        unixFile,
                    );
                    return 16 as i32;
                }
                sFd = get_fd(File);
                if !(sFd == -(1 as i32)) {
                    if fstat(sFd, &mut srcStbuf) == 0 && stbuf.st_dev == srcStbuf.st_dev
                        && stbuf.st_ino == srcStbuf.st_ino
                    {
                        fprintf(
                            stderr,
                            b"Attempt to copy file on itself\n\0" as *const u8
                                as *const i8,
                        );
                        return 16 as i32;
                    }
                }
            }
            if ask_confirmation(
                b"File \"%s\" exists, overwrite (y/n) ? \0" as *const u8 as *const i8,
                unixFile,
            ) != 0
            {
                return 16 as i32;
            }
        }
    }
    if (*arg).type_0 == 0 && (*arg).verbose != 0 {
        fprintf(stderr, b"Copying \0" as *const u8 as *const i8);
        mpPrintFilename(stderr, mp);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    }
    if got_signal != 0 {
        return 16 as i32;
    }
    Target = SimpleFileOpen(
        0 as *mut device,
        0 as *mut device,
        unixFile,
        0o1 as i32 | 0o100 as i32 | 0o1000 as i32,
        errmsg.as_mut_ptr(),
        0 as i32,
        0 as i32,
        0 as *mut mt_off_t,
    );
    if !Target.is_null() {
        let mut ret: mt_off_t = 0;
        Source = copy_stream(File);
        if needfilter != 0 && (*arg).textmode != 0 {
            Source = open_dos2unix(Source, (*arg).convertCharset);
        }
        if !Source.is_null() {
            ret = copyfile(Source, Target);
        } else {
            ret = -(1 as i32) as mt_off_t;
        }
        free_stream(&mut Source);
        free_stream(&mut Target);
        if ret < 0 as i32 as i64 {
            if (*arg).type_0 == 0 {
                unlink(unixFile);
            }
            return 16 as i32;
        }
        if (*arg).type_0 == 0 {
            set_mtime(unixFile, mtime);
        }
        return 4 as i32;
    } else {
        fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, errmsg.as_mut_ptr());
        return 16 as i32;
    };
}
unsafe extern "C" fn makeUnixDir(mut filename: *mut i8) -> i32 {
    if mkdir(filename, 0o777 as i32 as __mode_t) == 0 {
        return 0 as i32;
    }
    if *__errno_location() == 17 as i32 {
        let mut buf: stat = stat {
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
        if stat(filename, &mut buf) < 0 as i32 {
            return -(1 as i32);
        }
        if buf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
            return 0 as i32;
        }
        *__errno_location() = 20 as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn unix_copydir(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut mtime: time_t = 0;
    let mut File: *mut Stream_t = (*mp).File;
    let mut ret: i32 = 0;
    let mut unixFile: *mut i8 = 0 as *mut i8;
    if (*arg).recursive == 0 && (*mp).basenameHasWildcard != 0 {
        return 0 as i32;
    }
    ((*(*File).Class).get_data)
        .expect(
            "non-null function pointer",
        )(File, &mut mtime, 0 as *mut mt_off_t, 0 as *mut i32, 0 as *mut uint32_t);
    if (*arg).preserveTime == 0 {
        mtime = 0 as i64;
    }
    if (*arg).type_0 == 0 && (*arg).verbose != 0 {
        fprintf(stderr, b"Copying \0" as *const u8 as *const i8);
        fprintPwd(stderr, entry, 0 as i32);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    }
    if got_signal != 0 {
        return 16 as i32;
    }
    unixFile = buildUnixFilename(arg);
    if unixFile.is_null() {
        printOom();
        return 16 as i32;
    }
    if (*arg).type_0 != 0 || makeUnixDir(unixFile) == 0 {
        let mut newArg: Arg_t = Arg_t {
            recursive: 0,
            preserveAttributes: 0,
            preserveTime: 0,
            attr: 0,
            path: 0 as *mut i8,
            textmode: 0,
            needfilter: 0,
            nowarn: 0,
            verbose: 0,
            type_0: 0,
            convertCharset: 0,
            mp: MainParam_t {
                loop_0: None,
                dirCallback: None,
                callback: None,
                unixcallback: None,
                arg: 0 as *mut libc::c_void,
                openflags: 0,
                lookupflags: 0,
                fast_quit: 0,
                shortname: bounded_string {
                    data: 0 as *mut i8,
                    len: 0,
                },
                longname: bounded_string {
                    data: 0 as *mut i8,
                    len: 0,
                },
                File: 0 as *mut Stream_t,
                direntry: 0 as *mut direntry_t,
                unixSourceName: 0 as *mut i8,
                targetDir: 0 as *mut Stream_t,
                targetName: 0 as *const i8,
                originalArg: 0 as *mut i8,
                basenameHasWildcard: 0,
                mcwd: [0; 132],
                targetBuffer: [0; 1021],
            },
            ch: ClashHandling_t {
                action: [clash_action::NAMEMATCH_NONE; 2],
                namematch_default: [clash_action::NAMEMATCH_NONE; 2],
                nowarn: 0,
                got_slots: 0,
                mod_time: 0,
                myname: 0 as *mut i8,
                dosname: 0 as *mut u8,
                single: 0,
                use_longname: 0,
                ignore_entry: 0,
                source: 0,
                source_entry: 0,
                name_converter: None,
                is_label: 0,
            },
            noClobber: 0,
            unixTarget: 0 as *const i8,
        };
        newArg = *arg;
        newArg.mp.arg = &mut newArg as *mut Arg_t as *mut libc::c_void;
        newArg.unixTarget = unixFile;
        newArg.mp.targetName = 0 as *const i8;
        newArg.mp.basenameHasWildcard = 1 as i32;
        ret = ((*mp).loop_0)
            .expect(
                "non-null function pointer",
            )(File, &mut newArg.mp, b"*\0" as *const u8 as *const i8);
        set_mtime(unixFile, mtime);
        free(unixFile as *mut libc::c_void);
        return ret | 4 as i32;
    } else {
        fprintf(
            stderr,
            b"Failure to make directory %s: %s\n\0" as *const u8 as *const i8,
            unixFile,
            strerror(*__errno_location()),
        );
        free(unixFile as *mut libc::c_void);
        return 16 as i32;
    };
}
unsafe extern "C" fn dos_to_unix(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    return unix_write(mp, 1 as i32);
}
unsafe extern "C" fn unix_to_unix(mut mp: *mut MainParam_t) -> i32 {
    return unix_write(mp, 0 as i32);
}
unsafe extern "C" fn directory_dos_to_unix(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    return unix_copydir(entry, mp);
}
unsafe extern "C" fn writeit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut arg0: *mut libc::c_void,
    mut entry: *mut direntry_t,
) -> i32 {
    let mut Target: *mut Stream_t = 0 as *mut Stream_t;
    let mut now: time_t = 0;
    let mut type_0: i32 = 0;
    let mut ret: mt_off_t = 0;
    let mut fat: uint32_t = 0;
    let mut date: time_t = 0;
    let mut filesize: mt_off_t = 0;
    let mut arg: *mut Arg_t = arg0 as *mut Arg_t;
    let mut Source: *mut Stream_t = copy_stream((*arg).mp.File);
    if ((*(*Source).Class).get_data)
        .expect(
            "non-null function pointer",
        )(Source, &mut date, &mut filesize, &mut type_0, 0 as *mut uint32_t) < 0 as i32
    {
        fprintf(stderr, b"Can't stat source file\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    if fileTooBig(filesize) != 0 {
        fprintf(stderr, b"File \"%s\" too big\n\0" as *const u8 as *const i8, longname);
        return 1 as i32;
    }
    if type_0 != 0 {
        if (*arg).verbose != 0 {
            fprintf(
                stderr,
                b"\"%s\" is a directory\n\0" as *const u8 as *const i8,
                longname,
            );
        }
        return -(1 as i32);
    }
    if (*arg).verbose != 0 {
        fprintf(stderr, b"Copying %s\n\0" as *const u8 as *const i8, longname);
    }
    if got_signal != 0 {
        return -(1 as i32);
    }
    if getfreeMinBytes((*arg).mp.targetDir, filesize) == 0 {
        return -(1 as i32);
    }
    if (*arg).preserveTime != 0 {
        now = date;
    } else {
        getTimeNow(&mut now);
    }
    mk_entry(
        dosname,
        (*arg).attr,
        1 as i32 as u32,
        0 as i32 as uint32_t,
        now,
        &mut (*entry).dir,
    );
    Target = OpenFileByDirentry(entry);
    if Target.is_null() {
        fprintf(stderr, b"Could not open Target\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if (*arg).needfilter & (*arg).textmode != 0 {
        Source = open_unix2dos(Source, (*arg).convertCharset);
    }
    ret = copyfile(Source, Target);
    ((*(*Target).Class).get_data)
        .expect(
            "non-null function pointer",
        )(Target, 0 as *mut time_t, 0 as *mut mt_off_t, 0 as *mut i32, &mut fat);
    free_stream(&mut Source);
    free_stream(&mut Target);
    if ret < 0 as i32 as i64 {
        fat_free((*arg).mp.targetDir, fat);
        return -(1 as i32);
    } else {
        mk_entry(dosname, (*arg).attr, fat, ret as uint32_t, now, &mut (*entry).dir);
        return 0 as i32;
    };
}
unsafe extern "C" fn dos_write(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
    mut needfilter: i32,
) -> i32 {
    let mut result: i32 = 0;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut targetName: *const i8 = mpPickTargetName(mp);
    if !entry.is_null() && (*arg).preserveAttributes != 0 {
        (*arg).attr = (*entry).dir.attr;
    } else {
        (*arg).attr = 0x20 as i32 as u8;
    }
    (*arg).needfilter = needfilter;
    if !entry.is_null() && (*mp).targetDir == (*entry).Dir {
        (*arg).ch.ignore_entry = -(1 as i32);
        (*arg).ch.source = (*entry).entry;
    } else {
        (*arg).ch.ignore_entry = -(1 as i32);
        (*arg).ch.source = -(2 as i32);
    }
    result = mwrite_one(
        (*mp).targetDir,
        targetName,
        0 as *const i8,
        Some(
            writeit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut i8,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> i32,
        ),
        arg as *mut libc::c_void,
        &mut (*arg).ch,
    );
    if result == 1 as i32 { return 4 as i32 } else { return 16 as i32 };
}
unsafe extern "C" fn subDir(
    mut parent: *mut Stream_t,
    mut filename: *const i8,
) -> *mut Stream_t {
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    initializeDirentry(&mut entry, parent);
    match vfat_lookup_zt(
        &mut entry,
        filename,
        0x10 as i32,
        0 as *mut i8,
        0 as i32 as size_t,
        0 as *mut i8,
        0 as i32 as size_t,
    ) {
        0 => return OpenFileByDirentry(&mut entry),
        -1 => return 0 as *mut Stream_t,
        _ => return 0 as *mut Stream_t,
    };
}
unsafe extern "C" fn dos_copydir(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut newArg: Arg_t = Arg_t {
        recursive: 0,
        preserveAttributes: 0,
        preserveTime: 0,
        attr: 0,
        path: 0 as *mut i8,
        textmode: 0,
        needfilter: 0,
        nowarn: 0,
        verbose: 0,
        type_0: 0,
        convertCharset: 0,
        mp: MainParam_t {
            loop_0: None,
            dirCallback: None,
            callback: None,
            unixcallback: None,
            arg: 0 as *mut libc::c_void,
            openflags: 0,
            lookupflags: 0,
            fast_quit: 0,
            shortname: bounded_string {
                data: 0 as *mut i8,
                len: 0,
            },
            longname: bounded_string {
                data: 0 as *mut i8,
                len: 0,
            },
            File: 0 as *mut Stream_t,
            direntry: 0 as *mut direntry_t,
            unixSourceName: 0 as *mut i8,
            targetDir: 0 as *mut Stream_t,
            targetName: 0 as *const i8,
            originalArg: 0 as *mut i8,
            basenameHasWildcard: 0,
            mcwd: [0; 132],
            targetBuffer: [0; 1021],
        },
        ch: ClashHandling_t {
            action: [clash_action::NAMEMATCH_NONE; 2],
            namematch_default: [clash_action::NAMEMATCH_NONE; 2],
            nowarn: 0,
            got_slots: 0,
            mod_time: 0,
            myname: 0 as *mut i8,
            dosname: 0 as *mut u8,
            single: 0,
            use_longname: 0,
            ignore_entry: 0,
            source: 0,
            source_entry: 0,
            name_converter: None,
            is_label: 0,
        },
        noClobber: 0,
        unixTarget: 0 as *const i8,
    };
    let mut now: time_t = 0;
    let mut date: time_t = 0;
    let mut ret: i32 = 0;
    let mut targetName: *const i8 = mpPickTargetName(mp);
    if (*arg).recursive == 0 && (*mp).basenameHasWildcard != 0 {
        return 0 as i32;
    }
    if !entry.is_null() && isSubdirOf((*mp).targetDir, (*mp).File) != 0 {
        fprintf(
            stderr,
            b"Cannot recursively copy directory \0" as *const u8 as *const i8,
        );
        fprintPwd(stderr, entry, 0 as i32);
        fprintf(
            stderr,
            b" into one of its own subdirectories \0" as *const u8 as *const i8,
        );
        fprintPwd(stderr, getDirentry((*mp).targetDir), 0 as i32);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        return 16 as i32;
    }
    if ((*(*(*arg).mp.File).Class).get_data)
        .expect(
            "non-null function pointer",
        )(
        (*arg).mp.File,
        &mut date,
        0 as *mut mt_off_t,
        0 as *mut i32,
        0 as *mut uint32_t,
    ) < 0 as i32
    {
        fprintf(stderr, b"Can't stat source file\n\0" as *const u8 as *const i8);
        return 16 as i32;
    }
    if (*arg).type_0 == 0 && (*arg).verbose != 0 {
        fprintf(stderr, b"Copying %s\n\0" as *const u8 as *const i8, mpGetBasename(mp));
    }
    if !entry.is_null() && (*arg).preserveAttributes != 0 {
        (*arg).attr = (*entry).dir.attr;
    } else {
        (*arg).attr = 0 as i32 as u8;
    }
    if !entry.is_null() && (*mp).targetDir == (*entry).Dir {
        (*arg).ch.ignore_entry = -(1 as i32);
        (*arg).ch.source = (*entry).entry;
    } else {
        (*arg).ch.ignore_entry = -(1 as i32);
        (*arg).ch.source = -(2 as i32);
    }
    if (*arg).preserveTime != 0 {
        now = date;
    } else {
        getTimeNow(&mut now);
    }
    newArg = *arg;
    newArg.mp.arg = &mut newArg as *mut Arg_t as *mut libc::c_void;
    newArg.mp.targetName = 0 as *const i8;
    newArg.mp.basenameHasWildcard = 1 as i32;
    if *targetName != 0 {
        newArg.mp.targetDir = subDir((*mp).targetDir, targetName);
        if (newArg.mp.targetDir).is_null() {
            newArg.mp.targetDir = createDir(
                (*mp).targetDir,
                targetName,
                &mut (*arg).ch,
                (*arg).attr,
                now,
            );
        }
    } else {
        newArg.mp.targetDir = (*mp).targetDir;
    }
    if (newArg.mp.targetDir).is_null() {
        return 16 as i32;
    }
    ret = ((*mp).loop_0)
        .expect(
            "non-null function pointer",
        )((*mp).File, &mut newArg.mp, b"*\0" as *const u8 as *const i8);
    if *targetName != 0 {
        free_stream(&mut newArg.mp.targetDir);
    }
    return ret | 4 as i32;
}
unsafe extern "C" fn dos_to_dos(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    return dos_write(entry, mp, 0 as i32);
}
unsafe extern "C" fn unix_to_dos(mut mp: *mut MainParam_t) -> i32 {
    return dos_write(0 as *mut direntry_t, mp, 1 as i32);
}
unsafe extern "C" fn usage(mut ret: i32) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-spatnmQVBT] [-D clash_option] sourcefile targetfile\n\0"
            as *const u8 as *const i8,
        progname,
    );
    fprintf(
        stderr,
        b"       %s [-spatnmQVBT] [-D clash_option] sourcefile [sourcefiles...] targetdirectory\n\0"
            as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mcopy(mut argc: i32, mut argv: *mut *mut i8, mut mtype: i32) {
    let mut arg: Arg_t = Arg_t {
        recursive: 0,
        preserveAttributes: 0,
        preserveTime: 0,
        attr: 0,
        path: 0 as *mut i8,
        textmode: 0,
        needfilter: 0,
        nowarn: 0,
        verbose: 0,
        type_0: 0,
        convertCharset: 0,
        mp: MainParam_t {
            loop_0: None,
            dirCallback: None,
            callback: None,
            unixcallback: None,
            arg: 0 as *mut libc::c_void,
            openflags: 0,
            lookupflags: 0,
            fast_quit: 0,
            shortname: bounded_string {
                data: 0 as *mut i8,
                len: 0,
            },
            longname: bounded_string {
                data: 0 as *mut i8,
                len: 0,
            },
            File: 0 as *mut Stream_t,
            direntry: 0 as *mut direntry_t,
            unixSourceName: 0 as *mut i8,
            targetDir: 0 as *mut Stream_t,
            targetName: 0 as *const i8,
            originalArg: 0 as *mut i8,
            basenameHasWildcard: 0,
            mcwd: [0; 132],
            targetBuffer: [0; 1021],
        },
        ch: ClashHandling_t {
            action: [clash_action::NAMEMATCH_NONE; 2],
            namematch_default: [clash_action::NAMEMATCH_NONE; 2],
            nowarn: 0,
            got_slots: 0,
            mod_time: 0,
            myname: 0 as *mut i8,
            dosname: 0 as *mut u8,
            single: 0,
            use_longname: 0,
            ignore_entry: 0,
            source: 0,
            source_entry: 0,
            name_converter: None,
            is_label: 0,
        },
        noClobber: 0,
        unixTarget: 0 as *const i8,
    };
    let mut c: i32 = 0;
    let mut fastquit: i32 = 0;
    init_clash_handling(&mut arg.ch);
    arg.recursive = 0 as i32;
    arg.preserveTime = 0 as i32;
    arg.preserveAttributes = 0 as i32;
    arg.nowarn = 0 as i32;
    arg.textmode = 0 as i32;
    arg.verbose = 0 as i32;
    arg.convertCharset = 0 as i32;
    arg.type_0 = mtype;
    fastquit = 0 as i32;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:abB/sptTnmvQD:oh\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        let mut current_block_26: u64;
        match c {
            105 => {
                set_cmd_line_image(optarg);
                current_block_26 = 11932355480408055363;
            }
            115 | 47 => {
                arg.recursive = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            112 => {
                arg.preserveAttributes = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            84 => {
                arg.convertCharset = 1 as i32;
                current_block_26 = 2535692301906169108;
            }
            97 | 116 => {
                current_block_26 = 2535692301906169108;
            }
            110 => {
                arg.nowarn = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            109 => {
                arg.preserveTime = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            118 => {
                arg.verbose = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            81 => {
                fastquit = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            66 | 98 => {
                batchmode = 1 as i32;
                current_block_26 = 11932355480408055363;
            }
            111 => {
                handle_clash_options(&mut arg.ch, c);
                current_block_26 = 11932355480408055363;
            }
            68 => {
                if handle_clash_options(&mut arg.ch, *optarg as i32) != 0 {
                    usage(1 as i32);
                }
                current_block_26 = 11932355480408055363;
            }
            104 => {
                usage(0 as i32);
            }
            63 => {
                usage(1 as i32);
            }
            _ => {
                current_block_26 = 11932355480408055363;
            }
        }
        match current_block_26 {
            2535692301906169108 => {
                arg.textmode = 1 as i32;
            }
            _ => {}
        }
    }
    if argc - optind < 1 as i32 {
        usage(1 as i32);
    }
    init_mp(&mut arg.mp);
    arg.unixTarget = 0 as *const i8;
    arg.mp.lookupflags = 0x20 as i32 | 0x10 as i32 | 1 as i32 | 0x100 as i32
        | 0x2000 as i32;
    arg.mp.fast_quit = fastquit;
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.openflags = 0 as i32;
    arg.noClobber = 0 as i32;
    if mtype == 0
        && strcmp(
            *argv.offset((argc - 1 as i32) as isize),
            b"-\0" as *const u8 as *const i8,
        ) == 0
    {
        mtype = 1 as i32;
        arg.type_0 = mtype;
        argc -= 1;
        argc;
    }
    if mtype != 0 {
        arg.mp.targetName = strdup(b"-\0" as *const u8 as *const i8);
        arg.unixTarget = strdup(b"\0" as *const u8 as *const i8);
        arg.mp.callback = Some(
            dos_to_unix as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
        arg.mp.dirCallback = Some(
            unix_copydir
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
        arg.mp.unixcallback = Some(
            unix_to_unix as unsafe extern "C" fn(*mut MainParam_t) -> i32,
        );
    } else {
        let mut target: *const i8 = 0 as *const i8;
        if argc - optind == 1 as i32 {
            target = b".\0" as *const u8 as *const i8;
            arg.noClobber = 1 as i32;
        } else {
            argc -= 1;
            argc;
            target = *argv.offset(argc as isize);
        }
        if target_lookup(&mut arg, target) == 16 as i32 {
            fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const i8,
                target,
                strerror(*__errno_location()),
            );
            exit(1 as i32);
        }
        if (arg.mp.targetDir).is_null() && (arg.unixTarget).is_null() {
            fprintf(stderr, b"Bad target %s\n\0" as *const u8 as *const i8, target);
            exit(1 as i32);
        }
        if !(arg.unixTarget).is_null() {
            arg.mp.callback = Some(
                dos_to_unix
                    as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
            );
            arg.mp.dirCallback = Some(
                directory_dos_to_unix
                    as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
            );
            arg.mp.unixcallback = Some(
                unix_to_unix as unsafe extern "C" fn(*mut MainParam_t) -> i32,
            );
        } else {
            arg.mp.dirCallback = Some(
                dos_copydir
                    as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
            );
            arg.mp.callback = Some(
                dos_to_dos
                    as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
            );
            arg.mp.unixcallback = Some(
                unix_to_dos as unsafe extern "C" fn(*mut MainParam_t) -> i32,
            );
        }
    }
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind));
}
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
    pub type doscp_t;
    fn random() -> i64;
    fn srandom(__seed: u32);
    fn exit(_: i32) -> !;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn wcschr(_: *const i32, _: i32) -> *mut i32;
    fn iswlower(__wc: wint_t) -> i32;
    fn iswupper(__wc: wint_t) -> i32;
    fn towupper(__wc: wint_t) -> wint_t;
    static mut progname: *const i8;
    fn mk_entry(
        filename: *const dos_name_t,
        attr: u8,
        fat: u32,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn allow_interrupts(ss: *mut saved_sig_state);
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn get_default_drive() -> i8;
    fn set_cmd_line_image(img: *mut i8);
    fn check_number_parse_errno(c: i8, optarg_0: *const i8, endptr: *mut i8);
    fn strtou32(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint32_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const i8,
        shortname: *const i8,
        cb: Option<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> i32;
    fn open_root_dir(drivename: i8, flags: i32, isRop: *mut i32) -> *mut Stream_t;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    fn getTimeNow(now: *mut time_t) -> time_t;
    fn ask_confirmation(_: *const i8, _: ...) -> i32;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    fn wchar_to_dos(
        toDos: *mut doscp_t,
        wchar: *mut wchar_t,
        dos: *mut i8,
        len: size_t,
        mangled: *mut i32,
    );
    fn native_to_wchar(
        native: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const i8,
        mangled: *mut i32,
    ) -> size_t;
    fn vfat_lookup(
        entry: *mut direntry_t,
        filename: *const i8,
        length: size_t,
        flags: i32,
        shortname: *mut i8,
        shortname_len: size_t,
        longname: *mut i8,
        longname_len: size_t,
    ) -> i32;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn isNotFound(entry: *mut direntry_t) -> i32;
    fn wipeEntry(entry: *mut direntry_t);
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
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
pub type wint_t = u32;
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
pub type mt_off_t = off_t;
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
pub struct dos_name_t {
    pub base: [i8; 8],
    pub ext: [i8; 3],
    pub sentinel: i8,
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
pub struct saved_sig_state {
    pub sa: [sigaction; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union bootsector {
    pub bytes: [u8; 4096],
    pub characters: [i8; 4096],
    pub boot: bootsector_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootsector_s {
    pub jump: [u8; 3],
    pub banner: [i8; 8],
    pub secsiz: [u8; 2],
    pub clsiz: u8,
    pub nrsvsect: [u8; 2],
    pub nfat: u8,
    pub dirents: [u8; 2],
    pub psect: [u8; 2],
    pub descr: u8,
    pub fatlen: [u8; 2],
    pub nsect: [u8; 2],
    pub nheads: [u8; 2],
    pub nhs: [u8; 4],
    pub bigsect: [u8; 4],
    pub ext: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub fat32: fat32_t,
    pub old: oldboot_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldboot_t {
    pub labelBlock: label_blk_t,
    pub res_2m: u8,
    pub CheckSum: u8,
    pub fmt_2mf: u8,
    pub wt: u8,
    pub rate_0: u8,
    pub rate_any: u8,
    pub BootP: [u8; 2],
    pub Infp0: [u8; 2],
    pub InfpX: [u8; 2],
    pub InfTm: [u8; 2],
    pub DateF: [u8; 2],
    pub TimeF: [u8; 2],
    pub junk: [u8; 944],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_blk_t {
    pub physdrive: u8,
    pub reserved: u8,
    pub dos4: u8,
    pub serial: [u8; 4],
    pub label: [i8; 11],
    pub fat_type: [i8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fat32_t {
    pub bigFat: [u8; 4],
    pub extFlags: [u8; 2],
    pub fsVersion: [u8; 2],
    pub rootCluster: [u8; 4],
    pub infoSector: [u8; 2],
    pub backupBoot: [u8; 2],
    pub reserved: [u8; 6],
    pub reserved2: [u8; 6],
    pub labelBlock: label_blk_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
    SER_NONE = 0,
    SER_SET = 2,
    SER_RANDOM = 1,
}
impl C2RustUnnamed_11 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_11::SER_NONE => 0,
            C2RustUnnamed_11::SER_SET => 2,
            C2RustUnnamed_11::SER_RANDOM => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_11 {
        match value {
            0 => C2RustUnnamed_11::SER_NONE,
            2 => C2RustUnnamed_11::SER_SET,
            1 => C2RustUnnamed_11::SER_RANDOM,
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
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut i8,
    *mut libc::c_void,
    *mut direntry_t,
) -> i32;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_toupper(mut ch: i8) -> i8 {
    return ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = ch as u8 as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(ch as u8 as i32);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(ch as u8 as i32 as isize);
        }
        __res
    }) as i8;
}
#[inline]
unsafe extern "C" fn init_random() {
    srandom(time(0 as *mut time_t) as u32);
}
#[inline]
unsafe extern "C" fn ch_towupper(mut ch: wchar_t) -> wchar_t {
    return towupper(ch as wint_t) as wchar_t;
}
#[inline]
unsafe extern "C" fn set_dword(mut data: *mut u8, mut value: uint32_t) {
    *data.offset(3 as i32 as isize) = (value >> 24 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(2 as i32 as isize) = (value >> 16 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(1 as i32 as isize) = (value >> 8 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(0 as i32 as isize) = (value >> 0 as i32 & 0xff as i32 as u32) as u8;
}
unsafe extern "C" fn mt_label_name(
    mut cp: *mut doscp_t,
    mut filename: *const i8,
    mut verbose: i32,
    mut mangled: *mut i32,
    mut ans: *mut dos_name_t,
    mut preserve_case: i32,
) {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut have_lower: i32 = 0;
    let mut have_upper: i32 = 0;
    let mut wbuffer: [wchar_t; 12] = [0; 12];
    memset(
        ans as *mut libc::c_void,
        ' ' as i32,
        (::core::mem::size_of::<dos_name_t>() as u64).wrapping_sub(1 as i32 as u64),
    );
    (*ans).sentinel = '\0' as i32 as i8;
    len = native_to_wchar(
        filename,
        wbuffer.as_mut_ptr(),
        11 as i32 as size_t,
        0 as *const i8,
        0 as *mut i32,
    );
    if len > 11 as i32 as u64 {
        *mangled = 1 as i32;
        len = 11 as i32 as size_t;
    } else {
        *mangled = 0 as i32;
    }
    have_upper = 0 as i32;
    have_lower = have_upper;
    i = 0 as i32 as size_t;
    while i < len {
        if iswlower(wbuffer[i as usize] as wint_t) != 0 {
            have_lower = 1 as i32;
        }
        if iswupper(wbuffer[i as usize] as wint_t) != 0 {
            have_upper = 1 as i32;
        }
        if preserve_case == 0 {
            wbuffer[i as usize] = ch_towupper(wbuffer[i as usize]);
        }
        if !(wcschr(
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i32; 17],
            >(
                b"^\0\0\0+\0\0\0=\0\0\0/\0\0\0[\0\0\0]\0\0\0:\0\0\0,\0\0\0?\0\0\0*\0\0\0\\\0\0\0<\0\0\0>\0\0\0|\0\0\0\"\0\0\0.\0\0\0\0\0\0\0",
            ))
                .as_ptr(),
            wbuffer[i as usize],
        ))
            .is_null()
        {
            *mangled = 1 as i32;
            wbuffer[i as usize] = '~' as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    if have_lower != 0 && have_upper != 0 {
        *mangled = 1 as i32;
    }
    wchar_to_dos(cp, wbuffer.as_mut_ptr(), ((*ans).base).as_mut_ptr(), len, mangled);
}
#[no_mangle]
pub unsafe extern "C" fn label_name_uc(
    mut cp: *mut doscp_t,
    mut filename: *const i8,
    mut verbose: i32,
    mut mangled: *mut i32,
    mut ans: *mut dos_name_t,
) {
    mt_label_name(cp, filename, verbose, mangled, ans, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn label_name_pc(
    mut cp: *mut doscp_t,
    mut filename: *const i8,
    mut verbose: i32,
    mut mangled: *mut i32,
    mut ans: *mut dos_name_t,
) {
    mt_label_name(cp, filename, verbose, mangled, ans, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn labelit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut arg0: *mut libc::c_void,
    mut entry: *mut direntry_t,
) -> i32 {
    let mut now: time_t = 0;
    getTimeNow(&mut now);
    mk_entry(
        dosname,
        0x8 as i32 as u8,
        0 as i32 as u32,
        0 as i32 as uint32_t,
        now,
        &mut (*entry).dir,
    );
    return 0 as i32;
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
        b"Usage: %s [-vscVn] [-N serial] drive:\n\0" as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mlabel(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut newLabel: *const i8 = b"\0" as *const u8 as *const i8;
    let mut verbose: i32 = 0;
    let mut clear: i32 = 0;
    let mut interactive: i32 = 0;
    let mut show: i32 = 0;
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
    let mut result: i32 = 0 as i32;
    let mut longname: [i8; 261] = [0; 261];
    let mut shortname: [i8; 45] = [0; 45];
    let mut ch: ClashHandling_t = ClashHandling_t {
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
    };
    let mut RootDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut c: i32 = 0;
    let mut mangled: i32 = 0;
    let mut set_serial: C2RustUnnamed_11 = C2RustUnnamed_11::SER_NONE;
    let mut serial: uint32_t = 0 as i32 as uint32_t;
    let mut need_write_boot: i32 = 0 as i32;
    let mut have_boot: i32 = 0 as i32;
    let mut eptr: *mut i8 = 0 as *mut i8;
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut Fs: *mut Stream_t = 0 as *mut Stream_t;
    let mut r: i32 = 0;
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    let mut isRo: i32 = 0 as i32;
    let mut isRop: *mut i32 = 0 as *mut i32;
    let mut drive: i8 = 0;
    init_clash_handling(&mut ch);
    ch.name_converter = Some(
        label_name_uc
            as unsafe extern "C" fn(
                *mut doscp_t,
                *const i8,
                i32,
                *mut i32,
                *mut dos_name_t,
            ) -> (),
    );
    ch.ignore_entry = -(2 as i32);
    ch.is_label = 1 as i32;
    verbose = 0 as i32;
    clear = 0 as i32;
    show = 0 as i32;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:vcsnN:h\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            118 => {
                verbose = 1 as i32;
            }
            99 => {
                clear = 1 as i32;
            }
            115 => {
                show = 1 as i32;
            }
            110 => {
                set_serial = C2RustUnnamed_11::SER_RANDOM;
                init_random();
                serial = random() as uint32_t;
            }
            78 => {
                set_serial = C2RustUnnamed_11::SER_SET;
                *__errno_location() = 0 as i32;
                serial = strtou32(optarg, &mut eptr, 16 as i32);
                if *eptr != 0 {
                    fprintf(
                        stderr,
                        b"%s not a valid serial number\n\0" as *const u8 as *const i8,
                        optarg,
                    );
                    exit(1 as i32);
                }
                check_number_parse_errno(c as i8, optarg, eptr);
            }
            104 => {
                usage(0 as i32);
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    if argc - optind > 1 as i32 {
        usage(1 as i32);
    }
    if argc - optind == 1 as i32 {
        if *(*argv.offset(optind as isize)).offset(0 as i32 as isize) == 0
            || *(*argv.offset(optind as isize)).offset(1 as i32 as isize) as i32
                != ':' as i32
        {
            usage(1 as i32);
        }
        drive = ch_toupper(
            *(*argv.offset((argc - 1 as i32) as isize)).offset(0 as i32 as isize),
        );
        newLabel = (*argv.offset(optind as isize)).offset(2 as i32 as isize);
    } else {
        drive = get_default_drive();
    }
    if strlen(newLabel) > (20 as i32 * 13 as i32 + 1 as i32) as u64 {
        fprintf(stderr, b"Label too long\n\0" as *const u8 as *const i8);
        free_stream(&mut RootDir);
        exit(1 as i32);
    }
    interactive = (show == 0 && clear == 0 && *newLabel.offset(0 as i32 as isize) == 0
        && set_serial as u32 == C2RustUnnamed_11::SER_NONE as i32 as u32) as i32;
    if clear == 0 && *newLabel.offset(0 as i32 as isize) == 0 {
        isRop = &mut isRo;
    }
    if clear != 0 && *newLabel.offset(0 as i32 as isize) as i32 != 0 {
        fprintf(
            stderr,
            b"Both clear and new label specified\n\0" as *const u8 as *const i8,
        );
        free_stream(&mut RootDir);
        exit(1 as i32);
    }
    RootDir = open_root_dir(
        drive,
        if !isRop.is_null() { 0 as i32 } else { 0o2 as i32 },
        isRop,
    );
    if isRo != 0 {
        show = 1 as i32;
        interactive = 0 as i32;
    }
    if RootDir.is_null() {
        fprintf(
            stderr,
            b"%s: Cannot initialize drive\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
        );
        exit(1 as i32);
    }
    initializeDirentry(&mut entry, RootDir);
    r = vfat_lookup(
        &mut entry,
        0 as *const i8,
        0 as i32 as size_t,
        0x8 as i32 | 0x40 as i32,
        shortname.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 45]>() as u64,
        longname.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 261]>() as u64,
    );
    if r == -(2 as i32) {
        free_stream(&mut RootDir);
        exit(1 as i32);
    }
    if show != 0 || interactive != 0 {
        if isNotFound(&mut entry) != 0 {
            printf(b" Volume has no label\n\0" as *const u8 as *const i8);
        } else if *longname.as_mut_ptr() != 0 {
            printf(
                b" Volume label is %s (abbr=%s)\n\0" as *const u8 as *const i8,
                longname.as_mut_ptr(),
                shortname.as_mut_ptr(),
            );
        } else {
            printf(
                b" Volume label is %s\n\0" as *const u8 as *const i8,
                shortname.as_mut_ptr(),
            );
        }
    }
    if interactive != 0 {
        let mut ss: saved_sig_state = saved_sig_state {
            sa: [sigaction {
                __sigaction_handler: C2RustUnnamed_9 {
                    sa_handler: None,
                },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0,
                sa_restorer: None,
            }; 4],
        };
        newLabel = longname.as_mut_ptr();
        allow_interrupts(&mut ss);
        fprintf(stderr, b"Enter the new volume label : \0" as *const u8 as *const i8);
        if (fgets(longname.as_mut_ptr(), 20 as i32 * 13 as i32 + 1 as i32, stdin))
            .is_null()
        {
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            if *__errno_location() == 4 as i32 {
                free_stream(&mut RootDir);
                exit(1 as i32);
            }
            longname[0 as i32 as usize] = '\0' as i32 as i8;
        }
        if longname[0 as i32 as usize] != 0 {
            longname[(strlen(newLabel)).wrapping_sub(1 as i32 as u64) as usize] = '\0'
                as i32 as i8;
        }
    }
    if strlen(newLabel) > 11 as i32 as u64 {
        fprintf(stderr, b"New label too long\n\0" as *const u8 as *const i8);
        free_stream(&mut RootDir);
        exit(1 as i32);
    }
    if (show == 0 || *newLabel.offset(0 as i32 as isize) as i32 != 0)
        && isNotFound(&mut entry) == 0
    {
        if interactive != 0 && *newLabel.offset(0 as i32 as isize) as i32 == '\0' as i32
        {
            if ask_confirmation(
                b"Delete volume label (y/n): \0" as *const u8 as *const i8,
            ) != 0
            {
                free_stream(&mut RootDir);
                exit(0 as i32);
            }
        }
        entry.dir.attr = 0 as i32 as u8;
        wipeEntry(&mut entry);
    }
    if *newLabel.offset(0 as i32 as isize) as i32 != '\0' as i32 {
        ch.ignore_entry = 1 as i32;
        result = if mwrite_one(
            RootDir,
            newLabel,
            0 as *const i8,
            Some(
                labelit
                    as unsafe extern "C" fn(
                        *mut dos_name_t,
                        *mut i8,
                        *mut libc::c_void,
                        *mut direntry_t,
                    ) -> i32,
            ),
            0 as *mut libc::c_void,
            &mut ch,
        ) != 0
        {
            0 as i32
        } else {
            1 as i32
        };
    }
    have_boot = 0 as i32;
    if show == 0 || *newLabel.offset(0 as i32 as isize) as i32 != 0
        || set_serial as u32 != C2RustUnnamed_11::SER_NONE as i32 as u32
    {
        Fs = GetFs(RootDir);
        have_boot = (force_pread(
            Fs,
            (boot.characters).as_mut_ptr(),
            0 as i32 as mt_off_t,
            ::core::mem::size_of::<bootsector>() as u64,
        ) as u64 == ::core::mem::size_of::<bootsector>() as u64) as i32;
    }
    if (boot.boot.fatlen[0 as i32 as usize] as i32
        + ((boot.boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t != 0
    {
        labelBlock = &mut boot.boot.ext.old.labelBlock;
    } else {
        labelBlock = &mut boot.boot.ext.fat32.labelBlock;
    }
    if show == 0 || *newLabel.offset(0 as i32 as isize) as i32 != 0 {
        let mut dosname: dos_name_t = dos_name_t {
            base: [0; 8],
            ext: [0; 3],
            sentinel: 0,
        };
        let mut shrtLabel: *const i8 = 0 as *const i8;
        let mut cp: *mut doscp_t = 0 as *mut doscp_t;
        if *newLabel.offset(0 as i32 as isize) == 0 {
            shrtLabel = b"NO NAME    \0" as *const u8 as *const i8;
        } else {
            shrtLabel = newLabel;
        }
        cp = ((*(*Fs).Class).get_dosConvert).expect("non-null function pointer")(Fs);
        label_name_pc(cp, shrtLabel, verbose, &mut mangled, &mut dosname);
        if have_boot != 0 && boot.boot.descr as i32 >= 0xf0 as i32
            && ((*labelBlock).dos4 as i32 == 0x28 as i32
                || (*labelBlock).dos4 as i32 == 0x29 as i32)
        {
            strncpy(
                ((*labelBlock).label).as_mut_ptr(),
                (dosname.base).as_mut_ptr(),
                8 as i32 as u64,
            );
            strncpy(
                ((*labelBlock).label).as_mut_ptr().offset(8 as i32 as isize),
                (dosname.ext).as_mut_ptr(),
                3 as i32 as u64,
            );
            need_write_boot = 1 as i32;
        }
    }
    if (set_serial as u32 != C2RustUnnamed_11::SER_NONE as i32 as u32) as i32 & have_boot
        != 0
    {
        if have_boot != 0 && boot.boot.descr as i32 >= 0xf0 as i32
            && ((*labelBlock).dos4 as i32 == 0x28 as i32
                || (*labelBlock).dos4 as i32 == 0x29 as i32)
        {
            set_dword(((*labelBlock).serial).as_mut_ptr(), serial);
            need_write_boot = 1 as i32;
        }
    }
    if need_write_boot != 0 {
        force_pwrite(
            Fs,
            &mut boot as *mut bootsector as *mut i8,
            0 as i32 as mt_off_t,
            ::core::mem::size_of::<bootsector>() as u64,
        );
        if (boot.boot.fatlen[0 as i32 as usize] as i32
            + ((boot.boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            == 0
        {
            let mut backupBoot: i32 = (boot.boot.ext.fat32.backupBoot[0 as i32 as usize]
                as i32
                + ((boot.boot.ext.fat32.backupBoot[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t as i32;
            force_pwrite(
                Fs,
                &mut boot as *mut bootsector as *mut i8,
                (backupBoot
                    * (boot.boot.secsiz[0 as i32 as usize] as i32
                        + ((boot.boot.secsiz[1 as i32 as usize] as i32) << 8 as i32))
                        as uint16_t as i32) as mt_off_t,
                ::core::mem::size_of::<bootsector>() as u64,
            );
        }
    }
    free_stream(&mut RootDir);
    exit(result);
}
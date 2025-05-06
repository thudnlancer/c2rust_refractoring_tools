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
    pub type FatMap_t;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn set_cmd_line_image(img: *mut i8);
    fn check_number_parse_errno(c: i8, optarg_0: *const i8, endptr: *mut i8);
    fn strtoui(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u32;
    fn strtou32(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint32_t;
    static mut progname: *const i8;
    static mut mdate: *const i8;
    static mut mversion: *const i8;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    fn main_loop(MainParam: *mut MainParam_t, argv: *mut *mut i8, argc: i32) -> i32;
    fn dir_write(entry: *mut direntry_t);
    fn isRootEntry(entry: *mut direntry_t) -> i32;
    fn init_mp(MainParam: *mut MainParam_t);
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn getFs(Stream: *mut Stream_t) -> *mut Fs_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
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
pub struct Fs_t {
    pub head: Stream_t,
    pub serialized: i32,
    pub serial_number: u64,
    pub cluster_size: uint8_t,
    pub sector_size: uint16_t,
    pub fat_error: i32,
    pub fat_decode: Option<unsafe extern "C" fn(*mut Fs_t, u32) -> u32>,
    pub fat_encode: Option<unsafe extern "C" fn(*mut Fs_t, u32, u32) -> ()>,
    pub fat_dirty: i32,
    pub fat_start: uint16_t,
    pub fat_len: uint32_t,
    pub num_fat: uint8_t,
    pub end_fat: uint32_t,
    pub last_fat: uint32_t,
    pub fat_bits: u32,
    pub FatMap: *mut FatMap_t,
    pub dir_start: uint32_t,
    pub dir_len: uint16_t,
    pub clus_start: uint32_t,
    pub num_clus: uint32_t,
    pub drive: i8,
    pub primaryFat: uint32_t,
    pub writeAllFats: uint32_t,
    pub rootCluster: uint32_t,
    pub infoSectorLoc: uint32_t,
    pub backupBoot: uint16_t,
    pub last: uint32_t,
    pub freeSpace: uint32_t,
    pub preallocatedClusters: u32,
    pub lastFatSectorNr: uint32_t,
    pub lastFatSectorData: *mut u8,
    pub lastFatAccessMode: fatAccessMode_t,
    pub sectorMask: u32,
    pub sectorShift: u32,
    pub cp: *mut doscp_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum fatAccessMode_t {
    FAT_ACCESS_READ,
    FAT_ACCESS_WRITE,
}
impl fatAccessMode_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            fatAccessMode_t::FAT_ACCESS_READ => 0,
            fatAccessMode_t::FAT_ACCESS_WRITE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> fatAccessMode_t {
        match value {
            0 => fatAccessMode_t::FAT_ACCESS_READ,
            1 => fatAccessMode_t::FAT_ACCESS_WRITE,
            _ => panic!("Invalid value for fatAccessMode_t: {}", value),
        }
    }
}
impl AddAssign<u32> for fatAccessMode_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for fatAccessMode_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for fatAccessMode_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for fatAccessMode_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for fatAccessMode_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn add(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn sub(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn mul(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn div(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn rem(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_t {
    pub target: *mut i8,
    pub mp: MainParam_t,
    pub ch: ClashHandling_t,
    pub sourcefile: *mut Stream_t,
    pub fat: uint32_t,
    pub markbad: i32,
    pub setsize: i32,
    pub size: uint32_t,
    pub Fs: *mut Fs_t,
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
#[inline]
unsafe extern "C" fn set_dword(mut data: *mut u8, mut value: uint32_t) {
    *data.offset(3 as i32 as isize) = (value >> 24 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(2 as i32 as isize) = (value >> 16 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(1 as i32 as isize) = (value >> 8 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(0 as i32 as isize) = (value >> 0 as i32 & 0xff as i32 as u32) as u8;
}
#[inline]
unsafe extern "C" fn set_word(mut data: *mut u8, mut value: libc::c_ushort) {
    *data.offset(1 as i32 as isize) = (value as i32 >> 8 as i32 & 0xff as i32) as u8;
    *data.offset(0 as i32 as isize) = (value as i32 >> 0 as i32 & 0xff as i32) as u8;
}
unsafe extern "C" fn dos_doctorfat(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut Fs: *mut Fs_t = getFs((*mp).File);
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if (*arg).markbad == 0 && isRootEntry(entry) != 0 {
        set_word(
            ((*entry).dir.start).as_mut_ptr(),
            ((*arg).fat & 0xffff as i32 as u32) as libc::c_ushort,
        );
        set_word(
            ((*entry).dir.startHi).as_mut_ptr(),
            ((*arg).fat >> 16 as i32) as libc::c_ushort,
        );
        if (*arg).setsize != 0 {
            set_dword(((*entry).dir.size).as_mut_ptr(), (*arg).size);
        }
        dir_write(entry);
    }
    (*arg).Fs = Fs;
    return 4 as i32;
}
unsafe extern "C" fn unix_doctorfat(mut mp: *mut MainParam_t) -> i32 {
    fprintf(stderr, b"File does not reside on a Dos fs\n\0" as *const u8 as *const i8);
    return 16 as i32;
}
unsafe extern "C" fn usage(mut ret: i32) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
        mversion,
        mdate,
    );
    fprintf(stderr, b"Usage: [-b] %s file fat\n\0" as *const u8 as *const i8, progname);
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mdoctorfat(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut mtype: i32,
) {
    let mut arg: Arg_t = Arg_t {
        target: 0 as *mut i8,
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
        sourcefile: 0 as *mut Stream_t,
        fat: 0,
        markbad: 0,
        setsize: 0,
        size: 0,
        Fs: 0 as *mut Fs_t,
    };
    let mut c: i32 = 0;
    let mut ret: i32 = 0;
    let mut address: u32 = 0;
    let mut begin: u32 = 0;
    let mut end: u32 = 0;
    let mut number: *mut i8 = 0 as *mut i8;
    let mut eptr: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut offset: u32 = 0;
    init_clash_handling(&mut arg.ch);
    offset = 0 as i32 as u32;
    arg.markbad = 0 as i32;
    arg.setsize = 0 as i32;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:bo:s:h\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        let mut endptr: *mut i8 = 0 as *mut i8;
        *__errno_location() = 0 as i32;
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            98 => {
                arg.markbad = 1 as i32;
            }
            111 => {
                offset = strtoui(optarg, &mut endptr, 0 as i32);
            }
            115 => {
                arg.setsize = 1 as i32;
                arg.size = strtou32(optarg, &mut endptr, 0 as i32);
            }
            104 => {
                usage(0 as i32);
            }
            63 => {
                usage(1 as i32);
            }
            _ => {}
        }
        check_number_parse_errno(c as i8, optarg, endptr);
    }
    if argc - optind < 2 as i32 {
        usage(1 as i32);
    }
    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.callback = Some(
        dos_doctorfat as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    arg.mp.unixcallback = Some(
        unix_doctorfat as unsafe extern "C" fn(*mut MainParam_t) -> i32,
    );
    arg.mp.lookupflags = 0x20 as i32 | 0x10 as i32 | 1 as i32;
    arg.mp.openflags = 0o2 as i32;
    arg.fat = (strtoui(
        *argv.offset((optind + 1 as i32) as isize),
        0 as *mut *mut i8,
        0 as i32,
    ))
        .wrapping_add(offset);
    ret = main_loop(&mut arg.mp, argv.offset(optind as isize), 1 as i32);
    if ret != 0 {
        exit(ret);
    }
    address = 0 as i32 as u32;
    i = optind + 1 as i32;
    while i < argc {
        let mut j: u32 = 0;
        number = *argv.offset(i as isize);
        if *number as i32 == '<' as i32 {
            number = number.offset(1);
            number;
        }
        begin = strtoui(number, &mut eptr, 0 as i32);
        if !eptr.is_null() && *eptr as i32 == '-' as i32 {
            number = eptr.offset(1 as i32 as isize);
            end = strtoui(number, &mut eptr, 0 as i32);
        } else {
            end = begin;
        }
        if eptr == number {
            fprintf(stderr, b"Not a number: %s\n\0" as *const u8 as *const i8, number);
            exit(-(1 as i32));
        }
        if !eptr.is_null() && *eptr as i32 == '>' as i32 {
            eptr = eptr.offset(1);
            eptr;
        }
        if !eptr.is_null() && *eptr as i32 != 0 {
            fprintf(stderr, b"Not a number: %s\n\0" as *const u8 as *const i8, eptr);
            exit(-(1 as i32));
        }
        j = begin;
        while j <= end {
            if arg.markbad != 0 {
                ((*arg.Fs).fat_encode)
                    .expect(
                        "non-null function pointer",
                    )(
                    arg.Fs,
                    j.wrapping_add(offset),
                    (*arg.Fs).last_fat ^ 6 as i32 as u32 ^ 8 as i32 as u32,
                );
            } else {
                if address != 0 {
                    ((*arg.Fs).fat_encode)
                        .expect(
                            "non-null function pointer",
                        )(arg.Fs, address, j.wrapping_add(offset));
                }
                address = j.wrapping_add(offset);
            }
            j = j.wrapping_add(1);
            j;
        }
        i += 1;
        i;
    }
    if address != 0 && arg.markbad == 0 {
        ((*arg.Fs).fat_encode)
            .expect("non-null function pointer")(arg.Fs, address, (*arg.Fs).end_fat);
    }
    exit(ret);
}
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
    pub type doscp_t;
    pub type FatMap_t;
    fn random() -> i64;
    fn srandom(__seed: u32);
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn isatty(__fd: i32) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    fn label_name_uc(
        cp: *mut doscp_t,
        filename: *const i8,
        verbose: i32,
        mangled: *mut i32,
        ans: *mut dos_name_t,
    );
    fn label_name_pc(
        cp: *mut doscp_t,
        filename: *const i8,
        verbose: i32,
        mangled: *mut i32,
        ans: *mut dos_name_t,
    );
    static mut mtools_rate_0: uint8_t;
    static mut mtools_rate_any: uint8_t;
    fn get_default_drive() -> i8;
    fn set_cmd_line_image(img: *mut i8);
    fn check_number_parse_errno(c: i8, optarg_0: *const i8, endptr: *mut i8);
    fn parseSize(sizeStr: *mut i8) -> uint32_t;
    fn strtoui(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u32;
    fn atoui(nptr: *const i8) -> u32;
    fn atoul(nptr: *const i8) -> u64;
    fn strtou8(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint8_t;
    fn atou8(str: *const i8) -> uint8_t;
    fn strtou16(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint16_t;
    fn atou16(str: *const i8) -> uint16_t;
    fn strtou32(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint32_t;
    static mut progname: *const i8;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    fn expand(_: *const i8, _: *mut i8) -> *const i8;
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    static mut mformat_banner: *const i8;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    static mut devices: *mut device;
    fn chs_to_totsectors(dev: *mut device, errmsg: *mut i8) -> i32;
    fn check_if_sectors_fit(
        tot_sectors: uint32_t,
        maxBytes: mt_off_t,
        sectorSize: uint32_t,
        errmsg: *mut i8,
    ) -> i32;
    fn getOldDosBySize(size: size_t) -> *mut OldDos_t;
    fn getOldDosByParams(
        tracks: u32,
        heads: u32,
        sectors: u32,
        dir_len: u32,
        cluster_size: u32,
    ) -> *mut OldDos_t;
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn set_fat(This: *mut Fs_t, haveBigFatLen: bool);
    fn fatAllocate(This: *mut Fs_t, pos: u32, value: u32);
    fn fatEncode(This: *mut Fs_t, pos: u32, value: u32);
    static mut FsClass: Class_t;
    fn zero_fat(Fs: *mut Fs_t, media_descriptor: uint8_t) -> i32;
    fn calc_clus_start(Fs: *mut Fs_t) -> uint32_t;
    fn calc_num_clus(Fs: *mut Fs_t, tot_sectors: uint32_t) -> i32;
    fn labelit(
        dosname: *mut dos_name_t,
        longname: *mut i8,
        arg0: *mut libc::c_void,
        entry: *mut direntry_t,
    ) -> i32;
    fn OpenRoot(Dir: *mut Stream_t) -> *mut Stream_t;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const i8,
        shortname: *const i8,
        cb: Option<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> i32;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn buf_init(
        Next: *mut Stream_t,
        size: size_t,
        cylinderSize: size_t,
        sectorSize: size_t,
    ) -> *mut Stream_t;
    fn setBeginEnd(
        partTable: *mut partition,
        begin: uint32_t,
        end: uint32_t,
        iheads: uint16_t,
        isectors: uint16_t,
        activate: i32,
        type_0: uint8_t,
        fat_bits: u32,
    );
    fn OpenImage(
        out_dev: *mut device,
        dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        flags: i32,
        lockMode: i32,
        maxSize: *mut mt_off_t,
        geomFailureP: *mut i32,
        xdf_info: *mut xdf_info,
    ) -> *mut Stream_t;
    fn cp_open(codepage: u32) -> *mut doscp_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
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
pub struct dos_name_t {
    pub base: [i8; 8],
    pub ext: [i8; 3],
    pub sentinel: i8,
}
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
pub union C2RustUnnamed {
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
    pub ext: C2RustUnnamed,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: u32,
    pub RootDirSize: uint16_t,
    pub BadSectors: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partition {
    pub start: hsc,
    pub end: hsc,
    pub start_sect: [u8; 4],
    pub nr_sects: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsc {
    pub byte0: u8,
    pub head: u8,
    pub sector: u8,
    pub cyl: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OldDos_t {
    pub tracks: u32,
    pub sectors: uint16_t,
    pub heads: uint16_t,
    pub dir_len: uint16_t,
    pub cluster_size: uint8_t,
    pub fat_len: uint32_t,
    pub media: uint8_t,
}
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
unsafe extern "C" fn ptrdiff(mut end: *const i8, mut begin: *const i8) -> size_t {
    return end.offset_from(begin) as i64 as size_t;
}
#[inline]
unsafe extern "C" fn set_word(mut data: *mut u8, mut value: libc::c_ushort) {
    *data.offset(1 as i32 as isize) = (value as i32 >> 8 as i32 & 0xff as i32) as u8;
    *data.offset(0 as i32 as isize) = (value as i32 >> 0 as i32 & 0xff as i32) as u8;
}
#[inline]
unsafe extern "C" fn set_dword(mut data: *mut u8, mut value: uint32_t) {
    *data.offset(3 as i32 as isize) = (value >> 24 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(2 as i32 as isize) = (value >> 16 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(1 as i32 as isize) = (value >> 8 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(0 as i32 as isize) = (value >> 0 as i32 & 0xff as i32 as u32) as u8;
}
unsafe extern "C" fn init_geometry_boot(
    mut boot: *mut bootsector,
    mut dev: *mut device,
    mut sectors0: uint8_t,
    mut rate_0: uint8_t,
    mut rate_any: uint8_t,
    mut tot_sectors: *mut uint32_t,
    mut keepBoot: i32,
) -> uint16_t {
    let mut nb_renum: i32 = 0;
    let mut sector2: i32 = 0;
    let mut sum: i32 = 0;
    set_word(((*boot).boot.nsect).as_mut_ptr(), (*dev).sectors);
    set_word(((*boot).boot.nheads).as_mut_ptr(), (*dev).heads);
    if *tot_sectors != 0 as i32 as u32 {} else {
        __assert_fail(
            b"*tot_sectors != 0\0" as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            60 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 109],
                &[i8; 109],
            >(
                b"uint16_t init_geometry_boot(union bootsector *, struct device *, uint8_t, uint8_t, uint8_t, uint32_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8997: {
        if *tot_sectors != 0 as i32 as u32 {} else {
            __assert_fail(
                b"*tot_sectors != 0\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                60 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 109],
                    &[i8; 109],
                >(
                    b"uint16_t init_geometry_boot(union bootsector *, struct device *, uint8_t, uint8_t, uint8_t, uint32_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if *tot_sectors <= 65535 as i32 as u32 && (*dev).hidden <= 65535 as i32 as u32 {
        set_word(((*boot).boot.psect).as_mut_ptr(), *tot_sectors as uint16_t);
        set_dword(((*boot).boot.bigsect).as_mut_ptr(), 0 as i32 as uint32_t);
        set_word(((*boot).boot.nhs).as_mut_ptr(), (*dev).hidden as uint16_t);
    } else {
        set_word(((*boot).boot.psect).as_mut_ptr(), 0 as i32 as libc::c_ushort);
        set_dword(((*boot).boot.bigsect).as_mut_ptr(), *tot_sectors);
        set_dword(((*boot).boot.nhs).as_mut_ptr(), (*dev).hidden);
    }
    if (*dev).use_2m & 0x7f as i32 as u32 != 0 {
        let mut bootOffset: uint16_t = 0;
        let mut j: uint8_t = 0;
        let mut size2: uint8_t = 0;
        let mut i: uint16_t = 0;
        strncpy(
            ((*boot).boot.banner).as_mut_ptr(),
            b"2M-STV04\0" as *const u8 as *const i8,
            8 as i32 as u64,
        );
        (*boot).boot.ext.old.res_2m = 0 as i32 as u8;
        (*boot).boot.ext.old.fmt_2mf = 6 as i32 as u8;
        if (*dev).sectors as i32
            % (((1 as i32) << (*dev).ssize as i32) + 3 as i32 >> 2 as i32) != 0
        {
            (*boot).boot.ext.old.wt = 1 as i32 as u8;
        } else {
            (*boot).boot.ext.old.wt = 0 as i32 as u8;
        }
        (*boot).boot.ext.old.rate_0 = rate_0;
        (*boot).boot.ext.old.rate_any = rate_any;
        if (*boot).boot.ext.old.rate_any as i32 == 2 as i32 {
            (*boot).boot.ext.old.rate_any = 1 as i32 as u8;
        }
        i = 76 as i32 as uint16_t;
        set_word(((*boot).boot.ext.old.Infp0).as_mut_ptr(), i);
        let fresh0 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh0 as usize] = sectors0;
        let fresh1 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh1 as usize] = 108 as i32 as u8;
        j = 1 as i32 as uint8_t;
        while j as i32 <= sectors0 as i32 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh2 as usize] = j;
            j = j.wrapping_add(1);
            j;
        }
        set_word(((*boot).boot.ext.old.InfpX).as_mut_ptr(), i);
        let fresh3 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh3 as usize] = 64 as i32 as u8;
        let fresh4 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh4 as usize] = 3 as i32 as u8;
        let fresh5 = i;
        i = i.wrapping_add(1);
        nb_renum = fresh5 as i32;
        sector2 = (*dev).sectors as i32;
        size2 = (*dev).ssize;
        j = 1 as i32 as uint8_t;
        while sector2 != 0 {
            while sector2 < (1 as i32) << size2 as i32 >> 2 as i32 {
                size2 = size2.wrapping_sub(1);
                size2;
            }
            let fresh6 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh6 as usize] = (128 as i32 + j as i32) as u8;
            let fresh7 = j;
            j = j.wrapping_add(1);
            let fresh8 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh8 as usize] = fresh7;
            let fresh9 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh9 as usize] = size2;
            sector2 -= (1 as i32) << size2 as i32 >> 2 as i32;
        }
        (*boot).bytes[nb_renum as usize] = ((i as i32 - nb_renum - 1 as i32) / 3 as i32)
            as uint8_t;
        set_word(((*boot).boot.ext.old.InfTm).as_mut_ptr(), i);
        sector2 = (*dev).sectors as i32;
        size2 = (*dev).ssize;
        while sector2 != 0 {
            while sector2 < (1 as i32) << size2 as i32 - 2 as i32 {
                size2 = size2.wrapping_sub(1);
                size2;
            }
            let fresh10 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh10 as usize] = size2;
            sector2 -= (1 as i32) << size2 as i32 - 2 as i32;
        }
        set_word(((*boot).boot.ext.old.BootP).as_mut_ptr(), i);
        bootOffset = i;
        sum = 0 as i32;
        j = 64 as i32 as uint8_t;
        while (j as i32) < i as i32 {
            sum += (*boot).bytes[j as usize] as i32;
            j = j.wrapping_add(1);
            j;
        }
        (*boot).boot.ext.old.CheckSum = -sum as u8;
        return bootOffset;
    } else {
        if keepBoot == 0 {
            (*boot).boot.jump[0 as i32 as usize] = 0xeb as i32 as u8;
            (*boot).boot.jump[1 as i32 as usize] = 0 as i32 as u8;
            (*boot).boot.jump[2 as i32 as usize] = 0x90 as i32 as u8;
            strncpy(((*boot).boot.banner).as_mut_ptr(), mformat_banner, 8 as i32 as u64);
        }
        return 0 as i32 as uint16_t;
    };
}
static mut bootprog: [u8; 47] = [
    0xfa as i32 as u8,
    0x31 as i32 as u8,
    0xc0 as i32 as u8,
    0x8e as i32 as u8,
    0xd8 as i32 as u8,
    0x8e as i32 as u8,
    0xc0 as i32 as u8,
    0xfc as i32 as u8,
    0xb9 as i32 as u8,
    0 as i32 as u8,
    0x1 as i32 as u8,
    0xbe as i32 as u8,
    0 as i32 as u8,
    0x7c as i32 as u8,
    0xbf as i32 as u8,
    0 as i32 as u8,
    0x80 as i32 as u8,
    0xf3 as i32 as u8,
    0xa5 as i32 as u8,
    0xea as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x8 as i32 as u8,
    0xb8 as i32 as u8,
    0x1 as i32 as u8,
    0x2 as i32 as u8,
    0xbb as i32 as u8,
    0 as i32 as u8,
    0x7c as i32 as u8,
    0xba as i32 as u8,
    0x80 as i32 as u8,
    0 as i32 as u8,
    0xb9 as i32 as u8,
    0x1 as i32 as u8,
    0 as i32 as u8,
    0xcd as i32 as u8,
    0x13 as i32 as u8,
    0x72 as i32 as u8,
    0x5 as i32 as u8,
    0xea as i32 as u8,
    0 as i32 as u8,
    0x7c as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0xcd as i32 as u8,
    0x19 as i32 as u8,
];
#[inline]
unsafe extern "C" fn inst_boot_prg(mut boot: *mut bootsector, mut offset: uint16_t) {
    memcpy(
        ((*boot).bytes).as_mut_ptr().offset(offset as i32 as isize) as *mut libc::c_void,
        bootprog.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[u8; 47]>() as u64,
    );
    if (offset as i32 - 2 as i32) < 0x80 as i32 {
        (*boot).boot.jump[0 as i32 as usize] = 0xeb as i32 as u8;
        (*boot).boot.jump[1 as i32 as usize] = (offset as i32 - 2 as i32) as uint8_t;
        (*boot).boot.jump[2 as i32 as usize] = 0x90 as i32 as u8;
    } else {
        (*boot).boot.jump[0 as i32 as usize] = 0xe9 as i32 as u8;
        (*boot).boot.jump[1 as i32 as usize] = (offset as i32 - 3 as i32) as uint8_t;
        (*boot).boot.jump[2 as i32 as usize] = (offset as i32 - 3 as i32 >> 8 as i32)
            as uint8_t;
    }
    set_word(
        ((*boot).boot.jump)
            .as_mut_ptr()
            .offset(offset as i32 as isize)
            .offset(20 as i32 as isize),
        (offset as i32 + 24 as i32) as libc::c_ushort,
    );
}
#[inline]
unsafe extern "C" fn format_root(
    mut Fs: *mut Fs_t,
    mut label: *mut i8,
    mut boot: *mut bootsector,
) {
    let mut RootDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut i: u32 = 0;
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
    let mut dirlen: u32 = 0;
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
    buf = safe_malloc((*Fs).sector_size as size_t) as *mut i8;
    RootDir = OpenRoot(Fs as *mut Stream_t);
    if RootDir.is_null() {
        fprintf(stderr, b"Could not open root directory\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    memset(buf as *mut libc::c_void, '\0' as i32, (*Fs).sector_size as u64);
    if (*Fs).fat_bits == 32 as i32 as u32 {
        dirlen = (*Fs).cluster_size as u32;
        fatAllocate(Fs, (*Fs).rootCluster, (*Fs).end_fat);
    } else {
        dirlen = (*Fs).dir_len as u32;
    }
    i = 0 as i32 as u32;
    while i < dirlen {
        ((*(*RootDir).Class).pwrite)
            .expect(
                "non-null function pointer",
            )(RootDir, buf, sectorsToBytes(Fs, i), (*Fs).sector_size as size_t);
        i = i.wrapping_add(1);
        i;
    }
    ch.ignore_entry = 1 as i32;
    if *label.offset(0 as i32 as isize) != 0 {
        mwrite_one(
            RootDir,
            label,
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
        );
    }
    free_stream(&mut RootDir);
    if (*Fs).fat_bits == 32 as i32 as u32 {
        set_word(((*boot).boot.dirents).as_mut_ptr(), 0 as i32 as libc::c_ushort);
    } else {
        set_word(
            ((*boot).boot.dirents).as_mut_ptr(),
            ((*Fs).dir_len as i32 * ((*Fs).sector_size as i32 / 32 as i32)) as uint16_t,
        );
    }
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn calc_fat_len(mut Fs: *mut Fs_t, mut tot_sectors: uint32_t) -> i32 {
    let mut rem_sect: uint32_t = 0;
    let mut numerator: uint32_t = 0;
    let mut denominator: uint32_t = 0;
    let mut corr: uint32_t = 0 as i32 as uint32_t;
    let mut clus_start: uint32_t = 0;
    let mut fat_nybbles: u32 = 0;
    if (*Fs).fat_bits != 0 as i32 as u32 {} else {
        __assert_fail(
            b"Fs->fat_bits != 0\0" as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            239 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[i8; 35],
            >(b"int calc_fat_len(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_11372: {
        if (*Fs).fat_bits != 0 as i32 as u32 {} else {
            __assert_fail(
                b"Fs->fat_bits != 0\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                239 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[i8; 35],
                >(b"int calc_fat_len(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    (*Fs).fat_len = 0 as i32 as uint32_t;
    clus_start = calc_clus_start(Fs);
    if tot_sectors < clus_start {
        return -(2 as i32);
    }
    rem_sect = tot_sectors.wrapping_sub(clus_start);
    if rem_sect.wrapping_rem(2 as i32 as u32) == 1 as i32 as u32
        && (*Fs).num_fat as i32 % 2 as i32 == 0 as i32
        && (*Fs).cluster_size as i32 % 2 as i32 == 0 as i32
    {
        rem_sect = rem_sect.wrapping_sub(1);
        rem_sect;
    }
    fat_nybbles = ((*Fs).fat_bits).wrapping_div(4 as i32 as u32);
    numerator = rem_sect.wrapping_add((2 as i32 * (*Fs).cluster_size as i32) as u32);
    denominator = (((*Fs).cluster_size as i32 * (*Fs).sector_size as i32 * 2 as i32)
        as u32)
        .wrapping_add(((*Fs).num_fat as u32).wrapping_mul(fat_nybbles));
    if fat_nybbles == 3 as i32 as u32 {
        if rem_sect > (256 as i32 * 0xff5 as i32) as u32 {
            return 1 as i32;
        }
        numerator = (numerator as u32).wrapping_mul(fat_nybbles) as uint32_t as uint32_t;
    } else {
        denominator = denominator.wrapping_div(fat_nybbles);
    }
    if rem_sect > denominator {
        numerator = (numerator as u32).wrapping_sub(denominator) as uint32_t as uint32_t;
        corr = corr.wrapping_add(1);
        corr;
    }
    (*Fs).fat_len = numerator
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div(denominator)
        .wrapping_add(1 as i32 as u32)
        .wrapping_add(corr);
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn clusters_fit_into_fat(mut Fs: *mut Fs_t) -> bool {
    return ((*Fs).num_clus)
        .wrapping_add(2 as i32 as u32)
        .wrapping_mul(((*Fs).fat_bits).wrapping_div(4 as i32 as u32))
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div(((*Fs).sector_size as i32 * 2 as i32) as u32) < (*Fs).fat_len;
}
unsafe extern "C" fn check_fs_params_and_set_fat(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
) {
    let mut provisional_fat_bits: u32 = 0;
    if (if (*Fs).fat_bits == 32 as i32 as u32 {
        ((*Fs).dir_len as i32 == 0 as i32) as i32
    } else {
        ((*Fs).dir_len as i32 != 0 as i32) as i32
    }) != 0
    {} else {
        __assert_fail(
            b"Fs->fat_bits == 32 ? (Fs->dir_len == 0) : (Fs->dir_len != 0)\0"
                as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            341 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10191: {
        if (if (*Fs).fat_bits == 32 as i32 as u32 {
            ((*Fs).dir_len as i32 == 0 as i32) as i32
        } else {
            ((*Fs).dir_len as i32 != 0 as i32) as i32
        }) != 0
        {} else {
            __assert_fail(
                b"Fs->fat_bits == 32 ? (Fs->dir_len == 0) : (Fs->dir_len != 0)\0"
                    as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                341 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if tot_sectors
        >= ((*Fs).clus_start)
            .wrapping_add(((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as u32))
    {} else {
        __assert_fail(
            b"tot_sectors >= Fs->clus_start + Fs->num_clus * Fs->cluster_size\0"
                as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            346 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10126: {
        if tot_sectors
            >= ((*Fs).clus_start)
                .wrapping_add(((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as u32))
        {} else {
            __assert_fail(
                b"tot_sectors >= Fs->clus_start + Fs->num_clus * Fs->cluster_size\0"
                    as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                346 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if tot_sectors
        <= ((*Fs).clus_start)
            .wrapping_add(((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as u32))
            .wrapping_add((*Fs).cluster_size as u32)
            .wrapping_sub(1 as i32 as u32)
    {} else {
        __assert_fail(
            b"tot_sectors <= Fs->clus_start + Fs->num_clus * Fs->cluster_size + Fs->cluster_size - 1\0"
                as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            349 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10043: {
        if tot_sectors
            <= ((*Fs).clus_start)
                .wrapping_add(((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as u32))
                .wrapping_add((*Fs).cluster_size as u32)
                .wrapping_sub(1 as i32 as u32)
        {} else {
            __assert_fail(
                b"tot_sectors <= Fs->clus_start + Fs->num_clus * Fs->cluster_size + Fs->cluster_size - 1\0"
                    as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                349 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if clusters_fit_into_fat(Fs) {} else {
        __assert_fail(
            b"clusters_fit_into_fat(Fs)\0" as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            352 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_9960: {
        if clusters_fit_into_fat(Fs) {} else {
            __assert_fail(
                b"clusters_fit_into_fat(Fs)\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                352 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    provisional_fat_bits = (*Fs).fat_bits;
    set_fat(Fs, provisional_fat_bits == 32 as i32 as u32);
    if provisional_fat_bits == (*Fs).fat_bits {} else {
        __assert_fail(
            b"provisional_fat_bits == Fs->fat_bits\0" as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            357 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_9894: {
        if provisional_fat_bits == (*Fs).fat_bits {} else {
            __assert_fail(
                b"provisional_fat_bits == Fs->fat_bits\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                357 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn fat32_specific_init(mut Fs: *mut Fs_t) {
    (*Fs).primaryFat = 0 as i32 as uint32_t;
    (*Fs).writeAllFats = 1 as i32 as uint32_t;
    if (*Fs).backupBoot == 0 {
        if (*Fs).fat_start as i32 <= 6 as i32 {
            (*Fs).backupBoot = ((*Fs).fat_start as i32 - 1 as i32) as uint16_t;
        } else {
            (*Fs).backupBoot = 6 as i32 as uint16_t;
        }
    }
    if ((*Fs).fat_start as i32) < 3 as i32 {
        fprintf(
            stderr,
            b"For FAT 32, reserved sectors need to be at least 3\n\0" as *const u8
                as *const i8,
        );
        exit(1 as i32);
    }
    if (*Fs).fat_start as i32 <= (*Fs).backupBoot as i32 {
        fprintf(
            stderr,
            b"Reserved sectors (%d) must be more than backupBoot (%d)\n\0" as *const u8
                as *const i8,
            (*Fs).fat_start as i32,
            (*Fs).backupBoot as i32,
        );
        (*Fs).backupBoot = 0 as i32 as uint16_t;
    }
}
unsafe extern "C" fn try_cluster_size(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
    mut may_change_boot_size: bool,
    mut may_change_fat_len: bool,
    mut may_change_root_size: bool,
    mut may_pad: bool,
) -> i32 {
    let mut maxClus: uint32_t = 0;
    let mut minClus: uint32_t = 0;
    match (*Fs).fat_bits {
        12 => {
            minClus = 1 as i32 as uint32_t;
            maxClus = 0xff5 as i32 as uint32_t;
        }
        16 => {
            minClus = 4096 as i32 as uint32_t;
            maxClus = 0xfff5 as i32 as uint32_t;
        }
        32 => {
            minClus = 0xfff5 as i32 as uint32_t;
            maxClus = 0xffffff5 as i32 as uint32_t;
        }
        _ => {
            if 0 as i32 != 0
                && !(b"Bad number of FAT bits\0" as *const u8 as *const i8).is_null()
            {} else {
                __assert_fail(
                    b"false && \"Bad number of FAT bits\"\0" as *const u8 as *const i8,
                    b"mformat.c\0" as *const u8 as *const i8,
                    438 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 67],
                        &[i8; 67],
                    >(
                        b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_11467: {
                if 0 as i32 != 0
                    && !(b"Bad number of FAT bits\0" as *const u8 as *const i8).is_null()
                {} else {
                    __assert_fail(
                        b"false && \"Bad number of FAT bits\"\0" as *const u8
                            as *const i8,
                        b"mformat.c\0" as *const u8 as *const i8,
                        438 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 67],
                            &[i8; 67],
                        >(
                            b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return -(2 as i32);
        }
    }
    if !(getenv(b"MTOOLS_DEBUG_FAT\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"FAT=%d Cluster=%d%s\n\0" as *const u8 as *const i8,
            (*Fs).fat_bits,
            (*Fs).cluster_size as i32,
            if may_pad as i32 != 0 {
                b" may_pad\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
    }
    if may_change_fat_len {
        let mut fit: i32 = calc_fat_len(Fs, tot_sectors);
        if fit != 0 as i32 {
            return fit;
        }
    }
    loop {
        let mut bwaste: uint32_t = 0;
        let mut waste: uint16_t = 0;
        let mut dir_grow: uint16_t = 0 as i32 as uint16_t;
        if calc_num_clus(Fs, tot_sectors) < 0 as i32 {
            return -(2 as i32);
        }
        if (*Fs).num_clus < minClus {
            return -(1 as i32);
        }
        if !may_change_fat_len {
            if (*Fs).num_clus >= 0xffffff5 as i32 as u32 || !clusters_fit_into_fat(Fs) {
                return 2 as i32;
            }
        }
        if (*Fs).num_clus < maxClus {
            break;
        }
        if !may_pad {
            return 1 as i32;
        }
        bwaste = tot_sectors
            .wrapping_sub((*Fs).clus_start)
            .wrapping_sub(maxClus.wrapping_mul((*Fs).cluster_size as u32))
            .wrapping_add(1 as i32 as u32);
        if bwaste <= 65535 as i32 as u32 {} else {
            __assert_fail(
                b"bwaste <= UINT16_MAX\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                510 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[i8; 67],
                >(
                    b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_11027: {
            if bwaste <= 65535 as i32 as u32 {} else {
                __assert_fail(
                    b"bwaste <= UINT16_MAX\0" as *const u8 as *const i8,
                    b"mformat.c\0" as *const u8 as *const i8,
                    510 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 67],
                        &[i8; 67],
                    >(
                        b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        waste = bwaste as uint16_t;
        if may_change_root_size {
            dir_grow = (32 as i32 - (*Fs).dir_len as i32) as uint16_t;
            if dir_grow as i32 > waste as i32 {
                dir_grow = waste;
            }
            waste = (waste as i32 - dir_grow as i32) as uint16_t;
        }
        if may_change_fat_len as i32 != 0
            && (!may_change_boot_size || (*Fs).fat_bits == 12 as i32 as u32)
        {
            let mut fat_grow: uint16_t = ((waste as i32 + (*Fs).num_fat as i32
                - 1 as i32) / (*Fs).num_fat as i32) as uint16_t;
            let mut dir_shrink: uint16_t = 0 as i32 as uint16_t;
            (*Fs).fat_len = ((*Fs).fat_len as u32).wrapping_add(fat_grow as u32)
                as uint32_t as uint32_t;
            dir_shrink = (waste as i32 - fat_grow as i32 * (*Fs).num_fat as i32)
                as uint16_t;
            if dir_shrink as i32 > dir_grow as i32 {
                dir_shrink = dir_grow;
            }
            dir_grow = (dir_grow as i32 - dir_shrink as i32) as uint16_t;
        } else if may_change_boot_size {
            (*Fs).fat_start = ((*Fs).fat_start as i32 + waste as i32) as uint16_t;
        }
        (*Fs).dir_len = ((*Fs).dir_len as i32 + dir_grow as i32) as uint16_t;
        may_pad = 0 as i32 != 0;
    }
    if (*Fs).num_clus >= minClus {} else {
        __assert_fail(
            b"Fs->num_clus >= minClus\0" as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            544 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[i8; 67],
            >(b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_10814: {
        if (*Fs).num_clus >= minClus {} else {
            __assert_fail(
                b"Fs->num_clus >= minClus\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                544 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[i8; 67],
                >(
                    b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*Fs).num_clus < maxClus {} else {
        __assert_fail(
            b"Fs->num_clus < maxClus\0" as *const u8 as *const i8,
            b"mformat.c\0" as *const u8 as *const i8,
            545 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[i8; 67],
            >(b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_10769: {
        if (*Fs).num_clus < maxClus {} else {
            __assert_fail(
                b"Fs->num_clus < maxClus\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                545 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 67],
                    &[i8; 67],
                >(
                    b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn calc_fs_parameters(
    mut dev: *mut device,
    mut fat32Requested: bool,
    mut tot_sectors: uint32_t,
    mut Fs: *mut Fs_t,
    mut descr: *mut uint8_t,
) -> i32 {
    let mut may_change_boot_size: bool = (*Fs).fat_start as i32 == 0 as i32;
    let mut may_change_fat_bits: bool = (*dev).fat_bits == 0 as i32 && !fat32Requested;
    let mut may_change_cluster_size: bool = (*Fs).cluster_size as i32 == 0 as i32;
    let mut may_change_root_size: bool = (*Fs).dir_len as i32 == 0 as i32;
    let mut may_change_fat_len: bool = (*Fs).fat_len == 0 as i32 as u32;
    let mut may_pad: bool = 0 as i32 != 0;
    let mut saved_dir_len: uint16_t = 0;
    let mut params: *mut OldDos_t = 0 as *mut OldDos_t;
    if fat32Requested {
        if (*dev).fat_bits != 0 && (*dev).fat_bits != 32 as i32 {
            fprintf(
                stderr,
                b"Fat bits 32 requested on command line, but %d in device description\n\0"
                    as *const u8 as *const i8,
                (*dev).fat_bits,
            );
            exit(1 as i32);
        }
        (*dev).fat_bits = 32 as i32;
    }
    (*Fs).infoSectorLoc = 0 as i32 as uint32_t;
    if (may_change_fat_bits as i32 != 0
        || (if (*dev).fat_bits > 0 as i32 { (*dev).fat_bits } else { -(*dev).fat_bits })
            as u32 == 12 as i32 as u32)
        && (may_change_boot_size as i32 != 0 || (*Fs).fat_start as i32 == 1 as i32)
    {
        params = getOldDosByParams(
            (*dev).tracks,
            (*dev).heads as u32,
            (*dev).sectors as u32,
            (*Fs).dir_len as u32,
            (*Fs).cluster_size as u32,
        );
    }
    if !params.is_null() {
        let mut num_clus_valid: i32 = 0;
        *descr = (*params).media;
        (*Fs).fat_start = 1 as i32 as uint16_t;
        (*Fs).cluster_size = (*params).cluster_size;
        (*Fs).dir_len = (*params).dir_len;
        (*Fs).fat_len = (*params).fat_len;
        (*Fs).fat_bits = 12 as i32 as u32;
        num_clus_valid = calc_num_clus(Fs, tot_sectors);
        if num_clus_valid >= 0 as i32 {} else {
            __assert_fail(
                b"num_clus_valid >= 0\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                597 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_11885: {
            if num_clus_valid >= 0 as i32 {} else {
                __assert_fail(
                    b"num_clus_valid >= 0\0" as *const u8 as *const i8,
                    b"mformat.c\0" as *const u8 as *const i8,
                    597 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[i8; 83],
                    >(
                        b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        check_fs_params_and_set_fat(Fs, tot_sectors);
        return 0 as i32;
    }
    if (*dev).hidden != 0
        || tot_sectors.wrapping_rem(((*dev).sectors as i32 * (*dev).heads as i32) as u32)
            != 0
    {
        *descr = 0xf8 as i32 as uint8_t;
    } else {
        *descr = 0xf0 as i32 as uint8_t;
    }
    (*Fs).fat_bits = (if (*dev).fat_bits > 0 as i32 {
        (*dev).fat_bits
    } else {
        -(*dev).fat_bits
    }) as u32;
    if (*Fs).fat_bits == 0 as i32 as u32 {
        (*Fs).fat_bits = 12 as i32 as u32;
    }
    if (*Fs).cluster_size == 0 {
        if tot_sectors < 2400 as i32 as u32 && (*dev).heads as i32 == 2 as i32 {
            (*Fs).cluster_size = 2 as i32 as uint8_t;
        } else if may_change_fat_len as i32 != 0 && (*Fs).fat_bits == 32 as i32 as u32 {
            (*Fs).cluster_size = 8 as i32 as uint8_t;
        } else {
            (*Fs).cluster_size = 1 as i32 as uint8_t;
        }
    }
    if (*Fs).dir_len == 0 {
        if tot_sectors < 1200 as i32 as u32 {
            if (*dev).heads as i32 == 1 as i32 {
                (*Fs).dir_len = 4 as i32 as uint16_t;
            } else {
                (*Fs).dir_len = 7 as i32 as uint16_t;
            }
        } else if tot_sectors <= 3840 as i32 as u32 {
            (*Fs).dir_len = 14 as i32 as uint16_t;
        } else if tot_sectors <= 7680 as i32 as u32 {
            (*Fs).dir_len = 15 as i32 as uint16_t;
        } else {
            (*Fs).dir_len = 32 as i32 as uint16_t;
        }
    }
    saved_dir_len = (*Fs).dir_len;
    loop {
        let mut fit: i32 = 0;
        if may_change_boot_size {
            if (*Fs).fat_bits == 32 as i32 as u32 {
                (*Fs).fat_start = 32 as i32 as uint16_t;
            } else {
                (*Fs).fat_start = 1 as i32 as uint16_t;
            }
        }
        if (*Fs).fat_bits == 32 as i32 as u32 {
            (*Fs).dir_len = 0 as i32 as uint16_t;
        } else if (*Fs).dir_len as i32 == 0 as i32 {
            (*Fs).dir_len = saved_dir_len;
        }
        if (*Fs).fat_bits == 32 as i32 as u32 && may_change_cluster_size as i32 != 0
            && may_change_fat_len as i32 != 0
        {
            (*Fs).cluster_size = (if tot_sectors
                >= (32 as i32 * 1024 as i32 * 1024 as i32 * 2 as i32) as u32
            {
                64 as i32
            } else if tot_sectors
                >= (16 as i32 * 1024 as i32 * 1024 as i32 * 2 as i32) as u32
            {
                32 as i32
            } else if tot_sectors
                >= (8 as i32 * 1024 as i32 * 1024 as i32 * 2 as i32) as u32
            {
                16 as i32
            } else {
                (*Fs).cluster_size as i32
            }) as uint8_t;
        }
        fit = try_cluster_size(
            Fs,
            tot_sectors,
            may_change_boot_size,
            may_change_fat_len,
            may_change_root_size,
            may_pad,
        );
        if !(getenv(b"MTOOLS_DEBUG_FAT\0" as *const u8 as *const i8)).is_null() {
            fprintf(stderr, b" fit=%d\n\0" as *const u8 as *const i8, fit);
        }
        if fit == 0 as i32 {
            break;
        }
        if fit == -(2 as i32) {
            return -(1 as i32);
        }
        if fit != 2 as i32 || !may_change_fat_len {} else {
            __assert_fail(
                b"fit != 2 || !may_change_fat_len\0" as *const u8 as *const i8,
                b"mformat.c\0" as *const u8 as *const i8,
                695 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_10647: {
            if fit != 2 as i32 || !may_change_fat_len {} else {
                __assert_fail(
                    b"fit != 2 || !may_change_fat_len\0" as *const u8 as *const i8,
                    b"mformat.c\0" as *const u8 as *const i8,
                    695 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[i8; 83],
                    >(
                        b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if fit < 0 as i32 {
            if may_change_cluster_size as i32 != 0 && may_change_fat_len as i32 != 0
                && (*Fs).cluster_size as i32 > 1 as i32
            {
                (*Fs).cluster_size = ((*Fs).cluster_size as i32 / 2 as i32) as uint8_t;
            } else {
                if fat32Requested {
                    break;
                }
                if !may_change_fat_bits && (*Fs).fat_bits == 32 as i32 as u32 {
                    fat32Requested = 1 as i32 != 0;
                    break;
                } else {
                    if !may_change_fat_bits || (*Fs).fat_bits == 12 as i32 as u32 {
                        return -(2 as i32);
                    }
                    match (*Fs).fat_bits {
                        16 => {
                            (*Fs).fat_bits = 12 as i32 as u32;
                        }
                        32 => {
                            (*Fs).fat_bits = 16 as i32 as u32;
                        }
                        _ => {}
                    }
                    may_pad = 1 as i32 != 0;
                }
            }
        } else {
            if fit == 1 as i32 && may_change_fat_bits as i32 != 0 && !may_pad {
                if (*Fs).fat_bits == 12 as i32 as u32
                    && (!may_change_cluster_size
                        || (*Fs).cluster_size as i32 >= 8 as i32)
                {
                    (*Fs).fat_bits = 16 as i32 as u32;
                    if may_change_cluster_size {
                        (*Fs).cluster_size = 1 as i32 as uint8_t;
                    }
                    continue;
                } else if (*Fs).fat_bits == 16 as i32 as u32
                    && (!may_change_cluster_size
                        || (*Fs).cluster_size as i32 >= 64 as i32)
                {
                    (*Fs).fat_bits = 32 as i32 as u32;
                    if may_change_cluster_size {
                        (*Fs).cluster_size = (if may_change_fat_len as i32 != 0 {
                            8 as i32
                        } else {
                            1 as i32
                        }) as uint8_t;
                    }
                    continue;
                }
            }
            if may_change_cluster_size as i32 != 0
                && ((*Fs).cluster_size as i32) < 128 as i32
            {
                (*Fs).cluster_size = (2 as i32 * (*Fs).cluster_size as i32) as uint8_t;
            } else if fit == 2 as i32 && may_change_fat_bits as i32 != 0
                && may_change_root_size as i32 != 0 && (*Fs).fat_bits == 16 as i32 as u32
            {
                (*Fs).fat_bits = 12 as i32 as u32;
                may_pad = 1 as i32 != 0;
            } else {
                return if fit == 2 as i32 { -(4 as i32) } else { -(3 as i32) }
            }
        }
    }
    if !(getenv(b"MTOOLS_DEBUG_FAT\0" as *const u8 as *const i8)).is_null()
        || !(getenv(b"MTOOLS_DEBUG_FAT_SUMMARY\0" as *const u8 as *const i8)).is_null()
    {
        fprintf(
            stderr,
            b" FAT%d Cluster_size=%d %d clusters FAT_LEN=%d\n\0" as *const u8
                as *const i8,
            (*Fs).fat_bits,
            (*Fs).cluster_size as i32,
            (*Fs).num_clus,
            (*Fs).fat_len,
        );
    }
    check_fs_params_and_set_fat(Fs, tot_sectors);
    if (*Fs).fat_bits == 32 as i32 as u32 {
        fat32_specific_init(Fs);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn initFsForFormat(mut Fs: *mut Fs_t) {
    memset(Fs as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<Fs_t>() as u64);
    init_head(&mut (*Fs).head, &mut FsClass, 0 as *mut Stream_t);
    (*Fs).cluster_size = 0 as i32 as uint8_t;
    (*Fs).dir_len = 0 as i32 as uint16_t;
    (*Fs).fat_len = 0 as i32 as uint32_t;
    (*Fs).num_fat = 2 as i32 as uint8_t;
    (*Fs).backupBoot = 0 as i32 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn setFsSectorSize(
    mut Fs: *mut Fs_t,
    mut dev: *mut device,
    mut msize: uint16_t,
) {
    let mut j: u32 = 0;
    (*Fs).sector_size = 512 as i32 as uint16_t;
    if (*dev).use_2m & 0x7f as i32 as u32 == 0 {
        (*Fs).sector_size = ((128 as u32) << ((*dev).ssize as i32 & 0x7f as i32))
            as uint16_t;
    }
    if msize != 0 {
        (*Fs).sector_size = msize;
    }
    j = 0 as i32 as u32;
    while j < 31 as i32 as u32 {
        if (*Fs).sector_size as u32 == ((1 as i32) << j) as u32 {
            (*Fs).sectorShift = j;
            break;
        } else {
            j = j.wrapping_add(1);
            j;
        }
    }
    (*Fs).sectorMask = ((*Fs).sector_size as i32 - 1 as i32) as u32;
}
unsafe extern "C" fn old_dos_size_to_geom(
    mut size: size_t,
    mut cyls: *mut u32,
    mut heads: *mut libc::c_ushort,
    mut sects: *mut libc::c_ushort,
) -> i32 {
    let mut params: *mut OldDos_t = getOldDosBySize(size);
    if !params.is_null() {
        *cyls = (*params).tracks;
        *heads = (*params).heads;
        *sects = (*params).sectors;
        return 0 as i32;
    } else {
        return 1 as i32
    };
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
        b"Usage: %s [-V] [-t tracks] [-h heads] [-n sectors] [-v label] [-1] [-4] [-8] [-f size] [-N serialnumber] [-k] [-B bootsector] [-r root_dir_len] [-L fat_len] [-F] [-I fsVersion] [-C] [-c cluster_size] [-H hidden_sectors] [-X] [-S hardsectorsize] [-M softsectorsize] [-3] [-2 track0sectors] [-0 rate0] [-A rateany] [-a]device\n\0"
            as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mformat(mut argc: i32, mut argv: *mut *mut i8, mut dummy: i32) {
    let mut r: i32 = 0;
    let mut Fs: *mut Fs_t = 0 as *mut Fs_t;
    let mut hs: u32 = 0;
    let mut hs_set: i32 = 0;
    let mut arguse_2m: u32 = 0 as i32 as u32;
    let mut sectors0: uint8_t = 18 as i32 as uint8_t;
    let mut create: i32 = 0 as i32;
    let mut rate_0: uint8_t = 0;
    let mut rate_any: uint8_t = 0;
    let mut mangled: i32 = 0;
    let mut argssize: uint8_t = 0 as i32 as uint8_t;
    let mut msize: uint16_t = 0 as i32 as uint16_t;
    let mut fat32: i32 = 0 as i32;
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    let mut bootOffset: size_t = 0;
    let mut i: u32 = 0;
    let mut format_xdf: i32 = 0 as i32;
    let mut info: xdf_info = xdf_info {
        FatSize: 0,
        RootDirSize: 0,
        BadSectors: 0,
    };
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut bootSector: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    let mut keepBoot: i32 = 0 as i32;
    let mut used_dev: device = device {
        name: 0 as *const i8,
        drive: 0,
        fat_bits: 0,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0,
        ssize: 0,
        use_2m: 0,
        precmd: 0 as *mut i8,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: 0 as *const i8,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: 0 as *mut i8,
        cfg_filename: 0 as *const i8,
    };
    let mut argtracks: u32 = 0;
    let mut argheads: uint16_t = 0;
    let mut argsectors: uint16_t = 0;
    let mut tot_sectors: uint32_t = 0 as i32 as uint32_t;
    let mut blocksize: uint32_t = 0;
    let mut drive: i8 = 0;
    let mut name: [i8; 2048] = [0; 2048];
    let mut label: [i8; 261] = [0; 261];
    let mut shortlabel: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    let mut dev: *mut device = 0 as *mut device;
    let mut errmsg: [i8; 2100] = [0; 2100];
    let mut serial: uint32_t = 0;
    let mut serial_set: i32 = 0;
    let mut fsVersion: uint16_t = 0;
    let mut mediaDesc: uint8_t = 0 as i32 as uint8_t;
    let mut haveMediaDesc: bool = 0 as i32 != 0;
    let mut biosDisk: uint8_t = 0 as i32 as uint8_t;
    let mut haveBiosDisk: bool = 0 as i32 != 0;
    let mut maxSize: mt_off_t = 0;
    let mut Atari: i32 = 0 as i32;
    let mut endptr: *mut i8 = 0 as *mut i8;
    hs_set = 0 as i32;
    hs = hs_set as u32;
    argtracks = 0 as i32 as u32;
    argheads = 0 as i32 as uint16_t;
    argsectors = 0 as i32 as uint16_t;
    arguse_2m = 0 as i32 as u32;
    argssize = 0x2 as i32 as uint8_t;
    label[0 as i32 as usize] = '\0' as i32 as i8;
    serial_set = 0 as i32;
    serial = 0 as i32 as uint32_t;
    fsVersion = 0 as i32 as uint16_t;
    Fs = calloc(1 as i32 as u64, ::core::mem::size_of::<Fs_t>() as u64) as *mut Fs_t;
    if Fs.is_null() {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    initFsForFormat(Fs);
    if !(getenv(b"MTOOLS_DIR_LEN\0" as *const u8 as *const i8)).is_null() {
        (*Fs).dir_len = atou16(getenv(b"MTOOLS_DIR_LEN\0" as *const u8 as *const i8));
        if (*Fs).dir_len as i32 <= 0 as i32 {
            (*Fs).dir_len = 0 as i32 as uint16_t;
        }
    }
    if !(getenv(b"MTOOLS_NFATS\0" as *const u8 as *const i8)).is_null() {
        (*Fs).num_fat = atou8(getenv(b"MTOOLS_NFATS\0" as *const u8 as *const i8));
        if (*Fs).num_fat as i32 <= 0 as i32 {
            (*Fs).num_fat = 2 as i32 as uint8_t;
        }
    }
    rate_0 = mtools_rate_0;
    rate_any = mtools_rate_any;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(
            argc,
            argv,
            b"i:148f:t:n:v:qub:kK:R:B:r:L:I:FCc:Xh:s:T:l:N:H:M:S:2:30:Aad:m:\0"
                as *const u8 as *const i8,
        );
        if !(c != -(1 as i32)) {
            break;
        }
        *__errno_location() = 0 as i32;
        endptr = 0 as *mut i8;
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            49 => {
                argheads = 1 as i32 as uint16_t;
            }
            52 => {
                argsectors = 9 as i32 as uint16_t;
                argtracks = 40 as i32 as u32;
            }
            56 => {
                argsectors = 8 as i32 as uint16_t;
                argtracks = 40 as i32 as u32;
            }
            102 => {
                r = old_dos_size_to_geom(
                    atoul(optarg),
                    &mut argtracks,
                    &mut argheads,
                    &mut argsectors,
                );
                if r != 0 {
                    fprintf(
                        stderr,
                        b"Bad size %s\n\0" as *const u8 as *const i8,
                        optarg,
                    );
                    exit(1 as i32);
                }
            }
            116 => {
                argtracks = atou16(optarg) as u32;
            }
            84 => {
                tot_sectors = parseSize(optarg);
            }
            110 | 115 => {
                argsectors = atou16(optarg);
            }
            108 | 118 => {
                strncpy(
                    label.as_mut_ptr(),
                    optarg,
                    (20 as i32 * 13 as i32 + 1 as i32 - 1 as i32) as u64,
                );
                label[(20 as i32 * 13 as i32 + 1 as i32 - 1 as i32) as usize] = '\0'
                    as i32 as i8;
            }
            113 | 117 => {
                fprintf(
                    stderr,
                    b"Flag %c not supported by mtools\n\0" as *const u8 as *const i8,
                    c,
                );
                exit(1 as i32);
            }
            98 => {
                haveBiosDisk = 1 as i32 != 0;
                biosDisk = atou8(optarg);
            }
            70 => {
                fat32 = 1 as i32;
            }
            83 => {
                argssize = (atou8(optarg) as i32 | 0x80 as i32) as uint8_t;
                if (argssize as i32) < 0x80 as i32 {
                    usage(1 as i32);
                }
                if argssize as i32 >= 0x87 as i32 {
                    fprintf(
                        stderr,
                        b"argssize must be less than 6\n\0" as *const u8 as *const i8,
                    );
                    usage(1 as i32);
                }
            }
            88 => {
                format_xdf = 1 as i32;
            }
            50 => {
                arguse_2m = 0xff as i32 as u32;
                sectors0 = atou8(optarg);
            }
            51 => {
                arguse_2m = 0x80 as i32 as u32;
            }
            48 => {
                rate_0 = atou8(optarg);
            }
            65 => {
                rate_any = atou8(optarg);
            }
            77 => {
                msize = atou16(optarg);
                if msize as i32 != 512 as i32 && msize as i32 != 1024 as i32
                    && msize as i32 != 2048 as i32 && msize as i32 != 4096 as i32
                {
                    fprintf(
                        stderr,
                        b"Only sector sizes of 512, 1024, 2048 or 4096 bytes are allowed\n\0"
                            as *const u8 as *const i8,
                    );
                    usage(1 as i32);
                }
            }
            78 => {
                serial = strtou32(optarg, &mut endptr, 16 as i32);
                serial_set = 1 as i32;
            }
            97 => {
                Atari = 1 as i32;
            }
            67 => {
                create = 0o100 as i32 | 0o1000 as i32;
            }
            72 => {
                hs = atoui(optarg);
                hs_set = 1 as i32;
            }
            73 => {
                fsVersion = strtou16(optarg, &mut endptr, 0 as i32);
            }
            99 => {
                (*Fs).cluster_size = atou8(optarg);
            }
            114 => {
                (*Fs).dir_len = strtou16(optarg, &mut endptr, 0 as i32);
            }
            76 => {
                (*Fs).fat_len = strtoui(optarg, &mut endptr, 0 as i32);
            }
            66 => {
                bootSector = optarg;
            }
            107 => {
                keepBoot = 1 as i32;
            }
            75 => {
                (*Fs).backupBoot = atou16(optarg);
                if ((*Fs).backupBoot as i32) < 2 as i32 {
                    fprintf(
                        stderr,
                        b"Backupboot must be greater than 2\n\0" as *const u8
                            as *const i8,
                    );
                    exit(1 as i32);
                }
            }
            82 => {
                (*Fs).fat_start = atou16(optarg);
            }
            104 => {
                argheads = atou16(optarg);
            }
            100 => {
                (*Fs).num_fat = atou8(optarg);
            }
            109 => {
                mediaDesc = strtou8(optarg, &mut endptr, 0 as i32);
                if *endptr != 0 {
                    mediaDesc = strtou8(optarg, &mut endptr, 16 as i32);
                }
                if optarg == endptr || *endptr as i32 != 0 {
                    fprintf(
                        stderr,
                        b"Bad mediadesc %s\n\0" as *const u8 as *const i8,
                        optarg,
                    );
                    exit(1 as i32);
                }
                haveMediaDesc = 1 as i32 != 0;
            }
            _ => {
                usage(1 as i32);
            }
        }
        check_number_parse_errno(c as i8, optarg, endptr);
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
    } else {
        drive = get_default_drive();
        if drive as i32 != ':' as i32 {
            fprintf(stderr, b"Drive letter missing\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
    }
    if argtracks != 0 && tot_sectors != 0 {
        fprintf(
            stderr,
            b"Only one of -t or -T may be specified\n\0" as *const u8 as *const i8,
        );
        usage(1 as i32);
    }
    if create != 0 && format_xdf != 0 {
        fprintf(
            stderr,
            b"Create and XDF can't be used together\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const i8,
        drive as i32,
    );
    blocksize = 0 as i32 as uint32_t;
    dev = devices;
    while (*dev).drive != 0 {
        free_stream(&mut (*Fs).head.Next);
        if !((*dev).drive as i32 != drive as i32) {
            used_dev = *dev;
            if argtracks != 0 {
                used_dev.tracks = argtracks;
            }
            if argheads != 0 {
                used_dev.heads = argheads;
            }
            if argsectors != 0 {
                used_dev.sectors = argsectors;
            }
            if arguse_2m != 0 {
                used_dev.use_2m = arguse_2m;
            }
            if argssize != 0 {
                used_dev.ssize = argssize;
            }
            if hs_set != 0 {
                used_dev.hidden = hs;
            }
            expand((*dev).name, name.as_mut_ptr());
            if format_xdf != 0 {
                used_dev.misc_flags |= 0x8 as u32;
            }
            info.FatSize = 0 as i32 as u32;
            if tot_sectors != 0 {
                used_dev.tot_sectors = tot_sectors;
            }
            (*Fs).head.Next = OpenImage(
                &mut used_dev,
                dev,
                name.as_mut_ptr(),
                0o2 as i32 | create,
                errmsg.as_mut_ptr(),
                4 as i32,
                0o2 as i32,
                &mut maxSize,
                0 as *mut i32,
                &mut info,
            );
            if !((*Fs).head.Next).is_null() && info.FatSize != 0 {
                if (*Fs).fat_len == 0 {
                    (*Fs).fat_len = info.FatSize;
                }
                if (*Fs).dir_len == 0 {
                    (*Fs).dir_len = info.RootDirSize;
                }
            }
            if !((*Fs).head.Next).is_null() {
                if tot_sectors != 0 {
                    used_dev.tot_sectors = tot_sectors;
                }
                setFsSectorSize(Fs, &mut used_dev, msize);
                if used_dev.blocksize == 0
                    || used_dev.blocksize < (*Fs).sector_size as u32
                {
                    blocksize = (*Fs).sector_size as uint32_t;
                } else {
                    blocksize = used_dev.blocksize;
                }
                if blocksize > 8192 as i32 as u32 {
                    blocksize = 8192 as i32 as uint32_t;
                }
                if chs_to_totsectors(&mut used_dev, errmsg.as_mut_ptr()) < 0 as i32
                    || check_if_sectors_fit(
                        (*dev).tot_sectors,
                        maxSize,
                        blocksize,
                        errmsg.as_mut_ptr(),
                    ) < 0 as i32
                {
                    free_stream(&mut (*Fs).head.Next);
                } else {
                    if tot_sectors == 0 {
                        tot_sectors = used_dev.tot_sectors;
                    }
                    if !(create == 0
                        && ((*(*(*Fs).head.Next).Class).pread)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*Fs).head.Next,
                            &mut boot.characters as *mut [i8; 4096] as *mut i8,
                            0 as i32 as mt_off_t,
                            (*Fs).sector_size as size_t,
                        ) != (*Fs).sector_size as i32 as i64)
                    {
                        break;
                    }
                    snprintf(
                        errmsg.as_mut_ptr(),
                        (::core::mem::size_of::<[i8; 2100]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                        b"Error reading from '%s', wrong parameters?\0" as *const u8
                            as *const i8,
                        name.as_mut_ptr(),
                    );
                    free_stream(&mut (*Fs).head.Next);
                }
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if (*dev).drive as i32 == 0 as i32 {
        free_stream(&mut (*Fs).head.Next);
        fprintf(
            stderr,
            b"%s: %s\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
            errmsg.as_mut_ptr(),
        );
        exit(1 as i32);
    }
    if tot_sectors == 0 as i32 as u32 {
        fprintf(stderr, b"Disk size not known\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if create != 0 {
        ((*(*(*Fs).head.Next).Class).pwrite)
            .expect(
                "non-null function pointer",
            )(
            (*Fs).head.Next,
            &mut boot.characters as *mut [i8; 4096] as *mut i8,
            sectorsToBytes(Fs, tot_sectors.wrapping_sub(1 as i32 as u32)),
            (*Fs).sector_size as size_t,
        );
    }
    if !bootSector.is_null() {
        let mut fd: i32 = 0;
        let mut ret: ssize_t = 0;
        fd = open(bootSector, 0 as i32 | 0 as i32 | 0 as i32);
        if fd < 0 as i32 {
            perror(b"open boot sector\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        ret = read(
            fd,
            &mut boot.bytes as *mut [u8; 4096] as *mut libc::c_void,
            blocksize as size_t,
        );
        if ret < 0 as i32 as i64 || (ret as size_t) < blocksize as u64 {
            perror(b"short read on boot sector\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        keepBoot = 1 as i32;
        close(fd);
    }
    if keepBoot == 0 && used_dev.use_2m & 0x7f as i32 as u32 == 0 {
        memset(
            (boot.characters).as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            (*Fs).sector_size as u64,
        );
    }
    (*Fs).head.Next = buf_init(
        (*Fs).head.Next,
        blocksize
            .wrapping_mul(used_dev.heads as u32)
            .wrapping_mul(used_dev.sectors as u32) as size_t,
        blocksize
            .wrapping_mul(used_dev.heads as u32)
            .wrapping_mul(used_dev.sectors as u32) as size_t,
        blocksize as size_t,
    );
    boot.boot.nfat = (*Fs).num_fat;
    if keepBoot == 0 {
        set_word(
            &mut *(boot.bytes).as_mut_ptr().offset(510 as i32 as isize),
            0xaa55 as i32 as libc::c_ushort,
        );
    }
    set_word((boot.boot.nsect).as_mut_ptr(), used_dev.sectors);
    set_word((boot.boot.nheads).as_mut_ptr(), used_dev.heads);
    match calc_fs_parameters(
        &mut used_dev,
        fat32 != 0,
        tot_sectors,
        Fs,
        &mut boot.boot.descr,
    ) {
        -1 => {
            fprintf(stderr, b"Too few sectors\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        -2 => {
            fprintf(
                stderr,
                b"Too few clusters for %d bit fat\n\0" as *const u8 as *const i8,
                (*Fs).fat_bits,
            );
            exit(1 as i32);
        }
        -3 => {
            fprintf(
                stderr,
                b"Too many clusters for %d bit FAT\n\0" as *const u8 as *const i8,
                (*Fs).fat_bits,
            );
            exit(1 as i32);
        }
        -4 => {
            fprintf(
                stderr,
                b"Too many clusters for fat length %d\n\0" as *const u8 as *const i8,
                (*Fs).fat_len,
            );
            exit(1 as i32);
        }
        _ => {}
    }
    if keepBoot == 0 && used_dev.use_2m & 0x7f as i32 as u32 == 0 {
        if used_dev.partition == 0 {
            let mut partTable: *mut partition = &mut *(boot.bytes)
                .as_mut_ptr()
                .offset(0x1ae as i32 as isize) as *mut u8 as *mut partition;
            setBeginEnd(
                &mut *partTable.offset(1 as i32 as isize),
                0 as i32 as uint32_t,
                ((used_dev.heads as i32 * used_dev.sectors as i32) as u32)
                    .wrapping_mul(used_dev.tracks),
                used_dev.heads as uint8_t as uint16_t,
                used_dev.sectors as uint8_t as uint16_t,
                1 as i32,
                0 as i32 as uint8_t,
                (*Fs).fat_bits,
            );
        }
    }
    if (*Fs).fat_bits == 32 as i32 as u32 {
        set_word((boot.boot.fatlen).as_mut_ptr(), 0 as i32 as libc::c_ushort);
        set_dword((boot.boot.ext.fat32.bigFat).as_mut_ptr(), (*Fs).fat_len);
        (*Fs).clus_start = ((*Fs).num_fat as u32)
            .wrapping_mul((*Fs).fat_len)
            .wrapping_add((*Fs).fat_start as u32);
        set_word(
            (boot.boot.ext.fat32.extFlags).as_mut_ptr(),
            0 as i32 as libc::c_ushort,
        );
        set_word((boot.boot.ext.fat32.fsVersion).as_mut_ptr(), fsVersion);
        (*Fs).rootCluster = 2 as i32 as uint32_t;
        set_dword((boot.boot.ext.fat32.rootCluster).as_mut_ptr(), (*Fs).rootCluster);
        (*Fs).infoSectorLoc = 1 as i32 as uint32_t;
        set_word(
            (boot.boot.ext.fat32.infoSector).as_mut_ptr(),
            (*Fs).infoSectorLoc as libc::c_ushort,
        );
        (*Fs).infoSectorLoc = 1 as i32 as uint32_t;
        set_word((boot.boot.ext.fat32.backupBoot).as_mut_ptr(), (*Fs).backupBoot);
        labelBlock = &mut boot.boot.ext.fat32.labelBlock;
    } else {
        set_word((boot.boot.fatlen).as_mut_ptr(), (*Fs).fat_len as uint16_t);
        (*Fs).dir_start = ((*Fs).num_fat as u32)
            .wrapping_mul((*Fs).fat_len)
            .wrapping_add((*Fs).fat_start as u32);
        (*Fs).clus_start = ((*Fs).dir_start).wrapping_add((*Fs).dir_len as u32);
        labelBlock = &mut boot.boot.ext.old.labelBlock;
    }
    (*Fs).cp = cp_open(used_dev.codepage);
    if ((*Fs).cp).is_null() {
        exit(1 as i32);
    }
    if haveMediaDesc {
        boot.boot.descr = mediaDesc;
    }
    if haveBiosDisk {
        (*labelBlock).physdrive = biosDisk;
    } else if keepBoot == 0 {
        (*labelBlock).physdrive = (if boot.boot.descr as i32 == 0xf8 as i32 {
            0x80 as i32
        } else {
            0 as i32
        }) as u8;
    }
    (*labelBlock).reserved = 0 as i32 as u8;
    (*labelBlock).dos4 = 0x29 as i32 as u8;
    if serial_set == 0 || Atari != 0 {
        init_random();
    }
    if serial_set == 0 {
        serial = random() as uint32_t;
    }
    set_dword(((*labelBlock).serial).as_mut_ptr(), serial);
    label_name_pc(
        ((*(*(Fs as *mut Stream_t)).Class).get_dosConvert)
            .expect("non-null function pointer")(Fs as *mut Stream_t),
        if label[0 as i32 as usize] as i32 != 0 {
            label.as_mut_ptr()
        } else {
            b"NO NAME    \0" as *const u8 as *const i8
        },
        0 as i32,
        &mut mangled,
        &mut shortlabel,
    );
    strncpy(
        ((*labelBlock).label).as_mut_ptr(),
        (shortlabel.base).as_mut_ptr(),
        8 as i32 as u64,
    );
    strncpy(
        ((*labelBlock).label).as_mut_ptr().offset(8 as i32 as isize),
        (shortlabel.ext).as_mut_ptr(),
        3 as i32 as u64,
    );
    sprintf(
        ((*labelBlock).fat_type).as_mut_ptr(),
        b"FAT%2.2d  \0" as *const u8 as *const i8,
        (*Fs).fat_bits,
    );
    (*labelBlock).fat_type[7 as i32 as usize] = ' ' as i32 as i8;
    set_word((boot.boot.secsiz).as_mut_ptr(), (*Fs).sector_size);
    boot.boot.clsiz = (*Fs).cluster_size;
    set_word((boot.boot.nrsvsect).as_mut_ptr(), (*Fs).fat_start);
    bootOffset = init_geometry_boot(
        &mut boot,
        &mut used_dev,
        sectors0,
        rate_0,
        rate_any,
        &mut tot_sectors,
        keepBoot,
    ) as size_t;
    if bootOffset == 0 {
        bootOffset = (ptrdiff(
            labelBlock as *mut i8,
            (boot.bytes).as_mut_ptr() as *mut i8,
        ))
            .wrapping_add(::core::mem::size_of::<label_blk_t>() as u64);
    }
    if Atari != 0 {
        boot.boot.banner[4 as i32 as usize] = 0 as i32 as i8;
        boot.boot.banner[5 as i32 as usize] = random() as i8;
        boot.boot.banner[6 as i32 as usize] = random() as i8;
        boot.boot.banner[7 as i32 as usize] = random() as i8;
    }
    if keepBoot == 0 && bootOffset <= 65535 as i32 as u64 {
        inst_boot_prg(&mut boot, bootOffset as uint16_t);
    }
    if used_dev.use_2m & 0x7f as i32 as u32 != 0 {
        boot.boot.jump[0 as i32 as usize] = 0xeb as i32 as u8;
        boot.boot.jump[1 as i32 as usize] = 0x80 as i32 as u8;
        boot.boot.jump[2 as i32 as usize] = 0x90 as i32 as u8;
    }
    if used_dev.use_2m & 0x7f as i32 as u32 != 0 {
        (*Fs).num_fat = 1 as i32 as uint8_t;
    }
    (*Fs).lastFatSectorNr = 0 as i32 as uint32_t;
    (*Fs).lastFatSectorData = 0 as *mut u8;
    zero_fat(Fs, boot.boot.descr);
    (*Fs).freeSpace = (*Fs).num_clus;
    (*Fs).last = 2 as i32 as uint32_t;
    if used_dev.misc_flags & 0x8 as u32 != 0 {
        i = 0 as i32 as u32;
        while i
            < (info.BadSectors)
                .wrapping_add((*Fs).cluster_size as u32)
                .wrapping_sub(1 as i32 as u32)
                .wrapping_div((*Fs).cluster_size as u32)
        {
            fatEncode(Fs, i.wrapping_add(2 as i32 as u32), 0xfff7 as i32 as u32);
            i = i.wrapping_add(1);
            i;
        }
    }
    format_root(Fs, label.as_mut_ptr(), &mut boot);
    if ((*(*(Fs as *mut Stream_t)).Class).pwrite)
        .expect(
            "non-null function pointer",
        )(
        Fs as *mut Stream_t,
        (boot.characters).as_mut_ptr(),
        0 as i32 as mt_off_t,
        (*Fs).sector_size as size_t,
    ) < 0 as i32 as i64
    {
        fprintf(stderr, b"Error writing boot sector\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if (*Fs).fat_bits == 32 as i32 as u32
        && (boot.boot.ext.fat32.backupBoot[0 as i32 as usize] as i32
            + ((boot.boot.ext.fat32.backupBoot[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32 != 0xffff as i32
    {
        if ((*(*(Fs as *mut Stream_t)).Class).pwrite)
            .expect(
                "non-null function pointer",
            )(
            Fs as *mut Stream_t,
            (boot.characters).as_mut_ptr(),
            sectorsToBytes(
                Fs,
                (boot.boot.ext.fat32.backupBoot[0 as i32 as usize] as i32
                    + ((boot.boot.ext.fat32.backupBoot[1 as i32 as usize] as i32)
                        << 8 as i32)) as uint16_t as uint32_t,
            ),
            (*Fs).sector_size as size_t,
        ) < 0 as i32 as i64
        {
            fprintf(
                stderr,
                b"Error writing backup boot sector\n\0" as *const u8 as *const i8,
            );
            exit(1 as i32);
        }
    }
    free_stream(&mut Fs as *mut *mut Fs_t as *mut *mut Stream_t);
    if format_xdf != 0 && isatty(0 as i32) != 0
        && (getenv(b"MTOOLS_USE_XDF\0" as *const u8 as *const i8)).is_null()
    {
        fprintf(
            stderr,
            b"Note:\nRemember to set the \"MTOOLS_USE_XDF\" environmental\nvariable before accessing this disk\n\nBourne shell syntax (sh, ash, bash, ksh, zsh etc):\n export MTOOLS_USE_XDF=1\n\nC shell syntax (csh and tcsh):\n setenv MTOOLS_USE_XDF 1\n\0"
                as *const u8 as *const i8,
        );
    }
    exit(0 as i32);
}
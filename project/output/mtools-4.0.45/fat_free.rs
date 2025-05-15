use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    pub type FatMap_t;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn fatDeallocate(This: *mut Fs_t, pos: u32);
    fn fatDecode(This: *mut Fs_t, pos: u32) -> u32;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
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
#[no_mangle]
pub unsafe extern "C" fn fat_free(mut Dir: *mut Stream_t, mut fat: u32) -> i32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut next_no_step: u32 = 0;
    if fat == 0 as i32 as u32 {
        return 0 as i32;
    }
    while (*This).fat_error == 0 {
        next_no_step = fatDecode(This, fat);
        fatDeallocate(This, fat);
        if next_no_step >= (*This).last_fat {
            break;
        }
        fat = next_no_step;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn fatFreeWithDir(
    mut Dir: *mut Stream_t,
    mut dir: *mut directory,
) -> i32 {
    let mut first: uint32_t = 0;
    if (strncmp(
        ((*dir).name).as_mut_ptr(),
        b".      \0" as *const u8 as *const i8,
        8 as i32 as u64,
    ) == 0
        || strncmp(
            ((*dir).name).as_mut_ptr(),
            b"..     \0" as *const u8 as *const i8,
            8 as i32 as u64,
        ) == 0)
        && strncmp(
            ((*dir).ext).as_mut_ptr(),
            b"   \0" as *const u8 as *const i8,
            3 as i32 as u64,
        ) == 0
    {
        fprintf(stderr, b"Trying to remove . or .. entry\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    first = ((*dir).start[0 as i32 as usize] as i32
        + (((*dir).start[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
        as uint32_t;
    if fat32RootCluster(Dir) != 0 {
        first
            |= (((*dir).startHi[0 as i32 as usize] as i32
                + (((*dir).startHi[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
                as uint32_t) << 16 as i32;
    }
    return fat_free(Dir, first);
}
#[no_mangle]
pub unsafe extern "C" fn fatFreeWithDirentry(mut entry: *mut direntry_t) -> i32 {
    return fatFreeWithDir((*entry).Dir, &mut (*entry).dir);
}
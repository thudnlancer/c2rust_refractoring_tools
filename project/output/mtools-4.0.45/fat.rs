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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut got_signal: i32;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    static mut mtools_skip_check: u32;
    static mut batchmode: i32;
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
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn cp_close(cp: *mut doscp_t);
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
pub struct InfoSector_t {
    pub signature1: [u8; 4],
    pub filler1: [u8; 480],
    pub signature2: [u8; 4],
    pub count: [u8; 4],
    pub pos: [u8; 4],
    pub filler2: [u8; 14],
    pub signature3: [u8; 2],
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
pub union C2RustUnnamed {
    pub fat32: fat32_t,
    pub old: oldboot_t,
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
pub type smt_off_t = mt_off_t;
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
pub struct FatMap_t {
    pub data: *mut u8,
    pub dirty: fatBitMask,
    pub valid: fatBitMask,
}
pub type fatBitMask = libc::c_longlong;
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
#[inline]
unsafe extern "C" fn readSector(
    mut This: *mut Fs_t,
    mut buf: *mut i8,
    mut off: u32,
    mut size: size_t,
) -> ssize_t {
    return ((*(*(*This).head.Next).Class).pread)
        .expect(
            "non-null function pointer",
        )(
        (*This).head.Next,
        buf,
        sectorsToBytes(This, off),
        size << (*This).sectorShift,
    );
}
#[inline]
unsafe extern "C" fn forceReadSector(
    mut This: *mut Fs_t,
    mut buf: *mut i8,
    mut off: u32,
    mut size: size_t,
) -> ssize_t {
    return force_pread(
        (*This).head.Next,
        buf,
        sectorsToBytes(This, off),
        size << (*This).sectorShift,
    );
}
#[inline]
unsafe extern "C" fn forceWriteSector(
    mut This: *mut Fs_t,
    mut buf: *mut i8,
    mut off: u32,
    mut size: size_t,
) -> ssize_t {
    return force_pwrite(
        (*This).head.Next,
        buf,
        sectorsToBytes(This, off),
        size << (*This).sectorShift,
    );
}
unsafe extern "C" fn GetFatMap(mut Stream: *mut Fs_t) -> *mut FatMap_t {
    let mut nr_entries: size_t = 0;
    let mut i: size_t = 0;
    let mut map: *mut FatMap_t = 0 as *mut FatMap_t;
    (*Stream).fat_error = 0 as i32;
    nr_entries = ((*Stream).fat_len as u64)
        .wrapping_add(
            (::core::mem::size_of::<fatBitMask>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(
            (::core::mem::size_of::<fatBitMask>() as u64).wrapping_mul(8 as i32 as u64),
        );
    map = calloc(nr_entries, ::core::mem::size_of::<FatMap_t>() as u64) as *mut FatMap_t;
    if map.is_null() {
        return 0 as *mut FatMap_t;
    }
    i = 0 as i32 as size_t;
    while i < nr_entries {
        let ref mut fresh0 = (*map.offset(i as isize)).data;
        *fresh0 = 0 as *mut u8;
        (*map.offset(i as isize)).valid = 0 as i32 as fatBitMask;
        (*map.offset(i as isize)).dirty = 0 as i32 as fatBitMask;
        i = i.wrapping_add(1);
        i;
    }
    return map;
}
#[inline]
unsafe extern "C" fn locate(
    mut Stream: *mut Fs_t,
    mut offset: uint32_t,
    mut slot: *mut uint32_t,
    mut bit: *mut uint32_t,
) -> i32 {
    if offset >= (*Stream).fat_len {
        return -(1 as i32);
    }
    *slot = (offset as u64)
        .wrapping_div(
            (::core::mem::size_of::<fatBitMask>() as u64).wrapping_mul(8 as i32 as u64),
        ) as uint32_t;
    *bit = (offset as u64)
        .wrapping_rem(
            (::core::mem::size_of::<fatBitMask>() as u64).wrapping_mul(8 as i32 as u64),
        ) as uint32_t;
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn fatReadSector(
    mut This: *mut Fs_t,
    mut sector: u32,
    mut slot: u32,
    mut bit: u32,
    mut dupe: u32,
    mut bitmap: fatBitMask,
) -> ssize_t {
    let mut fat_start: u32 = 0;
    let mut ret: ssize_t = 0;
    let mut nr_sectors: u32 = 0;
    dupe = dupe.wrapping_add((*This).primaryFat).wrapping_rem((*This).num_fat as u32);
    fat_start = ((*This).fat_start as u32)
        .wrapping_add(((*This).fat_len).wrapping_mul(dupe));
    if bitmap == 0 as i32 as libc::c_longlong {
        nr_sectors = (::core::mem::size_of::<fatBitMask>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_sub(
                (bit as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<fatBitMask>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ),
            ) as u32;
    } else {
        nr_sectors = 1 as i32 as u32;
    }
    ret = readSector(
        This,
        ((*((*This).FatMap).offset(slot as isize)).data)
            .offset((bit << (*This).sectorShift) as isize) as *mut i8,
        fat_start.wrapping_add(sector),
        nr_sectors as size_t,
    );
    if ret < 0 as i32 as i64 {
        return 0 as i32 as ssize_t;
    }
    if (ret as size_t) < (*This).sector_size as u64 {
        ret = forceReadSector(
            This,
            ((*((*This).FatMap).offset(slot as isize)).data)
                .offset((bit << (*This).sectorShift) as isize) as *mut i8,
            fat_start.wrapping_add(sector),
            1 as i32 as size_t,
        );
        if ret < (*This).sector_size as i32 as i64 {
            return 0 as i32 as ssize_t;
        }
        return 1 as i32 as ssize_t;
    }
    return ret >> (*This).sectorShift;
}
unsafe extern "C" fn fatWriteSector(
    mut This: *mut Fs_t,
    mut sector: u32,
    mut slot: u32,
    mut bit: u32,
    mut dupe: u32,
) -> ssize_t {
    let mut fat_start: u32 = 0;
    dupe = dupe.wrapping_add((*This).primaryFat).wrapping_rem((*This).num_fat as u32);
    if dupe != 0 && (*This).writeAllFats == 0 {
        return (*This).sector_size as ssize_t;
    }
    fat_start = ((*This).fat_start as u32)
        .wrapping_add(((*This).fat_len).wrapping_mul(dupe));
    return forceWriteSector(
        This,
        ((*((*This).FatMap).offset(slot as isize)).data)
            .offset(bit.wrapping_mul((*This).sector_size as u32) as isize) as *mut i8,
        fat_start.wrapping_add(sector),
        1 as i32 as size_t,
    );
}
unsafe extern "C" fn loadSector(
    mut This: *mut Fs_t,
    mut sector: u32,
    mut mode: fatAccessMode_t,
    mut recurs: i32,
) -> *mut u8 {
    let mut slot: uint32_t = 0;
    let mut bit: uint32_t = 0;
    let mut ret: ssize_t = 0;
    if locate(This, sector, &mut slot, &mut bit) < 0 as i32 {
        return 0 as *mut u8;
    }
    if ((*((*This).FatMap).offset(slot as isize)).data).is_null() {
        let ref mut fresh1 = (*((*This).FatMap).offset(slot as isize)).data;
        *fresh1 = malloc(
            ((*This).sector_size as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<fatBitMask>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                ),
        ) as *mut u8;
        if ((*((*This).FatMap).offset(slot as isize)).data).is_null() {
            return 0 as *mut u8;
        }
        memset(
            (*((*This).FatMap).offset(slot as isize)).data as *mut libc::c_void,
            0xee as i32,
            ((*This).sector_size as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<fatBitMask>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                ),
        );
    }
    if (*((*This).FatMap).offset(slot as isize)).valid & (1 as i32 as fatBitMask) << bit
        == 0
    {
        let mut i: u32 = 0;
        ret = -(1 as i32) as ssize_t;
        i = 0 as i32 as u32;
        while i < (*This).num_fat as u32 {
            ret = fatReadSector(
                This,
                sector,
                slot,
                bit,
                i,
                (*((*This).FatMap).offset(slot as isize)).valid,
            );
            if ret == 0 as i32 as i64 {
                fprintf(
                    stderr,
                    b"Error reading fat number %d\n\0" as *const u8 as *const i8,
                    i,
                );
                i = i.wrapping_add(1);
                i;
            } else {
                if (*((*This).FatMap).offset(slot as isize)).valid != 0 {
                    recurs = 1 as i32;
                }
                break;
            }
        }
        if ret == 0 as i32 as i64 {
            return 0 as *mut u8;
        }
        i = 0 as i32 as u32;
        while (i as i32 as i64) < ret {
            let ref mut fresh2 = (*((*This).FatMap).offset(slot as isize)).valid;
            *fresh2 |= (1 as i32 as fatBitMask) << bit.wrapping_add(i);
            i = i.wrapping_add(1);
            i;
        }
        if recurs == 0 && ret == 1 as i32 as i64 {
            loadSector(This, sector.wrapping_add(1 as i32 as u32), mode, 1 as i32);
        }
        if recurs == 0 && batchmode != 0 {
            i = 0 as i32 as u32;
            while i < 1024 as i32 as u32 {
                loadSector(This, sector.wrapping_add(i), mode, 1 as i32);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    if mode as u32 == fatAccessMode_t::FAT_ACCESS_WRITE as i32 as u32 {
        let ref mut fresh3 = (*((*This).FatMap).offset(slot as isize)).dirty;
        *fresh3 |= (1 as i32 as fatBitMask) << bit;
        (*This).fat_dirty = 1 as i32;
    }
    return ((*((*This).FatMap).offset(slot as isize)).data)
        .offset((bit << (*This).sectorShift) as isize);
}
unsafe extern "C" fn getAddress(
    mut Stream: *mut Fs_t,
    mut num: u32,
    mut mode: fatAccessMode_t,
) -> *mut u8 {
    let mut ret: *mut u8 = 0 as *mut u8;
    let mut sector: u32 = 0;
    let mut offset: u32 = 0;
    sector = num >> (*Stream).sectorShift;
    ret = 0 as *mut u8;
    if sector == (*Stream).lastFatSectorNr
        && (*Stream).lastFatAccessMode as u32 >= mode as u32
    {
        ret = (*Stream).lastFatSectorData;
    }
    if ret.is_null() {
        ret = loadSector(Stream, sector, mode, 0 as i32);
        if ret.is_null() {
            return 0 as *mut u8;
        }
        (*Stream).lastFatSectorNr = sector;
        (*Stream).lastFatSectorData = ret;
        (*Stream).lastFatAccessMode = mode;
    }
    offset = num & (*Stream).sectorMask;
    return ret.offset(offset as isize);
}
unsafe extern "C" fn readByte(mut Stream: *mut Fs_t, mut start: u32) -> i32 {
    let mut address: *mut u8 = 0 as *mut u8;
    address = getAddress(Stream, start, fatAccessMode_t::FAT_ACCESS_READ);
    if address.is_null() {
        return -(1 as i32);
    }
    return *address as i32;
}
unsafe extern "C" fn fat12_decode(mut Stream: *mut Fs_t, mut num: u32) -> u32 {
    let mut start: u32 = num.wrapping_mul(3 as i32 as u32).wrapping_div(2 as i32 as u32);
    let mut byte0: i32 = readByte(Stream, start);
    let mut byte1: i32 = readByte(Stream, start.wrapping_add(1 as i32 as u32));
    if num < 2 as i32 as u32 || byte0 < 0 as i32 || byte1 < 0 as i32
        || num > ((*Stream).num_clus).wrapping_add(1 as i32 as u32)
    {
        fprintf(stderr, b"[1] Bad address %d\n\0" as *const u8 as *const i8, num);
        return 1 as i32 as u32;
    }
    if num & 1 as i32 as u32 != 0 {
        return (byte1 as uint32_t) << 4 as i32
            | (byte0 as uint32_t & 0xf0 as i32 as u32) >> 4 as i32
    } else {
        return (byte1 as uint32_t & 0xf as i32 as u32) << 8 as i32 | byte0 as uint32_t
    };
}
unsafe extern "C" fn fat12_encode(mut Stream: *mut Fs_t, mut num: u32, mut code: u32) {
    let mut start: u32 = num.wrapping_mul(3 as i32 as u32).wrapping_div(2 as i32 as u32);
    let mut address0: *mut u8 = getAddress(
        Stream,
        start,
        fatAccessMode_t::FAT_ACCESS_WRITE,
    );
    let mut address1: *mut u8 = getAddress(
        Stream,
        start.wrapping_add(1 as i32 as u32),
        fatAccessMode_t::FAT_ACCESS_WRITE,
    );
    if num & 1 as i32 as u32 != 0 {
        *address0 = ((*address0 as i32 & 0xf as i32) as u32
            | code << 4 as i32 & 0xf0 as i32 as u32) as u8;
        *address1 = (code >> 4 as i32 & 0xff as i32 as u32) as u8;
    } else {
        *address0 = (code & 0xff as i32 as u32) as u8;
        *address1 = ((*address1 as i32 & 0xf0 as i32) as u32
            | code >> 8 as i32 & 0xf as i32 as u32) as u8;
    };
}
unsafe extern "C" fn fat16_decode(mut Stream: *mut Fs_t, mut num: u32) -> u32 {
    let mut address: *mut u8 = getAddress(
        Stream,
        num << 1 as i32,
        fatAccessMode_t::FAT_ACCESS_READ,
    );
    if address.is_null() {
        return 1 as i32 as u32;
    }
    return (*address.offset(0 as i32 as isize) as i32
        + ((*address.offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as u32;
}
unsafe extern "C" fn fat16_encode(mut Stream: *mut Fs_t, mut num: u32, mut code: u32) {
    let mut address: *mut u8 = 0 as *mut u8;
    if code > 65535 as i32 as u32 {
        fprintf(stderr, b"FAT16 code %x too big\n\0" as *const u8 as *const i8, code);
        exit(1 as i32);
    }
    address = getAddress(Stream, num << 1 as i32, fatAccessMode_t::FAT_ACCESS_WRITE);
    set_word(address, code as uint16_t);
}
unsafe extern "C" fn fast_fat16_decode(mut Stream: *mut Fs_t, mut num: u32) -> u32 {
    let mut address: *mut libc::c_ushort = getAddress(
        Stream,
        num << 1 as i32,
        fatAccessMode_t::FAT_ACCESS_READ,
    ) as *mut libc::c_ushort;
    if address.is_null() {
        return 1 as i32 as u32;
    }
    return *address as u32;
}
unsafe extern "C" fn fast_fat16_encode(
    mut Stream: *mut Fs_t,
    mut num: u32,
    mut code: u32,
) {
    let mut address: *mut libc::c_ushort = getAddress(
        Stream,
        num << 1 as i32,
        fatAccessMode_t::FAT_ACCESS_WRITE,
    ) as *mut libc::c_ushort;
    if code > 65535 as i32 as u32 {
        fprintf(stderr, b"FAT16 code %x too big\n\0" as *const u8 as *const i8, code);
        exit(1 as i32);
    }
    *address = code as uint16_t;
}
unsafe extern "C" fn fat32_decode(mut Stream: *mut Fs_t, mut num: u32) -> u32 {
    let mut address: *mut u8 = getAddress(
        Stream,
        num << 2 as i32,
        fatAccessMode_t::FAT_ACCESS_READ,
    );
    if address.is_null() {
        return 1 as i32 as u32;
    }
    return ((*address.offset(0 as i32 as isize) as i32
        + ((*address.offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32
        + (((*address.offset(2 as i32 as isize).offset(0 as i32 as isize) as i32
            + ((*address.offset(2 as i32 as isize).offset(1 as i32 as isize) as i32)
                << 8 as i32)) as uint16_t as i32) << 16 as i32)) as uint32_t
        & 0xfffffff as i32 as u32;
}
unsafe extern "C" fn fat32_encode(mut Stream: *mut Fs_t, mut num: u32, mut code: u32) {
    let mut address: *mut u8 = getAddress(
        Stream,
        num << 2 as i32,
        fatAccessMode_t::FAT_ACCESS_WRITE,
    );
    set_dword(
        address,
        code & 0xfffffff as i32 as u32
            | ((*address.offset(0 as i32 as isize) as i32
                + ((*address.offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                as i32
                + (((*address.offset(2 as i32 as isize).offset(0 as i32 as isize) as i32
                    + ((*address.offset(2 as i32 as isize).offset(1 as i32 as isize)
                        as i32) << 8 as i32)) as uint16_t as i32) << 16 as i32))
                as uint32_t & 0xf0000000 as u32,
    );
}
unsafe extern "C" fn fast_fat32_decode(mut Stream: *mut Fs_t, mut num: u32) -> u32 {
    let mut address: *mut u32 = getAddress(
        Stream,
        num << 2 as i32,
        fatAccessMode_t::FAT_ACCESS_READ,
    ) as *mut u32;
    if address.is_null() {
        return 1 as i32 as u32;
    }
    return *address & 0xfffffff as i32 as u32;
}
unsafe extern "C" fn fast_fat32_encode(
    mut Stream: *mut Fs_t,
    mut num: u32,
    mut code: u32,
) {
    let mut address: *mut u32 = getAddress(
        Stream,
        num << 2 as i32,
        fatAccessMode_t::FAT_ACCESS_WRITE,
    ) as *mut u32;
    *address = *address & 0xf0000000 as u32 | code & 0xfffffff as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn fat_write(mut This: *mut Fs_t) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut dups: u32 = 0;
    let mut bit: u32 = 0;
    let mut slot: u32 = 0;
    let mut ret: ssize_t = 0;
    if (*This).fat_dirty == 0 {
        return;
    }
    dups = (*This).num_fat as u32;
    if (*This).fat_error != 0 {
        dups = 1 as i32 as u32;
    }
    i = 0 as i32 as u32;
    while i < dups {
        j = 0 as i32 as u32;
        slot = 0 as i32 as u32;
        while j < (*This).fat_len {
            if (*((*This).FatMap).offset(slot as isize)).dirty == 0 {
                j = (j as u64)
                    .wrapping_add(
                        (::core::mem::size_of::<fatBitMask>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ) as u32 as u32;
            } else {
                bit = 0 as i32 as u32;
                while (bit as u64)
                    < (::core::mem::size_of::<fatBitMask>() as u64)
                        .wrapping_mul(8 as i32 as u64) && j < (*This).fat_len
                {
                    if !((*((*This).FatMap).offset(slot as isize)).dirty
                        & (1 as i32 as fatBitMask) << bit == 0)
                    {
                        ret = fatWriteSector(This, j, slot, bit, i);
                        if ret < (*This).sector_size as i32 as i64 {
                            if ret < 0 as i32 as i64 {
                                perror(b"error in fat_write\0" as *const u8 as *const i8);
                                exit(1 as i32);
                            } else {
                                fprintf(
                                    stderr,
                                    b"end of file in fat_write\n\0" as *const u8 as *const i8,
                                );
                                exit(1 as i32);
                            }
                        }
                        if i == dups.wrapping_sub(1 as i32 as u32) {
                            let ref mut fresh4 = (*((*This).FatMap)
                                .offset(slot as isize))
                                .dirty;
                            *fresh4 &= !((1 as i32 as fatBitMask) << bit);
                        }
                    }
                    bit = bit.wrapping_add(1);
                    bit;
                    j = j.wrapping_add(1);
                    j;
                }
            }
            slot = slot.wrapping_add(1);
            slot;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*This).infoSectorLoc != 0 && (*This).infoSectorLoc != 0xffffffff as u32 {
        let mut infoSector: *mut InfoSector_t = 0 as *mut InfoSector_t;
        infoSector = safe_malloc((*This).sector_size as size_t) as *mut InfoSector_t;
        if forceReadSector(
            This,
            infoSector as *mut i8,
            (*This).infoSectorLoc,
            1 as i32 as size_t,
        ) != (*This).sector_size as i32 as i64
        {
            fprintf(
                stderr,
                b"Trouble reading the info sector\n\0" as *const u8 as *const i8,
            );
            memset(
                ((*infoSector).filler1).as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<[u8; 480]>() as u64,
            );
            memset(
                ((*infoSector).filler2).as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<[u8; 14]>() as u64,
            );
        }
        set_dword(
            ((*infoSector).signature1).as_mut_ptr(),
            0x41615252 as i32 as uint32_t,
        );
        set_dword(
            ((*infoSector).signature2).as_mut_ptr(),
            0x61417272 as i32 as uint32_t,
        );
        set_dword(((*infoSector).pos).as_mut_ptr(), (*This).last);
        set_dword(((*infoSector).count).as_mut_ptr(), (*This).freeSpace);
        set_word(
            ((*infoSector).signature3).as_mut_ptr(),
            0xaa55 as i32 as libc::c_ushort,
        );
        if forceWriteSector(
            This,
            infoSector as *mut i8,
            (*This).infoSectorLoc,
            1 as i32 as size_t,
        ) != (*This).sector_size as i32 as i64
        {
            fprintf(
                stderr,
                b"Trouble writing the info sector\n\0" as *const u8 as *const i8,
            );
        }
        free(infoSector as *mut libc::c_void);
    }
    (*This).fat_dirty = 0 as i32;
    (*This).lastFatAccessMode = fatAccessMode_t::FAT_ACCESS_READ;
}
#[no_mangle]
pub unsafe extern "C" fn zero_fat(
    mut Stream: *mut Fs_t,
    mut media_descriptor: uint8_t,
) -> i32 {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut fat_start: u32 = 0;
    let mut buf: *mut u8 = 0 as *mut u8;
    buf = malloc((*Stream).sector_size as u64) as *mut u8;
    if buf.is_null() {
        perror(b"alloc fat sector buffer\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    i = 0 as i32 as u32;
    while i < (*Stream).num_fat as u32 {
        fat_start = ((*Stream).fat_start as u32)
            .wrapping_add(i.wrapping_mul((*Stream).fat_len));
        j = 0 as i32 as u32;
        while j < (*Stream).fat_len {
            if j <= 1 as i32 as u32 {
                memset(buf as *mut libc::c_void, 0 as i32, (*Stream).sector_size as u64);
            }
            if j == 0 {
                *buf.offset(0 as i32 as isize) = media_descriptor;
                let ref mut fresh5 = *buf.offset(1 as i32 as isize);
                *fresh5 = 0xff as i32 as u8;
                *buf.offset(2 as i32 as isize) = *fresh5;
                if (*Stream).fat_bits > 12 as i32 as u32 {
                    *buf.offset(3 as i32 as isize) = 0xff as i32 as u8;
                }
                if (*Stream).fat_bits > 16 as i32 as u32 {
                    *buf.offset(3 as i32 as isize) = 0xf as i32 as u8;
                    *buf.offset(4 as i32 as isize) = 0xff as i32 as u8;
                    *buf.offset(5 as i32 as isize) = 0xff as i32 as u8;
                    *buf.offset(6 as i32 as isize) = 0xff as i32 as u8;
                    *buf.offset(7 as i32 as isize) = 0xff as i32 as u8;
                }
            }
            if forceWriteSector(
                Stream,
                buf as *mut i8,
                fat_start.wrapping_add(j),
                1 as i32 as size_t,
            ) != (*Stream).sector_size as i32 as i64
            {
                fprintf(
                    stderr,
                    b"Trouble initializing a FAT sector\n\0" as *const u8 as *const i8,
                );
                free(buf as *mut libc::c_void);
                return -(1 as i32);
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(buf as *mut libc::c_void);
    (*Stream).FatMap = GetFatMap(Stream);
    if ((*Stream).FatMap).is_null() {
        perror(b"alloc fat map\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn set_fat12(mut This: *mut Fs_t) {
    (*This).fat_bits = 12 as i32 as u32;
    (*This).end_fat = 0xfff as i32 as uint32_t;
    (*This).last_fat = 0xff6 as i32 as uint32_t;
    (*This).fat_decode = Some(
        fat12_decode as unsafe extern "C" fn(*mut Fs_t, u32) -> u32,
    );
    (*This).fat_encode = Some(
        fat12_encode as unsafe extern "C" fn(*mut Fs_t, u32, u32) -> (),
    );
}
static mut word_endian_test: uint16_t = 0x1234 as i32 as uint16_t;
unsafe extern "C" fn set_fat16(mut This: *mut Fs_t) {
    let mut t: *mut uint8_t = &mut word_endian_test as *mut uint16_t as *mut uint8_t;
    (*This).fat_bits = 16 as i32 as u32;
    (*This).end_fat = 0xffff as i32 as uint32_t;
    (*This).last_fat = 0xfff6 as i32 as uint32_t;
    if *t.offset(0 as i32 as isize) as i32 == 0x34 as i32
        && *t.offset(1 as i32 as isize) as i32 == 0x12 as i32
    {
        (*This).fat_decode = Some(
            fast_fat16_decode as unsafe extern "C" fn(*mut Fs_t, u32) -> u32,
        );
        (*This).fat_encode = Some(
            fast_fat16_encode as unsafe extern "C" fn(*mut Fs_t, u32, u32) -> (),
        );
    } else {
        (*This).fat_decode = Some(
            fat16_decode as unsafe extern "C" fn(*mut Fs_t, u32) -> u32,
        );
        (*This).fat_encode = Some(
            fat16_encode as unsafe extern "C" fn(*mut Fs_t, u32, u32) -> (),
        );
    };
}
static mut dword_endian_test: uint32_t = 0x12345678 as i32 as uint32_t;
unsafe extern "C" fn set_fat32(mut This: *mut Fs_t) {
    let mut t: *mut uint8_t = &mut dword_endian_test as *mut uint32_t as *mut uint8_t;
    (*This).fat_bits = 32 as i32 as u32;
    (*This).end_fat = 0xfffffff as i32 as uint32_t;
    (*This).last_fat = 0xffffff6 as i32 as uint32_t;
    if *t.offset(0 as i32 as isize) as i32 == 0x78 as i32
        && *t.offset(1 as i32 as isize) as i32 == 0x56 as i32
        && *t.offset(2 as i32 as isize) as i32 == 0x34 as i32
        && *t.offset(3 as i32 as isize) as i32 == 0x12 as i32
    {
        (*This).fat_decode = Some(
            fast_fat32_decode as unsafe extern "C" fn(*mut Fs_t, u32) -> u32,
        );
        (*This).fat_encode = Some(
            fast_fat32_encode as unsafe extern "C" fn(*mut Fs_t, u32, u32) -> (),
        );
    } else {
        (*This).fat_decode = Some(
            fat32_decode as unsafe extern "C" fn(*mut Fs_t, u32) -> u32,
        );
        (*This).fat_encode = Some(
            fat32_encode as unsafe extern "C" fn(*mut Fs_t, u32, u32) -> (),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_fat(mut This: *mut Fs_t, mut haveBigFatLen: bool) {
    if haveBigFatLen {
        set_fat32(This);
    } else if (*This).num_clus < 0xff5 as i32 as u32 {
        set_fat12(This);
    } else if (*This).num_clus < 0xfff5 as i32 as u32 {
        set_fat16(This);
    } else {
        set_fat32(This);
    };
}
unsafe extern "C" fn check_fat(mut This: *mut Fs_t) -> i32 {
    let mut i: u32 = 0;
    let mut f: u32 = 0;
    let mut tocheck: u32 = 0;
    if mtools_skip_check != 0 {
        return 0 as i32;
    }
    if (*This).fat_len
        < ((*This).num_clus)
            .wrapping_add(2 as i32 as u32)
            .wrapping_mul(((*This).fat_bits).wrapping_div(4 as i32 as u32))
            .wrapping_sub(1 as i32 as u32)
            .wrapping_div(2 as i32 as u32)
            .wrapping_div((*This).sector_size as u32)
            .wrapping_add(1 as i32 as u32)
    {
        fprintf(stderr, b"Too few sectors in FAT\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    tocheck = (*This).num_clus;
    if tocheck.wrapping_add(1 as i32 as u32) >= (*This).last_fat {
        fprintf(stderr, b"Too many clusters in FAT\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    if tocheck > 4096 as i32 as u32 {
        tocheck = 4096 as i32 as u32;
    }
    i = 3 as i32 as u32;
    while i < tocheck {
        f = ((*This).fat_decode).expect("non-null function pointer")(This, i);
        if f == 1 as i32 as u32 || f < (*This).last_fat && f > (*This).num_clus {
            fprintf(
                stderr,
                b"Cluster # at %d too big(%#x)\n\0" as *const u8 as *const i8,
                i,
                f,
            );
            fprintf(stderr, b"Probably non MS-DOS disk\n\0" as *const u8 as *const i8);
            return -(1 as i32);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn check_media_type(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
) -> i32 {
    let mut address: *mut u8 = 0 as *mut u8;
    (*This).FatMap = GetFatMap(This);
    if ((*This).FatMap).is_null() {
        perror(b"alloc fat map\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    address = getAddress(This, 0 as i32 as u32, fatAccessMode_t::FAT_ACCESS_READ);
    if address.is_null() {
        fprintf(
            stderr,
            b"Could not read first FAT sector\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    if mtools_skip_check != 0 {
        return 0 as i32;
    }
    if *address.offset(0 as i32 as isize) == 0 && *address.offset(1 as i32 as isize) == 0
        && *address.offset(2 as i32 as isize) == 0
    {
        return 0 as i32;
    }
    if *address.offset(0 as i32 as isize) as i32 != (*boot).boot.descr as i32
        && (*boot).boot.descr as i32 >= 0xf0 as i32
        && (*address.offset(0 as i32 as isize) as i32 != 0xf9 as i32
            && *address.offset(0 as i32 as isize) as i32 != 0xf7 as i32
            || (*boot).boot.descr as i32 != 0xf0 as i32)
        || (*address.offset(0 as i32 as isize) as i32) < 0xf0 as i32
    {
        fprintf(
            stderr,
            b"Bad media types %02x/%02x, probably non-MSDOS disk\n\0" as *const u8
                as *const i8,
            *address.offset(0 as i32 as isize) as i32,
            (*boot).boot.descr as i32,
        );
        return -(1 as i32);
    }
    if *address.offset(1 as i32 as isize) as i32 != 0xff as i32
        || *address.offset(2 as i32 as isize) as i32 != 0xff as i32
    {
        fprintf(
            stderr,
            b"Initial bytes of fat is not 0xff\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn fat_32_read(mut This: *mut Fs_t, mut boot: *mut bootsector) -> i32 {
    let mut size: size_t = 0;
    (*This).fat_len = (((*boot).boot.ext.fat32.bigFat[0 as i32 as usize] as i32
        + (((*boot).boot.ext.fat32.bigFat[1 as i32 as usize] as i32) << 8 as i32))
        as uint16_t as i32
        + (((*((*boot).boot.ext.fat32.bigFat)
            .as_mut_ptr()
            .offset(2 as i32 as isize)
            .offset(0 as i32 as isize) as i32
            + ((*((*boot).boot.ext.fat32.bigFat)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
            << 16 as i32)) as uint32_t;
    (*This).writeAllFats = ((*boot).boot.ext.fat32.extFlags[0 as i32 as usize] as i32
        & 0x80 as i32 == 0) as i32 as uint32_t;
    (*This).primaryFat = ((*boot).boot.ext.fat32.extFlags[0 as i32 as usize] as i32
        & 0xf as i32) as uint32_t;
    (*This).rootCluster = (((*boot).boot.ext.fat32.rootCluster[0 as i32 as usize] as i32
        + (((*boot).boot.ext.fat32.rootCluster[1 as i32 as usize] as i32) << 8 as i32))
        as uint16_t as i32
        + (((*((*boot).boot.ext.fat32.rootCluster)
            .as_mut_ptr()
            .offset(2 as i32 as isize)
            .offset(0 as i32 as isize) as i32
            + ((*((*boot).boot.ext.fat32.rootCluster)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
            << 16 as i32)) as uint32_t;
    size = (*This).sector_size as size_t;
    (*This).infoSectorLoc = ((*boot).boot.ext.fat32.infoSector[0 as i32 as usize] as i32
        + (((*boot).boot.ext.fat32.infoSector[1 as i32 as usize] as i32) << 8 as i32))
        as uint16_t as uint32_t;
    if (*This).sector_size as i32 >= 512 as i32 && (*This).infoSectorLoc != 0
        && (*This).infoSectorLoc != 0xffffffff as u32
    {
        let mut infoSector: *mut InfoSector_t = 0 as *mut InfoSector_t;
        infoSector = safe_malloc(size) as *mut InfoSector_t;
        if forceReadSector(
            This,
            infoSector as *mut i8,
            (*This).infoSectorLoc,
            1 as i32 as size_t,
        ) == (*This).sector_size as i32 as i64
            && (((*infoSector).signature1[0 as i32 as usize] as i32
                + (((*infoSector).signature1[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*infoSector).signature1)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*infoSector).signature1)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t == 0x41615252 as i32 as u32
            && (((*infoSector).signature2[0 as i32 as usize] as i32
                + (((*infoSector).signature2[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*infoSector).signature2)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*infoSector).signature2)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t == 0x61417272 as i32 as u32
        {
            (*This).freeSpace = (((*infoSector).count[0 as i32 as usize] as i32
                + (((*infoSector).count[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*infoSector).count)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*infoSector).count)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t;
            (*This).last = (((*infoSector).pos[0 as i32 as usize] as i32
                + (((*infoSector).pos[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*infoSector).pos)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*infoSector).pos)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t;
        }
        free(infoSector as *mut libc::c_void);
    }
    return (check_media_type(This, boot) != 0 || check_fat(This) != 0) as i32;
}
unsafe extern "C" fn old_fat_read(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
    mut nodups: i32,
) -> i32 {
    (*This).writeAllFats = 1 as i32 as uint32_t;
    (*This).primaryFat = 0 as i32 as uint32_t;
    (*This).dir_start = ((*This).fat_start as u32)
        .wrapping_add(((*This).num_fat as u32).wrapping_mul((*This).fat_len));
    (*This).infoSectorLoc = 0xffffffff as u32;
    if nodups != 0 {
        (*This).num_fat = 1 as i32 as uint8_t;
    }
    if check_media_type(This, boot) != 0 {
        return -(1 as i32);
    }
    if (*This).fat_bits == 16 as i32 as u32 {
        if mtools_skip_check == 0 && readByte(This, 3 as i32 as u32) != 0xff as i32 {
            return -(1 as i32);
        }
    }
    return check_fat(This);
}
#[no_mangle]
pub unsafe extern "C" fn fat_read(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
    mut nodups: i32,
) -> i32 {
    (*This).fat_error = 0 as i32;
    (*This).fat_dirty = 0 as i32;
    (*This).last = 0xffffffff as u32;
    (*This).freeSpace = 0xffffffff as u32;
    (*This).lastFatSectorNr = 0 as i32 as uint32_t;
    (*This).lastFatSectorData = 0 as *mut u8;
    if (*This).fat_bits >= 12 as i32 as u32 {} else {
        __assert_fail(
            b"This->fat_bits >= 12\0" as *const u8 as *const i8,
            b"fat.c\0" as *const u8 as *const i8,
            783 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[i8; 46],
            >(b"int fat_read(Fs_t *, union bootsector *, int)\0"))
                .as_ptr(),
        );
    }
    'c_11556: {
        if (*This).fat_bits >= 12 as i32 as u32 {} else {
            __assert_fail(
                b"This->fat_bits >= 12\0" as *const u8 as *const i8,
                b"fat.c\0" as *const u8 as *const i8,
                783 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[i8; 46],
                >(b"int fat_read(Fs_t *, union bootsector *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*This).fat_bits <= 16 as i32 as u32 {
        return old_fat_read(This, boot, nodups)
    } else {
        return fat_32_read(This, boot)
    };
}
#[no_mangle]
pub unsafe extern "C" fn fatDecode(mut This: *mut Fs_t, mut pos: u32) -> u32 {
    let mut ret: u32 = 0;
    ret = ((*This).fat_decode).expect("non-null function pointer")(This, pos);
    if ret != 0
        && (ret < 2 as i32 as u32
            || ret > ((*This).num_clus).wrapping_add(1 as i32 as u32))
        && ret < (*This).last_fat
    {
        fprintf(
            stderr,
            b"Bad FAT entry %d at %d\n\0" as *const u8 as *const i8,
            ret,
            pos,
        );
        (*This).fat_error += 1;
        (*This).fat_error;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fatAppend(mut This: *mut Fs_t, mut pos: u32, mut newpos: u32) {
    ((*This).fat_encode).expect("non-null function pointer")(This, pos, newpos);
    ((*This).fat_encode)
        .expect("non-null function pointer")(This, newpos, (*This).end_fat);
    if (*This).freeSpace != 0xffffffff as u32 {
        (*This).freeSpace = ((*This).freeSpace).wrapping_sub(1);
        (*This).freeSpace;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fatDeallocate(mut This: *mut Fs_t, mut pos: u32) {
    ((*This).fat_encode).expect("non-null function pointer")(This, pos, 0 as i32 as u32);
    if (*This).freeSpace != 0xffffffff as u32 {
        (*This).freeSpace = ((*This).freeSpace).wrapping_add(1);
        (*This).freeSpace;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fatAllocate(mut This: *mut Fs_t, mut pos: u32, mut value: u32) {
    ((*This).fat_encode).expect("non-null function pointer")(This, pos, value);
    if (*This).freeSpace != 0xffffffff as u32 {
        (*This).freeSpace = ((*This).freeSpace).wrapping_sub(1);
        (*This).freeSpace;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fatEncode(mut This: *mut Fs_t, mut pos: u32, mut value: u32) {
    let mut oldvalue: u32 = ((*This).fat_decode)
        .expect("non-null function pointer")(This, pos);
    ((*This).fat_encode).expect("non-null function pointer")(This, pos, value);
    if (*This).freeSpace != 0xffffffff as u32 {
        if oldvalue != 0 {
            (*This).freeSpace = ((*This).freeSpace).wrapping_add(1);
            (*This).freeSpace;
        }
        if value != 0 {
            (*This).freeSpace = ((*This).freeSpace).wrapping_sub(1);
            (*This).freeSpace;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_next_free_cluster(
    mut This: *mut Fs_t,
    mut last: u32,
) -> u32 {
    let mut current_block: u64;
    let mut i: u32 = 0;
    if (*This).last != 0xffffffff as u32 {
        last = (*This).last;
    }
    if last < 2 as i32 as u32 || last >= ((*This).num_clus).wrapping_add(1 as i32 as u32)
    {
        last = 1 as i32 as u32;
    }
    i = last.wrapping_add(1 as i32 as u32);
    loop {
        if !(i < ((*This).num_clus).wrapping_add(2 as i32 as u32)) {
            current_block = 1917311967535052937;
            break;
        }
        let mut r: u32 = fatDecode(This, i);
        if r == 1 as i32 as u32 {
            current_block = 4851421062151579398;
            break;
        }
        if r == 0 {
            (*This).last = i;
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        1917311967535052937 => {
            i = 2 as i32 as u32;
            loop {
                if !(i < last.wrapping_add(1 as i32 as u32)) {
                    current_block = 7149356873433890176;
                    break;
                }
                let mut r_0: u32 = fatDecode(This, i);
                if r_0 == 1 as i32 as u32 {
                    current_block = 4851421062151579398;
                    break;
                }
                if r_0 == 0 {
                    (*This).last = i;
                    return i;
                }
                i = i.wrapping_add(1);
                i;
            }
            match current_block {
                4851421062151579398 => {}
                _ => {
                    fprintf(
                        stderr,
                        b"No free cluster %d %d\n\0" as *const u8 as *const i8,
                        (*This).preallocatedClusters,
                        (*This).last,
                    );
                    return 1 as i32 as u32;
                }
            }
        }
        _ => {}
    }
    fprintf(stderr, b"FAT error\n\0" as *const u8 as *const i8);
    return 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn getSerialized(mut Fs: *mut Fs_t) -> bool {
    return (*Fs).serialized != 0;
}
#[no_mangle]
pub unsafe extern "C" fn getSerialNumber(mut Fs: *mut Fs_t) -> u64 {
    return (*Fs).serial_number;
}
#[no_mangle]
pub unsafe extern "C" fn getClusterBytes(mut Fs: *mut Fs_t) -> uint32_t {
    return ((*Fs).cluster_size as i32 * (*Fs).sector_size as i32) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn fat_error(mut Dir: *mut Stream_t) -> i32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).fat_error != 0 {
        fprintf(stderr, b"Fat error detected\n\0" as *const u8 as *const i8);
    }
    return (*This).fat_error;
}
#[no_mangle]
pub unsafe extern "C" fn fat32RootCluster(mut Dir: *mut Stream_t) -> uint32_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).fat_bits == 32 as i32 as u32 {
        return (*This).rootCluster
    } else {
        return 0 as i32 as uint32_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn getfree(mut Dir: *mut Stream_t) -> mt_off_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).freeSpace == 0xffffffff as u32 || (*This).freeSpace == 0 as i32 as u32 {
        let mut i: u32 = 0;
        let mut total: uint32_t = 0;
        total = 0 as i64 as uint32_t;
        i = 2 as i32 as u32;
        while i < ((*This).num_clus).wrapping_add(2 as i32 as u32) {
            let mut r: u32 = fatDecode(This, i);
            if r == 1 as i32 as u32 {
                return -(1 as i32) as mt_off_t;
            }
            if r == 0 {
                total = total.wrapping_add(1);
                total;
            }
            i = i.wrapping_add(1);
            i;
        }
        (*This).freeSpace = total;
    }
    return sectorsToBytes(
        This,
        ((*This).freeSpace).wrapping_mul((*This).cluster_size as u32),
    );
}
#[no_mangle]
pub unsafe extern "C" fn getfreeMinClusters(
    mut Dir: *mut Stream_t,
    mut size: uint32_t,
) -> i32 {
    let mut current_block: u64;
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut i: u32 = 0;
    let mut last: u32 = 0;
    let mut total: size_t = 0;
    if batchmode != 0 && (*This).freeSpace == 0xffffffff as u32 {
        getfree(Stream);
    }
    if (*This).freeSpace != 0xffffffff as u32 {
        if (*This).freeSpace >= size {
            return 1 as i32
        } else {
            fprintf(stderr, b"Disk full\n\0" as *const u8 as *const i8);
            got_signal = 1 as i32;
            return 0 as i32;
        }
    }
    total = 0 as i64 as size_t;
    last = (*This).last;
    if last < 2 as i32 as u32 || last >= ((*This).num_clus).wrapping_add(2 as i32 as u32)
    {
        last = 1 as i32 as u32;
    }
    i = last.wrapping_add(1 as i32 as u32);
    loop {
        if !(i < ((*This).num_clus).wrapping_add(2 as i32 as u32)) {
            current_block = 15904375183555213903;
            break;
        }
        let mut r: u32 = fatDecode(This, i);
        if r == 1 as i32 as u32 {
            current_block = 17538587610894112971;
            break;
        }
        if r == 0 {
            total = total.wrapping_add(1);
            total;
        }
        if total >= size as u64 {
            return 1 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        15904375183555213903 => {
            i = 2 as i32 as u32;
            loop {
                if !(i < last.wrapping_add(1 as i32 as u32)) {
                    current_block = 18386322304582297246;
                    break;
                }
                let mut r_0: u32 = fatDecode(This, i);
                if r_0 == 1 as i32 as u32 {
                    current_block = 17538587610894112971;
                    break;
                }
                if r_0 == 0 {
                    total = total.wrapping_add(1);
                    total;
                }
                if total >= size as u64 {
                    return 1 as i32;
                }
                i = i.wrapping_add(1);
                i;
            }
            match current_block {
                17538587610894112971 => {}
                _ => {
                    fprintf(stderr, b"Disk full\n\0" as *const u8 as *const i8);
                    got_signal = 1 as i32;
                    return 0 as i32;
                }
            }
        }
        _ => {}
    }
    fprintf(stderr, b"FAT error\n\0" as *const u8 as *const i8);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn getfreeMinBytes(
    mut Dir: *mut Stream_t,
    mut size: mt_off_t,
) -> i32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut size2: mt_off_t = 0;
    size2 = size / ((*This).sector_size as i32 * (*This).cluster_size as i32) as i64;
    if size % ((*This).sector_size as i32 * (*This).cluster_size as i32) as i64 != 0 {
        size2 += 1;
        size2;
    }
    if size2 > 4294967295 as u32 as i64 {
        fprintf(stderr, b"Requested size too big\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    return getfreeMinClusters(Dir, size2 as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn getStart(
    mut Dir: *mut Stream_t,
    mut dir: *mut directory,
) -> u32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut first: uint32_t = 0;
    first = ((*dir).start[0 as i32 as usize] as i32
        + (((*dir).start[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
        as uint32_t;
    if fat32RootCluster(Stream) != 0 {
        first
            |= (((*dir).startHi[0 as i32 as usize] as i32
                + (((*dir).startHi[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
                as uint32_t) << 16 as i32;
    }
    return first;
}
#[no_mangle]
pub unsafe extern "C" fn fs_free(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if !((*This).FatMap).is_null() {
        let mut i: i32 = 0;
        let mut nr_entries: i32 = 0;
        nr_entries = ((*This).fat_len as u64)
            .wrapping_add(
                (::core::mem::size_of::<fatBitMask>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            )
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(
                (::core::mem::size_of::<fatBitMask>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            ) as i32;
        i = 0 as i32;
        while i < nr_entries {
            if !((*((*This).FatMap).offset(i as isize)).data).is_null() {
                free((*((*This).FatMap).offset(i as isize)).data as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        free((*This).FatMap as *mut libc::c_void);
    }
    if !((*This).cp).is_null() {
        cp_close((*This).cp);
    }
    return 0 as i32;
}
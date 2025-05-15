use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    pub type dirCacheEntry_t;
    pub type FatMap_t;
    pub type hashtable;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn tzset();
    static mut timezone: i64;
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn mk_entry_from_base(
        base: *const i8,
        attr: u8,
        fat: u32,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn dir_grow(Dir: *mut Stream_t, size: u32) -> i32;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    static mut batchmode: i32;
    fn getStart(Dir: *mut Stream_t, dir: *mut directory) -> u32;
    fn truncMtOffTo32u(off: mt_off_t) -> uint32_t;
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn truncSizeTo32u(siz: size_t) -> uint32_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn fsReleasePreallocateClusters(Fs: *mut Fs_t, _: uint32_t);
    fn fsPreallocateClusters(Fs: *mut Fs_t, _: uint32_t) -> i32;
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn fatDecode(This: *mut Fs_t, pos: u32) -> u32;
    fn fatAllocate(This: *mut Fs_t, pos: u32, value: u32);
    fn fatAppend(This: *mut Fs_t, pos: u32, newpos: u32);
    fn get_next_free_cluster(Fs: *mut Fs_t, last: u32) -> u32;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
    fn isRootEntry(entry: *mut direntry_t) -> i32;
    fn dir_write(entry: *mut direntry_t);
    fn make_ht(
        f1: T_HashFunc,
        f2: T_HashFunc,
        c: T_ComparFunc,
        size: size_t,
        H: *mut *mut T_HashTable,
    ) -> i32;
    fn hash_add(H: *mut T_HashTable, E: *mut libc::c_void, hint: *mut size_t) -> i32;
    fn hash_remove(H: *mut T_HashTable, E: *mut libc::c_void, hint: size_t) -> i32;
    fn hash_lookup(
        H: *mut T_HashTable,
        E: *mut libc::c_void,
        E2: *mut *mut libc::c_void,
        hint: *mut size_t,
    ) -> i32;
    fn freeDirCache(Stream: *mut Stream_t);
    fn buf_init(
        Next: *mut Stream_t,
        size: size_t,
        cylinderSize: size_t,
        sectorSize: size_t,
    ) -> *mut Stream_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirCache_t {
    pub entries: *mut *mut dirCacheEntry_t,
    pub nr_entries: u32,
    pub nrHashed: u32,
    pub bm0: [u32; 128],
    pub bm1: [u32; 128],
    pub bm2: [u32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct File_t {
    pub head: Stream_t,
    pub Buffer: *mut Stream_t,
    pub map: Option<
        unsafe extern "C" fn(
            *mut File_t,
            uint32_t,
            *mut uint32_t,
            i32,
            *mut mt_off_t,
        ) -> i32,
    >,
    pub FileSize: uint32_t,
    pub preallocatedSize: uint32_t,
    pub preallocatedClusters: uint32_t,
    pub FirstAbsCluNr: u32,
    pub PreviousAbsCluNr: u32,
    pub PreviousRelCluNr: u32,
    pub direntry: direntry_t,
    pub hint: size_t,
    pub dcp: *mut dirCache_t,
    pub loopDetectRel: u32,
    pub loopDetectAbs: u32,
    pub where_0: uint32_t,
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
pub type T_HashTable = hashtable;
pub type T_ComparFunc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
>;
pub type T_HashFunc = Option<unsafe extern "C" fn(*mut libc::c_void) -> uint32_t>;
#[inline]
unsafe extern "C" fn set_word(mut data: *mut u8, mut value: libc::c_ushort) {
    *data.offset(1 as i32 as isize) = (value as i32 >> 8 as i32 & 0xff as i32) as u8;
    *data.offset(0 as i32 as isize) = (value as i32 >> 0 as i32 & 0xff as i32) as u8;
}
static mut filehash: *mut T_HashTable = 0 as *const T_HashTable as *mut T_HashTable;
unsafe extern "C" fn getUnbufferedFile(mut Stream: *mut Stream_t) -> *mut File_t {
    while (*Stream).Class != &mut FileClass as *mut Class_t {
        Stream = (*Stream).Next;
    }
    return Stream as *mut File_t;
}
#[inline]
unsafe extern "C" fn mt_getFs(mut File: *mut File_t) -> *mut Fs_t {
    return (*File).head.Next as *mut Fs_t;
}
#[no_mangle]
pub unsafe extern "C" fn getFs(mut Stream: *mut Stream_t) -> *mut Fs_t {
    return (*getUnbufferedFile(Stream)).head.Next as *mut Fs_t;
}
#[no_mangle]
pub unsafe extern "C" fn getDirCacheP(
    mut Stream: *mut Stream_t,
) -> *mut *mut dirCache_t {
    return &mut (*(getUnbufferedFile
        as unsafe extern "C" fn(*mut Stream_t) -> *mut File_t)(Stream))
        .dcp;
}
#[no_mangle]
pub unsafe extern "C" fn getDirentry(mut Stream: *mut Stream_t) -> *mut direntry_t {
    return &mut (*(getUnbufferedFile
        as unsafe extern "C" fn(*mut Stream_t) -> *mut File_t)(Stream))
        .direntry;
}
unsafe extern "C" fn filebytesToClusters(
    mut bytes: uint32_t,
    mut clus_size: uint32_t,
) -> uint32_t {
    let mut ret: uint32_t = bytes.wrapping_div(clus_size);
    if bytes.wrapping_rem(clus_size) != 0 {
        ret = ret.wrapping_add(1);
        ret;
    }
    return ret;
}
unsafe extern "C" fn recalcPreallocSize(mut This: *mut File_t) -> i32 {
    let mut currentClusters: uint32_t = 0;
    let mut neededClusters: uint32_t = 0;
    let mut clus_size: u32 = 0;
    let mut neededPrealloc: uint32_t = 0;
    let mut Fs: *mut Fs_t = mt_getFs(This);
    clus_size = ((*Fs).cluster_size as i32 * (*Fs).sector_size as i32) as u32;
    currentClusters = filebytesToClusters((*This).FileSize, clus_size);
    neededClusters = filebytesToClusters((*This).preallocatedSize, clus_size);
    if neededClusters < currentClusters {
        neededPrealloc = 0 as i32 as uint32_t;
    } else {
        neededPrealloc = neededClusters.wrapping_sub(currentClusters);
    }
    if neededPrealloc > (*This).preallocatedClusters {
        let mut r: i32 = fsPreallocateClusters(
            Fs,
            neededPrealloc.wrapping_sub((*This).preallocatedClusters),
        );
        if r != 0 {
            return r;
        }
    } else {
        fsReleasePreallocateClusters(
            Fs,
            ((*This).preallocatedClusters).wrapping_sub(neededPrealloc),
        );
    }
    (*This).preallocatedClusters = neededPrealloc;
    return 0 as i32;
}
unsafe extern "C" fn mt_loopDetect(
    mut oldrel: *mut u32,
    mut rel: u32,
    mut oldabs: *mut u32,
    mut absol: u32,
) -> i32 {
    if *oldrel != 0 && rel > *oldrel && absol == *oldabs {
        fprintf(
            stderr,
            b"loop detected! oldrel=%d newrel=%d abs=%d\n\0" as *const u8 as *const i8,
            *oldrel,
            rel,
            absol,
        );
        return -(1 as i32);
    }
    if rel >= (2 as i32 as u32).wrapping_mul(*oldrel).wrapping_add(1 as i32 as u32) {
        *oldrel = rel;
        *oldabs = absol;
    }
    return 0 as i32;
}
unsafe extern "C" fn loopDetect(
    mut This: *mut File_t,
    mut rel: u32,
    mut absol: u32,
) -> i32 {
    return mt_loopDetect(
        &mut (*This).loopDetectRel,
        rel,
        &mut (*This).loopDetectAbs,
        absol,
    );
}
unsafe extern "C" fn mt_countBlocks(mut This: *mut Fs_t, mut block: u32) -> u32 {
    let mut blocks: u32 = 0;
    let mut rel: u32 = 0;
    let mut oldabs: u32 = 0;
    let mut oldrel: u32 = 0;
    blocks = 0 as i32 as u32;
    rel = 0 as i32 as u32;
    oldrel = rel;
    oldabs = oldrel;
    while block <= (*This).last_fat && block != 1 as i32 as u32 && block != 0 {
        blocks = blocks.wrapping_add(1);
        blocks;
        block = fatDecode(This, block);
        rel = rel.wrapping_add(1);
        rel;
        if mt_loopDetect(&mut oldrel, rel, &mut oldabs, block) < 0 as i32 {
            block = 1 as i32 as u32;
        }
    }
    return blocks;
}
#[no_mangle]
pub unsafe extern "C" fn countBlocks(mut Dir: *mut Stream_t, mut block: u32) -> u32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    return mt_countBlocks(This, block);
}
unsafe extern "C" fn countBytes(mut Dir: *mut Stream_t, mut block: u32) -> uint32_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    return (mt_countBlocks(This, block))
        .wrapping_mul((*This).sector_size as u32)
        .wrapping_mul((*This).cluster_size as u32);
}
#[no_mangle]
pub unsafe extern "C" fn printFat(mut Stream: *mut Stream_t) {
    let mut This: *mut File_t = getUnbufferedFile(Stream);
    let mut n: uint32_t = 0;
    let mut rel: u32 = 0;
    let mut begin: u64 = 0;
    let mut end: u64 = 0;
    let mut first: i32 = 0;
    n = (*This).FirstAbsCluNr;
    if n == 0 {
        printf(b"Root directory or empty file\n\0" as *const u8 as *const i8);
        return;
    }
    rel = 0 as i32 as u32;
    first = 1 as i32;
    end = 0 as i32 as u64;
    begin = end;
    loop {
        if first != 0 || n as u64 != end.wrapping_add(1 as i32 as u64) {
            if first == 0 {
                if begin != end {
                    printf(b"-%lu\0" as *const u8 as *const i8, end);
                }
                printf(b"> \0" as *const u8 as *const i8);
            }
            end = n as u64;
            begin = end;
            printf(b"<%lu\0" as *const u8 as *const i8, begin);
        } else {
            end = end.wrapping_add(1);
            end;
        }
        first = 0 as i32;
        n = fatDecode(mt_getFs(This), n);
        rel = rel.wrapping_add(1);
        rel;
        if loopDetect(This, rel, n) < 0 as i32 {
            n = 1 as i32 as uint32_t;
        }
        if !(n <= (*mt_getFs(This)).last_fat && n != 1 as i32 as u32) {
            break;
        }
    }
    if first == 0 {
        if begin != end {
            printf(b"-%lu\0" as *const u8 as *const i8, end);
        }
        printf(b">\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn printFatWithOffset(
    mut Stream: *mut Stream_t,
    mut offset: off_t,
) {
    let mut This: *mut File_t = getUnbufferedFile(Stream);
    let mut n: uint32_t = 0;
    let mut rel: u32 = 0;
    let mut clusSize: off_t = 0;
    n = (*This).FirstAbsCluNr;
    if n == 0 {
        printf(b"Root directory or empty file\n\0" as *const u8 as *const i8);
        return;
    }
    clusSize = ((*mt_getFs(This)).cluster_size as i32
        * (*mt_getFs(This)).sector_size as i32) as off_t;
    rel = 0 as i32 as u32;
    while offset >= clusSize {
        n = fatDecode(mt_getFs(This), n);
        rel = rel.wrapping_add(1);
        rel;
        if loopDetect(This, rel, n) < 0 as i32 {
            return;
        }
        if n > (*mt_getFs(This)).last_fat {
            return;
        }
        offset -= clusSize;
    }
    printf(b"%lu\0" as *const u8 as *const i8, n as u64);
}
unsafe extern "C" fn normal_map(
    mut This: *mut File_t,
    mut where_0: uint32_t,
    mut len: *mut uint32_t,
    mut isReadonly: i32,
    mut res: *mut mt_off_t,
) -> i32 {
    let mut offset: u32 = 0;
    let mut end: size_t = 0;
    let mut NrClu: uint32_t = 0;
    let mut RelCluNr: uint32_t = 0;
    let mut CurCluNr: uint32_t = 0;
    let mut NewCluNr: uint32_t = 0;
    let mut AbsCluNr: uint32_t = 0;
    let mut clus_size: uint32_t = 0;
    let mut Fs: *mut Fs_t = mt_getFs(This);
    *res = 0 as i32 as mt_off_t;
    clus_size = ((*Fs).cluster_size as i32 * (*Fs).sector_size as i32) as uint32_t;
    offset = where_0.wrapping_rem(clus_size);
    if isReadonly != 0 {
        if *len > ((*This).FileSize).wrapping_sub(where_0) {
            *len = ((*This).FileSize).wrapping_sub(where_0);
        }
    }
    if *len == 0 as i32 as u32 {
        return 0 as i32;
    }
    if (*This).FirstAbsCluNr < 2 as i32 as u32 {
        if isReadonly != 0 || *len == 0 as i32 as u32 {
            *len = 0 as i32 as uint32_t;
            return 0 as i32;
        }
        NewCluNr = get_next_free_cluster(mt_getFs(This), 1 as i32 as u32);
        if NewCluNr == 1 as i32 as u32 {
            *__errno_location() = 28 as i32;
            return -(2 as i32);
        }
        hash_remove(filehash, This as *mut libc::c_void, (*This).hint);
        (*This).FirstAbsCluNr = NewCluNr;
        hash_add(filehash, This as *mut libc::c_void, &mut (*This).hint);
        fatAllocate(mt_getFs(This), NewCluNr, (*Fs).end_fat);
    }
    RelCluNr = where_0.wrapping_div(clus_size);
    if RelCluNr >= (*This).PreviousRelCluNr {
        CurCluNr = (*This).PreviousRelCluNr;
        AbsCluNr = (*This).PreviousAbsCluNr;
    } else {
        CurCluNr = 0 as i32 as uint32_t;
        AbsCluNr = (*This).FirstAbsCluNr;
    }
    NrClu = offset
        .wrapping_add(*len)
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div(clus_size);
    while CurCluNr <= RelCluNr.wrapping_add(NrClu) {
        if CurCluNr == RelCluNr {
            (*This).PreviousRelCluNr = RelCluNr;
            (*This).PreviousAbsCluNr = AbsCluNr;
        }
        NewCluNr = fatDecode(mt_getFs(This), AbsCluNr);
        if NewCluNr == 1 as i32 as u32 || NewCluNr == 0 as i32 as u32 {
            fprintf(
                stderr,
                b"Fat problem while decoding %d %x\n\0" as *const u8 as *const i8,
                AbsCluNr,
                NewCluNr,
            );
            exit(1 as i32);
        }
        if CurCluNr == RelCluNr.wrapping_add(NrClu) {
            break;
        }
        if NewCluNr > (*Fs).last_fat && isReadonly == 0 {
            NewCluNr = get_next_free_cluster(mt_getFs(This), AbsCluNr);
            if NewCluNr == 1 as i32 as u32 {
                *__errno_location() = 28 as i32;
                return -(2 as i32);
            }
            fatAppend(mt_getFs(This), AbsCluNr, NewCluNr);
        }
        if CurCluNr < RelCluNr && NewCluNr > (*Fs).last_fat {
            *len = 0 as i32 as uint32_t;
            return 0 as i32;
        }
        if CurCluNr >= RelCluNr && NewCluNr != AbsCluNr.wrapping_add(1 as i32 as u32) {
            break;
        }
        CurCluNr = CurCluNr.wrapping_add(1);
        CurCluNr;
        AbsCluNr = NewCluNr;
        if loopDetect(This, CurCluNr, AbsCluNr) != 0 {
            *__errno_location() = 5 as i32;
            return -(2 as i32);
        }
    }
    if *len
        > (1 as i32 as u32)
            .wrapping_add(CurCluNr)
            .wrapping_sub(RelCluNr)
            .wrapping_mul(clus_size)
            .wrapping_sub(offset)
    {
        *len = (1 as i32 as u32)
            .wrapping_add(CurCluNr)
            .wrapping_sub(RelCluNr)
            .wrapping_mul(clus_size)
            .wrapping_sub(offset);
    }
    end = where_0.wrapping_add(*len) as size_t;
    if batchmode != 0 && isReadonly == 0 && end >= (*This).FileSize as u64 {
        *len = (*len as u64)
            .wrapping_add(
                end
                    .wrapping_add(clus_size as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_sub(
                        end
                            .wrapping_add(clus_size as u64)
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_rem(clus_size as u64),
                    )
                    .wrapping_sub(end),
            ) as uint32_t as uint32_t;
    }
    if (*len)
        .wrapping_add(offset)
        .wrapping_div(clus_size)
        .wrapping_add((*This).PreviousAbsCluNr)
        .wrapping_sub(2 as i32 as u32) > (*Fs).num_clus
    {
        fprintf(stderr, b"cluster too big\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    *res = sectorsToBytes(
        Fs,
        ((*This).PreviousAbsCluNr)
            .wrapping_sub(2 as i32 as u32)
            .wrapping_mul((*Fs).cluster_size as u32)
            .wrapping_add((*Fs).clus_start),
    ) + offset as i64;
    return 1 as i32;
}
unsafe extern "C" fn root_map(
    mut This: *mut File_t,
    mut where_0: uint32_t,
    mut len: *mut uint32_t,
    mut mode: i32,
    mut res: *mut mt_off_t,
) -> i32 {
    let mut Fs: *mut Fs_t = mt_getFs(This);
    if ((*Fs).dir_len as u32)
        .wrapping_add(0 as u32)
        .wrapping_mul((*Fs).sector_size as u32) < where_0
    {
        *len = 0 as i32 as uint32_t;
        *__errno_location() = 28 as i32;
        return -(2 as i32);
    }
    if *len
        > (((*Fs).dir_len as i32 * (*Fs).sector_size as i32) as u32)
            .wrapping_sub(where_0)
    {
        *len = (((*Fs).dir_len as i32 * (*Fs).sector_size as i32) as u32)
            .wrapping_sub(where_0);
    }
    if *len == 0 as i32 as u32 {
        return 0 as i32;
    }
    *res = sectorsToBytes(Fs, (*Fs).dir_start) + where_0 as i64;
    return 1 as i32;
}
unsafe extern "C" fn read_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut pos: mt_off_t = 0;
    let mut err: i32 = 0;
    let mut len: uint32_t = truncSizeTo32u(ilen);
    let mut ret: ssize_t = 0;
    let mut Disk: *mut Stream_t = (*mt_getFs(This)).head.Next;
    err = ((*This).map)
        .expect(
            "non-null function pointer",
        )(This, (*This).where_0, &mut len, 1 as i32, &mut pos);
    if err <= 0 as i32 {
        return err as ssize_t;
    }
    ret = ((*(*Disk).Class).pread)
        .expect("non-null function pointer")(Disk, buf, pos, len as size_t);
    if ret < 0 as i32 as i64 {
        return ret;
    }
    (*This).where_0 = ((*This).where_0 as u64).wrapping_add(ret as size_t) as uint32_t
        as uint32_t;
    return ret;
}
unsafe extern "C" fn write_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut pos: mt_off_t = 0;
    let mut ret: ssize_t = 0;
    let mut requestedLen: uint32_t = 0;
    let mut bytesWritten: uint32_t = 0;
    let mut Disk: *mut Stream_t = (*mt_getFs(This)).head.Next;
    let mut maxLen: uint32_t = (4294967295 as u32).wrapping_sub((*This).where_0);
    let mut len: uint32_t = 0;
    let mut err: i32 = 0;
    if ilen > maxLen as u64 {
        len = maxLen;
    } else {
        len = ilen as uint32_t;
    }
    requestedLen = len;
    err = ((*This).map)
        .expect(
            "non-null function pointer",
        )(This, (*This).where_0, &mut len, 0 as i32, &mut pos);
    if err <= 0 as i32 {
        return err as ssize_t;
    }
    if batchmode != 0 {
        ret = force_pwrite(Disk, buf, pos, len as size_t);
    } else {
        ret = ((*(*Disk).Class).pwrite)
            .expect("non-null function pointer")(Disk, buf, pos, len as size_t);
    }
    if ret < 0 as i32 as i64 {
        return ret;
    }
    if ret as uint32_t > requestedLen {
        bytesWritten = requestedLen;
    } else {
        bytesWritten = ret as uint32_t;
    }
    (*This).where_0 = ((*This).where_0 as u32).wrapping_add(bytesWritten) as uint32_t
        as uint32_t;
    if (*This).where_0 > (*This).FileSize {
        (*This).FileSize = (*This).where_0;
    }
    recalcPreallocSize(This);
    return bytesWritten as ssize_t;
}
unsafe extern "C" fn pread_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    (*This).where_0 = truncMtOffTo32u(where_0);
    return read_file(Stream, buf, ilen);
}
unsafe extern "C" fn pwrite_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    (*This).where_0 = truncMtOffTo32u(where_0);
    return write_file(Stream, buf, ilen);
}
static mut month: [i32; 15] = [
    0 as i32,
    31 as i32,
    59 as i32,
    90 as i32,
    120 as i32,
    151 as i32,
    181 as i32,
    212 as i32,
    243 as i32,
    273 as i32,
    304 as i32,
    334 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
];
#[inline]
unsafe extern "C" fn conv_stamp(mut dir: *mut directory) -> time_t {
    let mut tmbuf: *mut tm = 0 as *mut tm;
    let mut tzone: i64 = 0;
    let mut dst: i64 = 0;
    let mut accum: time_t = 0;
    let mut tmp: time_t = 0;
    accum = (((*dir).date[1 as i32 as usize] as i32 >> 1 as i32) + 1980 as i32
        - 1970 as i32) as time_t;
    accum = accum * 365 as i64
        + month[((((*dir).date[1 as i32 as usize] as i32 & 0x1 as i32) << 3 as i32)
            + ((*dir).date[0 as i32 as usize] as i32 >> 5 as i32) - 1 as i32) as usize]
            as i64 + ((*dir).date[0 as i32 as usize] as i32 & 0x1f as i32) as i64;
    accum
        += (((*dir).date[1 as i32 as usize] as i32 >> 1 as i32) + 1980 as i32
            - 1972 as i32) as i64 / 4 as i64;
    if (((*dir).date[1 as i32 as usize] as i32 >> 1 as i32) + 1980 as i32) % 4 as i32
        == 0
        && (((*dir).date[1 as i32 as usize] as i32 & 0x1 as i32) << 3 as i32)
            + ((*dir).date[0 as i32 as usize] as i32 >> 5 as i32) < 3 as i32
    {
        accum -= 1;
        accum;
    }
    accum = accum * 24 as i64
        + ((*dir).time[1 as i32 as usize] as i32 >> 3 as i32) as i64;
    accum = accum * 60 as i64
        + ((((*dir).time[1 as i32 as usize] as i32 & 0x7 as i32) << 3 as i32)
            + ((*dir).time[0 as i32 as usize] as i32 >> 5 as i32)) as i64;
    accum = accum * 60 as i64
        + (((*dir).time[0 as i32 as usize] as i32 & 0x1f as i32) * 2 as i32) as i64;
    extern "C" {
        #[link_name = "timezone"]
        static mut timezone_0: i64;
    }
    tzset();
    tzone = timezone;
    accum += tzone;
    tmp = accum;
    tmbuf = localtime(&mut tmp);
    if !tmbuf.is_null() {
        dst = if (*tmbuf).tm_isdst != 0 { -(60 as i64) * 60 as i64 } else { 0 as i64 };
        accum += dst;
    }
    return accum;
}
unsafe extern "C" fn get_file_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut uint32_t,
) -> i32 {
    let mut This: *mut File_t = Stream as *mut File_t;
    if !date.is_null() {
        *date = conv_stamp(&mut (*This).direntry.dir);
    }
    if !size.is_null() {
        *size = (*This).FileSize as mt_off_t;
    }
    if !type_0.is_null() {
        *type_0 = (*This).direntry.dir.attr as i32 & 0x10 as i32;
    }
    if !address.is_null() {
        *address = (*This).FirstAbsCluNr;
    }
    return 0 as i32;
}
unsafe extern "C" fn free_file(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut Fs: *mut Fs_t = mt_getFs(This);
    fsReleasePreallocateClusters(Fs, (*This).preallocatedClusters);
    free_stream(&mut (*This).direntry.Dir);
    freeDirCache(Stream);
    return hash_remove(filehash, Stream as *mut libc::c_void, (*This).hint);
}
unsafe extern "C" fn flush_file(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut entry: *mut direntry_t = &mut (*This).direntry;
    if isRootDir(Stream) != 0 {
        return 0 as i32;
    }
    if (*This).FirstAbsCluNr != getStart((*entry).Dir, &mut (*entry).dir) {
        set_word(
            ((*entry).dir.start).as_mut_ptr(),
            ((*This).FirstAbsCluNr & 0xffff as i32 as u32) as libc::c_ushort,
        );
        set_word(
            ((*entry).dir.startHi).as_mut_ptr(),
            ((*This).FirstAbsCluNr >> 16 as i32) as libc::c_ushort,
        );
        dir_write(entry);
    }
    return 0 as i32;
}
unsafe extern "C" fn pre_allocate_file(
    mut Stream: *mut Stream_t,
    mut isize: mt_off_t,
) -> i32 {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut size: uint32_t = truncMtOffTo32u(isize);
    if size > (*This).FileSize && size > (*This).preallocatedSize {
        (*This).preallocatedSize = size;
        return recalcPreallocSize(This);
    } else {
        return 0 as i32
    };
}
static mut FileClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: Some(
                read_file
                    as unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t,
            ),
            write: Some(
                write_file
                    as unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t,
            ),
            pread: Some(
                pread_file
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                pwrite_file
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(flush_file as unsafe extern "C" fn(*mut Stream_t) -> i32),
            freeFunc: Some(free_file as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: None,
            get_data: Some(
                get_file_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut i32,
                        *mut uint32_t,
                    ) -> i32,
            ),
            pre_allocate: Some(
                pre_allocate_file as unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32,
            ),
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
unsafe extern "C" fn getAbsCluNr(mut This: *mut File_t) -> u32 {
    if (*This).FirstAbsCluNr != 0 {
        return (*This).FirstAbsCluNr;
    }
    if isRootDir(This as *mut Stream_t) != 0 {
        return 0 as i32 as u32;
    }
    return 1 as i32 as u32;
}
unsafe extern "C" fn func1(mut Stream: *mut libc::c_void) -> uint32_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    return getAbsCluNr(This) ^ (*This).head.Next as u64 as uint32_t;
}
unsafe extern "C" fn func2(mut Stream: *mut libc::c_void) -> uint32_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    return getAbsCluNr(This);
}
unsafe extern "C" fn comp(
    mut Stream: *mut libc::c_void,
    mut Stream2: *mut libc::c_void,
) -> i32 {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut This2: *mut File_t = Stream2 as *mut File_t;
    return (mt_getFs(This) != mt_getFs(This2) || getAbsCluNr(This) != getAbsCluNr(This2))
        as i32;
}
unsafe extern "C" fn init_hash() {
    static mut is_initialised: i32 = 0 as i32;
    if is_initialised == 0 {
        make_ht(
            Some(func1 as unsafe extern "C" fn(*mut libc::c_void) -> uint32_t),
            Some(func2 as unsafe extern "C" fn(*mut libc::c_void) -> uint32_t),
            Some(
                comp as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
            ),
            20 as i32 as size_t,
            &mut filehash,
        );
        is_initialised = 1 as i32;
    }
}
unsafe extern "C" fn mt_internalFileOpen(
    mut Dir: *mut Stream_t,
    mut first: u32,
    mut size: uint32_t,
    mut entry: *mut direntry_t,
) -> *mut Stream_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut Pattern: File_t = File_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        Buffer: 0 as *mut Stream_t,
        map: None,
        FileSize: 0,
        preallocatedSize: 0,
        preallocatedClusters: 0,
        FirstAbsCluNr: 0,
        PreviousAbsCluNr: 0,
        PreviousRelCluNr: 0,
        direntry: direntry_t {
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
        },
        hint: 0,
        dcp: 0 as *mut dirCache_t,
        loopDetectRel: 0,
        loopDetectAbs: 0,
        where_0: 0,
    };
    let mut File: *mut File_t = 0 as *mut File_t;
    init_hash();
    (*This).head.refs += 1;
    (*This).head.refs;
    if first != 1 as i32 as u32 {
        let mut Result: *mut libc::c_void = 0 as *mut libc::c_void;
        init_head(&mut Pattern.head, &mut FileClass, &mut (*This).head);
        if first != 0 || !entry.is_null() && (*entry).dir.attr as i32 & 0x10 as i32 == 0
        {
            Pattern.map = Some(
                normal_map
                    as unsafe extern "C" fn(
                        *mut File_t,
                        uint32_t,
                        *mut uint32_t,
                        i32,
                        *mut mt_off_t,
                    ) -> i32,
            );
        } else {
            Pattern.map = Some(
                root_map
                    as unsafe extern "C" fn(
                        *mut File_t,
                        uint32_t,
                        *mut uint32_t,
                        i32,
                        *mut mt_off_t,
                    ) -> i32,
            );
        }
        Pattern.FirstAbsCluNr = first;
        Pattern.loopDetectRel = 0 as i32 as u32;
        Pattern.loopDetectAbs = first;
        if hash_lookup(
            filehash,
            &mut Pattern as *mut File_t as *mut libc::c_void,
            &mut Result,
            0 as *mut size_t,
        ) == 0
        {
            File = Result as *mut File_t;
            (*File).head.refs += 1;
            (*File).head.refs;
            (*This).head.refs -= 1;
            (*This).head.refs;
            return File as *mut Stream_t;
        }
    }
    File = calloc(1 as i32 as u64, ::core::mem::size_of::<File_t>() as u64)
        as *mut File_t;
    if File.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*File).head, &mut FileClass, &mut (*This).head);
    (*File).Buffer = 0 as *mut Stream_t;
    (*File).dcp = 0 as *mut dirCache_t;
    (*File).preallocatedClusters = 0 as i32 as uint32_t;
    (*File).preallocatedSize = 0 as i32 as uint32_t;
    (*File).direntry = *entry;
    if isRootEntry(entry) != 0 {
        (*File).direntry.Dir = File as *mut Stream_t;
    } else {
        copy_stream((*File).direntry.Dir);
    }
    (*File).where_0 = 0 as i32 as uint32_t;
    if first != 0 || !entry.is_null() && (*entry).dir.attr as i32 & 0x10 as i32 == 0 {
        (*File).map = Some(
            normal_map
                as unsafe extern "C" fn(
                    *mut File_t,
                    uint32_t,
                    *mut uint32_t,
                    i32,
                    *mut mt_off_t,
                ) -> i32,
        );
    } else {
        (*File).map = Some(
            root_map
                as unsafe extern "C" fn(
                    *mut File_t,
                    uint32_t,
                    *mut uint32_t,
                    i32,
                    *mut mt_off_t,
                ) -> i32,
        );
    }
    if first == 1 as i32 as u32 {
        (*File).FirstAbsCluNr = 0 as i32 as u32;
    } else {
        (*File).FirstAbsCluNr = first;
    }
    (*File).loopDetectRel = 0 as i32 as u32;
    (*File).loopDetectAbs = 0 as i32 as u32;
    (*File).PreviousRelCluNr = 0xffff as i32 as u32;
    (*File).FileSize = size;
    hash_add(filehash, File as *mut libc::c_void, &mut (*File).hint);
    return File as *mut Stream_t;
}
unsafe extern "C" fn bufferize(mut Dir: *mut *mut Stream_t) {
    let mut BDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut file: *mut File_t = *Dir as *mut File_t;
    if (*Dir).is_null() {
        return;
    }
    if !((*file).Buffer).is_null() {
        (**Dir).refs -= 1;
        (**Dir).refs;
        (*(*file).Buffer).refs += 1;
        (*(*file).Buffer).refs;
        *Dir = (*file).Buffer;
        return;
    }
    BDir = buf_init(
        *Dir,
        16384 as i32 as size_t,
        512 as i32 as size_t,
        32 as i32 as size_t,
    );
    if BDir.is_null() {
        free_stream(Dir);
        *Dir = 0 as *mut Stream_t;
    } else {
        (*file).Buffer = BDir;
        *Dir = BDir;
    };
}
#[no_mangle]
pub unsafe extern "C" fn OpenRoot(mut Dir: *mut Stream_t) -> *mut Stream_t {
    let mut num: u32 = 0;
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
    let mut size: uint32_t = 0;
    let mut file: *mut Stream_t = 0 as *mut Stream_t;
    memset(
        &mut entry as *mut direntry_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<direntry_t>() as u64,
    );
    num = fat32RootCluster(Dir);
    entry.entry = -(3 as i32);
    entry.name[0 as i32 as usize] = '\0' as i32;
    mk_entry_from_base(
        b"/\0" as *const u8 as *const i8,
        0x10 as i32 as u8,
        num,
        0 as i32 as uint32_t,
        0 as i32 as time_t,
        &mut entry.dir,
    );
    if num != 0 {
        size = countBytes(Dir, num);
    } else {
        let mut Fs: *mut Fs_t = GetFs(Dir) as *mut Fs_t;
        size = ((*Fs).dir_len as i32 * (*Fs).sector_size as i32) as uint32_t;
    }
    file = mt_internalFileOpen(Dir, num, size, &mut entry);
    bufferize(&mut file);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn OpenFileByDirentry(
    mut entry: *mut direntry_t,
) -> *mut Stream_t {
    let mut file: *mut Stream_t = 0 as *mut Stream_t;
    let mut first: u32 = 0;
    let mut size: uint32_t = 0;
    first = getStart((*entry).Dir, &mut (*entry).dir);
    if first == 0 && (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
        return OpenRoot((*entry).Dir);
    }
    if (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
        size = countBytes((*entry).Dir, first);
    } else {
        size = (((*entry).dir.size[0 as i32 as usize] as i32
            + (((*entry).dir.size[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32
            + (((*((*entry).dir.size)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*entry).dir.size)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t;
    }
    file = mt_internalFileOpen((*entry).Dir, first, size, entry);
    if (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
        bufferize(&mut file);
        if first == 1 as i32 as u32 {
            dir_grow(file, 0 as i32 as u32);
        }
    }
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn isRootDir(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut File_t = getUnbufferedFile(Stream);
    return ((*This).map
        == Some(
            root_map
                as unsafe extern "C" fn(
                    *mut File_t,
                    uint32_t,
                    *mut uint32_t,
                    i32,
                    *mut mt_off_t,
                ) -> i32,
        )) as i32;
}
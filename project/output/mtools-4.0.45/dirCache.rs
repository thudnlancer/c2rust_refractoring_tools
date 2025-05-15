use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn wcsdup(__s: *const wchar_t) -> *mut wchar_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn getDirCacheP(Stream: *mut Stream_t) -> *mut *mut dirCache_t;
    fn low_level_dir_write_end(Dir: *mut Stream_t, entry: i32);
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
pub type wint_t = u32;
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
pub struct dirCacheEntry_t {
    pub type_0: dirCacheEntryType_t,
    pub beginSlot: u32,
    pub endSlot: u32,
    pub shortName: *mut wchar_t,
    pub longName: *mut wchar_t,
    pub dir: directory,
    pub endMarkPos: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum dirCacheEntryType_t {
    DCET_FREE,
    DCET_USED,
    DCET_END,
}
impl dirCacheEntryType_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            dirCacheEntryType_t::DCET_FREE => 0,
            dirCacheEntryType_t::DCET_USED => 1,
            dirCacheEntryType_t::DCET_END => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> dirCacheEntryType_t {
        match value {
            0 => dirCacheEntryType_t::DCET_FREE,
            1 => dirCacheEntryType_t::DCET_USED,
            2 => dirCacheEntryType_t::DCET_END,
            _ => panic!("Invalid value for dirCacheEntryType_t: {}", value),
        }
    }
}
impl AddAssign<u32> for dirCacheEntryType_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for dirCacheEntryType_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for dirCacheEntryType_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for dirCacheEntryType_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for dirCacheEntryType_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for dirCacheEntryType_t {
    type Output = dirCacheEntryType_t;
    fn add(self, rhs: u32) -> dirCacheEntryType_t {
        dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for dirCacheEntryType_t {
    type Output = dirCacheEntryType_t;
    fn sub(self, rhs: u32) -> dirCacheEntryType_t {
        dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for dirCacheEntryType_t {
    type Output = dirCacheEntryType_t;
    fn mul(self, rhs: u32) -> dirCacheEntryType_t {
        dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for dirCacheEntryType_t {
    type Output = dirCacheEntryType_t;
    fn div(self, rhs: u32) -> dirCacheEntryType_t {
        dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for dirCacheEntryType_t {
    type Output = dirCacheEntryType_t;
    fn rem(self, rhs: u32) -> dirCacheEntryType_t {
        dirCacheEntryType_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn rol(mut arg: uint32_t, mut shift: i32) -> uint32_t {
    arg &= 0xffffffff as u32;
    return arg << shift | arg >> 32 as i32 - shift;
}
unsafe extern "C" fn calcHash(mut name: *mut wchar_t) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut i: u32 = 0;
    let mut c: wint_t = 0;
    hash = 0 as i32 as uint32_t;
    i = 0 as i32 as u32;
    while *name != 0 {
        hash = rol(hash, 5 as i32);
        c = towupper(*name as wint_t);
        hash
            ^= c.wrapping_mul(c.wrapping_add(2 as i32 as u32))
                ^ i.wrapping_mul(i.wrapping_add(2 as i32 as u32));
        hash &= 0xffffffff as u32;
        i = i.wrapping_add(1);
        i;
        name = name.offset(1);
        name;
    }
    hash = hash.wrapping_mul(hash.wrapping_add(2 as i32 as u32));
    hash ^= (hash & 0xfff as i32 as u32) << 12 as i32;
    hash ^= (hash & 0xff000 as i32 as u32) << 24 as i32;
    return hash;
}
unsafe extern "C" fn addBit(
    mut bitmap: *mut u32,
    mut hash: u32,
    mut checkOnly: i32,
) -> u32 {
    let mut bit: u32 = 0;
    let mut entry: i32 = 0;
    bit = (1 as u32)
        << (hash as u64)
            .wrapping_rem(
                (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
            );
    entry = (hash as u64)
        .wrapping_div(
            (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_rem(128 as i32 as u64) as i32;
    if checkOnly != 0 {
        return *bitmap.offset(entry as isize) & bit
    } else {
        *bitmap.offset(entry as isize) |= bit;
        return 1 as i32 as u32;
    };
}
unsafe extern "C" fn mt_addHash(
    mut cache: *mut dirCache_t,
    mut hash: u32,
    mut checkOnly: i32,
) -> i32 {
    return (addBit(((*cache).bm0).as_mut_ptr(), hash, checkOnly) != 0
        && addBit(((*cache).bm1).as_mut_ptr(), rol(hash, 12 as i32), checkOnly) != 0
        && addBit(((*cache).bm2).as_mut_ptr(), rol(hash, 24 as i32), checkOnly) != 0)
        as i32;
}
unsafe extern "C" fn addNameToHash(mut cache: *mut dirCache_t, mut name: *mut wchar_t) {
    mt_addHash(cache, calcHash(name), 0 as i32);
}
unsafe extern "C" fn hashDce(mut cache: *mut dirCache_t, mut dce: *mut dirCacheEntry_t) {
    if (*dce).beginSlot != (*cache).nrHashed {
        return;
    }
    (*cache).nrHashed = (*dce).endSlot;
    if !((*dce).longName).is_null() {
        addNameToHash(cache, (*dce).longName);
    }
    addNameToHash(cache, (*dce).shortName);
}
#[no_mangle]
pub unsafe extern "C" fn isHashed(
    mut cache: *mut dirCache_t,
    mut name: *mut wchar_t,
) -> i32 {
    let mut ret: i32 = 0;
    ret = mt_addHash(cache, calcHash(name), 1 as i32);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn growDirCache(mut cache: *mut dirCache_t, mut slot: u32) -> i32 {
    if (slot as i32) < 0 as i32 {
        fprintf(stderr, b"Bad slot %d\n\0" as *const u8 as *const i8, slot);
        exit(1 as i32);
    }
    if (*cache).nr_entries <= slot {
        let mut i: u32 = 0;
        (*cache).entries = realloc(
            (*cache).entries as *mut libc::c_void,
            (slot.wrapping_add(1 as i32 as u32).wrapping_mul(2 as i32 as u32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut dirCacheEntry_t>() as u64),
        ) as *mut *mut dirCacheEntry_t;
        if ((*cache).entries).is_null() {
            return -(1 as i32);
        }
        i = (*cache).nr_entries;
        while i < slot.wrapping_add(1 as i32 as u32).wrapping_mul(2 as i32 as u32) {
            let ref mut fresh0 = *((*cache).entries).offset(i as isize);
            *fresh0 = 0 as *mut dirCacheEntry_t;
            i = i.wrapping_add(1);
            i;
        }
        (*cache).nr_entries = slot
            .wrapping_add(1 as i32 as u32)
            .wrapping_mul(2 as i32 as u32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn allocDirCache(
    mut Stream: *mut Stream_t,
    mut slot: u32,
) -> *mut dirCache_t {
    let mut dcp: *mut *mut dirCache_t = 0 as *mut *mut dirCache_t;
    if (slot as i32) < 0 as i32 {
        fprintf(stderr, b"Bad slot %d\n\0" as *const u8 as *const i8, slot);
        exit(1 as i32);
    }
    dcp = getDirCacheP(Stream);
    if (*dcp).is_null() {
        *dcp = calloc(1 as i32 as u64, ::core::mem::size_of::<dirCache_t>() as u64)
            as *mut dirCache_t;
        if (*dcp).is_null() {
            return 0 as *mut dirCache_t;
        }
        (**dcp).entries = calloc(
            slot
                .wrapping_add(1 as i32 as u32)
                .wrapping_mul(2 as i32 as u32)
                .wrapping_add(5 as i32 as u32) as u64,
            ::core::mem::size_of::<*mut dirCacheEntry_t>() as u64,
        ) as *mut *mut dirCacheEntry_t;
        if ((**dcp).entries).is_null() {
            free(*dcp as *mut libc::c_void);
            return 0 as *mut dirCache_t;
        }
        (**dcp).nr_entries = slot
            .wrapping_add(1 as i32 as u32)
            .wrapping_mul(2 as i32 as u32);
        memset(
            ((**dcp).bm0).as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<[u32; 128]>() as u64,
        );
        memset(
            ((**dcp).bm1).as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<[u32; 128]>() as u64,
        );
        memset(
            ((**dcp).bm2).as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<[u32; 128]>() as u64,
        );
        (**dcp).nrHashed = 0 as i32 as u32;
    } else if growDirCache(*dcp, slot) < 0 as i32 {
        return 0 as *mut dirCache_t
    }
    return *dcp;
}
unsafe extern "C" fn freeDirCacheRange(
    mut cache: *mut dirCache_t,
    mut beginSlot: u32,
    mut endSlot: u32,
) -> i32 {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut clearBegin: u32 = 0;
    let mut clearEnd: u32 = 0;
    let mut i: u32 = 0;
    if endSlot < beginSlot {
        fprintf(
            stderr,
            b"Bad slots %d %d in free range\n\0" as *const u8 as *const i8,
            beginSlot,
            endSlot,
        );
        exit(1 as i32);
    }
    while beginSlot < endSlot {
        entry = *((*cache).entries).offset(beginSlot as isize);
        if entry.is_null() {
            beginSlot = beginSlot.wrapping_add(1);
            beginSlot;
        } else {
            if (*entry).beginSlot == beginSlot {} else {
                __assert_fail(
                    b"entry->beginSlot == beginSlot\0" as *const u8 as *const i8,
                    b"dirCache.c\0" as *const u8 as *const i8,
                    198 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 64],
                        &[i8; 64],
                    >(
                        b"int freeDirCacheRange(dirCache_t *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7966: {
                if (*entry).beginSlot == beginSlot {} else {
                    __assert_fail(
                        b"entry->beginSlot == beginSlot\0" as *const u8 as *const i8,
                        b"dirCache.c\0" as *const u8 as *const i8,
                        198 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 64],
                            &[i8; 64],
                        >(
                            b"int freeDirCacheRange(dirCache_t *, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            clearEnd = (*entry).endSlot;
            if clearEnd > endSlot {
                clearEnd = endSlot;
            }
            clearBegin = beginSlot;
            i = clearBegin;
            while i < clearEnd {
                let ref mut fresh1 = *((*cache).entries).offset(i as isize);
                *fresh1 = 0 as *mut dirCacheEntry_t;
                i = i.wrapping_add(1);
                i;
            }
            (*entry).beginSlot = clearEnd;
            if (*entry).beginSlot == (*entry).endSlot {
                let mut needWriteEnd: i32 = 0 as i32;
                if (*entry).endMarkPos != -(1 as i32)
                    && (*entry).endMarkPos < beginSlot as i32
                {
                    needWriteEnd = 1 as i32;
                }
                if !((*entry).longName).is_null() {
                    free((*entry).longName as *mut libc::c_void);
                }
                if !((*entry).shortName).is_null() {
                    free((*entry).shortName as *mut libc::c_void);
                }
                free(entry as *mut libc::c_void);
                if needWriteEnd != 0 {
                    return beginSlot as i32;
                }
            }
            beginSlot = clearEnd;
        }
    }
    return -(1 as i32);
}
unsafe extern "C" fn allocDirCacheEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: u32,
    mut endSlot: u32,
    mut type_0: dirCacheEntryType_t,
) -> *mut dirCacheEntry_t {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut i: u32 = 0;
    if growDirCache(cache, endSlot) < 0 as i32 {
        return 0 as *mut dirCacheEntry_t;
    }
    entry = calloc(1 as i32 as u64, ::core::mem::size_of::<dirCacheEntry_t>() as u64)
        as *mut dirCacheEntry_t;
    if entry.is_null() {
        return 0 as *mut dirCacheEntry_t;
    }
    (*entry).type_0 = type_0;
    (*entry).longName = 0 as *mut wchar_t;
    (*entry).shortName = 0 as *mut wchar_t;
    (*entry).beginSlot = beginSlot;
    (*entry).endSlot = endSlot;
    (*entry).endMarkPos = -(1 as i32);
    freeDirCacheRange(cache, beginSlot, endSlot);
    i = beginSlot;
    while i < endSlot {
        let ref mut fresh2 = *((*cache).entries).offset(i as isize);
        *fresh2 = entry;
        i = i.wrapping_add(1);
        i;
    }
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn addUsedEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: u32,
    mut endSlot: u32,
    mut longName: *mut wchar_t,
    mut shortName: *mut wchar_t,
    mut dir: *mut directory,
) -> *mut dirCacheEntry_t {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    if endSlot < beginSlot {
        fprintf(
            stderr,
            b"Bad slots %d %d in add used entry\n\0" as *const u8 as *const i8,
            beginSlot,
            endSlot,
        );
        exit(1 as i32);
    }
    entry = allocDirCacheEntry(
        cache,
        beginSlot,
        endSlot,
        dirCacheEntryType_t::DCET_USED,
    );
    if entry.is_null() {
        return 0 as *mut dirCacheEntry_t;
    }
    (*entry).beginSlot = beginSlot;
    (*entry).endSlot = endSlot;
    if !longName.is_null() {
        (*entry).longName = wcsdup(longName);
    }
    (*entry).shortName = wcsdup(shortName);
    (*entry).dir = *dir;
    hashDce(cache, entry);
    return entry;
}
unsafe extern "C" fn mergeFreeSlots(mut cache: *mut dirCache_t, mut slot: u32) {
    let mut previous: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut next: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut i: u32 = 0;
    if slot == 0 as i32 as u32 {
        return;
    }
    previous = *((*cache).entries).offset(slot.wrapping_sub(1 as i32 as u32) as isize);
    next = *((*cache).entries).offset(slot as isize);
    if !next.is_null()
        && (*next).type_0 as u32 == dirCacheEntryType_t::DCET_FREE as i32 as u32
        && !previous.is_null()
        && (*previous).type_0 as u32 == dirCacheEntryType_t::DCET_FREE as i32 as u32
    {
        i = (*next).beginSlot;
        while i < (*next).endSlot {
            let ref mut fresh3 = *((*cache).entries).offset(i as isize);
            *fresh3 = previous;
            i = i.wrapping_add(1);
            i;
        }
        (*previous).endSlot = (*next).endSlot;
        (*previous).endMarkPos = (*next).endMarkPos;
        free(next as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn addFreeEndEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: u32,
    mut endSlot: u32,
    mut isAtEnd: i32,
) -> *mut dirCacheEntry_t {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    if beginSlot < (*cache).nrHashed {
        (*cache).nrHashed = beginSlot;
    }
    if endSlot < beginSlot {
        fprintf(
            stderr,
            b"Bad slots %d %d in add free entry\n\0" as *const u8 as *const i8,
            beginSlot,
            endSlot,
        );
        exit(1 as i32);
    }
    if endSlot == beginSlot {
        return 0 as *mut dirCacheEntry_t;
    }
    entry = allocDirCacheEntry(
        cache,
        beginSlot,
        endSlot,
        dirCacheEntryType_t::DCET_FREE,
    );
    if isAtEnd != 0 {
        (*entry).endMarkPos = beginSlot as i32;
    }
    mergeFreeSlots(cache, beginSlot);
    mergeFreeSlots(cache, endSlot);
    return *((*cache).entries).offset(beginSlot as isize);
}
#[no_mangle]
pub unsafe extern "C" fn addFreeEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: u32,
    mut endSlot: u32,
) -> *mut dirCacheEntry_t {
    return addFreeEndEntry(cache, beginSlot, endSlot, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn addEndEntry(
    mut cache: *mut dirCache_t,
    mut pos: u32,
) -> *mut dirCacheEntry_t {
    return allocDirCacheEntry(
        cache,
        pos,
        pos.wrapping_add(1 as u32),
        dirCacheEntryType_t::DCET_END,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lookupInDircache(
    mut cache: *mut dirCache_t,
    mut pos: u32,
) -> *mut dirCacheEntry_t {
    if growDirCache(cache, pos.wrapping_add(1 as i32 as u32)) < 0 as i32 {
        return 0 as *mut dirCacheEntry_t;
    }
    return *((*cache).entries).offset(pos as isize);
}
#[no_mangle]
pub unsafe extern "C" fn freeDirCache(mut Stream: *mut Stream_t) {
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut dcp: *mut *mut dirCache_t = 0 as *mut *mut dirCache_t;
    dcp = getDirCacheP(Stream);
    cache = *dcp;
    if !cache.is_null() {
        let mut n: i32 = 0;
        n = freeDirCacheRange(cache, 0 as i32 as u32, (*cache).nr_entries);
        if n >= 0 as i32 {
            low_level_dir_write_end(Stream, n);
        }
        free(cache as *mut libc::c_void);
        *dcp = 0 as *mut dirCache_t;
    }
}
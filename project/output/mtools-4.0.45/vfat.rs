use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    fn wcscasecmp(__s1: *const wchar_t, __s2: *const wchar_t) -> i32;
    fn wcslen(_: *const i32) -> u64;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    static mut mtools_numeric_tail: u32;
    fn isRootDir(Stream: *mut Stream_t) -> i32;
    fn dir_read(entry: *mut direntry_t, error: *mut i32) -> *mut directory;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn isNotFound(entry: *mut direntry_t) -> i32;
    fn isRootEntry(entry: *mut direntry_t) -> i32;
    fn setEntryForIteration(entry: *mut direntry_t, pos: u32);
    fn setEntryToPos(entry: *mut direntry_t, pos: u32);
    fn getEntryAsPos(entry: *mut direntry_t) -> u32;
    fn getNextEntryAsPos(entry: *mut direntry_t) -> u32;
    fn low_level_dir_write(entry: *mut direntry_t);
    fn growDirCache(cache: *mut dirCache_t, slot: u32) -> i32;
    fn allocDirCache(Stream: *mut Stream_t, slot: u32) -> *mut dirCache_t;
    fn addFreeEntry(
        Stream: *mut dirCache_t,
        begin: u32,
        end: u32,
    ) -> *mut dirCacheEntry_t;
    fn addFreeEndEntry(
        Stream: *mut dirCache_t,
        begin: u32,
        end: u32,
        isAtEnd: i32,
    ) -> *mut dirCacheEntry_t;
    fn addEndEntry(Stream: *mut dirCache_t, pos: u32) -> *mut dirCacheEntry_t;
    fn isHashed(cache: *mut dirCache_t, name: *mut wchar_t) -> i32;
    fn addUsedEntry(
        Stream: *mut dirCache_t,
        begin: u32,
        end: u32,
        longName: *mut wchar_t,
        shortName: *mut wchar_t,
        dir: *mut directory,
    ) -> *mut dirCacheEntry_t;
    fn dos_to_wchar(
        fromDos: *mut doscp_t,
        dos: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
    ) -> size_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut i8,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
    fn native_to_wchar(
        native: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const i8,
        mangled: *mut i32,
    ) -> size_t;
    #[link_name = "match"]
    fn match_0(
        _: *const wchar_t,
        _: *const wchar_t,
        _: *mut wchar_t,
        _: i32,
        _: i32,
    ) -> i32;
    fn unix_name(
        fromDos: *mut doscp_t,
        base: *const i8,
        ext: *const i8,
        Case: uint8_t,
        answer: *mut wchar_t,
    ) -> *mut wchar_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scan_state {
    pub match_free: i32,
    pub shortmatch: i32,
    pub longmatch: i32,
    pub free_start: u32,
    pub free_end: u32,
    pub slot: u32,
    pub got_slots: i32,
    pub size_needed: u32,
    pub max_entry: u32,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum result_t {
    RES_MATCH = 1,
    RES_ERROR = 3,
    RES_END = 2,
    RES_NOMATCH = 0,
}
impl result_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            result_t::RES_MATCH => 1,
            result_t::RES_ERROR => 3,
            result_t::RES_END => 2,
            result_t::RES_NOMATCH => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> result_t {
        match value {
            1 => result_t::RES_MATCH,
            3 => result_t::RES_ERROR,
            2 => result_t::RES_END,
            0 => result_t::RES_NOMATCH,
            _ => panic!("Invalid value for result_t: {}", value),
        }
    }
}
impl AddAssign<u32> for result_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = result_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for result_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = result_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for result_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = result_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for result_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = result_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for result_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = result_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for result_t {
    type Output = result_t;
    fn add(self, rhs: u32) -> result_t {
        result_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for result_t {
    type Output = result_t;
    fn sub(self, rhs: u32) -> result_t {
        result_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for result_t {
    type Output = result_t;
    fn mul(self, rhs: u32) -> result_t {
        result_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for result_t {
    type Output = result_t;
    fn div(self, rhs: u32) -> result_t {
        result_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for result_t {
    type Output = result_t;
    fn rem(self, rhs: u32) -> result_t {
        result_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfat_state {
    pub name: [wchar_t; 261],
    pub status: i32,
    pub subentries: u32,
    pub sum: u8,
    pub present: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfat_subentry {
    pub id: u8,
    pub text1: [u8; 10],
    pub attribute: u8,
    pub hash1: u8,
    pub sum: u8,
    pub text2: [u8; 12],
    pub sector_l: u8,
    pub sector_u: u8,
    pub text3: [u8; 4],
}
#[no_mangle]
pub static mut short_illegals: *const i8 = b";+=[]',\"*\\<>/?:|\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut long_illegals: *const i8 = b"\"*\\<>/?:|\x05\0" as *const u8 as *const i8;
unsafe extern "C" fn fmt_num(
    mut num: u32,
    mut base: *mut i8,
    mut end: i32,
    mut prefix: i8,
) {
    while end != 0 {
        *base.offset((end - 1 as i32) as isize) = ('0' as i32 as u32)
            .wrapping_add(num.wrapping_rem(10 as i32 as u32)) as i8;
        num = num.wrapping_div(10 as i32 as u32);
        end -= 1;
        end;
    }
    *base.offset(end as isize) = prefix;
}
unsafe extern "C" fn autorename(
    mut name: *mut i8,
    mut tilda: i8,
    mut dot: i8,
    mut illegals: *const i8,
    mut limit: i32,
    mut bump: i32,
) {
    let mut tildapos: i32 = 0;
    let mut dotpos: i32 = 0;
    let mut seqnum: u32 = 0 as i32 as u32;
    let mut maxseq: u32 = 0 as i32 as u32;
    let mut p: *mut i8 = 0 as *mut i8;
    tildapos = -(1 as i32);
    p = name;
    while *p != 0 {
        if !(strchr(illegals, *p as i32)).is_null() {
            *p = '_' as i32 as i8;
            bump = 0 as i32;
        }
        p = p.offset(1);
        p;
    }
    dotpos = 0 as i32;
    while *name.offset(dotpos as isize) as i32 != 0 && dotpos < limit
        && *name.offset(dotpos as isize) as i32 != dot as i32
    {
        if *name.offset(dotpos as isize) as i32 == tilda as i32 {
            tildapos = dotpos;
            seqnum = 0 as i32 as u32;
            maxseq = 1 as i32 as u32;
        } else if *name.offset(dotpos as isize) as i32 >= '0' as i32
            && *name.offset(dotpos as isize) as i32 <= '9' as i32
        {
            seqnum = seqnum
                .wrapping_mul(10 as i32 as u32)
                .wrapping_add(
                    (*name.offset(dotpos as isize) as i32 - '0' as i32) as uint8_t as u32,
                );
            maxseq = maxseq.wrapping_mul(10 as i32 as u32);
        } else {
            tildapos = -(1 as i32);
        }
        dotpos += 1;
        dotpos;
    }
    if tildapos == -(1 as i32) {
        if dotpos > limit - 2 as i32 {
            tildapos = limit - 2 as i32;
            dotpos = limit;
        } else {
            tildapos = dotpos;
            dotpos += 2 as i32;
        }
        seqnum = 1 as i32 as u32;
    } else {
        if bump != 0 {
            seqnum = seqnum.wrapping_add(1);
            seqnum;
        }
        if seqnum > 999999 as i32 as u32 {
            seqnum = 1 as i32 as u32;
            tildapos = dotpos - 2 as i32;
        }
        if seqnum == maxseq {
            if dotpos >= limit {
                tildapos -= 1;
                tildapos;
            } else {
                dotpos += 1;
                dotpos;
            }
        }
    }
    if bump != 0 && seqnum == 1 as i32 as u32 || seqnum > 1 as i32 as u32
        || mtools_numeric_tail != 0
    {
        fmt_num(seqnum, name.offset(tildapos as isize), dotpos - tildapos, tilda);
    }
    if dot == 0 {
        *name.offset(dotpos as isize) = '\0' as i32 as i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn autorename_short(mut name: *mut dos_name_t, mut bump: i32) {
    autorename(
        ((*name).base).as_mut_ptr(),
        '~' as i32 as i8,
        ' ' as i32 as i8,
        short_illegals,
        8 as i32,
        bump,
    );
}
#[no_mangle]
pub unsafe extern "C" fn autorename_long(mut name: *mut i8, mut bump: i32) {
    autorename(
        name,
        '-' as i32 as i8,
        '\0' as i32 as i8,
        long_illegals,
        255 as i32,
        bump,
    );
}
unsafe extern "C" fn unicode_write(
    mut in_0: *mut wchar_t,
    mut out: *mut u8,
    mut num: i32,
    mut end_p: *mut i32,
) -> i32 {
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < num {
        if *end_p != 0 {
            let ref mut fresh0 = *out.offset(1 as i32 as isize);
            *fresh0 = 0xff as i32 as u8;
            *out.offset(0 as i32 as isize) = *fresh0;
        } else {
            *out.offset(1 as i32 as isize) = ((*in_0 & 0xffff as i32) >> 8 as i32) as u8;
            *out.offset(0 as i32 as isize) = (*in_0 & 0xff as i32) as u8;
            if *in_0 == 0 {
                *end_p = 0x40 as i32;
            }
        }
        out = out.offset(1);
        out;
        out = out.offset(1);
        out;
        in_0 = in_0.offset(1);
        in_0;
        j += 1;
        j;
    }
    return num;
}
#[inline]
unsafe extern "C" fn unicode_read(
    mut in_0: *mut u8,
    mut out: *mut wchar_t,
    mut num: i32,
) -> i32 {
    let mut end_out: *mut wchar_t = out.offset(num as isize);
    while out < end_out {
        *out = *in_0.offset(0 as i32 as isize) as i32
            | (*in_0.offset(1 as i32 as isize) as i32) << 8 as i32;
        out = out.offset(1);
        out;
        in_0 = in_0.offset(1);
        in_0;
        in_0 = in_0.offset(1);
        in_0;
    }
    return num;
}
unsafe extern "C" fn clear_vfat(mut v: *mut vfat_state) {
    (*v).subentries = 0 as i32 as u32;
    (*v).status = 0 as i32;
    (*v).present = 0 as i32;
}
#[inline]
unsafe extern "C" fn sum_shortname(mut dn: *const dos_name_t) -> u8 {
    let mut sum: u8 = 0;
    let mut name: *const i8 = ((*dn).base).as_ptr();
    let mut end: *const i8 = name.offset(11 as i32 as isize);
    sum = 0 as i32 as u8;
    while name < end {
        sum = ((if sum as i32 & 1 as i32 != 0 { 0x80 as i32 } else { 0 as i32 })
            + (sum as i32 >> 1 as i32) + *name as uint8_t as i32) as u8;
        name = name.offset(1);
        name;
    }
    return sum;
}
#[inline]
unsafe extern "C" fn check_vfat(mut v: *mut vfat_state, mut dir: *mut directory) {
    let mut dn: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    if (*v).subentries == 0 {
        return;
    }
    memcpy(
        (dn.base).as_mut_ptr() as *mut libc::c_void,
        ((*dir).name).as_mut_ptr() as *const libc::c_void,
        8 as i32 as u64,
    );
    memcpy(
        (dn.ext).as_mut_ptr() as *mut libc::c_void,
        ((*dir).ext).as_mut_ptr() as *const libc::c_void,
        3 as i32 as u64,
    );
    if (*v).sum as i32 != sum_shortname(&mut dn) as i32 {
        return;
    }
    if (*v).status & ((1 as i32) << (*v).subentries) - 1 as i32
        != ((1 as i32) << (*v).subentries) - 1 as i32
    {
        return;
    }
    (*v).name[(13 as i32 as u32).wrapping_mul((*v).subentries) as usize] = 0 as i32;
    (*v).present = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn write_vfat(
    mut Dir: *mut Stream_t,
    mut shortname: *mut dos_name_t,
    mut longname: *mut i8,
    mut start: u32,
    mut mainEntry: *mut direntry_t,
) -> u32 {
    let mut vse: *mut vfat_subentry = 0 as *mut vfat_subentry;
    let mut vse_id: uint8_t = 0;
    let mut num_vses: uint8_t = 0;
    let mut c: *mut wchar_t = 0 as *mut wchar_t;
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
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut unixyName: [wchar_t; 13] = [0; 13];
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert)
        .expect("non-null function pointer")(Dir);
    let mut wlongname: [wchar_t; 256] = [0; 256];
    let mut wlen: size_t = 0;
    if !longname.is_null() {
        entry.Dir = Dir;
        vse = &mut entry.dir as *mut directory as *mut vfat_subentry;
        (*vse).attribute = 0xf as i32 as u8;
        (*vse).sector_u = 0 as i32 as u8;
        (*vse).sector_l = (*vse).sector_u;
        (*vse).hash1 = (*vse).sector_l;
        (*vse).sum = sum_shortname(shortname);
        wlen = native_to_wchar(
            longname,
            wlongname.as_mut_ptr(),
            (255 as i32 + 1 as i32) as size_t,
            0 as *const i8,
            0 as *mut i32,
        );
        num_vses = wlen
            .wrapping_add(13 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(13 as i32 as u64) as uint8_t;
        vse_id = num_vses;
        while vse_id != 0 {
            let mut end: i32 = 0 as i32;
            c = wlongname
                .as_mut_ptr()
                .offset(((vse_id as i32 - 1 as i32) * 13 as i32) as isize);
            c = c
                .offset(
                    unicode_write(c, ((*vse).text1).as_mut_ptr(), 5 as i32, &mut end)
                        as isize,
                );
            c = c
                .offset(
                    unicode_write(c, ((*vse).text2).as_mut_ptr(), 6 as i32, &mut end)
                        as isize,
                );
            c = c
                .offset(
                    unicode_write(c, ((*vse).text3).as_mut_ptr(), 2 as i32, &mut end)
                        as isize,
                );
            (*vse).id = (if vse_id as i32 == num_vses as i32 {
                vse_id as i32 | 0x40 as i32
            } else {
                vse_id as i32
            }) as u8;
            setEntryToPos(
                &mut entry,
                start.wrapping_add(num_vses as u32).wrapping_sub(vse_id as u32),
            );
            low_level_dir_write(&mut entry);
            vse_id = vse_id.wrapping_sub(1);
            vse_id;
        }
    } else {
        num_vses = 0 as i32 as uint8_t;
        wlongname[0 as i32 as usize] = '\0' as i32;
    }
    cache = allocDirCache(
        Dir,
        start.wrapping_add(num_vses as u32).wrapping_add(1 as i32 as u32),
    );
    if cache.is_null() {
        fprintf(stderr, b"Out of memory error\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    unix_name(
        cp,
        ((*shortname).base).as_mut_ptr(),
        ((*shortname).ext).as_mut_ptr(),
        0 as i32 as uint8_t,
        unixyName.as_mut_ptr(),
    );
    addUsedEntry(
        cache,
        start,
        start.wrapping_add(num_vses as u32).wrapping_add(1 as i32 as u32),
        wlongname.as_mut_ptr(),
        unixyName.as_mut_ptr(),
        &mut (*mainEntry).dir,
    );
    low_level_dir_write(mainEntry);
    return start.wrapping_add(num_vses as u32);
}
#[no_mangle]
pub unsafe extern "C" fn dir_write(mut entry: *mut direntry_t) {
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    if isRootEntry(entry) != 0 {
        fprintf(
            stderr,
            b"Attempt to write root directory pointer\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    cache = allocDirCache((*entry).Dir, getNextEntryAsPos(entry));
    if cache.is_null() {
        fprintf(
            stderr,
            b"Out of memory error in dir_write\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    dce = *((*cache).entries).offset((*entry).entry as isize);
    if !dce.is_null() {
        if (*entry).dir.name[0 as i32 as usize] as i32 == 0xe5 as i32 as i8 as i32 {
            addFreeEntry(cache, (*dce).beginSlot, (*dce).endSlot);
        } else {
            (*dce).dir = (*entry).dir;
        }
    }
    low_level_dir_write(entry);
}
#[inline]
unsafe extern "C" fn parse_vses(mut entry: *mut direntry_t, mut v: *mut vfat_state) {
    let mut vse: *mut vfat_subentry = 0 as *mut vfat_subentry;
    let mut id: u8 = 0;
    let mut last_flag: u8 = 0;
    let mut c: *mut wchar_t = 0 as *mut wchar_t;
    vse = &mut (*entry).dir as *mut directory as *mut vfat_subentry;
    id = ((*vse).id as i32 & 0x1f as i32) as u8;
    last_flag = ((*vse).id as i32 & 0x40 as i32) as u8;
    if id as i32 > 20 as i32 {
        fprintf(
            stderr,
            b"parse_vses: invalid VSE ID %d at %d.\n\0" as *const u8 as *const i8,
            id as i32,
            (*entry).entry,
        );
        return;
    }
    if (*v).sum as i32 != (*vse).sum as i32 {
        clear_vfat(v);
        (*v).sum = (*vse).sum;
    }
    (*v).status |= (1 as i32) << id as i32 - 1 as i32;
    if last_flag != 0 {
        (*v).subentries = id as u32;
    }
    c = &mut *((*v).name)
        .as_mut_ptr()
        .offset((13 as i32 * (id as i32 - 1 as i32)) as isize) as *mut wchar_t;
    c = c.offset(unicode_read(((*vse).text1).as_mut_ptr(), c, 5 as i32) as isize);
    c = c.offset(unicode_read(((*vse).text2).as_mut_ptr(), c, 6 as i32) as isize);
    c = c.offset(unicode_read(((*vse).text3).as_mut_ptr(), c, 2 as i32) as isize);
    if last_flag != 0 {
        *c = '\0' as i32;
    }
}
unsafe extern "C" fn vfat_lookup_loop_common(
    mut cp: *mut doscp_t,
    mut direntry: *mut direntry_t,
    mut cache: *mut dirCache_t,
    mut lookForFreeSpace: i32,
    mut io_error: *mut i32,
) -> *mut dirCacheEntry_t {
    let mut newfile: [wchar_t; 13] = [0; 13];
    let mut initpos: u32 = getNextEntryAsPos(direntry);
    let mut vfat: vfat_state = vfat_state {
        name: [0; 261],
        status: 0,
        subentries: 0,
        sum: 0,
        present: 0,
    };
    let mut longname: *mut wchar_t = 0 as *mut wchar_t;
    let mut error: i32 = 0;
    let mut endmarkSeen: i32 = 0 as i32;
    *io_error = 0 as i32;
    clear_vfat(&mut vfat);
    loop {
        (*direntry).entry += 1;
        (*direntry).entry;
        if (dir_read(direntry, &mut error)).is_null() {
            if error != 0 {
                *io_error = error;
                return 0 as *mut dirCacheEntry_t;
            }
            addFreeEndEntry(cache, initpos, getEntryAsPos(direntry), endmarkSeen);
            return addEndEntry(cache, getEntryAsPos(direntry));
        }
        if endmarkSeen != 0
            || (*direntry).dir.name[0 as i32 as usize] as i32 == 0 as i32 as i8 as i32
        {
            if lookForFreeSpace != 0 {
                endmarkSeen = 1 as i32;
            } else {
                return addEndEntry(cache, getEntryAsPos(direntry))
            }
        } else {
            if !((*direntry).dir.name[0 as i32 as usize] as i32
                != 0xe5 as i32 as i8 as i32 && (*direntry).dir.attr as i32 == 0xf as i32)
            {
                break;
            }
            parse_vses(direntry, &mut vfat);
        }
    }
    if (*direntry).dir.name[0 as i32 as usize] as i32 == 0xe5 as i32 as i8 as i32 {
        return addFreeEntry(cache, initpos, getNextEntryAsPos(direntry));
    }
    check_vfat(&mut vfat, &mut (*direntry).dir);
    if vfat.present == 0 {
        vfat.subentries = 0 as i32 as u32;
    }
    addFreeEntry(
        cache,
        initpos,
        (getEntryAsPos(direntry)).wrapping_sub(vfat.subentries),
    );
    if (*direntry).dir.attr as i32 & 0x8 as i32 != 0 {
        let mut ptr: *mut wchar_t = newfile.as_mut_ptr();
        if (*direntry).dir.name[0 as i32 as usize] as i32 == '\u{5}' as i32 {
            ptr = ptr
                .offset(
                    dos_to_wchar(
                        cp,
                        b"\xE5\0" as *const u8 as *const i8,
                        ptr,
                        1 as i32 as size_t,
                    ) as isize,
                );
            ptr = ptr
                .offset(
                    dos_to_wchar(
                        cp,
                        ((*direntry).dir.name).as_mut_ptr().offset(1 as i32 as isize),
                        ptr,
                        7 as i32 as size_t,
                    ) as isize,
                );
        } else {
            ptr = ptr
                .offset(
                    dos_to_wchar(
                        cp,
                        ((*direntry).dir.name).as_mut_ptr(),
                        ptr,
                        8 as i32 as size_t,
                    ) as isize,
                );
        }
        ptr = ptr
            .offset(
                dos_to_wchar(
                    cp,
                    ((*direntry).dir.ext).as_mut_ptr(),
                    ptr,
                    3 as i32 as size_t,
                ) as isize,
            );
        *ptr = '\0' as i32;
    } else {
        unix_name(
            cp,
            ((*direntry).dir.name).as_mut_ptr(),
            ((*direntry).dir.ext).as_mut_ptr(),
            (*direntry).dir.Case,
            newfile.as_mut_ptr(),
        );
    }
    if vfat.present != 0 {
        longname = (vfat.name).as_mut_ptr();
    } else {
        longname = 0 as *mut wchar_t;
    }
    return addUsedEntry(
        cache,
        (getEntryAsPos(direntry)).wrapping_sub(vfat.subentries),
        getNextEntryAsPos(direntry),
        longname,
        newfile.as_mut_ptr(),
        &mut (*direntry).dir,
    );
}
#[inline]
unsafe extern "C" fn vfat_lookup_loop_for_read(
    mut cp: *mut doscp_t,
    mut direntry: *mut direntry_t,
    mut cache: *mut dirCache_t,
    mut io_error: *mut i32,
) -> *mut dirCacheEntry_t {
    let mut initpos: i32 = (*direntry).entry + 1 as i32;
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    *io_error = 0 as i32;
    dce = *((*cache).entries).offset(initpos as isize);
    if !dce.is_null() {
        setEntryToPos(direntry, ((*dce).endSlot).wrapping_sub(1 as i32 as u32));
        return dce;
    } else {
        return vfat_lookup_loop_common(cp, direntry, cache, 0 as i32, io_error)
    };
}
unsafe extern "C" fn checkNameForMatch(
    mut direntry: *mut direntry_t,
    mut dce: *mut dirCacheEntry_t,
    mut filename: *const wchar_t,
    mut length: i32,
    mut flags: i32,
) -> result_t {
    match (*dce).type_0 as u32 {
        0 => return result_t::RES_NOMATCH,
        2 => return result_t::RES_END,
        1 | _ => {}
    }
    (*direntry).dir = (*dce).dir;
    if (*direntry).dir.attr as i32 & 0x8 as i32 != 0 && flags & 0x8 as i32 == 0 {
        return result_t::RES_NOMATCH;
    }
    if !(flags & 0x40 as i32 != 0
        || !((*dce).longName).is_null()
            && match_0(
                (*dce).longName,
                filename,
                ((*direntry).name).as_mut_ptr(),
                0 as i32,
                length,
            ) != 0
        || match_0(
            (*dce).shortName,
            filename,
            ((*direntry).name).as_mut_ptr(),
            1 as i32,
            length,
        ) != 0)
    {
        return result_t::RES_NOMATCH;
    }
    if (*direntry).dir.attr as i32 & 0x10 as i32 != 0 && flags & 0x10 as i32 == 0 {
        if flags & (0x8 as i32 | 0x40 as i32 | 0x80 as i32) == 0 {
            let mut tmp: [i8; 53] = [0; 53];
            wchar_to_native(
                (*dce).shortName,
                tmp.as_mut_ptr(),
                13 as i32 as size_t,
                ::core::mem::size_of::<[i8; 53]>() as u64,
            );
            fprintf(
                stderr,
                b"Skipping \"%s\", is a directory\n\0" as *const u8 as *const i8,
                tmp.as_mut_ptr(),
            );
        }
        return result_t::RES_NOMATCH;
    }
    if (*direntry).dir.attr as i32 & (0x8 as i32 | 0x10 as i32) == 0
        && flags & 0x20 as i32 == 0
    {
        if flags & (0x8 as i32 | 0x40 as i32 | 0x80 as i32) == 0 {
            let mut tmp_0: [i8; 53] = [0; 53];
            wchar_to_native(
                (*dce).shortName,
                tmp_0.as_mut_ptr(),
                13 as i32 as size_t,
                ::core::mem::size_of::<[i8; 53]>() as u64,
            );
            fprintf(
                stderr,
                b"Skipping \"%s\", is not a directory\n\0" as *const u8 as *const i8,
                tmp_0.as_mut_ptr(),
            );
        }
        return result_t::RES_NOMATCH;
    }
    return result_t::RES_MATCH;
}
#[no_mangle]
pub unsafe extern "C" fn vfat_lookup_zt(
    mut direntry: *mut direntry_t,
    mut filename: *const i8,
    mut flags: i32,
    mut shortname: *mut i8,
    mut shortname_size: size_t,
    mut longname: *mut i8,
    mut longname_size: size_t,
) -> i32 {
    return vfat_lookup(
        direntry,
        filename,
        strlen(filename),
        flags,
        shortname,
        shortname_size,
        longname,
        longname_size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn vfat_lookup(
    mut direntry: *mut direntry_t,
    mut filename: *const i8,
    mut length: size_t,
    mut flags: i32,
    mut shortname: *mut i8,
    mut shortname_size: size_t,
    mut longname: *mut i8,
    mut longname_size: size_t,
) -> i32 {
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut result: result_t = result_t::RES_NOMATCH;
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut io_error: i32 = 0;
    let mut wfilename: [wchar_t; 256] = [0; 256];
    let mut cp: *mut doscp_t = ((*(*(*direntry).Dir).Class).get_dosConvert)
        .expect("non-null function pointer")((*direntry).Dir);
    if !filename.is_null() {
        length = native_to_wchar(
            filename,
            wfilename.as_mut_ptr(),
            255 as i32 as size_t,
            filename.offset(length as isize),
            0 as *mut i32,
        );
    } else {
        length = 0 as i32 as size_t;
    }
    if isNotFound(direntry) != 0 {
        return -(1 as i32);
    }
    cache = allocDirCache((*direntry).Dir, getNextEntryAsPos(direntry));
    if cache.is_null() {
        fprintf(
            stderr,
            b"Out of memory error in vfat_lookup [0]\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    loop {
        dce = vfat_lookup_loop_for_read(cp, direntry, cache, &mut io_error);
        if dce.is_null() {
            if io_error != 0 {
                return -(2 as i32);
            }
            fprintf(
                stderr,
                b"Out of memory error in vfat_lookup\n\0" as *const u8 as *const i8,
            );
            exit(1 as i32);
        }
        result = checkNameForMatch(
            direntry,
            dce,
            wfilename.as_mut_ptr(),
            length as i32,
            flags,
        );
        if !(result as u32 == result_t::RES_NOMATCH as i32 as u32) {
            break;
        }
    }
    if result as u32 == result_t::RES_MATCH as i32 as u32 {
        if !longname.is_null() {
            if !((*dce).longName).is_null() {
                wchar_to_native(
                    (*dce).longName,
                    longname,
                    255 as i32 as size_t,
                    longname_size,
                );
            } else {
                *longname = '\0' as i32 as i8;
            }
        }
        if !shortname.is_null() {
            wchar_to_native(
                (*dce).shortName,
                shortname,
                12 as i32 as size_t,
                shortname_size,
            );
        }
        (*direntry).beginSlot = (*dce).beginSlot;
        (*direntry).endSlot = ((*dce).endSlot).wrapping_sub(1 as i32 as u32);
        return 0 as i32;
    } else {
        (*direntry).entry = -(2 as i32);
        return -(1 as i32);
    };
}
#[inline]
unsafe extern "C" fn vfat_lookup_loop_for_insert(
    mut cp: *mut doscp_t,
    mut direntry: *mut direntry_t,
    mut initpos: u32,
    mut cache: *mut dirCache_t,
) -> *mut dirCacheEntry_t {
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut io_error: i32 = 0;
    dce = *((*cache).entries).offset(initpos as isize);
    if !dce.is_null()
        && (*dce).type_0 as u32 != dirCacheEntryType_t::DCET_END as i32 as u32
    {
        return dce
    } else {
        setEntryForIteration(direntry, initpos);
        dce = vfat_lookup_loop_common(cp, direntry, cache, 1 as i32, &mut io_error);
        if dce.is_null() {
            if io_error != 0 {
                return 0 as *mut dirCacheEntry_t;
            }
            fprintf(
                stderr,
                b"Out of memory error in vfat_lookup_loop\n\0" as *const u8 as *const i8,
            );
            exit(1 as i32);
        }
        return *((*cache).entries).offset(initpos as isize);
    };
}
unsafe extern "C" fn accountFreeSlots(
    mut ssp: *mut scan_state,
    mut dce: *mut dirCacheEntry_t,
) {
    if (*ssp).got_slots != 0 {
        return;
    }
    if (*ssp).free_end != (*dce).beginSlot {
        (*ssp).free_start = (*dce).beginSlot;
    }
    (*ssp).free_end = (*dce).endSlot;
    if ((*ssp).free_end).wrapping_sub((*ssp).free_start) >= (*ssp).size_needed {
        (*ssp).got_slots = 1 as i32;
        (*ssp).slot = ((*ssp).free_start)
            .wrapping_add((*ssp).size_needed)
            .wrapping_sub(1 as i32 as u32);
    }
}
unsafe extern "C" fn clear_scan(
    mut longname: *mut wchar_t,
    mut use_longname: i32,
    mut s: *mut scan_state,
) {
    (*s).longmatch = -(1 as i32);
    (*s).shortmatch = (*s).longmatch;
    (*s).free_start = 0 as i32 as u32;
    (*s).got_slots = (*s).free_start as i32;
    (*s).free_end = (*s).got_slots as u32;
    if use_longname & 1 as i32 != 0 {
        (*s).size_needed = (1 as i32 as u64)
            .wrapping_add(
                (wcslen(longname))
                    .wrapping_add(13 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_div(13 as i32 as u64),
            ) as u32;
    } else {
        (*s).size_needed = 1 as i32 as u32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lookupForInsert(
    mut Dir: *mut Stream_t,
    mut direntry: *mut direntry_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut ssp: *mut scan_state,
    mut ignore_entry: i32,
    mut source_entry: i32,
    mut pessimisticShortRename: i32,
    mut use_longname: i32,
) -> i32 {
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
    let mut ignore_match: i32 = 0;
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut pos: u32 = 0;
    let mut shortName: [wchar_t; 13] = [0; 13];
    let mut wlongname: [wchar_t; 256] = [0; 256];
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert)
        .expect("non-null function pointer")(Dir);
    native_to_wchar(
        longname,
        wlongname.as_mut_ptr(),
        (255 as i32 + 1 as i32) as size_t,
        0 as *const i8,
        0 as *mut i32,
    );
    clear_scan(wlongname.as_mut_ptr(), use_longname, ssp);
    ignore_match = (ignore_entry == -(2 as i32)) as i32;
    initializeDirentry(&mut entry, Dir);
    (*ssp).match_free = 0 as i32;
    cache = allocDirCache(Dir, 1 as i32 as u32);
    if cache.is_null() {
        fprintf(
            stderr,
            b"Out of memory error in lookupForInsert\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    if ignore_match == 0 {
        unix_name(
            cp,
            ((*dosname).base).as_mut_ptr(),
            ((*dosname).ext).as_mut_ptr(),
            0 as i32 as uint8_t,
            shortName.as_mut_ptr(),
        );
    }
    pos = (*cache).nrHashed;
    if source_entry >= 0 as i32
        || pos != 0 && isHashed(cache, wlongname.as_mut_ptr()) != 0
    {
        pos = 0 as i32 as u32;
    } else if pos != 0 && ignore_match == 0
        && isHashed(cache, shortName.as_mut_ptr()) != 0
    {
        if pessimisticShortRename != 0 {
            (*ssp).shortmatch = -(2 as i32);
            return 1 as i32;
        }
        pos = 0 as i32 as u32;
    } else if growDirCache(cache, pos) < 0 as i32 {
        fprintf(
            stderr,
            b"Out of memory error in vfat_looup [0]\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    loop {
        dce = vfat_lookup_loop_for_insert(cp, &mut entry, pos, cache);
        match (*dce).type_0 as u32 {
            0 => {
                accountFreeSlots(ssp, dce);
            }
            1 => {
                if (*dce).dir.attr as i32 & 0x8 as i32 == 0
                    && (*dce).endSlot as i32 - 1 as i32 == source_entry
                {
                    accountFreeSlots(ssp, dce);
                }
                if !((*dce).dir.attr as i32 & 0x8 as i32 != 0
                    || (*dce).endSlot as i32 - 1 as i32 == ignore_entry)
                {
                    if !((*dce).longName).is_null()
                        && wcscasecmp((*dce).longName, wlongname.as_mut_ptr()) == 0
                        || !((*dce).shortName).is_null()
                            && wcscasecmp((*dce).shortName, wlongname.as_mut_ptr()) == 0
                    {
                        (*ssp).longmatch = ((*dce).endSlot).wrapping_sub(1 as i32 as u32)
                            as i32;
                        (*direntry).beginSlot = (*dce).beginSlot;
                        (*direntry).endSlot = ((*dce).endSlot)
                            .wrapping_sub(1 as i32 as u32);
                        return 1 as i32;
                    }
                    if ignore_match == 0
                        && wcscasecmp(shortName.as_mut_ptr(), (*dce).shortName) == 0
                    {
                        (*ssp).shortmatch = ((*dce).endSlot)
                            .wrapping_sub(1 as i32 as u32) as i32;
                    }
                }
            }
            2 | _ => {}
        }
        pos = (*dce).endSlot;
        if !((*dce).type_0 as u32 != dirCacheEntryType_t::DCET_END as i32 as u32) {
            break;
        }
    }
    if (*ssp).shortmatch > -(1 as i32) {
        return 1 as i32;
    }
    (*ssp).max_entry = (*dce).beginSlot;
    if (*ssp).got_slots != 0 {
        return 6 as i32;
    }
    if isRootDir(Dir) == 0 {
        return 5 as i32;
    }
    fprintf(stderr, b"No directory slots\n\0" as *const u8 as *const i8);
    return -(1 as i32);
}
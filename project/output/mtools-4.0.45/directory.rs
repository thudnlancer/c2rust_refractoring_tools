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
extern "C" {
    pub type doscp_t;
    pub type Fs_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn perror(__s: *const i8);
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> i32;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn dosnameToDirentry(n: *const dos_name_t, dir: *mut directory);
    fn getClusterBytes(File: *mut Fs_t) -> uint32_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
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
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: i32,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: u32,
    pub endSlot: u32,
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
#[no_mangle]
pub unsafe extern "C" fn dir_read(
    mut entry: *mut direntry_t,
    mut error: *mut i32,
) -> *mut directory {
    let mut n: ssize_t = 0;
    *error = 0 as i32;
    n = force_pread(
        (*entry).Dir,
        &mut (*entry).dir as *mut directory as *mut i8,
        (*entry).entry as mt_off_t * 32 as i32 as i64,
        32 as i32 as size_t,
    );
    if n != 32 as i32 as i64 {
        if n < 0 as i32 as i64 {
            *error = -(1 as i32);
        }
        return 0 as *mut directory;
    }
    return &mut (*entry).dir;
}
#[no_mangle]
pub unsafe extern "C" fn dir_grow(mut Dir: *mut Stream_t, mut size: u32) -> i32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut ret: ssize_t = 0;
    let mut buflen: u32 = 0;
    let mut buffer: *mut i8 = 0 as *mut i8;
    if getfreeMinClusters(Dir, 1 as i32 as uint32_t) == 0 {
        return -(1 as i32);
    }
    buflen = getClusterBytes(This);
    buffer = malloc(buflen as u64) as *mut i8;
    if buffer.is_null() {
        perror(b"dir_grow: malloc\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    memset(buffer as *mut libc::c_void, '\0' as i32, buflen as u64);
    ret = force_pwrite(
        Dir,
        buffer,
        size as mt_off_t * 32 as i32 as i64,
        buflen as size_t,
    );
    free(buffer as *mut libc::c_void);
    if ret < buflen as i32 as i64 {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn low_level_dir_write(mut entry: *mut direntry_t) {
    force_pwrite(
        (*entry).Dir,
        &mut (*entry).dir as *mut directory as *mut i8,
        (*entry).entry as mt_off_t * 32 as i32 as i64,
        32 as i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn low_level_dir_write_end(
    mut Dir: *mut Stream_t,
    mut entry: i32,
) {
    let mut zero: i8 = 0 as i32 as i8;
    force_pwrite(
        Dir,
        &mut zero,
        entry as mt_off_t * 32 as i32 as i64,
        1 as i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mk_entry(
    mut dn: *const dos_name_t,
    mut attr: u8,
    mut fat: u32,
    mut size: uint32_t,
    mut date: time_t,
    mut ndir: *mut directory,
) -> *mut directory {
    let mut now: *mut tm = 0 as *mut tm;
    let mut date2: time_t = date;
    let mut hour: uint8_t = 0;
    let mut min_hi: uint8_t = 0;
    let mut min_low: uint8_t = 0;
    let mut sec: uint8_t = 0;
    let mut year: uint8_t = 0;
    let mut month_hi: uint8_t = 0;
    let mut month_low: uint8_t = 0;
    let mut day: uint8_t = 0;
    now = localtime(&mut date2);
    dosnameToDirentry(dn, ndir);
    (*ndir).attr = attr;
    (*ndir).ctime_ms = 0 as i32 as u8;
    hour = ((*now).tm_hour << 3 as i32) as uint8_t;
    min_hi = ((*now).tm_min >> 3 as i32) as uint8_t;
    min_low = ((*now).tm_min << 5 as i32) as uint8_t;
    sec = ((*now).tm_sec / 2 as i32) as uint8_t;
    (*ndir).time[1 as i32 as usize] = (hour as i32 + min_hi as i32) as u8;
    (*ndir).ctime[1 as i32 as usize] = (*ndir).time[1 as i32 as usize];
    (*ndir).time[0 as i32 as usize] = (min_low as i32 + sec as i32) as u8;
    (*ndir).ctime[0 as i32 as usize] = (*ndir).time[0 as i32 as usize];
    year = (((*now).tm_year - 80 as i32) << 1 as i32) as uint8_t;
    month_hi = ((*now).tm_mon + 1 as i32 >> 3 as i32) as uint8_t;
    month_low = (((*now).tm_mon + 1 as i32) << 5 as i32) as uint8_t;
    day = (*now).tm_mday as uint8_t;
    (*ndir).date[1 as i32 as usize] = (year as i32 + month_hi as i32) as u8;
    (*ndir).cdate[1 as i32 as usize] = (*ndir).date[1 as i32 as usize];
    (*ndir).adate[1 as i32 as usize] = (*ndir).cdate[1 as i32 as usize];
    (*ndir).date[0 as i32 as usize] = (month_low as i32 + day as i32) as u8;
    (*ndir).cdate[0 as i32 as usize] = (*ndir).date[0 as i32 as usize];
    (*ndir).adate[0 as i32 as usize] = (*ndir).cdate[0 as i32 as usize];
    set_word(
        ((*ndir).start).as_mut_ptr(),
        (fat & 0xffff as i32 as u32) as libc::c_ushort,
    );
    set_word(((*ndir).startHi).as_mut_ptr(), (fat >> 16 as i32) as libc::c_ushort);
    set_dword(((*ndir).size).as_mut_ptr(), size);
    return ndir;
}
#[no_mangle]
pub unsafe extern "C" fn mk_entry_from_base(
    mut base: *const i8,
    mut attr: u8,
    mut fat: u32,
    mut size: uint32_t,
    mut date: time_t,
    mut ndir: *mut directory,
) -> *mut directory {
    let mut entry: *mut directory = 0 as *mut directory;
    let mut dn: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    strncpy((dn.base).as_mut_ptr(), base, 8 as i32 as u64);
    strncpy((dn.ext).as_mut_ptr(), b"   \0" as *const u8 as *const i8, 3 as i32 as u64);
    entry = mk_entry(&mut dn, attr, fat, size, date, ndir);
    (*entry).Case = 0 as i32 as u8;
    return entry;
}
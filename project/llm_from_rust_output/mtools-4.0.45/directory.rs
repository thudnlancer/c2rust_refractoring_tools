use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar, c_ushort, c_uint, c_int, c_long, c_ulong};

type uint8_t = c_uchar;
type uint16_t = c_ushort;
type uint32_t = c_uint;
type size_t = c_ulong;
type ssize_t = c_long;
type off_t = c_long;
type time_t = c_long;
type mt_off_t = off_t;
type wchar_t = c_int;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct dos_name_t {
    pub base: [c_char; 8],
    pub ext: [c_char; 3],
    pub sentinel: c_char,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct directory {
    pub name: [c_char; 8],
    pub ext: [c_char; 3],
    pub attr: c_uchar,
    pub Case: c_uchar,
    pub ctime_ms: c_uchar,
    pub ctime: [c_uchar; 2],
    pub cdate: [c_uchar; 2],
    pub adate: [c_uchar; 2],
    pub startHi: [c_uchar; 2],
    pub time: [c_uchar; 2],
    pub date: [c_uchar; 2],
    pub start: [c_uchar; 2],
    pub size: [c_uchar; 4],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: c_uint,
    pub endSlot: c_uint,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: c_int,
    pub Next: *mut Stream_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut c_char, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut c_char, size_t) -> ssize_t>,
    pub pread: Option<unsafe extern "C" fn(*mut Stream_t, *mut c_char, mt_off_t, size_t) -> ssize_t>,
    pub pwrite: Option<unsafe extern "C" fn(*mut Stream_t, *mut c_char, mt_off_t, size_t) -> ssize_t>,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> c_int>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> c_int>,
    pub set_geom: Option<unsafe extern "C" fn(*mut Stream_t, *mut device, *mut device) -> c_int>,
    pub get_data: Option<unsafe extern "C" fn(*mut Stream_t, *mut time_t, *mut mt_off_t, *mut c_int, *mut uint32_t) -> c_int>,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> c_int>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> c_int>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct device {
    pub name: *const c_char,
    pub drive: c_char,
    pub fat_bits: c_int,
    pub mode: c_int,
    pub tracks: c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: c_uint,
    pub offset: mt_off_t,
    pub partition: c_uint,
    pub misc_flags: c_uint,
    pub ssize: uint8_t,
    pub use_2m: c_uint,
    pub precmd: *mut c_char,
    pub file_nr: c_int,
    pub blocksize: c_uint,
    pub codepage: c_uint,
    pub data_map: *const c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut c_char,
    pub cfg_filename: *const c_char,
}

pub type doscp_t = ();
pub type Fs_t = ();

fn set_word(data: &mut [u8], value: u16) {
    data[0] = (value & 0xFF) as u8;
    data[1] = (value >> 8) as u8;
}

fn set_dword(data: &mut [u8], value: u32) {
    data[0] = (value & 0xFF) as u8;
    data[1] = ((value >> 8) & 0xFF) as u8;
    data[2] = ((value >> 16) & 0xFF) as u8;
    data[3] = ((value >> 24) & 0xFF) as u8;
}

pub fn dir_read(entry: &mut direntry_t) -> Result<&mut directory, i32> {
    let n = unsafe {
        force_pread(
            entry.Dir,
            &mut entry.dir as *mut directory as *mut c_char,
            entry.entry as mt_off_t * 32,
            32,
        )
    };

    if n != 32 {
        if n < 0 {
            return Err(-1);
        }
        return Err(0);
    }
    Ok(&mut entry.dir)
}

pub fn dir_grow(dir: *mut Stream_t, size: u32) -> Result<(), i32> {
    let stream = unsafe { GetFs(dir) };
    let this = stream as *mut Fs_t;

    if getfreeMinClusters(dir, 1) == 0 {
        return Err(-1);
    }

    let buflen = unsafe { getClusterBytes(this) };
    let buffer = unsafe {
        let buf = libc::malloc(buflen as libc::size_t) as *mut c_char;
        if buf.is_null() {
            eprintln!("dir_grow: malloc");
            return Err(-1);
        }
        libc::memset(buf as *mut libc::c_void, 0, buflen as libc::size_t);
        buf
    };

    let ret = unsafe {
        force_pwrite(
            dir,
            buffer,
            size as mt_off_t * 32,
            buflen as size_t,
        )
    };

    unsafe { libc::free(buffer as *mut libc::c_void) };

    if ret < buflen as ssize_t {
        return Err(-1);
    }
    Ok(())
}

pub fn low_level_dir_write(entry: &mut direntry_t) {
    unsafe {
        force_pwrite(
            entry.Dir,
            &mut entry.dir as *mut directory as *mut c_char,
            entry.entry as mt_off_t * 32,
            32,
        );
    }
}

pub fn low_level_dir_write_end(dir: *mut Stream_t, entry: i32) {
    let zero: c_char = 0;
    unsafe {
        force_pwrite(
            dir,
            &zero as *const c_char as *mut c_char,
            entry as mt_off_t * 32,
            1,
        );
    }
}

pub fn mk_entry(
    dn: &dos_name_t,
    attr: u8,
    fat: u32,
    size: u32,
    date: time_t,
    ndir: &mut directory,
) -> &mut directory {
    let mut date2 = date;
    let now = unsafe { localtime(&mut date2) };

    unsafe { dosnameToDirentry(dn, ndir) };
    ndir.attr = attr;
    ndir.ctime_ms = 0;

    let hour = ((*now).tm_hour << 3) as u8;
    let min_hi = ((*now).tm_min >> 3) as u8;
    let min_low = ((*now).tm_min << 5) as u8;
    let sec = ((*now).tm_sec / 2) as u8;

    ndir.time[1] = (hour + min_hi) as u8;
    ndir.ctime[1] = ndir.time[1];
    ndir.time[0] = (min_low + sec) as u8;
    ndir.ctime[0] = ndir.time[0];

    let year = (((*now).tm_year - 80) << 1) as u8;
    let month_hi = ((*now).tm_mon + 1 >> 3) as u8;
    let month_low = (((*now).tm_mon + 1) << 5) as u8;
    let day = (*now).tm_mday as u8;

    ndir.date[1] = (year + month_hi) as u8;
    ndir.cdate[1] = ndir.date[1];
    ndir.adate[1] = ndir.cdate[1];
    ndir.date[0] = (month_low + day) as u8;
    ndir.cdate[0] = ndir.date[0];
    ndir.adate[0] = ndir.cdate[0];

    set_word(&mut ndir.start, (fat & 0xFFFF) as u16);
    set_word(&mut ndir.startHi, (fat >> 16) as u16);
    set_dword(&mut ndir.size, size);

    ndir
}

pub fn mk_entry_from_base(
    base: &str,
    attr: u8,
    fat: u32,
    size: u32,
    date: time_t,
    ndir: &mut directory,
) -> &mut directory {
    let mut dn = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };

    let base_cstr = CString::new(base).unwrap();
    let ext_cstr = CString::new("   ").unwrap();

    unsafe {
        libc::strncpy(
            dn.base.as_mut_ptr(),
            base_cstr.as_ptr(),
            dn.base.len() as libc::size_t,
        );
        libc::strncpy(
            dn.ext.as_mut_ptr(),
            ext_cstr.as_ptr(),
            dn.ext.len() as libc::size_t,
        );
    }

    let entry = mk_entry(&dn, attr, fat, size, date, ndir);
    entry.Case = 0;
    entry
}

// These would need to be implemented or wrapped properly
extern "C" {
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> c_int;
    fn getClusterBytes(File: *mut Fs_t) -> uint32_t;
    fn force_pread(Stream: *mut Stream_t, buf: *mut c_char, start: mt_off_t, len: size_t) -> ssize_t;
    fn force_pwrite(Stream: *mut Stream_t, buf: *mut c_char, start: mt_off_t, len: size_t) -> ssize_t;
    fn dosnameToDirentry(n: *const dos_name_t, dir: *mut directory);
    fn localtime(__timer: *const time_t) -> *mut tm;
}
use std::ffi::CString;
use std::os::raw::{c_char, c_uint, c_ulong, c_ushort, c_long, c_void};
use std::ptr;

pub type uint32_t = c_uint;
pub type ino_t = c_ulong;
pub type gid_t = c_uint;
pub type mode_t = c_uint;
pub type uid_t = c_uint;
pub type off_t = c_long;
pub type time_t = c_long;
pub type size_t = c_ulong;

#[derive(Clone)]
pub struct CpioFileStat {
    pub c_magic: c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: c_uint,
    pub c_dev_min: c_uint,
    pub c_rdev_maj: c_uint,
    pub c_rdev_min: c_uint,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: CString,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: Option<CString>,
}

#[derive(Clone)]
pub struct Deferment {
    pub next: Option<Box<Deferment>>,
    pub header: CpioFileStat,
}

impl Deferment {
    pub fn new(file_hdr: &CpioFileStat) -> Box<Self> {
        Box::new(Deferment {
            next: None,
            header: CpioFileStat {
                c_magic: file_hdr.c_magic,
                c_ino: file_hdr.c_ino,
                c_mode: file_hdr.c_mode,
                c_uid: file_hdr.c_uid,
                c_gid: file_hdr.c_gid,
                c_nlink: file_hdr.c_nlink,
                c_mtime: file_hdr.c_mtime,
                c_filesize: file_hdr.c_filesize,
                c_dev_maj: file_hdr.c_dev_maj,
                c_dev_min: file_hdr.c_dev_min,
                c_rdev_maj: file_hdr.c_rdev_maj,
                c_rdev_min: file_hdr.c_rdev_min,
                c_namesize: file_hdr.c_namesize,
                c_chksum: file_hdr.c_chksum,
                c_name: file_hdr.c_name.clone(),
                c_name_buflen: file_hdr.c_name_buflen,
                c_tar_linkname: file_hdr.c_tar_linkname.clone(),
            },
        })
    }
}

// Helper function to convert CString to raw pointer (for FFI if needed)
#[allow(dead_code)]
unsafe fn cstring_to_raw(s: &CString) -> *mut c_char {
    s.as_ptr() as *mut c_char
}

// Helper function to create CString from raw pointer (for FFI if needed)
#[allow(dead_code)]
unsafe fn raw_to_cstring(ptr: *const c_char) -> Option<CString> {
    if ptr.is_null() {
        None
    } else {
        Some(CString::from_raw(ptr as *mut c_char))
    }
}
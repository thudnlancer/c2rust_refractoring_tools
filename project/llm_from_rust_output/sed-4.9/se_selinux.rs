use std::ffi::CStr;
use std::io;
use std::os::raw::{c_char, c_int, c_uint, c_ushort};

pub type __mode_t = c_uint;
pub type mode_t = __mode_t;
pub type security_class_t = c_ushort;

fn set_errno(err: i32) {
    io::Error::last_os_error().raw_os_error().map(|_| {
        unsafe { *libc::__errno_location() = err as c_int };
    });
}

pub fn matchpathcon_init_prefix(path: &CStr, prefix: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn string_to_security_class(name: &CStr) -> io::Result<security_class_t> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn security_compute_create(
    scon: &CStr,
    tcon: &CStr,
    tclass: security_class_t,
) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn setexeccon(con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn security_check_context_raw(con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn security_check_context(con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn fsetfilecon(fd: c_int, con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn lsetfilecon(file: &CStr, con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn setfilecon(file: &CStr, con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn fgetfilecon(fd: c_int) -> io::Result<Box<CStr>> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn lgetfilecon(file: &CStr) -> io::Result<Box<CStr>> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn getfilecon(file: &CStr) -> io::Result<Box<CStr>> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn matchpathcon(file: &CStr, m: mode_t) -> io::Result<Box<CStr>> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn setfscreatecon(con: &CStr) -> io::Result<()> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn getfscreatecon() -> io::Result<Box<CStr>> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}

pub fn freecon(_con: Box<CStr>) {}

pub fn getcon() -> io::Result<Box<CStr>> {
    set_errno(95);
    Err(io::Error::from_raw_os_error(95))
}
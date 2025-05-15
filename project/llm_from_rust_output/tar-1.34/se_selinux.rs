use std::ffi::CStr;
use std::io;
use std::os::raw::{c_char, c_int, c_uint, c_ushort};

pub type __mode_t = c_uint;
pub type mode_t = __mode_t;
pub type security_class_t = c_ushort;

#[derive(Debug)]
pub enum SelinuxError {
    UnsupportedOperation,
}

impl From<SelinuxError> for io::Error {
    fn from(err: SelinuxError) -> io::Error {
        io::Error::new(io::ErrorKind::Unsupported, err)
    }
}

pub fn matchpathcon_init_prefix(
    _path: &CStr,
    _prefix: &CStr,
) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn getcon() -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn freecon(_con: String) {}

pub fn getfscreatecon() -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn setfscreatecon(_con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn matchpathcon(
    _file: &CStr,
    _m: mode_t,
) -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn getfilecon(_file: &CStr) -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn lgetfilecon(_file: &CStr) -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn fgetfilecon(_fd: c_int) -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn setfilecon(_file: &CStr, _con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn lsetfilecon(_file: &CStr, _con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn fsetfilecon(_fd: c_int, _con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn security_check_context(_con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn security_check_context_raw(_con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn setexeccon(_con: &CStr) -> Result<(), SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn security_compute_create(
    _scon: &CStr,
    _tcon: &CStr,
    _tclass: security_class_t,
) -> Result<String, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}

pub fn string_to_security_class(_name: &CStr) -> Result<security_class_t, SelinuxError> {
    Err(SelinuxError::UnsupportedOperation)
}
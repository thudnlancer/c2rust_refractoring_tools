use std::ffi::CString;
use std::io;
use std::os::raw::{c_char, c_int, c_uint, c_ushort};

pub type __mode_t = c_uint;
pub type mode_t = __mode_t;
pub type security_class_t = c_ushort;

#[derive(Debug)]
pub enum SelinuxError {
    OperationNotSupported,
}

impl From<SelinuxError> for io::Error {
    fn from(err: SelinuxError) -> Self {
        io::Error::new(io::ErrorKind::Unsupported, err)
    }
}

pub fn matchpathcon_init_prefix(_path: &str, _prefix: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn getcon() -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn freecon(_con: String) {}

pub fn getfscreatecon() -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn setfscreatecon(_con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn matchpathcon(_file: &str, _m: mode_t) -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn getfilecon(_file: &str) -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn lgetfilecon(_file: &str) -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn fgetfilecon(_fd: c_int) -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn setfilecon(_file: &str, _con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn lsetfilecon(_file: &str, _con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn fsetfilecon(_fd: c_int, _con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn security_check_context(_con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn security_check_context_raw(_con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn setexeccon(_con: &str) -> Result<(), SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn security_compute_create(
    _scon: &str,
    _tcon: &str,
    _tclass: security_class_t,
) -> Result<String, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}

pub fn string_to_security_class(_name: &str) -> Result<security_class_t, SelinuxError> {
    Err(SelinuxError::OperationNotSupported)
}
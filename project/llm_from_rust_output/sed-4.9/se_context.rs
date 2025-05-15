use std::ffi::{CStr, CString};
use std::io::{Error, ErrorKind};

pub type Context = i32;

#[derive(Debug)]
pub enum ContextError {
    UnsupportedOperation,
}

impl From<ContextError> for Error {
    fn from(e: ContextError) -> Error {
        Error::new(ErrorKind::Other, format!("{:?}", e))
    }
}

pub fn context_new(s: &CStr) -> Result<Context, ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_str(con: Context) -> Result<CString, ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_free(_c: Context) {}

pub fn context_user_set(sc: Context, s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_role_set(sc: Context, s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_range_set(sc: Context, s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_type_set(sc: Context, s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_type_get(sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_range_get(sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_role_get(sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::UnsupportedOperation)
}

pub fn context_user_get(sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::UnsupportedOperation)
}
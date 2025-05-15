use std::ffi::{CStr, CString};
use std::io::{Error, ErrorKind};

pub type Context = i32;

#[derive(Debug)]
pub enum ContextError {
    NotSupported,
}

impl From<ContextError> for Error {
    fn from(e: ContextError) -> Error {
        Error::new(ErrorKind::Other, format!("{:?}", e))
    }
}

pub fn context_new(_s: &CStr) -> Result<Context, ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_str(_con: Context) -> Result<CString, ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_free(_c: Context) {}

pub fn context_user_set(_sc: Context, _s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_role_set(_sc: Context, _s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_range_set(_sc: Context, _s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_type_set(_sc: Context, _s: &CStr) -> Result<(), ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_type_get(_sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_range_get(_sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_role_get(_sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::NotSupported)
}

pub fn context_user_get(_sc: Context) -> Result<CString, ContextError> {
    Err(ContextError::NotSupported)
}
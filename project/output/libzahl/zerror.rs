#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    static mut libzahl_error: libc::c_int;
    fn abort() -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_ERRNO_SET = 0,
    ZERROR_0_POW_0,
    ZERROR_0_DIV_0,
    ZERROR_DIV_0,
    ZERROR_NEGATIVE,
    ZERROR_INVALID_RADIX,
impl zerror {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            zerror::ZERROR_ERRNO_SET => 0,
            zerror::ZERROR_0_POW_0 => 1,
            zerror::ZERROR_0_DIV_0 => 2,
            zerror::ZERROR_DIV_0 => 3,
            zerror::ZERROR_NEGATIVE => 4,
            zerror::ZERROR_INVALID_RADIX => 5,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn zerror(mut desc: *mut *const libc::c_char) -> zerror {
    if libzahl_error >= 0 as libc::c_int {
        if !desc.is_null() {
            *desc = strerror(libzahl_error);
        }
        *__errno_location() = libzahl_error;
        return ZERROR_ERRNO_SET;
    }
    if !desc.is_null() {
        match -libzahl_error {
            1 => {
                *desc = b"indeterminate form: 0:th power of 0\0" as *const u8
                    as *const libc::c_char;
            }
            2 => {
                *desc = b"indeterminate form: 0 divided by 0\0" as *const u8
                    as *const libc::c_char;
            }
            3 => {
                *desc = b"undefined result: division by 0\0" as *const u8
                    as *const libc::c_char;
            }
            4 => {
                *desc = b"argument must be non-negative\0" as *const u8
                    as *const libc::c_char;
            }
            _ => {
                abort();
            }
        }
    }
    return -libzahl_error as zerror;
}

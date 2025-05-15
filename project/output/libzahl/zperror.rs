use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn zerror(_: *mut *const i8) -> zerror;
    static mut libzahl_error: i32;
    fn __errno_location() -> *mut i32;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_ERRNO_SET = 0,
    ZERROR_0_POW_0,
    ZERROR_0_DIV_0,
    ZERROR_DIV_0,
    ZERROR_NEGATIVE,
    ZERROR_INVALID_RADIX,
}
impl zerror {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zerror::ZERROR_ERRNO_SET => 0,
            zerror::ZERROR_0_POW_0 => 1,
            zerror::ZERROR_0_DIV_0 => 2,
            zerror::ZERROR_DIV_0 => 3,
            zerror::ZERROR_NEGATIVE => 4,
            zerror::ZERROR_INVALID_RADIX => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> zerror {
        match value {
            0 => zerror::ZERROR_ERRNO_SET,
            1 => zerror::ZERROR_0_POW_0,
            2 => zerror::ZERROR_0_DIV_0,
            3 => zerror::ZERROR_DIV_0,
            4 => zerror::ZERROR_NEGATIVE,
            5 => zerror::ZERROR_INVALID_RADIX,
            _ => panic!("Invalid value for zerror: {}", value),
        }
    }
}
impl AddAssign<u32> for zerror {
    fn add_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zerror {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zerror {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zerror {
    fn div_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zerror {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zerror {
    type Output = zerror;
    fn add(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zerror {
    type Output = zerror;
    fn sub(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zerror {
    type Output = zerror;
    fn mul(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zerror {
    type Output = zerror;
    fn div(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zerror {
    type Output = zerror;
    fn rem(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
#[no_mangle]
pub unsafe extern "C" fn zperror(mut prefix: *const i8) {
    if libzahl_error >= 0 as i32 {
        *__errno_location() = libzahl_error;
        perror(prefix);
    } else {
        let mut desc: *const i8 = 0 as *const i8;
        zerror(&mut desc);
        if !prefix.is_null() && *prefix as i32 != 0 {
            fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const i8, prefix, desc);
        } else {
            fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, desc);
        }
    };
}
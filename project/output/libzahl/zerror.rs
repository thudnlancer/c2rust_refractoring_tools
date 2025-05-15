use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    static mut libzahl_error: i32;
    fn abort() -> !;
    fn __errno_location() -> *mut i32;
    fn strerror(_: i32) -> *mut i8;
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
#[no_mangle]
pub unsafe extern "C" fn zerror(mut desc: *mut *const i8) -> zerror {
    if libzahl_error >= 0 as i32 {
        if !desc.is_null() {
            *desc = strerror(libzahl_error);
        }
        *__errno_location() = libzahl_error;
        return zerror::ZERROR_ERRNO_SET;
    }
    if !desc.is_null() {
        match -libzahl_error {
            1 => {
                *desc = b"indeterminate form: 0:th power of 0\0" as *const u8
                    as *const i8;
            }
            2 => {
                *desc = b"indeterminate form: 0 divided by 0\0" as *const u8
                    as *const i8;
            }
            3 => {
                *desc = b"undefined result: division by 0\0" as *const u8 as *const i8;
            }
            4 => {
                *desc = b"argument must be non-negative\0" as *const u8 as *const i8;
            }
            _ => {
                abort();
            }
        }
    }
    return zerror::from_libc_c_uint(-libzahl_error as u32);
}
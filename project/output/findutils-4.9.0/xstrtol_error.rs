#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn abort() -> !;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    static mut exit_failure: i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum strtol_error {
    LONGINT_INVALID = 4,
    LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW = 3,
    LONGINT_INVALID_SUFFIX_CHAR = 2,
    LONGINT_OVERFLOW = 1,
    LONGINT_OK = 0,
}
impl strtol_error {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            strtol_error::LONGINT_INVALID => 4,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => 3,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR => 2,
            strtol_error::LONGINT_OVERFLOW => 1,
            strtol_error::LONGINT_OK => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> strtol_error {
        match value {
            4 => strtol_error::LONGINT_INVALID,
            3 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
            2 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR,
            1 => strtol_error::LONGINT_OVERFLOW,
            0 => strtol_error::LONGINT_OK,
            _ => panic!("Invalid value for strtol_error: {}", value),
        }
    }
}
impl AddAssign<u32> for strtol_error {
    fn add_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for strtol_error {
    fn sub_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for strtol_error {
    fn mul_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for strtol_error {
    fn div_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for strtol_error {
    fn rem_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for strtol_error {
    type Output = strtol_error;
    fn add(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for strtol_error {
    type Output = strtol_error;
    fn sub(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for strtol_error {
    type Output = strtol_error;
    fn mul(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for strtol_error {
    type Output = strtol_error;
    fn div(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for strtol_error {
    type Output = strtol_error;
    fn rem(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
unsafe extern "C" fn xstrtol_error(
    mut err: strtol_error,
    mut opt_idx: i32,
    mut c: i8,
    mut long_options: *const option,
    mut arg: *const i8,
    mut exit_status: i32,
) {
    let mut hyphens: *const i8 = b"--\0" as *const u8 as *const i8;
    let mut msgid: *const i8 = 0 as *const i8;
    let mut option: *const i8 = 0 as *const i8;
    let mut option_buffer: [i8; 2] = [0; 2];
    match err as u32 {
        4 => {
            msgid = b"invalid %s%s argument '%s'\0" as *const u8 as *const i8;
        }
        2 | 3 => {
            msgid = b"invalid suffix in %s%s argument '%s'\0" as *const u8 as *const i8;
        }
        1 => {
            msgid = b"%s%s argument '%s' too large\0" as *const u8 as *const i8;
        }
        _ => {
            abort();
        }
    }
    if opt_idx < 0 as i32 {
        hyphens = hyphens.offset(-(opt_idx as isize));
        option_buffer[0 as i32 as usize] = c;
        option_buffer[1 as i32 as usize] = '\0' as i32 as i8;
        option = option_buffer.as_mut_ptr();
    } else {
        option = (*long_options.offset(opt_idx as isize)).name;
    }
    error(
        exit_status,
        0 as i32,
        dcgettext(0 as *const i8, msgid, 5 as i32),
        hyphens,
        option,
        arg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtol_fatal(
    mut err: strtol_error,
    mut opt_idx: i32,
    mut c: i8,
    mut long_options: *const option,
    mut arg: *const i8,
) {
    xstrtol_error(err, opt_idx, c, long_options, arg, exit_failure);
    abort();
}
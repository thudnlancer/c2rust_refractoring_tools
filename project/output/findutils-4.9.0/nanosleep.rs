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
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> i32;
    fn __errno_location() -> *mut i32;
}
pub type __time_t = i64;
pub type __syscall_slong_t = i64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    BILLION = 1000000000,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::BILLION => 1000000000,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            1000000000 => C2RustUnnamed::BILLION,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub unsafe extern "C" fn rpl_nanosleep(
    mut requested_delay: *const timespec,
    mut remaining_delay: *mut timespec,
) -> i32 {
    if (*requested_delay).tv_nsec < 0 as i32 as i64
        || C2RustUnnamed::BILLION as i32 as i64 <= (*requested_delay).tv_nsec
    {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    let limit: time_t = (24 as i32 * 24 as i32 * 60 as i32 * 60 as i32) as time_t;
    let mut seconds: time_t = (*requested_delay).tv_sec;
    let mut intermediate: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    intermediate.tv_nsec = (*requested_delay).tv_nsec;
    while limit < seconds {
        let mut result: i32 = 0;
        intermediate.tv_sec = limit;
        result = nanosleep(&mut intermediate, remaining_delay);
        seconds -= limit;
        if result != 0 {
            if !remaining_delay.is_null() {
                (*remaining_delay).tv_sec += seconds;
            }
            return result;
        }
        intermediate.tv_nsec = 0 as i32 as __syscall_slong_t;
    }
    intermediate.tv_sec = seconds;
    return nanosleep(&mut intermediate, remaining_delay);
}
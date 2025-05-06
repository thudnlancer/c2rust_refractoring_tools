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
    fn rpl_getopt_internal(
        ___argc: i32,
        ___argv: *mut *mut i8,
        __shortopts: *const i8,
        __longopts: *const rpl_option,
        __longind: *mut i32,
        __long_only: i32,
        __posixly_correct: i32,
    ) -> i32;
    fn _getopt_internal_r(
        ___argc: i32,
        ___argv: *mut *mut i8,
        __shortopts: *const i8,
        __longopts: *const rpl_option,
        __longind: *mut i32,
        __long_only: i32,
        __data: *mut _getopt_data,
        __posixly_correct: i32,
    ) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __ord {
    REQUIRE_ORDER,
    PERMUTE,
    RETURN_IN_ORDER,
}
impl __ord {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __ord::REQUIRE_ORDER => 0,
            __ord::PERMUTE => 1,
            __ord::RETURN_IN_ORDER => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> __ord {
        match value {
            0 => __ord::REQUIRE_ORDER,
            1 => __ord::PERMUTE,
            2 => __ord::RETURN_IN_ORDER,
            _ => panic!("Invalid value for __ord: {}", value),
        }
    }
}
impl AddAssign<u32> for __ord {
    fn add_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __ord {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __ord {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __ord {
    fn div_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __ord {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __ord {
    type Output = __ord;
    fn add(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __ord {
    type Output = __ord;
    fn sub(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __ord {
    type Output = __ord;
    fn mul(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __ord {
    type Output = __ord;
    fn div(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __ord {
    type Output = __ord;
    fn rem(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub rpl_optind: i32,
    pub rpl_opterr: i32,
    pub rpl_optopt: i32,
    pub rpl_optarg: *mut i8,
    pub __initialized: i32,
    pub __nextchar: *mut i8,
    pub __ordering: __ord,
    pub __first_nonopt: i32,
    pub __last_nonopt: i32,
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt_long(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
) -> i32 {
    return rpl_getopt_internal(
        argc,
        argv as *mut *mut i8,
        options,
        long_options,
        opt_index,
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_long_r(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
    mut d: *mut _getopt_data,
) -> i32 {
    return _getopt_internal_r(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        0 as i32,
        d,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt_long_only(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
) -> i32 {
    return rpl_getopt_internal(
        argc,
        argv as *mut *mut i8,
        options,
        long_options,
        opt_index,
        1 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_long_only_r(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
    mut d: *mut _getopt_data,
) -> i32 {
    return _getopt_internal_r(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        1 as i32,
        d,
        0 as i32,
    );
}
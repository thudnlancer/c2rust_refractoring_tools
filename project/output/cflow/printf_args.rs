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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type wint_t = u32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum arg_type {
    TYPE_COUNT_LONGLONGINT_POINTER = 22,
    TYPE_COUNT_LONGINT_POINTER = 21,
    TYPE_COUNT_INT_POINTER = 20,
    TYPE_COUNT_SHORT_POINTER = 19,
    TYPE_COUNT_SCHAR_POINTER = 18,
    TYPE_POINTER = 17,
    TYPE_WIDE_STRING = 16,
    TYPE_STRING = 15,
    TYPE_WIDE_CHAR = 14,
    TYPE_CHAR = 13,
    TYPE_LONGDOUBLE = 12,
    TYPE_DOUBLE = 11,
    TYPE_ULONGLONGINT = 10,
    TYPE_LONGLONGINT = 9,
    TYPE_ULONGINT = 8,
    TYPE_LONGINT = 7,
    TYPE_UINT = 6,
    TYPE_INT = 5,
    TYPE_USHORT = 4,
    TYPE_SHORT = 3,
    TYPE_UCHAR = 2,
    TYPE_SCHAR = 1,
    TYPE_NONE = 0,
}
impl arg_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            arg_type::TYPE_COUNT_LONGLONGINT_POINTER => 22,
            arg_type::TYPE_COUNT_LONGINT_POINTER => 21,
            arg_type::TYPE_COUNT_INT_POINTER => 20,
            arg_type::TYPE_COUNT_SHORT_POINTER => 19,
            arg_type::TYPE_COUNT_SCHAR_POINTER => 18,
            arg_type::TYPE_POINTER => 17,
            arg_type::TYPE_WIDE_STRING => 16,
            arg_type::TYPE_STRING => 15,
            arg_type::TYPE_WIDE_CHAR => 14,
            arg_type::TYPE_CHAR => 13,
            arg_type::TYPE_LONGDOUBLE => 12,
            arg_type::TYPE_DOUBLE => 11,
            arg_type::TYPE_ULONGLONGINT => 10,
            arg_type::TYPE_LONGLONGINT => 9,
            arg_type::TYPE_ULONGINT => 8,
            arg_type::TYPE_LONGINT => 7,
            arg_type::TYPE_UINT => 6,
            arg_type::TYPE_INT => 5,
            arg_type::TYPE_USHORT => 4,
            arg_type::TYPE_SHORT => 3,
            arg_type::TYPE_UCHAR => 2,
            arg_type::TYPE_SCHAR => 1,
            arg_type::TYPE_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> arg_type {
        match value {
            22 => arg_type::TYPE_COUNT_LONGLONGINT_POINTER,
            21 => arg_type::TYPE_COUNT_LONGINT_POINTER,
            20 => arg_type::TYPE_COUNT_INT_POINTER,
            19 => arg_type::TYPE_COUNT_SHORT_POINTER,
            18 => arg_type::TYPE_COUNT_SCHAR_POINTER,
            17 => arg_type::TYPE_POINTER,
            16 => arg_type::TYPE_WIDE_STRING,
            15 => arg_type::TYPE_STRING,
            14 => arg_type::TYPE_WIDE_CHAR,
            13 => arg_type::TYPE_CHAR,
            12 => arg_type::TYPE_LONGDOUBLE,
            11 => arg_type::TYPE_DOUBLE,
            10 => arg_type::TYPE_ULONGLONGINT,
            9 => arg_type::TYPE_LONGLONGINT,
            8 => arg_type::TYPE_ULONGINT,
            7 => arg_type::TYPE_LONGINT,
            6 => arg_type::TYPE_UINT,
            5 => arg_type::TYPE_INT,
            4 => arg_type::TYPE_USHORT,
            3 => arg_type::TYPE_SHORT,
            2 => arg_type::TYPE_UCHAR,
            1 => arg_type::TYPE_SCHAR,
            0 => arg_type::TYPE_NONE,
            _ => panic!("Invalid value for arg_type: {}", value),
        }
    }
}
impl AddAssign<u32> for arg_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for arg_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for arg_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for arg_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for arg_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = arg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for arg_type {
    type Output = arg_type;
    fn add(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for arg_type {
    type Output = arg_type;
    fn sub(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for arg_type {
    type Output = arg_type;
    fn mul(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for arg_type {
    type Output = arg_type;
    fn div(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for arg_type {
    type Output = arg_type;
    fn rem(self, rhs: u32) -> arg_type {
        arg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argument {
    pub type_0: arg_type,
    pub a: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub a_schar: libc::c_schar,
    pub a_uchar: u8,
    pub a_short: libc::c_short,
    pub a_ushort: libc::c_ushort,
    pub a_int: i32,
    pub a_uint: u32,
    pub a_longint: i64,
    pub a_ulongint: u64,
    pub a_longlongint: libc::c_longlong,
    pub a_ulonglongint: libc::c_ulonglong,
    pub a_float: libc::c_float,
    pub a_double: libc::c_double,
    pub a_longdouble: f128::f128,
    pub a_char: i32,
    pub a_wide_char: wint_t,
    pub a_string: *const i8,
    pub a_wide_string: *const wchar_t,
    pub a_pointer: *mut libc::c_void,
    pub a_count_schar_pointer: *mut libc::c_schar,
    pub a_count_short_pointer: *mut libc::c_short,
    pub a_count_int_pointer: *mut i32,
    pub a_count_longint_pointer: *mut i64,
    pub a_count_longlongint_pointer: *mut libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arguments {
    pub count: size_t,
    pub arg: *mut argument,
}
#[no_mangle]
pub unsafe extern "C" fn printf_fetchargs(
    mut args: ::core::ffi::VaList,
    mut a: *mut arguments,
) -> i32 {
    let mut i: size_t = 0;
    let mut ap: *mut argument = 0 as *mut argument;
    i = 0 as i32 as size_t;
    ap = &mut *((*a).arg).offset(0 as i32 as isize) as *mut argument;
    while i < (*a).count {
        match (*ap).type_0 as u32 {
            1 => {
                (*ap).a.a_schar = args.arg::<i32>() as libc::c_schar;
            }
            2 => {
                (*ap).a.a_uchar = args.arg::<i32>() as u8;
            }
            3 => {
                (*ap).a.a_short = args.arg::<i32>() as libc::c_short;
            }
            4 => {
                (*ap).a.a_ushort = args.arg::<i32>() as libc::c_ushort;
            }
            5 => {
                (*ap).a.a_int = args.arg::<i32>();
            }
            6 => {
                (*ap).a.a_uint = args.arg::<u32>();
            }
            7 => {
                (*ap).a.a_longint = args.arg::<i64>();
            }
            8 => {
                (*ap).a.a_ulongint = args.arg::<u64>();
            }
            9 => {
                (*ap).a.a_longlongint = args.arg::<libc::c_longlong>();
            }
            10 => {
                (*ap).a.a_ulonglongint = args.arg::<libc::c_ulonglong>();
            }
            11 => {
                (*ap).a.a_double = args.arg::<libc::c_double>();
            }
            12 => {
                (*ap).a.a_longdouble = args.arg::<f128::f128>();
            }
            13 => {
                (*ap).a.a_char = args.arg::<i32>();
            }
            14 => {
                (*ap).a.a_wide_char = if (::core::mem::size_of::<wint_t>() as u64)
                    < ::core::mem::size_of::<i32>() as u64
                {
                    args.arg::<i32>() as u32
                } else {
                    args.arg::<wint_t>()
                };
            }
            15 => {
                (*ap).a.a_string = args.arg::<*const i8>();
                if ((*ap).a.a_string).is_null() {
                    (*ap).a.a_string = b"(NULL)\0" as *const u8 as *const i8;
                }
            }
            16 => {
                (*ap).a.a_wide_string = args.arg::<*const wchar_t>();
                if ((*ap).a.a_wide_string).is_null() {
                    static mut wide_null_string: [wchar_t; 7] = [
                        '(' as i32,
                        'N' as i32,
                        'U' as i32,
                        'L' as i32,
                        'L' as i32,
                        ')' as i32,
                        0 as i32,
                    ];
                    (*ap).a.a_wide_string = wide_null_string.as_ptr();
                }
            }
            17 => {
                (*ap).a.a_pointer = args.arg::<*mut libc::c_void>();
            }
            18 => {
                (*ap).a.a_count_schar_pointer = args.arg::<*mut libc::c_schar>();
            }
            19 => {
                (*ap).a.a_count_short_pointer = args.arg::<*mut libc::c_short>();
            }
            20 => {
                (*ap).a.a_count_int_pointer = args.arg::<*mut i32>();
            }
            21 => {
                (*ap).a.a_count_longint_pointer = args.arg::<*mut i64>();
            }
            22 => {
                (*ap).a.a_count_longlongint_pointer = args
                    .arg::<*mut libc::c_longlong>();
            }
            _ => return -(1 as i32),
        }
        i = i.wrapping_add(1);
        i;
        ap = ap.offset(1);
        ap;
    }
    return 0 as i32;
}
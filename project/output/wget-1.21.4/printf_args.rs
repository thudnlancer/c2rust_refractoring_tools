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
pub type __int8_t = libc::c_schar;
pub type __uint8_t = u8;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = libc::c_schar;
pub type int_fast16_t = i64;
pub type int_fast32_t = i64;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum arg_type {
    TYPE_COUNT_INT_FAST64_T_POINTER = 46,
    TYPE_COUNT_INT_FAST32_T_POINTER = 45,
    TYPE_COUNT_INT_FAST16_T_POINTER = 44,
    TYPE_COUNT_INT_FAST8_T_POINTER = 43,
    TYPE_COUNT_INT64_T_POINTER = 42,
    TYPE_COUNT_INT32_T_POINTER = 41,
    TYPE_COUNT_INT16_T_POINTER = 40,
    TYPE_COUNT_INT8_T_POINTER = 39,
    TYPE_COUNT_LONGLONGINT_POINTER = 38,
    TYPE_COUNT_LONGINT_POINTER = 37,
    TYPE_COUNT_INT_POINTER = 36,
    TYPE_COUNT_SHORT_POINTER = 35,
    TYPE_COUNT_SCHAR_POINTER = 34,
    TYPE_POINTER = 33,
    TYPE_WIDE_STRING = 32,
    TYPE_STRING = 31,
    TYPE_WIDE_CHAR = 30,
    TYPE_CHAR = 29,
    TYPE_LONGDOUBLE = 28,
    TYPE_DOUBLE = 27,
    TYPE_UINT_FAST64_T = 26,
    TYPE_INT_FAST64_T = 25,
    TYPE_UINT_FAST32_T = 24,
    TYPE_INT_FAST32_T = 23,
    TYPE_UINT_FAST16_T = 22,
    TYPE_INT_FAST16_T = 21,
    TYPE_UINT_FAST8_T = 20,
    TYPE_INT_FAST8_T = 19,
    TYPE_UINT64_T = 18,
    TYPE_INT64_T = 17,
    TYPE_UINT32_T = 16,
    TYPE_INT32_T = 15,
    TYPE_UINT16_T = 14,
    TYPE_INT16_T = 13,
    TYPE_UINT8_T = 12,
    TYPE_INT8_T = 11,
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
            arg_type::TYPE_COUNT_INT_FAST64_T_POINTER => 46,
            arg_type::TYPE_COUNT_INT_FAST32_T_POINTER => 45,
            arg_type::TYPE_COUNT_INT_FAST16_T_POINTER => 44,
            arg_type::TYPE_COUNT_INT_FAST8_T_POINTER => 43,
            arg_type::TYPE_COUNT_INT64_T_POINTER => 42,
            arg_type::TYPE_COUNT_INT32_T_POINTER => 41,
            arg_type::TYPE_COUNT_INT16_T_POINTER => 40,
            arg_type::TYPE_COUNT_INT8_T_POINTER => 39,
            arg_type::TYPE_COUNT_LONGLONGINT_POINTER => 38,
            arg_type::TYPE_COUNT_LONGINT_POINTER => 37,
            arg_type::TYPE_COUNT_INT_POINTER => 36,
            arg_type::TYPE_COUNT_SHORT_POINTER => 35,
            arg_type::TYPE_COUNT_SCHAR_POINTER => 34,
            arg_type::TYPE_POINTER => 33,
            arg_type::TYPE_WIDE_STRING => 32,
            arg_type::TYPE_STRING => 31,
            arg_type::TYPE_WIDE_CHAR => 30,
            arg_type::TYPE_CHAR => 29,
            arg_type::TYPE_LONGDOUBLE => 28,
            arg_type::TYPE_DOUBLE => 27,
            arg_type::TYPE_UINT_FAST64_T => 26,
            arg_type::TYPE_INT_FAST64_T => 25,
            arg_type::TYPE_UINT_FAST32_T => 24,
            arg_type::TYPE_INT_FAST32_T => 23,
            arg_type::TYPE_UINT_FAST16_T => 22,
            arg_type::TYPE_INT_FAST16_T => 21,
            arg_type::TYPE_UINT_FAST8_T => 20,
            arg_type::TYPE_INT_FAST8_T => 19,
            arg_type::TYPE_UINT64_T => 18,
            arg_type::TYPE_INT64_T => 17,
            arg_type::TYPE_UINT32_T => 16,
            arg_type::TYPE_INT32_T => 15,
            arg_type::TYPE_UINT16_T => 14,
            arg_type::TYPE_INT16_T => 13,
            arg_type::TYPE_UINT8_T => 12,
            arg_type::TYPE_INT8_T => 11,
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
            46 => arg_type::TYPE_COUNT_INT_FAST64_T_POINTER,
            45 => arg_type::TYPE_COUNT_INT_FAST32_T_POINTER,
            44 => arg_type::TYPE_COUNT_INT_FAST16_T_POINTER,
            43 => arg_type::TYPE_COUNT_INT_FAST8_T_POINTER,
            42 => arg_type::TYPE_COUNT_INT64_T_POINTER,
            41 => arg_type::TYPE_COUNT_INT32_T_POINTER,
            40 => arg_type::TYPE_COUNT_INT16_T_POINTER,
            39 => arg_type::TYPE_COUNT_INT8_T_POINTER,
            38 => arg_type::TYPE_COUNT_LONGLONGINT_POINTER,
            37 => arg_type::TYPE_COUNT_LONGINT_POINTER,
            36 => arg_type::TYPE_COUNT_INT_POINTER,
            35 => arg_type::TYPE_COUNT_SHORT_POINTER,
            34 => arg_type::TYPE_COUNT_SCHAR_POINTER,
            33 => arg_type::TYPE_POINTER,
            32 => arg_type::TYPE_WIDE_STRING,
            31 => arg_type::TYPE_STRING,
            30 => arg_type::TYPE_WIDE_CHAR,
            29 => arg_type::TYPE_CHAR,
            28 => arg_type::TYPE_LONGDOUBLE,
            27 => arg_type::TYPE_DOUBLE,
            26 => arg_type::TYPE_UINT_FAST64_T,
            25 => arg_type::TYPE_INT_FAST64_T,
            24 => arg_type::TYPE_UINT_FAST32_T,
            23 => arg_type::TYPE_INT_FAST32_T,
            22 => arg_type::TYPE_UINT_FAST16_T,
            21 => arg_type::TYPE_INT_FAST16_T,
            20 => arg_type::TYPE_UINT_FAST8_T,
            19 => arg_type::TYPE_INT_FAST8_T,
            18 => arg_type::TYPE_UINT64_T,
            17 => arg_type::TYPE_INT64_T,
            16 => arg_type::TYPE_UINT32_T,
            15 => arg_type::TYPE_INT32_T,
            14 => arg_type::TYPE_UINT16_T,
            13 => arg_type::TYPE_INT16_T,
            12 => arg_type::TYPE_UINT8_T,
            11 => arg_type::TYPE_INT8_T,
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
    pub a_int8_t: int8_t,
    pub a_uint8_t: uint8_t,
    pub a_int16_t: int16_t,
    pub a_uint16_t: uint16_t,
    pub a_int32_t: int32_t,
    pub a_uint32_t: uint32_t,
    pub a_int64_t: int64_t,
    pub a_uint64_t: uint64_t,
    pub a_int_fast8_t: int_fast8_t,
    pub a_uint_fast8_t: uint_fast8_t,
    pub a_int_fast16_t: int_fast16_t,
    pub a_uint_fast16_t: uint_fast16_t,
    pub a_int_fast32_t: int_fast32_t,
    pub a_uint_fast32_t: uint_fast32_t,
    pub a_int_fast64_t: int_fast64_t,
    pub a_uint_fast64_t: uint_fast64_t,
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
    pub a_count_int8_t_pointer: *mut int8_t,
    pub a_count_int16_t_pointer: *mut int16_t,
    pub a_count_int32_t_pointer: *mut int32_t,
    pub a_count_int64_t_pointer: *mut int64_t,
    pub a_count_int_fast8_t_pointer: *mut int_fast8_t,
    pub a_count_int_fast16_t_pointer: *mut int_fast16_t,
    pub a_count_int_fast32_t_pointer: *mut int_fast32_t,
    pub a_count_int_fast64_t_pointer: *mut int_fast64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arguments {
    pub count: size_t,
    pub arg: *mut argument,
    pub direct_alloc_arg: [argument; 7],
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
                (*ap).a.a_int8_t = args.arg::<i32>() as int8_t;
            }
            12 => {
                (*ap).a.a_uint8_t = args.arg::<i32>() as uint8_t;
            }
            13 => {
                (*ap).a.a_int16_t = args.arg::<i32>() as int16_t;
            }
            14 => {
                (*ap).a.a_uint16_t = args.arg::<i32>() as uint16_t;
            }
            15 => {
                (*ap).a.a_int32_t = args.arg::<int32_t>();
            }
            16 => {
                (*ap).a.a_uint32_t = args.arg::<uint32_t>();
            }
            17 => {
                (*ap).a.a_int64_t = args.arg::<int64_t>();
            }
            18 => {
                (*ap).a.a_uint64_t = args.arg::<uint64_t>();
            }
            19 => {
                (*ap).a.a_int_fast8_t = args.arg::<i32>() as int_fast8_t;
            }
            20 => {
                (*ap).a.a_uint_fast8_t = args.arg::<i32>() as uint_fast8_t;
            }
            21 => {
                (*ap).a.a_int_fast16_t = args.arg::<int_fast16_t>();
            }
            22 => {
                (*ap).a.a_uint_fast16_t = args.arg::<uint_fast16_t>();
            }
            23 => {
                (*ap).a.a_int_fast32_t = args.arg::<int_fast32_t>();
            }
            24 => {
                (*ap).a.a_uint_fast32_t = args.arg::<uint_fast32_t>();
            }
            25 => {
                (*ap).a.a_int_fast64_t = args.arg::<int_fast64_t>();
            }
            26 => {
                (*ap).a.a_uint_fast64_t = args.arg::<uint_fast64_t>();
            }
            27 => {
                (*ap).a.a_double = args.arg::<libc::c_double>();
            }
            28 => {
                (*ap).a.a_longdouble = args.arg::<f128::f128>();
            }
            29 => {
                (*ap).a.a_char = args.arg::<i32>();
            }
            30 => {
                (*ap).a.a_wide_char = if (::core::mem::size_of::<wint_t>() as u64)
                    < ::core::mem::size_of::<i32>() as u64
                {
                    args.arg::<i32>() as wint_t
                } else {
                    args.arg::<wint_t>()
                };
            }
            31 => {
                (*ap).a.a_string = args.arg::<*const i8>();
                if ((*ap).a.a_string).is_null() {
                    (*ap).a.a_string = b"(NULL)\0" as *const u8 as *const i8;
                }
            }
            32 => {
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
            33 => {
                (*ap).a.a_pointer = args.arg::<*mut libc::c_void>();
            }
            34 => {
                (*ap).a.a_count_schar_pointer = args.arg::<*mut libc::c_schar>();
            }
            35 => {
                (*ap).a.a_count_short_pointer = args.arg::<*mut libc::c_short>();
            }
            36 => {
                (*ap).a.a_count_int_pointer = args.arg::<*mut i32>();
            }
            37 => {
                (*ap).a.a_count_longint_pointer = args.arg::<*mut i64>();
            }
            38 => {
                (*ap).a.a_count_longlongint_pointer = args
                    .arg::<*mut libc::c_longlong>();
            }
            39 => {
                (*ap).a.a_count_int8_t_pointer = args.arg::<*mut int8_t>();
            }
            40 => {
                (*ap).a.a_count_int16_t_pointer = args.arg::<*mut int16_t>();
            }
            41 => {
                (*ap).a.a_count_int32_t_pointer = args.arg::<*mut int32_t>();
            }
            42 => {
                (*ap).a.a_count_int64_t_pointer = args.arg::<*mut int64_t>();
            }
            43 => {
                (*ap).a.a_count_int_fast8_t_pointer = args.arg::<*mut int_fast8_t>();
            }
            44 => {
                (*ap).a.a_count_int_fast16_t_pointer = args.arg::<*mut int_fast16_t>();
            }
            45 => {
                (*ap).a.a_count_int_fast32_t_pointer = args.arg::<*mut int_fast32_t>();
            }
            46 => {
                (*ap).a.a_count_int_fast64_t_pointer = args.arg::<*mut int_fast64_t>();
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
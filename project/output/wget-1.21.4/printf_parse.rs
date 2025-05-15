use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn rpl_free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
}
pub type ptrdiff_t = i64;
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
pub type __intmax_t = i64;
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
pub type intmax_t = __intmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_directive {
    pub dir_start: *const i8,
    pub dir_end: *const i8,
    pub flags: i32,
    pub width_start: *const i8,
    pub width_end: *const i8,
    pub width_arg_index: size_t,
    pub precision_start: *const i8,
    pub precision_end: *const i8,
    pub precision_arg_index: size_t,
    pub conversion: i8,
    pub arg_index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_directives {
    pub count: size_t,
    pub dir: *mut char_directive,
    pub max_width_length: size_t,
    pub max_precision_length: size_t,
    pub direct_alloc_dir: [char_directive; 7],
}
#[inline]
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as u64 };
}
#[no_mangle]
pub unsafe extern "C" fn printf_parse(
    mut format: *const i8,
    mut d: *mut char_directives,
    mut a: *mut arguments,
) -> i32 {
    let mut current_block: u64;
    let mut cp: *const i8 = format;
    let mut arg_posn: size_t = 0 as i32 as size_t;
    let mut d_allocated: size_t = 0;
    let mut a_allocated: size_t = 0;
    let mut max_width_length: size_t = 0 as i32 as size_t;
    let mut max_precision_length: size_t = 0 as i32 as size_t;
    (*d).count = 0 as i32 as size_t;
    d_allocated = 7 as i32 as size_t;
    (*d).dir = ((*d).direct_alloc_dir).as_mut_ptr();
    (*a).count = 0 as i32 as size_t;
    a_allocated = 7 as i32 as size_t;
    (*a).arg = ((*a).direct_alloc_arg).as_mut_ptr();
    loop {
        if !(*cp as i32 != '\0' as i32) {
            current_block = 9028248570441629388;
            break;
        }
        let fresh0 = cp;
        cp = cp.offset(1);
        let mut c: i8 = *fresh0;
        if !(c as i32 == '%' as i32) {
            continue;
        }
        let mut arg_index: size_t = !(0 as i32 as size_t);
        let mut dp: *mut char_directive = &mut *((*d).dir).offset((*d).count as isize)
            as *mut char_directive;
        (*dp).dir_start = cp.offset(-(1 as i32 as isize));
        (*dp).flags = 0 as i32;
        (*dp).width_start = 0 as *const i8;
        (*dp).width_end = 0 as *const i8;
        (*dp).width_arg_index = !(0 as i32 as size_t);
        (*dp).precision_start = 0 as *const i8;
        (*dp).precision_end = 0 as *const i8;
        (*dp).precision_arg_index = !(0 as i32 as size_t);
        (*dp).arg_index = !(0 as i32 as size_t);
        if *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
            let mut np: *const i8 = 0 as *const i8;
            np = cp;
            while *np as i32 >= '0' as i32 && *np as i32 <= '9' as i32 {
                np = np.offset(1);
                np;
            }
            if *np as i32 == '$' as i32 {
                let mut n: size_t = 0 as i32 as size_t;
                np = cp;
                while *np as i32 >= '0' as i32 && *np as i32 <= '9' as i32 {
                    n = xsum(
                        if n
                            <= (18446744073709551615 as u64)
                                .wrapping_div(10 as i32 as u64)
                        {
                            n.wrapping_mul(10 as i32 as u64)
                        } else {
                            18446744073709551615 as u64
                        },
                        (*np as i32 - '0' as i32) as size_t,
                    );
                    np = np.offset(1);
                    np;
                }
                if n == 0 as i32 as u64 {
                    current_block = 8543771624226285275;
                    break;
                } else if n == 18446744073709551615 as u64 {
                    current_block = 8543771624226285275;
                    break;
                } else {
                    arg_index = n.wrapping_sub(1 as i32 as u64);
                    cp = np.offset(1 as i32 as isize);
                }
            }
        }
        loop {
            if *cp as i32 == '\'' as i32 {
                (*dp).flags |= 1 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == '-' as i32 {
                (*dp).flags |= 2 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == '+' as i32 {
                (*dp).flags |= 4 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == ' ' as i32 {
                (*dp).flags |= 8 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == '#' as i32 {
                (*dp).flags |= 16 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == '0' as i32 {
                (*dp).flags |= 32 as i32;
                cp = cp.offset(1);
                cp;
            } else {
                if !(*cp as i32 == 'I' as i32) {
                    break;
                }
                (*dp).flags |= 64 as i32;
                cp = cp.offset(1);
                cp;
            }
        }
        if *cp as i32 == '*' as i32 {
            (*dp).width_start = cp;
            cp = cp.offset(1);
            cp;
            (*dp).width_end = cp;
            if max_width_length < 1 as i32 as u64 {
                max_width_length = 1 as i32 as size_t;
            }
            if *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
                let mut np_0: *const i8 = 0 as *const i8;
                np_0 = cp;
                while *np_0 as i32 >= '0' as i32 && *np_0 as i32 <= '9' as i32 {
                    np_0 = np_0.offset(1);
                    np_0;
                }
                if *np_0 as i32 == '$' as i32 {
                    let mut n_0: size_t = 0 as i32 as size_t;
                    np_0 = cp;
                    while *np_0 as i32 >= '0' as i32 && *np_0 as i32 <= '9' as i32 {
                        n_0 = xsum(
                            if n_0
                                <= (18446744073709551615 as u64)
                                    .wrapping_div(10 as i32 as u64)
                            {
                                n_0.wrapping_mul(10 as i32 as u64)
                            } else {
                                18446744073709551615 as u64
                            },
                            (*np_0 as i32 - '0' as i32) as size_t,
                        );
                        np_0 = np_0.offset(1);
                        np_0;
                    }
                    if n_0 == 0 as i32 as u64 {
                        current_block = 8543771624226285275;
                        break;
                    } else if n_0 == 18446744073709551615 as u64 {
                        current_block = 8543771624226285275;
                        break;
                    } else {
                        (*dp).width_arg_index = n_0.wrapping_sub(1 as i32 as u64);
                        cp = np_0.offset(1 as i32 as isize);
                    }
                }
            }
            if (*dp).width_arg_index == !(0 as i32 as size_t) {
                let fresh1 = arg_posn;
                arg_posn = arg_posn.wrapping_add(1);
                (*dp).width_arg_index = fresh1;
                if (*dp).width_arg_index == !(0 as i32 as size_t) {
                    current_block = 8543771624226285275;
                    break;
                }
            }
            let mut n_1: size_t = (*dp).width_arg_index;
            if n_1 >= a_allocated {
                let mut memory_size: size_t = 0;
                let mut memory: *mut argument = 0 as *mut argument;
                a_allocated = if a_allocated
                    <= (18446744073709551615 as u64).wrapping_div(2 as i32 as u64)
                {
                    a_allocated.wrapping_mul(2 as i32 as u64)
                } else {
                    18446744073709551615 as u64
                };
                if a_allocated <= n_1 {
                    a_allocated = xsum(n_1, 1 as i32 as size_t);
                }
                memory_size = if a_allocated
                    <= (18446744073709551615 as u64)
                        .wrapping_div(::core::mem::size_of::<argument>() as u64)
                {
                    a_allocated.wrapping_mul(::core::mem::size_of::<argument>() as u64)
                } else {
                    18446744073709551615 as u64
                };
                if memory_size == 18446744073709551615 as u64 {
                    current_block = 10295444042842710584;
                    break;
                }
                memory = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                    realloc((*a).arg as *mut libc::c_void, memory_size)
                } else {
                    malloc(memory_size)
                }) as *mut argument;
                if memory.is_null() {
                    current_block = 10295444042842710584;
                    break;
                }
                if (*a).arg == ((*a).direct_alloc_arg).as_mut_ptr() {
                    memcpy(
                        memory as *mut libc::c_void,
                        (*a).arg as *const libc::c_void,
                        ((*a).count)
                            .wrapping_mul(::core::mem::size_of::<argument>() as u64),
                    );
                }
                (*a).arg = memory;
            }
            while (*a).count <= n_1 {
                let fresh2 = (*a).count;
                (*a).count = ((*a).count).wrapping_add(1);
                (*((*a).arg).offset(fresh2 as isize)).type_0 = arg_type::TYPE_NONE;
            }
            if (*((*a).arg).offset(n_1 as isize)).type_0 as u32
                == arg_type::TYPE_NONE as i32 as u32
            {
                (*((*a).arg).offset(n_1 as isize)).type_0 = arg_type::TYPE_INT;
            } else if (*((*a).arg).offset(n_1 as isize)).type_0 as u32
                != arg_type::TYPE_INT as i32 as u32
            {
                current_block = 8543771624226285275;
                break;
            }
        } else if *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
            let mut width_length: size_t = 0;
            (*dp).width_start = cp;
            while *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
                cp = cp.offset(1);
                cp;
            }
            (*dp).width_end = cp;
            width_length = ((*dp).width_end).offset_from((*dp).width_start) as i64
                as size_t;
            if max_width_length < width_length {
                max_width_length = width_length;
            }
        }
        if *cp as i32 == '.' as i32 {
            cp = cp.offset(1);
            cp;
            if *cp as i32 == '*' as i32 {
                (*dp).precision_start = cp.offset(-(1 as i32 as isize));
                cp = cp.offset(1);
                cp;
                (*dp).precision_end = cp;
                if max_precision_length < 2 as i32 as u64 {
                    max_precision_length = 2 as i32 as size_t;
                }
                if *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
                    let mut np_1: *const i8 = 0 as *const i8;
                    np_1 = cp;
                    while *np_1 as i32 >= '0' as i32 && *np_1 as i32 <= '9' as i32 {
                        np_1 = np_1.offset(1);
                        np_1;
                    }
                    if *np_1 as i32 == '$' as i32 {
                        let mut n_2: size_t = 0 as i32 as size_t;
                        np_1 = cp;
                        while *np_1 as i32 >= '0' as i32 && *np_1 as i32 <= '9' as i32 {
                            n_2 = xsum(
                                if n_2
                                    <= (18446744073709551615 as u64)
                                        .wrapping_div(10 as i32 as u64)
                                {
                                    n_2.wrapping_mul(10 as i32 as u64)
                                } else {
                                    18446744073709551615 as u64
                                },
                                (*np_1 as i32 - '0' as i32) as size_t,
                            );
                            np_1 = np_1.offset(1);
                            np_1;
                        }
                        if n_2 == 0 as i32 as u64 {
                            current_block = 8543771624226285275;
                            break;
                        } else if n_2 == 18446744073709551615 as u64 {
                            current_block = 8543771624226285275;
                            break;
                        } else {
                            (*dp).precision_arg_index = n_2
                                .wrapping_sub(1 as i32 as u64);
                            cp = np_1.offset(1 as i32 as isize);
                        }
                    }
                }
                if (*dp).precision_arg_index == !(0 as i32 as size_t) {
                    let fresh3 = arg_posn;
                    arg_posn = arg_posn.wrapping_add(1);
                    (*dp).precision_arg_index = fresh3;
                    if (*dp).precision_arg_index == !(0 as i32 as size_t) {
                        current_block = 8543771624226285275;
                        break;
                    }
                }
                let mut n_3: size_t = (*dp).precision_arg_index;
                if n_3 >= a_allocated {
                    let mut memory_size_0: size_t = 0;
                    let mut memory_0: *mut argument = 0 as *mut argument;
                    a_allocated = if a_allocated
                        <= (18446744073709551615 as u64).wrapping_div(2 as i32 as u64)
                    {
                        a_allocated.wrapping_mul(2 as i32 as u64)
                    } else {
                        18446744073709551615 as u64
                    };
                    if a_allocated <= n_3 {
                        a_allocated = xsum(n_3, 1 as i32 as size_t);
                    }
                    memory_size_0 = if a_allocated
                        <= (18446744073709551615 as u64)
                            .wrapping_div(::core::mem::size_of::<argument>() as u64)
                    {
                        a_allocated
                            .wrapping_mul(::core::mem::size_of::<argument>() as u64)
                    } else {
                        18446744073709551615 as u64
                    };
                    if memory_size_0 == 18446744073709551615 as u64 {
                        current_block = 10295444042842710584;
                        break;
                    }
                    memory_0 = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                        realloc((*a).arg as *mut libc::c_void, memory_size_0)
                    } else {
                        malloc(memory_size_0)
                    }) as *mut argument;
                    if memory_0.is_null() {
                        current_block = 10295444042842710584;
                        break;
                    }
                    if (*a).arg == ((*a).direct_alloc_arg).as_mut_ptr() {
                        memcpy(
                            memory_0 as *mut libc::c_void,
                            (*a).arg as *const libc::c_void,
                            ((*a).count)
                                .wrapping_mul(::core::mem::size_of::<argument>() as u64),
                        );
                    }
                    (*a).arg = memory_0;
                }
                while (*a).count <= n_3 {
                    let fresh4 = (*a).count;
                    (*a).count = ((*a).count).wrapping_add(1);
                    (*((*a).arg).offset(fresh4 as isize)).type_0 = arg_type::TYPE_NONE;
                }
                if (*((*a).arg).offset(n_3 as isize)).type_0 as u32
                    == arg_type::TYPE_NONE as i32 as u32
                {
                    (*((*a).arg).offset(n_3 as isize)).type_0 = arg_type::TYPE_INT;
                } else if (*((*a).arg).offset(n_3 as isize)).type_0 as u32
                    != arg_type::TYPE_INT as i32 as u32
                {
                    current_block = 8543771624226285275;
                    break;
                }
            } else {
                let mut precision_length: size_t = 0;
                (*dp).precision_start = cp.offset(-(1 as i32 as isize));
                while *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
                    cp = cp.offset(1);
                    cp;
                }
                (*dp).precision_end = cp;
                precision_length = ((*dp).precision_end)
                    .offset_from((*dp).precision_start) as i64 as size_t;
                if max_precision_length < precision_length {
                    max_precision_length = precision_length;
                }
            }
        }
        let mut type_0: arg_type = arg_type::TYPE_NONE;
        let mut signed_type: arg_type = arg_type::TYPE_INT;
        let mut unsigned_type: arg_type = arg_type::TYPE_UINT;
        let mut pointer_type: arg_type = arg_type::TYPE_COUNT_INT_POINTER;
        let mut floatingpoint_type: arg_type = arg_type::TYPE_DOUBLE;
        if *cp as i32 == 'h' as i32 {
            if *cp.offset(1 as i32 as isize) as i32 == 'h' as i32 {
                signed_type = arg_type::TYPE_SCHAR;
                unsigned_type = arg_type::TYPE_UCHAR;
                pointer_type = arg_type::TYPE_COUNT_SCHAR_POINTER;
                cp = cp.offset(2 as i32 as isize);
            } else {
                signed_type = arg_type::TYPE_SHORT;
                unsigned_type = arg_type::TYPE_USHORT;
                pointer_type = arg_type::TYPE_COUNT_SHORT_POINTER;
                cp = cp.offset(1);
                cp;
            }
        } else if *cp as i32 == 'l' as i32 {
            if *cp.offset(1 as i32 as isize) as i32 == 'l' as i32 {
                signed_type = arg_type::TYPE_LONGLONGINT;
                unsigned_type = arg_type::TYPE_ULONGLONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGLONGINT_POINTER;
                floatingpoint_type = arg_type::TYPE_LONGDOUBLE;
                cp = cp.offset(2 as i32 as isize);
            } else {
                signed_type = arg_type::TYPE_LONGINT;
                unsigned_type = arg_type::TYPE_ULONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGINT_POINTER;
                cp = cp.offset(1);
                cp;
            }
        } else if *cp as i32 == 'j' as i32 {
            if ::core::mem::size_of::<intmax_t>() as u64
                > ::core::mem::size_of::<i64>() as u64
            {
                signed_type = arg_type::TYPE_LONGLONGINT;
                unsigned_type = arg_type::TYPE_ULONGLONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGLONGINT_POINTER;
                floatingpoint_type = arg_type::TYPE_LONGDOUBLE;
            } else if ::core::mem::size_of::<intmax_t>() as u64
                > ::core::mem::size_of::<i32>() as u64
            {
                signed_type = arg_type::TYPE_LONGINT;
                unsigned_type = arg_type::TYPE_ULONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGINT_POINTER;
            }
            cp = cp.offset(1);
            cp;
        } else if *cp as i32 == 'z' as i32 || *cp as i32 == 'Z' as i32 {
            if ::core::mem::size_of::<size_t>() as u64
                > ::core::mem::size_of::<i64>() as u64
            {
                signed_type = arg_type::TYPE_LONGLONGINT;
                unsigned_type = arg_type::TYPE_ULONGLONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGLONGINT_POINTER;
                floatingpoint_type = arg_type::TYPE_LONGDOUBLE;
            } else if ::core::mem::size_of::<size_t>() as u64
                > ::core::mem::size_of::<i32>() as u64
            {
                signed_type = arg_type::TYPE_LONGINT;
                unsigned_type = arg_type::TYPE_ULONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGINT_POINTER;
            }
            cp = cp.offset(1);
            cp;
        } else if *cp as i32 == 't' as i32 {
            if ::core::mem::size_of::<ptrdiff_t>() as u64
                > ::core::mem::size_of::<i64>() as u64
            {
                signed_type = arg_type::TYPE_LONGLONGINT;
                unsigned_type = arg_type::TYPE_ULONGLONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGLONGINT_POINTER;
                floatingpoint_type = arg_type::TYPE_LONGDOUBLE;
            } else if ::core::mem::size_of::<ptrdiff_t>() as u64
                > ::core::mem::size_of::<i32>() as u64
            {
                signed_type = arg_type::TYPE_LONGINT;
                unsigned_type = arg_type::TYPE_ULONGINT;
                pointer_type = arg_type::TYPE_COUNT_LONGINT_POINTER;
            }
            cp = cp.offset(1);
            cp;
        } else if *cp as i32 == 'w' as i32 {
            if *cp.offset(1 as i32 as isize) as i32 == 'f' as i32 {
                if *cp.offset(2 as i32 as isize) as i32 == '8' as i32 {
                    signed_type = arg_type::TYPE_INT_FAST8_T;
                    unsigned_type = arg_type::TYPE_UINT_FAST8_T;
                    pointer_type = arg_type::TYPE_COUNT_INT_FAST8_T_POINTER;
                    cp = cp.offset(3 as i32 as isize);
                } else if *cp.offset(2 as i32 as isize) as i32 == '1' as i32
                    && *cp.offset(3 as i32 as isize) as i32 == '6' as i32
                {
                    signed_type = arg_type::TYPE_INT_FAST16_T;
                    unsigned_type = arg_type::TYPE_UINT_FAST16_T;
                    pointer_type = arg_type::TYPE_COUNT_INT_FAST16_T_POINTER;
                    cp = cp.offset(4 as i32 as isize);
                } else if *cp.offset(2 as i32 as isize) as i32 == '3' as i32
                    && *cp.offset(3 as i32 as isize) as i32 == '2' as i32
                {
                    signed_type = arg_type::TYPE_INT_FAST32_T;
                    unsigned_type = arg_type::TYPE_UINT_FAST32_T;
                    pointer_type = arg_type::TYPE_COUNT_INT_FAST32_T_POINTER;
                    cp = cp.offset(4 as i32 as isize);
                } else if *cp.offset(2 as i32 as isize) as i32 == '6' as i32
                    && *cp.offset(3 as i32 as isize) as i32 == '4' as i32
                {
                    signed_type = arg_type::TYPE_INT_FAST64_T;
                    unsigned_type = arg_type::TYPE_UINT_FAST64_T;
                    pointer_type = arg_type::TYPE_COUNT_INT_FAST64_T_POINTER;
                    cp = cp.offset(4 as i32 as isize);
                }
            } else if *cp.offset(1 as i32 as isize) as i32 == '8' as i32 {
                signed_type = arg_type::TYPE_INT8_T;
                unsigned_type = arg_type::TYPE_UINT8_T;
                pointer_type = arg_type::TYPE_COUNT_INT8_T_POINTER;
                cp = cp.offset(2 as i32 as isize);
            } else if *cp.offset(1 as i32 as isize) as i32 == '1' as i32
                && *cp.offset(2 as i32 as isize) as i32 == '6' as i32
            {
                signed_type = arg_type::TYPE_INT16_T;
                unsigned_type = arg_type::TYPE_UINT16_T;
                pointer_type = arg_type::TYPE_COUNT_INT16_T_POINTER;
                cp = cp.offset(3 as i32 as isize);
            } else if *cp.offset(1 as i32 as isize) as i32 == '3' as i32
                && *cp.offset(2 as i32 as isize) as i32 == '2' as i32
            {
                signed_type = arg_type::TYPE_INT32_T;
                unsigned_type = arg_type::TYPE_UINT32_T;
                pointer_type = arg_type::TYPE_COUNT_INT32_T_POINTER;
                cp = cp.offset(3 as i32 as isize);
            } else if *cp.offset(1 as i32 as isize) as i32 == '6' as i32
                && *cp.offset(2 as i32 as isize) as i32 == '4' as i32
            {
                signed_type = arg_type::TYPE_INT64_T;
                unsigned_type = arg_type::TYPE_UINT64_T;
                pointer_type = arg_type::TYPE_COUNT_INT64_T_POINTER;
                cp = cp.offset(3 as i32 as isize);
            }
        } else if *cp as i32 == 'L' as i32 {
            signed_type = arg_type::TYPE_LONGLONGINT;
            unsigned_type = arg_type::TYPE_ULONGLONGINT;
            pointer_type = arg_type::TYPE_COUNT_LONGLONGINT_POINTER;
            floatingpoint_type = arg_type::TYPE_LONGDOUBLE;
            cp = cp.offset(1);
            cp;
        }
        let fresh5 = cp;
        cp = cp.offset(1);
        c = *fresh5;
        match c as i32 {
            100 | 105 => {
                type_0 = signed_type;
            }
            98 | 111 | 117 | 120 | 88 => {
                type_0 = unsigned_type;
            }
            102 | 70 | 101 | 69 | 103 | 71 | 97 | 65 => {
                type_0 = floatingpoint_type;
            }
            99 => {
                if signed_type as u32 == arg_type::TYPE_LONGINT as i32 as u32
                    || signed_type as u32 == arg_type::TYPE_LONGLONGINT as i32 as u32
                {
                    type_0 = arg_type::TYPE_WIDE_CHAR;
                } else {
                    type_0 = arg_type::TYPE_CHAR;
                }
            }
            67 => {
                type_0 = arg_type::TYPE_WIDE_CHAR;
                c = 'c' as i32 as i8;
            }
            115 => {
                if signed_type as u32 == arg_type::TYPE_LONGINT as i32 as u32
                    || signed_type as u32 == arg_type::TYPE_LONGLONGINT as i32 as u32
                {
                    type_0 = arg_type::TYPE_WIDE_STRING;
                } else {
                    type_0 = arg_type::TYPE_STRING;
                }
            }
            83 => {
                type_0 = arg_type::TYPE_WIDE_STRING;
                c = 's' as i32 as i8;
            }
            112 => {
                type_0 = arg_type::TYPE_POINTER;
            }
            110 => {
                type_0 = pointer_type;
            }
            37 => {
                type_0 = arg_type::TYPE_NONE;
            }
            _ => {
                current_block = 8543771624226285275;
                break;
            }
        }
        if type_0 as u32 != arg_type::TYPE_NONE as i32 as u32 {
            (*dp).arg_index = arg_index;
            if (*dp).arg_index == !(0 as i32 as size_t) {
                let fresh6 = arg_posn;
                arg_posn = arg_posn.wrapping_add(1);
                (*dp).arg_index = fresh6;
                if (*dp).arg_index == !(0 as i32 as size_t) {
                    current_block = 8543771624226285275;
                    break;
                }
            }
            let mut n_4: size_t = (*dp).arg_index;
            if n_4 >= a_allocated {
                let mut memory_size_1: size_t = 0;
                let mut memory_1: *mut argument = 0 as *mut argument;
                a_allocated = if a_allocated
                    <= (18446744073709551615 as u64).wrapping_div(2 as i32 as u64)
                {
                    a_allocated.wrapping_mul(2 as i32 as u64)
                } else {
                    18446744073709551615 as u64
                };
                if a_allocated <= n_4 {
                    a_allocated = xsum(n_4, 1 as i32 as size_t);
                }
                memory_size_1 = if a_allocated
                    <= (18446744073709551615 as u64)
                        .wrapping_div(::core::mem::size_of::<argument>() as u64)
                {
                    a_allocated.wrapping_mul(::core::mem::size_of::<argument>() as u64)
                } else {
                    18446744073709551615 as u64
                };
                if memory_size_1 == 18446744073709551615 as u64 {
                    current_block = 10295444042842710584;
                    break;
                }
                memory_1 = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                    realloc((*a).arg as *mut libc::c_void, memory_size_1)
                } else {
                    malloc(memory_size_1)
                }) as *mut argument;
                if memory_1.is_null() {
                    current_block = 10295444042842710584;
                    break;
                }
                if (*a).arg == ((*a).direct_alloc_arg).as_mut_ptr() {
                    memcpy(
                        memory_1 as *mut libc::c_void,
                        (*a).arg as *const libc::c_void,
                        ((*a).count)
                            .wrapping_mul(::core::mem::size_of::<argument>() as u64),
                    );
                }
                (*a).arg = memory_1;
            }
            while (*a).count <= n_4 {
                let fresh7 = (*a).count;
                (*a).count = ((*a).count).wrapping_add(1);
                (*((*a).arg).offset(fresh7 as isize)).type_0 = arg_type::TYPE_NONE;
            }
            if (*((*a).arg).offset(n_4 as isize)).type_0 as u32
                == arg_type::TYPE_NONE as i32 as u32
            {
                (*((*a).arg).offset(n_4 as isize)).type_0 = type_0;
            } else if (*((*a).arg).offset(n_4 as isize)).type_0 as u32 != type_0 as u32 {
                current_block = 8543771624226285275;
                break;
            }
        }
        (*dp).conversion = c;
        (*dp).dir_end = cp;
        (*d).count = ((*d).count).wrapping_add(1);
        (*d).count;
        if !((*d).count >= d_allocated) {
            continue;
        }
        let mut memory_size_2: size_t = 0;
        let mut memory_2: *mut char_directive = 0 as *mut char_directive;
        d_allocated = if d_allocated
            <= (18446744073709551615 as u64).wrapping_div(2 as i32 as u64)
        {
            d_allocated.wrapping_mul(2 as i32 as u64)
        } else {
            18446744073709551615 as u64
        };
        memory_size_2 = if d_allocated
            <= (18446744073709551615 as u64)
                .wrapping_div(::core::mem::size_of::<char_directive>() as u64)
        {
            d_allocated.wrapping_mul(::core::mem::size_of::<char_directive>() as u64)
        } else {
            18446744073709551615 as u64
        };
        if memory_size_2 == 18446744073709551615 as u64 {
            current_block = 10295444042842710584;
            break;
        } else {
            memory_2 = (if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                realloc((*d).dir as *mut libc::c_void, memory_size_2)
            } else {
                malloc(memory_size_2)
            }) as *mut char_directive;
            if memory_2.is_null() {
                current_block = 10295444042842710584;
                break;
            }
            if (*d).dir == ((*d).direct_alloc_dir).as_mut_ptr() {
                memcpy(
                    memory_2 as *mut libc::c_void,
                    (*d).dir as *const libc::c_void,
                    ((*d).count)
                        .wrapping_mul(::core::mem::size_of::<char_directive>() as u64),
                );
            }
            (*d).dir = memory_2;
        }
    }
    match current_block {
        10295444042842710584 => {
            if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                rpl_free((*a).arg as *mut libc::c_void);
            }
            if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                rpl_free((*d).dir as *mut libc::c_void);
            }
            *__errno_location() = 12 as i32;
            return -(1 as i32);
        }
        8543771624226285275 => {
            if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                rpl_free((*a).arg as *mut libc::c_void);
            }
            if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                rpl_free((*d).dir as *mut libc::c_void);
            }
            *__errno_location() = 22 as i32;
            return -(1 as i32);
        }
        _ => {
            let ref mut fresh8 = (*((*d).dir).offset((*d).count as isize)).dir_start;
            *fresh8 = cp;
            (*d).max_width_length = max_width_length;
            (*d).max_precision_length = max_precision_length;
            return 0 as i32;
        }
    };
}
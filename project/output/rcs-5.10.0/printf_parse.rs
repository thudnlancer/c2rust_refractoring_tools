use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
}
pub type ptrdiff_t = i64;
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
pub type intmax_t = __intmax_t;
pub type __intmax_t = i64;
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
            current_block = 7942882540430375978;
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
                    current_block = 4042378345336412398;
                    break;
                } else if n == 18446744073709551615 as u64 {
                    current_block = 4042378345336412398;
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
                        current_block = 4042378345336412398;
                        break;
                    } else if n_0 == 18446744073709551615 as u64 {
                        current_block = 4042378345336412398;
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
                    current_block = 4042378345336412398;
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
                    current_block = 1188562353059013519;
                    break;
                }
                memory = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                    realloc((*a).arg as *mut libc::c_void, memory_size)
                } else {
                    malloc(memory_size)
                }) as *mut argument;
                if memory.is_null() {
                    current_block = 1188562353059013519;
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
                current_block = 4042378345336412398;
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
                            current_block = 4042378345336412398;
                            break;
                        } else if n_2 == 18446744073709551615 as u64 {
                            current_block = 4042378345336412398;
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
                        current_block = 4042378345336412398;
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
                        current_block = 1188562353059013519;
                        break;
                    }
                    memory_0 = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                        realloc((*a).arg as *mut libc::c_void, memory_size_0)
                    } else {
                        malloc(memory_size_0)
                    }) as *mut argument;
                    if memory_0.is_null() {
                        current_block = 1188562353059013519;
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
                    current_block = 4042378345336412398;
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
        let mut flags: i32 = 0 as i32;
        loop {
            if *cp as i32 == 'h' as i32 {
                flags |= (1 as i32) << (flags & 1 as i32);
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == 'L' as i32 {
                flags |= 4 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == 'l' as i32 {
                flags += 8 as i32;
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == 'j' as i32 {
                if ::core::mem::size_of::<intmax_t>() as u64
                    > ::core::mem::size_of::<i64>() as u64
                {
                    flags += 16 as i32;
                } else if ::core::mem::size_of::<intmax_t>() as u64
                    > ::core::mem::size_of::<i32>() as u64
                {
                    flags += 8 as i32;
                }
                cp = cp.offset(1);
                cp;
            } else if *cp as i32 == 'z' as i32 || *cp as i32 == 'Z' as i32 {
                if ::core::mem::size_of::<size_t>() as u64
                    > ::core::mem::size_of::<i64>() as u64
                {
                    flags += 16 as i32;
                } else if ::core::mem::size_of::<size_t>() as u64
                    > ::core::mem::size_of::<i32>() as u64
                {
                    flags += 8 as i32;
                }
                cp = cp.offset(1);
                cp;
            } else {
                if !(*cp as i32 == 't' as i32) {
                    break;
                }
                if ::core::mem::size_of::<ptrdiff_t>() as u64
                    > ::core::mem::size_of::<i64>() as u64
                {
                    flags += 16 as i32;
                } else if ::core::mem::size_of::<ptrdiff_t>() as u64
                    > ::core::mem::size_of::<i32>() as u64
                {
                    flags += 8 as i32;
                }
                cp = cp.offset(1);
                cp;
            }
        }
        let fresh5 = cp;
        cp = cp.offset(1);
        c = *fresh5;
        match c as i32 {
            100 | 105 => {
                if flags >= 16 as i32 || flags & 4 as i32 != 0 {
                    type_0 = arg_type::TYPE_LONGLONGINT;
                } else if flags >= 8 as i32 {
                    type_0 = arg_type::TYPE_LONGINT;
                } else if flags & 2 as i32 != 0 {
                    type_0 = arg_type::TYPE_SCHAR;
                } else if flags & 1 as i32 != 0 {
                    type_0 = arg_type::TYPE_SHORT;
                } else {
                    type_0 = arg_type::TYPE_INT;
                }
            }
            111 | 117 | 120 | 88 => {
                if flags >= 16 as i32 || flags & 4 as i32 != 0 {
                    type_0 = arg_type::TYPE_ULONGLONGINT;
                } else if flags >= 8 as i32 {
                    type_0 = arg_type::TYPE_ULONGINT;
                } else if flags & 2 as i32 != 0 {
                    type_0 = arg_type::TYPE_UCHAR;
                } else if flags & 1 as i32 != 0 {
                    type_0 = arg_type::TYPE_USHORT;
                } else {
                    type_0 = arg_type::TYPE_UINT;
                }
            }
            102 | 70 | 101 | 69 | 103 | 71 | 97 | 65 => {
                if flags >= 16 as i32 || flags & 4 as i32 != 0 {
                    type_0 = arg_type::TYPE_LONGDOUBLE;
                } else {
                    type_0 = arg_type::TYPE_DOUBLE;
                }
            }
            99 => {
                if flags >= 8 as i32 {
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
                if flags >= 8 as i32 {
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
                if flags >= 16 as i32 || flags & 4 as i32 != 0 {
                    type_0 = arg_type::TYPE_COUNT_LONGLONGINT_POINTER;
                } else if flags >= 8 as i32 {
                    type_0 = arg_type::TYPE_COUNT_LONGINT_POINTER;
                } else if flags & 2 as i32 != 0 {
                    type_0 = arg_type::TYPE_COUNT_SCHAR_POINTER;
                } else if flags & 1 as i32 != 0 {
                    type_0 = arg_type::TYPE_COUNT_SHORT_POINTER;
                } else {
                    type_0 = arg_type::TYPE_COUNT_INT_POINTER;
                }
            }
            37 => {
                type_0 = arg_type::TYPE_NONE;
            }
            _ => {
                current_block = 4042378345336412398;
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
                    current_block = 4042378345336412398;
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
                    current_block = 1188562353059013519;
                    break;
                }
                memory_1 = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                    realloc((*a).arg as *mut libc::c_void, memory_size_1)
                } else {
                    malloc(memory_size_1)
                }) as *mut argument;
                if memory_1.is_null() {
                    current_block = 1188562353059013519;
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
                current_block = 4042378345336412398;
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
            current_block = 1188562353059013519;
            break;
        } else {
            memory_2 = (if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                realloc((*d).dir as *mut libc::c_void, memory_size_2)
            } else {
                malloc(memory_size_2)
            }) as *mut char_directive;
            if memory_2.is_null() {
                current_block = 1188562353059013519;
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
        1188562353059013519 => {
            if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                free((*a).arg as *mut libc::c_void);
            }
            if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                free((*d).dir as *mut libc::c_void);
            }
            *__errno_location() = 12 as i32;
            return -(1 as i32);
        }
        4042378345336412398 => {
            if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                free((*a).arg as *mut libc::c_void);
            }
            if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                free((*d).dir as *mut libc::c_void);
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
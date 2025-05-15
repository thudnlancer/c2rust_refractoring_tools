use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
    fn printf_fetchargs(args: ::core::ffi::VaList, a: *mut arguments) -> i32;
    fn printf_parse(
        format: *const i8,
        d: *mut char_directives,
        a: *mut arguments,
    ) -> i32;
}
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
pub type wint_t = u32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum arg_type {
    TYPE_NONE,
    TYPE_SCHAR,
    TYPE_UCHAR,
    TYPE_SHORT,
    TYPE_USHORT,
    TYPE_INT,
    TYPE_UINT,
    TYPE_LONGINT,
    TYPE_ULONGINT,
    TYPE_LONGLONGINT,
    TYPE_ULONGLONGINT,
    TYPE_DOUBLE,
    TYPE_LONGDOUBLE,
    TYPE_CHAR,
    TYPE_WIDE_CHAR,
    TYPE_STRING,
    TYPE_WIDE_STRING,
    TYPE_POINTER,
    TYPE_COUNT_SCHAR_POINTER,
    TYPE_COUNT_SHORT_POINTER,
    TYPE_COUNT_INT_POINTER,
    TYPE_COUNT_LONGINT_POINTER,
    TYPE_COUNT_LONGLONGINT_POINTER,
}
impl arg_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            arg_type::TYPE_NONE => 0,
            arg_type::TYPE_SCHAR => 1,
            arg_type::TYPE_UCHAR => 2,
            arg_type::TYPE_SHORT => 3,
            arg_type::TYPE_USHORT => 4,
            arg_type::TYPE_INT => 5,
            arg_type::TYPE_UINT => 6,
            arg_type::TYPE_LONGINT => 7,
            arg_type::TYPE_ULONGINT => 8,
            arg_type::TYPE_LONGLONGINT => 9,
            arg_type::TYPE_ULONGLONGINT => 10,
            arg_type::TYPE_DOUBLE => 11,
            arg_type::TYPE_LONGDOUBLE => 12,
            arg_type::TYPE_CHAR => 13,
            arg_type::TYPE_WIDE_CHAR => 14,
            arg_type::TYPE_STRING => 15,
            arg_type::TYPE_WIDE_STRING => 16,
            arg_type::TYPE_POINTER => 17,
            arg_type::TYPE_COUNT_SCHAR_POINTER => 18,
            arg_type::TYPE_COUNT_SHORT_POINTER => 19,
            arg_type::TYPE_COUNT_INT_POINTER => 20,
            arg_type::TYPE_COUNT_LONGINT_POINTER => 21,
            arg_type::TYPE_COUNT_LONGLONGINT_POINTER => 22,
        }
    }
    fn from_libc_c_uint(value: u32) -> arg_type {
        match value {
            0 => arg_type::TYPE_NONE,
            1 => arg_type::TYPE_SCHAR,
            2 => arg_type::TYPE_UCHAR,
            3 => arg_type::TYPE_SHORT,
            4 => arg_type::TYPE_USHORT,
            5 => arg_type::TYPE_INT,
            6 => arg_type::TYPE_UINT,
            7 => arg_type::TYPE_LONGINT,
            8 => arg_type::TYPE_ULONGINT,
            9 => arg_type::TYPE_LONGLONGINT,
            10 => arg_type::TYPE_ULONGLONGINT,
            11 => arg_type::TYPE_DOUBLE,
            12 => arg_type::TYPE_LONGDOUBLE,
            13 => arg_type::TYPE_CHAR,
            14 => arg_type::TYPE_WIDE_CHAR,
            15 => arg_type::TYPE_STRING,
            16 => arg_type::TYPE_WIDE_STRING,
            17 => arg_type::TYPE_POINTER,
            18 => arg_type::TYPE_COUNT_SCHAR_POINTER,
            19 => arg_type::TYPE_COUNT_SHORT_POINTER,
            20 => arg_type::TYPE_COUNT_INT_POINTER,
            21 => arg_type::TYPE_COUNT_LONGINT_POINTER,
            22 => arg_type::TYPE_COUNT_LONGLONGINT_POINTER,
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
unsafe extern "C" fn xmax(mut size1: size_t, mut size2: size_t) -> size_t {
    return if size1 >= size2 { size1 } else { size2 };
}
#[inline]
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as u64 };
}
#[inline]
unsafe extern "C" fn xsum4(
    mut size1: size_t,
    mut size2: size_t,
    mut size3: size_t,
    mut size4: size_t,
) -> size_t {
    return xsum(xsum(xsum(size1, size2), size3), size4);
}
#[no_mangle]
pub unsafe extern "C" fn vasnprintf(
    mut resultbuf: *mut i8,
    mut lengthp: *mut size_t,
    mut format: *const i8,
    mut args: ::core::ffi::VaList,
) -> *mut i8 {
    let mut current_block: u64;
    let mut d: char_directives = char_directives {
        count: 0,
        dir: 0 as *mut char_directive,
        max_width_length: 0,
        max_precision_length: 0,
        direct_alloc_dir: [char_directive {
            dir_start: 0 as *const i8,
            dir_end: 0 as *const i8,
            flags: 0,
            width_start: 0 as *const i8,
            width_end: 0 as *const i8,
            width_arg_index: 0,
            precision_start: 0 as *const i8,
            precision_end: 0 as *const i8,
            precision_arg_index: 0,
            conversion: 0,
            arg_index: 0,
        }; 7],
    };
    let mut a: arguments = arguments {
        count: 0,
        arg: 0 as *mut argument,
        direct_alloc_arg: [argument {
            type_0: arg_type::TYPE_NONE,
            a: C2RustUnnamed { a_schar: 0 },
        }; 7],
    };
    if printf_parse(format, &mut d, &mut a) < 0 as i32 {
        return 0 as *mut i8;
    }
    if printf_fetchargs(args.as_va_list(), &mut a) < 0 as i32 {
        if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
            rpl_free(d.dir as *mut libc::c_void);
        }
        if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
            rpl_free(a.arg as *mut libc::c_void);
        }
        *__errno_location() = 22 as i32;
        return 0 as *mut i8;
    }
    let mut buf_neededlength: size_t = 0;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buf_malloced: *mut i8 = 0 as *mut i8;
    let mut cp: *const i8 = 0 as *const i8;
    let mut i: size_t = 0;
    let mut dp: *mut char_directive = 0 as *mut char_directive;
    let mut result: *mut i8 = 0 as *mut i8;
    let mut allocated: size_t = 0;
    let mut length: size_t = 0;
    buf_neededlength = xsum4(
        7 as i32 as size_t,
        d.max_width_length,
        d.max_precision_length,
        6 as i32 as size_t,
    );
    if buf_neededlength
        < (4000 as i32 as u64).wrapping_div(::core::mem::size_of::<i8>() as u64)
    {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            buf_neededlength.wrapping_mul(::core::mem::size_of::<i8>() as u64) as usize,
        );
        buf = fresh0.as_mut_ptr() as *mut i8;
        buf_malloced = 0 as *mut i8;
        current_block = 13797916685926291137;
    } else {
        let mut buf_memsize: size_t = if buf_neededlength
            <= (18446744073709551615 as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
        {
            buf_neededlength.wrapping_mul(::core::mem::size_of::<i8>() as u64)
        } else {
            18446744073709551615 as u64
        };
        if buf_memsize == 18446744073709551615 as u64 {
            current_block = 11849915316084770193;
        } else {
            buf = malloc(buf_memsize) as *mut i8;
            if buf.is_null() {
                current_block = 11849915316084770193;
            } else {
                buf_malloced = buf;
                current_block = 13797916685926291137;
            }
        }
    }
    match current_block {
        13797916685926291137 => {
            if !resultbuf.is_null() {
                result = resultbuf;
                allocated = *lengthp;
            } else {
                result = 0 as *mut i8;
                allocated = 0 as i32 as size_t;
            }
            length = 0 as i32 as size_t;
            cp = format;
            i = 0 as i32 as size_t;
            dp = &mut *(d.dir).offset(0 as i32 as isize) as *mut char_directive;
            's_139: loop {
                if cp != (*dp).dir_start {
                    let mut n: size_t = ((*dp).dir_start).offset_from(cp) as i64
                        as size_t;
                    let mut augmented_length: size_t = xsum(length, n);
                    if augmented_length > allocated {
                        let mut memory_size: size_t = 0;
                        let mut memory: *mut i8 = 0 as *mut i8;
                        allocated = if allocated > 0 as i32 as u64 {
                            if allocated
                                <= (18446744073709551615 as u64)
                                    .wrapping_div(2 as i32 as u64)
                            {
                                allocated.wrapping_mul(2 as i32 as u64)
                            } else {
                                18446744073709551615 as u64
                            }
                        } else {
                            12 as i32 as u64
                        };
                        if augmented_length > allocated {
                            allocated = augmented_length;
                        }
                        memory_size = if allocated
                            <= (18446744073709551615 as u64)
                                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        {
                            allocated.wrapping_mul(::core::mem::size_of::<i8>() as u64)
                        } else {
                            18446744073709551615 as u64
                        };
                        if memory_size == 18446744073709551615 as u64 {
                            current_block = 9265786686875583127;
                            break;
                        }
                        if result == resultbuf || result.is_null() {
                            memory = malloc(memory_size) as *mut i8;
                        } else {
                            memory = realloc(result as *mut libc::c_void, memory_size)
                                as *mut i8;
                        }
                        if memory.is_null() {
                            current_block = 9265786686875583127;
                            break;
                        }
                        if result == resultbuf && length > 0 as i32 as u64 {
                            memcpy(
                                memory as *mut libc::c_void,
                                result as *const libc::c_void,
                                length,
                            );
                        }
                        result = memory;
                    }
                    if ::core::mem::size_of::<i8>() as u64
                        == ::core::mem::size_of::<i8>() as u64
                    {
                        memcpy(
                            result.offset(length as isize) as *mut libc::c_void,
                            cp as *const libc::c_void,
                            n,
                        );
                        length = augmented_length;
                    } else {
                        loop {
                            let fresh1 = cp;
                            cp = cp.offset(1);
                            let fresh2 = length;
                            length = length.wrapping_add(1);
                            *result.offset(fresh2 as isize) = *fresh1;
                            n = n.wrapping_sub(1);
                            if !(n > 0 as i32 as u64) {
                                break;
                            }
                        }
                    }
                }
                if i == d.count {
                    current_block = 3105969743042996984;
                    break;
                }
                if (*dp).conversion as i32 == '%' as i32 {
                    let mut augmented_length_0: size_t = 0;
                    if !((*dp).arg_index == !(0 as i32 as size_t)) {
                        abort();
                    }
                    augmented_length_0 = xsum(length, 1 as i32 as size_t);
                    if augmented_length_0 > allocated {
                        let mut memory_size_0: size_t = 0;
                        let mut memory_0: *mut i8 = 0 as *mut i8;
                        allocated = if allocated > 0 as i32 as u64 {
                            if allocated
                                <= (18446744073709551615 as u64)
                                    .wrapping_div(2 as i32 as u64)
                            {
                                allocated.wrapping_mul(2 as i32 as u64)
                            } else {
                                18446744073709551615 as u64
                            }
                        } else {
                            12 as i32 as u64
                        };
                        if augmented_length_0 > allocated {
                            allocated = augmented_length_0;
                        }
                        memory_size_0 = if allocated
                            <= (18446744073709551615 as u64)
                                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        {
                            allocated.wrapping_mul(::core::mem::size_of::<i8>() as u64)
                        } else {
                            18446744073709551615 as u64
                        };
                        if memory_size_0 == 18446744073709551615 as u64 {
                            current_block = 9265786686875583127;
                            break;
                        }
                        if result == resultbuf || result.is_null() {
                            memory_0 = malloc(memory_size_0) as *mut i8;
                        } else {
                            memory_0 = realloc(
                                result as *mut libc::c_void,
                                memory_size_0,
                            ) as *mut i8;
                        }
                        if memory_0.is_null() {
                            current_block = 9265786686875583127;
                            break;
                        }
                        if result == resultbuf && length > 0 as i32 as u64 {
                            memcpy(
                                memory_0 as *mut libc::c_void,
                                result as *const libc::c_void,
                                length,
                            );
                        }
                        result = memory_0;
                    }
                    *result.offset(length as isize) = '%' as i32 as i8;
                    length = augmented_length_0;
                } else {
                    if !((*dp).arg_index != !(0 as i32 as size_t)) {
                        abort();
                    }
                    if (*dp).conversion as i32 == 'n' as i32 {
                        match (*(a.arg).offset((*dp).arg_index as isize)).type_0 as u32 {
                            18 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_schar_pointer = length as libc::c_schar;
                            }
                            19 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_short_pointer = length as libc::c_short;
                            }
                            20 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_int_pointer = length as i32;
                            }
                            21 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_longint_pointer = length as i64;
                            }
                            22 => {
                                *(*(a.arg).offset((*dp).arg_index as isize))
                                    .a
                                    .a_count_longlongint_pointer = length as libc::c_longlong;
                            }
                            _ => {
                                abort();
                            }
                        }
                    } else {
                        let mut type_0: arg_type = (*(a.arg)
                            .offset((*dp).arg_index as isize))
                            .type_0;
                        let mut flags: i32 = (*dp).flags;
                        let mut fbp: *mut i8 = 0 as *mut i8;
                        let mut prefix_count: u32 = 0;
                        let mut prefixes: [i32; 2] = [0; 2];
                        let mut orig_errno: i32 = 0;
                        fbp = buf;
                        let fresh3 = fbp;
                        fbp = fbp.offset(1);
                        *fresh3 = '%' as i32 as i8;
                        if flags & 1 as i32 != 0 {
                            let fresh4 = fbp;
                            fbp = fbp.offset(1);
                            *fresh4 = '\'' as i32 as i8;
                        }
                        if flags & 2 as i32 != 0 {
                            let fresh5 = fbp;
                            fbp = fbp.offset(1);
                            *fresh5 = '-' as i32 as i8;
                        }
                        if flags & 4 as i32 != 0 {
                            let fresh6 = fbp;
                            fbp = fbp.offset(1);
                            *fresh6 = '+' as i32 as i8;
                        }
                        if flags & 8 as i32 != 0 {
                            let fresh7 = fbp;
                            fbp = fbp.offset(1);
                            *fresh7 = ' ' as i32 as i8;
                        }
                        if flags & 16 as i32 != 0 {
                            let fresh8 = fbp;
                            fbp = fbp.offset(1);
                            *fresh8 = '#' as i32 as i8;
                        }
                        if flags & 64 as i32 != 0 {
                            let fresh9 = fbp;
                            fbp = fbp.offset(1);
                            *fresh9 = 'I' as i32 as i8;
                        }
                        if 0 as i32 == 0 {
                            if flags & 32 as i32 != 0 {
                                let fresh10 = fbp;
                                fbp = fbp.offset(1);
                                *fresh10 = '0' as i32 as i8;
                            }
                            if (*dp).width_start != (*dp).width_end {
                                let mut n_0: size_t = ((*dp).width_end)
                                    .offset_from((*dp).width_start) as i64 as size_t;
                                if ::core::mem::size_of::<i8>() as u64
                                    == ::core::mem::size_of::<i8>() as u64
                                {
                                    memcpy(
                                        fbp as *mut libc::c_void,
                                        (*dp).width_start as *const libc::c_void,
                                        n_0.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                    );
                                    fbp = fbp.offset(n_0 as isize);
                                } else {
                                    let mut mp: *const i8 = (*dp).width_start;
                                    loop {
                                        let fresh11 = mp;
                                        mp = mp.offset(1);
                                        let fresh12 = fbp;
                                        fbp = fbp.offset(1);
                                        *fresh12 = *fresh11;
                                        n_0 = n_0.wrapping_sub(1);
                                        if !(n_0 > 0 as i32 as u64) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if 0 as i32 == 0 {
                            if (*dp).precision_start != (*dp).precision_end {
                                let mut n_1: size_t = ((*dp).precision_end)
                                    .offset_from((*dp).precision_start) as i64 as size_t;
                                if ::core::mem::size_of::<i8>() as u64
                                    == ::core::mem::size_of::<i8>() as u64
                                {
                                    memcpy(
                                        fbp as *mut libc::c_void,
                                        (*dp).precision_start as *const libc::c_void,
                                        n_1.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                    );
                                    fbp = fbp.offset(n_1 as isize);
                                } else {
                                    let mut mp_0: *const i8 = (*dp).precision_start;
                                    loop {
                                        let fresh13 = mp_0;
                                        mp_0 = mp_0.offset(1);
                                        let fresh14 = fbp;
                                        fbp = fbp.offset(1);
                                        *fresh14 = *fresh13;
                                        n_1 = n_1.wrapping_sub(1);
                                        if !(n_1 > 0 as i32 as u64) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        let mut current_block_111: u64;
                        match type_0 as u32 {
                            9 | 10 => {
                                let fresh15 = fbp;
                                fbp = fbp.offset(1);
                                *fresh15 = 'l' as i32 as i8;
                                current_block_111 = 16625017989975992022;
                            }
                            7 | 8 | 14 | 16 => {
                                current_block_111 = 16625017989975992022;
                            }
                            12 => {
                                let fresh17 = fbp;
                                fbp = fbp.offset(1);
                                *fresh17 = 'L' as i32 as i8;
                                current_block_111 = 9073771928613846474;
                            }
                            _ => {
                                current_block_111 = 9073771928613846474;
                            }
                        }
                        match current_block_111 {
                            16625017989975992022 => {
                                let fresh16 = fbp;
                                fbp = fbp.offset(1);
                                *fresh16 = 'l' as i32 as i8;
                            }
                            _ => {}
                        }
                        *fbp = (*dp).conversion;
                        *fbp.offset(1 as i32 as isize) = '\0' as i32 as i8;
                        prefix_count = 0 as i32 as u32;
                        if 0 as i32 == 0
                            && (*dp).width_arg_index != !(0 as i32 as size_t)
                        {
                            if !((*(a.arg).offset((*dp).width_arg_index as isize)).type_0
                                as u32 == arg_type::TYPE_INT as i32 as u32)
                            {
                                abort();
                            }
                            let fresh18 = prefix_count;
                            prefix_count = prefix_count.wrapping_add(1);
                            prefixes[fresh18 as usize] = (*(a.arg)
                                .offset((*dp).width_arg_index as isize))
                                .a
                                .a_int;
                        }
                        if 0 as i32 == 0
                            && (*dp).precision_arg_index != !(0 as i32 as size_t)
                        {
                            if !((*(a.arg).offset((*dp).precision_arg_index as isize))
                                .type_0 as u32 == arg_type::TYPE_INT as i32 as u32)
                            {
                                abort();
                            }
                            let fresh19 = prefix_count;
                            prefix_count = prefix_count.wrapping_add(1);
                            prefixes[fresh19 as usize] = (*(a.arg)
                                .offset((*dp).precision_arg_index as isize))
                                .a
                                .a_int;
                        }
                        if xsum(
                            length,
                            (2 as i32 as u64)
                                .wrapping_add(
                                    (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                )
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_div(
                                    (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) > allocated
                        {
                            let mut memory_size_1: size_t = 0;
                            let mut memory_1: *mut i8 = 0 as *mut i8;
                            allocated = if allocated > 0 as i32 as u64 {
                                if allocated
                                    <= (18446744073709551615 as u64)
                                        .wrapping_div(2 as i32 as u64)
                                {
                                    allocated.wrapping_mul(2 as i32 as u64)
                                } else {
                                    18446744073709551615 as u64
                                }
                            } else {
                                12 as i32 as u64
                            };
                            if xsum(
                                length,
                                (2 as i32 as u64)
                                    .wrapping_add(
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                    )
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_div(
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                    ),
                            ) > allocated
                            {
                                allocated = xsum(
                                    length,
                                    (2 as i32 as u64)
                                        .wrapping_add(
                                            (::core::mem::size_of::<i8>() as u64)
                                                .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                        )
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_div(
                                            (::core::mem::size_of::<i8>() as u64)
                                                .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                        ),
                                );
                            }
                            memory_size_1 = if allocated
                                <= (18446744073709551615 as u64)
                                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                            {
                                allocated.wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            } else {
                                18446744073709551615 as u64
                            };
                            if memory_size_1 == 18446744073709551615 as u64 {
                                current_block = 9265786686875583127;
                                break;
                            }
                            if result == resultbuf || result.is_null() {
                                memory_1 = malloc(memory_size_1) as *mut i8;
                            } else {
                                memory_1 = realloc(
                                    result as *mut libc::c_void,
                                    memory_size_1,
                                ) as *mut i8;
                            }
                            if memory_1.is_null() {
                                current_block = 9265786686875583127;
                                break;
                            }
                            if result == resultbuf && length > 0 as i32 as u64 {
                                memcpy(
                                    memory_1 as *mut libc::c_void,
                                    result as *const libc::c_void,
                                    length,
                                );
                            }
                            result = memory_1;
                        }
                        *result.offset(length as isize) = '\0' as i32 as i8;
                        orig_errno = *__errno_location();
                        loop {
                            let mut count: i32 = -(1 as i32);
                            let mut retcount: i32 = 0 as i32;
                            let mut maxlen: size_t = allocated.wrapping_sub(length);
                            if maxlen
                                > (2147483647 as i32 as u64)
                                    .wrapping_div(
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                    )
                            {
                                maxlen = (2147483647 as i32 as u64)
                                    .wrapping_div(
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                    );
                            }
                            maxlen = maxlen
                                .wrapping_mul(
                                    (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                );
                            *__errno_location() = 0 as i32;
                            match type_0 as u32 {
                                1 => {
                                    let mut arg: i32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_schar as i32;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                2 => {
                                    let mut arg_0: u32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_uchar as u32;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_0,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_0,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_0,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                3 => {
                                    let mut arg_1: i32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_short as i32;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_1,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_1,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_1,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                4 => {
                                    let mut arg_2: u32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_ushort as u32;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_2,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_2,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_2,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                5 => {
                                    let mut arg_3: i32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_int;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_3,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_3,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_3,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                6 => {
                                    let mut arg_4: u32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_uint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_4,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_4,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_4,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                7 => {
                                    let mut arg_5: i64 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_longint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_5,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_5,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_5,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                8 => {
                                    let mut arg_6: u64 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_ulongint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_6,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_6,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_6,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                9 => {
                                    let mut arg_7: libc::c_longlong = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_longlongint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_7,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_7,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_7,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                10 => {
                                    let mut arg_8: libc::c_ulonglong = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_ulonglongint;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_8,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_8,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_8,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                11 => {
                                    let mut arg_9: libc::c_double = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_double;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_9,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_9,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_9,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                12 => {
                                    let mut arg_10: f128::f128 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_longdouble;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_10,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_10,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_10,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                13 => {
                                    let mut arg_11: i32 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_char;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_11,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_11,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_11,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                14 => {
                                    let mut arg_12: wint_t = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_wide_char;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_12,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_12,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_12,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                15 => {
                                    let mut arg_13: *const i8 = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_string;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_13,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_13,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_13,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                16 => {
                                    let mut arg_14: *const wchar_t = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_wide_string;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_14,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_14,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_14,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                17 => {
                                    let mut arg_15: *mut libc::c_void = (*(a.arg)
                                        .offset((*dp).arg_index as isize))
                                        .a
                                        .a_pointer;
                                    match prefix_count {
                                        0 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                arg_15,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        1 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                arg_15,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        2 => {
                                            retcount = snprintf(
                                                result.offset(length as isize),
                                                maxlen,
                                                buf,
                                                prefixes[0 as i32 as usize],
                                                prefixes[1 as i32 as usize],
                                                arg_15,
                                                &mut count as *mut i32,
                                            );
                                        }
                                        _ => {
                                            abort();
                                        }
                                    }
                                }
                                _ => {
                                    abort();
                                }
                            }
                            if count >= 0 as i32 {
                                if (count as u32 as u64) < maxlen
                                    && *result.offset(length as isize).offset(count as isize)
                                        as i32 != '\0' as i32
                                {
                                    abort();
                                }
                                if retcount > count {
                                    count = retcount;
                                }
                            } else if *fbp.offset(1 as i32 as isize) as i32
                                != '\0' as i32
                            {
                                *fbp.offset(1 as i32 as isize) = '\0' as i32 as i8;
                                continue;
                            } else if !(retcount < 0 as i32) {
                                count = retcount;
                            }
                            if count < 0 as i32 {
                                let mut saved_errno: i32 = *__errno_location();
                                if saved_errno == 0 as i32 {
                                    if (*dp).conversion as i32 == 'c' as i32
                                        || (*dp).conversion as i32 == 's' as i32
                                    {
                                        saved_errno = 84 as i32;
                                    } else {
                                        saved_errno = 22 as i32;
                                    }
                                }
                                if !(result == resultbuf || result.is_null()) {
                                    rpl_free(result as *mut libc::c_void);
                                }
                                if !buf_malloced.is_null() {
                                    rpl_free(buf_malloced as *mut libc::c_void);
                                }
                                if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                                    rpl_free(d.dir as *mut libc::c_void);
                                }
                                if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                                    rpl_free(a.arg as *mut libc::c_void);
                                }
                                *__errno_location() = saved_errno;
                                return 0 as *mut i8;
                            }
                            if (count as u32).wrapping_add(1 as i32 as u32) as u64
                                >= maxlen
                            {
                                if maxlen
                                    == (2147483647 as i32 as u64)
                                        .wrapping_div(
                                            (::core::mem::size_of::<i8>() as u64)
                                                .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                        )
                                {
                                    current_block = 5571981504993195602;
                                    break 's_139;
                                }
                                let mut n_2: size_t = xmax(
                                    xsum(
                                        length,
                                        ((count as u32).wrapping_add(2 as i32 as u32) as u64)
                                            .wrapping_add(
                                                (::core::mem::size_of::<i8>() as u64)
                                                    .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                            )
                                            .wrapping_sub(1 as i32 as u64)
                                            .wrapping_div(
                                                (::core::mem::size_of::<i8>() as u64)
                                                    .wrapping_div(::core::mem::size_of::<i8>() as u64),
                                            ),
                                    ),
                                    if allocated
                                        <= (18446744073709551615 as u64)
                                            .wrapping_div(2 as i32 as u64)
                                    {
                                        allocated.wrapping_mul(2 as i32 as u64)
                                    } else {
                                        18446744073709551615 as u64
                                    },
                                );
                                if !(n_2 > allocated) {
                                    continue;
                                }
                                let mut memory_size_2: size_t = 0;
                                let mut memory_2: *mut i8 = 0 as *mut i8;
                                allocated = if allocated > 0 as i32 as u64 {
                                    if allocated
                                        <= (18446744073709551615 as u64)
                                            .wrapping_div(2 as i32 as u64)
                                    {
                                        allocated.wrapping_mul(2 as i32 as u64)
                                    } else {
                                        18446744073709551615 as u64
                                    }
                                } else {
                                    12 as i32 as u64
                                };
                                if n_2 > allocated {
                                    allocated = n_2;
                                }
                                memory_size_2 = if allocated
                                    <= (18446744073709551615 as u64)
                                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                {
                                    allocated.wrapping_mul(::core::mem::size_of::<i8>() as u64)
                                } else {
                                    18446744073709551615 as u64
                                };
                                if memory_size_2 == 18446744073709551615 as u64 {
                                    current_block = 9265786686875583127;
                                    break 's_139;
                                }
                                if result == resultbuf || result.is_null() {
                                    memory_2 = malloc(memory_size_2) as *mut i8;
                                } else {
                                    memory_2 = realloc(
                                        result as *mut libc::c_void,
                                        memory_size_2,
                                    ) as *mut i8;
                                }
                                if memory_2.is_null() {
                                    current_block = 9265786686875583127;
                                    break 's_139;
                                }
                                if result == resultbuf && length > 0 as i32 as u64 {
                                    memcpy(
                                        memory_2 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_2;
                            } else {
                                length = (length as u64).wrapping_add(count as u64)
                                    as size_t as size_t;
                                break;
                            }
                        }
                        *__errno_location() = orig_errno;
                    }
                }
                cp = (*dp).dir_end;
                i = i.wrapping_add(1);
                i;
                dp = dp.offset(1);
                dp;
            }
            match current_block {
                3105969743042996984 => {
                    if xsum(length, 1 as i32 as size_t) > allocated {
                        let mut memory_size_3: size_t = 0;
                        let mut memory_3: *mut i8 = 0 as *mut i8;
                        allocated = if allocated > 0 as i32 as u64 {
                            if allocated
                                <= (18446744073709551615 as u64)
                                    .wrapping_div(2 as i32 as u64)
                            {
                                allocated.wrapping_mul(2 as i32 as u64)
                            } else {
                                18446744073709551615 as u64
                            }
                        } else {
                            12 as i32 as u64
                        };
                        if xsum(length, 1 as i32 as size_t) > allocated {
                            allocated = xsum(length, 1 as i32 as size_t);
                        }
                        memory_size_3 = if allocated
                            <= (18446744073709551615 as u64)
                                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        {
                            allocated.wrapping_mul(::core::mem::size_of::<i8>() as u64)
                        } else {
                            18446744073709551615 as u64
                        };
                        if memory_size_3 == 18446744073709551615 as u64 {
                            current_block = 9265786686875583127;
                        } else {
                            if result == resultbuf || result.is_null() {
                                memory_3 = malloc(memory_size_3) as *mut i8;
                            } else {
                                memory_3 = realloc(
                                    result as *mut libc::c_void,
                                    memory_size_3,
                                ) as *mut i8;
                            }
                            if memory_3.is_null() {
                                current_block = 9265786686875583127;
                            } else {
                                if result == resultbuf && length > 0 as i32 as u64 {
                                    memcpy(
                                        memory_3 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_3;
                                current_block = 8098702283015542489;
                            }
                        }
                    } else {
                        current_block = 8098702283015542489;
                    }
                    match current_block {
                        9265786686875583127 => {}
                        _ => {
                            *result.offset(length as isize) = '\0' as i32 as i8;
                            if result != resultbuf
                                && length.wrapping_add(1 as i32 as u64) < allocated
                            {
                                let mut memory_4: *mut i8 = 0 as *mut i8;
                                memory_4 = realloc(
                                    result as *mut libc::c_void,
                                    length
                                        .wrapping_add(1 as i32 as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ) as *mut i8;
                                if !memory_4.is_null() {
                                    result = memory_4;
                                }
                            }
                            if !buf_malloced.is_null() {
                                rpl_free(buf_malloced as *mut libc::c_void);
                            }
                            if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                                rpl_free(d.dir as *mut libc::c_void);
                            }
                            if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                                rpl_free(a.arg as *mut libc::c_void);
                            }
                            *lengthp = length;
                            return result;
                        }
                    }
                }
                5571981504993195602 => {
                    if !(result == resultbuf || result.is_null()) {
                        rpl_free(result as *mut libc::c_void);
                    }
                    if !buf_malloced.is_null() {
                        rpl_free(buf_malloced as *mut libc::c_void);
                    }
                    if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                        rpl_free(d.dir as *mut libc::c_void);
                    }
                    if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                        rpl_free(a.arg as *mut libc::c_void);
                    }
                    *__errno_location() = 75 as i32;
                    return 0 as *mut i8;
                }
                _ => {}
            }
            if !(result == resultbuf || result.is_null()) {
                rpl_free(result as *mut libc::c_void);
            }
            if !buf_malloced.is_null() {
                rpl_free(buf_malloced as *mut libc::c_void);
            }
        }
        _ => {}
    }
    if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
        rpl_free(d.dir as *mut libc::c_void);
    }
    if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
        rpl_free(a.arg as *mut libc::c_void);
    }
    *__errno_location() = 12 as i32;
    return 0 as *mut i8;
}
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
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
}  // end of enum

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
    pub a_uchar: libc::c_uchar,
    pub a_short: libc::c_short,
    pub a_ushort: libc::c_ushort,
    pub a_int: libc::c_int,
    pub a_uint: libc::c_uint,
    pub a_longint: libc::c_long,
    pub a_ulongint: libc::c_ulong,
    pub a_longlongint: libc::c_longlong,
    pub a_ulonglongint: libc::c_ulonglong,
    pub a_float: libc::c_float,
    pub a_double: libc::c_double,
    pub a_longdouble: f128::f128,
    pub a_char: libc::c_int,
    pub a_wide_char: wint_t,
    pub a_string: *const libc::c_char,
    pub a_wide_string: *const wchar_t,
    pub a_pointer: *mut libc::c_void,
    pub a_count_schar_pointer: *mut libc::c_schar,
    pub a_count_short_pointer: *mut libc::c_short,
    pub a_count_int_pointer: *mut libc::c_int,
    pub a_count_longint_pointer: *mut libc::c_long,
    pub a_count_longlongint_pointer: *mut libc::c_longlong,
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
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ap: *mut argument = 0 as *mut argument;
    i = 0 as libc::c_int as size_t;
    ap = &mut *((*a).arg).offset(0 as libc::c_int as isize) as *mut argument;
    while i < (*a).count {
        match (*ap).type_0 as libc::c_uint {
            1 => {
                (*ap).a.a_schar = args.arg::<libc::c_int>() as libc::c_schar;
            }
            2 => {
                (*ap).a.a_uchar = args.arg::<libc::c_int>() as libc::c_uchar;
            }
            3 => {
                (*ap).a.a_short = args.arg::<libc::c_int>() as libc::c_short;
            }
            4 => {
                (*ap).a.a_ushort = args.arg::<libc::c_int>() as libc::c_ushort;
            }
            5 => {
                (*ap).a.a_int = args.arg::<libc::c_int>();
            }
            6 => {
                (*ap).a.a_uint = args.arg::<libc::c_uint>();
            }
            7 => {
                (*ap).a.a_longint = args.arg::<libc::c_long>();
            }
            8 => {
                (*ap).a.a_ulongint = args.arg::<libc::c_ulong>();
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
                (*ap).a.a_char = args.arg::<libc::c_int>();
            }
            14 => {
                (*ap)
                    .a
                    .a_wide_char = if (::core::mem::size_of::<wint_t>() as libc::c_ulong)
                    < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    args.arg::<libc::c_int>() as wint_t
                } else {
                    args.arg::<wint_t>()
                };
            }
            15 => {
                (*ap).a.a_string = args.arg::<*const libc::c_char>();
                if ((*ap).a.a_string).is_null() {
                    (*ap).a.a_string = b"(NULL)\0" as *const u8 as *const libc::c_char;
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
                        0 as libc::c_int,
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
                (*ap).a.a_count_int_pointer = args.arg::<*mut libc::c_int>();
            }
            21 => {
                (*ap).a.a_count_longint_pointer = args.arg::<*mut libc::c_long>();
            }
            22 => {
                (*ap)
                    .a
                    .a_count_longlongint_pointer = args.arg::<*mut libc::c_longlong>();
            }
            _ => return -(1 as libc::c_int),
        }
        i = i.wrapping_add(1);
        i;
        ap = ap.offset(1);
        ap;
    }
    return 0 as libc::c_int;
}

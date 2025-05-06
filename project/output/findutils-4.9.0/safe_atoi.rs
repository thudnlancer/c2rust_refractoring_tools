#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __errno_location() -> *mut i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
}
pub type quoting_style = u32;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: i32,
}
#[no_mangle]
pub unsafe extern "C" fn safe_atoi(mut s: *const i8, mut style: quoting_style) -> i32 {
    let mut lval: i64 = 0;
    let mut end: *mut i8 = 0 as *mut i8;
    *__errno_location() = 0 as i32;
    lval = strtol(s, &mut end, 10 as i32);
    if 9223372036854775807 as i64 == lval
        || -(9223372036854775807 as i64) - 1 as i64 == lval
    {
        if *__errno_location() == 34 as i32 {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    s,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    s,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    s,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    s,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if lval > 2147483647 as i32 as i64 || lval < (-(2147483647 as i32) - 1 as i32) as i64
    {
        *__errno_location() = 34 as i32;
        if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
            error(1 as i32, *__errno_location(), b"%s\0" as *const u8 as *const i8, s);
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(1 as i32, *__errno_location(), b"%s\0" as *const u8 as *const i8, s);
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    } else if *end != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Unexpected suffix %s on %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, style, end),
                quotearg_n_style(1 as i32, style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Unexpected suffix %s on %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, style, end),
                quotearg_n_style(1 as i32, style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    } else if end == s as *mut i8 {
        if ::core::mem::size_of::<C2RustUnnamed>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Expected an integer: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Expected an integer: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, style, s),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    return lval as i32;
}
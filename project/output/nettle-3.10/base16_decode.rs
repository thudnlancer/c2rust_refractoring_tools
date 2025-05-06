#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn abort() -> !;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: u8,
    pub bits: u8,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base16_decode_init(mut ctx: *mut base16_decode_ctx) {
    (*ctx).bits = 0 as i32 as u8;
    (*ctx).word = (*ctx).bits;
}
static mut hex_decode_table: [libc::c_schar; 128] = [
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    0 as i32 as libc::c_schar,
    1 as i32 as libc::c_schar,
    2 as i32 as libc::c_schar,
    3 as i32 as libc::c_schar,
    4 as i32 as libc::c_schar,
    5 as i32 as libc::c_schar,
    6 as i32 as libc::c_schar,
    7 as i32 as libc::c_schar,
    8 as i32 as libc::c_schar,
    9 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    10 as i32 as libc::c_schar,
    11 as i32 as libc::c_schar,
    12 as i32 as libc::c_schar,
    13 as i32 as libc::c_schar,
    14 as i32 as libc::c_schar,
    15 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    10 as i32 as libc::c_schar,
    11 as i32 as libc::c_schar,
    12 as i32 as libc::c_schar,
    13 as i32 as libc::c_schar,
    14 as i32 as libc::c_schar,
    15 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
];
#[no_mangle]
pub unsafe extern "C" fn nettle_base16_decode_single(
    mut ctx: *mut base16_decode_ctx,
    mut dst: *mut uint8_t,
    mut src: i8,
) -> i32 {
    let mut usrc: u8 = src as u8;
    let mut digit: i32 = 0;
    if usrc as i32 >= 0x80 as i32 {
        return -(1 as i32);
    }
    digit = hex_decode_table[usrc as usize] as i32;
    match digit {
        -1 => return -(1 as i32),
        -2 => return 0 as i32,
        _ => {
            if digit >= 0 as i32 {} else {
                __assert_fail(
                    b"digit >= 0\0" as *const u8 as *const i8,
                    b"base16-decode.c\0" as *const u8 as *const i8,
                    86 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 77],
                        &[i8; 77],
                    >(
                        b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1361: {
                if digit >= 0 as i32 {} else {
                    __assert_fail(
                        b"digit >= 0\0" as *const u8 as *const i8,
                        b"base16-decode.c\0" as *const u8 as *const i8,
                        86 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 77],
                            &[i8; 77],
                        >(
                            b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if digit < 0x10 as i32 {} else {
                __assert_fail(
                    b"digit < 0x10\0" as *const u8 as *const i8,
                    b"base16-decode.c\0" as *const u8 as *const i8,
                    87 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 77],
                        &[i8; 77],
                    >(
                        b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_1319: {
                if digit < 0x10 as i32 {} else {
                    __assert_fail(
                        b"digit < 0x10\0" as *const u8 as *const i8,
                        b"base16-decode.c\0" as *const u8 as *const i8,
                        87 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 77],
                            &[i8; 77],
                        >(
                            b"int nettle_base16_decode_single(struct base16_decode_ctx *, uint8_t *, char)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*ctx).bits != 0 {
                *dst = (((*ctx).word as i32) << 4 as i32 | digit) as uint8_t;
                (*ctx).bits = 0 as i32 as u8;
                return 1 as i32;
            } else {
                (*ctx).word = digit as u8;
                (*ctx).bits = 4 as i32 as u8;
                return 0 as i32;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base16_decode_update(
    mut ctx: *mut base16_decode_ctx,
    mut dst_length: *mut size_t,
    mut dst: *mut uint8_t,
    mut src_length: size_t,
    mut src: *const i8,
) -> i32 {
    let mut done: size_t = 0;
    let mut i: size_t = 0;
    done = 0 as i32 as size_t;
    i = done;
    while i < src_length {
        match nettle_base16_decode_single(
            ctx,
            dst.offset(done as isize),
            *src.offset(i as isize),
        ) {
            -1 => return 0 as i32,
            1 => {
                done = done.wrapping_add(1);
                done;
            }
            0 => {}
            _ => {
                abort();
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if done <= src_length.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
    {} else {
        __assert_fail(
            b"done <= BASE16_DECODE_LENGTH(src_length)\0" as *const u8 as *const i8,
            b"base16-decode.c\0" as *const u8 as *const i8,
            128 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 103],
                &[i8; 103],
            >(
                b"int nettle_base16_decode_update(struct base16_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1818: {
        if done <= src_length.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64)
        {} else {
            __assert_fail(
                b"done <= BASE16_DECODE_LENGTH(src_length)\0" as *const u8 as *const i8,
                b"base16-decode.c\0" as *const u8 as *const i8,
                128 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 103],
                    &[i8; 103],
                >(
                    b"int nettle_base16_decode_update(struct base16_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *dst_length = done;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base16_decode_final(
    mut ctx: *mut base16_decode_ctx,
) -> i32 {
    return ((*ctx).bits as i32 == 0 as i32) as i32;
}
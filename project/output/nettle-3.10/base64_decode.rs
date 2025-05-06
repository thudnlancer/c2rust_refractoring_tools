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
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: u8,
    pub padding: u8,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_decode_init(mut ctx: *mut base64_decode_ctx) {
    static mut base64_decode_table: [libc::c_schar; 256] = [
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
        -(2 as i32) as libc::c_schar,
        -(2 as i32) as libc::c_schar,
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
        62 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        63 as i32 as libc::c_schar,
        52 as i32 as libc::c_schar,
        53 as i32 as libc::c_schar,
        54 as i32 as libc::c_schar,
        55 as i32 as libc::c_schar,
        56 as i32 as libc::c_schar,
        57 as i32 as libc::c_schar,
        58 as i32 as libc::c_schar,
        59 as i32 as libc::c_schar,
        60 as i32 as libc::c_schar,
        61 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(3 as i32) as libc::c_schar,
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
        10 as i32 as libc::c_schar,
        11 as i32 as libc::c_schar,
        12 as i32 as libc::c_schar,
        13 as i32 as libc::c_schar,
        14 as i32 as libc::c_schar,
        15 as i32 as libc::c_schar,
        16 as i32 as libc::c_schar,
        17 as i32 as libc::c_schar,
        18 as i32 as libc::c_schar,
        19 as i32 as libc::c_schar,
        20 as i32 as libc::c_schar,
        21 as i32 as libc::c_schar,
        22 as i32 as libc::c_schar,
        23 as i32 as libc::c_schar,
        24 as i32 as libc::c_schar,
        25 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        26 as i32 as libc::c_schar,
        27 as i32 as libc::c_schar,
        28 as i32 as libc::c_schar,
        29 as i32 as libc::c_schar,
        30 as i32 as libc::c_schar,
        31 as i32 as libc::c_schar,
        32 as i32 as libc::c_schar,
        33 as i32 as libc::c_schar,
        34 as i32 as libc::c_schar,
        35 as i32 as libc::c_schar,
        36 as i32 as libc::c_schar,
        37 as i32 as libc::c_schar,
        38 as i32 as libc::c_schar,
        39 as i32 as libc::c_schar,
        40 as i32 as libc::c_schar,
        41 as i32 as libc::c_schar,
        42 as i32 as libc::c_schar,
        43 as i32 as libc::c_schar,
        44 as i32 as libc::c_schar,
        45 as i32 as libc::c_schar,
        46 as i32 as libc::c_schar,
        47 as i32 as libc::c_schar,
        48 as i32 as libc::c_schar,
        49 as i32 as libc::c_schar,
        50 as i32 as libc::c_schar,
        51 as i32 as libc::c_schar,
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
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
    ];
    (*ctx).padding = 0 as i32 as u8;
    (*ctx).bits = (*ctx).padding;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).table = base64_decode_table.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_decode_single(
    mut ctx: *mut base64_decode_ctx,
    mut dst: *mut uint8_t,
    mut src: i8,
) -> i32 {
    let mut data: i32 = *((*ctx).table).offset(src as uint8_t as isize) as i32;
    match data {
        -1 => return -(1 as i32),
        -2 => return 0 as i32,
        -3 => {
            if (*ctx).bits == 0 || (*ctx).padding as i32 > 2 as i32 {
                return -(1 as i32);
            }
            if (*ctx).word as i32 & ((1 as i32) << (*ctx).bits as i32) - 1 as i32 != 0 {
                return -(1 as i32);
            }
            (*ctx).padding = ((*ctx).padding).wrapping_add(1);
            (*ctx).padding;
            (*ctx).bits = ((*ctx).bits as i32 - 2 as i32) as u8;
            return 0 as i32;
        }
        _ => {
            if data >= 0 as i32 && data < 0x40 as i32 {} else {
                __assert_fail(
                    b"data >= 0 && data < 0x40\0" as *const u8 as *const i8,
                    b"base64-decode.c\0" as *const u8 as *const i8,
                    83 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 77],
                        &[i8; 77],
                    >(
                        b"int nettle_base64_decode_single(struct base64_decode_ctx *, uint8_t *, char)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2163: {
                if data >= 0 as i32 && data < 0x40 as i32 {} else {
                    __assert_fail(
                        b"data >= 0 && data < 0x40\0" as *const u8 as *const i8,
                        b"base64-decode.c\0" as *const u8 as *const i8,
                        83 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 77],
                            &[i8; 77],
                        >(
                            b"int nettle_base64_decode_single(struct base64_decode_ctx *, uint8_t *, char)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*ctx).padding != 0 {
                return -(1 as i32);
            }
            (*ctx).word = (((*ctx).word as i32) << 6 as i32 | data) as libc::c_ushort;
            (*ctx).bits = ((*ctx).bits as i32 + 6 as i32) as u8;
            if (*ctx).bits as i32 >= 8 as i32 {
                (*ctx).bits = ((*ctx).bits as i32 - 8 as i32) as u8;
                *dst.offset(0 as i32 as isize) = ((*ctx).word as i32
                    >> (*ctx).bits as i32) as uint8_t;
                return 1 as i32;
            } else {
                return 0 as i32
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_decode_update(
    mut ctx: *mut base64_decode_ctx,
    mut dst_length: *mut size_t,
    mut dst: *mut uint8_t,
    mut src_length: size_t,
    mut src: *const i8,
) -> i32 {
    let mut done: size_t = 0;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    done = 0 as i32 as size_t;
    while i < src_length {
        match nettle_base64_decode_single(
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
    if done
        <= src_length
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(6 as i32 as u64)
            .wrapping_div(8 as i32 as u64)
    {} else {
        __assert_fail(
            b"done <= BASE64_DECODE_LENGTH(src_length)\0" as *const u8 as *const i8,
            b"base64-decode.c\0" as *const u8 as *const i8,
            144 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 103],
                &[i8; 103],
            >(
                b"int nettle_base64_decode_update(struct base64_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2237: {
        if done
            <= src_length
                .wrapping_add(1 as i32 as u64)
                .wrapping_mul(6 as i32 as u64)
                .wrapping_div(8 as i32 as u64)
        {} else {
            __assert_fail(
                b"done <= BASE64_DECODE_LENGTH(src_length)\0" as *const u8 as *const i8,
                b"base64-decode.c\0" as *const u8 as *const i8,
                144 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 103],
                    &[i8; 103],
                >(
                    b"int nettle_base64_decode_update(struct base64_decode_ctx *, size_t *, uint8_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *dst_length = done;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_decode_final(
    mut ctx: *mut base64_decode_ctx,
) -> i32 {
    return ((*ctx).bits as i32 == 0 as i32) as i32;
}
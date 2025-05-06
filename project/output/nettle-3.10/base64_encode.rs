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
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const i8,
    pub word: libc::c_ushort,
    pub bits: u8,
}
unsafe extern "C" fn encode_raw(
    mut alphabet: *const i8,
    mut dst: *mut i8,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    let mut in_0: *const uint8_t = src.offset(length as isize);
    let mut out: *mut i8 = dst
        .offset(
            length
                .wrapping_add(2 as i32 as u64)
                .wrapping_div(3 as i32 as u64)
                .wrapping_mul(4 as i32 as u64) as isize,
        );
    let mut left_over: u32 = length.wrapping_rem(3 as i32 as u64) as u32;
    if left_over != 0 {
        in_0 = in_0.offset(-(left_over as isize));
        out = out.offset(-1);
        *out = '=' as i32 as i8;
        match left_over {
            1 => {
                out = out.offset(-1);
                *out = '=' as i32 as i8;
                out = out.offset(-1);
                *out = *alphabet
                    .offset(
                        (0x3f as i32
                            & (*in_0.offset(0 as i32 as isize) as i32) << 4 as i32)
                            as isize,
                    );
            }
            2 => {
                out = out.offset(-1);
                *out = *alphabet
                    .offset(
                        (0x3f as i32
                            & (*in_0.offset(1 as i32 as isize) as i32) << 2 as i32)
                            as isize,
                    );
                out = out.offset(-1);
                *out = *alphabet
                    .offset(
                        (0x3f as i32
                            & ((*in_0.offset(0 as i32 as isize) as i32) << 4 as i32
                                | *in_0.offset(1 as i32 as isize) as i32 >> 4 as i32))
                            as isize,
                    );
            }
            _ => {
                abort();
            }
        }
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as i32 & *in_0.offset(0 as i32 as isize) as i32 >> 2 as i32)
                    as isize,
            );
    }
    while in_0 > src {
        in_0 = in_0.offset(-(3 as i32 as isize));
        out = out.offset(-1);
        *out = *alphabet
            .offset((0x3f as i32 & *in_0.offset(2 as i32 as isize) as i32) as isize);
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as i32
                    & ((*in_0.offset(1 as i32 as isize) as i32) << 2 as i32
                        | *in_0.offset(2 as i32 as isize) as i32 >> 6 as i32)) as isize,
            );
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as i32
                    & ((*in_0.offset(0 as i32 as isize) as i32) << 4 as i32
                        | *in_0.offset(1 as i32 as isize) as i32 >> 4 as i32)) as isize,
            );
        out = out.offset(-1);
        *out = *alphabet
            .offset(
                (0x3f as i32 & *in_0.offset(0 as i32 as isize) as i32 >> 2 as i32)
                    as isize,
            );
    }
    if in_0 == src {} else {
        __assert_fail(
            b"in == src\0" as *const u8 as *const i8,
            b"base64-encode.c\0" as *const u8 as *const i8,
            82 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 63],
                &[i8; 63],
            >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1633: {
        if in_0 == src {} else {
            __assert_fail(
                b"in == src\0" as *const u8 as *const i8,
                b"base64-encode.c\0" as *const u8 as *const i8,
                82 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 63],
                    &[i8; 63],
                >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if out == dst {} else {
        __assert_fail(
            b"out == dst\0" as *const u8 as *const i8,
            b"base64-encode.c\0" as *const u8 as *const i8,
            83 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 63],
                &[i8; 63],
            >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1574: {
        if out == dst {} else {
            __assert_fail(
                b"out == dst\0" as *const u8 as *const i8,
                b"base64-encode.c\0" as *const u8 as *const i8,
                83 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 63],
                    &[i8; 63],
                >(b"void encode_raw(const char *, char *, size_t, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
}
static mut base64_encode_table: [i8; 64] = unsafe {
    *::core::mem::transmute::<
        &[u8; 64],
        &[i8; 64],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
};
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_encode_raw(
    mut dst: *mut i8,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    encode_raw(base64_encode_table.as_ptr(), dst, length, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_encode_group(
    mut dst: *mut i8,
    mut group: uint32_t,
) {
    let fresh0 = dst;
    dst = dst.offset(1);
    *fresh0 = base64_encode_table[(0x3f as i32 as u32 & group >> 18 as i32) as usize];
    let fresh1 = dst;
    dst = dst.offset(1);
    *fresh1 = base64_encode_table[(0x3f as i32 as u32 & group >> 12 as i32) as usize];
    let fresh2 = dst;
    dst = dst.offset(1);
    *fresh2 = base64_encode_table[(0x3f as i32 as u32 & group >> 6 as i32) as usize];
    let fresh3 = dst;
    dst = dst.offset(1);
    *fresh3 = base64_encode_table[(0x3f as i32 as u32 & group) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_encode_init(mut ctx: *mut base64_encode_ctx) {
    (*ctx).bits = 0 as i32 as u8;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).alphabet = base64_encode_table.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_encode_single(
    mut ctx: *mut base64_encode_ctx,
    mut dst: *mut i8,
    mut src: uint8_t,
) -> size_t {
    let mut done: u32 = 0 as i32 as u32;
    let mut word: u32 = (((*ctx).word as i32) << 8 as i32 | src as i32) as u32;
    let mut bits: u32 = ((*ctx).bits as i32 + 8 as i32) as u32;
    while bits >= 6 as i32 as u32 {
        bits = bits.wrapping_sub(6 as i32 as u32);
        let fresh4 = done;
        done = done.wrapping_add(1);
        *dst.offset(fresh4 as isize) = *((*ctx).alphabet)
            .offset((0x3f as i32 as u32 & word >> bits) as isize);
    }
    (*ctx).bits = bits as u8;
    (*ctx).word = word as libc::c_ushort;
    if done <= 2 as i32 as u32 {} else {
        __assert_fail(
            b"done <= 2\0" as *const u8 as *const i8,
            b"base64-encode.c\0" as *const u8 as *const i8,
            132 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[i8; 80],
            >(
                b"size_t nettle_base64_encode_single(struct base64_encode_ctx *, char *, uint8_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1276: {
        if done <= 2 as i32 as u32 {} else {
            __assert_fail(
                b"done <= 2\0" as *const u8 as *const i8,
                b"base64-encode.c\0" as *const u8 as *const i8,
                132 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[i8; 80],
                >(
                    b"size_t nettle_base64_encode_single(struct base64_encode_ctx *, char *, uint8_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return done as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_encode_update(
    mut ctx: *mut base64_encode_ctx,
    mut dst: *mut i8,
    mut length: size_t,
    mut src: *const uint8_t,
) -> size_t {
    let mut done: size_t = 0 as i32 as size_t;
    let mut left: size_t = length;
    let mut left_over: u32 = 0;
    let mut bulk: size_t = 0;
    while (*ctx).bits as i32 != 0 && left != 0 {
        left = left.wrapping_sub(1);
        left;
        let fresh5 = src;
        src = src.offset(1);
        done = (done as u64)
            .wrapping_add(
                nettle_base64_encode_single(ctx, dst.offset(done as isize), *fresh5),
            ) as size_t as size_t;
    }
    left_over = left.wrapping_rem(3 as i32 as u64) as u32;
    bulk = left.wrapping_sub(left_over as u64);
    if bulk != 0 {
        if bulk.wrapping_rem(3 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(bulk % 3)\0" as *const u8 as *const i8,
                b"base64-encode.c\0" as *const u8 as *const i8,
                161 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[i8; 96],
                >(
                    b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_1949: {
            if bulk.wrapping_rem(3 as i32 as u64) == 0 {} else {
                __assert_fail(
                    b"!(bulk % 3)\0" as *const u8 as *const i8,
                    b"base64-encode.c\0" as *const u8 as *const i8,
                    161 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 96],
                        &[i8; 96],
                    >(
                        b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        encode_raw((*ctx).alphabet, dst.offset(done as isize), bulk, src);
        done = (done as u64)
            .wrapping_add(
                bulk
                    .wrapping_add(2 as i32 as u64)
                    .wrapping_div(3 as i32 as u64)
                    .wrapping_mul(4 as i32 as u64),
            ) as size_t as size_t;
        src = src.offset(bulk as isize);
        left = left_over as size_t;
    }
    while left != 0 {
        left = left.wrapping_sub(1);
        left;
        let fresh6 = src;
        src = src.offset(1);
        done = (done as u64)
            .wrapping_add(
                nettle_base64_encode_single(ctx, dst.offset(done as isize), *fresh6),
            ) as size_t as size_t;
    }
    if done
        <= length
            .wrapping_mul(8 as i32 as u64)
            .wrapping_add(4 as i32 as u64)
            .wrapping_div(6 as i32 as u64)
    {} else {
        __assert_fail(
            b"done <= BASE64_ENCODE_LENGTH(length)\0" as *const u8 as *const i8,
            b"base64-encode.c\0" as *const u8 as *const i8,
            175 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[i8; 96],
            >(
                b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1415: {
        if done
            <= length
                .wrapping_mul(8 as i32 as u64)
                .wrapping_add(4 as i32 as u64)
                .wrapping_div(6 as i32 as u64)
        {} else {
            __assert_fail(
                b"done <= BASE64_ENCODE_LENGTH(length)\0" as *const u8 as *const i8,
                b"base64-encode.c\0" as *const u8 as *const i8,
                175 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[i8; 96],
                >(
                    b"size_t nettle_base64_encode_update(struct base64_encode_ctx *, char *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return done;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64_encode_final(
    mut ctx: *mut base64_encode_ctx,
    mut dst: *mut i8,
) -> size_t {
    let mut done: u32 = 0 as i32 as u32;
    let mut bits: u32 = (*ctx).bits as u32;
    if bits != 0 {
        let fresh7 = done;
        done = done.wrapping_add(1);
        *dst.offset(fresh7 as isize) = *((*ctx).alphabet)
            .offset(
                (0x3f as i32 & ((*ctx).word as i32) << 6 as i32 - (*ctx).bits as i32)
                    as isize,
            );
        while bits < 6 as i32 as u32 {
            let fresh8 = done;
            done = done.wrapping_add(1);
            *dst.offset(fresh8 as isize) = '=' as i32 as i8;
            bits = bits.wrapping_add(2 as i32 as u32);
        }
        (*ctx).bits = 0 as i32 as u8;
    }
    if done <= 3 as i32 as u32 {} else {
        __assert_fail(
            b"done <= BASE64_ENCODE_FINAL_LENGTH\0" as *const u8 as *const i8,
            b"base64-encode.c\0" as *const u8 as *const i8,
            198 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"size_t nettle_base64_encode_final(struct base64_encode_ctx *, char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2054: {
        if done <= 3 as i32 as u32 {} else {
            __assert_fail(
                b"done <= BASE64_ENCODE_FINAL_LENGTH\0" as *const u8 as *const i8,
                b"base64-encode.c\0" as *const u8 as *const i8,
                198 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"size_t nettle_base64_encode_final(struct base64_encode_ctx *, char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return done as size_t;
}
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
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arcfour_ctx {
    pub S: [uint8_t; 256],
    pub i: uint8_t,
    pub j: uint8_t,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arcfour_set_key(
    mut ctx: *mut arcfour_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut k: u32 = 0;
    if length >= 1 as i32 as u64 {} else {
        __assert_fail(
            b"length >= ARCFOUR_MIN_KEY_SIZE\0" as *const u8 as *const i8,
            b"arcfour.c\0" as *const u8 as *const i8,
            50 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_416: {
        if length >= 1 as i32 as u64 {} else {
            __assert_fail(
                b"length >= ARCFOUR_MIN_KEY_SIZE\0" as *const u8 as *const i8,
                b"arcfour.c\0" as *const u8 as *const i8,
                50 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 256 as i32 as u64 {} else {
        __assert_fail(
            b"length <= ARCFOUR_MAX_KEY_SIZE\0" as *const u8 as *const i8,
            b"arcfour.c\0" as *const u8 as *const i8,
            51 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_373: {
        if length <= 256 as i32 as u64 {} else {
            __assert_fail(
                b"length <= ARCFOUR_MAX_KEY_SIZE\0" as *const u8 as *const i8,
                b"arcfour.c\0" as *const u8 as *const i8,
                51 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"void nettle_arcfour_set_key(struct arcfour_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as i32 as u32;
    while i < 256 as i32 as u32 {
        (*ctx).S[i as usize] = i as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    k = 0 as i32 as u32;
    j = k;
    i = j;
    while i < 256 as i32 as u32 {
        j = j
            .wrapping_add(
                ((*ctx).S[i as usize] as i32 + *key.offset(k as isize) as i32) as u32,
            );
        j &= 0xff as i32 as u32;
        let mut _t: i32 = (*ctx).S[i as usize] as i32;
        (*ctx).S[i as usize] = (*ctx).S[j as usize];
        (*ctx).S[j as usize] = _t as uint8_t;
        k = (k.wrapping_add(1 as i32 as u32) as u64).wrapping_rem(length) as u32;
        i = i.wrapping_add(1);
        i;
    }
    (*ctx).j = 0 as i32 as uint8_t;
    (*ctx).i = (*ctx).j;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arcfour128_set_key(
    mut ctx: *mut arcfour_ctx,
    mut key: *const uint8_t,
) {
    nettle_arcfour_set_key(ctx, 16 as i32 as size_t, key);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arcfour_crypt(
    mut ctx: *mut arcfour_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut i: uint8_t = 0;
    let mut j: uint8_t = 0;
    let mut si: i32 = 0;
    let mut sj: i32 = 0;
    i = (*ctx).i;
    j = (*ctx).j;
    loop {
        let fresh0 = length;
        length = length.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        i = i.wrapping_add(1);
        i;
        i = (i as i32 & 0xff as i32) as uint8_t;
        si = (*ctx).S[i as usize] as i32;
        j = (j as i32 + si) as uint8_t;
        j = (j as i32 & 0xff as i32) as uint8_t;
        (*ctx).S[i as usize] = (*ctx).S[j as usize];
        sj = (*ctx).S[i as usize] as i32;
        (*ctx).S[j as usize] = si as uint8_t;
        let fresh1 = src;
        src = src.offset(1);
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = (*fresh1 as i32 ^ (*ctx).S[(si + sj & 0xff as i32) as usize] as i32)
            as uint8_t;
    }
    (*ctx).i = i;
    (*ctx).j = j;
}
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
pub struct knuth_lfib_ctx {
    pub x: [uint32_t; 100],
    pub index: u32,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_knuth_lfib_init(
    mut ctx: *mut knuth_lfib_ctx,
    mut seed: uint32_t,
) {
    let mut t: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut x: [uint32_t; 199] = [0; 199];
    let mut ss: uint32_t = (seed.wrapping_add(2 as i32 as u32) as u64
        & ((1 as u64) << 30 as i32).wrapping_sub(2 as i32 as u64)) as uint32_t;
    j = 0 as i32 as uint32_t;
    while j < 100 as i32 as u32 {
        x[j as usize] = ss;
        ss <<= 1 as i32;
        if ss as u64 >= (1 as u64) << 30 as i32 {
            ss = (ss as u64)
                .wrapping_sub(((1 as u64) << 30 as i32).wrapping_sub(2 as i32 as u64))
                as uint32_t as uint32_t;
        }
        j = j.wrapping_add(1);
        j;
    }
    while j < (2 as i32 * 100 as i32 - 1 as i32) as u32 {
        x[j as usize] = 0 as i32 as uint32_t;
        j = j.wrapping_add(1);
        j;
    }
    x[1 as i32 as usize] = (x[1 as i32 as usize]).wrapping_add(1);
    x[1 as i32 as usize];
    ss = (seed as u64 & ((1 as u64) << 30 as i32).wrapping_sub(1 as i32 as u64))
        as uint32_t;
    t = (70 as i32 - 1 as i32) as uint32_t;
    while t != 0 {
        j = (100 as i32 - 1 as i32) as uint32_t;
        while j > 0 as i32 as u32 {
            x[j.wrapping_add(j) as usize] = x[j as usize];
            j = j.wrapping_sub(1);
            j;
        }
        j = (2 as i32 * 100 as i32 - 2 as i32) as uint32_t;
        while j > (100 as i32 - 37 as i32) as u32 {
            x[((2 as i32 * 100 as i32 - 1 as i32) as u32).wrapping_sub(j) as usize] = x[j
                as usize] & !(1 as i32) as u32;
            j = (j as u32).wrapping_sub(2 as i32 as u32) as uint32_t as uint32_t;
        }
        j = (2 as i32 * 100 as i32 - 2 as i32) as uint32_t;
        while j >= 100 as i32 as u32 {
            if x[j as usize] & 1 as i32 as u32 != 0 {
                x[j.wrapping_sub((100 as i32 - 37 as i32) as u32) as usize] = ((x[j
                    .wrapping_sub((100 as i32 - 37 as i32) as u32) as usize])
                    .wrapping_sub(x[j as usize]) as u64
                    & ((1 as u64) << 30 as i32).wrapping_sub(1 as i32 as u64))
                    as uint32_t;
                x[j.wrapping_sub(100 as i32 as u32) as usize] = ((x[j
                    .wrapping_sub(100 as i32 as u32) as usize])
                    .wrapping_sub(x[j as usize]) as u64
                    & ((1 as u64) << 30 as i32).wrapping_sub(1 as i32 as u64))
                    as uint32_t;
            }
            j = j.wrapping_sub(1);
            j;
        }
        if ss & 1 as i32 as u32 != 0 {
            j = 100 as i32 as uint32_t;
            while j > 0 as i32 as u32 {
                x[j as usize] = x[j.wrapping_sub(1 as i32 as u32) as usize];
                j = j.wrapping_sub(1);
                j;
            }
            x[0 as i32 as usize] = x[100 as i32 as usize];
            if x[100 as i32 as usize] & 1 as i32 as u32 != 0 {
                x[37 as i32 as usize] = ((x[37 as i32 as usize])
                    .wrapping_sub(x[100 as i32 as usize]) as u64
                    & ((1 as u64) << 30 as i32).wrapping_sub(1 as i32 as u64))
                    as uint32_t;
            }
        }
        if ss != 0 {
            ss >>= 1 as i32;
        } else {
            t = t.wrapping_sub(1);
            t;
        }
    }
    j = 0 as i32 as uint32_t;
    while j < 37 as i32 as u32 {
        (*ctx)
            .x[j.wrapping_add(100 as i32 as u32).wrapping_sub(37 as i32 as u32)
            as usize] = x[j as usize];
        j = j.wrapping_add(1);
        j;
    }
    while j < 100 as i32 as u32 {
        (*ctx).x[j.wrapping_sub(37 as i32 as u32) as usize] = x[j as usize];
        j = j.wrapping_add(1);
        j;
    }
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_knuth_lfib_get(
    mut ctx: *mut knuth_lfib_ctx,
) -> uint32_t {
    let mut value: uint32_t = 0;
    if (*ctx).index < 100 as i32 as u32 {} else {
        __assert_fail(
            b"ctx->index < KK\0" as *const u8 as *const i8,
            b"knuth-lfib.c\0" as *const u8 as *const i8,
            119 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[i8; 56],
            >(b"uint32_t nettle_knuth_lfib_get(struct knuth_lfib_ctx *)\0"))
                .as_ptr(),
        );
    }
    'c_1803: {
        if (*ctx).index < 100 as i32 as u32 {} else {
            __assert_fail(
                b"ctx->index < KK\0" as *const u8 as *const i8,
                b"knuth-lfib.c\0" as *const u8 as *const i8,
                119 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[i8; 56],
                >(b"uint32_t nettle_knuth_lfib_get(struct knuth_lfib_ctx *)\0"))
                    .as_ptr(),
            );
        }
    };
    value = (*ctx).x[(*ctx).index as usize];
    (*ctx).x[(*ctx).index as usize] = ((*ctx).x[(*ctx).index as usize] as u32)
        .wrapping_sub(
            (*ctx)
                .x[((*ctx).index)
                .wrapping_add(100 as i32 as u32)
                .wrapping_sub(37 as i32 as u32)
                .wrapping_rem(100 as i32 as u32) as usize],
        ) as uint32_t as uint32_t;
    (*ctx).x[(*ctx).index as usize] = ((*ctx).x[(*ctx).index as usize] as u64
        & ((1 as u64) << 30 as i32).wrapping_sub(1 as i32 as u64)) as uint32_t;
    (*ctx).index = ((*ctx).index)
        .wrapping_add(1 as i32 as u32)
        .wrapping_rem(100 as i32 as u32);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_knuth_lfib_get_array(
    mut ctx: *mut knuth_lfib_ctx,
    mut n: size_t,
    mut a: *mut uint32_t,
) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64) < n {
        *a.offset(i as isize) = nettle_knuth_lfib_get(ctx);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_knuth_lfib_random(
    mut ctx: *mut knuth_lfib_ctx,
    mut n: size_t,
    mut dst: *mut uint8_t,
) {
    while n >= 3 as i32 as u64 {
        let mut value: uint32_t = nettle_knuth_lfib_get(ctx);
        value ^= value >> 24 as i32;
        *dst.offset(0 as i32 as isize) = (value >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(1 as i32 as isize) = (value >> 8 as i32 & 0xff as i32 as u32)
            as uint8_t;
        *dst.offset(2 as i32 as isize) = (value & 0xff as i32 as u32) as uint8_t;
        n = (n as u64).wrapping_sub(3 as i32 as u64) as size_t as size_t;
        dst = dst.offset(3 as i32 as isize);
    }
    if n != 0 {
        let mut value_0: uint32_t = nettle_knuth_lfib_get(ctx);
        match n {
            1 => {
                let fresh0 = dst;
                dst = dst.offset(1);
                *fresh0 = (value_0 & 0xff as i32 as u32) as uint8_t;
            }
            2 => {
                *dst.offset(0 as i32 as isize) = (value_0 >> 8 as i32
                    & 0xff as i32 as u32) as uint8_t;
                *dst.offset(1 as i32 as isize) = (value_0 & 0xff as i32 as u32)
                    as uint8_t;
            }
            _ => {
                abort();
            }
        }
    }
}
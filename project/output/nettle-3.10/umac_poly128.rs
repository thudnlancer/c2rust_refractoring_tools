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
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
unsafe extern "C" fn poly128_mul(mut k: *const uint32_t, mut y: *mut uint64_t) {
    let mut y0: uint64_t = 0;
    let mut y1: uint64_t = 0;
    let mut y2: uint64_t = 0;
    let mut y3: uint64_t = 0;
    let mut p0: uint64_t = 0;
    let mut p1: uint64_t = 0;
    let mut p2: uint64_t = 0;
    let mut p3: uint64_t = 0;
    let mut m0: uint64_t = 0;
    let mut m1: uint64_t = 0;
    let mut m2: uint64_t = 0;
    y0 = *y.offset(1 as i32 as isize) & 0xffffffff as u64;
    y1 = *y.offset(1 as i32 as isize) >> 32 as i32;
    y2 = *y.offset(0 as i32 as isize) & 0xffffffff as u64;
    y3 = *y.offset(0 as i32 as isize) >> 32 as i32;
    p0 = y0.wrapping_mul(*k.offset(3 as i32 as isize) as u64);
    m0 = y0
        .wrapping_mul(*k.offset(2 as i32 as isize) as u64)
        .wrapping_add(y1.wrapping_mul(*k.offset(3 as i32 as isize) as u64));
    p1 = y0
        .wrapping_mul(*k.offset(1 as i32 as isize) as u64)
        .wrapping_add(y1.wrapping_mul(*k.offset(2 as i32 as isize) as u64))
        .wrapping_add(y2.wrapping_mul(*k.offset(3 as i32 as isize) as u64));
    m1 = y0
        .wrapping_mul(*k.offset(0 as i32 as isize) as u64)
        .wrapping_add(y1.wrapping_mul(*k.offset(1 as i32 as isize) as u64))
        .wrapping_add(y2.wrapping_mul(*k.offset(2 as i32 as isize) as u64))
        .wrapping_add(y3.wrapping_mul(*k.offset(3 as i32 as isize) as u64));
    p2 = y1
        .wrapping_mul(*k.offset(0 as i32 as isize) as u64)
        .wrapping_add(y2.wrapping_mul(*k.offset(1 as i32 as isize) as u64))
        .wrapping_add(y3.wrapping_mul(*k.offset(2 as i32 as isize) as u64));
    m2 = y2
        .wrapping_mul(*k.offset(0 as i32 as isize) as u64)
        .wrapping_add(y3.wrapping_mul(*k.offset(1 as i32 as isize) as u64));
    p3 = y3.wrapping_mul(*k.offset(0 as i32 as isize) as u64);
    m1 = (m1 as u64).wrapping_add((159 as i32 as u64).wrapping_mul(p3 >> 32 as i32))
        as uint64_t as uint64_t;
    p1 = (p1 as u64)
        .wrapping_add(
            (159 as i32 as u64)
                .wrapping_mul((p3 & 0xffffffff as u64).wrapping_add(m2 >> 32 as i32)),
        ) as uint64_t as uint64_t;
    m0 = (m0 as u64)
        .wrapping_add(
            (159 as i32 as u64)
                .wrapping_mul((p2 >> 32 as i32).wrapping_add(m2 & 0xffffffff as u64)),
        ) as uint64_t as uint64_t;
    p0 = (p0 as u64)
        .wrapping_add(
            (159 as i32 as u64)
                .wrapping_mul((p2 & 0xffffffff as u64).wrapping_add(m1 >> 32 as i32)),
        ) as uint64_t as uint64_t;
    p1 = (p1 as u64).wrapping_add(m0 >> 32 as i32) as uint64_t as uint64_t;
    m0 <<= 32 as i32;
    m1 <<= 32 as i32;
    p0 = (p0 as u64).wrapping_add(m0) as uint64_t as uint64_t;
    p1 = (p1 as u64).wrapping_add((p0 < m0) as i32 as u64) as uint64_t as uint64_t;
    p1 = (p1 as u64).wrapping_add(m1) as uint64_t as uint64_t;
    if p1 < m1 {
        p0 = (p0 as u64).wrapping_add(159 as i32 as u64) as uint64_t as uint64_t;
        p1 = (p1 as u64).wrapping_add((p0 < 159 as i32 as u64) as i32 as u64) as uint64_t
            as uint64_t;
    }
    *y.offset(0 as i32 as isize) = p1;
    *y.offset(1 as i32 as isize) = p0;
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_poly128(
    mut k: *const uint32_t,
    mut y: *mut uint64_t,
    mut mh: uint64_t,
    mut ml: uint64_t,
) {
    let mut yh: uint64_t = 0;
    let mut yl: uint64_t = 0;
    let mut cy: uint64_t = 0;
    if mh >> 32 as i32 == 0xffffffff as u32 as u64 {
        poly128_mul(k, y);
        if *y.offset(1 as i32 as isize) > 0 as i32 as u64 {
            let ref mut fresh0 = *y.offset(1 as i32 as isize);
            *fresh0 = (*fresh0).wrapping_sub(1);
            *fresh0;
        } else if *y.offset(0 as i32 as isize) > 0 as i32 as u64 {
            let ref mut fresh1 = *y.offset(0 as i32 as isize);
            *fresh1 = (*fresh1).wrapping_sub(1);
            *fresh1;
            *y.offset(1 as i32 as isize) = !(0 as i32 as uint64_t);
        } else {
            *y.offset(0 as i32 as isize) = !(0 as i32 as uint64_t);
            *y.offset(1 as i32 as isize) = (159 as i32 as uint64_t)
                .wrapping_neg()
                .wrapping_sub(1 as i32 as u64);
        }
        mh = (mh as u64).wrapping_sub((ml < 159 as i32 as u64) as i32 as u64) as uint64_t
            as uint64_t;
        ml = (ml as u64).wrapping_sub(159 as i32 as u64) as uint64_t as uint64_t;
    }
    if mh < !(0 as i32 as uint64_t) || ml < (159 as i32 as uint64_t).wrapping_neg()
    {} else {
        __assert_fail(
            b"mh < UMAC_P128_HI || ml < UMAC_P128_LO\0" as *const u8 as *const i8,
            b"umac-poly128.c\0" as *const u8 as *const i8,
            131 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[i8; 76],
            >(
                b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1061: {
        if mh < !(0 as i32 as uint64_t) || ml < (159 as i32 as uint64_t).wrapping_neg()
        {} else {
            __assert_fail(
                b"mh < UMAC_P128_HI || ml < UMAC_P128_LO\0" as *const u8 as *const i8,
                b"umac-poly128.c\0" as *const u8 as *const i8,
                131 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[i8; 76],
                >(
                    b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    poly128_mul(k, y);
    yl = (*y.offset(1 as i32 as isize)).wrapping_add(ml);
    cy = (yl < ml) as i32 as uint64_t;
    yh = (*y.offset(0 as i32 as isize)).wrapping_add(cy);
    cy = (yh < cy) as i32 as uint64_t;
    yh = (yh as u64).wrapping_add(mh) as uint64_t as uint64_t;
    cy = (cy as u64).wrapping_add((yh < mh) as i32 as u64) as uint64_t as uint64_t;
    if cy <= 1 as i32 as u64 {} else {
        __assert_fail(
            b"cy <= 1\0" as *const u8 as *const i8,
            b"umac-poly128.c\0" as *const u8 as *const i8,
            140 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[i8; 76],
            >(
                b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_584: {
        if cy <= 1 as i32 as u64 {} else {
            __assert_fail(
                b"cy <= 1\0" as *const u8 as *const i8,
                b"umac-poly128.c\0" as *const u8 as *const i8,
                140 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[i8; 76],
                >(
                    b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if cy != 0 {
        yl = (yl as u64).wrapping_add(159 as i32 as u64) as uint64_t as uint64_t;
        yh = (yh as u64).wrapping_add((yl < 159 as i32 as u64) as i32 as u64) as uint64_t
            as uint64_t;
    }
    *y.offset(0 as i32 as isize) = yh;
    *y.offset(1 as i32 as isize) = yl;
}
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
unsafe extern "C" fn poly64_mul(
    mut kh: uint32_t,
    mut kl: uint32_t,
    mut y: uint64_t,
) -> uint64_t {
    let mut yl: uint64_t = 0;
    let mut yh: uint64_t = 0;
    let mut pl: uint64_t = 0;
    let mut ph: uint64_t = 0;
    let mut ml: uint64_t = 0;
    let mut mh: uint64_t = 0;
    yl = y & 0xffffffff as u32 as u64;
    yh = y >> 32 as i32;
    pl = yl.wrapping_mul(kl as u64);
    ph = yh.wrapping_mul(kh as u64);
    ml = yh.wrapping_mul(kl as u64).wrapping_add(yl.wrapping_mul(kh as u64));
    mh = ml >> 32 as i32;
    ml <<= 32 as i32;
    pl = (pl as u64).wrapping_add(ml) as uint64_t as uint64_t;
    ph = (ph as u64).wrapping_add(mh.wrapping_add((pl < ml) as i32 as u64)) as uint64_t
        as uint64_t;
    if ph < (1 as i32 as uint64_t) << 57 as i32 {} else {
        __assert_fail(
            b"ph < ((uint64_t) 1 << 57)\0" as *const u8 as *const i8,
            b"umac-poly64.c\0" as *const u8 as *const i8,
            56 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"uint64_t poly64_mul(uint32_t, uint32_t, uint64_t)\0"))
                .as_ptr(),
        );
    }
    'c_606: {
        if ph < (1 as i32 as uint64_t) << 57 as i32 {} else {
            __assert_fail(
                b"ph < ((uint64_t) 1 << 57)\0" as *const u8 as *const i8,
                b"umac-poly64.c\0" as *const u8 as *const i8,
                56 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"uint64_t poly64_mul(uint32_t, uint32_t, uint64_t)\0"))
                    .as_ptr(),
            );
        }
    };
    ph = (ph as u64).wrapping_mul(59 as i32 as u64) as uint64_t as uint64_t;
    pl = (pl as u64).wrapping_add(ph) as uint64_t as uint64_t;
    if pl < ph {
        pl = (pl as u64).wrapping_add(59 as i32 as u64) as uint64_t as uint64_t;
    }
    return pl;
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_poly64(
    mut kh: uint32_t,
    mut kl: uint32_t,
    mut y: uint64_t,
    mut m: uint64_t,
) -> uint64_t {
    if m >> 32 as i32 == 0xffffffff as u32 as u64 {
        y = poly64_mul(kh, kl, y);
        if y == 0 as i32 as u64 {
            y = (59 as i32 as uint64_t).wrapping_neg().wrapping_sub(1 as i32 as u64);
        } else {
            y = y.wrapping_sub(1);
            y;
        }
        m = (m as u64).wrapping_sub(59 as i32 as u64) as uint64_t as uint64_t;
    }
    y = poly64_mul(kh, kl, y);
    y = (y as u64).wrapping_add(m) as uint64_t as uint64_t;
    if y < m {
        y = (y as u64).wrapping_add(59 as i32 as u64) as uint64_t as uint64_t;
    }
    return y;
}
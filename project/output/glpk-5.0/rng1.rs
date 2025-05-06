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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_rng_next_rand(rand: *mut RNG) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: *mut i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_unif_01(mut rand: *mut RNG) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    x = _glp_rng_next_rand(rand) as libc::c_double / 2147483647.0f64;
    (0.0f64 <= x && x <= 1.0f64
        || {
            glp_assert_(
                b"0.0 <= x && x <= 1.0\0" as *const u8 as *const i8,
                b"misc/rng1.c\0" as *const u8 as *const i8,
                43 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_uniform(
    mut rand: *mut RNG,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    (a < b
        || {
            glp_assert_(
                b"a < b\0" as *const u8 as *const i8,
                b"misc/rng1.c\0" as *const u8 as *const i8,
                64 as i32,
            );
            1 as i32 != 0
        }) as i32;
    x = _glp_rng_unif_01(rand);
    x = a * (1.0f64 - x) + b * x;
    (a <= x && x <= b
        || {
            glp_assert_(
                b"a <= x && x <= b\0" as *const u8 as *const i8,
                b"misc/rng1.c\0" as *const u8 as *const i8,
                67 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return x;
}
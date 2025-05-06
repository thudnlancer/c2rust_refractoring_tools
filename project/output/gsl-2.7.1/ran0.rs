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
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const i8,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> ()>,
    pub get: Option<unsafe extern "C" fn(*mut libc::c_void) -> u64>,
    pub get_double: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ran0_state_t {
    pub x: u64,
}
static mut m: i64 = 2147483647 as i32 as i64;
static mut a: i64 = 16807 as i32 as i64;
static mut q: i64 = 127773 as i32 as i64;
static mut r: i64 = 2836 as i32 as i64;
static mut mask: u64 = 123459876 as i32 as u64;
#[inline]
unsafe extern "C" fn ran0_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran0_state_t = vstate as *mut ran0_state_t;
    let x: u64 = (*state).x;
    let h: i64 = x.wrapping_div(q as u64) as i64;
    let t: i64 = (a as u64)
        .wrapping_mul(x.wrapping_sub((h * q) as u64))
        .wrapping_sub((h * r) as u64) as i64;
    if t < 0 as i32 as i64 {
        (*state).x = (t + m) as u64;
    } else {
        (*state).x = t as u64;
    }
    return (*state).x;
}
unsafe extern "C" fn ran0_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ran0_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn ran0_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran0_state_t = vstate as *mut ran0_state_t;
    if s == mask {
        gsl_error(
            b"ran0 should not use seed == mask\0" as *const u8 as *const i8,
            b"ran0.c\0" as *const u8 as *const i8,
            83 as i32,
            GSL_EINVAL as i32,
        );
        return;
    }
    (*state).x = s ^ mask;
}
static mut ran0_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran0\0" as *const u8 as *const i8,
        max: 2147483646 as i32 as u64,
        min: 1 as i32 as u64,
        size: ::core::mem::size_of::<ran0_state_t>() as u64,
        set: Some(ran0_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ran0_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ran0_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ran0: *const gsl_rng_type = unsafe {
    &ran0_type as *const gsl_rng_type
};
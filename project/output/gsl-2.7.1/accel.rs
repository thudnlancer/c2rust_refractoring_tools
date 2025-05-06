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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct gsl_interp_accel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_accel_alloc() -> *mut gsl_interp_accel {
    let mut a: *mut gsl_interp_accel = malloc(
        ::core::mem::size_of::<gsl_interp_accel>() as u64,
    ) as *mut gsl_interp_accel;
    if a.is_null() {
        gsl_error(
            b"could not allocate space for gsl_interp_accel\0" as *const u8 as *const i8,
            b"accel.c\0" as *const u8 as *const i8,
            33 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_interp_accel;
    }
    (*a).cache = 0 as i32 as size_t;
    (*a).hit_count = 0 as i32 as size_t;
    (*a).miss_count = 0 as i32 as size_t;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_accel_reset(mut a: *mut gsl_interp_accel) -> i32 {
    (*a).cache = 0 as i32 as size_t;
    (*a).hit_count = 0 as i32 as size_t;
    (*a).miss_count = 0 as i32 as size_t;
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_interp_accel_free(mut a: *mut gsl_interp_accel) {
    if a.is_null() {
        return;
    }
    free(a as *mut libc::c_void);
}
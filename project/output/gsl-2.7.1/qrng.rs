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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub struct gsl_qrng_type {
    pub name: *const i8,
    pub max_dimension: u32,
    pub state_size: Option<unsafe extern "C" fn(u32) -> size_t>,
    pub init_state: Option<unsafe extern "C" fn(*mut libc::c_void, u32) -> i32>,
    pub get: Option<
        unsafe extern "C" fn(*mut libc::c_void, u32, *mut libc::c_double) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_qrng {
    pub type_0: *const gsl_qrng_type,
    pub dimension: u32,
    pub state_size: size_t,
    pub state: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_alloc(
    mut T: *const gsl_qrng_type,
    mut dimension: u32,
) -> *mut gsl_qrng {
    let mut q: *mut gsl_qrng = malloc(::core::mem::size_of::<gsl_qrng>() as u64)
        as *mut gsl_qrng;
    if q.is_null() {
        gsl_error(
            b"allocation failed for qrng struct\0" as *const u8 as *const i8,
            b"qrng.c\0" as *const u8 as *const i8,
            19 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_qrng;
    }
    (*q).dimension = dimension;
    (*q).state_size = ((*T).state_size).expect("non-null function pointer")(dimension);
    (*q).state = malloc((*q).state_size);
    if ((*q).state).is_null() {
        free(q as *mut libc::c_void);
        gsl_error(
            b"allocation failed for qrng state\0" as *const u8 as *const i8,
            b"qrng.c\0" as *const u8 as *const i8,
            30 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_qrng;
    }
    (*q).type_0 = T;
    ((*T).init_state).expect("non-null function pointer")((*q).state, (*q).dimension);
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_init(mut q: *mut gsl_qrng) {
    ((*(*q).type_0).init_state)
        .expect("non-null function pointer")((*q).state, (*q).dimension);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_memcpy(
    mut dest: *mut gsl_qrng,
    mut src: *const gsl_qrng,
) -> i32 {
    if (*dest).type_0 != (*src).type_0 {
        gsl_error(
            b"generators must be of the same type\0" as *const u8 as *const i8,
            b"qrng.c\0" as *const u8 as *const i8,
            51 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    (*dest).dimension = (*src).dimension;
    (*dest).state_size = (*src).state_size;
    memcpy((*dest).state, (*src).state, (*src).state_size);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_clone(mut q: *const gsl_qrng) -> *mut gsl_qrng {
    let mut r: *mut gsl_qrng = malloc(::core::mem::size_of::<gsl_qrng>() as u64)
        as *mut gsl_qrng;
    if r.is_null() {
        gsl_error(
            b"failed to allocate space for rng struct\0" as *const u8 as *const i8,
            b"qrng.c\0" as *const u8 as *const i8,
            70 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_qrng;
    }
    (*r).dimension = (*q).dimension;
    (*r).state_size = (*q).state_size;
    (*r).state = malloc((*r).state_size);
    if ((*r).state).is_null() {
        free(r as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for rng state\0" as *const u8 as *const i8,
            b"qrng.c\0" as *const u8 as *const i8,
            81 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_qrng;
    }
    (*r).type_0 = (*q).type_0;
    memcpy((*r).state, (*q).state, (*q).state_size);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_name(mut q: *const gsl_qrng) -> *const i8 {
    return (*(*q).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_size(mut q: *const gsl_qrng) -> size_t {
    return (*q).state_size;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_state(mut q: *const gsl_qrng) -> *mut libc::c_void {
    return (*q).state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_qrng_free(mut q: *mut gsl_qrng) {
    if q.is_null() {
        return;
    }
    if !((*q).state).is_null() {
        free((*q).state);
    }
    free(q as *mut libc::c_void);
}
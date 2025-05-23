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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    static mut gsl_rng_default_seed: u64;
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
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_alloc(mut T: *const gsl_rng_type) -> *mut gsl_rng {
    let mut r: *mut gsl_rng = malloc(::core::mem::size_of::<gsl_rng>() as u64)
        as *mut gsl_rng;
    if r.is_null() {
        gsl_error(
            b"failed to allocate space for rng struct\0" as *const u8 as *const i8,
            b"rng.c\0" as *const u8 as *const i8,
            36 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_rng;
    }
    (*r).state = calloc(1 as i32 as u64, (*T).size);
    if ((*r).state).is_null() {
        free(r as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for rng state\0" as *const u8 as *const i8,
            b"rng.c\0" as *const u8 as *const i8,
            46 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_rng;
    }
    (*r).type_0 = T;
    gsl_rng_set(r, gsl_rng_default_seed);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_memcpy(
    mut dest: *mut gsl_rng,
    mut src: *const gsl_rng,
) -> i32 {
    if (*dest).type_0 != (*src).type_0 {
        gsl_error(
            b"generators must be of the same type\0" as *const u8 as *const i8,
            b"rng.c\0" as *const u8 as *const i8,
            61 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    memcpy((*dest).state, (*src).state, (*(*src).type_0).size);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_clone(mut q: *const gsl_rng) -> *mut gsl_rng {
    let mut r: *mut gsl_rng = malloc(::core::mem::size_of::<gsl_rng>() as u64)
        as *mut gsl_rng;
    if r.is_null() {
        gsl_error(
            b"failed to allocate space for rng struct\0" as *const u8 as *const i8,
            b"rng.c\0" as *const u8 as *const i8,
            77 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_rng;
    }
    (*r).state = malloc((*(*q).type_0).size);
    if ((*r).state).is_null() {
        free(r as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for rng state\0" as *const u8 as *const i8,
            b"rng.c\0" as *const u8 as *const i8,
            87 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_rng;
    }
    (*r).type_0 = (*q).type_0;
    memcpy((*r).state, (*q).state, (*(*q).type_0).size);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_set(mut r: *const gsl_rng, mut seed: u64) {
    ((*(*r).type_0).set).expect("non-null function pointer")((*r).state, seed);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_max(mut r: *const gsl_rng) -> u64 {
    return (*(*r).type_0).max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_min(mut r: *const gsl_rng) -> u64 {
    return (*(*r).type_0).min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_name(mut r: *const gsl_rng) -> *const i8 {
    return (*(*r).type_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_size(mut r: *const gsl_rng) -> size_t {
    return (*(*r).type_0).size;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_state(mut r: *const gsl_rng) -> *mut libc::c_void {
    return (*r).state;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_print_state(mut r: *const gsl_rng) {
    let mut i: size_t = 0;
    let mut p: *mut u8 = (*r).state as *mut u8;
    let n: size_t = (*(*r).type_0).size;
    i = 0 as i32 as size_t;
    while i < n {
        printf(b"%.2x\0" as *const u8 as *const i8, *p.offset(i as isize) as i32);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_free(mut r: *mut gsl_rng) {
    if r.is_null() {
        return;
    }
    free((*r).state);
    free(r as *mut libc::c_void);
}
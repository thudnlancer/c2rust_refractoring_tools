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
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_int(mut r: *const gsl_rng, mut n: u64) -> u64 {
    let mut offset: u64 = (*(*r).type_0).min;
    let mut range: u64 = ((*(*r).type_0).max).wrapping_sub(offset);
    let mut scale: u64 = 0;
    let mut k: u64 = 0;
    if n > range || n == 0 as i32 as u64 {
        gsl_error(
            b"invalid n, either 0 or exceeds maximum value of generator\0" as *const u8
                as *const i8,
            b"../gsl/gsl_rng.h\0" as *const u8 as *const i8,
            200 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as i32 as u64;
    }
    scale = range.wrapping_div(n);
    loop {
        k = (((*(*r).type_0).get).expect("non-null function pointer")((*r).state))
            .wrapping_sub(offset)
            .wrapping_div(scale);
        if !(k >= n) {
            break;
        }
    }
    return k;
}
#[inline]
unsafe extern "C" fn swap(
    mut base: *mut libc::c_void,
    mut size: size_t,
    mut i: size_t,
    mut j: size_t,
) {
    let mut a: *mut i8 = (base as *mut i8).offset(size.wrapping_mul(i) as isize);
    let mut b: *mut i8 = (base as *mut i8).offset(size.wrapping_mul(j) as isize);
    let mut s: size_t = size;
    if i == j {
        return;
    }
    loop {
        let mut tmp: i8 = *a;
        let fresh0 = a;
        a = a.offset(1);
        *fresh0 = *b;
        let fresh1 = b;
        b = b.offset(1);
        *fresh1 = tmp;
        s = s.wrapping_sub(1);
        if !(s > 0 as i32 as u64) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn copy(
    mut dest: *mut libc::c_void,
    mut i: size_t,
    mut src: *mut libc::c_void,
    mut j: size_t,
    mut size: size_t,
) {
    let mut a: *mut i8 = (dest as *mut i8).offset(size.wrapping_mul(i) as isize);
    let mut b: *mut i8 = (src as *mut i8).offset(size.wrapping_mul(j) as isize);
    let mut s: size_t = size;
    loop {
        let fresh2 = b;
        b = b.offset(1);
        let fresh3 = a;
        a = a.offset(1);
        *fresh3 = *fresh2;
        s = s.wrapping_sub(1);
        if !(s > 0 as i32 as u64) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_shuffle(
    mut r: *const gsl_rng,
    mut base: *mut libc::c_void,
    mut n: size_t,
    mut size: size_t,
) {
    let mut i: size_t = 0;
    i = n.wrapping_sub(1 as i32 as u64);
    while i > 0 as i32 as u64 {
        let mut j: size_t = gsl_rng_uniform_int(r, i.wrapping_add(1 as i32 as u64));
        swap(base, size, i, j);
        i = i.wrapping_sub(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_choose(
    mut r: *const gsl_rng,
    mut dest: *mut libc::c_void,
    mut k: size_t,
    mut src: *mut libc::c_void,
    mut n: size_t,
    mut size: size_t,
) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as i32 as size_t;
    if k > n {
        gsl_error(
            b"k is greater than n, cannot sample more than n items\0" as *const u8
                as *const i8,
            b"shuffle.c\0" as *const u8 as *const i8,
            95 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    i = 0 as i32 as size_t;
    while i < n && j < k {
        if n.wrapping_sub(i) as libc::c_double * gsl_rng_uniform(r)
            < k.wrapping_sub(j) as libc::c_double
        {
            copy(dest, j, src, i, size);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_sample(
    mut r: *const gsl_rng,
    mut dest: *mut libc::c_void,
    mut k: size_t,
    mut src: *mut libc::c_void,
    mut n: size_t,
    mut size: size_t,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as i32 as size_t;
    i = 0 as i32 as size_t;
    while i < k {
        j = gsl_rng_uniform_int(r, n);
        copy(dest, i, src, j, size);
        i = i.wrapping_add(1);
        i;
    }
}
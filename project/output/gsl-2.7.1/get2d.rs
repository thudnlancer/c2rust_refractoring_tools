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
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
unsafe extern "C" fn find(
    n: size_t,
    mut range: *const libc::c_double,
    x: libc::c_double,
    mut i: *mut size_t,
) -> i32 {
    let mut i_linear: size_t = 0;
    let mut lower: size_t = 0;
    let mut upper: size_t = 0;
    let mut mid: size_t = 0;
    if x < *range.offset(0 as i32 as isize) {
        return -(1 as i32);
    }
    if x >= *range.offset(n as isize) {
        return 1 as i32;
    }
    let mut u: libc::c_double = (x - *range.offset(0 as i32 as isize))
        / (*range.offset(n as isize) - *range.offset(0 as i32 as isize));
    i_linear = (u * n as libc::c_double) as size_t;
    if x >= *range.offset(i_linear as isize)
        && x < *range.offset(i_linear.wrapping_add(1 as i32 as u64) as isize)
    {
        *i = i_linear;
        return 0 as i32;
    }
    upper = n;
    lower = 0 as i32 as size_t;
    while upper.wrapping_sub(lower) > 1 as i32 as u64 {
        mid = upper.wrapping_add(lower).wrapping_div(2 as i32 as u64);
        if x >= *range.offset(mid as isize) {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    *i = lower;
    if x < *range.offset(lower as isize)
        || x >= *range.offset(lower.wrapping_add(1 as i32 as u64) as isize)
    {
        gsl_error(
            b"x not found in range\0" as *const u8 as *const i8,
            b"./find.c\0" as *const u8 as *const i8,
            81 as i32,
            GSL_ESANITY as i32,
        );
        return GSL_ESANITY as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_get(
    mut h: *const gsl_histogram2d,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    if i >= nx {
        gsl_error(
            b"index i lies outside valid range of 0 .. nx - 1\0" as *const u8
                as *const i8,
            b"get2d.c\0" as *const u8 as *const i8,
            35 as i32,
            GSL_EDOM as i32,
        );
        return 0 as i32 as libc::c_double;
    }
    if j >= ny {
        gsl_error(
            b"index j lies outside valid range of 0 .. ny - 1\0" as *const u8
                as *const i8,
            b"get2d.c\0" as *const u8 as *const i8,
            41 as i32,
            GSL_EDOM as i32,
        );
        return 0 as i32 as libc::c_double;
    }
    return *((*h).bin).offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_get_xrange(
    mut h: *const gsl_histogram2d,
    i: size_t,
    mut xlower: *mut libc::c_double,
    mut xupper: *mut libc::c_double,
) -> i32 {
    let nx: size_t = (*h).nx;
    if i >= nx {
        gsl_error(
            b"index i lies outside valid range of 0 .. nx - 1\0" as *const u8
                as *const i8,
            b"get2d.c\0" as *const u8 as *const i8,
            55 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    }
    *xlower = *((*h).xrange).offset(i as isize);
    *xupper = *((*h).xrange).offset(i.wrapping_add(1 as i32 as u64) as isize);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_get_yrange(
    mut h: *const gsl_histogram2d,
    j: size_t,
    mut ylower: *mut libc::c_double,
    mut yupper: *mut libc::c_double,
) -> i32 {
    let ny: size_t = (*h).ny;
    if j >= ny {
        gsl_error(
            b"index j lies outside valid range of 0 .. ny - 1\0" as *const u8
                as *const i8,
            b"get2d.c\0" as *const u8 as *const i8,
            72 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    }
    *ylower = *((*h).yrange).offset(j as isize);
    *yupper = *((*h).yrange).offset(j.wrapping_add(1 as i32 as u64) as isize);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_find(
    mut h: *const gsl_histogram2d,
    x: libc::c_double,
    y: libc::c_double,
    mut i: *mut size_t,
    mut j: *mut size_t,
) -> i32 {
    let mut status: i32 = find((*h).nx, (*h).xrange as *const libc::c_double, x, i);
    if status != 0 {
        gsl_error(
            b"x not found in range of h\0" as *const u8 as *const i8,
            b"get2d.c\0" as *const u8 as *const i8,
            90 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    }
    status = find((*h).ny, (*h).yrange as *const libc::c_double, y, j);
    if status != 0 {
        gsl_error(
            b"y not found in range of h\0" as *const u8 as *const i8,
            b"get2d.c\0" as *const u8 as *const i8,
            97 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    }
    return GSL_SUCCESS as i32;
}
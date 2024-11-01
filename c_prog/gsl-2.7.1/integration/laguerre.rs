#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma(x: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
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
pub struct gsl_integration_fixed_params {
    pub alpha: libc::c_double,
    pub beta: libc::c_double,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub zemu: libc::c_double,
    pub shft: libc::c_double,
    pub slp: libc::c_double,
    pub al: libc::c_double,
    pub be: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_integration_fixed_type {
    pub check: Option::<
        unsafe extern "C" fn(size_t, *const gsl_integration_fixed_params) -> libc::c_int,
    >,
    pub init: Option::<
        unsafe extern "C" fn(
            size_t,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut gsl_integration_fixed_params,
        ) -> libc::c_int,
    >,
}
unsafe extern "C" fn laguerre_check(
    n: size_t,
    mut params: *const gsl_integration_fixed_params,
) -> libc::c_int {
    if (*params).b <= 0.0f64 {
        gsl_error(
            b"b must be positive\0" as *const u8 as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (*params).alpha <= -1.0f64 {
        gsl_error(
            b"alpha must be > -1\0" as *const u8 as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn laguerre_init(
    n: size_t,
    mut diag: *mut libc::c_double,
    mut subdiag: *mut libc::c_double,
    mut params: *mut gsl_integration_fixed_params,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *diag
            .offset(
                i as isize,
            ) = 2.0f64 * i as libc::c_double + 1.0f64 + (*params).alpha;
        *subdiag
            .offset(
                i as isize,
            ) = sqrt(
            (i as libc::c_double + 1.0f64)
                * ((*params).alpha + i as libc::c_double + 1.0f64),
        );
        i = i.wrapping_add(1);
        i;
    }
    (*params).zemu = gsl_sf_gamma((*params).alpha + 1.0f64);
    (*params).shft = (*params).a;
    (*params).slp = 1.0f64 / (*params).b;
    (*params).al = (*params).alpha;
    (*params).be = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
static mut laguerre_type: gsl_integration_fixed_type = unsafe {
    {
        let mut init = gsl_integration_fixed_type {
            check: Some(
                laguerre_check
                    as unsafe extern "C" fn(
                        size_t,
                        *const gsl_integration_fixed_params,
                    ) -> libc::c_int,
            ),
            init: Some(
                laguerre_init
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut gsl_integration_fixed_params,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_integration_fixed_laguerre: *const gsl_integration_fixed_type = unsafe {
    &laguerre_type as *const gsl_integration_fixed_type
};

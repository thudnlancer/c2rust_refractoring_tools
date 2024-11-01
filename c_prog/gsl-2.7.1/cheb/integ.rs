#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub struct gsl_cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: size_t,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: size_t,
    pub f: *mut libc::c_double,
}
pub type gsl_cheb_series = gsl_cheb_series_struct;
#[no_mangle]
pub unsafe extern "C" fn gsl_cheb_calc_integ(
    mut integ: *mut gsl_cheb_series,
    mut f: *const gsl_cheb_series,
) -> libc::c_int {
    let n: size_t = ((*f).order).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let con: libc::c_double = 0.25f64 * ((*f).b - (*f).a);
    if (*integ).order != (*f).order {
        gsl_error(
            b"order of chebyshev series must be equal\0" as *const u8
                as *const libc::c_char,
            b"integ.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*integ).a = (*f).a;
    (*integ).b = (*f).b;
    if n == 1 as libc::c_int as libc::c_ulong {
        *((*integ).c).offset(0 as libc::c_int as isize) = 0.0f64;
    } else if n == 2 as libc::c_int as libc::c_ulong {
        *((*integ).c)
            .offset(
                1 as libc::c_int as isize,
            ) = con * *((*f).c).offset(0 as libc::c_int as isize);
        *((*integ).c)
            .offset(
                0 as libc::c_int as isize,
            ) = 2.0f64 * *((*integ).c).offset(1 as libc::c_int as isize);
    } else {
        let mut sum: libc::c_double = 0.0f64;
        let mut fac: libc::c_double = 1.0f64;
        let mut i: size_t = 0;
        i = 1 as libc::c_int as size_t;
        while i <= n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            *((*integ).c)
                .offset(
                    i as isize,
                ) = con
                * (*((*f).c)
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    - *((*f).c)
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )) / i as libc::c_double;
            sum += fac * *((*integ).c).offset(i as isize);
            fac = -fac;
            i = i.wrapping_add(1);
            i;
        }
        *((*integ).c)
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = con
            * *((*f).c)
                .offset(n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            / (n as libc::c_double - 1.0f64);
        sum
            += fac
                * *((*integ).c)
                    .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *((*integ).c).offset(0 as libc::c_int as isize) = 2.0f64 * sum;
    }
    return GSL_SUCCESS as libc::c_int;
}

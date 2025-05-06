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
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sf_fact_e(n: u32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_lnfact_e(n: u32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_psi_int_e(n: i32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_bessel_Y1_e(x: libc::c_double, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_bessel_Y0_e(x: libc::c_double, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_bessel_Ynu_asympx_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
    fn gsl_sf_bessel_Ynu_asymp_Olver_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
}
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
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
unsafe extern "C" fn bessel_Yn_small_x(
    n: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let mut k: i32 = 0;
    let mut y: libc::c_double = 0.25f64 * x * x;
    let mut ln_x_2: libc::c_double = log(0.5f64 * x);
    let mut ln_nm1_fact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut k_term: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut sum1: libc::c_double = 0.;
    let mut ln_pre1: libc::c_double = 0.;
    let mut term2: libc::c_double = 0.;
    let mut sum2: libc::c_double = 0.;
    let mut pre2: libc::c_double = 0.;
    gsl_sf_lnfact_e((n - 1 as i32) as u32, &mut ln_nm1_fact);
    ln_pre1 = -n as libc::c_double * ln_x_2 + ln_nm1_fact.val;
    if ln_pre1 > 7.0978271289338397e+02f64 - 3.0f64 {
        gsl_error(
            b"error\0" as *const u8 as *const i8,
            b"bessel_Yn.c\0" as *const u8 as *const i8,
            51 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    }
    sum1 = 1.0f64;
    k_term = 1.0f64;
    k = 1 as i32;
    while k <= n - 1 as i32 {
        k_term *= y / (k * (n - k)) as libc::c_double;
        sum1 += k_term;
        k += 1;
        k;
    }
    term1 = -exp(ln_pre1) * sum1 / 3.14159265358979323846f64;
    pre2 = -exp(n as libc::c_double * ln_x_2) / 3.14159265358979323846f64;
    if fabs(pre2) > 0.0f64 {
        let KMAX: i32 = 20 as i32;
        let mut psi_n: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut npk_fact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut yk: libc::c_double = 1.0f64;
        let mut k_fact: libc::c_double = 1.0f64;
        let mut psi_kp1: libc::c_double = -0.57721566490153286060651209008f64;
        let mut psi_npkp1: libc::c_double = 0.;
        gsl_sf_psi_int_e(n, &mut psi_n);
        gsl_sf_fact_e(n as u32, &mut npk_fact);
        psi_npkp1 = psi_n.val + 1.0f64 / n as libc::c_double;
        sum2 = (psi_kp1 + psi_npkp1 - 2.0f64 * ln_x_2) / npk_fact.val;
        k = 1 as i32;
        while k < KMAX {
            psi_kp1 += 1.0f64 / k as libc::c_double;
            psi_npkp1 += 1.0f64 / (n + k) as libc::c_double;
            k_fact *= k as libc::c_double;
            npk_fact.val *= (n + k) as libc::c_double;
            yk *= -y;
            k_term = yk * (psi_kp1 + psi_npkp1 - 2.0f64 * ln_x_2)
                / (k_fact * npk_fact.val);
            sum2 += k_term;
            k += 1;
            k;
        }
        term2 = pre2 * sum2;
    } else {
        term2 = 0.0f64;
    }
    (*result).val = term1 + term2;
    (*result).err = 2.2204460492503131e-16f64
        * (fabs(ln_pre1) * fabs(term1) + fabs(term2));
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Yn_e(
    mut n: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let mut sign: i32 = 1 as i32;
    if n < 0 as i32 {
        n = -n;
        if n & 1 as i32 != 0 {
            sign = -(1 as i32);
        }
    }
    if n == 0 as i32 {
        let mut status: i32 = gsl_sf_bessel_Y0_e(x, result);
        (*result).val *= sign as libc::c_double;
        return status;
    } else if n == 1 as i32 {
        let mut status_0: i32 = gsl_sf_bessel_Y1_e(x, result);
        (*result).val *= sign as libc::c_double;
        return status_0;
    } else {
        if x <= 0.0f64 {
            (*result).val = ::core::f32::NAN as libc::c_double;
            (*result).err = ::core::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const i8,
                b"bessel_Yn.c\0" as *const u8 as *const i8,
                125 as i32,
                GSL_EDOM as i32,
            );
            return GSL_EDOM as i32;
        }
        if x < 5.0f64 {
            let mut status_1: i32 = bessel_Yn_small_x(n, x, result);
            (*result).val *= sign as libc::c_double;
            return status_1;
        } else if 6.0554544523933429e-06f64 * x > (n * n) as libc::c_double + 1.0f64 {
            let mut status_2: i32 = gsl_sf_bessel_Ynu_asympx_e(
                n as libc::c_double,
                x,
                result,
            );
            (*result).val *= sign as libc::c_double;
            return status_2;
        } else if n > 50 as i32 {
            let mut status_3: i32 = gsl_sf_bessel_Ynu_asymp_Olver_e(
                n as libc::c_double,
                x,
                result,
            );
            (*result).val *= sign as libc::c_double;
            return status_3;
        } else {
            let mut two_over_x: libc::c_double = 2.0f64 / x;
            let mut r_by: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut r_bym: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_1: i32 = gsl_sf_bessel_Y1_e(x, &mut r_by);
            let mut stat_0: i32 = gsl_sf_bessel_Y0_e(x, &mut r_bym);
            let mut bym: libc::c_double = r_bym.val;
            let mut by: libc::c_double = r_by.val;
            let mut byp: libc::c_double = 0.;
            let mut j: i32 = 0;
            j = 1 as i32;
            while j < n {
                byp = j as libc::c_double * two_over_x * by - bym;
                bym = by;
                by = byp;
                j += 1;
                j;
            }
            (*result).val = sign as libc::c_double * by;
            (*result).err = fabs((*result).val)
                * (fabs(r_by.err / r_by.val) + fabs(r_bym.err / r_bym.val));
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_1 != GSL_SUCCESS as i32 {
                stat_1
            } else if stat_0 != GSL_SUCCESS as i32 {
                stat_0
            } else {
                GSL_SUCCESS as i32
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Yn_array(
    nmin: i32,
    nmax: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> i32 {
    if nmin < 0 as i32 || nmax < nmin || x <= 0.0f64 {
        let mut j: i32 = 0;
        j = 0 as i32;
        while j <= nmax - nmin {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        gsl_error(
            b"error\0" as *const u8 as *const i8,
            b"bessel_Yn.c\0" as *const u8 as *const i8,
            176 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        let mut r_Ynm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Yn: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_nm1: i32 = gsl_sf_bessel_Yn_e(nmin, x, &mut r_Ynm1);
        let mut stat_n: i32 = gsl_sf_bessel_Yn_e(nmin + 1 as i32, x, &mut r_Yn);
        let mut Ynp1: libc::c_double = 0.;
        let mut Yn: libc::c_double = r_Yn.val;
        let mut Ynm1: libc::c_double = r_Ynm1.val;
        let mut n: i32 = 0;
        let mut stat: i32 = if stat_nm1 != GSL_SUCCESS as i32 {
            stat_nm1
        } else if stat_n != GSL_SUCCESS as i32 {
            stat_n
        } else {
            GSL_SUCCESS as i32
        };
        if stat == GSL_SUCCESS as i32 {
            n = nmin + 1 as i32;
            while n <= nmax + 1 as i32 {
                *result_array.offset((n - nmin - 1 as i32) as isize) = Ynm1;
                Ynp1 = -Ynm1 + 2.0f64 * n as libc::c_double / x * Yn;
                Ynm1 = Yn;
                Yn = Ynp1;
                n += 1;
                n;
            }
        } else {
            n = nmin;
            while n <= nmax {
                *result_array.offset((n - nmin) as isize) = 0.0f64;
                n += 1;
                n;
            }
        }
        return stat;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Yn(n: i32, x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_Yn_e(n, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_Yn_e(n, x, &result)\0" as *const u8 as *const i8,
            b"bessel_Yn.c\0" as *const u8 as *const i8,
            215 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
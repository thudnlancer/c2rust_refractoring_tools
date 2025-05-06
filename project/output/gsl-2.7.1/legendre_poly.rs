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
    fn acos(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sf_bessel_J0_e(x: libc::c_double, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_bessel_Jn_e(n: i32, x: libc::c_double, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_lnpoch_e(
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
    fn gsl_sf_log_1plusx_e(x: libc::c_double, result: *mut gsl_sf_result) -> i32;
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
unsafe extern "C" fn legendre_Pmm(mut m: i32, mut x: libc::c_double) -> libc::c_double {
    if m == 0 as i32 {
        return 1.0f64
    } else {
        let mut p_mm: libc::c_double = 1.0f64;
        let mut root_factor: libc::c_double = sqrt(1.0f64 - x) * sqrt(1.0f64 + x);
        let mut fact_coeff: libc::c_double = 1.0f64;
        let mut i: i32 = 0;
        i = 1 as i32;
        while i <= m {
            p_mm *= -fact_coeff * root_factor;
            fact_coeff += 2.0f64;
            i += 1;
            i;
        }
        return p_mm;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_P1_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    (*result).val = x;
    (*result).err = 0.0f64;
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_P2_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    (*result).val = 0.5f64 * (3.0f64 * x * x - 1.0f64);
    (*result).err = 2.2204460492503131e-16f64 * (fabs(3.0f64 * x * x) + 1.0f64);
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_P3_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    (*result).val = 0.5f64 * x * (5.0f64 * x * x - 3.0f64);
    (*result).err = 2.2204460492503131e-16f64
        * (fabs((*result).val) + 0.5f64 * fabs(x) * (fabs(5.0f64 * x * x) + 3.0f64));
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Pl_e(
    l: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if l < 0 as i32 || x < -1.0f64 || x > 1.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            108 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if l == 0 as i32 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else if l == 1 as i32 {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else if l == 2 as i32 {
        (*result).val = 0.5f64 * (3.0f64 * x * x - 1.0f64);
        (*result).err = 2.2204460492503131e-16f64 * (fabs(3.0f64 * x * x) + 1.0f64);
        return GSL_SUCCESS as i32;
    } else if x == 1.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else if x == -1.0f64 {
        (*result).val = if l & 1 as i32 != 0 { -1.0f64 } else { 1.0f64 };
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else if l < 100000 as i32 {
        let mut p_ellm2: libc::c_double = 1.0f64;
        let mut p_ellm1: libc::c_double = x;
        let mut p_ell: libc::c_double = p_ellm1;
        let mut e_ellm2: libc::c_double = 2.2204460492503131e-16f64;
        let mut e_ellm1: libc::c_double = fabs(x) * 2.2204460492503131e-16f64;
        let mut e_ell: libc::c_double = e_ellm1;
        let mut ell: i32 = 0;
        ell = 2 as i32;
        while ell <= l {
            p_ell = (x * (2 as i32 * ell - 1 as i32) as libc::c_double * p_ellm1
                - (ell - 1 as i32) as libc::c_double * p_ellm2) / ell as libc::c_double;
            p_ellm2 = p_ellm1;
            p_ellm1 = p_ell;
            e_ell = 0.5f64
                * (fabs(x) * ((2 as i32 * ell) as libc::c_double - 1.0f64) * e_ellm1
                    + (ell as libc::c_double - 1.0f64) * e_ellm2)
                / ell as libc::c_double;
            e_ellm2 = e_ellm1;
            e_ellm1 = e_ell;
            ell += 1;
            ell;
        }
        (*result).val = p_ell;
        (*result).err = e_ell
            + l as libc::c_double * fabs(p_ell) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as i32;
    } else {
        let mut u: libc::c_double = l as libc::c_double + 0.5f64;
        let mut th: libc::c_double = acos(x);
        let mut J0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Jm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_J0: i32 = gsl_sf_bessel_J0_e(u * th, &mut J0);
        let mut stat_Jm1: i32 = gsl_sf_bessel_Jn_e(-(1 as i32), u * th, &mut Jm1);
        let mut pre: libc::c_double = 0.;
        let mut B00: libc::c_double = 0.;
        let mut c1: libc::c_double = 0.;
        if th < 1.2207031250000000e-04f64 {
            B00 = (1.0f64 + th * th / 15.0f64) / 24.0f64;
            pre = 1.0f64 + th * th / 12.0f64;
        } else {
            let mut sin_th: libc::c_double = sqrt(1.0f64 - x * x);
            let mut cot_th: libc::c_double = x / sin_th;
            B00 = 1.0f64 / 8.0f64 * (1.0f64 - th * cot_th) / (th * th);
            pre = sqrt(th / sin_th);
        }
        c1 = th / u * B00;
        (*result).val = pre * (J0.val + c1 * Jm1.val);
        (*result).err = pre * (J0.err + fabs(c1) * Jm1.err);
        (*result).err += 1.4901161193847656e-08f64 * fabs((*result).val);
        return if stat_J0 != GSL_SUCCESS as i32 {
            stat_J0
        } else if stat_Jm1 != GSL_SUCCESS as i32 {
            stat_Jm1
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Pl_array(
    lmax: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> i32 {
    if lmax < 0 as i32 || x < -1.0f64 || x > 1.0f64 {
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if lmax == 0 as i32 {
        *result_array.offset(0 as i32 as isize) = 1.0f64;
        return GSL_SUCCESS as i32;
    } else if lmax == 1 as i32 {
        *result_array.offset(0 as i32 as isize) = 1.0f64;
        *result_array.offset(1 as i32 as isize) = x;
        return GSL_SUCCESS as i32;
    } else {
        let mut p_ellm2: libc::c_double = 1.0f64;
        let mut p_ellm1: libc::c_double = x;
        let mut p_ell: libc::c_double = p_ellm1;
        let mut ell: i32 = 0;
        *result_array.offset(0 as i32 as isize) = 1.0f64;
        *result_array.offset(1 as i32 as isize) = x;
        ell = 2 as i32;
        while ell <= lmax {
            p_ell = (x * (2 as i32 * ell - 1 as i32) as libc::c_double * p_ellm1
                - (ell - 1 as i32) as libc::c_double * p_ellm2) / ell as libc::c_double;
            p_ellm2 = p_ellm1;
            p_ellm1 = p_ell;
            *result_array.offset(ell as isize) = p_ell;
            ell += 1;
            ell;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Pl_deriv_array(
    lmax: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> i32 {
    let mut stat_array: i32 = gsl_sf_legendre_Pl_array(lmax, x, result_array);
    if lmax >= 0 as i32 {
        *result_deriv_array.offset(0 as i32 as isize) = 0.0f64;
    }
    if lmax >= 1 as i32 {
        *result_deriv_array.offset(1 as i32 as isize) = 1.0f64;
    }
    if stat_array == GSL_SUCCESS as i32 {
        let mut ell: i32 = 0;
        if fabs(x - 1.0f64) * (lmax as libc::c_double + 1.0f64)
            * (lmax as libc::c_double + 1.0f64) < 1.4901161193847656e-08f64
        {
            ell = 2 as i32;
            while ell <= lmax {
                let pre: libc::c_double = 0.5f64 * ell as libc::c_double
                    * (ell as libc::c_double + 1.0f64);
                *result_deriv_array.offset(ell as isize) = pre
                    * (1.0f64
                        - 0.25f64 * (1.0f64 - x) * (ell as libc::c_double + 2.0f64)
                            * (ell as libc::c_double - 1.0f64));
                ell += 1;
                ell;
            }
        } else if fabs(x + 1.0f64) * (lmax as libc::c_double + 1.0f64)
            * (lmax as libc::c_double + 1.0f64) < 1.4901161193847656e-08f64
        {
            ell = 2 as i32;
            while ell <= lmax {
                let sgn: libc::c_double = if ell & 1 as i32 != 0 {
                    1.0f64
                } else {
                    -1.0f64
                };
                let pre_0: libc::c_double = sgn * 0.5f64 * ell as libc::c_double
                    * (ell as libc::c_double + 1.0f64);
                *result_deriv_array.offset(ell as isize) = pre_0
                    * (1.0f64
                        - 0.25f64 * (1.0f64 + x) * (ell as libc::c_double + 2.0f64)
                            * (ell as libc::c_double - 1.0f64));
                ell += 1;
                ell;
            }
        } else {
            let diff_a: libc::c_double = 1.0f64 + x;
            let diff_b: libc::c_double = 1.0f64 - x;
            ell = 2 as i32;
            while ell <= lmax {
                *result_deriv_array.offset(ell as isize) = -ell as libc::c_double
                    * (x * *result_array.offset(ell as isize)
                        - *result_array.offset((ell - 1 as i32) as isize))
                    / (diff_a * diff_b);
                ell += 1;
                ell;
            }
        }
        return GSL_SUCCESS as i32;
    } else {
        return stat_array
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Plm_e(
    l: i32,
    m: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let dif: libc::c_double = (l - m) as libc::c_double;
    let sum: libc::c_double = (l + m) as libc::c_double;
    let t_d: libc::c_double = if dif == 0.0f64 {
        0.0f64
    } else {
        0.5f64 * dif * (log(dif) - 1.0f64)
    };
    let t_s: libc::c_double = if dif == 0.0f64 {
        0.0f64
    } else {
        0.5f64 * sum * (log(sum) - 1.0f64)
    };
    let exp_check: libc::c_double = 0.5f64 * log(2.0f64 * l as libc::c_double + 1.0f64)
        + t_d - t_s;
    if m < 0 as i32 || l < m || x < -1.0f64 || x > 1.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            309 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if exp_check < -7.0839641853226408e+02f64 + 10.0f64 {
        (*result).val = ::core::f32::INFINITY as libc::c_double;
        (*result).err = ::core::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            313 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        let err_amp: libc::c_double = 1.0f64
            / (2.2204460492503131e-16f64 + fabs(1.0f64 - fabs(x)));
        let mut p_mm: libc::c_double = legendre_Pmm(m, x);
        let mut p_mmp1: libc::c_double = x * (2 as i32 * m + 1 as i32) as libc::c_double
            * p_mm;
        if l == m {
            (*result).val = p_mm;
            (*result).err = err_amp * 2.0f64 * 2.2204460492503131e-16f64 * fabs(p_mm);
            return GSL_SUCCESS as i32;
        } else if l == m + 1 as i32 {
            (*result).val = p_mmp1;
            (*result).err = err_amp * 2.0f64 * 2.2204460492503131e-16f64 * fabs(p_mmp1);
            return GSL_SUCCESS as i32;
        } else {
            let mut p_ellm2: libc::c_double = p_mm;
            let mut p_ellm1: libc::c_double = p_mmp1;
            let mut p_ell: libc::c_double = 0.0f64;
            let mut ell: i32 = 0;
            ell = m + 2 as i32;
            while ell <= l {
                p_ell = (x * (2 as i32 * ell - 1 as i32) as libc::c_double * p_ellm1
                    - (ell + m - 1 as i32) as libc::c_double * p_ellm2)
                    / (ell - m) as libc::c_double;
                p_ellm2 = p_ellm1;
                p_ellm1 = p_ell;
                ell += 1;
                ell;
            }
            (*result).val = p_ell;
            (*result).err = err_amp * (0.5f64 * (l - m) as libc::c_double + 1.0f64)
                * 2.2204460492503131e-16f64 * fabs(p_ell);
            return GSL_SUCCESS as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_sphPlm_e(
    l: i32,
    mut m: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if m < 0 as i32 || l < m || x < -1.0f64 || x > 1.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            365 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if m == 0 as i32 {
        let mut P: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_P: i32 = gsl_sf_legendre_Pl_e(l, x, &mut P);
        let mut pre: libc::c_double = sqrt(
            (2.0f64 * l as libc::c_double + 1.0f64)
                / (4.0f64 * 3.14159265358979323846f64),
        );
        (*result).val = pre * P.val;
        (*result).err = pre * P.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_P;
    } else if x == 1.0f64 || x == -1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else {
        let mut lncirc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnpoch: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnpre_val: libc::c_double = 0.;
        let mut lnpre_err: libc::c_double = 0.;
        let mut ex_pre: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sr: libc::c_double = 0.;
        let sgn: libc::c_double = if m & 1 as i32 != 0 { -1.0f64 } else { 1.0f64 };
        let y_mmp1_factor: libc::c_double = x
            * sqrt(2.0f64 * m as libc::c_double + 3.0f64);
        let mut y_mm: libc::c_double = 0.;
        let mut y_mm_err: libc::c_double = 0.;
        let mut y_mmp1: libc::c_double = 0.;
        let mut y_mmp1_err: libc::c_double = 0.;
        gsl_sf_log_1plusx_e(-x * x, &mut lncirc);
        gsl_sf_lnpoch_e(m as libc::c_double, 0.5f64, &mut lnpoch);
        lnpre_val = -0.25f64 * 1.14472988584940017414342735135f64
            + 0.5f64 * (lnpoch.val + m as libc::c_double * lncirc.val);
        lnpre_err = 0.25f64 * 1.14472988584940017414342735135f64
            * 2.2204460492503131e-16f64
            + 0.5f64 * (lnpoch.err + fabs(m as libc::c_double) * lncirc.err);
        ex_pre.val = exp(lnpre_val);
        ex_pre.err = 2.0f64 * (sinh(lnpre_err) + 2.2204460492503131e-16f64) * ex_pre.val;
        sr = sqrt(
            (2.0f64 + 1.0f64 / m as libc::c_double)
                / (4.0f64 * 3.14159265358979323846f64),
        );
        y_mm = sgn * sr * ex_pre.val;
        y_mm_err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(y_mm) + sr * ex_pre.err;
        y_mm_err *= 1.0f64 + 1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - x));
        y_mmp1 = y_mmp1_factor * y_mm;
        y_mmp1_err = fabs(y_mmp1_factor) * y_mm_err;
        if l == m {
            (*result).val = y_mm;
            (*result).err = y_mm_err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(y_mm);
            return GSL_SUCCESS as i32;
        } else if l == m + 1 as i32 {
            (*result).val = y_mmp1;
            (*result).err = y_mmp1_err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(y_mmp1);
            return GSL_SUCCESS as i32;
        } else {
            let mut y_ell: libc::c_double = 0.0f64;
            let mut y_ell_err: libc::c_double = 0.0f64;
            let mut ell: i32 = 0;
            ell = m + 2 as i32;
            while ell <= l {
                let rat1: libc::c_double = (ell - m) as libc::c_double
                    / (ell + m) as libc::c_double;
                let rat2: libc::c_double = ((ell - m) as libc::c_double - 1.0f64)
                    / ((ell + m) as libc::c_double - 1.0f64);
                let factor1: libc::c_double = sqrt(
                    rat1 * (2.0f64 * ell as libc::c_double + 1.0f64)
                        * (2.0f64 * ell as libc::c_double - 1.0f64),
                );
                let factor2: libc::c_double = sqrt(
                    rat1 * rat2 * (2.0f64 * ell as libc::c_double + 1.0f64)
                        / (2.0f64 * ell as libc::c_double - 3.0f64),
                );
                y_ell = (x * y_mmp1 * factor1
                    - ((ell + m) as libc::c_double - 1.0f64) * y_mm * factor2)
                    / (ell - m) as libc::c_double;
                y_mm = y_mmp1;
                y_mmp1 = y_ell;
                y_ell_err = 0.5f64
                    * (fabs(x * factor1) * y_mmp1_err
                        + fabs(((ell + m) as libc::c_double - 1.0f64) * factor2)
                            * y_mm_err) / fabs((ell - m) as libc::c_double);
                y_mm_err = y_mmp1_err;
                y_mmp1_err = y_ell_err;
                ell += 1;
                ell;
            }
            (*result).val = y_ell;
            (*result).err = y_ell_err
                + (0.5f64 * (l - m) as libc::c_double + 1.0f64)
                    * 2.2204460492503131e-16f64 * fabs(y_ell);
            return GSL_SUCCESS as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Plm_array(
    lmax: i32,
    m: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> i32 {
    let dif: libc::c_double = (lmax - m) as libc::c_double;
    let sum: libc::c_double = (lmax + m) as libc::c_double;
    let t_d: libc::c_double = if dif == 0.0f64 {
        0.0f64
    } else {
        0.5f64 * dif * (log(dif) - 1.0f64)
    };
    let t_s: libc::c_double = if dif == 0.0f64 {
        0.0f64
    } else {
        0.5f64 * sum * (log(sum) - 1.0f64)
    };
    let exp_check: libc::c_double = 0.5f64
        * log(2.0f64 * lmax as libc::c_double + 1.0f64) + t_d - t_s;
    if m < 0 as i32 || lmax < m || x < -1.0f64 || x > 1.0f64 {
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            470 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if m > 0 as i32 && (x == 1.0f64 || x == -1.0f64) {
        let mut ell: i32 = 0;
        ell = m;
        while ell <= lmax {
            *result_array.offset((ell - m) as isize) = 0.0f64;
            ell += 1;
            ell;
        }
        return GSL_SUCCESS as i32;
    } else if exp_check < -7.0839641853226408e+02f64 + 10.0f64 {
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            479 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        let mut p_mm: libc::c_double = legendre_Pmm(m, x);
        let mut p_mmp1: libc::c_double = x * (2.0f64 * m as libc::c_double + 1.0f64)
            * p_mm;
        if lmax == m {
            *result_array.offset(0 as i32 as isize) = p_mm;
            return GSL_SUCCESS as i32;
        } else if lmax == m + 1 as i32 {
            *result_array.offset(0 as i32 as isize) = p_mm;
            *result_array.offset(1 as i32 as isize) = p_mmp1;
            return GSL_SUCCESS as i32;
        } else {
            let mut p_ellm2: libc::c_double = p_mm;
            let mut p_ellm1: libc::c_double = p_mmp1;
            let mut p_ell: libc::c_double = 0.0f64;
            let mut ell_0: i32 = 0;
            *result_array.offset(0 as i32 as isize) = p_mm;
            *result_array.offset(1 as i32 as isize) = p_mmp1;
            ell_0 = m + 2 as i32;
            while ell_0 <= lmax {
                p_ell = (x * (2.0f64 * ell_0 as libc::c_double - 1.0f64) * p_ellm1
                    - (ell_0 + m - 1 as i32) as libc::c_double * p_ellm2)
                    / (ell_0 - m) as libc::c_double;
                p_ellm2 = p_ellm1;
                p_ellm1 = p_ell;
                *result_array.offset((ell_0 - m) as isize) = p_ell;
                ell_0 += 1;
                ell_0;
            }
            return GSL_SUCCESS as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Plm_deriv_array(
    lmax: i32,
    m: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> i32 {
    if m < 0 as i32 || m > lmax {
        gsl_error(
            b"m < 0 or m > lmax\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            523 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if m == 0 as i32 {
        return gsl_sf_legendre_Pl_deriv_array(lmax, x, result_array, result_deriv_array)
    } else {
        let mut stat_array: i32 = gsl_sf_legendre_Plm_array(lmax, m, x, result_array);
        if stat_array == GSL_SUCCESS as i32 {
            let mut ell: i32 = 0;
            if m == 1 as i32 && 1.0f64 - fabs(x) < 2.2204460492503131e-16f64 {
                gsl_error(
                    b"divergence near |x| = 1.0 since m = 1\0" as *const u8 as *const i8,
                    b"legendre_poly.c\0" as *const u8 as *const i8,
                    545 as i32,
                    GSL_EOVRFLW as i32,
                );
                return GSL_EOVRFLW as i32;
            } else if m == 2 as i32 && 1.0f64 - fabs(x) < 2.2204460492503131e-16f64 {
                if fabs(x - 1.0f64) < 2.2204460492503131e-16f64 {
                    ell = m;
                    while ell <= lmax {
                        *result_deriv_array.offset((ell - m) as isize) = -0.25f64 * x
                            * (ell as libc::c_double - 1.0f64) * ell as libc::c_double
                            * (ell as libc::c_double + 1.0f64)
                            * (ell as libc::c_double + 2.0f64);
                        ell += 1;
                        ell;
                    }
                } else if fabs(x + 1.0f64) < 2.2204460492503131e-16f64 {
                    ell = m;
                    while ell <= lmax {
                        let sgn: libc::c_double = if ell & 1 as i32 != 0 {
                            1.0f64
                        } else {
                            -1.0f64
                        };
                        *result_deriv_array.offset((ell - m) as isize) = -0.25f64 * sgn
                            * x * (ell as libc::c_double - 1.0f64)
                            * ell as libc::c_double * (ell as libc::c_double + 1.0f64)
                            * (ell as libc::c_double + 2.0f64);
                        ell += 1;
                        ell;
                    }
                }
                return GSL_SUCCESS as i32;
            } else if 1.0f64 - fabs(x) < 2.2204460492503131e-16f64 {
                ell = m;
                while ell <= lmax {
                    *result_deriv_array.offset((ell - m) as isize) = 0.0f64;
                    ell += 1;
                    ell;
                }
                return GSL_SUCCESS as i32;
            } else {
                let diff_a: libc::c_double = 1.0f64 + x;
                let diff_b: libc::c_double = 1.0f64 - x;
                *result_deriv_array.offset(0 as i32 as isize) = -m as libc::c_double * x
                    / (diff_a * diff_b) * *result_array.offset(0 as i32 as isize);
                if lmax - m >= 1 as i32 {
                    *result_deriv_array.offset(1 as i32 as isize) = (2.0f64
                        * m as libc::c_double + 1.0f64)
                        * (x * *result_deriv_array.offset(0 as i32 as isize)
                            + *result_array.offset(0 as i32 as isize));
                }
                ell = m + 2 as i32;
                while ell <= lmax {
                    *result_deriv_array.offset((ell - m) as isize) = -(ell
                        as libc::c_double * x * *result_array.offset((ell - m) as isize)
                        - (ell + m) as libc::c_double
                            * *result_array.offset((ell - 1 as i32 - m) as isize))
                        / (diff_a * diff_b);
                    ell += 1;
                    ell;
                }
                return GSL_SUCCESS as i32;
            }
        } else {
            return stat_array
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_sphPlm_array(
    lmax: i32,
    mut m: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> i32 {
    if m < 0 as i32 || lmax < m || x < -1.0f64 || x > 1.0f64 {
        gsl_error(
            b"error\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            599 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if m > 0 as i32 && (x == 1.0f64 || x == -1.0f64) {
        let mut ell: i32 = 0;
        ell = m;
        while ell <= lmax {
            *result_array.offset((ell - m) as isize) = 0.0f64;
            ell += 1;
            ell;
        }
        return GSL_SUCCESS as i32;
    } else {
        let mut y_mm: libc::c_double = 0.;
        let mut y_mmp1: libc::c_double = 0.;
        if m == 0 as i32 {
            y_mm = 0.5f64 / 1.77245385090551602729816748334f64;
            y_mmp1 = x * 1.73205080756887729352744634151f64 * y_mm;
        } else {
            let mut lncirc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lnpoch: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lnpre: libc::c_double = 0.;
            let sgn: libc::c_double = if m & 1 as i32 != 0 { -1.0f64 } else { 1.0f64 };
            gsl_sf_log_1plusx_e(-x * x, &mut lncirc);
            gsl_sf_lnpoch_e(m as libc::c_double, 0.5f64, &mut lnpoch);
            lnpre = -0.25f64 * 1.14472988584940017414342735135f64
                + 0.5f64 * (lnpoch.val + m as libc::c_double * lncirc.val);
            y_mm = sqrt(
                (2.0f64 + 1.0f64 / m as libc::c_double)
                    / (4.0f64 * 3.14159265358979323846f64),
            ) * sgn * exp(lnpre);
            y_mmp1 = x * sqrt(2.0f64 * m as libc::c_double + 3.0f64) * y_mm;
        }
        if lmax == m {
            *result_array.offset(0 as i32 as isize) = y_mm;
            return GSL_SUCCESS as i32;
        } else if lmax == m + 1 as i32 {
            *result_array.offset(0 as i32 as isize) = y_mm;
            *result_array.offset(1 as i32 as isize) = y_mmp1;
            return GSL_SUCCESS as i32;
        } else {
            let mut y_ell: libc::c_double = 0.;
            let mut ell_0: i32 = 0;
            *result_array.offset(0 as i32 as isize) = y_mm;
            *result_array.offset(1 as i32 as isize) = y_mmp1;
            ell_0 = m + 2 as i32;
            while ell_0 <= lmax {
                let rat1: libc::c_double = (ell_0 - m) as libc::c_double
                    / (ell_0 + m) as libc::c_double;
                let rat2: libc::c_double = ((ell_0 - m) as libc::c_double - 1.0f64)
                    / ((ell_0 + m) as libc::c_double - 1.0f64);
                let factor1: libc::c_double = sqrt(
                    rat1 * (2 as i32 * ell_0 + 1 as i32) as libc::c_double
                        * (2 as i32 * ell_0 - 1 as i32) as libc::c_double,
                );
                let factor2: libc::c_double = sqrt(
                    rat1 * rat2 * (2 as i32 * ell_0 + 1 as i32) as libc::c_double
                        / (2 as i32 * ell_0 - 3 as i32) as libc::c_double,
                );
                y_ell = (x * y_mmp1 * factor1
                    - (ell_0 + m - 1 as i32) as libc::c_double * y_mm * factor2)
                    / (ell_0 - m) as libc::c_double;
                y_mm = y_mmp1;
                y_mmp1 = y_ell;
                *result_array.offset((ell_0 - m) as isize) = y_ell;
                ell_0 += 1;
                ell_0;
            }
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_sphPlm_deriv_array(
    lmax: i32,
    m: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> i32 {
    if m < 0 as i32 || lmax < m || x < -1.0f64 || x > 1.0f64 {
        gsl_error(
            b"domain\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            670 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if m == 0 as i32 {
        let stat_array: i32 = gsl_sf_legendre_Pl_deriv_array(
            lmax,
            x,
            result_array,
            result_deriv_array,
        );
        let mut ell: i32 = 0;
        ell = 0 as i32;
        while ell <= lmax {
            let prefactor: libc::c_double = sqrt(
                (2.0f64 * ell as libc::c_double + 1.0f64)
                    / (4.0f64 * 3.14159265358979323846f64),
            );
            *result_array.offset(ell as isize) *= prefactor;
            *result_deriv_array.offset(ell as isize) *= prefactor;
            ell += 1;
            ell;
        }
        return stat_array;
    } else if m == 1 as i32 {
        let stat_array_0: i32 = gsl_sf_legendre_Plm_deriv_array(
            lmax,
            m,
            x,
            result_array,
            result_deriv_array,
        );
        let mut ell_0: i32 = 0;
        ell_0 = 1 as i32;
        while ell_0 <= lmax {
            let prefactor_0: libc::c_double = sqrt(
                (2.0f64 * ell_0 as libc::c_double + 1.0f64)
                    / (ell_0 as libc::c_double + 1.0f64)
                    / (4.0f64 * 3.14159265358979323846f64 * ell_0 as libc::c_double),
            );
            *result_array.offset((ell_0 - 1 as i32) as isize) *= prefactor_0;
            *result_deriv_array.offset((ell_0 - 1 as i32) as isize) *= prefactor_0;
            ell_0 += 1;
            ell_0;
        }
        return stat_array_0;
    } else {
        let mut stat_array_1: i32 = gsl_sf_legendre_sphPlm_array(
            lmax,
            m,
            x,
            result_array,
        );
        if stat_array_1 == GSL_SUCCESS as i32 {
            let mut ell_1: i32 = 0;
            if 1.0f64 - fabs(x) < 2.2204460492503131e-16f64 {
                ell_1 = m;
                while ell_1 <= lmax {
                    *result_deriv_array.offset((ell_1 - m) as isize) = 0.0f64;
                    ell_1 += 1;
                    ell_1;
                }
                return GSL_SUCCESS as i32;
            } else {
                let diff_a: libc::c_double = 1.0f64 + x;
                let diff_b: libc::c_double = 1.0f64 - x;
                *result_deriv_array.offset(0 as i32 as isize) = -m as libc::c_double * x
                    / (diff_a * diff_b) * *result_array.offset(0 as i32 as isize);
                if lmax - m >= 1 as i32 {
                    *result_deriv_array.offset(1 as i32 as isize) = sqrt(
                        2.0f64 * m as libc::c_double + 3.0f64,
                    )
                        * (x * *result_deriv_array.offset(0 as i32 as isize)
                            + *result_array.offset(0 as i32 as isize));
                }
                ell_1 = m + 2 as i32;
                while ell_1 <= lmax {
                    let c1: libc::c_double = sqrt(
                        (2.0f64 * ell_1 as libc::c_double + 1.0f64)
                            / (2.0f64 * ell_1 as libc::c_double - 1.0f64)
                            * ((ell_1 - m) as libc::c_double
                                / (ell_1 + m) as libc::c_double),
                    );
                    *result_deriv_array.offset((ell_1 - m) as isize) = -(ell_1
                        as libc::c_double * x
                        * *result_array.offset((ell_1 - m) as isize)
                        - c1 * (ell_1 + m) as libc::c_double
                            * *result_array.offset((ell_1 - 1 as i32 - m) as isize))
                        / (diff_a * diff_b);
                    ell_1 += 1;
                    ell_1;
                }
                return GSL_SUCCESS as i32;
            }
        } else {
            return stat_array_1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_array_size(lmax: i32, m: i32) -> i32 {
    return lmax - m + 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_P1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_legendre_P1_e(x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_legendre_P1_e(x, &result)\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            751 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_P2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_legendre_P2_e(x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_legendre_P2_e(x, &result)\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            756 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_P3(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_legendre_P3_e(x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_legendre_P3_e(x, &result)\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            761 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Pl(
    l: i32,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_legendre_Pl_e(l, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_legendre_Pl_e(l, x, &result)\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            766 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_Plm(
    l: i32,
    m: i32,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_legendre_Plm_e(l, m, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_legendre_Plm_e(l, m, x, &result)\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            771 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_sphPlm(
    l: i32,
    m: i32,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_legendre_sphPlm_e(l, m, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_legendre_sphPlm_e(l, m, x, &result)\0" as *const u8 as *const i8,
            b"legendre_poly.c\0" as *const u8 as *const i8,
            776 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
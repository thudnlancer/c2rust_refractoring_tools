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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
    fn gsl_sf_lnfact_e(n: u32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_pow_int(x: libc::c_double, n: i32) -> libc::c_double;
    fn gsl_sf_laguerre_n_e(
        n: i32,
        a: libc::c_double,
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
unsafe extern "C" fn R_norm(
    n: i32,
    l: i32,
    Z: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let mut A: libc::c_double = 2.0f64 * Z / n as libc::c_double;
    let mut pre: libc::c_double = sqrt(A * A * A / (2.0f64 * n as libc::c_double));
    let mut ln_a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ln_b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ex: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_a: i32 = gsl_sf_lnfact_e((n + l) as u32, &mut ln_a);
    let mut stat_b: i32 = gsl_sf_lnfact_e((n - l - 1 as i32) as u32, &mut ln_b);
    let mut diff_val: libc::c_double = 0.5f64 * (ln_b.val - ln_a.val);
    let mut diff_err: libc::c_double = 0.5f64 * (ln_b.err + ln_a.err)
        + 2.2204460492503131e-16f64 * fabs(diff_val);
    let mut stat_e: i32 = gsl_sf_exp_err_e(diff_val, diff_err, &mut ex);
    (*result).val = pre * ex.val;
    (*result).err = pre * ex.err;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_e != GSL_SUCCESS as i32 {
        stat_e
    } else if stat_a != GSL_SUCCESS as i32 {
        stat_a
    } else if stat_b != GSL_SUCCESS as i32 {
        stat_b
    } else {
        GSL_SUCCESS as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_hydrogenicR_1_e(
    Z: libc::c_double,
    r: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if Z > 0.0f64 && r >= 0.0f64 {
        let mut A: libc::c_double = 2.0f64 * Z;
        let mut norm: libc::c_double = A * sqrt(Z);
        let mut ea: libc::c_double = exp(-Z * r);
        (*result).val = norm * ea;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
            * fabs(Z * r);
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const i8,
                b"coulomb_bound.c\0" as *const u8 as *const i8,
                66 as i32,
                GSL_EUNDRFLW as i32,
            );
            return GSL_EUNDRFLW as i32;
        }
        return GSL_SUCCESS as i32;
    } else {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"coulomb_bound.c\0" as *const u8 as *const i8,
            70 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_hydrogenicR_e(
    n: i32,
    l: i32,
    Z: libc::c_double,
    r: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if n < 1 as i32 || l > n - 1 as i32 || Z <= 0.0f64 || r < 0.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"coulomb_bound.c\0" as *const u8 as *const i8,
            81 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        let mut A: libc::c_double = 2.0f64 * Z / n as libc::c_double;
        let mut norm: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_norm: i32 = R_norm(n, l, Z, &mut norm);
        let mut rho: libc::c_double = A * r;
        let mut ea: libc::c_double = exp(-0.5f64 * rho);
        let mut pp: libc::c_double = gsl_sf_pow_int(rho, l);
        let mut lag: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_lag: i32 = gsl_sf_laguerre_n_e(
            n - l - 1 as i32,
            (2 as i32 * l + 1 as i32) as libc::c_double,
            rho,
            &mut lag,
        );
        let mut W_val: libc::c_double = norm.val * ea * pp;
        let mut W_err: libc::c_double = norm.err * ea * pp;
        W_err
            += norm.val * ((0.5f64 * rho + 1.0f64) * 2.2204460492503131e-16f64) * ea
                * pp;
        W_err
            += norm.val * ea
                * ((l as libc::c_double + 1.0f64) * 2.2204460492503131e-16f64) * pp;
        (*result).val = W_val * lag.val;
        (*result).err = W_val * lag.err + W_err * fabs(lag.val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if (l == 0 as i32 || r > 0 as i32 as libc::c_double && l > 0 as i32)
            && lag.val != 0.0f64 && stat_lag == GSL_SUCCESS as i32
            && stat_norm == GSL_SUCCESS as i32
        {
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const i8,
                    b"coulomb_bound.c\0" as *const u8 as *const i8,
                    101 as i32,
                    GSL_EUNDRFLW as i32,
                );
                return GSL_EUNDRFLW as i32;
            }
        }
        return if stat_lag != GSL_SUCCESS as i32 {
            stat_lag
        } else if stat_norm != GSL_SUCCESS as i32 {
            stat_norm
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_hydrogenicR_1(
    Z: libc::c_double,
    r: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_hydrogenicR_1_e(Z, r, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_hydrogenicR_1_e(Z, r, &result)\0" as *const u8 as *const i8,
            b"coulomb_bound.c\0" as *const u8 as *const i8,
            113 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_hydrogenicR(
    n: i32,
    l: i32,
    Z: libc::c_double,
    r: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_hydrogenicR_e(n, l, Z, r, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_hydrogenicR_e(n, l, Z, r, &result)\0" as *const u8 as *const i8,
            b"coulomb_bound.c\0" as *const u8 as *const i8,
            119 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
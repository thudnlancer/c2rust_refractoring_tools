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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
    fn gsl_sf_bessel_IJ_taylor_e(
        nu: libc::c_double,
        x: libc::c_double,
        sign: i32,
        kmax: i32,
        threshold: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
    fn gsl_sf_bessel_Inu_scaled_asymp_unif_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> i32;
    fn gsl_sf_bessel_I_CF1_ser(
        nu: libc::c_double,
        x: libc::c_double,
        ratio: *mut libc::c_double,
    ) -> i32;
    fn gsl_sf_bessel_K_scaled_steed_temme_CF2(
        nu: libc::c_double,
        x: libc::c_double,
        K_nu: *mut libc::c_double,
        K_nup1: *mut libc::c_double,
        Kp_nu: *mut libc::c_double,
    ) -> i32;
    fn gsl_sf_bessel_K_scaled_temme(
        nu: libc::c_double,
        x: libc::c_double,
        K_nu: *mut libc::c_double,
        K_nup1: *mut libc::c_double,
        Kp_nu: *mut libc::c_double,
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
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Inu_scaled_e(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if x < 0.0f64 || nu < 0.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"bessel_Inu.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if x * x < 10.0f64 * (nu + 1.0f64) {
        let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ex: libc::c_double = exp(-x);
        let mut stat: i32 = gsl_sf_bessel_IJ_taylor_e(
            nu,
            x,
            1 as i32,
            100 as i32,
            2.2204460492503131e-16f64,
            &mut b,
        );
        (*result).val = b.val * ex;
        (*result).err = b.err * ex;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat;
    } else if 0.5f64 / (nu * nu + x * x) < 6.0554544523933429e-06f64 {
        return gsl_sf_bessel_Inu_scaled_asymp_unif_e(nu, x, result)
    } else {
        let mut N: i32 = (nu + 0.5f64) as i32;
        let mut mu: libc::c_double = nu - N as libc::c_double;
        let mut K_mu: libc::c_double = 0.;
        let mut K_mup1: libc::c_double = 0.;
        let mut Kp_mu: libc::c_double = 0.;
        let mut K_nu: libc::c_double = 0.;
        let mut K_nup1: libc::c_double = 0.;
        let mut K_num1: libc::c_double = 0.;
        let mut I_nu_ratio: libc::c_double = 0.;
        let mut stat_Irat: i32 = 0;
        let mut stat_Kmu: i32 = 0;
        let mut n: i32 = 0;
        if x < 2.0f64 {
            stat_Kmu = gsl_sf_bessel_K_scaled_temme(
                mu,
                x,
                &mut K_mu,
                &mut K_mup1,
                &mut Kp_mu,
            );
        } else {
            stat_Kmu = gsl_sf_bessel_K_scaled_steed_temme_CF2(
                mu,
                x,
                &mut K_mu,
                &mut K_mup1,
                &mut Kp_mu,
            );
        }
        K_nu = K_mu;
        K_nup1 = K_mup1;
        n = 0 as i32;
        while n < N {
            K_num1 = K_nu;
            K_nu = K_nup1;
            K_nup1 = 2.0f64 * (mu + n as libc::c_double + 1 as i32 as libc::c_double) / x
                * K_nu + K_num1;
            n += 1;
            n;
        }
        stat_Irat = gsl_sf_bessel_I_CF1_ser(nu, x, &mut I_nu_ratio);
        (*result).val = 1.0f64 / (x * (K_nup1 + I_nu_ratio * K_nu));
        (*result).err = 2.2204460492503131e-16f64
            * (0.5f64 * N as libc::c_double + 2.0f64) * fabs((*result).val);
        return if stat_Kmu != GSL_SUCCESS as i32 {
            stat_Kmu
        } else if stat_Irat != GSL_SUCCESS as i32 {
            stat_Irat
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Inu_e(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_I: i32 = gsl_sf_bessel_Inu_scaled_e(nu, x, &mut b);
    let mut stat_e: i32 = gsl_sf_exp_mult_err_e(
        x,
        fabs(x * 2.2204460492503131e-16f64),
        b.val,
        b.err,
        result,
    );
    return if stat_e != GSL_SUCCESS as i32 {
        stat_e
    } else if stat_I != GSL_SUCCESS as i32 {
        stat_I
    } else {
        GSL_SUCCESS as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Inu_scaled(
    mut nu: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_Inu_scaled_e(nu, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_Inu_scaled_e(nu, x, &result)\0" as *const u8 as *const i8,
            b"bessel_Inu.c\0" as *const u8 as *const i8,
            115 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Inu(
    mut nu: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_Inu_e(nu, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_Inu_e(nu, x, &result)\0" as *const u8 as *const i8,
            b"bessel_Inu.c\0" as *const u8 as *const i8,
            121 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
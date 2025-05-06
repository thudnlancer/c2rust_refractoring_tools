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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sf_pow_int(x: libc::c_double, n: i32) -> libc::c_double;
    fn gsl_sf_doublefact_e(n: u32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_sin_e(x: libc::c_double, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_cos_e(x: libc::c_double, result: *mut gsl_sf_result) -> i32;
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
unsafe extern "C" fn bessel_yl_small_x(
    mut l: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let mut num_fact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut den: libc::c_double = gsl_sf_pow_int(x, l + 1 as i32);
    let mut stat_df: i32 = gsl_sf_doublefact_e(
        (2 as i32 * l - 1 as i32) as u32,
        &mut num_fact,
    );
    if stat_df != GSL_SUCCESS as i32 || den == 0.0f64 {
        (*result).val = ::core::f32::INFINITY as libc::c_double;
        (*result).err = ::core::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            49 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        let lmax: i32 = 200 as i32;
        let mut t: libc::c_double = -0.5f64 * x * x;
        let mut sum: libc::c_double = 1.0f64;
        let mut t_coeff: libc::c_double = 1.0f64;
        let mut t_power: libc::c_double = 1.0f64;
        let mut delta: libc::c_double = 0.;
        let mut i: i32 = 0;
        i = 1 as i32;
        while i <= lmax {
            t_coeff /= (i * (2 as i32 * (i - l) - 1 as i32)) as libc::c_double;
            t_power *= t;
            delta = t_power * t_coeff;
            sum += delta;
            if fabs(delta / sum) < 0.5f64 * 2.2204460492503131e-16f64 {
                break;
            }
            i += 1;
            i;
        }
        (*result).val = -num_fact.val / den * sum;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_y0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if x <= 0.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            82 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if 1.0f64 / 1.7976931348623157e+308f64 > 0.0f64
        && x < 1.0f64 / 1.7976931348623157e+308f64
    {
        (*result).val = ::core::f32::INFINITY as libc::c_double;
        (*result).err = ::core::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            85 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat: i32 = gsl_sf_cos_e(x, &mut cos_result);
        (*result).val = -cos_result.val / x;
        (*result).err = fabs(cos_result.err / x);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_y1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if x <= 0.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            103 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if x < 1.0f64 / 1.3407807929942596e+154f64 {
        (*result).val = ::core::f32::INFINITY as libc::c_double;
        (*result).err = ::core::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            106 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else if x < 0.25f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = 1.0f64 / 2.0f64;
        let c2: libc::c_double = -1.0f64 / 8.0f64;
        let c3: libc::c_double = 1.0f64 / 144.0f64;
        let c4: libc::c_double = -1.0f64 / 5760.0f64;
        let c5: libc::c_double = 1.0f64 / 403200.0f64;
        let c6: libc::c_double = -1.0f64 / 43545600.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * c6)))));
        (*result).val = -sum / y;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as i32;
    } else {
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_cos: i32 = gsl_sf_cos_e(x, &mut cos_result);
        let stat_sin: i32 = gsl_sf_sin_e(x, &mut sin_result);
        let cx: libc::c_double = cos_result.val;
        let sx: libc::c_double = sin_result.val;
        (*result).val = -(cx / x + sx) / x;
        (*result).err = (fabs(cos_result.err / x) + sin_result.err) / fabs(x);
        (*result).err += 2.2204460492503131e-16f64 * (fabs(sx / x) + fabs(cx / (x * x)));
        return if stat_cos != GSL_SUCCESS as i32 {
            stat_cos
        } else if stat_sin != GSL_SUCCESS as i32 {
            stat_sin
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_y2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if x <= 0.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            141 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if x < 1.0f64 / 5.6438030941222897e+102f64 {
        (*result).val = ::core::f32::INFINITY as libc::c_double;
        (*result).err = ::core::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            144 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else if x < 0.5f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = 1.0f64 / 6.0f64;
        let c2: libc::c_double = 1.0f64 / 24.0f64;
        let c3: libc::c_double = -1.0f64 / 144.0f64;
        let c4: libc::c_double = 1.0f64 / 3456.0f64;
        let c5: libc::c_double = -1.0f64 / 172800.0f64;
        let c6: libc::c_double = 1.0f64 / 14515200.0f64;
        let c7: libc::c_double = -1.0f64 / 1828915200.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * c7))))));
        (*result).val = -3.0f64 / (x * x * x) * sum;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as i32;
    } else {
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_cos: i32 = gsl_sf_cos_e(x, &mut cos_result);
        let stat_sin: i32 = gsl_sf_sin_e(x, &mut sin_result);
        let sx: libc::c_double = sin_result.val;
        let cx: libc::c_double = cos_result.val;
        let a: libc::c_double = 3.0f64 / (x * x);
        (*result).val = (1.0f64 - a) / x * cx - a * sx;
        (*result).err = cos_result.err * fabs((1.0f64 - a) / x)
            + sin_result.err * fabs(a);
        (*result).err += 2.2204460492503131e-16f64 * (fabs(cx / x) + fabs(sx / (x * x)));
        return if stat_cos != GSL_SUCCESS as i32 {
            stat_cos
        } else if stat_sin != GSL_SUCCESS as i32 {
            stat_sin
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_yl_e(
    mut l: i32,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if l < 0 as i32 || x <= 0.0f64 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if l == 0 as i32 {
        return gsl_sf_bessel_y0_e(x, result)
    } else if l == 1 as i32 {
        return gsl_sf_bessel_y1_e(x, result)
    } else if l == 2 as i32 {
        return gsl_sf_bessel_y2_e(x, result)
    } else if x < 3.0f64 {
        return bessel_yl_small_x(l, x, result)
    } else if 6.0554544523933429e-06f64 * x > (l * l + l) as libc::c_double + 1.0f64 {
        let mut status: i32 = gsl_sf_bessel_Ynu_asympx_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= pre;
        (*result).err *= pre;
        return status;
    } else if l > 40 as i32 {
        let mut status_0: i32 = gsl_sf_bessel_Ynu_asymp_Olver_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre_0: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= pre_0;
        (*result).err *= pre_0;
        return status_0;
    } else {
        let mut r_by: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_bym: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_1: i32 = gsl_sf_bessel_y1_e(x, &mut r_by);
        let mut stat_0: i32 = gsl_sf_bessel_y0_e(x, &mut r_bym);
        let mut bym: libc::c_double = r_bym.val;
        let mut by: libc::c_double = r_by.val;
        let mut byp: libc::c_double = 0.;
        let mut j: i32 = 0;
        j = 1 as i32;
        while j < l {
            byp = (2 as i32 * j + 1 as i32) as libc::c_double / x * by - bym;
            bym = by;
            by = byp;
            j += 1;
            j;
        }
        (*result).val = by;
        (*result).err = fabs((*result).val)
            * (2.2204460492503131e-16f64 + fabs(r_by.err / r_by.val)
                + fabs(r_bym.err / r_bym.val));
        return if stat_1 != GSL_SUCCESS as i32 {
            stat_1
        } else if stat_0 != GSL_SUCCESS as i32 {
            stat_0
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_yl_array(
    lmax: i32,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> i32 {
    if lmax < 0 as i32 || x <= 0.0f64 {
        gsl_error(
            b"error\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            237 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if lmax == 0 as i32 {
        let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: i32 = gsl_sf_bessel_y0_e(x, &mut result);
        *result_array.offset(0 as i32 as isize) = result.val;
        return stat;
    } else {
        let mut r_yell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_yellm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_1: i32 = gsl_sf_bessel_y1_e(x, &mut r_yell);
        let mut stat_0: i32 = gsl_sf_bessel_y0_e(x, &mut r_yellm1);
        let mut yellp1: libc::c_double = 0.;
        let mut yell: libc::c_double = r_yell.val;
        let mut yellm1: libc::c_double = r_yellm1.val;
        let mut ell: i32 = 0;
        *result_array.offset(0 as i32 as isize) = yellm1;
        *result_array.offset(1 as i32 as isize) = yell;
        ell = 1 as i32;
        while ell < lmax {
            yellp1 = (2 as i32 * ell + 1 as i32) as libc::c_double / x * yell - yellm1;
            *result_array.offset((ell + 1 as i32) as isize) = yellp1;
            yellm1 = yell;
            yell = yellp1;
            ell += 1;
            ell;
        }
        return if stat_0 != GSL_SUCCESS as i32 {
            stat_0
        } else if stat_1 != GSL_SUCCESS as i32 {
            stat_1
        } else {
            GSL_SUCCESS as i32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_y0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_y0_e(x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_y0_e(x, &result)\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            274 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_y1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_y1_e(x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_y1_e(x, &result)\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            279 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_y2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_y2_e(x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_y2_e(x, &result)\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            284 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_yl(l: i32, x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_bessel_yl_e(l, x, &mut result);
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_bessel_yl_e(l, x, &result)\0" as *const u8 as *const i8,
            b"bessel_y.c\0" as *const u8 as *const i8,
            289 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
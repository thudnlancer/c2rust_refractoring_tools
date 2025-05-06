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
    fn abs(_: i32) -> i32;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sf_fact_e(n: u32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_lnchoose_e(n: u32, m: u32, result: *mut gsl_sf_result) -> i32;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
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
#[inline]
unsafe extern "C" fn locMax3(a: i32, b: i32, c: i32) -> i32 {
    let mut d: i32 = if a > b { a } else { b };
    return if d > c { d } else { c };
}
#[inline]
unsafe extern "C" fn locMin3(a: i32, b: i32, c: i32) -> i32 {
    let mut d: i32 = if a < b { a } else { b };
    return if d < c { d } else { c };
}
#[inline]
unsafe extern "C" fn locMin5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let mut f: i32 = if a < b { a } else { b };
    let mut g: i32 = if c < d { c } else { d };
    let mut h: i32 = if f < g { f } else { g };
    return if e < h { e } else { h };
}
unsafe extern "C" fn delta(
    mut ta: i32,
    mut tb: i32,
    mut tc: i32,
    mut d: *mut gsl_sf_result,
) -> i32 {
    let mut f1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut f2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut f3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut f4: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = 0 as i32;
    status += gsl_sf_fact_e(((ta + tb - tc) / 2 as i32) as u32, &mut f1);
    status += gsl_sf_fact_e(((ta + tc - tb) / 2 as i32) as u32, &mut f2);
    status += gsl_sf_fact_e(((tb + tc - ta) / 2 as i32) as u32, &mut f3);
    status += gsl_sf_fact_e(((ta + tb + tc) / 2 as i32 + 1 as i32) as u32, &mut f4);
    if status != 0 as i32 {
        (*d).val = ::core::f32::INFINITY as libc::c_double;
        (*d).err = ::core::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            72 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    }
    (*d).val = f1.val * f2.val * f3.val / f4.val;
    (*d).err = 4.0f64 * 2.2204460492503131e-16f64 * fabs((*d).val);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn triangle_selection_fails(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
) -> i32 {
    return (two_jb < abs(two_ja - two_jc) || two_jb > two_ja + two_jc
        || two_ja + two_jb + two_jc & 1 as i32 != 0) as i32;
}
unsafe extern "C" fn m_selection_fails(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_ma: i32,
    mut two_mb: i32,
    mut two_mc: i32,
) -> i32 {
    return (abs(two_ma) > two_ja || abs(two_mb) > two_jb || abs(two_mc) > two_jc
        || two_ja + two_ma & 1 as i32 != 0 || two_jb + two_mb & 1 as i32 != 0
        || two_jc + two_mc & 1 as i32 != 0 || two_ma + two_mb + two_mc != 0 as i32)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_3j_e(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_ma: i32,
    mut two_mb: i32,
    mut two_mc: i32,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if two_ja < 0 as i32 || two_jb < 0 as i32 || two_jc < 0 as i32 {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            120 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if triangle_selection_fails(two_ja, two_jb, two_jc) != 0
        || m_selection_fails(two_ja, two_jb, two_jc, two_ma, two_mb, two_mc) != 0
    {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else if two_ma == 0 as i32 && two_mb == 0 as i32 && two_mc == 0 as i32
        && (two_ja + two_jb + two_jc) % 4 as i32 == 2 as i32
    {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else {
        let mut jca: i32 = (-two_ja + two_jb + two_jc) / 2 as i32;
        let mut jcb: i32 = (two_ja - two_jb + two_jc) / 2 as i32;
        let mut jcc: i32 = (two_ja + two_jb - two_jc) / 2 as i32;
        let mut jmma: i32 = (two_ja - two_ma) / 2 as i32;
        let mut jmmb: i32 = (two_jb - two_mb) / 2 as i32;
        let mut jmmc: i32 = (two_jc - two_mc) / 2 as i32;
        let mut jpma: i32 = (two_ja + two_ma) / 2 as i32;
        let mut jpmb: i32 = (two_jb + two_mb) / 2 as i32;
        let mut jpmc: i32 = (two_jc + two_mc) / 2 as i32;
        let mut jsum: i32 = (two_ja + two_jb + two_jc) / 2 as i32;
        let mut kmin: i32 = locMax3(0 as i32, jpmb - jmmc, jmma - jpmc);
        let mut kmax: i32 = locMin3(jcc, jmma, jpmb);
        let mut k: i32 = 0;
        let mut sign: i32 = if kmin - jpma + jmmb & 1 as i32 != 0 {
            -(1 as i32)
        } else {
            1 as i32
        };
        let mut status: i32 = 0 as i32;
        let mut sum_pos: libc::c_double = 0.0f64;
        let mut sum_neg: libc::c_double = 0.0f64;
        let mut sum_err: libc::c_double = 0.0f64;
        let mut bc1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bc2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bc3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bcn1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bcn2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bcd1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bcd2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bcd3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bcd4: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut term: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnorm: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        status += gsl_sf_lnchoose_e(two_ja as u32, jcc as u32, &mut bcn1);
        status += gsl_sf_lnchoose_e(two_jb as u32, jcc as u32, &mut bcn2);
        status += gsl_sf_lnchoose_e((jsum + 1 as i32) as u32, jcc as u32, &mut bcd1);
        status += gsl_sf_lnchoose_e(two_ja as u32, jmma as u32, &mut bcd2);
        status += gsl_sf_lnchoose_e(two_jb as u32, jmmb as u32, &mut bcd3);
        status += gsl_sf_lnchoose_e(two_jc as u32, jpmc as u32, &mut bcd4);
        lnorm.val = 0.5f64
            * (bcn1.val + bcn2.val - bcd1.val - bcd2.val - bcd3.val - bcd4.val
                - log(two_jc as libc::c_double + 1.0f64));
        lnorm.err = 0.5f64
            * (bcn1.err + bcn2.err + bcd1.err + bcd2.err + bcd3.err + bcd4.err
                + 2.2204460492503131e-16f64 * log(two_jc as libc::c_double + 1.0f64));
        k = kmin;
        while k <= kmax {
            status += gsl_sf_lnchoose_e(jcc as u32, k as u32, &mut bc1);
            status += gsl_sf_lnchoose_e(jcb as u32, (jmma - k) as u32, &mut bc2);
            status += gsl_sf_lnchoose_e(jca as u32, (jpmb - k) as u32, &mut bc3);
            status
                += gsl_sf_exp_err_e(
                    bc1.val + bc2.val + bc3.val + lnorm.val,
                    bc1.err + bc2.err + bc3.err + lnorm.err,
                    &mut term,
                );
            if status != 0 as i32 {
                (*result).val = ::core::f32::INFINITY as libc::c_double;
                (*result).err = ::core::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const i8,
                    b"coupling.c\0" as *const u8 as *const i8,
                    175 as i32,
                    GSL_EOVRFLW as i32,
                );
                return GSL_EOVRFLW as i32;
            }
            if sign < 0 as i32 {
                sum_neg += term.val;
            } else {
                sum_pos += term.val;
            }
            sum_err += term.err;
            sign = -sign;
            k += 1;
            k;
        }
        (*result).val = sum_pos - sum_neg;
        (*result).err = sum_err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (kmax - kmin) as libc::c_double
                * fabs((*result).val);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_6j_INCORRECT_e(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
    mut result: *mut gsl_sf_result,
) -> i32 {
    return gsl_sf_coupling_6j_e(two_ja, two_jb, two_je, two_jd, two_jc, two_jf, result);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_6j_e(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if two_ja < 0 as i32 || two_jb < 0 as i32 || two_jc < 0 as i32 || two_jd < 0 as i32
        || two_je < 0 as i32 || two_jf < 0 as i32
    {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            221 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if triangle_selection_fails(two_ja, two_jb, two_jc) != 0
        || triangle_selection_fails(two_ja, two_je, two_jf) != 0
        || triangle_selection_fails(two_jb, two_jd, two_jf) != 0
        || triangle_selection_fails(two_je, two_jd, two_jc) != 0
    {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else {
        let mut n1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d4: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d5: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d6: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut norm: libc::c_double = 0.;
        let mut tk: i32 = 0;
        let mut tkmin: i32 = 0;
        let mut tkmax: i32 = 0;
        let mut phase: libc::c_double = 0.;
        let mut sum_pos: libc::c_double = 0.0f64;
        let mut sum_neg: libc::c_double = 0.0f64;
        let mut sumsq_err: libc::c_double = 0.0f64;
        let mut status: i32 = 0 as i32;
        status += delta(two_ja, two_jb, two_jc, &mut d1);
        status += delta(two_ja, two_je, two_jf, &mut d2);
        status += delta(two_jb, two_jd, two_jf, &mut d3);
        status += delta(two_je, two_jd, two_jc, &mut d4);
        if status != GSL_SUCCESS as i32 {
            (*result).val = ::core::f32::INFINITY as libc::c_double;
            (*result).err = ::core::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const i8,
                b"coupling.c\0" as *const u8 as *const i8,
                247 as i32,
                GSL_EOVRFLW as i32,
            );
            return GSL_EOVRFLW as i32;
        }
        norm = sqrt(d1.val) * sqrt(d2.val) * sqrt(d3.val) * sqrt(d4.val);
        tkmin = locMax3(
            0 as i32,
            two_ja + two_jd - two_jc - two_jf,
            two_jb + two_je - two_jc - two_jf,
        );
        tkmax = locMin5(
            two_ja + two_jb + two_je + two_jd + 2 as i32,
            two_ja + two_jb - two_jc,
            two_je + two_jd - two_jc,
            two_ja + two_je - two_jf,
            two_jb + two_jd - two_jf,
        );
        phase = if (two_ja + two_jb + two_je + two_jd + tkmin) / 2 as i32 & 1 as i32 != 0
        {
            -1.0f64
        } else {
            1.0f64
        };
        tk = tkmin;
        while tk <= tkmax {
            let mut term: libc::c_double = 0.;
            let mut term_err: libc::c_double = 0.;
            let mut den_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut den_2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut d1_a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut d1_b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            status = 0 as i32;
            status
                += gsl_sf_fact_e(
                    ((two_ja + two_jb + two_je + two_jd - tk) / 2 as i32 + 1 as i32)
                        as u32,
                    &mut n1,
                );
            status += gsl_sf_fact_e((tk / 2 as i32) as u32, &mut d1_a);
            status
                += gsl_sf_fact_e(
                    ((two_jc + two_jf - two_ja - two_jd + tk) / 2 as i32) as u32,
                    &mut d1_b,
                );
            status
                += gsl_sf_fact_e(
                    ((two_jc + two_jf - two_jb - two_je + tk) / 2 as i32) as u32,
                    &mut d2,
                );
            status
                += gsl_sf_fact_e(
                    ((two_ja + two_jb - two_jc - tk) / 2 as i32) as u32,
                    &mut d3,
                );
            status
                += gsl_sf_fact_e(
                    ((two_je + two_jd - two_jc - tk) / 2 as i32) as u32,
                    &mut d4,
                );
            status
                += gsl_sf_fact_e(
                    ((two_ja + two_je - two_jf - tk) / 2 as i32) as u32,
                    &mut d5,
                );
            status
                += gsl_sf_fact_e(
                    ((two_jb + two_jd - two_jf - tk) / 2 as i32) as u32,
                    &mut d6,
                );
            if status != GSL_SUCCESS as i32 {
                (*result).val = ::core::f32::INFINITY as libc::c_double;
                (*result).err = ::core::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const i8,
                    b"coupling.c\0" as *const u8 as *const i8,
                    282 as i32,
                    GSL_EOVRFLW as i32,
                );
                return GSL_EOVRFLW as i32;
            }
            d1.val = d1_a.val * d1_b.val;
            d1.err = d1_a.err * fabs(d1_b.val) + fabs(d1_a.val) * d1_b.err;
            den_1.val = d1.val * d2.val * d3.val;
            den_1.err = d1.err * fabs(d2.val * d3.val);
            den_1.err += d2.err * fabs(d1.val * d3.val);
            den_1.err += d3.err * fabs(d1.val * d2.val);
            den_2.val = d4.val * d5.val * d6.val;
            den_2.err = d4.err * fabs(d5.val * d6.val);
            den_2.err += d5.err * fabs(d4.val * d6.val);
            den_2.err += d6.err * fabs(d4.val * d5.val);
            term = phase * n1.val / den_1.val / den_2.val;
            phase = -phase;
            term_err = n1.err / fabs(den_1.val) / fabs(den_2.val);
            term_err += fabs(term / den_1.val) * den_1.err;
            term_err += fabs(term / den_2.val) * den_2.err;
            if term >= 0.0f64 {
                sum_pos += norm * term;
            } else {
                sum_neg -= norm * term;
            }
            sumsq_err += norm * norm * term_err * term_err;
            tk += 2 as i32;
        }
        (*result).val = sum_pos - sum_neg;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
        (*result).err
            += sqrt(sumsq_err / (0.5f64 * (tkmax - tkmin) as libc::c_double + 1.0f64));
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * ((tkmax - tkmin) as libc::c_double + 2.0f64) * fabs((*result).val);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_RacahW_e(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
    mut result: *mut gsl_sf_result,
) -> i32 {
    let mut status: i32 = gsl_sf_coupling_6j_e(
        two_ja,
        two_jb,
        two_je,
        two_jd,
        two_jc,
        two_jf,
        result,
    );
    let mut phase_sum: i32 = (two_ja + two_jb + two_jc + two_jd) / 2 as i32;
    (*result).val *= if phase_sum & 1 as i32 != 0 { -1.0f64 } else { 1.0f64 };
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_9j_e(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
    mut two_jg: i32,
    mut two_jh: i32,
    mut two_ji: i32,
    mut result: *mut gsl_sf_result,
) -> i32 {
    if two_ja < 0 as i32 || two_jb < 0 as i32 || two_jc < 0 as i32 || two_jd < 0 as i32
        || two_je < 0 as i32 || two_jf < 0 as i32 || two_jg < 0 as i32
        || two_jh < 0 as i32 || two_ji < 0 as i32
    {
        (*result).val = ::core::f32::NAN as libc::c_double;
        (*result).err = ::core::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            348 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else if triangle_selection_fails(two_ja, two_jb, two_jc) != 0
        || triangle_selection_fails(two_jd, two_je, two_jf) != 0
        || triangle_selection_fails(two_jg, two_jh, two_ji) != 0
        || triangle_selection_fails(two_ja, two_jd, two_jg) != 0
        || triangle_selection_fails(two_jb, two_je, two_jh) != 0
        || triangle_selection_fails(two_jc, two_jf, two_ji) != 0
    {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as i32;
    } else {
        let mut tk: i32 = 0;
        let mut tkmin: i32 = locMax3(
            abs(two_ja - two_ji),
            abs(two_jh - two_jd),
            abs(two_jb - two_jf),
        );
        let mut tkmax: i32 = locMin3(two_ja + two_ji, two_jh + two_jd, two_jb + two_jf);
        let mut sum_pos: libc::c_double = 0.0f64;
        let mut sum_neg: libc::c_double = 0.0f64;
        let mut sumsq_err: libc::c_double = 0.0f64;
        let mut phase: libc::c_double = 0.;
        tk = tkmin;
        while tk <= tkmax {
            let mut s1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut s2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut s3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut term: libc::c_double = 0.;
            let mut term_err: libc::c_double = 0.;
            let mut status: i32 = 0 as i32;
            status
                += gsl_sf_coupling_6j_e(
                    two_ja,
                    two_ji,
                    tk,
                    two_jh,
                    two_jd,
                    two_jg,
                    &mut s1,
                );
            status
                += gsl_sf_coupling_6j_e(
                    two_jb,
                    two_jf,
                    tk,
                    two_jd,
                    two_jh,
                    two_je,
                    &mut s2,
                );
            status
                += gsl_sf_coupling_6j_e(
                    two_ja,
                    two_ji,
                    tk,
                    two_jf,
                    two_jb,
                    two_jc,
                    &mut s3,
                );
            if status != GSL_SUCCESS as i32 {
                (*result).val = ::core::f32::INFINITY as libc::c_double;
                (*result).err = ::core::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const i8,
                    b"coupling.c\0" as *const u8 as *const i8,
                    380 as i32,
                    GSL_EOVRFLW as i32,
                );
                return GSL_EOVRFLW as i32;
            }
            term = s1.val * s2.val * s3.val;
            term_err = s1.err * fabs(s2.val * s3.val);
            term_err += s2.err * fabs(s1.val * s3.val);
            term_err += s3.err * fabs(s1.val * s2.val);
            if term >= 0.0f64 {
                sum_pos += (tk + 1 as i32) as libc::c_double * term;
            } else {
                sum_neg -= (tk + 1 as i32) as libc::c_double * term;
            }
            sumsq_err
                += (tk + 1 as i32) as libc::c_double * term_err
                    * ((tk + 1 as i32) as libc::c_double * term_err);
            tk += 2 as i32;
        }
        phase = if tkmin & 1 as i32 != 0 { -1.0f64 } else { 1.0f64 };
        (*result).val = phase * (sum_pos - sum_neg);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
        (*result).err
            += sqrt(sumsq_err / (0.5f64 * (tkmax - tkmin) as libc::c_double + 1.0f64));
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * ((tkmax - tkmin) as libc::c_double + 2.0f64) * fabs((*result).val);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_3j(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_ma: i32,
    mut two_mb: i32,
    mut two_mc: i32,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_coupling_3j_e(
        two_ja,
        two_jb,
        two_jc,
        two_ma,
        two_mb,
        two_mc,
        &mut result,
    );
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_coupling_3j_e(two_ja, two_jb, two_jc, two_ma, two_mb, two_mc, &result)\0"
                as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            418 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_6j_INCORRECT(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_coupling_6j_INCORRECT_e(
        two_ja,
        two_jb,
        two_jc,
        two_jd,
        two_je,
        two_jf,
        &mut result,
    );
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_coupling_6j_INCORRECT_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &result)\0"
                as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            428 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_6j(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_coupling_6j_e(
        two_ja,
        two_jb,
        two_jc,
        two_jd,
        two_je,
        two_jf,
        &mut result,
    );
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_coupling_6j_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &result)\0"
                as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            439 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_RacahW(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_coupling_RacahW_e(
        two_ja,
        two_jb,
        two_jc,
        two_jd,
        two_je,
        two_jf,
        &mut result,
    );
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_coupling_RacahW_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, &result)\0"
                as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            448 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_coupling_9j(
    mut two_ja: i32,
    mut two_jb: i32,
    mut two_jc: i32,
    mut two_jd: i32,
    mut two_je: i32,
    mut two_jf: i32,
    mut two_jg: i32,
    mut two_jh: i32,
    mut two_ji: i32,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: i32 = gsl_sf_coupling_9j_e(
        two_ja,
        two_jb,
        two_jc,
        two_jd,
        two_je,
        two_jf,
        two_jg,
        two_jh,
        two_ji,
        &mut result,
    );
    if status != GSL_SUCCESS as i32 {
        gsl_error(
            b"gsl_sf_coupling_9j_e(two_ja, two_jb, two_jc, two_jd, two_je, two_jf, two_jg, two_jh, two_ji, &result)\0"
                as *const u8 as *const i8,
            b"coupling.c\0" as *const u8 as *const i8,
            459 as i32,
            status,
        );
        return result.val;
    }
    return result.val;
}
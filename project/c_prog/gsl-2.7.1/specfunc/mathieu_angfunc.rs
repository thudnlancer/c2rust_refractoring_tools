use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_mathieu_b_coeff(
        order: libc::c_int,
        qq: libc::c_double,
        aa: libc::c_double,
        coeff: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_mathieu_a_coeff(
        order: libc::c_int,
        qq: libc::c_double,
        aa: libc::c_double,
        coeff: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_mathieu_b_e(
        order: libc::c_int,
        qq: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_mathieu_a_e(
        order: libc::c_int,
        qq: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_mathieu_b_array(
        order_min: libc::c_int,
        order_max: libc::c_int,
        qq: libc::c_double,
        work: *mut gsl_sf_mathieu_workspace,
        result_array: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_mathieu_a_array(
        order_min: libc::c_int,
        order_max: libc::c_int,
        qq: libc::c_double,
        work: *mut gsl_sf_mathieu_workspace,
        result_array: *mut libc::c_double,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
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
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_symmv_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
    pub gc: *mut libc::c_double,
    pub gs: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_mathieu_workspace {
    pub size: size_t,
    pub even_order: size_t,
    pub odd_order: size_t,
    pub extra_values: libc::c_int,
    pub qa: libc::c_double,
    pub qb: libc::c_double,
    pub aa: *mut libc::c_double,
    pub bb: *mut libc::c_double,
    pub dd: *mut libc::c_double,
    pub ee: *mut libc::c_double,
    pub tt: *mut libc::c_double,
    pub e2: *mut libc::c_double,
    pub zz: *mut libc::c_double,
    pub eval: *mut gsl_vector,
    pub evec: *mut gsl_matrix,
    pub wmat: *mut gsl_eigen_symmv_workspace,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_ce_e(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut zz: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut even_odd: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut coeff: [libc::c_double; 100] = [0.; 100];
    let mut norm: libc::c_double = 0.;
    let mut fn_0: libc::c_double = 0.;
    let mut factor: libc::c_double = 0.;
    let mut aa: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    norm = 0.0f64;
    even_odd = 0 as libc::c_int;
    if order % 2 as libc::c_int != 0 as libc::c_int {
        even_odd = 1 as libc::c_int;
    }
    if qq == 0.0f64 {
        norm = 1.0f64;
        if order == 0 as libc::c_int {
            norm = sqrt(2.0f64);
        }
        fn_0 = cos(order as libc::c_double * zz) / norm;
        (*result).val = fn_0;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64;
        factor = fabs(fn_0);
        if factor > 1.0f64 {
            (*result).err *= factor;
        }
        return GSL_SUCCESS as libc::c_int;
    }
    if order < 0 as libc::c_int {
        order *= -(1 as libc::c_int);
    }
    status = gsl_sf_mathieu_a_e(order, qq, &mut aa);
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    status = gsl_sf_mathieu_a_coeff(order, qq, aa.val, coeff.as_mut_ptr());
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    if even_odd == 0 as libc::c_int {
        fn_0 = 0.0f64;
        norm = coeff[0 as libc::c_int as usize] * coeff[0 as libc::c_int as usize];
        ii = 0 as libc::c_int;
        while ii < 100 as libc::c_int {
            fn_0 += coeff[ii as usize] * cos(2.0f64 * ii as libc::c_double * zz);
            norm += coeff[ii as usize] * coeff[ii as usize];
            ii += 1;
            ii;
        }
    } else {
        fn_0 = 0.0f64;
        ii = 0 as libc::c_int;
        while ii < 100 as libc::c_int {
            fn_0
                += coeff[ii as usize]
                    * cos((2.0f64 * ii as libc::c_double + 1.0f64) * zz);
            norm += coeff[ii as usize] * coeff[ii as usize];
            ii += 1;
            ii;
        }
    }
    norm = sqrt(norm);
    fn_0 /= norm;
    (*result).val = fn_0;
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64;
    factor = fabs(fn_0);
    if factor > 1.0f64 {
        (*result).err *= factor;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_se_e(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut zz: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut even_odd: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut coeff: [libc::c_double; 100] = [0.; 100];
    let mut norm: libc::c_double = 0.;
    let mut fn_0: libc::c_double = 0.;
    let mut factor: libc::c_double = 0.;
    let mut aa: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    norm = 0.0f64;
    even_odd = 0 as libc::c_int;
    if order % 2 as libc::c_int != 0 as libc::c_int {
        even_odd = 1 as libc::c_int;
    }
    if order == 0 as libc::c_int {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    if qq == 0.0f64 {
        norm = 1.0f64;
        fn_0 = sin(order as libc::c_double * zz);
        (*result).val = fn_0;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64;
        factor = fabs(fn_0);
        if factor > 1.0f64 {
            (*result).err *= factor;
        }
        return GSL_SUCCESS as libc::c_int;
    }
    if order < 0 as libc::c_int {
        order *= -(1 as libc::c_int);
    }
    status = gsl_sf_mathieu_b_e(order, qq, &mut aa);
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    status = gsl_sf_mathieu_b_coeff(order, qq, aa.val, coeff.as_mut_ptr());
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    if even_odd == 0 as libc::c_int {
        fn_0 = 0.0f64;
        ii = 0 as libc::c_int;
        while ii < 100 as libc::c_int {
            norm += coeff[ii as usize] * coeff[ii as usize];
            fn_0
                += coeff[ii as usize]
                    * sin(2.0f64 * (ii + 1 as libc::c_int) as libc::c_double * zz);
            ii += 1;
            ii;
        }
    } else {
        fn_0 = 0.0f64;
        ii = 0 as libc::c_int;
        while ii < 100 as libc::c_int {
            norm += coeff[ii as usize] * coeff[ii as usize];
            fn_0
                += coeff[ii as usize]
                    * sin(
                        (2.0f64 * ii as libc::c_double
                            + 1 as libc::c_int as libc::c_double) * zz,
                    );
            ii += 1;
            ii;
        }
    }
    norm = sqrt(norm);
    fn_0 /= norm;
    (*result).val = fn_0;
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64;
    factor = fabs(fn_0);
    if factor > 1.0f64 {
        (*result).err *= factor;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_ce_array(
    mut nmin: libc::c_int,
    mut nmax: libc::c_int,
    mut qq: libc::c_double,
    mut zz: libc::c_double,
    mut work: *mut gsl_sf_mathieu_workspace,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut even_odd: libc::c_int = 0;
    let mut order: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut coeff: [libc::c_double; 100] = [0.; 100];
    let mut aa: *mut libc::c_double = (*work).aa;
    let mut norm: libc::c_double = 0.;
    ii = 0 as libc::c_int;
    while ii < nmax - nmin + 1 as libc::c_int {
        *result_array.offset(ii as isize) = 0.0f64;
        ii += 1;
        ii;
    }
    if (*work).size < nmax as libc::c_uint as libc::c_ulong {
        gsl_error(
            b"Work space not large enough\0" as *const u8 as *const libc::c_char,
            b"mathieu_angfunc.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if nmin < 0 as libc::c_int || nmax < nmin {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"mathieu_angfunc.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    gsl_sf_mathieu_a_array(0 as libc::c_int, nmax, qq, work, aa);
    ii = 0 as libc::c_int;
    order = nmin;
    while order <= nmax {
        norm = 0.0f64;
        even_odd = 0 as libc::c_int;
        if order % 2 as libc::c_int != 0 as libc::c_int {
            even_odd = 1 as libc::c_int;
        }
        if qq == 0.0f64 {
            norm = 1.0f64;
            if order == 0 as libc::c_int {
                norm = sqrt(2.0f64);
            }
            *result_array.offset(ii as isize) = cos(order as libc::c_double * zz) / norm;
        } else {
            status = gsl_sf_mathieu_a_coeff(
                order,
                qq,
                *aa.offset(order as isize),
                coeff.as_mut_ptr(),
            );
            if status != GSL_SUCCESS as libc::c_int {
                return status;
            }
            if even_odd == 0 as libc::c_int {
                norm = coeff[0 as libc::c_int as usize]
                    * coeff[0 as libc::c_int as usize];
                jj = 0 as libc::c_int;
                while jj < 100 as libc::c_int {
                    *result_array.offset(ii as isize)
                        += coeff[jj as usize] * cos(2.0f64 * jj as libc::c_double * zz);
                    norm += coeff[jj as usize] * coeff[jj as usize];
                    jj += 1;
                    jj;
                }
            } else {
                jj = 0 as libc::c_int;
                while jj < 100 as libc::c_int {
                    *result_array.offset(ii as isize)
                        += coeff[jj as usize]
                            * cos((2.0f64 * jj as libc::c_double + 1.0f64) * zz);
                    norm += coeff[jj as usize] * coeff[jj as usize];
                    jj += 1;
                    jj;
                }
            }
            norm = sqrt(norm);
            *result_array.offset(ii as isize) /= norm;
        }
        ii += 1;
        ii;
        order += 1;
        order;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_se_array(
    mut nmin: libc::c_int,
    mut nmax: libc::c_int,
    mut qq: libc::c_double,
    mut zz: libc::c_double,
    mut work: *mut gsl_sf_mathieu_workspace,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut even_odd: libc::c_int = 0;
    let mut order: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut coeff: [libc::c_double; 100] = [0.; 100];
    let mut bb: *mut libc::c_double = (*work).bb;
    let mut norm: libc::c_double = 0.;
    ii = 0 as libc::c_int;
    while ii < nmax - nmin + 1 as libc::c_int {
        *result_array.offset(ii as isize) = 0.0f64;
        ii += 1;
        ii;
    }
    if (*work).size < nmax as libc::c_uint as libc::c_ulong {
        gsl_error(
            b"Work space not large enough\0" as *const u8 as *const libc::c_char,
            b"mathieu_angfunc.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if nmin < 0 as libc::c_int || nmax < nmin {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"mathieu_angfunc.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    gsl_sf_mathieu_b_array(0 as libc::c_int, nmax, qq, work, bb);
    ii = 0 as libc::c_int;
    order = nmin;
    while order <= nmax {
        norm = 0.0f64;
        even_odd = 0 as libc::c_int;
        if order % 2 as libc::c_int != 0 as libc::c_int {
            even_odd = 1 as libc::c_int;
        }
        if order == 0 as libc::c_int {
            norm = 1.0f64;
            *result_array.offset(ii as isize) = 0.0f64;
        } else if qq == 0.0f64 {
            norm = 1.0f64;
            *result_array.offset(ii as isize) = sin(order as libc::c_double * zz);
        } else {
            status = gsl_sf_mathieu_b_coeff(
                order,
                qq,
                *bb.offset(order as isize),
                coeff.as_mut_ptr(),
            );
            if status != GSL_SUCCESS as libc::c_int {
                return status;
            }
            if even_odd == 0 as libc::c_int {
                jj = 0 as libc::c_int;
                while jj < 100 as libc::c_int {
                    *result_array.offset(ii as isize)
                        += coeff[jj as usize]
                            * sin(
                                2.0f64 * (jj + 1 as libc::c_int) as libc::c_double * zz,
                            );
                    norm += coeff[jj as usize] * coeff[jj as usize];
                    jj += 1;
                    jj;
                }
            } else {
                jj = 0 as libc::c_int;
                while jj < 100 as libc::c_int {
                    *result_array.offset(ii as isize)
                        += coeff[jj as usize]
                            * sin((2.0f64 * jj as libc::c_double + 1.0f64) * zz);
                    norm += coeff[jj as usize] * coeff[jj as usize];
                    jj += 1;
                    jj;
                }
            }
            norm = sqrt(norm);
            *result_array.offset(ii as isize) /= norm;
        }
        ii += 1;
        ii;
        order += 1;
        order;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_ce(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut zz: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_mathieu_ce_e(order, qq, zz, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_mathieu_ce_e(order, qq, zz, &result)\0" as *const u8
                as *const libc::c_char,
            b"mathieu_angfunc.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_se(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut zz: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_mathieu_se_e(order, qq, zz, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_mathieu_se_e(order, qq, zz, &result)\0" as *const u8
                as *const libc::c_char,
            b"mathieu_angfunc.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}

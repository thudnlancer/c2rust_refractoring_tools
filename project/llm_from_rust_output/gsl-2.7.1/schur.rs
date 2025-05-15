use libc::{c_double, c_int, c_ulong};
use std::f64;

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [c_double; 2],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_block_complex {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

pub const GSL_SUCCESS: c_int = 0;
pub const GSL_FAILURE: c_int = -1;
pub const GSL_CONTINUE: c_int = -2;
pub const GSL_EDOM: c_int = 1;
pub const GSL_ERANGE: c_int = 2;
pub const GSL_EFAULT: c_int = 3;
pub const GSL_EINVAL: c_int = 4;
pub const GSL_EFAILED: c_int = 5;
pub const GSL_EFACTOR: c_int = 6;
pub const GSL_ESANITY: c_int = 7;
pub const GSL_ENOMEM: c_int = 8;
pub const GSL_EBADFUNC: c_int = 9;
pub const GSL_ERUNAWAY: c_int = 10;
pub const GSL_EMAXITER: c_int = 11;
pub const GSL_EZERODIV: c_int = 12;
pub const GSL_EBADTOL: c_int = 13;
pub const GSL_ETOL: c_int = 14;
pub const GSL_EUNDRFLW: c_int = 15;
pub const GSL_EOVRFLW: c_int = 16;
pub const GSL_ELOSS: c_int = 17;
pub const GSL_EROUND: c_int = 18;
pub const GSL_EBADLEN: c_int = 19;
pub const GSL_ENOTSQR: c_int = 20;
pub const GSL_ESING: c_int = 21;
pub const GSL_EDIVERGE: c_int = 22;
pub const GSL_EUNSUP: c_int = 23;
pub const GSL_EUNIMPL: c_int = 24;
pub const GSL_ECACHE: c_int = 25;
pub const GSL_ETABLE: c_int = 26;
pub const GSL_ENOPROG: c_int = 27;
pub const GSL_ENOPROGJ: c_int = 28;
pub const GSL_ETOLF: c_int = 29;
pub const GSL_ETOLX: c_int = 30;
pub const GSL_ETOLG: c_int = 31;
pub const GSL_EOF: c_int = 32;

fn sqrt(x: c_double) -> c_double {
    x.sqrt()
}

fn fabs(x: c_double) -> c_double {
    x.abs()
}

fn gsl_blas_dscal(alpha: c_double, x: &mut gsl_vector) {
    unsafe {
        libc::gsl_blas_dscal(alpha, x as *mut _);
    }
}

fn gsl_blas_zdscal(alpha: c_double, x: &mut gsl_vector_complex) {
    unsafe {
        libc::gsl_blas_zdscal(alpha, x as *mut _);
    }
}

fn gsl_complex_div(a: gsl_complex, b: gsl_complex) -> gsl_complex {
    unsafe { libc::gsl_complex_div(a, b) }
}

fn gsl_complex_mul_real(a: gsl_complex, x: c_double) -> gsl_complex {
    unsafe { libc::gsl_complex_mul_real(a, x) }
}

fn gsl_vector_get(v: &gsl_vector, i: size_t) -> c_double {
    unsafe { *v.data.offset((i * v.stride) as isize) }
}

fn gsl_vector_set(v: &mut gsl_vector, i: size_t, x: c_double) {
    unsafe {
        *v.data.offset((i * v.stride) as isize) = x;
    }
}

fn gsl_vector_complex_get(v: &gsl_vector_complex, i: size_t) -> gsl_complex {
    unsafe {
        *(v.data
            .offset((2 * i * v.stride) as isize) as *mut c_double as *mut gsl_complex)
    }
}

fn gsl_vector_complex_set(v: &mut gsl_vector_complex, i: size_t, z: gsl_complex) {
    unsafe {
        *(v.data
            .offset((2 * i * v.stride) as isize as *mut c_double as *mut gsl_complex) = z;
    }
}

fn gsl_matrix_get(m: &gsl_matrix, i: size_t, j: size_t) -> c_double {
    unsafe { *m.data.offset((i * m.tda + j) as isize) }
}

pub fn gsl_schur_gen_eigvals(
    A: &gsl_matrix,
    B: &gsl_matrix,
    wr1: &mut c_double,
    wr2: &mut c_double,
    wi: &mut c_double,
    scale1: &mut c_double,
    scale2: &mut c_double,
) -> c_int {
    let safemin = 2.2250738585072014e-308 * 1.0e2;
    let safemax = 1.0 / safemin;
    let rtmin = sqrt(safemin);
    let rtmax = 1.0 / rtmin;

    let a11 = gsl_matrix_get(A, 0, 0);
    let a12 = gsl_matrix_get(A, 0, 1);
    let a21 = gsl_matrix_get(A, 1, 0);
    let a22 = gsl_matrix_get(A, 1, 1);

    let b11 = gsl_matrix_get(B, 0, 0);
    let b12 = gsl_matrix_get(B, 0, 1);
    let b22 = gsl_matrix_get(B, 1, 1);

    let anorm = {
        let row1 = fabs(a11) + fabs(a21);
        let row2 = fabs(a12) + fabs(a22);
        if row1 > row2 { row1 } else { row2 }
    }.max(safemin);

    let ascale = 1.0 / anorm;
    let a11 = ascale * a11;
    let a12 = ascale * a12;
    let a21 = ascale * a21;
    let a22 = ascale * a22;

    let bmin = rtmin * {
        let babs = [fabs(b11), fabs(b12), fabs(b22)];
        babs.iter().fold(rtmin, |a, &b| a.max(b))
    };

    let b11 = if fabs(b11) < bmin {
        if b11 >= 0.0 { bmin } else { -bmin }
    } else {
        b11
    };

    let b22 = if fabs(b22) < bmin {
        if b22 >= 0.0 { bmin } else { -bmin }
    } else {
        b22
    };

    let bnorm = {
        let b11_abs = fabs(b11);
        let b12_b22 = fabs(b12) + fabs(b22);
        if b11_abs > b12_b22.max(safemin) {
            b11_abs
        } else {
            b12_b22.max(safemin)
        }
    };

    let bsize = fabs(b11).max(fabs(b22));
    let bscale = 1.0 / bsize;
    let b11 = b11 * bscale;
    let b12 = b12 * bscale;
    let b22 = b22 * bscale;

    let binv11 = 1.0 / b11;
    let binv22 = 1.0 / b22;

    let s1 = a11 * binv11;
    let s2 = a22 * binv22;

    let (as12, as22, abi22, shift) = if fabs(s1) <= fabs(s2) {
        let as12 = a12 - s1 * b12;
        let as22 = a22 - s1 * b22;
        let ss = a21 * (binv11 * binv22);
        let abi22 = as22 * binv22 - ss * b12;
        (as12, as22, abi22, s1)
    } else {
        let as12 = a12 - s2 * b12;
        let as11 = a11 - s2 * b11;
        let ss = a21 * (binv11 * binv22);
        let abi22 = -ss * b12;
        (as12, as11 * binv11 + abi22, abi22, s2)
    };

    let pp = 0.5 * abi22;
    let qq = ss * as12;

    let (discr, r) = if fabs(pp * rtmin) >= 1.0 {
        let discr = rtmin * pp * (rtmin * pp) + qq * safemin;
        (discr, sqrt(fabs(discr)) * rtmax)
    } else if pp * pp + fabs(qq) <= safemin {
        let discr = rtmax * pp * (rtmax * pp) + qq * safemax;
        (discr, sqrt(fabs(discr)) * rtmin)
    } else {
        let discr = pp * pp + qq;
        (discr, sqrt(fabs(discr)))
    };

    if discr >= 0.0 || r == 0.0 {
        let sum = pp + if pp >= 0.0 { r } else { -r };
        let diff = pp - if pp >= 0.0 { r } else { -r };
        let wbig = shift + sum;
        let wsmall = shift + diff;

        let (wbig, wsmall) = if 0.5 * fabs(wbig) > fabs(wsmall).max(safemin) {
            let wdet = (a11 * a22 - a12 * a21) * (binv11 * binv22);
            (wbig, wdet / wbig)
        } else {
            (wbig, wsmall)
        };

        if pp > abi22 {
            *wr1 = wbig.min(wsmall);
            *wr2 = wbig.max(wsmall);
        } else {
            *wr1 = wbig.max(wsmall);
            *wr2 = wbig.min(wsmall);
        }
        *wi = 0.0;
    } else {
        *wr1 = shift + pp;
        *wr2 = *wr1;
        *wi = r;
    }

    let fuzzy1 = 1.0 + 1.0e-5;
    let c1 = bsize * (safemin * ascale.max(1.0));
    let c2 = safemin * bnorm.max(1.0);
    let c3 = bsize * safemin;
    let c4 = if ascale <= 1.0 && bsize <= 1.0 {
        (ascale / safemin * bsize).min(1.0)
    } else {
        1.0
    };
    let c5 = if ascale <= 1.0 || bsize <= 1.0 {
        (ascale * bsize).min(1.0)
    } else {
        1.0
    };

    let wabs = fabs(*wr1) + fabs(*wi);
    let wsize = [safemin, c1, fuzzy1 * (wabs * c2 + c3), c4, 0.5 * wabs.max(c5)]
        .iter()
        .fold(1.0, |a, &b| a.max(b));

    if wsize != 1.0 {
        let wscale = 1.0 / wsize;
        *scale1 = if wsize > 1.0 {
            ascale.max(bsize) * wscale * ascale.min(bsize)
        } else {
            ascale.min(bsize) * wscale * ascale.max(bsize)
        };
        *wr1 *= wscale;
        if *wi != 0.0 {
            *wi *= wscale;
            *wr2 = *wr1;
            *scale2 = *scale1;
        }
    } else {
        *scale1 = ascale * bsize;
        *scale2 = *scale1;
    }

    if *wi == 0.0 {
        let wabs2 = fabs(*wr2);
        let wsize2 = [safemin, c1, fuzzy1 * (wabs2 * c2 + c3), c4, 0.5 * wabs2.max(c5)]
            .iter()
            .fold(1.0, |a, &b| a.max(b));

        if wsize2 != 1.0 {
            let wscale = 1.0 / wsize2;
            *scale2 = if wsize2 > 1.0 {
                ascale.max(bsize) * wscale * ascale.min(bsize)
            } else {
                ascale.min(bsize) * wscale * ascale.max(bsize)
            };
            *wr2 *= wscale;
        } else {
            *scale2 = ascale * bsize;
        }
    }

    GSL_SUCCESS
}

pub fn gsl_schur_solve_equation(
    ca: c_double,
    A: &gsl_matrix,
    z: c_double,
    d1: c_double,
    d2: c_double,
    b: &gsl_vector,
    x: &mut gsl_vector,
    s: &mut c_double,
    xnorm: &mut c_double,
    smin: c_double,
) -> c_int {
    let n = A.size1;
    let mut scale = 1.0;

    if n == 1 {
        let c = ca * gsl_matrix_get(A, 0, 0) - z * d1;
        let cnorm = fabs(c).max(smin);
        let bnorm = fabs(gsl_vector_get(b, 0));

        if cnorm < 1.0 && bnorm > 1.0 {
            if bnorm > (1.0 - 2.2204460492503131e-16) / (2.0 * 2.2250738585072014e-308) * cnorm {
                scale = 1.0 / bnorm;
            }
        }

        gsl_vector_set(x, 0, gsl_vector_get(b, 0) * scale / c);
        *xnorm = fabs(gsl_vector_get(x, 0));
    } else {
        let mut cr = [[0.0; 2]; 2];
        cr[0][0] = ca * gsl_matrix_get(A, 0, 0) - z * d1;
        cr[1][1] = ca * gsl_matrix_get(A, 1, 1) - z * d2;
        cr[0][1] = ca * gsl_matrix_get(A, 1, 0);
        cr[1][0] = ca * gsl_matrix_get(A, 0, 1);

        let ipivot = [
            [0, 1, 2, 3],
            [1, 0, 3, 2],
            [2, 3, 0, 1],
            [3, 2, 1, 0],
        ];
        let rswap = [0, 1, 0, 1];
        let zswap = [0, 0, 1, 1];

        let crv = &mut cr as *mut _ as *mut c_double;
        let mut cmax = 0.0;
        let mut icmax = 0;

        unsafe {
            for j in 0..4 {
                let val = fabs(*crv.offset(j));
                if val > cmax {
                    cmax = val;
                    icmax = j;
                }
            }
        }

        let bval1 = gsl_vector_get(b, 0);
        let bval2 = gsl_vector_get(b, 1);

        if cmax < smin {
            let bnorm = fabs(bval1).max(fabs(bval2));
            if smin < 1.0 && bnorm > 1.0 {
                if bnorm > (1.0 - 2.2204460492503131e-16) / (2.0 * 2.2250738585072014e-308) * smin {
                    scale = 1.0 / bnorm;
                }
            }

            let temp = scale / smin;
            gsl_vector_set(x, 0, temp * bval1);
            gsl_vector_set(x, 1, temp * bval2);
            *xnorm = temp * bnorm;
            *s = scale;
            return G
use num_complex::Complex64;
use std::f64::consts::*;
use std::cmp::{max, min};
use std::f64;

const GSL_SCHUR_SMLNUM: f64 = 2.0 * f64::MIN_POSITIVE;
const GSL_SCHUR_BIGNUM: f64 = (1.0 - f64::EPSILON) / GSL_SCHUR_SMLNUM;

/// Compute the eigenvalues of a 2-by-2 generalized block.
///
/// # Arguments
///
/// * `a` - 2x2 matrix represented as [[a11, a12], [a21, a22]]
/// * `b` - 2x2 upper triangular matrix represented as [[b11, b12], [0, b22]]
/// 
/// # Returns
///
/// Returns a tuple containing:
/// * wr1, wr2: real parts of eigenvalues
/// * wi: imaginary part (0 if real eigenvalues)
/// * scale1, scale2: scaling factors
pub fn gsl_schur_gen_eigvals(
    a: [[f64; 2]; 2],
    b: [[f64; 2]; 2],
) -> (f64, f64, f64, f64, f64) {
    let safemin = f64::MIN_POSITIVE * 1.0e2;
    let safemax = 1.0 / safemin;
    let rtmin = safemin.sqrt();
    let rtmax = 1.0 / rtmin;

    // scale A
    let anorm = max(
        max(a[0][0].abs() + a[1][0].abs(), a[0][1].abs() + a[1][1].abs()),
        safemin,
    );
    let ascale = 1.0 / anorm;
    let a11 = ascale * a[0][0];
    let a12 = ascale * a[0][1];
    let a21 = ascale * a[1][0];
    let a22 = ascale * a[1][1];

    // perturb B if necessary to ensure non-singularity
    let mut b11 = b[0][0];
    let mut b12 = b[0][1];
    let mut b22 = b[1][1];
    let bmin = rtmin * max(b11.abs(), max(b12.abs(), max(b22.abs(), rtmin)));
    if b11.abs() < bmin {
        b11 = if b11 >= 0.0 { bmin } else { -bmin };
    }
    if b22.abs() < bmin {
        b22 = if b22 >= 0.0 { bmin } else { -bmin };
    }

    // scale B
    let bnorm = max(b11.abs(), max(b12.abs() + b22.abs(), safemin));
    let bsize = max(b11.abs(), b22.abs());
    let bscale = 1.0 / bsize;
    let b11 = b11 * bscale;
    let b12 = b12 * bscale;
    let b22 = b22 * bscale;

    // compute larger eigenvalue
    let binv11 = 1.0 / b11;
    let binv22 = 1.0 / b22;
    let s1 = a11 * binv11;
    let s2 = a22 * binv22;

    let (as12, as22, abi22, pp, shift) = if s1.abs() <= s2.abs() {
        let as12 = a12 - s1 * b12;
        let as22 = a22 - s1 * b22;
        let ss = a21 * (binv11 * binv22);
        let abi22 = as22 * binv22 - ss * b12;
        let pp = 0.5 * abi22;
        (as12, as22, abi22, pp, s1)
    } else {
        let as12 = a12 - s2 * b12;
        let as11 = a11 - s2 * b11;
        let ss = a21 * (binv11 * binv22);
        let abi22 = -ss * b12;
        let pp = 0.5 * (as11 * binv11 + abi22);
        (as12, as11, abi22, pp, s2)
    };

    let qq = ss * as12;
    let (discr, r) = if (pp * rtmin).abs() >= 1.0 {
        let discr = (rtmin * pp).powi(2) + qq * safemin;
        (discr, discr.abs().sqrt() * rtmax)
    } else if pp.powi(2) + qq.abs() <= safemin {
        let discr = (rtmax * pp).powi(2) + qq * safemax;
        (discr, discr.abs().sqrt() * rtmin)
    } else {
        let discr = pp.powi(2) + qq;
        (discr, discr.abs().sqrt())
    };

    let (wr1, wr2, wi) = if discr >= 0.0 || r == 0.0 {
        let sum = pp + if pp >= 0.0 { r } else { -r };
        let diff = pp - if pp >= 0.0 { r } else { -r };
        let wbig = shift + sum;
        let wsmall = shift + diff;

        // compute smaller eigenvalue
        let wsmall = if 0.5 * wbig.abs() > max(wsmall.abs(), safemin) {
            let wdet = (a11 * a22 - a12 * a21) * (binv11 * binv22);
            wdet / wbig
        } else {
            wsmall
        };

        // choose (real) eigenvalue closest to 2,2 element of AB^{-1}
        if pp > abi22 {
            (min(wbig, wsmall), max(wbig, wsmall), 0.0)
        } else {
            (max(wbig, wsmall), min(wbig, wsmall), 0.0)
        }
    } else {
        // complex eigenvalues
        (shift + pp, shift + pp, r)
    };

    // compute scaling
    let fuzzy1 = 1.0 + 1.0e-5;
    let c1 = bsize * (safemin * max(1.0, ascale));
    let c2 = safemin * max(1.0, bnorm);
    let c3 = bsize * safemin;
    let c4 = if ascale <= 1.0 && bsize <= 1.0 {
        min(1.0, (ascale / safemin) * bsize)
    } else {
        1.0
    };
    let c5 = if ascale <= 1.0 || bsize <= 1.0 {
        min(1.0, ascale * bsize)
    } else {
        1.0
    };

    // scale first eigenvalue
    let wabs = wr1.abs() + wi.abs();
    let wsize = max(
        safemin,
        max(
            c1,
            max(
                fuzzy1 * (wabs * c2 + c3),
                min(c4, 0.5 * max(wabs, c5)),
            ),
        ),
    );

    let (wr1, wi, wr2, scale1, scale2) = if wsize != 1.0 {
        let wscale = 1.0 / wsize;
        let scale1 = if wsize > 1.0 {
            (max(ascale, bsize) * wscale) * min(ascale, bsize)
        } else {
            (min(ascale, bsize) * wscale) * max(ascale, bsize)
        };

        let wr1 = wr1 * wscale;
        let (wi, wr2, scale2) = if wi != 0.0 {
            (wi * wscale, wr1, scale1)
        } else {
            (wi, wr2, scale1)
        };

        (wr1, wi, wr2, scale1, scale2)
    } else {
        let scale1 = ascale * bsize;
        (wr1, wi, wr2, scale1, scale1)
    };

    // scale second eigenvalue if real
    let (wr2, scale2) = if wi == 0.0 {
        let wsize = max(
            safemin,
            max(
                c1,
                max(
                    fuzzy1 * (wr2.abs() * c2 + c3),
                    min(c4, 0.5 * max(wr2.abs(), c5)),
                ),
            ),
        );

        if wsize != 1.0 {
            let wscale = 1.0 / wsize;
            let scale2 = if wsize > 1.0 {
                (max(ascale, bsize) * wscale) * min(ascale, bsize)
            } else {
                (min(ascale, bsize) * wscale) * max(ascale, bsize)
            };
            (wr2 * wscale, scale2)
        } else {
            (wr2, ascale * bsize)
        }
    } else {
        (wr2, scale2)
    };

    (wr1, wr2, wi, scale1, scale2)
}

/// Solve the equation for real eigenvalues
pub fn gsl_schur_solve_equation(
    ca: f64,
    a: [[f64; 2]; 2],
    z: f64,
    d1: f64,
    d2: f64,
    b: [f64; 2],
    smin: f64,
) -> ([f64; 2], f64, f64, f64) {
    let n = 2; // since we're working with 2x2 matrices
    let mut scale = 1.0;
    let mut x = [0.0; 2];
    let mut xnorm = 0.0;

    if n == 1 {
        // 1x1 system
        let c = ca * a[0][0] - z * d1;
        let cnorm = c.abs();

        let c = if cnorm < smin { smin } else { c };
        let cnorm = if cnorm < smin { smin } else { cnorm };

        // check scaling
        let bnorm = b[0].abs();
        if cnorm < 1.0 && bnorm > 1.0 {
            if bnorm > GSL_SCHUR_BIGNUM * cnorm {
                scale = 1.0 / bnorm;
            }
        }

        x[0] = b[0] * scale / c;
        xnorm = x[0].abs();
    } else {
        // 2x2 system
        let mut cr = [[0.0; 2]; 2];
        cr[0][0] = ca * a[0][0] - z * d1;
        cr[1][1] = ca * a[1][1] - z * d2;
        cr[0][1] = ca * a[1][0];
        cr[1][0] = ca * a[0][1];

        // find largest element
        let mut cmax = 0.0;
        let mut icmax = 0;
        for j in 0..4 {
            let val = cr[j/2][j%2].abs();
            if val > cmax {
                cmax = val;
                icmax = j;
            }
        }

        // if norm(C) < smin, use smin*I
        if cmax < smin {
            let bnorm = max(b[0].abs(), b[1].abs());
            if smin < 1.0 && bnorm > 1.0 {
                if bnorm > GSL_SCHUR_BIGNUM * smin {
                    scale = 1.0 / bnorm;
                }
            }
            let temp = scale / smin;
            x = [temp * b[0], temp * b[1]];
            xnorm = temp * bnorm;
            return (x, xnorm, scale, 0.0);
        }

        // Gaussian elimination with complete pivoting
        let ipivot = [
            [0, 1, 2, 3],
            [1, 0, 3, 2],
            [2, 3, 0, 1],
            [3, 2, 1, 0],
        ];
        let rswap = [false, true, false, true];
        let zswap = [false, false, true, true];

        let ur11 = cr[icmax/2][icmax%2];
        let cr21 = cr[ipivot[icmax][1]/2][ipivot[icmax][1]%2];
        let ur12 = cr[ipivot[icmax][2]/2][ipivot[icmax][2]%2];
        let cr22 = cr[ipivot[icmax][3]/2][ipivot[icmax][3]%2];
        let ur11r = 1.0 / ur11;
        let lr21 = ur11r * cr21;
        let mut ur22 = cr22 - ur12 * lr21;

        // if smaller pivot < smin, use smin
        if ur22.abs() < smin {
            ur22 = smin;
        }

        let (b1, b2) = if rswap[icmax] {
            (b[1], b[0])
        } else {
            (b[0], b[1])
        };

        let b2 = b2 - lr21 * b1;
        let bbnd = max((b1 * (ur22 * ur11r)).abs(), b2.abs());
        if bbnd > 1.0 && ur22.abs() < 1.0 {
            if bbnd >= GSL_SCHUR_BIGNUM * ur22.abs() {
                scale = 1.0 / bbnd;
            }
        }

        let x2 = (b2 * scale) / ur22;
        let x1 = (scale * b1) * ur11r - x2 * (ur11r * ur12);
        if zswap[icmax] {
            x = [x2, x1];
        } else {
            x = [x1, x2];
        }

        xnorm = max(x[0].abs(), x[1].abs());

        // further scaling if norm(A) norm(X) > overflow
        if xnorm > 1.0 && cmax > 1.0 {
            if xnorm > GSL_SCHUR_BIGNUM / cmax {
                let temp = cmax / GSL_SCHUR_BIGNUM;
                x[0] *= temp;
                x[1] *= temp;
                xnorm *= temp;
                scale *= temp;
            }
        }
    }

    (x, xnorm, scale, 0.0)
}

/// Solve the equation for complex eigenvalues
pub fn gsl_schur_solve_equation_z(
    ca: f64,
    a: [[f64; 2]; 2],
    z: Complex64,
    d1: f64,
    d2: f64,
    b: [Complex64; 2],
    smin: f64,
) -> ([Complex64; 2], f64, f64, f64) {
    let n = 2; // since we're working with 2x2 matrices
    let mut scale = 1.0;
    let mut x = [Complex64::new(0.0, 0.0); 2];
    let mut xnorm = 0.0;

    if n == 1 {
        // 1x1 system
        let cr = ca * a[0][0] - z.re * d1;
        let ci = -z.im * d1;
        let cnorm = cr.abs() + ci.abs();

        let (cr, ci) = if cnorm < smin {
            (smin, 0.0)
        } else {
            (cr, ci)
        };
        let cnorm = if cnorm < smin { smin } else { cnorm };

        // check scaling
        let bnorm = b[0].norm();
        if cnorm < 1.0 && bnorm > 1.0 {
            if bnorm > GSL_SCHUR_BIGNUM * cnorm {
                scale = 1.0 / bnorm;
            }
        }

        x[0] = Complex64::new(scale * b[0].re, scale * b[0].im) / Complex64::new(cr, ci);
        xnorm = x[0].norm();
    } else {
        // 2x2 system
        let mut cr = [[0.0; 2]; 2];
        let mut ci = [[0.0; 2]; 2];
        cr[0][0] = ca * a[0][0] - z.re * d1;
        cr[1][1] = ca * a[1][1] - z.re * d2;
        cr[0][1] = ca * a[1][0];
        cr[1][0] = ca * a[0][1];
        ci[0][0] = -z.im * d1;
        ci[0][1] = 0.0;
        ci[1][0] = 0.0;
        ci[1][1] = -z.im * d2;

        // find largest element
        let mut cmax = 0.0;
        let mut icmax = 0;
        for j in 0..4 {
            let val = cr[j/2][j%2].abs() + ci[j/2][j%2].abs();
            if val > cmax {
                cmax = val;
                icmax = j;
            }
        }

        // if norm(C) < smin, use smin*I
        if cmax < smin {
            let bnorm = max(b[0].norm(), b[1].norm());
            if smin < 1.0 && bnorm > 1.0 {
                if bnorm > GSL_SCHUR_BIGNUM * smin {
                    scale = 1.0 / bnorm;
                }
            }
            let temp = scale /
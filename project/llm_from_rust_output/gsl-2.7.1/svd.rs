use gsl::{
    blas::{daxpy, ddot, dnrm2, dscal, CblasTranspose},
    error::{GslError, GslResult},
    linalg::{householder_hm, householder_hm1, householder_transform},
    matrix::{Matrix, MatrixView, MatrixViewMut},
    vector::{Vector, VectorView, VectorViewMut},
    Value,
};
use libc::{c_double, c_int};
use std::f64;

const GSL_SUCCESS: c_int = 0;
const GSL_EBADLEN: c_int = 19;
const GSL_ENOTSQR: c_int = 20;
const GSL_EUNIMPL: c_int = 24;
const GSL_EMAXITER: c_int = 11;
const GSL_ETOL: c_int = 14;

fn chop_small_elements(d: &mut Vector, f: &mut Vector) {
    let n = d.len();
    let mut d_i = d.get(0);
    for i in 0..n - 1 {
        let f_i = f.get(i);
        let d_ip1 = d.get(i + 1);
        if f_i.abs() < f64::EPSILON * (d_i.abs() + d_ip1.abs()) {
            f.set(i, 0.0);
        }
        d_i = d_ip1;
    }
}

fn trailing_eigenvalue(d: &Vector, f: &Vector) -> c_double {
    let n = d.len();
    let da = d.get(n - 2);
    let db = d.get(n - 1);
    let fa = if n > 2 { f.get(n - 3) } else { 0.0 };
    let fb = f.get(n - 2);

    let ta = da * da + fa * fa;
    let tb = db * db + fb * fb;
    let tab = da * fb;
    let dt = (ta - tb) / 2.0;
    let s = ta + tb;
    let da2 = da * da;
    let db2 = db * db;
    let fa2 = fa * fa;
    let fb2 = fb * fb;
    let p = da2 * db2 + fa2 * db2 + fa2 * fb2;
    let d_val = (dt * dt + tab * tab).sqrt();

    let r1 = s / 2.0 + d_val;
    if dt >= 0.0 {
        if r1 > 0.0 {
            p / r1
        } else {
            0.0
        }
    } else {
        r1
    }
}

fn create_schur(d0: c_double, f0: c_double, d1: c_double, c: &mut c_double, s: &mut c_double) {
    let mut apq = 2.0 * d0 * f0;
    if d0 == 0.0 || f0 == 0.0 {
        *c = 1.0;
        *s = 0.0;
        return;
    }

    let mut scale = 1.0;
    if d0.abs() < f64::MIN_POSITIVE || d0.abs() > f64::MAX
        || f0.abs() < f64::MIN_POSITIVE || f0.abs() > f64::MAX
        || d1.abs() < f64::MIN_POSITIVE || d1.abs() > f64::MAX
    {
        let d0_exp = d0.log2().floor() as i32;
        let f0_exp = f0.log2().floor() as i32;
        scale = 2.0f64.powi(-(d0_exp + f0_exp) / 4);
        let d0 = d0 * scale;
        let f0 = f0 * scale;
        let d1 = d1 * scale;
        apq = 2.0 * d0 * f0;
    }

    if apq != 0.0 {
        let tau = (f0 * f0 + (d1 + d0) * (d1 - d0)) / apq;
        let t = if tau >= 0.0 {
            1.0 / (tau + (1.0 + tau * tau).sqrt())
        } else {
            -1.0 / (-tau + (1.0 + tau * tau).sqrt())
        };
        *c = 1.0 / (1.0 + t * t).sqrt();
        *s = t * *c;
    } else {
        *c = 1.0;
        *s = 0.0;
    }
}

fn svd2(
    d: &mut Vector,
    f: &mut Vector,
    u: &mut Matrix,
    v: &mut Matrix,
) -> GslResult<()> {
    let m = u.size1();
    let n = v.size1();
    let mut c = 0.0;
    let mut s = 0.0;
    let mut a11 = 0.0;
    let mut a12 = 0.0;
    let mut a21 = 0.0;
    let mut a22 = 0.0;

    let d0 = d.get(0);
    let f0 = f.get(0);
    let d1 = d.get(1);

    if d0 == 0.0 {
        gsl_linalg_givens(f0, d1, &mut c, &mut s);
        d.set(0, c * f0 - s * d1);
        f.set(0, s * f0 + c * d1);
        d.set(1, 0.0);

        for i in 0..m {
            let u_ip = u.get(i, 0);
            let u_iq = u.get(i, 1);
            u.set(i, 0, c * u_ip - s * u_iq);
            u.set(i, 1, s * u_ip + c * u_iq);
        }

        v.swap_columns(0, 1)?;
        return Ok(());
    } else if d1 == 0.0 {
        gsl_linalg_givens(d0, f0, &mut c, &mut s);
        d.set(0, d0 * c - f0 * s);
        f.set(0, 0.0);

        for i in 0..n {
            let v_ip = v.get(i, 0);
            let v_iq = v.get(i, 1);
            v.set(i, 0, c * v_ip - s * v_iq);
            v.set(i, 1, s * v_ip + c * v_iq);
        }

        return Ok(());
    } else {
        create_schur(d0, f0, d1, &mut c, &mut s);
        a11 = c * d0 - s * f0;
        a21 = -s * d1;
        a12 = s * d0 + c * f0;
        a22 = c * d1;

        for i in 0..n {
            let v_ip = v.get(i, 0);
            let v_iq = v.get(i, 1);
            v.set(i, 0, c * v_ip - s * v_iq);
            v.set(i, 1, s * v_ip + c * v_iq);
        }

        if (a11 * a11 + a21 * a21).sqrt() < (a12 * a12 + a22 * a22).sqrt() {
            std::mem::swap(&mut a11, &mut a12);
            std::mem::swap(&mut a21, &mut a22);
            v.swap_columns(0, 1)?;
        }

        gsl_linalg_givens(a11, a21, &mut c, &mut s);
        d.set(0, c * a11 - s * a21);
        f.set(0, c * a12 - s * a22);
        d.set(1, s * a12 + c * a22);

        for i in 0..m {
            let u_ip = u.get(i, 0);
            let u_iq = u.get(i, 1);
            u.set(i, 0, c * u_ip - s * u_iq);
            u.set(i, 1, s * u_ip + c * u_iq);
        }
    }

    Ok(())
}

fn chase_out_intermediate_zero(
    d: &mut Vector,
    f: &mut Vector,
    u: &mut Matrix,
    k0: usize,
) -> GslResult<()> {
    let m = u.size1();
    let n = d.len();
    let mut c = 0.0;
    let mut s = 0.0;
    let mut x = f.get(k0);
    let mut y = d.get(k0 + 1);

    for k in k0..n - 1 {
        gsl_linalg_givens(y, -x, &mut c, &mut s);

        for i in 0..m {
            let u_ip = u.get(i, k0);
            let u_iq = u.get(i, k + 1);
            u.set(i, k0, c * u_ip - s * u_iq);
            u.set(i, k + 1, s * u_ip + c * u_iq);
        }

        d.set(k + 1, s * x + c * y);
        if k == k0 {
            f.set(k, c * x - s * y);
        }

        if k < n - 2 {
            let z = f.get(k + 1);
            f.set(k + 1, c * z);
            x = -s * z;
            y = d.get(k + 2);
        }
    }

    Ok(())
}

fn chase_out_trailing_zero(
    d: &mut Vector,
    f: &mut Vector,
    v: &mut Matrix,
) -> GslResult<()> {
    let n = v.size1();
    let len = d.len();
    let mut c = 0.0;
    let mut s = 0.0;
    let mut x = d.get(len - 2);
    let mut y = f.get(len - 2);

    for k in (1..len).rev() {
        gsl_linalg_givens(x, y, &mut c, &mut s);

        for i in 0..n {
            let v_ip = v.get(i, k - 1);
            let v_iq = v.get(i, len - 1);
            v.set(i, k - 1, c * v_ip - s * v_iq);
            v.set(i, len - 1, s * v_ip + c * v_iq);
        }

        d.set(k - 1, c * x - s * y);
        if k - 1 == len - 2 {
            f.set(k - 1, s * x + c * y);
        }

        if k > 1 {
            let z = f.get(k - 2);
            f.set(k - 2, c * z);
            x = d.get(k - 2);
            y = s * z;
        }
    }

    Ok(())
}

fn qrstep(
    d: &mut Vector,
    f: &mut Vector,
    u: &mut Matrix,
    v: &mut Matrix,
) -> GslResult<()> {
    let m = u.size1();
    let n = v.size1();
    let len = d.len();

    if len == 1 {
        return Ok(());
    }
    if len == 2 {
        return svd2(d, f, u, v);
    }

    for i in 0..len - 1 {
        if d.get(i) == 0.0 {
            return chase_out_intermediate_zero(d, f, u, i);
        }
    }

    let d_nm1 = d.get(len - 1);
    if d_nm1 == 0.0 {
        return chase_out_trailing_zero(d, f, v);
    }

    let d0 = d.get(0);
    let f0 = f.get(0);
    let d1 = d.get(1);
    let mu = trailing_eigenvalue(d, f);

    let mut y = d0 * d0 - mu;
    let mut z = d0 * f0;
    let mut ak = 0.0;
    let mut bk = 0.0;
    let mut zk = 0.0;
    let mut ap = d0;
    let mut bp = f0;
    let mut aq = d1;

    for k in 0..len - 1 {
        let mut c = 0.0;
        let mut s = 0.0;
        gsl_linalg_givens(y, z, &mut c, &mut s);

        for i in 0..n {
            let v_ip = v.get(i, k);
            let v_iq = v.get(i, k + 1);
            v.set(i, k, c * v_ip - s * v_iq);
            v.set(i, k + 1, s * v_ip + c * v_iq);
        }

        let bk1 = c * bk - s * z;
        let ap1 = c * ap - s * bp;
        let bp1 = s * ap + c * bp;
        let zp1 = -s * aq;
        let aq1 = c * aq;

        if k > 0 {
            f.set(k - 1, bk1);
        }

        ak = ap1;
        bk = bp1;
        zk = zp1;
        ap = aq1;
        bp = if k < len - 2 {
            f.get(k + 1)
        } else {
            0.0
        };

        y = ak;
        z = zk;

        gsl_linalg_givens(y, z, &mut c, &mut s);

        for i in 0..m {
            let u_ip = u.get(i, k);
            let u_iq = u.get(i, k + 1);
            u.set(i, k, c * u_ip - s * u_iq);
            u.set(i, k + 1, s * u_ip + c * u_iq);
        }

        let ak1 = c * ak - s * zk;
        let bk1 = c * bk - s * ap;
        let zk1 = -s * bp;
        let ap1 = s * bk + c * ap;
        let bp1 = c * bp;

        d.set(k, ak1);
        ak = ak1;
        bk = bk1;
        zk = zk1;
        ap = ap1;
        bp = bp1;
        aq = if k < len - 2 {
            d.get(k + 2)
        } else {
            0.0
        };

        y = bk;
        z = zk;
    }

    f.set(len - 2, bk);
    d.set(len - 1, ap);

    Ok(())
}

pub fn gsl_linalg_SV_decomp(
    a: &mut Matrix,
    v: &mut Matrix,
    s: &mut Vector,
    work: &mut Vector,
) -> GslResult<()> {
    let m = a.size1();
    let n = a.size2();
    let k = std::cmp::min(m, n);

    if m < n {
        return Err(GslError::new(GSL_EUNIMPL, "svd of MxN matrix, M<N, is not implemented"));
    }
    if v.size1() != n {
        return Err(GslError::new(GSL_EBADLEN, "square matrix V must match second dimension of matrix A"));
    }
    if v.size1() != v.size2() {
        return Err(GslError::new(GSL_ENOTSQR, "matrix V must be square"));
    }
    if s.len() != n {
        return Err(GslError::new(GSL_EBADLEN, "length of vector S must match second dimension of matrix A"));
    }
    if work.len() != n {
        return Err(GslError::new(GSL_EBADLEN, "length of workspace must match second dimension of matrix A"));
    }

    if n == 1 {
        let column = a.column(0);
        let norm = dnrm2(&column);
        s.set(0, norm);
        v.set(0, 0, 1.0);
        if norm != 0.0 {
            dscal(1.0 / norm, &mut a.column_mut(0));
        }
        return Ok(());
    }

    let mut f = work.subvector(0, k - 1);
    gsl_linalg_bidiag_decomp(a, s, &mut f)?;
    gsl_linalg_bidiag_unpack2(a, s, &mut f, v)?;
    chop_small_elements(s, &mut f);

    let mut b = n - 1;
    let mut iter = 0;
    let max_iter = 100 * n;

    while b > 0 {
        let fbm1 = f.get(b - 1);
        if fbm1 == 0.0 || fbm1.is_nan() {
            b -= 1;
        } else {
            let mut a_val = b - 1;
            while a_val > 0 {
                let fam1 = f.get(a_val - 1);
                if fam1 == 0.0 || fam1.is_nan() {
                    break;
                }
                a_val -= 1;
            }

            iter += 1;
            if iter > max_iter {
                return Err(GslError::new(GSL_EMAXITER, "SVD decomposition failed to converge"));
            }

            let n_block = b - a_val + 1;
            let mut s_block = s.subvector(a_val, n_block);
            let mut f_block = f.subvector(a_val, n_block - 1);
            let mut u_block = a.submatrix(0, a_val, m, n_block);
            let mut v_block = v.submatrix(0, a_val, n, n_block);

            let mut rescale = false;
            let mut scale = 1.0;
            let mut norm = 0.0;

            for i in 0..n_block {
                let s_i = s_block.get(i).abs();
                if s_i > norm {
                    norm
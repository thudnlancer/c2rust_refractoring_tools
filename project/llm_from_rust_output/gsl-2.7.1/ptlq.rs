use gsl::{
    blas::{CblasDiag, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    matrix::{Matrix, MatrixView},
    permutation::Permutation,
    vector::{Vector, VectorView},
};

pub fn gsl_linalg_givens(a: f64, b: f64) -> (f64, f64) {
    if b == 0.0 {
        (1.0, 0.0)
    } else if b.abs() > a.abs() {
        let t = -a / b;
        let s1 = 1.0 / (1.0 + t * t).sqrt();
        (s1 * t, s1)
    } else {
        let t = -b / a;
        let c1 = 1.0 / (1.0 + t * t).sqrt();
        (c1, c1 * t)
    }
}

pub fn gsl_linalg_givens_gv(v: &mut Vector, i: usize, j: usize, c: f64, s: f64) {
    let vi = v.get(i);
    let vj = v.get(j);
    v.set(i, c * vi - s * vj);
    v.set(j, s * vi + c * vj);
}

pub fn apply_givens_lq(
    m: usize,
    n: usize,
    q: &mut Matrix,
    l: &mut Matrix,
    i: usize,
    j: usize,
    c: f64,
    s: f64,
) {
    for k in 0..m {
        let qik = q.get(i, k);
        let qjk = q.get(j, k);
        q.set(i, k, qik * c - qjk * s);
        q.set(j, k, qik * s + qjk * c);
    }

    let start = i.min(j);
    for k in start..n {
        let lki = l.get(k, i);
        let lkj = l.get(k, j);
        l.set(k, i, c * lki - s * lkj);
        l.set(k, j, s * lki + c * lkj);
    }
}

pub fn gsl_linalg_PTLQ_decomp(
    a: &mut Matrix,
    tau: &mut Vector,
    p: &mut Permutation,
    signum: &mut i32,
    norm: &mut Vector,
) -> GslResult<()> {
    let n = a.size1();
    let m = a.size2();

    if tau.size() != m.min(n) {
        return Err(GslError::BadLen("size of tau must be MIN(M,N)"));
    }
    if p.size() != n {
        return Err(GslError::BadLen("permutation size must be N"));
    }
    if norm.size() != n {
        return Err(GslError::BadLen("norm size must be N"));
    }

    *signum = 1;
    p.init();

    for i in 0..n {
        let c = a.row(i);
        let x = c.nrm2()?;
        norm.set(i, x);
    }

    for i in 0..m.min(n) {
        let mut max_norm = norm.get(i);
        let mut kmax = i;

        for j in (i + 1)..n {
            let x = norm.get(j);
            if x > max_norm {
                max_norm = x;
                kmax = j;
            }
        }

        if kmax != i {
            a.swap_rows(i, kmax)?;
            p.swap(i, kmax)?;
            norm.swap_elements(i, kmax)?;
            *signum = -*signum;
        }

        let mut c = a.subrow(i, i, m - i);
        let tau_i = c.householder_transform()?;
        tau.set(i, tau_i);

        if i + 1 < n {
            let mut m_view = a.submatrix(i + 1, i, n - (i + 1), m - i);
            tau_i.householder_mh(&c, &mut m_view)?;
        }

        if i + 1 < m {
            for j in (i + 1)..n {
                let x = norm.get(j);
                if x > 0.0 {
                    let temp = a.get(j, i) / x;
                    let y = if temp.abs() >= 1.0 {
                        0.0
                    } else {
                        x * (1.0 - temp * temp).sqrt()
                    };

                    if (y / x).abs() < f64::EPSILON.sqrt() * 20.0 {
                        let c_view = a.subrow(j, i + 1, m - (i + 1));
                        y = c_view.nrm2()?;
                    }
                    norm.set(j, y);
                }
            }
        }
    }

    Ok(())
}

pub fn gsl_linalg_PTLQ_decomp2(
    a: &Matrix,
    q: &mut Matrix,
    r: &mut Matrix,
    tau: &mut Vector,
    p: &mut Permutation,
    signum: &mut i32,
    norm: &mut Vector,
) -> GslResult<()> {
    let n = a.size1();
    let m = a.size2();

    if q.size1() != m || q.size2() != m {
        return Err(GslError::BadLen("q must be M x M"));
    }
    if r.size1() != n || r.size2() != m {
        return Err(GslError::BadLen("r must be N x M"));
    }
    if tau.size() != m.min(n) {
        return Err(GslError::BadLen("size of tau must be MIN(M,N)"));
    }
    if p.size() != n {
        return Err(GslError::BadLen("permutation size must be N"));
    }
    if norm.size() != n {
        return Err(GslError::BadLen("norm size must be N"));
    }

    r.copy_from(a)?;
    gsl_linalg_PTLQ_decomp(r, tau, p, signum, norm)?;
    r.lq_unpack(tau, q, r)?;

    Ok(())
}

pub fn gsl_linalg_PTLQ_solve_T(
    qr: &Matrix,
    tau: &Vector,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> GslResult<()> {
    if qr.size1() != qr.size2() {
        return Err(GslError::NotSquare("QR matrix must be square"));
    }
    if qr.size2() != p.size() {
        return Err(GslError::BadLen("matrix size must match permutation size"));
    }
    if qr.size2() != b.size() {
        return Err(GslError::BadLen("matrix size must match b size"));
    }
    if qr.size1() != x.size() {
        return Err(GslError::BadLen("matrix size must match solution size"));
    }

    x.copy_from(b)?;
    gsl_linalg_PTLQ_svx_T(qr, tau, p, x)
}

pub fn gsl_linalg_PTLQ_svx_T(
    lq: &Matrix,
    tau: &Vector,
    p: &Permutation,
    x: &mut Vector,
) -> GslResult<()> {
    if lq.size1() != lq.size2() {
        return Err(GslError::NotSquare("LQ matrix must be square"));
    }
    if lq.size2() != p.size() {
        return Err(GslError::BadLen("matrix size must match permutation size"));
    }
    if lq.size1() != x.size() {
        return Err(GslError::BadLen("matrix size must match solution size"));
    }

    lq.lq_vecQT(tau, x)?;
    x.trsv(CblasLower, CblasTrans, CblasNonUnit, lq)?;
    p.inverse_permute(x)?;

    Ok(())
}

pub fn gsl_linalg_PTLQ_LQsolve_T(
    q: &Matrix,
    l: &Matrix,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> GslResult<()> {
    if q.size1() != q.size2() || l.size1() != l.size2() {
        return Err(GslError::NotSquare("Q and L must be square"));
    }
    if q.size1() != p.size() || q.size1() != l.size1() || q.size1() != b.size() {
        return Err(GslError::BadLen("matrix sizes must match"));
    }

    x.gemv(CblasNoTrans, 1.0, q, b, 0.0)?;
    x.trsv(CblasLower, CblasTrans, CblasNonUnit, l)?;
    p.inverse_permute(x)?;

    Ok(())
}

pub fn gsl_linalg_PTLQ_Lsolve_T(
    lq: &Matrix,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> GslResult<()> {
    if lq.size1() != lq.size2() {
        return Err(GslError::NotSquare("LQ matrix must be square"));
    }
    if lq.size1() != b.size() {
        return Err(GslError::BadLen("matrix size must match b size"));
    }
    if lq.size2() != x.size() {
        return Err(GslError::BadLen("matrix size must match x size"));
    }
    if p.size() != x.size() {
        return Err(GslError::BadLen("permutation size must match x size"));
    }

    x.copy_from(b)?;
    x.trsv(CblasLower, CblasTrans, CblasNonUnit, lq)?;
    p.inverse_permute(x)?;

    Ok(())
}

pub fn gsl_linalg_PTLQ_Lsvx_T(
    lq: &Matrix,
    p: &Permutation,
    x: &mut Vector,
) -> GslResult<()> {
    if lq.size1() != lq.size2() {
        return Err(GslError::NotSquare("LQ matrix must be square"));
    }
    if lq.size2() != x.size() {
        return Err(GslError::BadLen("matrix size must match x size"));
    }
    if p.size() != x.size() {
        return Err(GslError::BadLen("permutation size must match x size"));
    }

    x.trsv(CblasLower, CblasTrans, CblasNonUnit, lq)?;
    p.inverse_permute(x)?;

    Ok(())
}

pub fn gsl_linalg_PTLQ_update(
    q: &mut Matrix,
    l: &mut Matrix,
    p: &Permutation,
    v: &Vector,
    w: &mut Vector,
) -> GslResult<()> {
    if q.size1() != q.size2() || l.size1() != l.size2() {
        return Err(GslError::NotSquare("Q and L must be square"));
    }
    if l.size1() != q.size2() || v.size() != q.size2() || w.size() != q.size2() {
        return Err(GslError::BadLen("matrix and vector sizes must match"));
    }

    let n = q.size1();
    let m = q.size2();

    for k in (1..m).rev() {
        let wk = w.get(k);
        let wkm1 = w.get(k - 1);
        let (c, s) = gsl_linalg_givens(wkm1, wk);
        gsl_linalg_givens_gv(w, k - 1, k, c, s);
        apply_givens_lq(m, n, q, l, k - 1, k, c, s);
    }

    let w0 = w.get(0);
    for j in 0..n {
        let lj0 = l.get(j, 0);
        let p_j = p.get(j);
        let vj = v.get(p_j);
        l.set(j, 0, lj0 + w0 * vj);
    }

    for k in 1..n {
        let diag = l.get(k - 1, k - 1);
        let offdiag = l.get(k - 1, k);
        let (c, s) = gsl_linalg_givens(diag, offdiag);
        apply_givens_lq(m, n, q, l, k - 1, k, c, s);
    }

    Ok(())
}
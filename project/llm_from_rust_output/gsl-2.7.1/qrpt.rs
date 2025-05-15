use gsl::{
    blas::{CblasDiag, CblasTranspose, CblasUpper},
    error::Result,
    linalg::{householder, QR},
    matrix::Matrix,
    permutation::Permutation,
    vector::Vector,
};

pub fn qrpt_decomp(
    a: &mut Matrix,
    tau: &mut Vector,
    p: &mut Permutation,
    signum: &mut i32,
    norm: &mut Vector,
) -> Result<()> {
    let m = a.size1();
    let n = a.size2();
    let min_mn = m.min(n);

    if tau.size() != min_mn {
        return Err(gsl::error::Error::BadLen);
    }
    if p.size() != n {
        return Err(gsl::error::Error::BadLen);
    }
    if norm.size() != n {
        return Err(gsl::error::Error::BadLen);
    }

    *signum = 1;
    p.init();

    for i in 0..n {
        let col = a.column(i);
        let x = col.nrm2();
        norm.set(i, x);
    }

    for i in 0..min_mn {
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
            a.swap_columns(i, kmax)?;
            p.swap(i, kmax)?;
            norm.swap_elements(i, kmax)?;
            *signum = -*signum;
        }

        let mut c_full = a.column_mut(i);
        let mut c = c_full.subvector(i, m - i);
        let tau_i = householder::transform(&mut c)?;
        tau.set(i, tau_i);

        if i + 1 < n {
            let mut m_sub = a.submatrix_mut(i, i + 1, m - i, n - (i + 1));
            householder::hm(tau_i, &c, &mut m_sub)?;
        }

        if i + 1 < m {
            for j in (i + 1)..n {
                let x = norm.get(j);
                if x > 0.0 {
                    let temp = a.get(i, j) / x;
                    let y = if temp.abs() >= 1.0 {
                        0.0
                    } else {
                        x * (1.0 - temp * temp).sqrt()
                    };

                    if (y / x).abs() < f64::EPSILON.sqrt() * 20.0 {
                        let mut c_full = a.column_mut(j);
                        let c = c_full.subvector(i + 1, m - (i + 1));
                        y = c.nrm2();
                    }
                    norm.set(j, y);
                }
            }
        }
    }

    Ok(())
}

pub fn qrpt_decomp2(
    a: &Matrix,
    q: &mut Matrix,
    r: &mut Matrix,
    tau: &mut Vector,
    p: &mut Permutation,
    signum: &mut i32,
    norm: &mut Vector,
) -> Result<()> {
    let m = a.size1();
    let n = a.size2();

    if q.size1() != m || q.size2() != m {
        return Err(gsl::error::Error::BadLen);
    }
    if r.size1() != m || r.size2() != n {
        return Err(gsl::error::Error::BadLen);
    }
    if tau.size() != m.min(n) {
        return Err(gsl::error::Error::BadLen);
    }
    if p.size() != n {
        return Err(gsl::error::Error::BadLen);
    }
    if norm.size() != n {
        return Err(gsl::error::Error::BadLen);
    }

    r.copy_from(a)?;
    qrpt_decomp(r, tau, p, signum, norm)?;
    QR::unpack(r, tau, q, r)?;

    Ok(())
}

pub fn qrpt_solve(
    qr: &Matrix,
    tau: &Vector,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> Result<()> {
    if qr.size1() != qr.size2() {
        return Err(gsl::error::Error::NotSquare);
    }
    if qr.size1() != p.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if qr.size1() != b.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if qr.size2() != x.size() {
        return Err(gsl::error::Error::BadLen);
    }

    x.copy_from(b)?;
    qrpt_svx(qr, tau, p, x)?;

    Ok(())
}

pub fn qrpt_svx(qr: &Matrix, tau: &Vector, p: &Permutation, x: &mut Vector) -> Result<()> {
    if qr.size1() != qr.size2() {
        return Err(gsl::error::Error::NotSquare);
    }
    if qr.size1() != p.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if qr.size2() != x.size() {
        return Err(gsl::error::Error::BadLen);
    }

    QR::qtvec(qr, tau, x)?;
    x.trsv(CblasUpper, CblasTranspose::NoTrans, CblasDiag::NonUnit, qr)?;
    p.inv_permute(x)?;

    Ok(())
}

pub fn qrpt_lssolve(
    qr: &Matrix,
    tau: &Vector,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
    residual: &mut Vector,
) -> Result<()> {
    let n = qr.size2();
    qrpt_lssolve2(qr, tau, p, b, n, x, residual)
}

pub fn qrpt_lssolve2(
    qr: &Matrix,
    tau: &Vector,
    p: &Permutation,
    b: &Vector,
    rank: usize,
    x: &mut Vector,
    residual: &mut Vector,
) -> Result<()> {
    let m = qr.size1();
    let n = qr.size2();

    if m < n {
        return Err(gsl::error::Error::BadLen);
    }
    if m != b.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if rank == 0 || rank > n {
        return Err(gsl::error::Error::BadLen);
    }
    if n != x.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if m != residual.size() {
        return Err(gsl::error::Error::BadLen);
    }

    let r11 = qr.submatrix(0, 0, rank, rank);
    let mut qtb1 = residual.subvector_mut(0, rank);
    let mut x1 = x.subvector_mut(0, rank);

    residual.copy_from(b)?;
    QR::qtvec(qr, tau, residual)?;
    x1.copy_from(&qtb1)?;
    x1.trsv(CblasUpper, CblasTranspose::NoTrans, CblasDiag::NonUnit, &r11)?;

    for i in rank..n {
        x.set(i, 0.0);
    }

    p.inv_permute(x)?;
    qtb1.set_zero();
    QR::qvec(qr, tau, residual)?;

    Ok(())
}

pub fn qrpt_qrsolve(
    q: &Matrix,
    r: &Matrix,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> Result<()> {
    if q.size1() != q.size2() || r.size1() != r.size2() {
        return Err(gsl::error::Error::NotSquare);
    }
    if q.size1() != p.size() || q.size1() != r.size1() || q.size1() != b.size() {
        return Err(gsl::error::Error::BadLen);
    }

    x.gemv(CblasTranspose::Trans, 1.0, q, b, 0.0)?;
    x.trsv(CblasUpper, CblasTranspose::NoTrans, CblasDiag::NonUnit, r)?;
    p.inv_permute(x)?;

    Ok(())
}

pub fn qrpt_rsolve(
    qr: &Matrix,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> Result<()> {
    if qr.size1() != qr.size2() {
        return Err(gsl::error::Error::NotSquare);
    }
    if qr.size1() != b.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if qr.size2() != x.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if p.size() != x.size() {
        return Err(gsl::error::Error::BadLen);
    }

    x.copy_from(b)?;
    x.trsv(CblasUpper, CblasTranspose::NoTrans, CblasDiag::NonUnit, qr)?;
    p.inv_permute(x)?;

    Ok(())
}

pub fn qrpt_rsvx(qr: &Matrix, p: &Permutation, x: &mut Vector) -> Result<()> {
    if qr.size1() != qr.size2() {
        return Err(gsl::error::Error::NotSquare);
    }
    if qr.size2() != x.size() {
        return Err(gsl::error::Error::BadLen);
    }
    if p.size() != x.size() {
        return Err(gsl::error::Error::BadLen);
    }

    x.trsv(CblasUpper, CblasTranspose::NoTrans, CblasDiag::NonUnit, qr)?;
    p.inv_permute(x)?;

    Ok(())
}

pub fn qrpt_update(
    q: &mut Matrix,
    r: &mut Matrix,
    p: &Permutation,
    w: &mut Vector,
    v: &Vector,
) -> Result<()> {
    let m = r.size1();
    let n = r.size2();

    if q.size1() != m || q.size2() != m {
        return Err(gsl::error::Error::NotSquare);
    }
    if w.size() != m {
        return Err(gsl::error::Error::BadLen);
    }
    if v.size() != n {
        return Err(gsl::error::Error::BadLen);
    }

    for k in (1..m).rev() {
        let wk = w.get(k);
        let wkm1 = w.get(k - 1);
        let (c, s) = givens(wkm1, wk);
        givens_gv(w, k - 1, k, c, s);
        apply_givens_qr(m, n, q, r, k - 1, k, c, s);
    }

    let w0 = w.get(0);
    for j in 0..n {
        let r0j = r.get(0, j);
        let pj = p.get(j);
        let vj = v.get(pj);
        r.set(0, j, r0j + w0 * vj);
    }

    let k_max = m.min(n + 1);
    for k in 1..k_max {
        let diag = r.get(k - 1, k - 1);
        let offdiag = r.get(k, k - 1);
        let (c, s) = givens(diag, offdiag);
        apply_givens_qr(m, n, q, r, k - 1, k, c, s);
        r.set(k, k - 1, 0.0);
    }

    Ok(())
}

pub fn qrpt_rank(qr: &Matrix, tol: f64) -> usize {
    let m = qr.size1();
    let n = qr.size2();
    let diag = qr.diagonal();
    let eps = if tol < 0.0 {
        let (min, max) = diag.minmax();
        let absmax = min.abs().max(max.abs());
        let ee = (absmax.ln() / 2.0f64.ln()) as i32;
        20.0 * (m + n) as f64 * 2.0f64.powi(ee) * f64::EPSILON
    } else {
        tol
    };

    diag.iter().filter(|&&d| d.abs() > eps).count()
}

pub fn qrpt_rcond(qr: &Matrix, rcond: &mut f64, work: &mut Vector) -> Result<()> {
    let m = qr.size1();
    let n = qr.size2();

    if m < n {
        return Err(gsl::error::Error::BadLen);
    }
    if work.size() != 3 * n {
        return Err(gsl::error::Error::BadLen);
    }

    let r = qr.submatrix(0, 0, n, n);
    tri_rcond(CblasUpper, &r, rcond, work)
}

// Helper functions
fn givens(a: f64, b: f64) -> (f64, f64) {
    if b == 0.0 {
        (1.0, 0.0)
    } else if b.abs() > a.abs() {
        let t = -a / b;
        let s = 1.0 / (1.0 + t * t).sqrt();
        (s * t, s)
    } else {
        let t = -b / a;
        let c = 1.0 / (1.0 + t * t).sqrt();
        (c, c * t)
    }
}

fn givens_gv(v: &mut Vector, i: usize, j: usize, c: f64, s: f64) {
    let vi = v.get(i);
    let vj = v.get(j);
    v.set(i, c * vi - s * vj);
    v.set(j, s * vi + c * vj);
}

fn apply_givens_qr(
    m: usize,
    n: usize,
    q: &mut Matrix,
    r: &mut Matrix,
    i: usize,
    j: usize,
    c: f64,
    s: f64,
) {
    for k in 0..m {
        let qki = q.get(k, i);
        let qkj = q.get(k, j);
        q.set(k, i, c * qki - s * qkj);
        q.set(k, j, s * qki + c * qkj);
    }

    let k_start = i.min(j);
    for k in k_start..n {
        let rik = r.get(i, k);
        let rjk = r.get(j, k);
        r.set(i, k, c * rik - s * rjk);
        r.set(j, k, s * rik + c * rjk);
    }
}
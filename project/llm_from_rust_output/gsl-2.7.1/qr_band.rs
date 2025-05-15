use gsl::{
    blas::Level1,
    linalg::{householder::Householder, QR},
    matrix::Matrix,
    vector::Vector,
    types::{GslError, GslResult},
};

pub fn qr_band_decomp_l2(
    m: usize,
    p: usize,
    q: usize,
    ab: &mut Matrix,
    tau: &mut Vector,
) -> GslResult<()> {
    let n = ab.size1();
    if tau.size() != n {
        return Err(GslError::BadLen("tau must have length N"));
    }
    if ab.size2() != 2 * p + q + 1 {
        return Err(GslError::BadLen("dimensions of AB are inconsistent with (p,q)"));
    }

    let min_mn = m.min(n);
    if p > 0 {
        let mut submatrix = ab.submatrix(0, 0, n, p)?;
        submatrix.set_zero();
    }

    for j in 0..min_mn {
        let k1 = (p + 1).min(m - j);
        let k2 = (p + q).min(n - j - 1);

        let mut c = ab.subrow(j, p + q, k1)?;
        let tau_j = Householder::transform(&mut c)?;
        let ptr = c.ptr_mut(0);
        tau.set(j, tau_j)?;

        if k2 > 0 {
            let mut m_sub = ab.submatrix(
                j + 1,
                p + q - 1,
                k2,
                k1,
            )?;
            let mut work = tau.subvector(j + 1, k2)?;
            let tmp = *ptr;
            m_sub.set_tda(m_sub.tda() - 1);
            *ptr = 1.0;
            Householder::right(tau_j, &c, &mut m_sub, &mut work)?;
            *ptr = tmp;
        }
    }

    Ok(())
}

pub fn qr_band_unpack_l2(
    p: usize,
    q: usize,
    qrb: &Matrix,
    tau: &Vector,
    q_matrix: &mut Matrix,
    r: &mut Matrix,
) -> GslResult<()> {
    let m = q_matrix.size1();
    let n = qrb.size1();

    if q_matrix.size2() != m {
        return Err(GslError::NotSquare("Q matrix must be square"));
    }
    if r.size1() != m || r.size2() != n {
        return Err(GslError::NotSquare("R matrix must be M x N"));
    }
    if tau.size() < m.min(n) {
        return Err(GslError::BadLen("size of tau must be at least MIN(M,N)"));
    }
    if qrb.size2() != 2 * p + q + 1 {
        return Err(GslError::BadLen("dimensions of QRB are inconsistent with (p,q)"));
    }

    q_matrix.set_identity();
    let min_mn = m.min(n);

    for i in (0..min_mn).rev() {
        let k1 = (p + 1).min(m - i);
        let h = qrb.const_subrow(i, p + q, k1)?;
        let mut m_sub = q_matrix.submatrix(i, i, k1, m - i)?;
        let ti = tau.get(i)?;
        let mut work = r.subcolumn(0, 0, m - i)?;
        let ptr = h.ptr(0);
        let tmp = *ptr;
        *ptr = 1.0;
        Householder::left(ti, &h, &mut m_sub, &mut work)?;
        *ptr = tmp;
    }

    r.set_zero();
    let max_i = (p + q).min(n - 1);

    for i in 0..=max_i {
        let src = qrb.const_subcolumn(p + q - i, i, m.min(n - i))?;
        let mut dest = r.superdiagonal(i)?;
        dest.copy(&src)?;
    }

    Ok(())
}
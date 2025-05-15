use gsl::{
    blas::{
        CblasDiag, CblasSide, CblasTranspose, CblasUplo, dtrmm, dtrmv, dtrsv,
    },
    error::{GslError, GslResult},
    linalg::QR_UZ_decomp,
    matrix::{Matrix, MatrixView},
    vector::{Vector, VectorView},
};

pub fn qr_uu_decomp(
    u: &mut Matrix,
    s: &mut Matrix,
    t: &mut Matrix,
) -> GslResult<()> {
    let n = u.size1();
    if n != u.size2() {
        return Err(GslError::NotSquare("U matrix must be square"));
    }
    if s.size1() != s.size2() {
        return Err(GslError::NotSquare("S matrix must be square"));
    }
    if n != s.size1() {
        return Err(GslError::BadLength("S and U must have same dimensions"));
    }
    if t.size1() != n || t.size2() != n {
        return Err(GslError::BadLength("T matrix has wrong dimensions"));
    }

    if n == 1 {
        let tau = qrtt_householder_transform(u.get_mut(0, 0), s.get_mut(0, 0));
        t.set(0, 0, tau);
        return Ok(());
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let mut u11 = u.submatrix_mut(0, 0, n1, n1);
    let mut u12 = u.submatrix_mut(0, n1, n1, n2);
    let mut u22 = u.submatrix_mut(n1, n1, n2, n2);

    let mut s11 = s.submatrix_mut(0, 0, n1, n1);
    let mut s12 = s.submatrix_mut(0, n1, n1, n2);

    let mut t11 = t.submatrix_mut(0, 0, n1, n1);
    let mut t12 = t.submatrix_mut(0, n1, n1, n2);
    let mut t22 = t.submatrix_mut(n1, n1, n2, n2);

    qr_uu_decomp(&mut u11, &mut s11, &mut t11)?;

    t12.copy_from(&s12)?;
    dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &s11,
        &mut t12,
    )?;
    t12.add(&u12)?;
    dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &t11,
        &mut t12,
    )?;
    u12.sub(&t12)?;
    dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        1.0,
        &s11,
        &mut t12,
    )?;
    s12.sub(&t12)?;

    let mut m = s.submatrix_mut(0, n1, n, n2);
    QR_UZ_decomp(&mut u22, &mut m, &mut t22)?;

    t12.copy_from(&s12)?;
    dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &s11,
        &mut t12,
    )?;
    dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        -1.0,
        &t11,
        &mut t12,
    )?;
    dtrmm(
        CblasSide::Right,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        1.0,
        &t22,
        &mut t12,
    )?;

    Ok(())
}

pub fn qr_uu_lssolve(
    r: &Matrix,
    y: &Matrix,
    t: &Matrix,
    b: &Vector,
    x: &mut Vector,
    work: &mut Vector,
) -> GslResult<()> {
    let n = r.size1();
    let m = 2 * n;

    if r.size2() != n {
        return Err(GslError::NotSquare("R matrix must be square"));
    }
    if y.size1() != y.size2() {
        return Err(GslError::NotSquare("Y matrix must be square"));
    }
    if y.size1() != n {
        return Err(GslError::BadLength("Y and R must have same dimensions"));
    }
    if t.size1() != n || t.size2() != n {
        return Err(GslError::BadLength("T matrix must be N-by-N"));
    }
    if m != b.len() {
        return Err(GslError::BadLength("matrix size must match b size"));
    }
    if m != x.len() {
        return Err(GslError::BadLength("matrix size must match solution size"));
    }
    if n != work.len() {
        return Err(GslError::BadLength("workspace must be length N"));
    }

    x.copy_from(b)?;
    qr_uu_qtvec(y, t, x, work)?;

    let mut x1 = x.subvector_mut(0, n);
    dtrsv(
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        r,
        &mut x1,
    )?;

    Ok(())
}

pub fn qr_uu_qtvec(
    y: &Matrix,
    t: &Matrix,
    b: &mut Vector,
    work: &mut Vector,
) -> GslResult<()> {
    let n = y.size1();
    let m = 2 * n;

    if y.size2() != n {
        return Err(GslError::NotSquare("Y matrix must be square"));
    }
    if t.size1() != n || t.size2() != n {
        return Err(GslError::BadLength("T matrix must be N-by-N"));
    }
    if b.len() != m {
        return Err(GslError::BadLength("b vector must have length M"));
    }
    if work.len() != n {
        return Err(GslError::BadLength("workspace must be length N"));
    }

    let mut b1 = b.subvector_mut(0, n);
    let mut b2 = b.subvector_mut(n, n);

    work.copy_from(&b2)?;
    dtrmv(
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        y,
        work,
    )?;
    work.add(&b1)?;
    dtrmv(
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        t,
        work,
    )?;
    b1.sub(work)?;
    dtrmv(
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        y,
        work,
    )?;
    b2.sub(work)?;

    Ok(())
}

fn qrtt_householder_transform(v0: &mut f64, v1: &mut f64) -> f64 {
    let xnorm = v1.abs();
    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.powi(2) + xnorm.powi(2)).sqrt();
    let tau = (beta - alpha) / beta;

    let s = alpha - beta;
    if s.abs() > f64::MIN_POSITIVE {
        *v1 /= s;
    } else {
        *v1 *= f64::EPSILON / s;
        *v1 /= f64::EPSILON;
    }
    *v0 = beta;

    tau
}
use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    linalg::{QRDecomp, QRLSSolve},
    matrix::{Matrix, MatrixView, Submatrix},
    vector::{Vector, VectorView},
};

pub fn qr_ud_decomp(
    u: &mut Matrix,
    d: &Vector,
    y: &mut Matrix,
    t: &mut Matrix,
) -> GslResult<()> {
    let n = u.size1();
    if n != u.size2() {
        return Err(GslError::NotSquare);
    }
    if d.size() != n {
        return Err(GslError::BadLength);
    }
    if y.size1() != y.size2() {
        return Err(GslError::NotSquare);
    }
    if y.size1() != n {
        return Err(GslError::BadLength);
    }
    if t.size1() != n || t.size2() != n {
        return Err(GslError::BadLength);
    }

    if n == 1 {
        let t00 = t.get_mut(0, 0)?;
        let u00 = u.get_mut(0, 0)?;
        let y00 = y.get_mut(0, 0)?;
        *y00 = d.get(0)?;
        *t00 = qrtd_householder_transform(u00, y00);
        return Ok(());
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let mut u11 = u.submatrix(0, 0, n1, n1);
    let mut u12 = u.submatrix(0, n1, n1, n2);
    let mut u22 = u.submatrix(n1, n1, n2, n2);

    let d1 = d.subvector(0, n1);
    let d2 = d.subvector(n1, n2);

    let mut y11 = y.submatrix(0, 0, n1, n1);
    let mut y12 = y.submatrix(0, n1, n1, n2);

    let mut t11 = t.submatrix(0, 0, n1, n1);
    let mut t12 = t.submatrix(0, n1, n1, n2);
    let mut t22 = t.submatrix(n1, n1, n2, n2);

    QRDecomp::qr_ud(&mut u11, &d1, &mut y11, &mut t11)?;

    t12.copy_from(&u12)?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &t11,
    )?;
    u12.sub(&t12)?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        -1.0,
        &y11,
    )?;
    y12.copy_from(&t12)?;

    let mut m = y.submatrix(0, n1, n, n2);
    aux_qr_trd_decomp(&mut u22, &mut m, &d2, &mut t22)?;

    t12.copy_from(&y12)?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &y11,
    )?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        -1.0,
        &t11,
    )?;
    t12.dtrmm(
        CblasSide::Right,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        1.0,
        &t22,
    )?;

    Ok(())
}

pub fn qr_ud_lssolve(
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
        return Err(GslError::NotSquare);
    }
    if y.size1() != y.size2() {
        return Err(GslError::NotSquare);
    }
    if y.size1() != n {
        return Err(GslError::BadLength);
    }
    if t.size1() != n || t.size2() != n {
        return Err(GslError::BadLength);
    }
    if m != b.size() {
        return Err(GslError::BadLength);
    }
    if m != x.size() {
        return Err(GslError::BadLength);
    }
    if n != work.size() {
        return Err(GslError::BadLength);
    }

    QRLSSolve::qr_uu(r, y, t, b, x, work)
}

fn aux_qr_trd_decomp(
    u: &mut Matrix,
    a: &mut Matrix,
    d: &Vector,
    t: &mut Matrix,
) -> GslResult<()> {
    let n = u.size1();
    if n != u.size2() {
        return Err(GslError::NotSquare);
    }
    if a.size1() <= n {
        return Err(GslError::BadLength);
    }
    if d.size() != n {
        return Err(GslError::BadLength);
    }
    if t.size1() != n || t.size2() != n {
        return Err(GslError::BadLength);
    }

    if n == 1 {
        let m = a.size1() - n;
        let t00 = t.get_mut(0, 0)?;
        let u00 = u.get_mut(0, 0)?;
        let mut v = a.subcolumn(0, 0, m);
        let d00 = a.get_mut(m, 0)?;
        *d00 = d.get(0)?;
        *t00 = qrtrd_householder_transform(u00, &mut v, d00);
        return Ok(());
    }

    let m0 = a.size1() - n;
    let n1 = n / 2;
    let n2 = n - n1;

    let mut u11 = u.submatrix(0, 0, n1, n1);
    let mut u12 = u.submatrix(0, n1, n1, n2);
    let mut u22 = u.submatrix(n1, n1, n2, n2);

    let mut a1 = a.submatrix(0, 0, m0, n1);
    let mut a2 = a.submatrix(0, n1, m0, n2);

    let d1 = d.subvector(0, n1);
    let d2 = d.subvector(n1, n2);

    let mut t11 = t.submatrix(0, 0, n1, n1);
    let mut t12 = t.submatrix(0, n1, n1, n2);
    let mut t22 = t.submatrix(n1, n1, n2, n2);

    let mut y41 = a.submatrix(m0, 0, n1, n1);
    let mut y42 = a.submatrix(m0, n1, n1, n2);

    let mut m = a.submatrix(0, 0, m0 + n1, n1);
    aux_qr_trd_decomp(&mut u11, &mut m, &d1, &mut t11)?;

    t12.copy_from(&u12)?;
    t12.dgemm(
        CblasTranspose::Trans,
        CblasTranspose::NoTrans,
        1.0,
        &a1,
        &a2,
        1.0,
    )?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &t11,
    )?;
    u12.sub(&t12)?;
    a2.dgemm(
        CblasTranspose::NoTrans,
        CblasTranspose::NoTrans,
        -1.0,
        &a1,
        &t12,
        1.0,
    )?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        -1.0,
        &y41,
    )?;
    y42.copy_from(&t12)?;

    let mut m = a.submatrix(0, n1, m0 + n, n2);
    aux_qr_trd_decomp(&mut u22, &mut m, &d2, &mut t22)?;

    t12.copy_from(&y42)?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        &y41,
    )?;
    t12.dgemm(
        CblasTranspose::Trans,
        CblasTranspose::NoTrans,
        1.0,
        &a1,
        &a2,
        1.0,
    )?;
    t12.dtrmm(
        CblasSide::Left,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        -1.0,
        &t11,
    )?;
    t12.dtrmm(
        CblasSide::Right,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        1.0,
        &t22,
    )?;

    Ok(())
}

fn qrtd_householder_transform(v0: &mut f64, v1: &mut f64) -> f64 {
    let xnorm = v1.abs();
    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.hypot(xnorm));
    let tau = (beta - alpha) / beta;
    let s = alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        *v1 /= s;
        *v0 = beta;
    } else {
        *v1 *= f64::EPSILON / s;
        *v1 /= f64::EPSILON;
        *v0 = beta;
    }

    tau
}

fn qrtrd_householder_transform(v0: &mut f64, v: &mut Vector, d: &mut f64) -> f64 {
    let xnorm = v.nrm2().hypot(*d);
    if xnorm == 0.0 {
        return 0.0;
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.hypot(xnorm));
    let tau = (beta - alpha) / beta;
    let s = alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        v.scale(1.0 / s);
        *d /= s;
        *v0 = beta;
    } else {
        v.scale(f64::EPSILON / s);
        v.scale(1.0 / f64::EPSILON);
        *d *= f64::EPSILON / s;
        *d /= f64::EPSILON;
        *v0 = beta;
    }

    tau
}
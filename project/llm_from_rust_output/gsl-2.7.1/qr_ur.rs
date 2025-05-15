use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUpper},
    matrix::Matrix,
    vector::Vector,
    error::{Error, Result},
    linalg::QR,
    types::complex::ComplexFloat,
};

pub fn qr_ur_decomp(
    s: &mut Matrix,
    a: &mut Matrix,
    t: &mut Matrix,
) -> Result<()> {
    let m = a.size1();
    let n = s.size1();

    if n != s.size2() {
        return Err(Error::new("S matrix must be square", 77, "qr_ur.c"));
    } else if n != a.size2() {
        return Err(Error::new(
            "S and A have different number of columns",
            81,
            "qr_ur.c",
        ));
    } else if t.size1() != n || t.size2() != n {
        return Err(Error::new(
            "T matrix has wrong dimensions",
            85,
            "qr_ur.c",
        ));
    }

    if n == 1 {
        let t00 = t.get_mut(0, 0).unwrap();
        let s00 = s.get_mut(0, 0).unwrap();
        let mut v = a.column(0);
        *t00 = qr_householder_transform(s00, &mut v)?;
        return Ok(());
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let mut s11 = s.submatrix_mut(0, 0, n1, n1);
    let mut s12 = s.submatrix_mut(0, n1, n1, n2);
    let mut s22 = s.submatrix_mut(n1, n1, n2, n2);

    let mut a1 = a.submatrix_mut(0, 0, m, n1);
    let mut a2 = a.submatrix_mut(0, n1, m, n2);

    let mut t11 = t.submatrix_mut(0, 0, n1, n1);
    let mut t12 = t.submatrix_mut(0, n1, n1, n2);
    let mut t22 = t.submatrix_mut(n1, n1, n2, n2);

    QR::decomp(&mut s11, &mut a1, &mut t11)?;

    t12.copy_from(&s12)?;
    t12.gemm(
        CblasTrans,
        CblasNoTrans,
        1.0,
        &a1,
        &a2,
        1.0,
    )?;
    t12.trmm(
        CblasLeft,
        CblasUpper,
        CblasTrans,
        CblasNonUnit,
        1.0,
        &t11,
    )?;
    a2.gemm(
        CblasNoTrans,
        CblasNoTrans,
        -1.0,
        &a1,
        &t12,
        1.0,
    )?;
    s12.sub(&t12)?;

    QR::decomp(&mut s22, &mut a2, &mut t22)?;

    t12.gemm(
        CblasTrans,
        CblasNoTrans,
        1.0,
        &a1,
        &a2,
        0.0,
    )?;
    t12.trmm(
        CblasLeft,
        CblasUpper,
        CblasNoTrans,
        CblasNonUnit,
        -1.0,
        &t11,
    )?;
    t12.trmm(
        CblasRight,
        CblasUpper,
        CblasNoTrans,
        CblasNonUnit,
        1.0,
        &t22,
    )?;

    Ok(())
}

fn qr_householder_transform(
    v0: &mut f64,
    v: &mut Vector,
) -> Result<f64> {
    let xnorm = v.nrm2()?;
    if xnorm == 0.0 {
        return Ok(0.0);
    }

    let alpha = *v0;
    let beta = -alpha.signum() * (alpha.hypot(xnorm));
    let tau = (beta - alpha) / beta;
    let s = alpha - beta;

    if s.abs() > f64::MIN_POSITIVE {
        v.scale(1.0 / s)?;
    } else {
        v.scale(f64::EPSILON / s)?;
        v.scale(1.0 / f64::EPSILON)?;
    }

    *v0 = beta;
    Ok(tau)
}
use gsl::{
    blas::{CblasDiag, CblasTranspose, CblasUpperLower},
    error::{GslError, GslResult},
    matrix::Matrix,
    vector::Vector,
};

pub fn ldlt_decomp(a: &mut Matrix) -> GslResult<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(GslError::NotSquare);
    }

    if n == 1 {
        return Ok(());
    }

    let anorm = ldlt_norm1(a);
    let a00 = a.get(0, 0)?;
    if a00 == 0.0 {
        return Err(GslError::SingularMatrix);
    }

    {
        let mut v = a.subcolumn_mut(0, 1, n - 1)?;
        v.scale(1.0 / a00)?;
    }

    let mut work = a.subrow_mut(0, 1, n - 1)?;

    for j in 1..n {
        let mut w = work.subvector_mut(0, j)?;
        let mut ajj = a.get(j, j)?;
        let mut dval = 0.0;

        for i in 0..j {
            let aii = a.get(i, i)?;
            let aji = a.get(j, i)?;
            w.set(i, aji * aii)?;
        }

        let v = a.subrow(j, 0, j)?;
        dval = v.dot(&w)?;
        ajj -= dval;

        if ajj == 0.0 {
            return Err(GslError::SingularMatrix);
        }

        a.set(j, j, ajj)?;

        if j < n - 1 {
            let ajjinv = 1.0 / ajj;
            let m = a.submatrix(j + 1, 0, n - j - 1, j)?;
            let mut v = a.subcolumn_mut(j, j + 1, n - j - 1)?;
            m.gemv(
                CblasTranspose::NoTrans,
                -ajjinv,
                &w,
                ajjinv,
                &mut v,
            )?;
        }
    }

    a.set(0, n - 1, anorm)?;
    Ok(())
}

pub fn ldlt_solve(ldlt: &Matrix, b: &Vector, x: &mut Vector) -> GslResult<()> {
    if ldlt.size1() != ldlt.size2() {
        return Err(GslError::NotSquare);
    }
    if ldlt.size1() != b.len() {
        return Err(GslError::BadLength);
    }
    if ldlt.size2() != x.len() {
        return Err(GslError::BadLength);
    }

    x.copy(b)?;
    ldlt_svx(ldlt, x)
}

pub fn ldlt_svx(ldlt: &Matrix, x: &mut Vector) -> GslResult<()> {
    if ldlt.size1() != ldlt.size2() {
        return Err(GslError::NotSquare);
    }
    if ldlt.size2() != x.len() {
        return Err(GslError::BadLength);
    }

    let diag = ldlt.diagonal();
    ldlt.trsv(CblasUpperLower::Lower, CblasTranspose::NoTrans, CblasDiag::Unit, x)?;
    x.div(&diag)?;
    ldlt.trsv(CblasUpperLower::Lower, CblasTranspose::Trans, CblasDiag::Unit, x)?;
    Ok(())
}

pub fn ldlt_rcond(ldlt: &Matrix, rcond: &mut f64, work: &mut Vector) -> GslResult<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(GslError::NotSquare);
    }
    if work.len() != 3 * n {
        return Err(GslError::BadLength);
    }

    let anorm = if n == 1 {
        ldlt.get(0, 0)?.abs()
    } else {
        ldlt.get(0, n - 1)?
    };

    *rcond = 0.0;
    if anorm == 0.0 {
        return Ok(());
    }

    let mut ainvnorm = 0.0;
    gsl::linalg::invnorm1(n, ldlt_ainv, ldlt, &mut ainvnorm, work)?;

    if ainvnorm != 0.0 {
        *rcond = 1.0 / anorm / ainvnorm;
    }

    Ok(())
}

fn ldlt_norm1(a: &Matrix) -> f64 {
    let n = a.size1();
    let mut max = 0.0;

    for j in 0..n {
        let v = a.subcolumn(j, j, n - j).unwrap();
        let mut sum = v.asum();

        for i in 0..j {
            sum += a.get(i, j).unwrap().abs();
        }

        if sum > max {
            max = sum;
        }
    }

    max
}

fn ldlt_ainv(transa: CblasTranspose, x: &mut Vector, params: &Matrix) -> GslResult<()> {
    let diag = params.diagonal();
    params.trsv(CblasUpperLower::Lower, transa, CblasDiag::Unit, x)?;
    x.div(&diag)?;
    params.trsv(
        CblasUpperLower::Lower,
        transa.transpose(),
        CblasDiag::Unit,
        x,
    )?;
    Ok(())
}
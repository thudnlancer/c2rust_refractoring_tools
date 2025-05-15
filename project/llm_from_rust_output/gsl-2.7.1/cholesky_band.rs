use gsl::{
    blas::{CblasDiag, CblasOrder, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    matrix::Matrix,
    vector::Vector,
    types::{GslMatrix, GslVector},
};

pub fn cholesky_band_decomp(A: &mut Matrix) -> GslResult<()> {
    let N = A.size1();
    let ndiag = A.size2();

    if ndiag > N {
        return Err(GslError::BadLen);
    }

    let p = ndiag - 1;
    let kld = if 1 > p { 1 } else { p } as i32;

    if ndiag > 1 {
        let anorm = cholesky_band_norm1(A)?;
        A.set(N - 1, p, anorm)?;
    }

    for j in 0..N {
        let ajj = A.get(j, 0)?;
        if ajj <= 0.0 {
            return Err(GslError::Domain);
        }

        let ajj_sqrt = ajj.sqrt();
        A.set(j, 0, ajj_sqrt)?;

        let lenv = if p < N - j - 1 { p } else { N - j - 1 };
        if lenv > 0 {
            let mut v = A.subrow_mut(j, 1, lenv)?;
            let mut m = A.submatrix_mut(j + 1, 0, lenv, lenv)?;
            m.set_tda(kld as usize);

            v.scale(1.0 / ajj_sqrt)?;
            m.syr(CblasUplo::Upper, -1.0, &v)?;
        }
    }

    Ok(())
}

pub fn cholesky_band_solve(LLT: &Matrix, b: &Vector, x: &mut Vector) -> GslResult<()> {
    if LLT.size1() != b.size() {
        return Err(GslError::BadLen);
    }
    if LLT.size1() != x.size() {
        return Err(GslError::BadLen);
    }

    x.copy_from(b)?;
    cholesky_band_svx(LLT, x)
}

pub fn cholesky_band_svx(LLT: &Matrix, x: &mut Vector) -> GslResult<()> {
    if LLT.size1() != x.size() {
        return Err(GslError::BadLen);
    }

    let k = (LLT.size2() - 1) as i32;
    let tda = LLT.tda() as i32;

    x.tbsv(
        CblasOrder::ColMajor,
        CblasUplo::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        k,
        LLT.data(),
        tda,
    )?;

    x.tbsv(
        CblasOrder::ColMajor,
        CblasUplo::Lower,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        k,
        LLT.data(),
        tda,
    )?;

    Ok(())
}

pub fn cholesky_band_solvem(LLT: &Matrix, B: &Matrix, X: &mut Matrix) -> GslResult<()> {
    if LLT.size1() != B.size1() {
        return Err(GslError::BadLen);
    }
    if LLT.size1() != X.size1() {
        return Err(GslError::BadLen);
    }
    if B.size2() != X.size2() {
        return Err(GslError::BadLen);
    }

    X.copy_from(B)?;
    cholesky_band_svxm(LLT, X)
}

pub fn cholesky_band_svxm(LLT: &Matrix, X: &mut Matrix) -> GslResult<()> {
    if LLT.size1() != X.size1() {
        return Err(GslError::BadLen);
    }

    let nrhs = X.size2();
    for j in 0..nrhs {
        let mut xj = X.column_mut(j)?;
        cholesky_band_svx(LLT, &mut xj)?;
    }

    Ok(())
}

pub fn cholesky_band_invert(LLT: &Matrix, Ainv: &mut Matrix) -> GslResult<()> {
    if Ainv.size1() != Ainv.size2() {
        return Err(GslError::NotSquare);
    }
    if LLT.size1() != Ainv.size1() {
        return Err(GslError::BadLen);
    }

    cholesky_band_unpack(LLT, Ainv)?;
    Ainv.cholesky_invert()?;

    Ok(())
}

pub fn cholesky_band_unpack(LLT: &Matrix, L: &mut Matrix) -> GslResult<()> {
    let N = LLT.size1();
    if N != L.size1() {
        return Err(GslError::BadLen);
    }
    if L.size1() != L.size2() {
        return Err(GslError::NotSquare);
    }

    let p = LLT.size2() - 1;
    for i in 0..=p {
        let v = LLT.subcolumn(i, 0, N - i)?;
        let mut w = L.subdiagonal_mut(i)?;
        w.copy_from(&v)?;
    }

    for i in p + 1..N {
        let mut w = L.subdiagonal_mut(i)?;
        w.set_zero()?;
    }

    Ok(())
}

pub fn cholesky_band_rcond(
    LLT: &Matrix,
    rcond: &mut f64,
    work: &mut Vector,
) -> GslResult<()> {
    let N = LLT.size1();
    if work.size() != 3 * N {
        return Err(GslError::BadLen);
    }

    let ndiag = LLT.size2();
    let anorm = if ndiag == 1 {
        let v = LLT.column(0)?;
        let max = v.max()?;
        max * max
    } else {
        LLT.get(N - 1, ndiag - 1)?
    };

    *rcond = 0.0;
    if anorm == 0.0 {
        return Ok(());
    }

    let mut ainvnorm = 0.0;
    gsl_linalg_invnorm1(
        N,
        cholesky_band_Ainv,
        LLT as *const _ as *mut _,
        &mut ainvnorm,
        work,
    )?;

    if ainvnorm != 0.0 {
        *rcond = 1.0 / anorm / ainvnorm;
    }

    Ok(())
}

pub fn cholesky_band_scale(A: &Matrix, S: &mut Vector) -> GslResult<()> {
    let N = A.size1();
    let ndiag = A.size2();

    if ndiag > N {
        return Err(GslError::BadLen);
    }
    if N != S.size() {
        return Err(GslError::BadLen);
    }

    for i in 0..N {
        let aii = A.get(i, 0)?;
        let val = if aii <= 0.0 {
            1.0
        } else {
            1.0 / aii.sqrt()
        };
        S.set(i, val)?;
    }

    Ok(())
}

pub fn cholesky_band_scale_apply(A: &mut Matrix, S: &Vector) -> GslResult<()> {
    let N = A.size1();
    let ndiag = A.size2();

    if ndiag > N {
        return Err(GslError::BadLen);
    }
    if N != S.size() {
        return Err(GslError::BadLen);
    }

    for j in 0..N {
        let sj = S.get(j)?;
        let end = std::cmp::min(j + ndiag, N);
        for i in j..end {
            let si = S.get(i)?;
            let ptr = A.ptr_mut(j, i - j)?;
            *ptr *= sj * si;
        }
    }

    Ok(())
}

fn cholesky_band_norm1(A: &Matrix) -> GslResult<f64> {
    let N = A.size1();
    let ndiag = A.size2();

    if ndiag == 1 {
        let v = A.column(0)?;
        Ok(v.max()?)
    } else {
        let mut value = 0.0;
        for j in 0..N {
            let ncol = std::cmp::min(ndiag, N - j);
            let v = A.subrow(j, 0, ncol)?;
            let mut sum = v.asum()?;

            let mut k = j;
            let mut l = 1;
            while k > 0 && l < ndiag {
                k -= 1;
                let akl = A.get(k, l)?;
                sum += akl.abs();
                l += 1;
            }

            value = value.max(sum);
        }
        Ok(value)
    }
}

fn cholesky_band_Ainv(
    transa: CblasTranspose,
    x: &mut Vector,
    params: *mut std::ffi::c_void,
) -> GslResult<()> {
    let llt = unsafe { &*(params as *const Matrix) };
    let k = (llt.size2() - 1) as i32;
    let tda = llt.tda() as i32;

    x.tbsv(
        CblasOrder::ColMajor,
        CblasUplo::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        k,
        llt.data(),
        tda,
    )?;

    x.tbsv(
        CblasOrder::ColMajor,
        CblasUplo::Lower,
        transa,
        CblasDiag::NonUnit,
        k,
        llt.data(),
        tda,
    )?;

    Ok(())
}
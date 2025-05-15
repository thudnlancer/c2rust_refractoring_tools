use gsl::{
    blas::{CblasConjTrans, CblasLower, CblasNoTrans, CblasNonUnit},
    cblas::CBLAS_TRANSPOSE,
    complex::{Complex, ComplexF64},
    error::{GslError, GslResult},
    matrix::MatrixComplexF64,
    vector::VectorComplexF64,
};

pub fn gsl_linalg_complex_cholesky_decomp(A: &mut MatrixComplexF64) -> GslResult<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(GslError::NotSquareMatrix);
    }
    complex_cholesky_decomp_l3(A)
}

pub fn gsl_linalg_complex_cholesky_solve(
    cholesky: &MatrixComplexF64,
    b: &VectorComplexF64,
    x: &mut VectorComplexF64,
) -> GslResult<()> {
    if cholesky.size1() != cholesky.size2() {
        return Err(GslError::NotSquareMatrix);
    }
    if cholesky.size1() != b.len() {
        return Err(GslError::BadLength);
    }
    if cholesky.size2() != x.len() {
        return Err(GslError::BadLength);
    }

    x.copy_from(b)?;
    gsl_linalg_complex_cholesky_svx(cholesky, x)
}

pub fn gsl_linalg_complex_cholesky_svx(
    cholesky: &MatrixComplexF64,
    x: &mut VectorComplexF64,
) -> GslResult<()> {
    if cholesky.size1() != cholesky.size2() {
        return Err(GslError::NotSquareMatrix);
    }
    if cholesky.size2() != x.len() {
        return Err(GslError::BadLength);
    }

    x.trsv(CblasLower, CblasNoTrans, CblasNonUnit, cholesky)?;
    x.trsv(CblasLower, CblasConjTrans, CblasNonUnit, cholesky)?;
    Ok(())
}

pub fn gsl_linalg_complex_cholesky_invert(LLT: &mut MatrixComplexF64) -> GslResult<()> {
    if LLT.size1() != LLT.size2() {
        return Err(GslError::NotSquareMatrix);
    }

    let n = LLT.size1();
    LLT.tri_invert(CblasLower, CblasNonUnit)?;
    LLT.tri_lhl()?;

    for i in 1..n {
        for j in 0..i {
            let z = LLT.get(i, j).conj();
            LLT.set(j, i, z);
        }
    }

    Ok(())
}

fn cholesky_complex_conj_vector(v: &mut VectorComplexF64) {
    for i in 0..v.len() {
        let z = v.get(i).conj();
        v.set(i, z);
    }
}

fn complex_cholesky_decomp_l2(A: &mut MatrixComplexF64) -> GslResult<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(GslError::NotSquareMatrix);
    }

    for j in 0..n {
        let mut ajj = A.get(j, j).re();
        if j > 0 {
            let aj = A.subrow(j, 0, j);
            let dot = aj.dot(&aj)?;
            ajj -= dot.re();
        }

        if ajj <= 0.0 {
            return Err(GslError::MatrixNotPosDef);
        }

        ajj = ajj.sqrt();
        A.set(j, j, ComplexF64::new(ajj, 0.0));

        if j < n - 1 {
            let mut av = A.subcolumn(j, j + 1, n - j - 1);
            if j > 0 {
                let mut aj = A.subrow(j, 0, j);
                let am = A.submatrix(j + 1, 0, n - j - 1, j);
                cholesky_complex_conj_vector(&mut aj);
                av.gemv(CblasNoTrans, ComplexF64::new(-1.0, 0.0), &am, &aj, ComplexF64::new(1.0, 0.0))?;
                cholesky_complex_conj_vector(&mut aj);
            }
            av.scale(ComplexF64::new(1.0 / ajj, 0.0));
        }
    }

    Ok(())
}

fn complex_cholesky_decomp_l3(A: &mut MatrixComplexF64) -> GslResult<()> {
    let n = A.size1();
    if n != A.size2() {
        return Err(GslError::NotSquareMatrix);
    }

    if n <= 24 {
        return complex_cholesky_decomp_l2(A);
    }

    let n1 = if n >= 8 {
        ((n + 4) / 8) * 4
    } else {
        n / 2
    };
    let n2 = n - n1;

    let (a11, a21, a22) = A.split_at_mut(n1, n1);
    complex_cholesky_decomp_l3(a11)?;

    a21.trsm(
        CblasRight,
        CblasLower,
        CblasConjTrans,
        CblasNonUnit,
        ComplexF64::new(1.0, 0.0),
        a11,
    )?;

    a22.herk(CblasLower, CblasNoTrans, -1.0, a21, 1.0)?;

    complex_cholesky_decomp_l3(a22)?;

    Ok(())
}
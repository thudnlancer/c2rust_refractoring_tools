use gsl::{
    blas::{dscal, dnrm2, dtrsm, CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    eigen::{symmv, symmv_workspace},
    linalg::cholesky_decomp1,
    matrix::Matrix,
    vector::Vector,
    Value,
};

#[derive(Debug)]
pub struct GenSymmvWorkspace {
    size: usize,
    symmv_workspace: symmv_workspace::Workspace,
}

impl GenSymmvWorkspace {
    pub fn new(n: usize) -> Option<Self> {
        if n == 0 {
            gsl::error("matrix dimension must be positive integer", "gensymmv.c", 57, Value::Invalid);
            return None;
        }

        let symmv_ws = symmv_workspace::Workspace::new(n)?;
        Some(Self {
            size: n,
            symmv_workspace: symmv_ws,
        })
    }
}

pub fn gensymmv(
    a: &mut Matrix,
    b: &mut Matrix,
    eval: &mut Vector,
    evec: &mut Matrix,
    w: &mut GenSymmvWorkspace,
) -> Result<(), Value> {
    let n = a.size1();
    
    if n != a.size2() {
        gsl::error(
            "matrix must be square to compute eigenvalues",
            "gensymmv.c",
            123,
            Value::MatrixNotSquare,
        );
        return Err(Value::MatrixNotSquare);
    } else if n != b.size1() || n != b.size2() {
        gsl::error(
            "B matrix dimensions must match A",
            "gensymmv.c",
            127,
            Value::BadLength,
        );
        return Err(Value::BadLength);
    } else if eval.len() != n {
        gsl::error(
            "eigenvalue vector must match matrix size",
            "gensymmv.c",
            131,
            Value::BadLength,
        );
        return Err(Value::BadLength);
    } else if evec.size1() != evec.size2() {
        gsl::error(
            "eigenvector matrix must be square",
            "gensymmv.c",
            135,
            Value::MatrixNotSquare,
        );
        return Err(Value::MatrixNotSquare);
    } else if evec.size1() != n {
        gsl::error(
            "eigenvector matrix has wrong size",
            "gensymmv.c",
            139,
            Value::BadLength,
        );
        return Err(Value::BadLength);
    } else if w.size != n {
        gsl::error(
            "matrix size does not match workspace",
            "gensymmv.c",
            143,
            Value::BadLength,
        );
        return Err(Value::BadLength);
    }

    cholesky_decomp1(b)?;
    gsl::eigen::gensymm_standardize(a, b)?;
    symmv(a, eval, evec, &mut w.symmv_workspace)?;

    dtrsm(
        CblasSide::Left,
        CblasUplo::Lower,
        CblasTranspose::Trans,
        CblasDiag::NonUnit,
        1.0,
        b,
        evec,
    )?;

    normalize_eigenvectors(evec);
    Ok(())
}

fn normalize_eigenvectors(evec: &mut Matrix) {
    let n = evec.size1();
    for i in 0..n {
        let mut vi = evec.column(i);
        let scale = 1.0 / dnrm2(&vi);
        dscal(scale, &mut vi);
    }
}
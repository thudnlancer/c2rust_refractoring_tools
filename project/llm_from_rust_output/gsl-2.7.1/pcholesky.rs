use gsl::{
    blas::{CblasDiag, CblasTranspose, CblasUplo},
    error::Result,
    linalg::{cholesky_scale, cholesky_scale_apply, tri_invert, tri_LTL},
    matrix::Matrix,
    permutation::Permutation,
    vector::Vector,
};

pub fn pcholesky_decomp(copy_uplo: bool, a: &mut Matrix, p: &mut Permutation) -> Result<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }

    if copy_uplo {
        a.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit, a)?;
    }

    p.init();
    let mut diag = a.diagonal();

    for k in 0..n {
        let w = diag.subvector(k, n - k)?;
        let j = w.max_index()? + k;

        p.swap(k, j)?;
        cholesky_swap_rowcol(a, k, j)?;

        if k < n - 1 {
            let alpha = a.get(k, k);
            let alphainv = 1.0 / alpha;

            let v = a.subcolumn(k, k + 1, n - k - 1)?;
            let mut m = a.submatrix(k + 1, k + 1, n - k - 1, n - k - 1)?;

            m.syr(CblasUplo::Lower, -alphainv, &v)?;
            v.scale(alphainv)?;
        }
    }

    Ok(())
}

pub fn cholesky_swap_rowcol(a: &mut Matrix, i: usize, j: usize) -> Result<()> {
    if i != j {
        let n = a.size1();
        let (ii, jj) = if i < j { (i, j) } else { (j, i) };

        for k in 0..ii {
            let tmp = a.get(jj, k);
            a.set(jj, k, a.get(ii, k));
            a.set(ii, k, tmp);
        }

        for k in ii + 1..jj {
            let tmp = a.get(k, ii);
            a.set(k, ii, a.get(jj, k));
            a.set(jj, k, tmp);
        }

        for k in jj + 1..n {
            let tmp = a.get(k, jj);
            a.set(k, jj, a.get(k, ii));
            a.set(k, ii, tmp);
        }

        let tmp = a.get(jj, jj);
        a.set(jj, jj, a.get(ii, ii));
        a.set(ii, ii, tmp);
    }

    Ok(())
}

pub fn pcholesky_decomp2(a: &mut Matrix, p: &mut Permutation, s: &mut Vector) -> Result<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if s.size() != n {
        return Err(gsl::Error::BadLength);
    }

    a.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit, a)?;
    cholesky_scale(a, s)?;
    cholesky_scale_apply(a, s)?;
    pcholesky_decomp(false, a, p)?;

    Ok(())
}

pub fn pcholesky_solve(
    ldlt: &Matrix,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> Result<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if b.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if x.size() != n {
        return Err(gsl::Error::BadLength);
    }

    x.copy(b)?;
    pcholesky_svx(ldlt, p, x)
}

pub fn pcholesky_svx(ldlt: &Matrix, p: &Permutation, x: &mut Vector) -> Result<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if x.size() != n {
        return Err(gsl::Error::BadLength);
    }

    p.permute_vector(x)?;
    x.trsv(CblasUplo::Lower, CblasTranspose::NoTrans, CblasDiag::Unit, ldlt)?;
    x.div_elements(&ldlt.diagonal())?;
    x.trsv(
        CblasUplo::Lower,
        CblasTranspose::Trans,
        CblasDiag::Unit,
        ldlt,
    )?;
    p.inverse_permute_vector(x)?;

    Ok(())
}

pub fn pcholesky_solve2(
    ldlt: &Matrix,
    p: &Permutation,
    s: &Vector,
    b: &Vector,
    x: &mut Vector,
) -> Result<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if s.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if b.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if x.size() != n {
        return Err(gsl::Error::BadLength);
    }

    x.copy(b)?;
    pcholesky_svx2(ldlt, p, s, x)
}

pub fn pcholesky_svx2(
    ldlt: &Matrix,
    p: &Permutation,
    s: &Vector,
    x: &mut Vector,
) -> Result<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if s.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if x.size() != n {
        return Err(gsl::Error::BadLength);
    }

    x.mul_elements(s)?;
    pcholesky_svx(ldlt, p, x)?;
    x.mul_elements(s)?;

    Ok(())
}

pub fn pcholesky_invert(ldlt: &Matrix, p: &Permutation, ainv: &mut Matrix) -> Result<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if p.size() != n {
        return Err(gsl::Error::BadLength);
    }
    if ainv.size1() != ainv.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if ainv.size1() != n {
        return Err(gsl::Error::BadLength);
    }

    ainv.copy(ldlt)?;
    tri_invert(CblasUplo::Lower, CblasDiag::Unit, ainv)?;

    for i in 0..n {
        let di = ldlt.get(i, i);
        let invsqrt_di = 1.0 / di.sqrt();
        
        if i > 0 {
            let mut v = ainv.subrow(i, 0, i)?;
            v.scale(invsqrt_di)?;
        }
        
        ainv.set(i, i, invsqrt_di);
    }

    tri_LTL(ainv)?;
    ainv.transpose_tricpy(CblasUplo::Lower, CblasDiag::Unit, ainv)?;

    for i in 0..n {
        let mut v = ainv.row(i)?;
        p.inverse_permute_vector(&mut v)?;
    }

    for i in 0..n {
        let mut v = ainv.column(i)?;
        p.inverse_permute_vector(&mut v)?;
    }

    Ok(())
}

pub fn pcholesky_rcond(
    ldlt: &Matrix,
    p: &Permutation,
    rcond: &mut f64,
    work: &mut Vector,
) -> Result<()> {
    let n = ldlt.size1();
    if n != ldlt.size2() {
        return Err(gsl::Error::NotSquare);
    }
    if work.size() != 3 * n {
        return Err(gsl::Error::BadLength);
    }

    let anorm = cholesky_ldlt_norm1(ldlt, p, work)?;
    *rcond = 0.0;

    if anorm == 0.0 {
        return Ok(());
    }

    let params = PCholeskyParams { ldlt, perm: p };
    let mut ainvnorm = 0.0;

    invnorm1(
        n,
        cholesky_ldlt_ainv,
        &params,
        &mut ainvnorm,
        work,
    )?;

    if ainvnorm != 0.0 {
        *rcond = 1.0 / anorm / ainvnorm;
    }

    Ok(())
}

fn cholesky_ldlt_norm1(
    ldlt: &Matrix,
    p: &Permutation,
    work: &mut Vector,
) -> Result<f64> {
    let n = ldlt.size1();
    let d = ldlt.diagonal();
    let mut diag_a = work.subvector(n, n)?;

    let mut max = 0.0;

    for j in 0..n {
        let mut ajj = d.get(j);

        for i in 0..j {
            let di = d.get(i);
            let lji = ldlt.get(j, i);
            ajj += di * lji * lji;
        }

        diag_a.set(j, ajj);
    }

    p.inverse_permute_vector(&mut diag_a)?;

    for j in 0..n {
        let mut sum = 0.0;
        let ajj = diag_a.get(j);

        for i in 0..j {
            let aij = ldlt.get(i, j);
            let abs_aij = aij.abs();
            sum += abs_aij;
            work.set(i, work.get(i) + abs_aij)?;
        }

        work.set(j, sum + ajj.abs())?;
    }

    for i in 0..n {
        let wi = work.get(i);
        max = max.max(wi);
    }

    Ok(max)
}

fn cholesky_ldlt_ainv(
    transa: CblasTranspose,
    x: &mut Vector,
    params: &PCholeskyParams,
) -> Result<()> {
    pcholesky_svx(params.ldlt, params.perm, x)
}

struct PCholeskyParams<'a> {
    ldlt: &'a Matrix,
    perm: &'a Permutation,
}
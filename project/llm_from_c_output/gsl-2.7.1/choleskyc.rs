use ndarray::{Array2, Array1, Axis};
use ndarray_linalg::{cholesky::*, solve::*, inverse::*};
use ndarray_linalg::error::Result;
use num_complex::Complex64;
use num_traits::Zero;

/// Performs the Cholesky decomposition on a Hermitian positive definite matrix
pub fn complex_cholesky_decomp(a: &mut Array2<Complex64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(ndarray_linalg::error::LinalgError::NotSquare);
    }
    a.cholesky(UPLO::Lower)?;
    Ok(())
}

/// Solves A x = b where A is in Cholesky form
pub fn complex_cholesky_solve(
    cholesky: &Array2<Complex64>,
    b: &Array1<Complex64>,
) -> Result<Array1<Complex64>> {
    let n = cholesky.nrows();
    if n != cholesky.ncols() {
        return Err(ndarray_linalg::error::LinalgError::NotSquare);
    }
    if n != b.len() {
        return Err(ndarray_linalg::error::LinalgError::IncompatibleShape);
    }
    
    let mut x = b.clone();
    complex_cholesky_svx(cholesky, &mut x)?;
    Ok(x)
}

/// Solves A x = b in place where A is in Cholesky form
pub fn complex_cholesky_svx(
    cholesky: &Array2<Complex64>,
    x: &mut Array1<Complex64>,
) -> Result<()> {
    let n = cholesky.nrows();
    if n != cholesky.ncols() {
        return Err(ndarray_linalg::error::LinalgError::NotSquare);
    }
    if n != x.len() {
        return Err(ndarray_linalg::error::LinalgError::IncompatibleShape);
    }

    // Solve for y using forward-substitution, L y = b
    solve_triangular(
        cholesky,
        UPLO::Lower,
        Diag::NonUnit,
        Transpose::No,
        x,
    )?;

    // Perform back-substitution, L^H x = y
    solve_triangular(
        cholesky,
        UPLO::Lower,
        Diag::NonUnit,
        Transpose::Conj,
        x,
    )?;

    Ok(())
}

/// Computes the inverse of a Hermitian positive definite matrix in Cholesky form
pub fn complex_cholesky_invert(llt: &mut Array2<Complex64>) -> Result<()> {
    let n = llt.nrows();
    if n != llt.ncols() {
        return Err(ndarray_linalg::error::LinalgError::NotSquare);
    }

    // Invert the lower triangle
    inverse_triangular(llt, UPLO::Lower, Diag::NonUnit)?;

    // Compute A^{-1} = L^{-H} L^{-1}
    let mut inv = Array2::zeros((n, n));
    for i in 0..n {
        for j in 0..=i {
            let mut sum = Complex64::zero();
            for k in i..n {
                sum += llt[(k, i)].conj() * llt[(k, j)];
            }
            inv[(i, j)] = sum;
        }
    }

    // Copy the Hermitian lower triangle to the upper triangle
    for i in 0..n {
        for j in i+1..n {
            inv[(i, j)] = inv[(j, i)].conj();
        }
    }

    *llt = inv;
    Ok(())
}

/// Internal function to conjugate a complex vector
fn cholesky_complex_conj_vector(v: &mut Array1<Complex64>) {
    for elem in v.iter_mut() {
        *elem = elem.conj();
    }
}

/// Level 2 BLAS implementation of Cholesky decomposition
fn complex_cholesky_decomp_l2(a: &mut Array2<Complex64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(ndarray_linalg::error::LinalgError::NotSquare);
    }

    for j in 0..n {
        let mut ajj = a[(j, j)].re;

        if j > 0 {
            let aj = a.slice(s![j, ..j]);
            let dot = aj.dot(&aj.conj()).re;
            ajj -= dot;
        }

        if ajj <= 0.0 {
            return Err(ndarray_linalg::error::LinalgError::PositiveDefinite);
        }

        ajj = ajj.sqrt();
        a[(j, j)] = Complex64::new(ajj, 0.0);

        if j < n - 1 {
            let mut av = a.slice_mut(s![j+1.., j]);

            if j > 0 {
                let mut aj = a.slice_mut(s![j, ..j]);
                cholesky_complex_conj_vector(&mut aj.to_owned());
                
                let am = a.slice(s![j+1.., ..j]);
                let gemv = am.dot(&aj);
                av -= &gemv;

                cholesky_complex_conj_vector(&mut aj.to_owned());
            }

            av.mapv_inplace(|x| x / ajj);
        }
    }

    Ok(())
}

/// Level 3 BLAS implementation of Cholesky decomposition
fn complex_cholesky_decomp_l3(a: &mut Array2<Complex64>) -> Result<()> {
    const CROSSOVER_CHOLESKY: usize = 64;
    let n = a.nrows();
    
    if n != a.ncols() {
        return Err(ndarray_linalg::error::LinalgError::NotSquare);
    } else if n <= CROSSOVER_CHOLESKY {
        return complex_cholesky_decomp_l2(a);
    } else {
        let n1 = n / 2;
        let n2 = n - n1;
        
        let (mut a11, mut a21, mut a22) = (
            a.slice_mut(s![..n1, ..n1]),
            a.slice_mut(s![n1.., ..n1]),
            a.slice_mut(s![n1.., n1..]),
        );

        // Recursion on A11
        complex_cholesky_decomp_l3(&mut a11.to_owned())?;

        // A21 = A21 * A11^{-1}
        solve_triangular(
            &a11.to_owned(),
            UPLO::Lower,
            Diag::NonUnit,
            Transpose::Conj,
            &mut a21.to_owned(),
        )?;

        // A22 -= A21 A21^H
        let a21_herm = a21.t().mapv(|x| x.conj());
        a22 -= &a21.dot(&a21_herm);

        // Recursion on A22
        complex_cholesky_decomp_l3(&mut a22.to_owned())?;

        Ok(())
    }
}
/*
 * Cholesky Decomposition in Rust
 *
 * Translated from C code with the following copyright notices:
 * Copyright (C) 2000 Thomas Walter
 * Copyright (C) 2000, 2001, 2002, 2003, 2005, 2007 Brian Gough, Gerard Jungman
 * Copyright (C) 2016, 2019 Patrick Alken
 */

use std::cmp;
use std::f64;
use ndarray::{Array2, Array1, ArrayView1, ArrayViewMut1, Axis};
use ndarray_linalg::{cholesky::*, solve::*, norm::*, Inverse};

const CROSSOVER_CHOLESKY: usize = 64;

pub enum Transpose {
    NoTrans,
    Trans,
}

pub enum TriangularPart {
    Lower,
    Upper,
}

pub enum DiagonalType {
    NonUnit,
    Unit,
}

#[derive(Debug)]
pub enum LinalgError {
    NotSquare,
    BadLength,
    NotPositiveDefinite,
    SingularMatrix,
}

pub type Result<T> = std::result::Result<T, LinalgError>;

pub fn cholesky_decomp(a: &mut Array2<f64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(LinalgError::NotSquare);
    }

    cholesky_decomp_l3(a)
}

pub fn cholesky_solve(llt: &Array2<f64>, b: &Array1<f64>, x: &mut Array1<f64>) -> Result<()> {
    if llt.nrows() != llt.ncols() {
        return Err(LinalgError::NotSquare);
    }
    if llt.nrows() != b.len() {
        return Err(LinalgError::BadLength);
    }
    if llt.ncols() != x.len() {
        return Err(LinalgError::BadLength);
    }

    x.assign(b);
    cholesky_svx(llt, x)
}

pub fn cholesky_svx(llt: &Array2<f64>, x: &mut Array1<f64>) -> Result<()> {
    if llt.nrows() != llt.ncols() {
        return Err(LinalgError::NotSquare);
    }
    if llt.ncols() != x.len() {
        return Err(LinalgError::BadLength);
    }

    // Solve L c = b using forward substitution
    solve_triangular(llt, x, TriangularPart::Lower, Transpose::NoTrans, DiagonalType::NonUnit)?;

    // Solve L^T x = c using back substitution
    solve_triangular(llt, x, TriangularPart::Lower, Transpose::Trans, DiagonalType::NonUnit)?;

    Ok(())
}

pub fn cholesky_invert(llt: &mut Array2<f64>) -> Result<()> {
    if llt.nrows() != llt.ncols() {
        return Err(LinalgError::NotSquare);
    }

    // Invert the lower triangle
    tri_invert(llt, TriangularPart::Lower, DiagonalType::NonUnit)?;

    // Compute A^{-1} = L^{-T} L^{-1}
    tri_ltl(llt)?;

    // Copy lower triangle to upper
    transpose_tricpy(llt, TriangularPart::Lower, DiagonalType::Unit);

    Ok(())
}

fn cholesky_decomp_l2(a: &mut Array2<f64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(LinalgError::NotSquare);
    }

    for j in 0..n {
        let ajj = a[(j, j)];

        if j > 0 {
            let (top, bottom) = a.split_at(Axis(0), j);
            let a_j = bottom.index_axis(Axis(0), 0);
            let a_j0j = top.slice(s![.., 0..j]);
            let a_jj = a_j.slice(s![0..j]);

            let mut temp = Array1::zeros(j);
            ndarray::linalg::general_mat_vec_mul(1.0, &a_j0j, &a_jj, -1.0, &mut temp);
            a.slice_mut(s![j.., j]).assign(&temp);
        }

        let ajj = a[(j, j)];
        if ajj <= 0.0 {
            return Err(LinalgError::NotPositiveDefinite);
        }

        let ajj_sqrt = ajj.sqrt();
        a.slice_mut(s![j.., j]).mapv_inplace(|x| x / ajj_sqrt);
    }

    Ok(())
}

fn cholesky_decomp_l3(a: &mut Array2<f64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(LinalgError::NotSquare);
    }

    if n <= CROSSOVER_CHOLESKY {
        return cholesky_decomp_l2(a);
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let (mut a11, mut a21, mut a22) = {
        let (mut a_top, a_bottom) = a.view_mut().split_at(Axis(0), n1);
        let (a11, a12) = a_top.split_at(Axis(1), n1);
        let (a21, a22) = a_bottom.split_at(Axis(1), n1);
        (a11.to_owned(), a21.to_owned(), a22.to_owned())
    };

    cholesky_decomp_l3(&mut a11)?;

    // A21 = A21 * L11^{-T}
    solve_triangular(&a11, &mut a21, TriangularPart::Lower, Transpose::Trans, DiagonalType::NonUnit)?;

    // A22 -= L21 L21^T
    ndarray::linalg::general_mat_mul(-1.0, &a21, &a21.t(), 1.0, &mut a22);

    cholesky_decomp_l3(&mut a22)?;

    // Copy results back into original matrix
    a.slice_mut(s![..n1, ..n1]).assign(&a11);
    a.slice_mut(s![n1.., ..n1]).assign(&a21);
    a.slice_mut(s![n1.., n1..]).assign(&a22);

    Ok(())
}

fn solve_triangular(
    a: &Array2<f64>,
    x: &mut Array1<f64>,
    uplo: TriangularPart,
    trans: Transpose,
    diag: DiagonalType,
) -> Result<()> {
    // Implementation of triangular solve
    // This would typically use BLAS dtrsm/dtrsv in a real implementation
    unimplemented!()
}

fn tri_invert(
    a: &mut Array2<f64>,
    uplo: TriangularPart,
    diag: DiagonalType,
) -> Result<()> {
    // Implementation of triangular matrix inversion
    unimplemented!()
}

fn tri_ltl(a: &mut Array2<f64>) -> Result<()> {
    // Implementation of L^T * L product
    unimplemented!()
}

fn transpose_tricpy(a: &mut Array2<f64>, uplo: TriangularPart, diag: DiagonalType) {
    // Implementation of triangular copy with transpose
    unimplemented!()
}

fn cholesky_norm1(llt: &Array2<f64>, work: &mut Array1<f64>) -> f64 {
    let n = llt.nrows();
    let mut max = 0.0;

    work.fill(0.0);

    for j in 0..n {
        let mut sum = 0.0;
        let lj = llt.slice(s![j, ..=j]);
        let ajj = lj.dot(&lj);

        for i in 0..j {
            let aij = llt[(i, j)].abs();
            sum += aij;
            work[i] += aij;
        }

        work[j] = sum + ajj.abs();
    }

    for i in 0..n {
        max = max.max(work[i]);
    }

    max
}
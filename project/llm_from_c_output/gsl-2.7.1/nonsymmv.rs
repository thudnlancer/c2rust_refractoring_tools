use std::cmp::max;
use std::f64::{MIN_POSITIVE, EPSILON};
use std::ptr::null_mut;
use std::mem::MaybeUninit;

use gsl::{
    blas::{daxpy, dgemv, dscal, dnrm2, zdscal},
    complex::{Complex, ComplexF64},
    linalg::schur_solve_equation,
    matrix::{Matrix, MatrixComplex, MatrixView, MatrixViewMut},
    vector::{Vector, VectorComplex, VectorView, VectorViewMut},
    error::{Error, Result},
};

const GSL_NONSYMMV_SMLNUM: f64 = 2.0 * MIN_POSITIVE;
const GSL_NONSYMMV_BIGNUM: f64 = (1.0 - EPSILON) / GSL_NONSYMMV_SMLNUM;

pub struct NonsymmvWorkspace {
    size: usize,
    z: Option<Matrix>,
    nonsymm_workspace: NonsymmWorkspace,
    work: Vector,
    work2: Vector,
    work3: Vector,
}

impl NonsymmvWorkspace {
    pub fn new(n: usize) -> Result<Self> {
        if n == 0 {
            return Err(Error::new("matrix dimension must be positive integer"));
        }

        let nonsymm_workspace = NonsymmWorkspace::new(n)?;
        let work = Vector::new(n)?;
        let work2 = Vector::new(n)?;
        let work3 = Vector::new(n)?;

        let mut w = Self {
            size: n,
            z: None,
            nonsymm_workspace,
            work,
            work2,
            work3,
        };

        w.nonsymm_workspace.set_params(true, false)?;

        Ok(w)
    }

    pub fn set_params(&mut self, balance: bool) -> Result<()> {
        self.nonsymm_workspace.set_params(true, balance)
    }

    pub fn eigen_nonsymmv(
        &mut self,
        a: &mut Matrix,
        eval: &mut VectorComplex,
        evec: &mut MatrixComplex,
    ) -> Result<()> {
        let n = a.size1();

        if n != a.size2() {
            return Err(Error::new("matrix must be square"));
        }
        if eval.size() != n {
            return Err(Error::new("eigenvalue vector size mismatch"));
        }
        if evec.size1() != evec.size2() || evec.size1() != n {
            return Err(Error::new("eigenvector matrix size mismatch"));
        }

        let mut z = Matrix::view_mut(
            evec.data_mut(),
            n,
            n,
            2 * n,
        );

        let s = self.nonsymm_workspace.eigen_nonsymm_z(a, eval, &mut z)?;

        if let Some(z_save) = &self.z {
            z.copy_from(z_save)?;
        }

        if s == Ok(()) {
            nonsymmv_get_right_eigenvectors(a, &z, eval, evec, self)?;
            nonsymmv_normalize_eigenvectors(eval, evec)?;
        }

        s
    }

    pub fn eigen_nonsymmv_z(
        &mut self,
        a: &mut Matrix,
        eval: &mut VectorComplex,
        evec: &mut MatrixComplex,
        z: &mut Matrix,
    ) -> Result<()> {
        let n = a.size1();

        if n != a.size2() {
            return Err(Error::new("matrix must be square"));
        }
        if eval.size() != n {
            return Err(Error::new("eigenvalue vector size mismatch"));
        }
        if evec.size1() != evec.size2() || evec.size1() != n {
            return Err(Error::new("eigenvector matrix size mismatch"));
        }
        if z.size1() != z.size2() || z.size1() != n {
            return Err(Error::new("Z matrix size mismatch"));
        }

        self.z = Some(z.clone());
        let result = self.eigen_nonsymmv(a, eval, evec);
        self.z = None;

        result
    }
}

fn nonsymmv_get_right_eigenvectors(
    t: &Matrix,
    z: &Matrix,
    eval: &mut VectorComplex,
    evec: &mut MatrixComplex,
    w: &mut NonsymmvWorkspace,
) -> Result<()> {
    let n = t.size1();
    let smlnum = MIN_POSITIVE * (n as f64) / EPSILON;
    let bignum = (1.0 - EPSILON) / smlnum;

    // Compute 1-norm of each column of upper triangular part of T
    w.work3.set(0, 0.0)?;
    for ju in 1..n {
        let mut sum = 0.0;
        for iu in 0..ju {
            sum += t.get(iu, ju)?.abs();
        }
        w.work3.set(ju, sum)?;
    }

    let mut i = n as isize - 1;
    while i >= 0 {
        let iu = i as usize;
        let lambda_re = t.get(iu, iu)?;
        let lambda_im = if iu != 0 && t.get(iu, iu - 1)? != 0.0 {
            (t.get(iu, iu - 1)?.abs() * t.get(iu - 1, iu)?.abs()).sqrt()
        } else {
            0.0
        };

        let lambda = ComplexF64::new(lambda_re, lambda_im);
        let smin = max(
            EPSILON * (lambda_re.abs() + lambda_im.abs()),
            max(smlnum, GSL_NONSYMMV_SMLNUM),
        );

        if lambda_im == 0.0 {
            // Real eigenvector
            eval.set(iu, lambda)?;

            // Construct right hand side
            for k in 0..i as usize {
                w.work.set(k, -t.get(k, iu)?)?;
            }
            w.work.set(iu, 1.0)?;

            let mut l = i - 1;
            while l >= 0 {
                let lu = l as usize;
                let complex_pair = lu != 0 && t.get(lu, lu - 1)? != 0.0;

                if !complex_pair {
                    // 1x1 diagonal block
                    let tv = t.submatrix(lu, lu, 1, 1);
                    let b = w.work.get(lu)?;
                    let mut x = 0.0;
                    let mut scale = 0.0;
                    let mut xnorm = 0.0;

                    schur_solve_equation(
                        1.0,
                        &tv,
                        lambda_re,
                        1.0,
                        1.0,
                        &[b],
                        &mut x,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    )?;

                    if xnorm > 1.0 && w.work3.get(lu)? > bignum / xnorm {
                        x /= xnorm;
                        scale /= xnorm;
                    }

                    if scale != 1.0 {
                        dscal(scale, &mut w.work.as_mut_slice()[0..=iu as usize])?;
                    }

                    w.work.set(lu, x)?;

                    if lu > 0 {
                        let v1 = t.column(lu)?;
                        let mut v2 = w.work.subvector_mut(0, lu)?;
                        daxpy(-x, &v1, &mut v2)?;
                    }
                } else {
                    // 2x2 diagonal block
                    let tv = t.submatrix(lu - 1, lu - 1, 2, 2);
                    let b1 = w.work.get(lu - 1)?;
                    let b2 = w.work.get(lu)?;
                    let mut x = [0.0; 2];
                    let mut scale = 0.0;
                    let mut xnorm = 0.0;

                    schur_solve_equation(
                        1.0,
                        &tv,
                        lambda_re,
                        1.0,
                        1.0,
                        &[b1, b2],
                        &mut x,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    )?;

                    let x11 = x[0];
                    let x21 = x[1];

                    if xnorm > 1.0 {
                        let beta = max(w.work3.get(lu - 1)?, w.work3.get(lu)?);
                        if beta > bignum / xnorm {
                            x[0] /= xnorm;
                            x[1] /= xnorm;
                            scale /= xnorm;
                        }
                    }

                    if scale != 1.0 {
                        dscal(scale, &mut w.work.as_mut_slice()[0..=iu as usize])?;
                    }

                    w.work.set(lu - 1, x11)?;
                    w.work.set(lu, x21)?;

                    if lu > 1 {
                        let v1 = t.column(lu - 1)?;
                        let v4 = t.column(lu)?;
                        let mut v2 = w.work.subvector_mut(0, lu - 1)?;
                        daxpy(-x11, &v1, &mut v2)?;
                        daxpy(-x21, &v4, &mut v2)?;
                    }

                    l -= 1;
                }
                l -= 1;
            }

            // Backtransform eigenvector
            let mut y = z.column_mut(iu)?;
            if iu > 0 {
                let zv = z.submatrix(0, 0, n, iu);
                let x = w.work.subvector(0, iu)?;
                dgemv(
                    false,
                    1.0,
                    &zv,
                    &x,
                    w.work.get(iu)?,
                    &mut y,
                )?;
            }

            // Store eigenvector
            let mut ecol = evec.column_mut(iu)?;
            let mut scale = 0.0;
            for ii in 0..n {
                let a = y.get(ii)?;
                ecol.set_real(ii, a)?;
                ecol.set_imag(ii, 0.0)?;
                scale = scale.max(a.abs());
            }

            if scale != 0.0 {
                dscal(1.0 / scale, &mut ecol.real_mut())?;
            }
        } else {
            // Complex eigenvector
            let lambda2 = ComplexF64::new(lambda_re, -lambda_im);
            eval.set(iu - 1, lambda)?;
            eval.set(iu, lambda2)?;

            // Initialize eigenvector
            if t.get(iu - 1, iu)?.abs() >= t.get(iu, iu - 1)?.abs() {
                w.work.set(iu - 1, 1.0)?;
                w.work2.set(iu, lambda_im / t.get(iu - 1, iu)?)?;
            } else {
                w.work.set(iu - 1, -lambda_im / t.get(iu, iu - 1)?)?;
                w.work2.set(iu, 1.0)?;
            }
            w.work.set(iu, 0.0)?;
            w.work2.set(iu - 1, 0.0)?;

            // Construct right hand side
            for k in 0..iu - 1 {
                w.work.set(
                    k,
                    -w.work.get(iu - 1)? * t.get(k, iu - 1)?,
                )?;
                w.work2.set(
                    k,
                    -w.work2.get(iu)? * t.get(k, iu)?,
                )?;
            }

            let mut l = i - 2;
            while l >= 0 {
                let lu = l as usize;
                let complex_pair = lu != 0 && t.get(lu, lu - 1)? != 0.0;

                if !complex_pair {
                    // 1x1 diagonal block
                    let tv = t.submatrix(lu, lu, 1, 1);
                    let b = ComplexF64::new(w.work.get(lu)?, w.work2.get(lu)?);
                    let mut x = ComplexF64::default();
                    let mut scale = 0.0;
                    let mut xnorm = 0.0;

                    schur_solve_equation_z(
                        1.0,
                        &tv,
                        &lambda,
                        1.0,
                        1.0,
                        &[b],
                        &mut x,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    )?;

                    if xnorm > 1.0 && w.work3.get(lu)? > bignum / xnorm {
                        x = x.scale(1.0 / xnorm);
                        scale /= xnorm;
                    }

                    if scale != 1.0 {
                        dscal(scale, &mut w.work.as_mut_slice()[0..=iu as usize])?;
                        dscal(scale, &mut w.work2.as_mut_slice()[0..=iu as usize])?;
                    }

                    w.work.set(lu, x.re())?;
                    w.work2.set(lu, x.im())?;

                    if lu > 0 {
                        let v1 = t.column(lu)?;
                        let mut v2 = w.work.subvector_mut(0, lu)?;
                        daxpy(-x.re(), &v1, &mut v2)?;
                        let mut v3 = w.work2.subvector_mut(0, lu)?;
                        daxpy(-x.im(), &v1, &mut v3)?;
                    }
                } else {
                    // 2x2 diagonal block
                    let tv = t.submatrix(lu - 1, lu - 1, 2, 2);
                    let b1 = ComplexF64::new(w.work.get(lu - 1)?, w.work2.get(lu - 1)?);
                    let b2 = ComplexF64::new(w.work.get(lu)?, w.work2.get(lu)?);
                    let mut x = [ComplexF64::default(); 2];
                    let mut scale = 0.0;
                    let mut xnorm = 0.0;

                    schur_solve_equation_z(
                        1.0,
                        &tv,
                        &lambda,
                        1.0,
                        1.0,
                        &[b1, b2],
                        &mut x,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    )?;

                    let x1 = x[0];
                    let x2 = x[1];

                    if xnorm > 1.0 {
                        let beta = max(w.work3.get(lu - 1)?, w.work3.get(lu)?);
                        if beta > bignum / xnorm {
                            x[0] = x1.scale(1.0 / xnorm);
                            x[1] = x2.scale(1.0 / xnorm);
                            scale /= xnorm;
                        }
                    }

                    if scale != 1.0 {
                        dscal(scale, &mut w.work.as_mut_slice()[0..=iu as usize])?;
                        dscal(scale, &mut w.work2.as_mut_slice()[0..=iu as usize])?;
                    }

                    w.work.set(lu - 1, x1.re())?;
                    w.work.set(lu, x2.re())?;
                    w.work2.set(lu - 1, x1.im())?;
                    w.work2.set(lu, x2.im())?;

                    if lu > 1 {
                        let v1 = t.column(lu - 1)?;
                        let v4 = t.column(lu)?;
                        let mut v2 = w.work.subvector_mut(0, lu - 1)?;
                        let mut v3 = w.work2.subvector_mut(0, lu - 1)?;

                        daxpy(-x1.re(), &v1, &mut v2)?;
                        daxpy(-x2.re(), &v4, &mut v2)?;
                        daxpy(-x1.im(), &v1, &mut v3)?;
                        daxpy(-x2.im(), &v4, &mut v3)?;
                    }

                    l -= 1;
                }
                l -= 1;
            }

            // Backtransform eigenvectors
            let mut y = z.column_mut(iu - 1)?;
            let mut y2 = z.column_mut(iu)?;

            if iu > 1 {
                let zv = z.submatrix(0, 0, n, iu - 1);
                let x = w.work.subvector(0, iu - 1)?;
                let x2 = w.work2.subvector(0, iu - 1)?;

                dgemv(
                    false,
                    1.0,
                    &zv,
                    &x,
                    w.work.get(iu - 1)?,
                    &mut y,
                )?;
                dgemv(
                    false,
                    1.0,
                    &zv,
                    &x2,
                    w.work2.get(iu)?,
                    &mut y2,
                )?;
            } else {
                dscal(w.work.get(iu - 1)?, &mut y)?;
                dscal(w.work2.get(iu)?, &mut y2)?;
            }

            // Store eigenvectors
            let mut ecol = evec.column_mut(iu - 1)?;
            let mut ecol2 = evec.column_mut(iu)?;
            let mut scale = 0.0;

            for ii in 0..n {
                let a = y.get(ii)?;
                let b = y2.get(ii)?;
                scale = scale.max(a.abs() + b.abs());

                ecol.set_real(ii, a)?;
                ecol.set_imag(ii, b)?;
                ecol2.set_real(ii, a)?;
                ecol2.set
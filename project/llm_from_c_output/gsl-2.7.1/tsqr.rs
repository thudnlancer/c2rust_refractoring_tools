/*
 * Rust implementation of TSQR algorithm based on the original C code
 * from GNU Scientific Library (GSL)
 */

use ndarray::{Array2, Array1, ArrayView2, ArrayView1, Axis, s};
use ndarray_linalg::{QR, Solve, Norm, Scalar};
use ndarray_linalg::svd::SVD;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsqrError {
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("dimension mismatch: {0}")]
    DimensionMismatch(String),
    #[error("linear algebra error: {0}")]
    LinearAlgebraError(String),
    #[error("allocation error")]
    AllocationError,
}

pub struct TsqrState {
    p: usize,                   // number of columns of LS matrix
    nblocks: i32,               // number of blocks processed
    rnorm: f64,                 // residual norm ||b - A x||
    svd_computed: bool,         // whether SVD of R has been computed

    r: Array2<f64>,             // R factor, p x p
    qtb: Array1<f64>,           // Q^T b vector, length p
    t: Array2<f64>,             // block reflector matrix, p x p
    work: Array1<f64>,          // workspace, length p
    work3: Array1<f64>,         // workspace, length 3*p

    svd_result: Option<SVD<Array2<f64>>>, // SVD decomposition of R
}

impl TsqrState {
    pub fn new(p: usize) -> Result<Self, TsqrError> {
        if p == 0 {
            return Err(TsqrError::InvalidParameter("p must be positive".to_string()));
        }

        Ok(TsqrState {
            p,
            nblocks: 0,
            rnorm: 0.0,
            svd_computed: false,
            r: Array2::zeros((p, p)),
            qtb: Array1::zeros(p),
            t: Array2::zeros((p, p)),
            work: Array1::zeros(p),
            work3: Array1::zeros(3 * p),
            svd_result: None,
        })
    }

    pub fn reset(&mut self) {
        self.r.fill(0.0);
        self.qtb.fill(0.0);
        self.nblocks = 0;
        self.rnorm = 0.0;
        self.svd_computed = false;
        self.svd_result = None;
    }

    pub fn accumulate(&mut self, a: &mut Array2<f64>, b: &mut Array1<f64>) -> Result<(), TsqrError> {
        let n = a.shape()[0];
        let p = a.shape()[1];

        if p != self.p {
            return Err(TsqrError::DimensionMismatch(
                "columns of A do not match workspace".to_string(),
            ));
        }
        if n != b.len() {
            return Err(TsqrError::DimensionMismatch(
                "A and b have different numbers of rows".to_string(),
            ));
        }
        if self.nblocks == 0 && n < p {
            return Err(TsqrError::DimensionMismatch("n must be >= p".to_string()));
        }

        if self.nblocks == 0 {
            // First block - compute dense QR decomposition
            let (q, r) = a.qr()?;
            self.r.assign(&r);
            
            // Compute Q^T b
            let qtb = q.t().dot(b);
            self.qtb.assign(&qtb.slice(s![..p]));
            
            // Compute residual norm for remaining elements
            if n > p {
                self.rnorm = b.slice(s![p..]).norm();
            } else {
                self.rnorm = 0.0;
            }
            
            self.nblocks = 1;
        } else {
            // Subsequent blocks - compute QR of [R_{i-1}; A_i]
            let mut combined = Array2::zeros((p + n, p));
            combined.slice_mut(s![..p, ..]).assign(&self.r);
            combined.slice_mut(s![p.., ..]).assign(a);
            
            let (_, r_new) = combined.qr()?;
            self.r.assign(&r_new.slice(s![..p, ..]));
            
            // Update Q^T b
            let mut w = self.qtb.clone();
            w += &a.t().dot(b)?;
            w = self.t.t().dot(&w);
            self.qtb -= &w;
            
            // Update residual norm
            let mut b_update = b.clone();
            b_update -= &a.dot(&w);
            self.rnorm = (self.rnorm.powi(2) + b_update.norm().powi(2)).sqrt();
        }
        
        Ok(())
    }

    pub fn solve(
        &mut self,
        lambda: f64,
        x: &mut Array1<f64>,
    ) -> Result<(f64, f64), TsqrError> {
        if x.len() != self.p {
            return Err(TsqrError::DimensionMismatch(
                "solution vector does not match workspace".to_string(),
            ));
        }
        if lambda < 0.0 {
            return Err(TsqrError::InvalidParameter(
                "regularization parameter must be non-negative".to_string(),
            ));
        }

        if lambda == 0.0 {
            // Solve R x = Q^T b
            x.assign(&self.qtb);
            let rnorm = self.rnorm;
            let snorm = x.norm();
            x.solve_triangular_into(ndarray_linalg::UPLO::Upper, &self.r)?;
            Ok((rnorm, snorm))
        } else {
            // Compute SVD if not already done
            if !self.svd_computed {
                self.svd_result = Some(self.r.svd(true, true)?);
                self.svd_computed = true;
            }

            // Use SVD to solve regularized system
            let svd = self.svd_result.as_ref().unwrap();
            let (rnorm, snorm) = svd.solve_regularized(lambda, &self.qtb, x)?;
            let total_rnorm = (rnorm.powi(2) + self.rnorm.powi(2)).sqrt();
            Ok((total_rnorm, snorm))
        }
    }

    pub fn rcond(&self) -> Result<f64, TsqrError> {
        // Compute reciprocal condition number of R
        if !self.svd_computed {
            return Err(TsqrError::LinearAlgebraError(
                "SVD not computed yet".to_string(),
            ));
        }
        let svd = self.svd_result.as_ref().unwrap();
        let s = svd.s();
        let smax = s[0];
        let smin = s[s.len() - 1];
        Ok(smin / smax)
    }

    pub fn r(&self) -> &Array2<f64> {
        &self.r
    }

    pub fn qtb(&self) -> &Array1<f64> {
        &self.qtb
    }
}

// Helper trait for QR decomposition with error handling
trait QRDecomp {
    fn qr(&self) -> Result<(Array2<f64>, Array2<f64>), TsqrError>;
}

impl QRDecomp for Array2<f64> {
    fn qr(&self) -> Result<(Array2<f64>, Array2<f64>), TsqrError> {
        let (q, r) = self.qr().map_err(|e| TsqrError::LinearAlgebraError(e.to_string()))?;
        Ok((q, r))
    }
}

// Helper trait for triangular solve with error handling
trait SolveTriangular {
    fn solve_triangular_into(
        &mut self,
        uplo: ndarray_linalg::UPLO,
        a: &Array2<f64>,
    ) -> Result<(), TsqrError>;
}

impl SolveTriangular for Array1<f64> {
    fn solve_triangular_into(
        &mut self,
        uplo: ndarray_linalg::UPLO,
        a: &Array2<f64>,
    ) -> Result<(), TsqrError> {
        a.solve_triangular_into(uplo, self)
            .map_err(|e| TsqrError::LinearAlgebraError(e.to_string()))
    }
}

// Helper trait for SVD with regularization
trait SVDSolve {
    fn solve_regularized(
        &self,
        lambda: f64,
        rhs: &Array1<f64>,
        x: &mut Array1<f64>,
    ) -> Result<(f64, f64), TsqrError>;
}

impl SVDSolve for SVD<Array2<f64>> {
    fn solve_regularized(
        &self,
        lambda: f64,
        rhs: &Array1<f64>,
        x: &mut Array1<f64>,
    ) -> Result<(f64, f64), TsqrError> {
        let u = self.u.as_ref().ok_or_else(|| {
            TsqrError::LinearAlgebraError("SVD missing U matrix".to_string())
        })?;
        let vt = self.vt.as_ref().ok_or_else(|| {
            TsqrError::LinearAlgebraError("SVD missing Vt matrix".to_string())
        })?;
        let s = &self.s;

        // Compute regularized solution
        let ut_b = u.t().dot(rhs);
        let mut x_reg = Array1::zeros(x.len());
        
        for i in 0..s.len() {
            let si = s[i];
            let filter = si / (si.powi(2) + lambda.powi(2));
            x_reg += &(vt.row(i).to_owned() * (filter * ut_b[i]));
        }

        x.assign(&x_reg);
        
        // Compute residual and solution norms
        let rnorm = (rhs - &self.a().dot(x)).norm();
        let snorm = x.norm();
        
        Ok((rnorm, snorm))
    }
}
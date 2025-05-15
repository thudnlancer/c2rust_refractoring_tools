use std::ptr;
use std::mem;
use std::ops::{Add, Mul};
use std::fmt;
use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_int;

use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use ndarray_linalg::{cholesky::*, solve::*, Norm, Scalar};
use ndarray_linalg::types::c32;
use ndarray_linalg::error::LinalgError;
use ndarray_linalg::eigh::*;
use ndarray_linalg::svd::*;
use ndarray_linalg::least_squares::*;

#[derive(Debug)]
struct NormalState {
    p: usize,
    ata: Array2<f64>,
    atb: Array1<f64>,
    normb: f64,
    work_ata: Array2<f64>,
    workp: Array1<f64>,
    work3p: Array1<f64>,
    d: Array1<f64>,
    c: Array1<f64>,
    eigen: bool,
    eval_min: f64,
    eval_max: f64,
}

impl NormalState {
    fn new(p: usize) -> Result<Self, Box<dyn Error>> {
        if p == 0 {
            return Err("p must be a positive integer".into());
        }

        Ok(NormalState {
            p,
            ata: Array2::zeros((p, p)),
            atb: Array1::zeros(p),
            normb: 0.0,
            work_ata: Array2::zeros((p, p)),
            workp: Array1::zeros(p),
            work3p: Array1::zeros(3 * p),
            d: Array1::zeros(p),
            c: Array1::zeros(p),
            eigen: false,
            eval_min: 0.0,
            eval_max: 0.0,
        })
    }

    fn reset(&mut self) {
        self.ata.fill(0.0);
        self.atb.fill(0.0);
        self.normb = 0.0;
        self.eigen = false;
        self.eval_min = 0.0;
        self.eval_max = 0.0;
    }

    fn accumulate(&mut self, a: &Array2<f64>, b: &Array1<f64>) -> Result<(), Box<dyn Error>> {
        if a.shape()[1] != self.p {
            return Err("columns of A do not match workspace".into());
        }
        if a.shape()[0] != b.len() {
            return Err("A and b have different numbers of rows".into());
        }

        // ATA += A^T A
        let ata_update = a.t().dot(a);
        self.ata += &ata_update;

        // ATb += A^T b
        let atb_update = a.t().dot(b);
        self.atb += &atb_update;

        // Update ||b||
        let b_norm = b.norm_l2();
        self.normb = (self.normb.powi(2) + b_norm.powi(2)).sqrt();

        Ok(())
    }

    fn solve(&mut self, lambda: f64, x: &mut Array1<f64>) -> Result<(f64, f64), Box<dyn Error>> {
        if x.len() != self.p {
            return Err("solution vector does not match workspace".into());
        }

        self.solve_system(lambda, x)?;
        let (rnorm, snorm) = self.calc_norms(x)?;

        Ok((rnorm, snorm))
    }

    fn rcond(&mut self) -> Result<f64, Box<dyn Error>> {
        let rcond_ata = self.work_ata.cholesky()?.rcond()?;
        Ok(rcond_ata.sqrt())
    }

    fn lcurve(
        &mut self,
        reg_param: &mut Array1<f64>,
        rho: &mut Array1<f64>,
        eta: &mut Array1<f64>,
    ) -> Result<(), Box<dyn Error>> {
        if !self.eigen {
            self.eigen()?;
        }

        if self.eval_max < 0.0 {
            return Err("matrix is not positive definite".into());
        }

        // Compute singular values (sqrt of eigenvalues)
        let smax = self.eval_max.sqrt();
        let smin = if self.eval_min > 0.0 {
            self.eval_min.sqrt()
        } else {
            0.0
        };

        // Compute regularization parameters
        // (Assuming gsl_multifit_linear_lreg equivalent)
        let n = reg_param.len();
        let smin_log = smin.ln();
        let smax_log = smax.ln();
        let step = (smax_log - smin_log) / (n - 1) as f64;
        for i in 0..n {
            let log_param = smin_log + step * i as f64;
            reg_param[i] = log_param.exp();
        }

        // Solve for each regularization parameter
        for i in 0..reg_param.len() {
            let lambda = reg_param[i];
            self.solve_system(lambda, &mut self.c)?;
            let (rnorm, snorm) = self.calc_norms(&self.c)?;
            rho[i] = rnorm;
            eta[i] = snorm;
        }

        Ok(())
    }

    fn ata(&self) -> &Array2<f64> {
        &self.ata
    }

    fn atb(&self) -> &Array1<f64> {
        &self.atb
    }

    fn solve_system(&mut self, lambda: f64, x: &mut Array1<f64>) -> Result<(), Box<dyn Error>> {
        let lambda_sq = lambda * lambda;

        // Copy ATA to workspace and regularize
        self.work_ata.assign(&self.ata);
        self.work_ata.diag_mut().add_scalar(lambda_sq);

        // Solve with Cholesky
        self.solve_cholesky(&self.work_ata, &self.atb, x)
    }

    fn solve_cholesky(
        &mut self,
        ata: &Array2<f64>,
        atb: &Array1<f64>,
        x: &mut Array1<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let chol = ata.cholesky()?;
        *x = chol.solve(atb)?;
        Ok(())
    }

    fn calc_norms(&self, x: &Array1<f64>) -> Result<(f64, f64), Box<dyn Error>> {
        // Solution norm ||x||
        let snorm = x.norm_l2();

        // Residual norm ||b - Ax||
        // Compute ATA x - 2 ATb
        let mut workp = self.atb.clone();
        let ata_x = self.ata.dot(x);
        workp.scaled_add(-2.0, &ata_x);

        // Compute x^T ATA x - 2 x^T ATb
        let r2 = x.dot(&workp) + self.normb.powi(2);

        let rnorm = r2.sqrt();

        Ok((rnorm, snorm))
    }

    fn eigen(&mut self) -> Result<(), Box<dyn Error>> {
        // Copy lower triangle
        self.work_ata.assign(&self.ata);

        // Compute eigenvalues
        let (evals, _) = self.work_ata.eigh()?;

        // Get min/max eigenvalues
        self.eval_min = *evals.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.eval_max = *evals.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        self.eigen = true;

        Ok(())
    }
}

struct NormalType;

impl NormalType {
    fn new() -> Self {
        NormalType
    }

    fn alloc(&self, p: usize) -> Result<Box<NormalState>, Box<dyn Error>> {
        NormalState::new(p).map(Box::new)
    }

    fn reset(&self, state: &mut NormalState) {
        state.reset();
    }

    fn accumulate(&self, a: &Array2<f64>, b: &Array1<f64>, state: &mut NormalState) -> Result<(), Box<dyn Error>> {
        state.accumulate(a, b)
    }

    fn solve(
        &self,
        lambda: f64,
        x: &mut Array1<f64>,
        state: &mut NormalState,
    ) -> Result<(f64, f64), Box<dyn Error>> {
        state.solve(lambda, x)
    }

    fn rcond(&self, state: &mut NormalState) -> Result<f64, Box<dyn Error>> {
        state.rcond()
    }

    fn lcurve(
        &self,
        reg_param: &mut Array1<f64>,
        rho: &mut Array1<f64>,
        eta: &mut Array1<f64>,
        state: &mut NormalState,
    ) -> Result<(), Box<dyn Error>> {
        state.lcurve(reg_param, rho, eta)
    }

    fn ata(&self, state: &NormalState) -> &Array2<f64> {
        state.ata()
    }

    fn atb(&self, state: &NormalState) -> &Array1<f64> {
        state.atb()
    }
}

// Public interface
pub static GSL_MULTILARGE_LINEAR_NORMAL: NormalType = NormalType;
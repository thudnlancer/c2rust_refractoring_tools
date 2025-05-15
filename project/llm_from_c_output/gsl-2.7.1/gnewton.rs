use std::f64;
use nalgebra::{DVector, DMatrix, LU};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GNewtonError {
    #[error("failed to allocate space")]
    AllocationFailed,
    #[error("bad function evaluation")]
    BadFunction,
    #[error("linear algebra error")]
    LinearAlgebraError,
}

pub struct GNewtonState {
    phi: f64,
    x_trial: DVector<f64>,
    d: DVector<f64>,
    lu: DMatrix<f64>,
}

pub struct MultiRootFunctionFDF {
    pub n: usize,
    pub f: Box<dyn Fn(&DVector<f64>) -> Result<DVector<f64>, GNewtonError>>,
    pub df: Box<dyn Fn(&DVector<f64>) -> Result<DMatrix<f64>, GNewtonError>>,
    pub fdf: Box<dyn Fn(&DVector<f64>) -> Result<(DVector<f64>, DMatrix<f64>), GNewtonError>>,
}

pub struct GNewtonSolver {
    state: GNewtonState,
}

impl GNewtonSolver {
    pub fn new(n: usize) -> Result<Self, GNewtonError> {
        let state = GNewtonState {
            phi: 0.0,
            x_trial: DVector::zeros(n),
            d: DVector::zeros(n),
            lu: DMatrix::zeros(n, n),
        };
        Ok(Self { state })
    }

    pub fn set(
        &mut self,
        fdf: &MultiRootFunctionFDF,
        x: &DVector<f64>,
        f: &mut DVector<f64>,
        j: &mut DMatrix<f64>,
        dx: &mut DVector<f64>,
    ) -> Result<(), GNewtonError> {
        let (f_val, j_val) = (fdf.fdf)(x)?;
        *f = f_val;
        *j = j_val;
        dx.fill(0.0);
        self.state.phi = enorm(f);
        Ok(())
    }

    pub fn iterate(
        &mut self,
        fdf: &MultiRootFunctionFDF,
        x: &mut DVector<f64>,
        f: &mut DVector<f64>,
        j: &mut DMatrix<f64>,
        dx: &mut DVector<f64>,
    ) -> Result<(), GNewtonError> {
        let n = fdf.n;
        self.state.lu.copy_from(j);

        let lu = LU::new(self.state.lu.clone());
        self.state.d = lu.solve(f).map_err(|_| GNewtonError::LinearAlgebraError)?;

        let mut t = 1.0;
        let phi0 = self.state.phi;

        loop {
            for i in 0..n {
                let di = self.state.d[i];
                let xi = x[i];
                self.state.x_trial[i] = xi - t * di;
            }

            *f = (fdf.f)(&self.state.x_trial)?;
            let phi1 = enorm(f);

            if phi1 > phi0 && t > f64::EPSILON {
                let theta = phi1 / phi0;
                let u = (f64::sqrt(1.0 + 6.0 * theta) - 1.0) / (3.0 * theta);
                t *= u;
                continue;
            }

            x.copy_from(&self.state.x_trial);
            for i in 0..n {
                let di = self.state.d[i];
                dx[i] = -t * di;
            }

            *j = (fdf.df)(x)?;
            self.state.phi = phi1;
            return Ok(());
        }
    }
}

fn enorm(v: &DVector<f64>) -> f64 {
    v.norm()
}
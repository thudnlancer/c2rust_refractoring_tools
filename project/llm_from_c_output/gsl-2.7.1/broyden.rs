use std::f64;
use ndarray::{Array1, Array2, s};
use ndarray_linalg::{Determinant, Inverse, LU, Solve};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BroydenError {
    #[error("failed to allocate memory")]
    MemoryAllocationFailed,
    #[error("approximation to Jacobian has collapsed")]
    JacobianCollapsed,
    #[error("function evaluation failed")]
    FunctionEvaluationFailed,
}

struct BroydenState {
    h: Array2<f64>,
    lu: Array2<f64>,
    permutation: Vec<usize>,
    v: Array1<f64>,
    w: Array1<f64>,
    y: Array1<f64>,
    p: Array1<f64>,
    fnew: Array1<f64>,
    x_trial: Array1<f64>,
    phi: f64,
}

impl BroydenState {
    fn new(n: usize) -> Result<Self, BroydenError> {
        Ok(Self {
            h: Array2::zeros((n, n)),
            lu: Array2::zeros((n, n)),
            permutation: vec![0; n],
            v: Array1::zeros(n),
            w: Array1::zeros(n),
            y: Array1::zeros(n),
            p: Array1::zeros(n),
            fnew: Array1::zeros(n),
            x_trial: Array1::zeros(n),
            phi: 0.0,
        })
    }
}

pub struct BroydenSolver {
    state: BroydenState,
}

impl BroydenSolver {
    pub fn new(n: usize) -> Result<Self, BroydenError> {
        let state = BroydenState::new(n)?;
        Ok(Self { state })
    }

    pub fn set(
        &mut self,
        function: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, BroydenError>,
        x: &Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), BroydenError> {
        *f = function(x)?;
        
        // Approximate Jacobian using finite differences
        self.finite_difference_jacobian(function, x, f)?;
        
        // Compute LU decomposition and inverse
        let lu = self.state.lu.clone();
        let (lu, perm) = LU::new(lu).decompose()?;
        self.state.h = lu.inv()?;
        
        // Negate the inverse Jacobian
        self.state.h.mapv_inplace(|v| -v);
        
        // Reset dx
        dx.fill(0.0);
        
        // Compute initial residual norm
        self.state.phi = enorm(f);
        
        Ok(())
    }

    pub fn iterate(
        &mut self,
        function: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, BroydenError>,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), BroydenError> {
        let n = x.len();
        let mut t = 1.0;
        let mut iter = 0;
        let phi0 = self.state.phi;

        // Compute p = H f
        self.state.p = self.state.h.dot(f);

        loop {
            // x_trial = x + t*p
            self.state.x_trial.assign(&x);
            self.state.x_trial.scaled_add(t, &self.state.p);

            // Evaluate function at trial point
            self.state.fnew = function(&self.state.x_trial)?;
            let phi1 = enorm(&self.state.fnew);

            iter += 1;

            if phi1 > phi0 && iter < 10 && t > 0.1 {
                // Reduce step size
                let theta = phi1 / phi0;
                t *= (f64::sqrt(1.0 + 6.0 * theta) - 1.0) / (3.0 * theta);
                continue;
            }

            if phi1 > phi0 {
                // Recompute Jacobian
                self.finite_difference_jacobian(function, x, f)?;
                
                let lu = self.state.lu.clone();
                let (lu, perm) = LU::new(lu).decompose()?;
                self.state.h = lu.inv()?;
                self.state.h.mapv_inplace(|v| -v);
                
                // Solve H f = p
                self.state.p = lu.solve_into(f.clone())?;
                
                t = 1.0;
                
                // Recompute x_trial with new p
                self.state.x_trial.assign(&x);
                self.state.x_trial.scaled_add(t, &self.state.p);
                
                // Re-evaluate function
                self.state.fnew = function(&self.state.x_trial)?;
                self.state.phi = enorm(&self.state.fnew);
            }

            break;
        }

        // y = fnew - f
        self.state.y.assign(&self.state.fnew);
        self.state.y -= f;

        // v = H y
        self.state.v = self.state.h.dot(&self.state.y);

        // lambda = p.v
        let lambda = self.state.p.dot(&self.state.v);

        if lambda.abs() < f64::EPSILON {
            return Err(BroydenError::JacobianCollapsed);
        }

        // v += t*p
        self.state.v.scaled_add(t, &self.state.p);

        // w = H^T p
        self.state.w = self.state.h.t().dot(&self.state.p);

        // Rank-1 update: H -= (v w^T)/lambda
        let update = self.state.v.view().insert_axis(ndarray::Axis(1))
            .dot(&self.state.w.view().insert_axis(ndarray::Axis(0)));
        self.state.h.scaled_add(-1.0/lambda, &update);

        // Update f and x
        f.assign(&self.state.fnew);
        x.assign(&self.state.x_trial);

        // Update dx
        dx.assign(&self.state.p);
        dx.mapv_inplace(|v| t * v);

        self.state.phi = enorm(f);

        Ok(())
    }

    fn finite_difference_jacobian(
        &mut self,
        function: &dyn Fn(&Array1<f64>) -> Result<Array1<f64>, BroydenError>,
        x: &Array1<f64>,
        f: &Array1<f64>,
    ) -> Result<(), BroydenError> {
        let n = x.len();
        let eps = f64::EPSILON.sqrt();
        let mut x_perturbed = x.clone();
        
        for j in 0..n {
            let h = eps * f64::max(f64::abs(x[j]), 1.0);
            let temp = x_perturbed[j];
            
            x_perturbed[j] = temp + h;
            let f_plus = function(&x_perturbed)?;
            
            x_perturbed[j] = temp - h;
            let f_minus = function(&x_perturbed)?;
            
            x_perturbed[j] = temp;
            
            for i in 0..n {
                let derivative = (f_plus[i] - f_minus[i]) / (2.0 * h);
                self.state.lu[[i, j]] = derivative;
            }
        }
        
        Ok(())
    }
}

fn enorm(f: &Array1<f64>) -> f64 {
    f.dot(f).sqrt()
}
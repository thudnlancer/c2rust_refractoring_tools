use std::cmp::max;
use std::f64::consts::ONE_THIRD as LM_ONE_THIRD;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};
use ndarray_linalg::{cholesky::*, dot::*, solve::*, Lapack};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LMError {
    #[error("memory allocation failed")]
    AllocationFailed,
    #[error("linear algebra operation failed")]
    LinearAlgebraError,
    #[error("function evaluation failed")]
    FunctionEvalFailed,
    #[error("parameter overflow")]
    ParameterOverflow,
}

#[derive(Debug)]
pub struct LMState {
    a: Array2<f64>,          // J^T J
    a_copy: Array2<f64>,     // copy of J^T J
    j: Array2<f64>,          // Jacobian J(x)
    diag: Array1<f64>,       // D = diag(J^T J)
    rhs: Array1<f64>,        // rhs vector = -g = -J^T f
    x_trial: Array1<f64>,    // trial parameter vector
    f_trial: Array1<f64>,    // trial function vector
    work: Array1<f64>,       // workspace length p
    nu: i64,                 // nu
    mu: f64,                 // LM damping parameter mu
    tau: f64,                // initial scale factor for mu
}

impl LMState {
    pub fn new(n: usize, p: usize) -> Result<Self, LMError> {
        Ok(Self {
            a: Array2::zeros((p, p)),
            a_copy: Array2::zeros((p, p)),
            j: Array2::zeros((n, p)),
            diag: Array1::zeros(p),
            rhs: Array1::zeros(p),
            x_trial: Array1::zeros(p),
            f_trial: Array1::zeros(n),
            work: Array1::zeros(p),
            nu: 2,
            mu: 0.0,
            tau: 1.0e-3,
        })
    }

    pub fn set(
        &mut self,
        swts: Option<ArrayView1<f64>>,
        fdf: &mut impl FnMut(ArrayView1<f64>) -> Result<Array1<f64>, LMError>,
        jac: &mut impl FnMut(ArrayView1<f64>) -> Result<Array2<f64>, LMError>,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
    ) -> Result<(), LMError> {
        // evaluate function and Jacobian at x and apply weight transform
        *f = fdf(x.view())?;
        
        self.j = jac(x.view())?;
        
        // compute rhs = -J^T f
        self.rhs = -self.j.t().dot(f);

        #[cfg(not(feature = "scale"))]
        {
            self.diag.fill(1.0);
            
            // compute mu_0 = tau * max(diag(J^T J))
            self.mu = self.j
                .axis_iter(Axis(1))
                .map(|col| col.dot(&col))
                .fold(-1.0, |max_val, val| max_val.max(val));
            
            self.mu *= self.tau;
        }

        #[cfg(feature = "scale")]
        {
            self.diag.fill(0.0);
            self.mu = self.tau;
        }

        Ok(())
    }

    pub fn iterate(
        &mut self,
        swts: Option<ArrayView1<f64>>,
        fdf: &mut impl FnMut(ArrayView1<f64>) -> Result<Array1<f64>, LMError>,
        jac: &mut impl FnMut(ArrayView1<f64>) -> Result<Array2<f64>, LMError>,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), LMError> {
        // compute A = J^T J
        self.a = self.j.t().dot(&self.j);
        
        #[cfg(feature = "scale")]
        self.update_diag();

        let mut found_step = false;
        
        while !found_step {
            // solve (A + mu*I) dx = g
            self.calc_dx(dx)?;
            
            // compute x_trial = x + dx
            self.x_trial.assign(&(x + dx));
            
            // compute f(x + dx)
            self.f_trial = fdf(self.x_trial.view())?;
            
            // compute dF = F(x) - F(x + dx)
            let d_f = self.calc_d_f(f, &self.f_trial);
            
            // compute dL = L(0) - L(dx) = dx^T (mu*dx - g)
            let d_l = self.calc_d_l(dx);
            
            if d_l > 0.0 && d_f >= 0.0 {
                // reduction in error, step acceptable
                let tmp = 2.0 * (d_f / d_l) - 1.0;
                let tmp = 1.0 - tmp.powi(3);
                self.mu *= max(LM_ONE_THIRD, tmp);
                self.nu = 2;
                
                // compute J <- J(x + dx)
                self.j = jac(self.x_trial.view())?;
                
                // update x and f
                x.assign(&self.x_trial);
                f.assign(&self.f_trial);
                
                // compute new rhs = -J^T f
                self.rhs = -self.j.t().dot(f);
                
                found_step = true;
            } else {
                let nu2 = self.nu * 2;
                if nu2 <= self.nu {
                    // nu has overflowed, reset
                    self.nu = 2;
                    self.mu = self.tau * self.a.diag().fold(-1.0, |max_val, &val| max_val.max(val));
                    break;
                }
                self.nu = nu2;
                self.mu *= self.nu as f64;
            }
        }
        
        Ok(())
    }

    fn calc_dx(&mut self, dx: &mut Array1<f64>) -> Result<(), LMError> {
        self.a_copy.assign(&self.a);
        
        // add mu to diagonal
        self.a_copy.diag_mut().add_scalar(self.mu);
        
        // Cholesky decomposition
        let chol = self.a_copy.cholesky(UPLO::Lower)?;
        
        // solve system
        *dx = chol.solve(&self.rhs)?;
        
        Ok(())
    }

    fn calc_d_f(&self, f: &Array1<f64>, f_trial: &Array1<f64>) -> f64 {
        0.5 * (f.dot(f) - f_trial.dot(f_trial))
    }

    fn calc_d_l(&self, dx: &Array1<f64>) -> f64 {
        let mu_dx = dx.mapv(|x| self.mu * x);
        let mu_dx_minus_g = &mu_dx - &self.rhs;
        dx.dot(&mu_dx_minus_g)
    }

    #[cfg(feature = "scale")]
    fn update_diag(&mut self) {
        self.diag.assign(&self.a.diag());
    }
}

pub struct LMSolver {
    state: LMState,
}

impl LMSolver {
    pub fn new(n: usize, p: usize) -> Result<Self, LMError> {
        Ok(Self {
            state: LMState::new(n, p)?,
        })
    }

    pub fn gradient(&self, g: &mut Array1<f64>) {
        g.assign(&(-&self.state.rhs));
    }

    pub fn jacobian(&self, j: &mut Array2<f64>) {
        j.assign(&self.state.j);
    }
}
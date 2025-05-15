use std::f64;
use std::error::Error;
use std::fmt;

use ndarray::{Array2, Array1};
use ndarray_linalg::{Determinant, LU, Solve};

#[derive(Debug)]
pub enum MultiRootError {
    AllocationFailed,
    FunctionEvalFailed,
    JacobianFailed,
    LUFailed,
    SolveFailed,
}

impl fmt::Display for MultiRootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MultiRootError::AllocationFailed => write!(f, "Failed to allocate space"),
            MultiRootError::FunctionEvalFailed => write!(f, "Function evaluation failed"),
            MultiRootError::JacobianFailed => write!(f, "Jacobian computation failed"),
            MultiRootError::LUFailed => write!(f, "LU decomposition failed"),
            MultiRootError::SolveFailed => write!(f, "Linear solve failed"),
        }
    }
}

impl Error for MultiRootError {}

pub type MultiRootResult<T> = Result<T, MultiRootError>;

pub struct DNewtonState {
    j: Array2<f64>,
    lu: Array2<f64>,
    permutation: Vec<usize>,
}

pub struct MultiRootFunction {
    pub n: usize,
    pub f: Box<dyn Fn(&Array1<f64>) -> Array1<f64>>,
}

impl DNewtonState {
    pub fn new(n: usize) -> MultiRootResult<Self> {
        let j = Array2::zeros((n, n));
        let lu = Array2::zeros((n, n));
        let permutation = vec![0; n];
        
        Ok(DNewtonState { j, lu, permutation })
    }

    pub fn set(
        &mut self,
        function: &MultiRootFunction,
        x: &Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> MultiRootResult<()> {
        *f = (function.f)(x);
        
        self.fdjacobian(function, x, f, f64::EPSILON.sqrt())?;
        
        for i in 0..function.n {
            dx[i] = 0.0;
        }
        
        Ok(())
    }

    pub fn iterate(
        &mut self,
        function: &MultiRootFunction,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> MultiRootResult<()> {
        self.lu.assign(&self.j);
        
        let lu = LU::new(self.lu.clone());
        let signum = lu.det().map_err(|_| MultiRootError::LUFailed)?;
        
        *dx = lu.solve(f).map_err(|_| MultiRootError::SolveFailed)?;
        
        for i in 0..function.n {
            let e = dx[i];
            let y = x[i];
            dx[i] = -e;
            x[i] = y - e;
        }
        
        *f = (function.f)(x);
        
        self.fdjacobian(function, x, f, f64::EPSILON.sqrt())?;
        
        Ok(())
    }

    fn fdjacobian(
        &mut self,
        function: &MultiRootFunction,
        x: &Array1<f64>,
        f: &Array1<f64>,
        h: f64,
    ) -> MultiRootResult<()> {
        let mut x_perturbed = x.clone();
        let mut f_perturbed = Array1::zeros(function.n);
        
        for j in 0..function.n {
            let xj = x[j];
            let dx = h * xj.abs().max(h);
            
            x_perturbed[j] = xj + dx;
            f_perturbed = (function.f)(&x_perturbed);
            
            for i in 0..function.n {
                self.j[[i, j]] = (f_perturbed[i] - f[i]) / dx;
            }
            
            x_perturbed[j] = xj;
        }
        
        Ok(())
    }
}

pub struct DNewtonSolver {
    state: DNewtonState,
}

impl DNewtonSolver {
    pub fn new(n: usize) -> MultiRootResult<Self> {
        let state = DNewtonState::new(n)?;
        Ok(DNewtonSolver { state })
    }

    pub fn set(
        &mut self,
        function: &MultiRootFunction,
        x: &Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> MultiRootResult<()> {
        self.state.set(function, x, f, dx)
    }

    pub fn iterate(
        &mut self,
        function: &MultiRootFunction,
        x: &mut Array1<f64>,
        f: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> MultiRootResult<()> {
        self.state.iterate(function, x, f, dx)
    }
}
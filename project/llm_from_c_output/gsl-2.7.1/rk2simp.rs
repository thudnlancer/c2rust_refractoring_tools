use std::mem;
use std::ptr;
use std::ops::{Add, Mul, Sub};
use ndarray::{Array1, Array2, ArrayView1, ArrayViewMut1};
use ndarray_linalg::{Determinant, LU, Solve};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OdeError {
    #[error("memory allocation failed")]
    MemoryAllocationFailed,
    #[error("function evaluation failed")]
    FunctionEvalFailed,
    #[error("jacobian evaluation failed")]
    JacobianEvalFailed,
    #[error("matrix operation failed")]
    MatrixOpFailed,
}

pub struct OdeSystem {
    pub dimension: usize,
    pub params: usize,
    pub func: fn(f64, &[f64], &mut [f64], &mut [f64]) -> Result<(), OdeError>,
    pub jacobian: fn(f64, &[f64], &mut [f64], &mut Array2<f64>, &mut [f64]) -> Result<(), OdeError>,
}

pub struct Rk2SimpState {
    y1: Array1<f64>,
    y0: Array1<f64>,
    y0_orig: Array1<f64>,
    ytmp: Array1<f64>,
    dfdy: Array2<f64>,
    dfdt: Array1<f64>,
    y_onestep: Array1<f64>,
    // No direct equivalent for gsl_permutation in Rust, using LU decomposition directly
}

impl Rk2SimpState {
    pub fn new(dim: usize) -> Result<Self, OdeError> {
        Ok(Self {
            y1: Array1::zeros(dim),
            y0: Array1::zeros(dim),
            y0_orig: Array1::zeros(dim),
            ytmp: Array1::zeros(dim),
            dfdy: Array2::zeros((dim, dim)),
            dfdt: Array1::zeros(dim),
            y_onestep: Array1::zeros(dim),
        })
    }

    pub fn reset(&mut self) {
        self.y1.fill(0.0);
        self.y0.fill(0.0);
        self.y0_orig.fill(0.0);
        self.ytmp.fill(0.0);
        self.dfdy.fill(0.0);
        self.dfdt.fill(0.0);
        self.y_onestep.fill(0.0);
    }
}

pub struct Rk2SimpStepper {
    state: Rk2SimpState,
}

impl Rk2SimpStepper {
    pub fn new(dim: usize) -> Result<Self, OdeError> {
        Ok(Self {
            state: Rk2SimpState::new(dim)?,
        })
    }

    fn step(
        &mut self,
        y: &mut [f64],
        h: f64,
        t: f64,
        system: &OdeSystem,
    ) -> Result<(), OdeError> {
        let dim = system.dimension;
        let y0 = &self.state.y0;
        let y1 = &mut self.state.y1;
        let ytmp = &mut self.state.ytmp;

        // Calculate Jacobian
        (system.jacobian)(t, y0.as_slice().unwrap(), &mut self.state.dfdt, &mut self.state.dfdy, &mut [])?;

        // Scale matrix and add identity
        self.state.dfdy *= -h / 2.0;
        for i in 0..dim {
            self.state.dfdy[(i, i)] += 1.0;
        }

        // LU decomposition
        let lu = self.state.dfdy.lu()?;

        // Evaluate function at midpoint
        (system.func)(t + 0.5 * h, y0.as_slice().unwrap(), ytmp.as_slice_mut().unwrap(), &mut [])?;

        // Solve linear system
        *y1 = lu.solve(&ytmp.view())?;
        *y1 *= 0.5 * h;
        *y1 += &y0.view();

        // Final function evaluation
        (system.func)(t + 0.5 * h, y1.as_slice().unwrap(), ytmp.as_slice_mut().unwrap(), &mut [])?;

        // Update y values
        for i in 0..dim {
            y[i] = y0[i] + h * ytmp[i];
        }

        Ok(())
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        system: &OdeSystem,
    ) -> Result<(), OdeError> {
        let dim = system.dimension;
        self.state.y0.assign(&ArrayView1::from(y));

        // Save original values
        self.state.y0_orig.assign(&self.state.y0.view());

        // One step
        self.state.y_onestep.assign(&self.state.y0.view());
        self.step(self.state.y_onestep.as_slice_mut().unwrap(), h, t, system)?;

        // Two half steps
        self.step(y, h / 2.0, t, system)?;
        self.state.y0.assign(&ArrayView1::from(y));
        self.step(y, h / 2.0, t + h / 2.0, system)?;

        // Derivatives at output
        if let Some(dydt_out) = dydt_out {
            (system.func)(t + h, y, dydt_out, &mut [])?;
        }

        // Error estimation
        for i in 0..dim {
            yerr[i] = 4.0 * (y[i] - self.state.y_onestep[i]) / 3.0;
        }

        Ok(())
    }

    pub fn order(&self) -> u32 {
        2
    }
}

pub struct Rk2SimpType;

impl Rk2SimpType {
    pub fn new() -> Self {
        Self
    }

    pub fn alloc(&self, dim: usize) -> Result<Rk2SimpStepper, OdeError> {
        Rk2SimpStepper::new(dim)
    }
}
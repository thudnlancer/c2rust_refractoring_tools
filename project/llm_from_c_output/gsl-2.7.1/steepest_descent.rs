use std::error::Error;
use std::fmt;
use ndarray::{Array1, ArrayView1};

#[derive(Debug)]
enum MinimizerError {
    NoProgress,
    AllocationFailed,
}

impl fmt::Display for MinimizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinimizerError::NoProgress => write!(f, "no progress made in minimization"),
            MinimizerError::AllocationFailed => write!(f, "memory allocation failed"),
        }
    }
}

impl Error for MinimizerError {}

pub trait MultiminFunctionFdf {
    fn eval_f(&self, x: &ArrayView1<f64>) -> f64;
    fn eval_df(&self, x: &ArrayView1<f64>, gradient: &mut Array1<f64>);
    fn eval_f_df(&self, x: &ArrayView1<f64>, f: &mut f64, gradient: &mut Array1<f64>) {
        *f = self.eval_f(x);
        self.eval_df(x, gradient);
    }
}

pub struct SteepestDescentState {
    step: f64,
    max_step: f64,
    tol: f64,
    x1: Array1<f64>,
    g1: Array1<f64>,
}

impl SteepestDescentState {
    pub fn new(n: usize) -> Result<Self, MinimizerError> {
        Ok(Self {
            step: 0.0,
            max_step: 0.0,
            tol: 0.0,
            x1: Array1::zeros(n),
            g1: Array1::zeros(n),
        })
    }

    pub fn set(
        &mut self,
        fdf: &impl MultiminFunctionFdf,
        x: &ArrayView1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        step_size: f64,
        tol: f64,
    ) {
        fdf.eval_f_df(x, f, gradient);
        self.step = step_size;
        self.max_step = step_size;
        self.tol = tol;
    }

    pub fn restart(&mut self) {
        self.step = self.max_step;
    }

    pub fn iterate(
        &mut self,
        fdf: &impl MultiminFunctionFdf,
        x: &mut Array1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), MinimizerError> {
        let f0 = *f;
        let mut f1;
        let mut step = self.step;
        let tol = self.tol;
        let mut failed = false;

        let gnorm = gradient.dot(gradient).sqrt();
        if gnorm == 0.0 {
            dx.fill(0.0);
            return Err(MinimizerError::NoProgress);
        }

        loop {
            dx.fill(0.0);
            dx.scaled_add(-step / gnorm, gradient);

            self.x1.assign(x);
            self.x1.scaled_add(1.0, dx);

            if self.x1 == *x {
                return Err(MinimizerError::NoProgress);
            }

            fdf.eval_f_df(&self.x1.view(), &mut f1, &mut self.g1);

            if f1 > f0 {
                failed = true;
                step *= tol;
                continue;
            }

            break;
        }

        if failed {
            step *= tol;
        } else {
            step *= 2.0;
        }

        self.step = step;
        x.assign(&self.x1);
        gradient.assign(&self.g1);
        *f = f1;

        Ok(())
    }
}

pub struct SteepestDescentMinimizer {
    state: SteepestDescentState,
}

impl SteepestDescentMinimizer {
    pub fn new(n: usize) -> Result<Self, MinimizerError> {
        Ok(Self {
            state: SteepestDescentState::new(n)?,
        })
    }

    pub fn set(
        &mut self,
        fdf: &impl MultiminFunctionFdf,
        x: &ArrayView1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        step_size: f64,
        tol: f64,
    ) {
        self.state.set(fdf, x, f, gradient, step_size, tol);
    }

    pub fn iterate(
        &mut self,
        fdf: &impl MultiminFunctionFdf,
        x: &mut Array1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), MinimizerError> {
        self.state.iterate(fdf, x, f, gradient, dx)
    }

    pub fn restart(&mut self) {
        self.state.restart();
    }
}
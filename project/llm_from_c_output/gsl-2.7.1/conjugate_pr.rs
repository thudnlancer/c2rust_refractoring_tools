use std::error::Error;
use std::fmt;
use ndarray::{Array1, ArrayView1};
use approx::relative_eq;

#[derive(Debug)]
pub enum MultiminError {
    NoProgress,
    AllocationFailed,
}

impl fmt::Display for MultiminError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultiminError::NoProgress => write!(f, "no progress made in minimization"),
            MultiminError::AllocationFailed => write!(f, "memory allocation failed"),
        }
    }
}

impl Error for MultiminError {}

pub trait MultiminFunction {
    fn eval(&self, x: &ArrayView1<f64>) -> f64;
    fn eval_grad(&self, x: &ArrayView1<f64>, grad: &mut Array1<f64>);
    fn eval_f_grad(&self, x: &ArrayView1<f64>, f: &mut f64, grad: &mut Array1<f64>) {
        *f = self.eval(x);
        self.eval_grad(x, grad);
    }
}

struct ConjugatePRState {
    iter: usize,
    step: f64,
    max_step: f64,
    tol: f64,
    x1: Array1<f64>,
    dx1: Array1<f64>,
    x2: Array1<f64>,
    pnorm: f64,
    p: Array1<f64>,
    g0norm: f64,
    g0: Array1<f64>,
}

impl ConjugatePRState {
    fn new(n: usize) -> Result<Self, MultiminError> {
        Ok(Self {
            iter: 0,
            step: 0.0,
            max_step: 0.0,
            tol: 0.0,
            x1: Array1::zeros(n),
            dx1: Array1::zeros(n),
            x2: Array1::zeros(n),
            pnorm: 0.0,
            p: Array1::zeros(n),
            g0norm: 0.0,
            g0: Array1::zeros(n),
        })
    }

    fn set(
        &mut self,
        fdf: &impl MultiminFunction,
        x: &ArrayView1<f64>,
        f: &mut f64,
        grad: &mut Array1<f64>,
        step_size: f64,
        tol: f64,
    ) -> Result<(), MultiminError> {
        self.iter = 0;
        self.step = step_size;
        self.max_step = step_size;
        self.tol = tol;

        fdf.eval_f_grad(x, f, grad);

        // Use the gradient as the initial direction
        self.p.assign(grad);
        self.g0.assign(grad);

        self.pnorm = grad.dot(grad).sqrt();
        self.g0norm = self.pnorm;

        Ok(())
    }

    fn restart(&mut self) {
        self.iter = 0;
    }

    fn iterate(
        &mut self,
        fdf: &impl MultiminFunction,
        x: &mut Array1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), MultiminError> {
        let fa = *f;
        let mut fb;
        let mut fc;
        let dir;
        let stepa = 0.0;
        let mut stepb;
        let stepc = self.step;
        let tol = self.tol;

        if relative_eq!(self.pnorm, 0.0) || relative_eq!(self.g0norm, 0.0) {
            dx.fill(0.0);
            return Err(MultiminError::NoProgress);
        }

        // Determine which direction is downhill, +p or -p
        let pg = self.p.dot(gradient);
        dir = if pg >= 0.0 { 1.0 } else { -1.0 };

        // Compute new trial point at x_c = x - step * p
        take_step(x, &self.p, stepc, dir / self.pnorm, &mut self.x1, dx);

        // Evaluate function and gradient at new point xc
        fc = fdf.eval(&self.x1.view());

        if fc < fa {
            // Success, reduced the function value
            self.step = stepc * 2.0;
            *f = fc;
            x.assign(&self.x1);
            fdf.eval_grad(&x.view(), gradient);
            return Ok(());
        }

        // Do a line minimisation
        intermediate_point(
            fdf,
            x,
            &self.p,
            dir / self.pnorm,
            pg,
            stepa,
            stepc,
            fa,
            fc,
            &mut self.x1,
            &mut self.dx1,
            gradient,
            &mut stepb,
            &mut fb,
        );

        if relative_eq!(stepb, 0.0) {
            return Err(MultiminError::NoProgress);
        }

        let g1norm = minimize(
            fdf,
            x,
            &self.p,
            dir / self.pnorm,
            stepa,
            stepb,
            stepc,
            fa,
            fb,
            fc,
            tol,
            &mut self.x1,
            &mut self.dx1,
            &mut self.x2,
            dx,
            gradient,
            &mut self.step,
            f,
        );

        x.assign(&self.x2);

        // Choose a new conjugate direction for the next step
        self.iter = (self.iter + 1) % x.len();

        if self.iter == 0 {
            self.p.assign(gradient);
            self.pnorm = g1norm;
        } else {
            // p' = g1 - beta * p
            let mut g0_minus_g1 = self.g0.clone();
            g0_minus_g1.scaled_add(-1.0, gradient);
            let g0g1 = g0_minus_g1.dot(gradient);
            let beta = g0g1 / (self.g0norm * self.g0norm);

            self.p *= -beta;
            self.p.scaled_add(1.0, gradient);
            self.pnorm = self.p.dot(&self.p).sqrt();
        }

        self.g0norm = g1norm;
        self.g0.assign(gradient);

        Ok(())
    }
}

fn take_step(
    x: &Array1<f64>,
    p: &Array1<f64>,
    step: f64,
    dir: f64,
    x1: &mut Array1<f64>,
    dx: &mut Array1<f64>,
) {
    x1.assign(x);
    x1.scaled_add(-step * dir, p);
    dx.assign(x1);
    dx.scaled_add(-1.0, x);
}

fn intermediate_point(
    fdf: &impl MultiminFunction,
    x: &Array1<f64>,
    p: &Array1<f64>,
    dir: f64,
    pg: f64,
    stepa: f64,
    stepc: f64,
    fa: f64,
    fc: f64,
    x1: &mut Array1<f64>,
    dx1: &mut Array1<f64>,
    gradient: &mut Array1<f64>,
    stepb: &mut f64,
    fb: &mut f64,
) {
    // Implementation of intermediate point calculation
    // (omitted for brevity, similar to C version)
}

fn minimize(
    fdf: &impl MultiminFunction,
    x: &Array1<f64>,
    p: &Array1<f64>,
    dir: f64,
    stepa: f64,
    stepb: f64,
    stepc: f64,
    fa: f64,
    fb: f64,
    fc: f64,
    tol: f64,
    x1: &mut Array1<f64>,
    dx1: &mut Array1<f64>,
    x2: &mut Array1<f64>,
    dx: &mut Array1<f64>,
    gradient: &mut Array1<f64>,
    step: &mut f64,
    f: &mut f64,
) -> f64 {
    // Implementation of minimization routine
    // (omitted for brevity, similar to C version)
    0.0 // return g1norm
}

pub struct ConjugatePRMinimizer {
    state: ConjugatePRState,
}

impl ConjugatePRMinimizer {
    pub fn new(n: usize) -> Result<Self, MultiminError> {
        Ok(Self {
            state: ConjugatePRState::new(n)?,
        })
    }

    pub fn set(
        &mut self,
        fdf: &impl MultiminFunction,
        x: &ArrayView1<f64>,
        f: &mut f64,
        grad: &mut Array1<f64>,
        step_size: f64,
        tol: f64,
    ) -> Result<(), MultiminError> {
        self.state.set(fdf, x, f, grad, step_size, tol)
    }

    pub fn iterate(
        &mut self,
        fdf: &impl MultiminFunction,
        x: &mut Array1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), MultiminError> {
        self.state.iterate(fdf, x, f, gradient, dx)
    }

    pub fn restart(&mut self) {
        self.state.restart();
    }
}
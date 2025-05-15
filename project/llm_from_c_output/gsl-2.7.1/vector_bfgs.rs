use std::error::Error;
use std::fmt;
use ndarray::{Array1, ArrayView1, ArrayViewMut1};
use ndarray_linalg::Norm;

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

pub struct VectorBfgsState {
    iter: usize,
    step: f64,
    max_step: f64,
    tol: f64,
    x1: Array1<f64>,
    dx1: Array1<f64>,
    x2: Array1<f64>,
    g0norm: f64,
    pnorm: f64,
    p: Array1<f64>,
    x0: Array1<f64>,
    g0: Array1<f64>,
    dx0: Array1<f64>,
    dg0: Array1<f64>,
}

impl VectorBfgsState {
    pub fn new(n: usize) -> Result<Self, MultiminError> {
        Ok(VectorBfgsState {
            iter: 0,
            step: 0.0,
            max_step: 0.0,
            tol: 0.0,
            x1: Array1::zeros(n),
            dx1: Array1::zeros(n),
            x2: Array1::zeros(n),
            g0norm: 0.0,
            pnorm: 0.0,
            p: Array1::zeros(n),
            x0: Array1::zeros(n),
            g0: Array1::zeros(n),
            dx0: Array1::zeros(n),
            dg0: Array1::zeros(n),
        })
    }

    pub fn set(
        &mut self,
        fdf: &dyn Fn(ArrayView1<f64>) -> (f64, Array1<f64>),
        x: ArrayView1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        step_size: f64,
        tol: f64,
    ) {
        self.iter = 0;
        self.step = step_size;
        self.max_step = step_size;
        self.tol = tol;

        (*f, *gradient) = fdf(x);

        self.x0.assign(&x);
        self.p.assign(gradient);
        self.g0.assign(gradient);

        self.pnorm = gradient.norm_l2();
        self.g0norm = self.pnorm;
    }

    pub fn iterate(
        &mut self,
        fdf: &dyn Fn(ArrayView1<f64>) -> (f64, Array1<f64>),
        x: &mut Array1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), MultiminError> {
        let pnorm = self.pnorm;
        let g0norm = self.g0norm;

        if pnorm == 0.0 || g0norm == 0.0 {
            dx.fill(0.0);
            return Err(MultiminError::NoProgress);
        }

        let mut pg = self.p.dot(gradient);
        let dir = if pg >= 0.0 { 1.0 } else { -1.0 };

        let mut stepa = 0.0;
        let mut stepc = self.step;
        let tol = self.tol;
        let mut fa = *f;
        let mut fb;
        let mut fc;

        // Compute new trial point at x_c = x - step * p
        take_step(x.view(), self.p.view(), stepc, dir / pnorm, &mut self.x1, dx);
        fc = fdf(self.x1.view()).0;

        if fc < fa {
            self.step = stepc * 2.0;
            *f = fc;
            x.assign(&self.x1);
            *gradient = fdf(x.view()).1;
            return Ok(());
        }

        intermediate_point(
            fdf,
            x.view(),
            self.p.view(),
            dir / pnorm,
            pg,
            stepa,
            stepc,
            fa,
            fc,
            &mut self.x1,
            &mut self.dx1,
            gradient,
            &mut fb,
        );

        if fb == 0.0 {
            return Err(MultiminError::NoProgress);
        }

        let g1norm = minimize(
            fdf,
            x.view(),
            self.p.view(),
            dir / pnorm,
            stepa,
            fb,
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

        self.iter = (self.iter + 1) % x.len();

        if self.iter == 0 {
            self.p.assign(gradient);
            self.pnorm = g1norm;
        } else {
            let dxg = self.dx0.dot(gradient);
            let dgg = self.dg0.dot(gradient);
            let dxdg = self.dx0.dot(&self.dg0);
            let dgnorm = self.dg0.norm_l2();

            let (a, b) = if dxdg != 0.0 {
                let b_val = dxg / dxdg;
                let a_val = -(1.0 + dgnorm * dgnorm / dxdg) * b_val + dgg / dxdg;
                (a_val, b_val)
            } else {
                (0.0, 0.0)
            };

            self.p.assign(gradient);
            self.p.scaled_add(-a, &self.dx0);
            self.p.scaled_add(-b, &self.dg0);
            self.pnorm = self.p.norm_l2();
        }

        self.g0.assign(gradient);
        self.x0.assign(x);
        self.g0norm = self.g0.norm_l2();

        Ok(())
    }
}

fn take_step(
    x: ArrayView1<f64>,
    p: ArrayView1<f64>,
    step: f64,
    scale: f64,
    x1: &mut Array1<f64>,
    dx: &mut Array1<f64>,
) {
    x1.assign(&x);
    x1.scaled_add(-step * scale, &p);
    dx.assign(&x);
    dx.scaled_add(-1.0, x1);
}

fn intermediate_point(
    fdf: &dyn Fn(ArrayView1<f64>) -> (f64, Array1<f64>),
    x: ArrayView1<f64>,
    p: ArrayView1<f64>,
    scale: f64,
    pg: f64,
    stepa: f64,
    stepc: f64,
    fa: f64,
    fc: f64,
    x1: &mut Array1<f64>,
    dx1: &mut Array1<f64>,
    gradient: &mut Array1<f64>,
    fb: &mut f64,
) {
    // Implementation of intermediate point calculation
    // ...
}

fn minimize(
    fdf: &dyn Fn(ArrayView1<f64>) -> (f64, Array1<f64>),
    x: ArrayView1<f64>,
    p: ArrayView1<f64>,
    scale: f64,
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
    // Implementation of line minimization
    // ...
    0.0
}
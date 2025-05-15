use std::f64;
use ndarray::{Array1, ArrayView1, ArrayViewMut1};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinimizerError {
    #[error("failed to allocate memory")]
    AllocationError,
    #[error("no progress can be made")]
    NoProgress,
    #[error("minimization error")]
    MinimizationError,
}

pub struct Wrapper {
    fdf_linear: Box<dyn Fn(f64) -> (f64, f64)>,
    x0: Array1<f64>,
    f0: f64,
    g0: Array1<f64>,
    p: Array1<f64>,
    x_alpha: Array1<f64>,
    g_alpha: Array1<f64>,
}

impl Wrapper {
    fn new(
        fdf: &dyn Fn(ArrayView1<f64>) -> (f64, Array1<f64>),
        x0: Array1<f64>,
        f0: f64,
        g0: Array1<f64>,
        p: Array1<f64>,
        x_alpha: Array1<f64>,
        g_alpha: Array1<f64>,
    ) -> Self {
        let fdf_linear = Box::new(move |alpha| {
            x_alpha.assign(&(&x0 + &p * alpha));
            let (f, g) = fdf(x_alpha.view());
            g_alpha.assign(&g);
            (f, g_alpha.dot(&p))
        });
        Wrapper {
            fdf_linear,
            x0,
            f0,
            g0,
            p,
            x_alpha,
            g_alpha,
        }
    }
}

pub struct VectorBFGS2State {
    iter: i32,
    step: f64,
    g0norm: f64,
    pnorm: f64,
    delta_f: f64,
    fp0: f64,
    x0: Array1<f64>,
    g0: Array1<f64>,
    p: Array1<f64>,
    dx0: Array1<f64>,
    dg0: Array1<f64>,
    x_alpha: Array1<f64>,
    g_alpha: Array1<f64>,
    wrap: Wrapper,
    rho: f64,
    sigma: f64,
    tau1: f64,
    tau2: f64,
    tau3: f64,
    order: i32,
}

impl VectorBFGS2State {
    pub fn new(n: usize) -> Result<Self, MinimizerError> {
        Ok(VectorBFGS2State {
            iter: 0,
            step: 0.0,
            g0norm: 0.0,
            pnorm: 0.0,
            delta_f: 0.0,
            fp0: 0.0,
            x0: Array1::zeros(n),
            g0: Array1::zeros(n),
            p: Array1::zeros(n),
            dx0: Array1::zeros(n),
            dg0: Array1::zeros(n),
            x_alpha: Array1::zeros(n),
            g_alpha: Array1::zeros(n),
            wrap: Wrapper::new(
                &|_| (0.0, Array1::zeros(n)),
                Array1::zeros(n),
                0.0,
                Array1::zeros(n),
                Array1::zeros(n),
                Array1::zeros(n),
                Array1::zeros(n),
            ),
            rho: 0.01,
            sigma: 1e-4,
            tau1: 9.0,
            tau2: 0.05,
            tau3: 0.5,
            order: 3,
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
    ) -> Result<(), MinimizerError> {
        self.iter = 0;
        self.step = step_size;
        self.delta_f = 0.0;

        (*f, *gradient) = fdf(x);
        self.x0.assign(&x);
        self.g0.assign(gradient);
        self.g0norm = self.g0.dot(&self.g0).sqrt();

        self.p.assign(gradient);
        self.p *= -1.0 / self.g0norm;
        self.pnorm = self.p.dot(&self.p).sqrt();
        self.fp0 = -self.g0norm;

        self.wrap = Wrapper::new(
            fdf,
            self.x0.clone(),
            *f,
            self.g0.clone(),
            self.p.clone(),
            self.x_alpha.clone(),
            self.g_alpha.clone(),
        );

        self.rho = 0.01;
        self.sigma = tol;
        self.tau1 = 9.0;
        self.tau2 = 0.05;
        self.tau3 = 0.5;
        self.order = 3;

        Ok(())
    }

    pub fn iterate(
        &mut self,
        fdf: &dyn Fn(ArrayView1<f64>) -> (f64, Array1<f64>),
        x: &mut Array1<f64>,
        f: &mut f64,
        gradient: &mut Array1<f64>,
        dx: &mut Array1<f64>,
    ) -> Result<(), MinimizerError> {
        let mut alpha = 0.0;
        let f0 = *f;

        if self.pnorm == 0.0 || self.g0norm == 0.0 || self.fp0 == 0.0 {
            dx.fill(0.0);
            return Err(MinimizerError::NoProgress);
        }

        let alpha1 = if self.delta_f < 0.0 {
            let del = f64::max(-self.delta_f, 10.0 * f64::EPSILON * f0.abs());
            f64::min(1.0, 2.0 * del / (-self.fp0))
        } else {
            self.step.abs()
        };

        let status = minimize(
            &self.wrap.fdf_linear,
            self.rho,
            self.sigma,
            self.tau1,
            self.tau2,
            self.tau3,
            self.order,
            alpha1,
            &mut alpha,
        );

        if status.is_err() {
            return Err(MinimizerError::MinimizationError);
        }

        update_position(&mut self.wrap, alpha, x, f, gradient);
        self.delta_f = *f - f0;

        // BFGS update
        {
            let dx0 = &mut self.dx0;
            let dg0 = &mut self.dg0;

            dx0.assign(x);
            dx0.scaled_add(-1.0, &self.x0);

            dx.assign(dx0);

            dg0.assign(gradient);
            dg0.scaled_add(-1.0, &self.g0);

            let dxg = dx0.dot(gradient);
            let dgg = dg0.dot(gradient);
            let dxdg = dx0.dot(dg0);
            let dgnorm = dg0.dot(dg0).sqrt();

            let (a, b) = if dxdg != 0.0 {
                let b_val = dxg / dxdg;
                let a_val = -(1.0 + dgnorm * dgnorm / dxdg) * b_val + dgg / dxdg;
                (a_val, b_val)
            } else {
                (0.0, 0.0)
            };

            self.p.assign(gradient);
            self.p.scaled_add(-a, dx0);
            self.p.scaled_add(-b, dg0);
        }

        self.g0.assign(gradient);
        self.x0.assign(x);
        self.g0norm = self.g0.dot(&self.g0).sqrt();
        self.pnorm = self.p.dot(&self.p).sqrt();

        let pg = self.p.dot(gradient);
        let dir = if pg >= 0.0 { -1.0 } else { 1.0 };
        self.p *= dir / self.pnorm;
        self.pnorm = self.p.dot(&self.p).sqrt();
        self.fp0 = self.p.dot(&self.g0);

        change_direction(&mut self.wrap);

        Ok(())
    }
}

fn minimize(
    fdf: &dyn Fn(f64) -> (f64, f64),
    rho: f64,
    sigma: f64,
    tau1: f64,
    tau2: f64,
    tau3: f64,
    order: i32,
    alpha1: f64,
    alpha: &mut f64,
) -> Result<(), MinimizerError> {
    // Implementation of the minimization algorithm
    Ok(())
}

fn update_position(
    wrap: &mut Wrapper,
    alpha: f64,
    x: &mut Array1<f64>,
    f: &mut f64,
    gradient: &mut Array1<f64>,
) {
    x.assign(&(&wrap.x0 + &wrap.p * alpha));
    (*f, *gradient) = (wrap.fdf_linear)(alpha);
}

fn change_direction(wrap: &mut Wrapper) {
    // Implementation to change direction
}
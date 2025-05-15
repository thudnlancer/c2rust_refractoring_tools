use std::cmp::max;
use std::f64;
use nalgebra::{DMatrix, DVector, Dynamic, OMatrix, OVector, U1};
use nalgebra::linalg::QR;

#[derive(Debug)]
struct HybridJState {
    iter: usize,
    ncfail: usize,
    ncsuc: usize,
    nslow1: usize,
    nslow2: usize,
    fnorm: f64,
    delta: f64,
    q: OMatrix<f64, Dynamic, Dynamic>,
    r: OMatrix<f64, Dynamic, Dynamic>,
    tau: OVector<f64, Dynamic>,
    diag: OVector<f64, Dynamic>,
    qtf: OVector<f64, Dynamic>,
    newton: OVector<f64, Dynamic>,
    gradient: OVector<f64, Dynamic>,
    x_trial: OVector<f64, Dynamic>,
    f_trial: OVector<f64, Dynamic>,
    df: OVector<f64, Dynamic>,
    qtdf: OVector<f64, Dynamic>,
    rdx: OVector<f64, Dynamic>,
    w: OVector<f64, Dynamic>,
    v: OVector<f64, Dynamic>,
}

impl HybridJState {
    fn new(n: usize) -> Result<Self, &'static str> {
        Ok(Self {
            iter: 0,
            ncfail: 0,
            ncsuc: 0,
            nslow1: 0,
            nslow2: 0,
            fnorm: 0.0,
            delta: 0.0,
            q: DMatrix::zeros(n, n),
            r: DMatrix::zeros(n, n),
            tau: DVector::zeros(n),
            diag: DVector::zeros(n),
            qtf: DVector::zeros(n),
            newton: DVector::zeros(n),
            gradient: DVector::zeros(n),
            x_trial: DVector::zeros(n),
            f_trial: DVector::zeros(n),
            df: DVector::zeros(n),
            qtdf: DVector::zeros(n),
            rdx: DVector::zeros(n),
            w: DVector::zeros(n),
            v: DVector::zeros(n),
        })
    }
}

struct MultiRootFunctionFDF {
    f: Box<dyn Fn(&DVector<f64>) -> DVector<f64>>,
    df: Box<dyn Fn(&DVector<f64>) -> DMatrix<f64>>,
    fdf: Box<dyn Fn(&DVector<f64>) -> (DVector<f64>, DMatrix<f64>)>,
}

fn hybridj_alloc(n: usize) -> Result<HybridJState, &'static str> {
    HybridJState::new(n)
}

fn hybridj_set(
    state: &mut HybridJState,
    fdf: &MultiRootFunctionFDF,
    x: &DVector<f64>,
    f: &mut DVector<f64>,
    j: &mut DMatrix<f64>,
    dx: &mut DVector<f64>,
    scale: bool,
) -> Result<(), &'static str> {
    let (f_val, j_val) = (fdf.fdf)(x);
    *f = f_val;
    *j = j_val;

    state.iter = 1;
    state.fnorm = enorm(&f);
    state.ncfail = 0;
    state.ncsuc = 0;
    state.nslow1 = 0;
    state.nslow2 = 0;

    dx.fill(0.0);

    if scale {
        compute_diag(j, &mut state.diag);
    } else {
        state.diag.fill(1.0);
    }

    state.delta = compute_delta(&state.diag, x);

    let qr = QR::new(j.clone());
    state.q = qr.q();
    state.r = qr.r();
    state.tau = qr.tau().clone();

    Ok(())
}

fn hybridj_iterate(
    state: &mut HybridJState,
    fdf: &MultiRootFunctionFDF,
    x: &mut DVector<f64>,
    f: &mut DVector<f64>,
    j: &mut DMatrix<f64>,
    dx: &mut DVector<f64>,
    scale: bool,
) -> Result<(), &'static str> {
    let fnorm = state.fnorm;
    let p1 = 0.1;
    let p5 = 0.5;
    let p001 = 0.001;
    let p0001 = 0.0001;

    compute_qtf(&state.q, f, &mut state.qtf);
    dogleg(
        &state.r,
        &state.qtf,
        &state.diag,
        state.delta,
        &mut state.newton,
        &mut state.gradient,
        dx,
    );

    compute_trial_step(x, dx, &mut state.x_trial);
    let pnorm = scaled_enorm(&state.diag, dx);

    if state.iter == 1 && pnorm < state.delta {
        state.delta = pnorm;
    }

    let f_trial_val = (fdf.f)(&state.x_trial);
    state.f_trial = f_trial_val;

    compute_df(&state.f_trial, f, &mut state.df);

    let fnorm1 = enorm(&state.f_trial);
    let actred = compute_actual_reduction(fnorm, fnorm1);

    compute_rdx(&state.r, dx, &mut state.rdx);
    let fnorm1p = enorm_sum(&state.qtf, &state.rdx);
    let prered = compute_predicted_reduction(fnorm, fnorm1p);

    let ratio = if prered > 0.0 {
        actred / prered
    } else {
        0.0
    };

    if ratio < p1 {
        state.ncsuc = 0;
        state.ncfail += 1;
        state.delta *= p5;
    } else {
        state.ncfail = 0;
        state.ncsuc += 1;

        if ratio >= p5 || state.ncsuc > 1 {
            state.delta = max(state.delta, pnorm / p5);
        }
        if (ratio - 1.0).abs() <= p1 {
            state.delta = pnorm / p5;
        }
    }

    if ratio >= p0001 {
        x.copy_from(&state.x_trial);
        f.copy_from(&state.f_trial);
        state.fnorm = fnorm1;
        state.iter += 1;
    }

    state.nslow1 += 1;
    if actred >= p001 {
        state.nslow1 = 0;
    }

    if actred >= p1 {
        state.nslow2 = 0;
    }

    if state.ncfail == 2 {
        let j_val = (fdf.df)(x);
        *j = j_val;
        state.nslow2 += 1;

        if state.iter == 1 {
            if scale {
                compute_diag(j, &mut state.diag);
            }
            state.delta = compute_delta(&state.diag, x);
        } else if scale {
            update_diag(j, &mut state.diag);
        }

        let qr = QR::new(j.clone());
        state.q = qr.q();
        state.r = qr.r();
        state.tau = qr.tau().clone();
        return Ok(());
    }

    compute_qtf(&state.q, &state.df, &mut state.qtdf);
    compute_wv(
        &state.qtdf,
        &state.rdx,
        dx,
        &state.diag,
        pnorm,
        &mut state.w,
        &mut state.v,
    );

    gsl_linalg_QR_update(&mut state.q, &mut state.r, &state.w, &state.v);

    if state.nslow2 == 5 {
        return Err("No progress in Jacobian evaluations");
    }

    if state.nslow1 == 10 {
        return Err("No progress in function evaluations");
    }

    Ok(())
}

// Helper functions implementations
fn enorm(v: &DVector<f64>) -> f64 {
    v.norm()
}

fn scaled_enorm(diag: &DVector<f64>, v: &DVector<f64>) -> f64 {
    (diag.component_mul(v)).norm()
}

fn compute_diag(j: &DMatrix<f64>, diag: &mut DVector<f64>) {
    for (i, col) in j.column_iter().enumerate() {
        diag[i] = col.norm();
    }
}

fn compute_delta(diag: &DVector<f64>, x: &DVector<f64>) -> f64 {
    let dx_norm = scaled_enorm(diag, x);
    if dx_norm == 0.0 {
        1.0
    } else {
        dx_norm
    }
}

fn compute_qtf(q: &DMatrix<f64>, f: &DVector<f64>, qtf: &mut DVector<f64>) {
    *qtf = q.transpose() * f;
}

fn dogleg(
    r: &DMatrix<f64>,
    qtf: &DVector<f64>,
    diag: &DVector<f64>,
    delta: f64,
    newton: &mut DVector<f64>,
    gradient: &mut DVector<f64>,
    dx: &mut DVector<f64>,
) {
    // Implementation of dogleg algorithm
    unimplemented!()
}

fn compute_trial_step(x: &DVector<f64>, dx: &DVector<f64>, x_trial: &mut DVector<f64>) {
    *x_trial = x + dx;
}

fn compute_df(f_trial: &DVector<f64>, f: &DVector<f64>, df: &mut DVector<f64>) {
    *df = f_trial - f;
}

fn compute_actual_reduction(fnorm: f64, fnorm1: f64) -> f64 {
    if fnorm1 < fnorm {
        1.0 - (fnorm1 / fnorm).powi(2)
    } else {
        0.0
    }
}

fn compute_rdx(r: &DMatrix<f64>, dx: &DVector<f64>, rdx: &mut DVector<f64>) {
    *rdx = r * dx;
}

fn enorm_sum(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
    (a + b).norm()
}

fn compute_predicted_reduction(fnorm: f64, fnorm1p: f64) -> f64 {
    if fnorm1p < fnorm {
        1.0 - (fnorm1p / fnorm).powi(2)
    } else {
        0.0
    }
}

fn compute_wv(
    qtdf: &DVector<f64>,
    rdx: &DVector<f64>,
    dx: &DVector<f64>,
    diag: &DVector<f64>,
    pnorm: f64,
    w: &mut DVector<f64>,
    v: &mut DVector<f64>,
) {
    *w = (qtdf - rdx) / pnorm;
    *v = diag.component_mul(&diag).component_mul(dx) / pnorm;
}

fn update_diag(j: &DMatrix<f64>, diag: &mut DVector<f64>) {
    for (i, col) in j.column_iter().enumerate() {
        diag[i] = diag[i].max(col.norm());
    }
}

fn gsl_linalg_QR_update(
    q: &mut DMatrix<f64>,
    r: &mut DMatrix<f64>,
    w: &DVector<f64>,
    v: &DVector<f64>,
) {
    // Implementation of QR update
    unimplemented!()
}

pub struct HybridJ {
    state: HybridJState,
}

impl HybridJ {
    pub fn new(n: usize) -> Result<Self, &'static str> {
        Ok(Self {
            state: hybridj_alloc(n)?,
        })
    }

    pub fn set(
        &mut self,
        fdf: &MultiRootFunctionFDF,
        x: &DVector<f64>,
        f: &mut DVector<f64>,
        j: &mut DMatrix<f64>,
        dx: &mut DVector<f64>,
    ) -> Result<(), &'static str> {
        hybridj_set(&mut self.state, fdf, x, f, j, dx, false)
    }

    pub fn iterate(
        &mut self,
        fdf: &MultiRootFunctionFDF,
        x: &mut DVector<f64>,
        f: &mut DVector<f64>,
        j: &mut DMatrix<f64>,
        dx: &mut DVector<f64>,
    ) -> Result<(), &'static str> {
        hybridj_iterate(&mut self.state, fdf, x, f, j, dx, false)
    }
}

pub struct HybridSJ {
    state: HybridJState,
}

impl HybridSJ {
    pub fn new(n: usize) -> Result<Self, &'static str> {
        Ok(Self {
            state: hybridj_alloc(n)?,
        })
    }

    pub fn set(
        &mut self,
        fdf: &MultiRootFunctionFDF,
        x: &DVector<f64>,
        f: &mut DVector<f64>,
        j: &mut DMatrix<f64>,
        dx: &mut DVector<f64>,
    ) -> Result<(), &'static str> {
        hybridj_set(&mut self.state, fdf, x, f, j, dx, true)
    }

    pub fn iterate(
        &mut self,
        fdf: &MultiRootFunctionFDF,
        x: &mut DVector<f64>,
        f: &mut DVector<f64>,
        j: &mut DMatrix<f64>,
        dx: &mut DVector<f64>,
    ) -> Result<(), &'static str> {
        hybridj_iterate(&mut self.state, fdf, x, f, j, dx, true)
    }
}
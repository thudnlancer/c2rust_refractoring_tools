use std::f64;
use std::mem;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use ndarray_linalg::{Determinant, LU, Solve};
use thiserror::Error;

const SEQUENCE_COUNT: usize = 8;
const SEQUENCE_MAX: usize = 7;

static BD_SEQUENCE: [usize; SEQUENCE_COUNT] = [2, 6, 10, 14, 22, 34, 50, 70];

#[derive(Error, Debug)]
pub enum OdeError {
    #[error("underflow")]
    Underflow,
    #[error("iteration failed")]
    IterationFailed,
    #[error("evaluation failed")]
    EvaluationFailed,
    #[error("unknown error")]
    Unknown,
}

pub struct BSimpState {
    d: Array2<f64>,
    a_mat: Array2<f64>,
    x: [f64; SEQUENCE_MAX],
    k_current: usize,
    k_choice: usize,
    h_next: f64,
    eps: f64,
    yp: Array1<f64>,
    y_save: Array1<f64>,
    yerr_save: Array1<f64>,
    y_extrap_save: Array1<f64>,
    y_extrap_sequence: Array1<f64>,
    extrap_work: Array1<f64>,
    dfdt: Array1<f64>,
    y_temp: Array1<f64>,
    delta_temp: Array1<f64>,
    weight: Array1<f64>,
    dfdy: Array2<f64>,
    rhs_temp: Array1<f64>,
    delta: Array1<f64>,
    order: usize,
}

pub trait OdeSystem {
    fn eval(&self, t: f64, y: ArrayView1<f64>, dydt: &mut Array1<f64>) -> Result<(), OdeError>;
    fn eval_jacobian(
        &self,
        t: f64,
        y: ArrayView1<f64>,
        dfdy: &mut Array2<f64>,
        dfdt: &mut Array1<f64>,
    ) -> Result<(), OdeError>;
}

fn compute_weights(y: ArrayView1<f64>, w: &mut Array1<f64>) {
    for i in 0..y.len() {
        let u = y[i].abs();
        w[i] = if u > 0.0 { u } else { 1.0 };
    }
}

fn bsimp_deuf_kchoice(eps: f64, dimension: usize) -> usize {
    let safety_f = 0.25;
    let small_eps = safety_f * eps;

    let mut a_work = [0.0; SEQUENCE_COUNT];
    let mut alpha = [[0.0; SEQUENCE_MAX]; SEQUENCE_MAX];

    a_work[0] = BD_SEQUENCE[0] as f64 + 1.0;

    for k in 0..SEQUENCE_MAX {
        a_work[k + 1] = a_work[k] + BD_SEQUENCE[k + 1] as f64;
    }

    for i in 0..SEQUENCE_MAX {
        alpha[i][i] = 1.0;
        for k in 0..i {
            let tmp1 = a_work[k + 1] - a_work[i + 1];
            let tmp2 = (a_work[i + 1] - a_work[0] + 1.0) * (2 * k + 1) as f64;
            alpha[k][i] = small_eps.powf(tmp1 / tmp2);
        }
    }

    a_work[0] += dimension as f64;

    for k in 0..SEQUENCE_MAX {
        a_work[k + 1] = a_work[k] + BD_SEQUENCE[k + 1] as f64;
    }

    let mut k_choice = 0;
    for k in 0..(SEQUENCE_MAX - 1) {
        if a_work[k + 2] > a_work[k + 1] * alpha[k][k + 1] {
            break;
        }
        k_choice = k;
    }

    k_choice
}

fn poly_extrap(
    d: &mut Array2<f64>,
    x: &[f64; SEQUENCE_MAX],
    i_step: usize,
    x_i: f64,
    y_i: ArrayView1<f64>,
    y_0: &mut Array1<f64>,
    y_0_err: &mut Array1<f64>,
    work: &mut Array1<f64>,
) {
    y_0_err.assign(&y_i);
    y_0.assign(&y_i);

    if i_step == 0 {
        for j in 0..y_i.len() {
            d[[0, j]] = y_i[j];
        }
    } else {
        work.assign(&y_i);

        for k in 0..i_step {
            let delta = 1.0 / (x[i_step - k - 1] - x_i);
            let f1 = delta * x_i;
            let f2 = delta * x[i_step - k - 1];

            for j in 0..y_i.len() {
                let q_kj = d[[k, j]];
                d[[k, j]] = y_0_err[j];
                let delta_val = work[j] - q_kj;
                y_0_err[j] = f1 * delta_val;
                work[j] = f2 * delta_val;
                y_0[j] += y_0_err[j];
            }
        }

        for j in 0..y_i.len() {
            d[[i_step, j]] = y_0_err[j];
        }
    }
}

fn bsimp_step_local(
    state: &mut BSimpState,
    dim: usize,
    t0: f64,
    h_total: f64,
    n_step: usize,
    y: ArrayView1<f64>,
    yp: ArrayView1<f64>,
    dfdt: ArrayView1<f64>,
    dfdy: ArrayView2<f64>,
    y_out: &mut Array1<f64>,
    sys: &impl OdeSystem,
) -> Result<(), OdeError> {
    let h = h_total / n_step as f64;
    let t = t0 + h;

    let max_sum = 100.0 * dim as f64;

    // Calculate the matrix for the linear system
    for i in 0..dim {
        for j in 0..dim {
            state.a_mat[[i, j]] = -h * dfdy[[i, j]];
        }
        state.a_mat[[i, i]] += 1.0;
    }

    // LU decomposition
    let lu = state.a_mat.lu().map_err(|_| OdeError::Unknown)?;

    // Compute weighting factors
    compute_weights(y, &mut state.weight);

    // Initial step
    for i in 0..dim {
        state.y_temp[i] = h * (yp[i] + h * dfdt[i]);
    }

    lu.solve_inplace(&mut state.delta_temp)
        .map_err(|_| OdeError::Unknown)?;

    let mut sum = 0.0;
    for i in 0..dim {
        let di = state.delta_temp[i];
        state.delta[i] = di;
        state.y_temp[i] = y[i] + di;
        sum += di.abs() / state.weight[i];
    }

    if sum > max_sum {
        return Err(OdeError::IterationFailed);
    }

    // Intermediate steps
    sys.eval(t, state.y_temp.view(), y_out)?;

    for n_inter in 1..n_step {
        for i in 0..dim {
            state.rhs_temp[i] = h * y_out[i] - state.delta[i];
        }

        lu.solve_inplace(&mut state.delta_temp)
            .map_err(|_| OdeError::Unknown)?;

        sum = 0.0;
        for i in 0..dim {
            state.delta[i] += 2.0 * state.delta_temp[i];
            state.y_temp[i] += state.delta[i];
            sum += state.delta[i].abs() / state.weight[i];
        }

        if sum > max_sum {
            return Err(OdeError::IterationFailed);
        }

        sys.eval(t + h * n_inter as f64, state.y_temp.view(), y_out)?;
    }

    // Final step
    for i in 0..dim {
        state.rhs_temp[i] = h * y_out[i] - state.delta[i];
    }

    lu.solve_inplace(&mut state.delta_temp)
        .map_err(|_| OdeError::Unknown)?;

    sum = 0.0;
    for i in 0..dim {
        y_out[i] = state.y_temp[i] + state.delta_temp[i];
        sum += state.delta_temp[i].abs() / state.weight[i];
    }

    if sum > max_sum {
        Err(OdeError::IterationFailed)
    } else {
        Ok(())
    }
}

impl BSimpState {
    pub fn new(dim: usize) -> Self {
        let k_choice = bsimp_deuf_kchoice(f64::EPSILON.sqrt(), dim);

        BSimpState {
            d: Array2::zeros((SEQUENCE_MAX, dim)),
            a_mat: Array2::zeros((dim, dim)),
            x: [0.0; SEQUENCE_MAX],
            k_current: k_choice,
            k_choice,
            h_next: -f64::MAX.sqrt(),
            eps: f64::EPSILON.sqrt(),
            yp: Array1::zeros(dim),
            y_save: Array1::zeros(dim),
            yerr_save: Array1::zeros(dim),
            y_extrap_save: Array1::zeros(dim),
            y_extrap_sequence: Array1::zeros(dim),
            extrap_work: Array1::zeros(dim),
            dfdt: Array1::zeros(dim),
            y_temp: Array1::zeros(dim),
            delta_temp: Array1::zeros(dim),
            weight: Array1::zeros(dim),
            dfdy: Array2::zeros((dim, dim)),
            rhs_temp: Array1::zeros(dim),
            delta: Array1::zeros(dim),
            order: 2 * k_choice,
        }
    }

    pub fn apply(
        &mut self,
        dim: usize,
        t: f64,
        h: f64,
        y: &mut Array1<f64>,
        yerr: &mut Array1<f64>,
        dydt_in: Option<ArrayView1<f64>>,
        dydt_out: Option<&mut Array1<f64>>,
        sys: &impl OdeSystem,
    ) -> Result<(), OdeError> {
        if h + t == t {
            return Err(OdeError::Underflow);
        }

        self.y_extrap_save.assign(y);

        // Save inputs
        self.y_save.assign(y);
        self.yerr_save.assign(yerr);

        // Evaluate derivative
        if let Some(dydt_in) = dydt_in {
            self.yp.assign(&dydt_in);
        } else {
            sys.eval(t, y.view(), &mut self.yp)?;
        }

        // Evaluate Jacobian
        sys.eval_jacobian(t, y.view(), &mut self.dfdy, &mut self.dfdt)?;

        // Refined extrapolations
        for k in 0..=self.k_current {
            let n = BD_SEQUENCE[k];
            let r = h / n as f64;
            self.x[k] = r * r;

            match bsimp_step_local(
                self,
                dim,
                t,
                h,
                n,
                self.y_extrap_save.view(),
                self.yp.view(),
                self.dfdt.view(),
                self.dfdy.view(),
                &mut self.y_extrap_sequence,
                sys,
            ) {
                Err(OdeError::IterationFailed) => {
                    for i in 0..dim {
                        yerr[i] = f64::INFINITY;
                    }
                    break;
                }
                Err(e) => return Err(e),
                Ok(_) => (),
            }

            poly_extrap(
                &mut self.d,
                &self.x,
                k,
                self.x[k],
                self.y_extrap_sequence.view(),
                y,
                yerr,
                &mut self.extrap_work,
            );
        }

        // Evaluate dydt_out
        if let Some(dydt_out) = dydt_out {
            sys.eval(t + h, y.view(), dydt_out)?;
        }

        Ok(())
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn reset(&mut self, dim: usize) {
        self.h_next = 0.0;
        self.yp.fill(0.0);
    }
}
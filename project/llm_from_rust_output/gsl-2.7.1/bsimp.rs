use gsl::{Matrix, Permutation, Vector, VectorView};
use std::f64;

const BD_SEQUENCE: [usize; 8] = [2, 6, 10, 14, 22, 34, 50, 70];

#[derive(Debug)]
struct BSimpState {
    d: Matrix,
    a_mat: Matrix,
    p_vec: Permutation,
    x: [f64; 7],
    k_current: usize,
    k_choice: usize,
    h_next: f64,
    eps: f64,
    yp: Vector,
    y_save: Vector,
    yerr_save: Vector,
    y_extrap_save: Vector,
    y_extrap_sequence: Vector,
    extrap_work: Vector,
    dfdt: Vector,
    y_temp: Vector,
    delta_temp: Vector,
    weight: Vector,
    dfdy: Matrix,
    rhs_temp: Vector,
    delta: Vector,
    order: usize,
}

fn compute_weights(y: &Vector, w: &mut Vector) {
    for i in 0..y.len() {
        let u = y[i].abs();
        w[i] = if u > 0.0 { u } else { 1.0 };
    }
}

fn bsimp_deuf_kchoice(eps: f64, dimension: usize) -> usize {
    let safety_f = 0.25;
    let small_eps = safety_f * eps;
    let mut a_work = [0.0; 8];
    let mut alpha = [[0.0; 7]; 7];

    a_work[0] = BD_SEQUENCE[0] as f64 + 1.0;
    for k in 0..7 {
        a_work[k + 1] = a_work[k] + BD_SEQUENCE[k + 1] as f64;
    }

    for i in 0..7 {
        alpha[i][i] = 1.0;
        for k in 0..i {
            let tmp1 = a_work[k + 1] - a_work[i + 1];
            let tmp2 = (a_work[i + 1] - a_work[0] + 1.0) * (2 * k + 1) as f64;
            alpha[k][i] = small_eps.powf(tmp1 / tmp2);
        }
    }

    a_work[0] += dimension as f64;
    for k in 0..7 {
        a_work[k + 1] = a_work[k] + BD_SEQUENCE[k + 1] as f64;
    }

    let mut k = 0;
    while k < 6 {
        if a_work[k + 2] > a_work[k + 1] * alpha[k][k + 1] {
            break;
        }
        k += 1;
    }

    k
}

fn poly_extrap(
    d: &mut Matrix,
    x: &[f64],
    i_step: usize,
    x_i: f64,
    y_i: &Vector,
    y_0: &mut Vector,
    y_0_err: &mut Vector,
    work: &mut Vector,
) {
    y_0_err.copy_from(y_i);
    y_0.copy_from(y_i);

    if i_step == 0 {
        for j in 0..y_i.len() {
            d.set(0, j, y_i[j]);
        }
    } else {
        work.copy_from(y_i);
        for k in 0..i_step {
            let delta = 1.0 / (x[i_step - k - 1] - x_i);
            let f1 = delta * x_i;
            let f2 = delta * x[i_step - k - 1];

            for j in 0..y_i.len() {
                let q_kj = d.get(k, j);
                d.set(k, j, y_0_err[j]);
                let delta_val = work[j] - q_kj;
                y_0_err[j] = f1 * delta_val;
                work[j] = f2 * delta_val;
                y_0[j] += y_0_err[j];
            }
        }

        for j in 0..y_i.len() {
            d.set(i_step, j, y_0_err[j]);
        }
    }
}

fn bsimp_step_local(
    state: &mut BSimpState,
    t0: f64,
    h_total: f64,
    n_step: usize,
    y: &Vector,
    yp: &Vector,
    dfdt: &Vector,
    dfdy: &Matrix,
    y_out: &mut Vector,
    sys: &gsl::OdeivSystem,
) -> Result<(), gsl::Value> {
    let h = h_total / n_step as f64;
    let mut t = t0 + h;
    let max_sum = 100.0 * y.len() as f64;

    // Initialize a_mat
    for i in 0..y.len() {
        for j in 0..y.len() {
            let val = -h * dfdy.get(i, j);
            state.a_mat.set(i, j, val);
        }
        state.a_mat.set(i, i, state.a_mat.get(i, i) + 1.0);
    }

    state.a_mat.lu_decomp(&mut state.p_vec)?;

    compute_weights(y, &mut state.weight);

    // Initialize y_temp
    for i in 0..y.len() {
        state.y_temp[i] = h * (yp[i] + h * dfdt[i]);
    }

    state.a_mat.lu_solve(&state.p_vec, &state.y_temp, &mut state.delta_temp)?;

    let mut sum = 0.0;
    for i in 0..y.len() {
        let di = state.delta_temp[i];
        state.delta[i] = di;
        state.y_temp[i] = y[i] + di;
        sum += di.abs() / state.weight[i];
    }

    if sum > max_sum {
        return Err(gsl::Value::Failed);
    }

    sys.function(t, &state.y_temp, y_out)?;

    let mut n_inter = 1;
    while n_inter < n_step {
        for i in 0..y.len() {
            state.rhs_temp[i] = h * y_out[i] - state.delta[i];
        }

        state.a_mat.lu_solve(&state.p_vec, &state.rhs_temp, &mut state.delta_temp)?;

        sum = 0.0;
        for i in 0..y.len() {
            state.delta[i] += 2.0 * state.delta_temp[i];
            state.y_temp[i] += state.delta[i];
            sum += state.delta[i].abs() / state.weight[i];
        }

        if sum > max_sum {
            return Err(gsl::Value::Failed);
        }

        t += h;
        sys.function(t, &state.y_temp, y_out)?;
        n_inter += 1;
    }

    for i in 0..y.len() {
        state.rhs_temp[i] = h * y_out[i] - state.delta[i];
    }

    state.a_mat.lu_solve(&state.p_vec, &state.rhs_temp, &mut state.delta_temp)?;

    sum = 0.0;
    for i in 0..y.len() {
        y_out[i] = state.y_temp[i] + state.delta_temp[i];
        sum += state.delta_temp[i].abs() / state.weight[i];
    }

    if sum > max_sum {
        Err(gsl::Value::Failed)
    } else {
        Ok(())
    }
}

fn bsimp_alloc(dim: usize) -> BSimpState {
    let k_choice = bsimp_deuf_kchoice(1.4901161193847656e-08, dim);
    
    BSimpState {
        d: Matrix::new(7, dim),
        a_mat: Matrix::new(dim, dim),
        p_vec: Permutation::new(dim),
        x: [0.0; 7],
        k_current: k_choice,
        k_choice,
        h_next: -1.3407807929942596e+154,
        eps: 1.4901161193847656e-08,
        yp: Vector::new(dim),
        y_save: Vector::new(dim),
        yerr_save: Vector::new(dim),
        y_extrap_save: Vector::new(dim),
        y_extrap_sequence: Vector::new(dim),
        extrap_work: Vector::new(dim),
        dfdt: Vector::new(dim),
        y_temp: Vector::new(dim),
        delta_temp: Vector::new(dim),
        weight: Vector::new(dim),
        dfdy: Matrix::new(dim, dim),
        rhs_temp: Vector::new(dim),
        delta: Vector::new(dim),
        order: 2 * k_choice,
    }
}

fn bsimp_apply(
    state: &mut BSimpState,
    t: f64,
    h: f64,
    y: &mut Vector,
    yerr: &mut Vector,
    dydt_in: Option<&Vector>,
    dydt_out: Option<&mut Vector>,
    sys: &gsl::OdeivSystem,
) -> Result<(), gsl::Value> {
    if h + t == t {
        return Err(gsl::Value::Underflow);
    }

    state.y_extrap_save.copy_from(y);
    state.y_save.copy_from(y);
    state.yerr_save.copy_from(yerr);

    if let Some(dydt_in) = dydt_in {
        state.yp.copy_from(dydt_in);
    } else {
        sys.function(t, y, &mut state.yp)?;
    }

    sys.jacobian(t, y, &mut state.dfdy, &mut state.dfdt)?;

    for k in 0..=state.k_current {
        let n = BD_SEQUENCE[k];
        let r = h / n as f64;
        let x_k = r * r;

        match bsimp_step_local(
            state,
            t,
            h,
            n,
            &state.y_extrap_save,
            &state.yp,
            &state.dfdt,
            &state.dfdy,
            &mut state.y_extrap_sequence,
            sys,
        ) {
            Ok(_) => {
                state.x[k] = x_k;
                poly_extrap(
                    &mut state.d,
                    &state.x,
                    k,
                    x_k,
                    &state.y_extrap_sequence,
                    y,
                    yerr,
                    &mut state.extrap_work,
                );
            }
            Err(e) => {
                yerr.fill(f64::INFINITY);
                return Err(e);
            }
        }
    }

    if let Some(dydt_out) = dydt_out {
        if let Err(e) = sys.function(t + h, y, dydt_out) {
            y.copy_from(&state.y_save);
            yerr.copy_from(&state.yerr_save);
            return Err(e);
        }
    }

    Ok(())
}

fn bsimp_order(state: &BSimpState) -> usize {
    state.order
}

fn bsimp_reset(state: &mut BSimpState) {
    state.h_next = 0.0;
    state.yp.fill(0.0);
}

pub struct BSimpStep {
    state: BSimpState,
}

impl BSimpStep {
    pub fn new(dim: usize) -> Self {
        Self {
            state: bsimp_alloc(dim),
        }
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut Vector,
        yerr: &mut Vector,
        dydt_in: Option<&Vector>,
        dydt_out: Option<&mut Vector>,
        sys: &gsl::OdeivSystem,
    ) -> Result<(), gsl::Value> {
        bsimp_apply(&mut self.state, t, h, y, yerr, dydt_in, dydt_out, sys)
    }

    pub fn order(&self) -> usize {
        bsimp_order(&self.state)
    }

    pub fn reset(&mut self) -> Result<(), gsl::Value> {
        bsimp_reset(&mut self.state);
        Ok(())
    }
}

impl Drop for BSimpStep {
    fn drop(&mut self) {
        // All resources are automatically freed when struct goes out of scope
    }
}

pub static GSL_ODEIV_STEP_BSIMP: gsl::OdeivStepType = gsl::OdeivStepType {
    name: "bsimp",
    can_use_dydt_in: true,
    gives_exact_dydt_out: true,
    alloc_fn: |dim| Box::new(BSimpStep::new(dim)) as Box<dyn gsl::OdeivStep>,
};
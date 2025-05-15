use rand::prelude::*;
use rand::distributions::Uniform;
use std::f64::consts::E;
use std::fmt;

// Parameters for simulated annealing
const N_TRIES: usize = 200;
const ITERS_FIXED_T: usize = 1000;
const STEP_SIZE: f64 = 1.0;
const K: f64 = 1.0;
const T_INITIAL: f64 = 0.008;
const MU_T: f64 = 1.003;
const T_MIN: f64 = 2.0e-6;

struct SimanParams {
    n_tries: usize,
    iters_fixed_t: usize,
    step_size: f64,
    k: f64,
    t_initial: f64,
    mu_t: f64,
    t_min: f64,
}

impl Default for SimanParams {
    fn default() -> Self {
        SimanParams {
            n_tries: N_TRIES,
            iters_fixed_t: ITERS_FIXED_T,
            step_size: STEP_SIZE,
            k: K,
            t_initial: T_INITIAL,
            mu_t: MU_T,
            t_min: T_MIN,
        }
    }
}

// Energy function E1
fn e1(x: &f64) -> f64 {
    (-(x - 1.0).powi(2)).exp() * (8.0 * x).sin()
}

// Distance metric M1
fn m1(x: &f64, y: &f64) -> f64 {
    (x - y).abs()
}

// Step function S1
fn s1(rng: &mut impl Rng, x: &mut f64, step_size: f64) {
    let u = rng.sample(Uniform::new(0.0, 1.0));
    *x = u * 2.0 * step_size - step_size + *x;
}

// Print function P1
fn p1(x: &f64) {
    println!("{:12}", x);
}

fn siman_solve(
    rng: &mut impl Rng,
    x_initial: &mut f64,
    energy_fn: fn(&f64) -> f64,
    step_fn: fn(&mut impl Rng, &mut f64, f64),
    metric_fn: fn(&f64, &f64) -> f64,
    print_fn: fn(&f64),
    params: SimanParams,
) {
    let mut x_current = *x_initial;
    let mut e_current = energy_fn(&x_current);
    let mut t = params.t_initial;

    while t > params.t_min {
        for _ in 0..params.iters_fixed_t {
            for _ in 0..params.n_tries {
                let mut x_proposed = x_current;
                step_fn(rng, &mut x_proposed, params.step_size);
                let e_proposed = energy_fn(&x_proposed);
                let delta_e = e_proposed - e_current;

                if delta_e < 0.0 || rng.gen::<f64>() < (-delta_e / (params.k * t)).exp() {
                    x_current = x_proposed;
                    e_current = e_proposed;
                }
            }
        }
        t *= params.mu_t;
        print_fn(&x_current);
    }
}

fn main() {
    let params = SimanParams::default();
    let mut rng = rand::thread_rng();
    let mut x_initial = 15.5;

    siman_solve(
        &mut rng,
        &mut x_initial,
        e1,
        s1,
        m1,
        p1,
        params,
    );
}
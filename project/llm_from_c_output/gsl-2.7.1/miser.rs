use std::f64;
use std::mem;
use std::ptr;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiserError {
    #[error("invalid input parameters")]
    InvalidInput,
    #[error("insufficient calls for subvolume")]
    InsufficientCalls,
    #[error("insufficient calls to sample all halfspaces")]
    InsufficientHalfspaceSamples,
    #[error("no points in left-half space")]
    NoPointsLeft,
    #[error("no points in right-half space")]
    NoPointsRight,
    #[error("memory allocation failed")]
    AllocationFailed,
}

#[derive(Debug, Clone)]
pub struct MiserParams {
    pub estimate_frac: f64,
    pub min_calls: usize,
    pub min_calls_per_bisection: usize,
    pub alpha: f64,
    pub dither: f64,
}

#[derive(Debug)]
pub struct MiserState {
    dim: usize,
    x: Vec<f64>,
    xmid: Vec<f64>,
    sigma_l: Vec<f64>,
    sigma_r: Vec<f64>,
    fmax_l: Vec<f64>,
    fmax_r: Vec<f64>,
    fmin_l: Vec<f64>,
    fmin_r: Vec<f64>,
    fsum_l: Vec<f64>,
    fsum_r: Vec<f64>,
    fsum2_l: Vec<f64>,
    fsum2_r: Vec<f64>,
    hits_l: Vec<usize>,
    hits_r: Vec<usize>,
    params: MiserParams,
}

pub trait MonteFunction {
    fn eval(&self, x: &[f64]) -> f64;
}

impl MiserState {
    pub fn new(dim: usize) -> Result<Self, MiserError> {
        Ok(MiserState {
            dim,
            x: vec![0.0; dim],
            xmid: vec![0.0; dim],
            sigma_l: vec![0.0; dim],
            sigma_r: vec![0.0; dim],
            fmax_l: vec![0.0; dim],
            fmax_r: vec![0.0; dim],
            fmin_l: vec![0.0; dim],
            fmin_r: vec![0.0; dim],
            fsum_l: vec![0.0; dim],
            fsum_r: vec![0.0; dim],
            fsum2_l: vec![0.0; dim],
            fsum2_r: vec![0.0; dim],
            hits_l: vec![0; dim],
            hits_r: vec![0; dim],
            params: MiserParams {
                estimate_frac: 0.1,
                min_calls: 16 * dim,
                min_calls_per_bisection: 32 * 16 * dim,
                alpha: 2.0,
                dither: 0.0,
            },
        })
    }

    pub fn init(&mut self) {
        self.params.min_calls = 16 * self.dim;
        self.params.min_calls_per_bisection = 32 * self.params.min_calls;
        self.params.estimate_frac = 0.1;
        self.params.alpha = 2.0;
        self.params.dither = 0.0;
    }

    pub fn params_get(&self) -> MiserParams {
        self.params.clone()
    }

    pub fn params_set(&mut self, params: &MiserParams) {
        self.params = params.clone();
    }
}

pub fn miser_integrate<F: MonteFunction, R: Rng>(
    f: &F,
    xl: &[f64],
    xu: &[f64],
    calls: usize,
    rng: &mut R,
    state: &mut MiserState,
) -> Result<(f64, f64), MiserError> {
    let dim = state.dim;
    if dim != xl.len() || dim != xu.len() {
        return Err(MiserError::InvalidInput);
    }

    for i in 0..dim {
        if xu[i] <= xl[i] {
            return Err(MiserError::InvalidInput);
        }
        if xu[i] - xl[i] > f64::MAX {
            return Err(MiserError::InvalidInput);
        }
    }

    if state.params.alpha < 0.0 {
        return Err(MiserError::InvalidInput);
    }

    let vol = xl.iter().zip(xu.iter()).fold(1.0, |acc, (&l, &u)| acc * (u - l));

    if calls < state.params.min_calls_per_bisection {
        if calls < 2 {
            return Err(MiserError::InsufficientCalls);
        }

        let mut m = 0.0;
        let mut q = 0.0;
        let mut x = vec![0.0; dim];
        let between = Uniform::new(0.0, 1.0);

        for n in 0..calls {
            for i in 0..dim {
                x[i] = xl[i] + between.sample(rng) * (xu[i] - xl[i]);
            }

            let fval = f.eval(&x);
            let d = fval - m;
            m += d / (n as f64 + 1.0);
            q += d * d * (n as f64 / (n as f64 + 1.0));
        }

        let result = vol * m;
        let abserr = vol * (q / (calls as f64 * (calls as f64 - 1.0))).sqrt();
        return Ok((result, abserr));
    }

    let estimate_calls = std::cmp::max(
        state.params.min_calls,
        (calls as f64 * state.params.estimate_frac) as usize,
    );

    if estimate_calls < 4 * dim {
        return Err(MiserError::InsufficientHalfspaceSamples);
    }

    let mut xmid = vec![0.0; dim];
    for i in 0..dim {
        let s = if rng.gen::<f64>() - 0.5 >= 0.0 {
            state.params.dither
        } else {
            -state.params.dither
        };
        xmid[i] = (0.5 + s) * xl[i] + (0.5 - s) * xu[i];
    }

    let (res_est, err_est, sigma_l, sigma_r) = estimate_corrmc(
        f, xl, xu, dim, estimate_calls, rng, state, &xmid,
    )?;

    let remaining_calls = calls - estimate_calls;

    let mut best_var = f64::MAX;
    let mut found_best = false;
    let mut i_bisect = 0;
    let beta = 2.0 / (1.0 + state.params.alpha);
    let mut weight_l = 1.0;
    let mut weight_r = 1.0;

    for i in 0..dim {
        if sigma_l[i] >= 0.0 && sigma_r[i] >= 0.0 {
            let var = sigma_l[i].powf(beta) + sigma_r[i].powf(beta);
            if var <= best_var {
                found_best = true;
                best_var = var;
                i_bisect = i;
                weight_l = sigma_l[i].powf(beta);
                weight_r = sigma_r[i].powf(beta);

                if weight_l == 0.0 && weight_r == 0.0 {
                    weight_l = 1.0;
                    weight_r = 1.0;
                }
            }
        } else {
            if sigma_l[i] < 0.0 {
                return Err(MiserError::NoPointsLeft);
            }
            if sigma_r[i] < 0.0 {
                return Err(MiserError::NoPointsRight);
            }
        }
    }

    if !found_best {
        i_bisect = rng.gen_range(0..dim);
    }

    let xbi_l = xl[i_bisect];
    let xbi_m = xmid[i_bisect];
    let xbi_r = xu[i_bisect];

    let fraction_l = ((xbi_m - xbi_l) / (xbi_r - xbi_l)).abs();
    let fraction_r = 1.0 - fraction_l;

    let a = fraction_l * weight_l;
    let b = fraction_r * weight_r;

    let calls_l = state.params.min_calls
        + ((remaining_calls - 2 * state.params.min_calls) as f64 * a / (a + b)) as usize;
    let calls_r = state.params.min_calls
        + ((remaining_calls - 2 * state.params.min_calls) as f64 * b / (a + b)) as usize;

    let mut xu_tmp = xu.to_vec();
    xu_tmp[i_bisect] = xbi_m;
    let (res_l, err_l) = miser_integrate(f, xl, &xu_tmp, calls_l, rng, state)?;

    let mut xl_tmp = xl.to_vec();
    xl_tmp[i_bisect] = xbi_m;
    let (res_r, err_r) = miser_integrate(f, &xl_tmp, xu, calls_r, rng, state)?;

    let result = res_l + res_r;
    let abserr = (err_l * err_l + err_r * err_r).sqrt();

    Ok((result, abserr))
}

fn estimate_corrmc<F: MonteFunction, R: Rng>(
    f: &F,
    xl: &[f64],
    xu: &[f64],
    dim: usize,
    calls: usize,
    rng: &mut R,
    state: &mut MiserState,
    xmid: &[f64],
) -> Result<(f64, f64, Vec<f64>, Vec<f64>), MiserError> {
    let mut m = 0.0;
    let mut q = 0.0;
    let vol = xl.iter().zip(xu.iter()).fold(1.0, |acc, (&l, &u)| acc * (u - l));

    let mut x = vec![0.0; dim];
    let mut hits_l = vec![0; dim];
    let mut hits_r = vec![0; dim];
    let mut fsum_l = vec![0.0; dim];
    let mut fsum_r = vec![0.0; dim];
    let mut fsum2_l = vec![0.0; dim];
    let mut fsum2_r = vec![0.0; dim];
    let mut sigma_l = vec![-1.0; dim];
    let mut sigma_r = vec![-1.0; dim];

    let between = Uniform::new(0.0, 1.0);

    for n in 0..calls {
        let j = (n / 2) % dim;
        let side = n % 2;

        for i in 0..dim {
            let z = between.sample(rng);
            if i != j {
                x[i] = xl[i] + z * (xu[i] - xl[i]);
            } else {
                if side == 0 {
                    x[i] = xmid[i] + z * (xu[i] - xmid[i]);
                } else {
                    x[i] = xl[i] + z * (xmid[i] - xl[i]);
                }
            }
        }

        let fval = f.eval(&x);

        let d = fval - m;
        m += d / (n as f64 + 1.0);
        q += d * d * (n as f64 / (n as f64 + 1.0));

        for i in 0..dim {
            if x[i] <= xmid[i] {
                fsum_l[i] += fval;
                fsum2_l[i] += fval * fval;
                hits_l[i] += 1;
            } else {
                fsum_r[i] += fval;
                fsum2_r[i] += fval * fval;
                hits_r[i] += 1;
            }
        }
    }

    for i in 0..dim {
        let fraction_l = (xmid[i] - xl[i]) / (xu[i] - xl[i]);

        if hits_l[i] > 0 {
            fsum_l[i] /= hits_l[i] as f64;
            sigma_l[i] = (fsum2_l[i] - fsum_l[i] * fsum_l[i] / hits_l[i] as f64).sqrt();
            sigma_l[i] *= fraction_l * vol / hits_l[i] as f64;
        }

        if hits_r[i] > 0 {
            fsum_r[i] /= hits_r[i] as f64;
            sigma_r[i] = (fsum2_r[i] - fsum_r[i] * fsum_r[i] / hits_r[i] as f64).sqrt();
            sigma_r[i] *= (1.0 - fraction_l) * vol / hits_r[i] as f64;
        }
    }

    let result = vol * m;
    let abserr = if calls < 2 {
        f64::INFINITY
    } else {
        vol * (q / (calls as f64 * (calls as f64 - 1.0))).sqrt()
    };

    Ok((result, abserr, sigma_l, sigma_r))
}
use std::f64;
use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonteError {
    #[error("number of dimensions must match allocated size")]
    DimensionMismatch,
    #[error("xu must be greater than xl")]
    InvalidRange,
    #[error("range of integration is too large, please rescale")]
    RangeTooLarge,
    #[error("failed to allocate space for state struct")]
    AllocationFailed,
}

pub struct MontePlainState {
    x: Vec<f64>,
    dim: usize,
}

pub fn monte_plain_integrate<F>(
    f: F,
    xl: &[f64],
    xu: &[f64],
    dim: usize,
    calls: usize,
    rng: &mut impl Rng,
    state: &mut MontePlainState,
) -> Result<(f64, f64), MonteError>
where
    F: Fn(&[f64]) -> f64,
{
    if dim != state.dim {
        return Err(MonteError::DimensionMismatch);
    }

    for i in 0..dim {
        if xu[i] <= xl[i] {
            return Err(MonteError::InvalidRange);
        }

        if xu[i] - xl[i] > f64::MAX {
            return Err(MonteError::RangeTooLarge);
        }
    }

    let mut vol = 1.0;
    for i in 0..dim {
        vol *= xu[i] - xl[i];
    }

    let mut m = 0.0;
    let mut q = 0.0;

    for n in 0..calls {
        for i in 0..dim {
            state.x[i] = xl[i] + rng.gen::<f64>() * (xu[i] - xl[i]);
        }

        let fval = f(&state.x);
        let d = fval - m;
        m += d / (n as f64 + 1.0);
        q += d * d * (n as f64 / (n as f64 + 1.0));
    }

    let result = vol * m;
    let abserr = if calls < 2 {
        f64::INFINITY
    } else {
        vol * (q / (calls as f64 * (calls as f64 - 1.0))).sqrt()
    };

    Ok((result, abserr))
}

pub fn monte_plain_alloc(dim: usize) -> Result<MontePlainState, MonteError> {
    let x = vec![0.0; dim];
    Ok(MontePlainState { x, dim })
}

pub fn monte_plain_init(state: &mut MontePlainState) {
    for x in state.x.iter_mut() {
        *x = 0.0;
    }
}

pub fn monte_plain_free(_state: MontePlainState) {
    // Rust's drop trait automatically handles memory deallocation
}
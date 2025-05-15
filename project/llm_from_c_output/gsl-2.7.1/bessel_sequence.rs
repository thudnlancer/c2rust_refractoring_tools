use std::f64::consts;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BesselError {
    #[error("domain error")]
    DomainError,
    #[error("invalid input")]
    InvalidInput,
    #[error("ordering failure")]
    OrderingFailure,
}

pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

fn dydx_p(p: f64, u: f64, x: f64, nu: f64) -> f64 {
    -(p) / x + ((nu * nu) / (x * x) - 1.0) * u
}

fn dydx_u(p: f64, _u: f64, _x: f64) -> f64 {
    p
}

fn rk_step(nu: f64, x: f64, dx: f64, jp: &mut f64, j: &mut f64) {
    let p_0 = *jp;
    let u_0 = *j;

    let p_1 = dx * dydx_p(p_0, u_0, x, nu);
    let u_1 = dx * dydx_u(p_0, u_0, x);

    let p_2 = dx * dydx_p(p_0 + 0.5 * p_1, u_0 + 0.5 * u_1, x + 0.5 * dx, nu);
    let u_2 = dx * dydx_u(p_0 + 0.5 * p_1, u_0 + 0.5 * u_1, x + 0.5 * dx);

    let p_3 = dx * dydx_p(p_0 + 0.5 * p_2, u_0 + 0.5 * u_2, x + 0.5 * dx, nu);
    let u_3 = dx * dydx_u(p_0 + 0.5 * p_2, u_0 + 0.5 * u_2, x + 0.5 * dx);

    let p_4 = dx * dydx_p(p_0 + p_3, u_0 + u_3, x + dx, nu);
    let u_4 = dx * dydx_u(p_0 + p_3, u_0 + u_3, x + dx);

    *jp = p_0 + p_1 / 6.0 + p_2 / 3.0 + p_3 / 3.0 + p_4 / 6.0;
    *j = u_0 + u_1 / 6.0 + u_2 / 3.0 + u_3 / 3.0 + u_4 / 6.0;
}

pub fn bessel_jnu(nu: f64, x: f64) -> Result<SfResult, BesselError> {
    // Placeholder for actual Bessel function implementation
    Ok(SfResult {
        val: 0.0,
        err: 0.0,
    })
}

pub fn bessel_sequence_jnu_e(
    nu: f64,
    mode: u32,
    size: usize,
    v: &mut [f64],
) -> Result<(), BesselError> {
    if nu < 0.0 {
        return Err(BesselError::DomainError);
    } else if size == 0 {
        return Err(BesselError::InvalidInput);
    }

    let goal = mode & 0x03; // GSL_MODE_PREC
    let dx_array = [0.001, 0.03, 0.1]; // double, single, approx
    let dx_nominal = dx_array[goal as usize];

    let cnu = nu.ceil() as usize;
    let nu13 = nu.powf(1.0 / 3.0);
    let smalls = [
        0.01, 0.02, 0.4, 0.7, 1.3, 2.0, 2.5, 3.2, 3.5, 4.5, 6.0,
    ];
    let x_small = if nu >= 10.0 {
        nu - nu13
    } else {
        smalls[cnu.min(smalls.len() - 1)]
    };

    let mut i = 0;

    // Calculate the first point
    let mut x = v[0];
    let mut j0 = bessel_jnu(nu, x)?;
    v[0] = j0.val;
    i += 1;

    // Step over the case where the first point was actually zero
    if x == 0.0 {
        if i >= size || v[i] <= x {
            return Err(BesselError::OrderingFailure);
        }
        x = v[i];
        j0 = bessel_jnu(nu, x)?;
        v[i] = j0.val;
        i += 1;
    }

    // Calculate directly while argument is small
    while i < size && v[i] < x_small {
        if v[i] <= x {
            return Err(BesselError::OrderingFailure);
        }
        x = v[i];
        j0 = bessel_jnu(nu, x)?;
        v[i] = j0.val;
        i += 1;
    }

    // Get derivative and prepare for integration
    let j1 = bessel_jnu(nu + 1.0, x)?;
    let mut j = j0.val;
    let mut jp = -j1.val + nu / x * j0.val;

    // Integrate over remaining points
    while i < size {
        let dv = v[i] - x;
        let nd = (dv / dx_nominal).ceil() as usize;
        let dx = dv / nd as f64;

        if v[i] <= x {
            return Err(BesselError::OrderingFailure);
        }

        // Integrate over interval up to next sample point
        let mut xj = x;
        for _ in 0..nd {
            rk_step(nu, xj, dx, &mut jp, &mut j);
            xj += dx;
        }

        // Go to next interval
        x = v[i];
        v[i] = j;
        i += 1;
    }

    Ok(())
}
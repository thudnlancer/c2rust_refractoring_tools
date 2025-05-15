use std::f64::consts::E;
use std::f64;

// Default tuning constants
const TUNING_BISQUARE: f64 = 4.685;
const TUNING_CAUCHY: f64 = 2.385;
const TUNING_FAIR: f64 = 1.4;
const TUNING_HUBER: f64 = 1.345;
const TUNING_OLS: f64 = 1.0;
const TUNING_WELSCH: f64 = 2.985;

#[derive(Debug)]
pub struct RobustType {
    name: &'static str,
    weight_fn: fn(&[f64], &mut [f64]) -> Result<(), String>,
    dpsi_fn: fn(&[f64], &mut [f64]) -> Result<(), String>,
    tuning: f64,
}

fn bisquare(r: &[f64], w: &mut [f64]) -> Result<(), String> {
    if r.len() != w.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        let abs_ri = ri.abs();
        w[i] = if abs_ri < 1.0 {
            (1.0 - ri * ri) * (1.0 - ri * ri)
        } else {
            0.0
        };
    }

    Ok(())
}

fn bisquare_dpsi(r: &[f64], dpsi: &mut [f64]) -> Result<(), String> {
    if r.len() != dpsi.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        let abs_ri = ri.abs();
        dpsi[i] = if abs_ri < 1.0 {
            (1.0 - ri * ri) * (1.0 - 5.0 * ri * ri)
        } else {
            0.0
        };
    }

    Ok(())
}

static BISQUARE_TYPE: RobustType = RobustType {
    name: "bisquare",
    weight_fn: bisquare,
    dpsi_fn: bisquare_dpsi,
    tuning: TUNING_BISQUARE,
};

fn cauchy(r: &[f64], w: &mut [f64]) -> Result<(), String> {
    if r.len() != w.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        w[i] = 1.0 / (1.0 + ri * ri);
    }

    Ok(())
}

fn cauchy_dpsi(r: &[f64], dpsi: &mut [f64]) -> Result<(), String> {
    if r.len() != dpsi.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        let rsq = ri * ri;
        dpsi[i] = (1.0 - rsq) / (1.0 + rsq) / (1.0 + rsq);
    }

    Ok(())
}

static CAUCHY_TYPE: RobustType = RobustType {
    name: "cauchy",
    weight_fn: cauchy,
    dpsi_fn: cauchy_dpsi,
    tuning: TUNING_CAUCHY,
};

fn fair(r: &[f64], w: &mut [f64]) -> Result<(), String> {
    if r.len() != w.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        w[i] = 1.0 / (1.0 + ri.abs());
    }

    Ok(())
}

fn fair_dpsi(r: &[f64], dpsi: &mut [f64]) -> Result<(), String> {
    if r.len() != dpsi.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        let abs_ri = ri.abs();
        dpsi[i] = 1.0 / (1.0 + abs_ri) / (1.0 + abs_ri);
    }

    Ok(())
}

static FAIR_TYPE: RobustType = RobustType {
    name: "fair",
    weight_fn: fair,
    dpsi_fn: fair_dpsi,
    tuning: TUNING_FAIR,
};

fn huber(r: &[f64], w: &mut [f64]) -> Result<(), String> {
    if r.len() != w.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        let abs_ri = ri.abs();
        w[i] = if abs_ri <= 1.0 {
            1.0
        } else {
            1.0 / abs_ri
        };
    }

    Ok(())
}

fn huber_dpsi(r: &[f64], dpsi: &mut [f64]) -> Result<(), String> {
    if r.len() != dpsi.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        dpsi[i] = if ri.abs() <= 1.0 {
            1.0
        } else {
            0.0
        };
    }

    Ok(())
}

static HUBER_TYPE: RobustType = RobustType {
    name: "huber",
    weight_fn: huber,
    dpsi_fn: huber_dpsi,
    tuning: TUNING_HUBER,
};

fn ols(_r: &[f64], w: &mut [f64]) -> Result<(), String> {
    for val in w.iter_mut() {
        *val = 1.0;
    }
    Ok(())
}

fn ols_dpsi(_r: &[f64], dpsi: &mut [f64]) -> Result<(), String> {
    for val in dpsi.iter_mut() {
        *val = 1.0;
    }
    Ok(())
}

static OLS_TYPE: RobustType = RobustType {
    name: "ols",
    weight_fn: ols,
    dpsi_fn: ols_dpsi,
    tuning: TUNING_OLS,
};

fn welsch(r: &[f64], w: &mut [f64]) -> Result<(), String> {
    if r.len() != w.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        w[i] = (-ri * ri).exp();
    }

    Ok(())
}

fn welsch_dpsi(r: &[f64], dpsi: &mut [f64]) -> Result<(), String> {
    if r.len() != dpsi.len() {
        return Err("Input and output vectors must have same length".to_string());
    }

    for (i, &ri) in r.iter().enumerate() {
        dpsi[i] = (1.0 - 2.0 * ri * ri) * (-ri * ri).exp();
    }

    Ok(())
}

static WELSCH_TYPE: RobustType = RobustType {
    name: "welsch",
    weight_fn: welsch,
    dpsi_fn: welsch_dpsi,
    tuning: TUNING_WELSCH,
};

pub static GSL_MULTIFIT_ROBUST_DEFAULT: &RobustType = &BISQUARE_TYPE;
pub static GSL_MULTIFIT_ROBUST_BISQUARE: &RobustType = &BISQUARE_TYPE;
pub static GSL_MULTIFIT_ROBUST_CAUCHY: &RobustType = &CAUCHY_TYPE;
pub static GSL_MULTIFIT_ROBUST_FAIR: &RobustType = &FAIR_TYPE;
pub static GSL_MULTIFIT_ROBUST_HUBER: &RobustType = &HUBER_TYPE;
pub static GSL_MULTIFIT_ROBUST_OLS: &RobustType = &OLS_TYPE;
pub static GSL_MULTIFIT_ROBUST_WELSCH: &RobustType = &WELSCH_TYPE;
//! Bessel amplitude and phase functions
//!
//! This module provides Chebyshev expansions for amplitude and phase functions
//! used in Bessel function evaluations, as well as asymptotic expansions for
//! large arguments.

use std::f64::consts::PI;

/// Chebyshev series structure
pub struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

// Bessel amplitude and phase Chebyshev expansions

static BM0_DATA: [f64; 21] = [
    0.09284961637381644,
    -0.00142987707403484,
    0.00002830579271257,
    -0.00000143300611424,
    0.00000012028628046,
    -0.00000001397113013,
    0.00000000204076188,
    -0.00000000035399669,
    0.00000000007024759,
    -0.00000000001554107,
    0.00000000000376226,
    -0.00000000000098282,
    0.00000000000027408,
    -0.00000000000008091,
    0.00000000000002511,
    -0.00000000000000814,
    0.00000000000000275,
    -0.00000000000000096,
    0.00000000000000034,
    -0.00000000000000012,
    0.00000000000000004,
];

pub const BESSEL_AMP_PHASE_BM0_CS: ChebSeries = ChebSeries {
    data: &BM0_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static BTH0_DATA: [f64; 24] = [
    -0.24639163774300119,
    0.001737098307508963,
    -0.000062183633402968,
    0.000004368050165742,
    -0.000000456093019869,
    0.000000062197400101,
    -0.000000010300442889,
    0.000000001979526776,
    -0.000000000428198396,
    0.000000000102035840,
    -0.000000000026363898,
    0.000000000007297935,
    -0.000000000002144188,
    0.000000000000663693,
    -0.000000000000215126,
    0.000000000000072659,
    -0.000000000000025465,
    0.000000000000009229,
    -0.000000000000003448,
    0.000000000000001325,
    -0.000000000000000522,
    0.000000000000000210,
    -0.000000000000000087,
    0.000000000000000036,
];

pub const BESSEL_AMP_PHASE_BTH0_CS: ChebSeries = ChebSeries {
    data: &BTH0_DATA,
    order: 23,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

static BM1_DATA: [f64; 21] = [
    0.1047362510931285,
    0.00442443893702345,
    -0.00005661639504035,
    0.00000231349417339,
    -0.00000017377182007,
    0.00000001893209930,
    -0.00000000265416023,
    0.00000000044740209,
    -0.00000000008691795,
    0.00000000001891492,
    -0.00000000000451884,
    0.00000000000116765,
    -0.00000000000032265,
    0.00000000000009450,
    -0.00000000000002913,
    0.00000000000000939,
    -0.00000000000000315,
    0.00000000000000109,
    -0.00000000000000039,
    0.00000000000000014,
    -0.00000000000000005,
];

pub const BESSEL_AMP_PHASE_BM1_CS: ChebSeries = ChebSeries {
    data: &BM1_DATA,
    order: 20,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

static BTH1_DATA: [f64; 24] = [
    0.74060141026313850,
    -0.004571755659637690,
    0.000119818510964326,
    -0.000006964561891648,
    0.000000655495621447,
    -0.000000084066228945,
    0.000000013376886564,
    -0.000000002499565654,
    0.000000000529495100,
    -0.000000000124135944,
    0.000000000031656485,
    -0.000000000008668640,
    0.000000000002523758,
    -0.000000000000775085,
    0.000000000000249527,
    -0.000000000000083773,
    0.000000000000029205,
    -0.000000000000010534,
    0.000000000000003919,
    -0.000000000000001500,
    0.000000000000000589,
    -0.000000000000000237,
    0.000000000000000097,
    -0.000000000000000040,
];

pub const BESSEL_AMP_PHASE_BTH1_CS: ChebSeries = ChebSeries {
    data: &BTH1_DATA,
    order: 23,
    a: -1.0,
    b: 1.0,
    order_sp: 12,
};

/// Compute the asymptotic expansion for M_nu(x)
///
/// # Arguments
/// * `nu` - Order parameter
/// * `x` - Argument (must be > 0)
///
/// # Returns
/// Result containing the computed value or an error if x <= 0
pub fn bessel_asymp_mnu(nu: f64, x: f64) -> Result<f64, &'static str> {
    if x <= 0.0 {
        return Err("x must be positive");
    }

    let r = 2.0 * nu / x;
    let r2 = r * r;
    let x2 = x * x;
    let term1 = (r2 - 1.0 / x2) / 8.0;
    let term2 = (r2 - 1.0 / x2) * (r2 - 9.0 / x2) * 3.0 / 128.0;
    let mnu2_c = 2.0 / PI * (1.0 + term1 + term2);
    Ok(mnu2_c.sqrt() / x.sqrt())
}

/// Compute the asymptotic expansion for the correction to theta_nu(x)
///
/// # Arguments
/// * `nu` - Order parameter
/// * `x` - Argument (must be > 0)
///
/// # Returns
/// Result containing the computed value or an error if x <= 0
pub fn bessel_asymp_thetanu_corr(nu: f64, x: f64) -> Result<f64, &'static str> {
    if x <= 0.0 {
        return Err("x must be positive");
    }

    let r = 2.0 * nu / x;
    let r2 = r * r;
    let x2 = x * x;
    let term1 = x * (r2 - 1.0 / x2) / 8.0;
    let term2 = x * (r2 - 1.0 / x2) * (r2 - 25.0 / x2) / 384.0;
    Ok(-0.25 * PI + term1 + term2)
}
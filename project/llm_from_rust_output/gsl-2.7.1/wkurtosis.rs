use num_traits::{ToPrimitive, Zero, NumCast};
use f128;

pub type Size = usize;

pub fn gsl_stats_float_wkurtosis(
    w: &[f32],
    wstride: Size,
    data: &[f32],
    stride: Size,
    n: Size,
) -> f64 {
    let wmean = gsl_stats_float_wmean(w, wstride, data, stride, n);
    let wsd = gsl_stats_float_wsd_m(w, wstride, data, stride, n, wmean);
    gsl_stats_float_wkurtosis_m_sd(w, wstride, data, stride, n, wmean, wsd)
}

pub fn gsl_stats_wkurtosis(
    w: &[f64],
    wstride: Size,
    data: &[f64],
    stride: Size,
    n: Size,
) -> f64 {
    let wmean = gsl_stats_wmean(w, wstride, data, stride, n);
    let wsd = gsl_stats_wsd_m(w, wstride, data, stride, n, wmean);
    gsl_stats_wkurtosis_m_sd(w, wstride, data, stride, n, wmean, wsd)
}

pub fn gsl_stats_long_double_wkurtosis(
    w: &[f128::f128],
    wstride: Size,
    data: &[f128::f128],
    stride: Size,
    n: Size,
) -> f64 {
    let wmean = gsl_stats_long_double_wmean(w, wstride, data, stride, n);
    let wsd = gsl_stats_long_double_wsd_m(w, wstride, data, stride, n, wmean);
    gsl_stats_long_double_wkurtosis_m_sd(w, wstride, data, stride, n, wmean, wsd)
}

pub fn gsl_stats_float_wkurtosis_m_sd(
    w: &[f32],
    wstride: Size,
    data: &[f32],
    stride: Size,
    n: Size,
    wmean: f64,
    wsd: f64,
) -> f64 {
    let mut wavg = f128::f128::zero();
    let mut kurtosis = f128::f128::zero();
    let mut W = f128::f128::zero();

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let x = f128::f128::from((data[i * stride] as f64 - wmean) / wsd);
            W += f128::f128::from(wi);
            wavg += (x * x * x * x - wavg) * (f128::f128::from(wi) / W);
        }
    }

    kurtosis = wavg - f128::f128::from(3.0f64);
    kurtosis.to_f64().unwrap()
}

pub fn gsl_stats_wkurtosis_m_sd(
    w: &[f64],
    wstride: Size,
    data: &[f64],
    stride: Size,
    n: Size,
    wmean: f64,
    wsd: f64,
) -> f64 {
    let mut wavg = f128::f128::zero();
    let mut kurtosis = f128::f128::zero();
    let mut W = f128::f128::zero();

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let x = f128::f128::from((data[i * stride] - wmean) / wsd);
            W += f128::f128::from(wi);
            wavg += (x * x * x * x - wavg) * (f128::f128::from(wi) / W);
        }
    }

    kurtosis = wavg - f128::f128::from(3.0f64);
    kurtosis.to_f64().unwrap()
}

pub fn gsl_stats_long_double_wkurtosis_m_sd(
    w: &[f128::f128],
    wstride: Size,
    data: &[f128::f128],
    stride: Size,
    n: Size,
    wmean: f64,
    wsd: f64,
) -> f64 {
    let mut wavg = f128::f128::zero();
    let mut kurtosis = f128::f128::zero();
    let mut W = f128::f128::zero();

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > f128::f128::zero() {
            let x = (data[i * stride] - f128::f128::from(wmean)) / f128::f128::from(wsd);
            W += wi;
            wavg += (x * x * x * x - wavg) * (wi / W);
        }
    }

    kurtosis = wavg - f128::f128::from(3.0f64);
    kurtosis.to_f64().unwrap()
}

// These would need to be implemented safely or wrapped from the C library
fn gsl_stats_float_wmean(_w: &[f32], _wstride: Size, _data: &[f32], _stride: Size, _n: Size) -> f64 { 0.0 }
fn gsl_stats_float_wsd_m(_w: &[f32], _wstride: Size, _data: &[f32], _stride: Size, _n: Size, _wmean: f64) -> f64 { 0.0 }
fn gsl_stats_wmean(_w: &[f64], _wstride: Size, _data: &[f64], _stride: Size, _n: Size) -> f64 { 0.0 }
fn gsl_stats_wsd_m(_w: &[f64], _wstride: Size, _data: &[f64], _stride: Size, _n: Size, _wmean: f64) -> f64 { 0.0 }
fn gsl_stats_long_double_wmean(_w: &[f128::f128], _wstride: Size, _data: &[f128::f128], _stride: Size, _n: Size) -> f64 { 0.0 }
fn gsl_stats_long_double_wsd_m(_w: &[f128::f128], _wstride: Size, _data: &[f128::f128], _stride: Size, _n: Size, _wmean: f64) -> f64 { 0.0 }
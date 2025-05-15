use num_traits::{Float, ToPrimitive};
use std::ops::{AddAssign, Div, Mul, Sub};

pub type SizeT = usize;

pub fn gsl_stats_float_wabsdev(
    w: &[f32],
    wstride: SizeT,
    data: &[f32],
    stride: SizeT,
    n: SizeT,
) -> f64 {
    let wmean = gsl_stats_float_wmean(w, wstride, data, stride, n);
    gsl_stats_float_wabsdev_m(w, wstride, data, stride, n, wmean)
}

pub fn gsl_stats_wabsdev(
    w: &[f64],
    wstride: SizeT,
    data: &[f64],
    stride: SizeT,
    n: SizeT,
) -> f64 {
    let wmean = gsl_stats_wmean(w, wstride, data, stride, n);
    gsl_stats_wabsdev_m(w, wstride, data, stride, n, wmean)
}

pub fn gsl_stats_long_double_wabsdev(
    w: &[f128::f128],
    wstride: SizeT,
    data: &[f128::f128],
    stride: SizeT,
    n: SizeT,
) -> f64 {
    let wmean = gsl_stats_long_double_wmean(w, wstride, data, stride, n);
    gsl_stats_long_double_wabsdev_m(w, wstride, data, stride, n, wmean)
}

pub fn gsl_stats_float_wabsdev_m(
    w: &[f32],
    wstride: SizeT,
    data: &[f32],
    stride: SizeT,
    n: SizeT,
    wmean: f64,
) -> f64 {
    let mut wabsdev = f128::f128::new(0);
    let mut W = f128::f128::new(0);

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let delta = f128::f128::new((data[i * stride] as f64 - wmean).abs());
            W += f128::f128::new(wi as f64);
            wabsdev += (delta - wabsdev) * (f128::f128::new(wi as f64) / W);
        }
    }

    wabsdev.to_f64().unwrap()
}

pub fn gsl_stats_wabsdev_m(
    w: &[f64],
    wstride: SizeT,
    data: &[f64],
    stride: SizeT,
    n: SizeT,
    wmean: f64,
) -> f64 {
    let mut wabsdev = f128::f128::new(0);
    let mut W = f128::f128::new(0);

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let delta = f128::f128::new((data[i * stride] - wmean).abs());
            W += f128::f128::new(wi);
            wabsdev += (delta - wabsdev) * (f128::f128::new(wi) / W);
        }
    }

    wabsdev.to_f64().unwrap()
}

pub fn gsl_stats_long_double_wabsdev_m(
    w: &[f128::f128],
    wstride: SizeT,
    data: &[f128::f128],
    stride: SizeT,
    n: SizeT,
    wmean: f64,
) -> f64 {
    let mut wabsdev = f128::f128::new(0);
    let mut W = f128::f128::new(0);

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > f128::f128::new(0) {
            let delta = f128::f128::new((data[i * stride] - f128::f128::new(wmean)).to_f64().unwrap().abs());
            W += wi;
            wabsdev += (delta - wabsdev) * (wi / W);
        }
    }

    wabsdev.to_f64().unwrap()
}

// Placeholder for external functions - these would need to be implemented or wrapped safely
fn gsl_stats_float_wmean(_w: &[f32], _wstride: SizeT, _data: &[f32], _stride: SizeT, _n: SizeT) -> f64 {
    unimplemented!()
}

fn gsl_stats_wmean(_w: &[f64], _wstride: SizeT, _data: &[f64], _stride: SizeT, _n: SizeT) -> f64 {
    unimplemented!()
}

fn gsl_stats_long_double_wmean(_w: &[f128::f128], _wstride: SizeT, _data: &[f128::f128], _stride: SizeT, _n: SizeT) -> f64 {
    unimplemented!()
}
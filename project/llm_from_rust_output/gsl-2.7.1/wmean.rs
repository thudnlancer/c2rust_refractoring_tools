use num_traits::{ToPrimitive, Zero, NumCast};
use std::ops::{AddAssign, Div, Sub};

pub type SizeT = usize;

pub fn gsl_stats_float_wmean(
    w: &[f32],
    wstride: SizeT,
    data: &[f32],
    stride: SizeT,
    size: SizeT,
) -> f64 {
    let mut wmean = f128::f128::zero();
    let mut w_sum = f128::f128::zero();
    
    for i in 0..size {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let wi_f128 = f128::f128::from(wi);
            w_sum += wi_f128;
            let data_val = f128::f128::from(data[i * stride]);
            wmean += (data_val - wmean) * (wi_f128 / w_sum);
        }
    }
    wmean.to_f64().unwrap()
}

pub fn gsl_stats_long_double_wmean(
    w: &[f128::f128],
    wstride: SizeT,
    data: &[f128::f128],
    stride: SizeT,
    size: SizeT,
) -> f64 {
    let mut wmean = f128::f128::zero();
    let mut w_sum = f128::f128::zero();
    
    for i in 0..size {
        let wi = w[i * wstride];
        if wi > f128::f128::zero() {
            w_sum += wi;
            let data_val = data[i * stride];
            wmean += (data_val - wmean) * (wi / w_sum);
        }
    }
    wmean.to_f64().unwrap()
}

pub fn gsl_stats_wmean(
    w: &[f64],
    wstride: SizeT,
    data: &[f64],
    stride: SizeT,
    size: SizeT,
) -> f64 {
    let mut wmean = f128::f128::zero();
    let mut w_sum = f128::f128::zero();
    
    for i in 0..size {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let wi_f128 = f128::f128::from(wi);
            w_sum += wi_f128;
            let data_val = f128::f128::from(data[i * stride]);
            wmean += (data_val - wmean) * (wi_f128 / w_sum);
        }
    }
    wmean.to_f64().unwrap()
}
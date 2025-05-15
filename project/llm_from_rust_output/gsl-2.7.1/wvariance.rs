use num_traits::{ToPrimitive, Zero};
use std::ops::{AddAssign, MulAssign, DivAssign, Sub};

pub type SizeT = usize;

trait WeightedOps<T> {
    fn compute_wvariance(w: &[T], data: &[T], wmean: f64) -> f64;
    fn compute_wtss(w: &[T], data: &[T], wmean: f64) -> f64;
    fn compute_factor(w: &[T]) -> f64;
}

impl WeightedOps<f64> for f64 {
    fn compute_wvariance(w: &[f64], data: &[f64], wmean: f64) -> f64 {
        let mut wvariance = 0.0;
        let mut W = 0.0;
        
        for (wi, di) in w.iter().zip(data.iter()) {
            if *wi > 0.0 {
                let delta = *di - wmean;
                W += *wi;
                wvariance += (delta * delta - wvariance) * (*wi / W);
            }
        }
        wvariance
    }

    fn compute_wtss(w: &[f64], data: &[f64], wmean: f64) -> f64 {
        let mut wtss = 0.0;
        
        for (wi, di) in w.iter().zip(data.iter()) {
            if *wi > 0.0 {
                let delta = *di - wmean;
                wtss += *wi * delta * delta;
            }
        }
        wtss
    }

    fn compute_factor(w: &[f64]) -> f64 {
        let mut a = 0.0;
        let mut b = 0.0;
        
        for wi in w {
            if *wi > 0.0 {
                a += *wi;
                b += *wi * *wi;
            }
        }
        a * a / (a * a - b)
    }
}

impl WeightedOps<f32> for f32 {
    fn compute_wvariance(w: &[f32], data: &[f32], wmean: f64) -> f64 {
        let mut wvariance = 0.0;
        let mut W = 0.0;
        
        for (wi, di) in w.iter().zip(data.iter()) {
            if *wi > 0.0 {
                let delta = f64::from(*di) - wmean;
                W += f64::from(*wi);
                wvariance += (delta * delta - wvariance) * (f64::from(*wi) / W);
            }
        }
        wvariance
    }

    fn compute_wtss(w: &[f32], data: &[f32], wmean: f64) -> f64 {
        let mut wtss = 0.0;
        
        for (wi, di) in w.iter().zip(data.iter()) {
            if *wi > 0.0 {
                let delta = f64::from(*di) - wmean;
                wtss += f64::from(*wi) * delta * delta;
            }
        }
        wtss
    }

    fn compute_factor(w: &[f32]) -> f64 {
        let mut a = 0.0;
        let mut b = 0.0;
        
        for wi in w {
            if *wi > 0.0 {
                a += f64::from(*wi);
                b += f64::from(*wi * *wi);
            }
        }
        a * a / (a * a - b)
    }
}

pub fn gsl_stats_wvariance_with_fixed_mean(w: &[f64], data: &[f64], wmean: f64) -> f64 {
    f64::compute_wvariance(w, data, wmean)
}

pub fn gsl_stats_wsd_with_fixed_mean(w: &[f64], data: &[f64], wmean: f64) -> f64 {
    f64::compute_wvariance(w, data, wmean).sqrt()
}

pub fn gsl_stats_wvariance_m(w: &[f64], data: &[f64], wmean: f64) -> f64 {
    let variance = f64::compute_wvariance(w, data, wmean);
    let scale = f64::compute_factor(w);
    scale * variance
}

pub fn gsl_stats_wsd_m(w: &[f64], data: &[f64], wmean: f64) -> f64 {
    let variance = f64::compute_wvariance(w, data, wmean);
    let scale = f64::compute_factor(w);
    (scale * variance).sqrt()
}

pub fn gsl_stats_wtss_m(w: &[f64], data: &[f64], wmean: f64) -> f64 {
    f64::compute_wtss(w, data, wmean)
}

// Similar implementations for f32 can be added following the same pattern
use num_traits::{ToPrimitive, Float};
use std::ops::Sub;

pub type size_t = usize;

trait AbsDev {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64
    where
        Self: Sized + Copy + ToPrimitive;
}

impl AbsDev for i8 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for u8 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for i16 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for u16 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for i32 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for u32 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for i64 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for u64 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for f32 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x.to_f64().unwrap() - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for f64 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        data.iter()
            .step_by(stride)
            .map(|&x| (x - mean).abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

impl AbsDev for f128::f128 {
    fn absdev(data: &[Self], stride: usize, mean: f64) -> f64 {
        let mean_f128 = f128::f128::new(mean);
        data.iter()
            .step_by(stride)
            .map(|&x| (x - mean_f128).to_f64().unwrap().abs())
            .sum::<f64>()
            / data.len() as f64
    }
}

// Generic functions for each type
pub fn gsl_stats_char_absdev(data: &[i8], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    i8::absdev(data, stride, mean)
}

pub fn gsl_stats_uchar_absdev(data: &[u8], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    u8::absdev(data, stride, mean)
}

pub fn gsl_stats_short_absdev(data: &[i16], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    i16::absdev(data, stride, mean)
}

pub fn gsl_stats_ushort_absdev(data: &[u16], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    u16::absdev(data, stride, mean)
}

pub fn gsl_stats_int_absdev(data: &[i32], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    i32::absdev(data, stride, mean)
}

pub fn gsl_stats_uint_absdev(data: &[u32], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    u32::absdev(data, stride, mean)
}

pub fn gsl_stats_long_absdev(data: &[i64], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    i64::absdev(data, stride, mean)
}

pub fn gsl_stats_ulong_absdev(data: &[u64], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    u64::absdev(data, stride, mean)
}

pub fn gsl_stats_float_absdev(data: &[f32], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    f32::absdev(data, stride, mean)
}

pub fn gsl_stats_absdev(data: &[f64], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).sum::<f64>() / data.len() as f64;
    f64::absdev(data, stride, mean)
}

pub fn gsl_stats_long_double_absdev(data: &[f128::f128], stride: usize) -> f64 {
    let mean = data.iter().step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / data.len() as f64;
    f128::f128::absdev(data, stride, mean)
}
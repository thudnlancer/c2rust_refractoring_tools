use std::f64;
use std::f32;
use std::mem;

pub fn kurtosis_long_double(data: &[f64]) -> f64 {
    kurtosis_impl(data)
}

pub fn kurtosis_double(data: &[f64]) -> f64 {
    kurtosis_impl(data)
}

pub fn kurtosis_float(data: &[f32]) -> f32 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64) as f32
}

pub fn kurtosis_ulong(data: &[u64]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_long(data: &[i64]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_uint(data: &[u32]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_int(data: &[i32]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_ushort(data: &[u16]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_short(data: &[i16]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_uchar(data: &[u8]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

pub fn kurtosis_char(data: &[i8]) -> f64 {
    let data_f64: Vec<f64> = data.iter().map(|&x| x as f64).collect();
    kurtosis_impl(&data_f64)
}

fn kurtosis_impl(data: &[f64]) -> f64 {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    let m4 = data.iter().map(|x| (x - mean).powi(4)).sum::<f64>() / data.len() as f64;
    
    m4 / (variance * variance) - 3.0
}
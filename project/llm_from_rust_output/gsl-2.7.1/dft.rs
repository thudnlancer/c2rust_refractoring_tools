use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub enum FftDirection {
    Forward = -1,
    Backward = 1,
}

pub fn gsl_dft_complex_forward(
    data: &[f64],
    stride: usize,
    n: usize,
    result: &mut [f64],
) -> i32 {
    gsl_dft_complex_transform(data, stride, n, result, FftDirection::Forward)
}

pub fn gsl_dft_complex_backward(
    data: &[f64],
    stride: usize,
    n: usize,
    result: &mut [f64],
) -> i32 {
    gsl_dft_complex_transform(data, stride, n, result, FftDirection::Backward)
}

pub fn gsl_dft_complex_inverse(
    data: &[f64],
    stride: usize,
    n: usize,
    result: &mut [f64],
) -> i32 {
    let status = gsl_dft_complex_transform(data, stride, n, result, FftDirection::Backward);
    let norm = 1.0 / n as f64;
    
    for i in 0..n {
        let idx = 2 * stride * i;
        result[idx] *= norm;
        result[idx + 1] *= norm;
    }
    
    status
}

pub fn gsl_dft_complex_transform(
    data: &[f64],
    stride: usize,
    n: usize,
    result: &mut [f64],
    sign: FftDirection,
) -> i32 {
    let d_theta = 2.0 * sign as i32 as f64 * PI / n as f64;
    
    for i in 0..n {
        let mut sum_real = 0.0;
        let mut sum_imag = 0.0;
        let mut exponent = 0;
        
        for j in 0..n {
            let theta = d_theta * exponent as f64;
            let w_real = theta.cos();
            let w_imag = theta.sin();
            
            let data_real = data[2 * stride * j];
            let data_imag = data[2 * stride * j + 1];
            
            sum_real += w_real * data_real - w_imag * data_imag;
            sum_imag += w_real * data_imag + w_imag * data_real;
            
            exponent = (exponent + i) % n;
        }
        
        result[2 * stride * i] = sum_real;
        result[2 * stride * i + 1] = sum_imag;
    }
    
    0
}

// Float versions

pub fn gsl_dft_complex_float_forward(
    data: &[f32],
    stride: usize,
    n: usize,
    result: &mut [f32],
) -> i32 {
    gsl_dft_complex_float_transform(data, stride, n, result, FftDirection::Forward)
}

pub fn gsl_dft_complex_float_backward(
    data: &[f32],
    stride: usize,
    n: usize,
    result: &mut [f32],
) -> i32 {
    gsl_dft_complex_float_transform(data, stride, n, result, FftDirection::Backward)
}

pub fn gsl_dft_complex_float_inverse(
    data: &[f32],
    stride: usize,
    n: usize,
    result: &mut [f32],
) -> i32 {
    let status = gsl_dft_complex_float_transform(data, stride, n, result, FftDirection::Backward);
    let norm = 1.0 / n as f32;
    
    for i in 0..n {
        let idx = 2 * stride * i;
        result[idx] *= norm;
        result[idx + 1] *= norm;
    }
    
    status
}

pub fn gsl_dft_complex_float_transform(
    data: &[f32],
    stride: usize,
    n: usize,
    result: &mut [f32],
    sign: FftDirection,
) -> i32 {
    let d_theta = 2.0 * sign as i32 as f64 * PI / n as f64;
    
    for i in 0..n {
        let mut sum_real = 0.0;
        let mut sum_imag = 0.0;
        let mut exponent = 0;
        
        for j in 0..n {
            let theta = d_theta * exponent as f64;
            let w_real = theta.cos() as f32;
            let w_imag = theta.sin() as f32;
            
            let data_real = data[2 * stride * j];
            let data_imag = data[2 * stride * j + 1];
            
            sum_real += w_real * data_real - w_imag * data_imag;
            sum_imag += w_real * data_imag + w_imag * data_real;
            
            exponent = (exponent + i) % n;
        }
        
        result[2 * stride * i] = sum_real;
        result[2 * stride * i + 1] = sum_imag;
    }
    
    0
}
use num_traits::{Float, ToPrimitive};

pub fn gsl_stats_trmean_from_sorted_data<T: Float + ToPrimitive>(
    trim: f64,
    sorted_data: &[T],
    stride: usize,
) -> f64 {
    if trim >= 0.5 {
        // For median calculation, we'd need a separate implementation
        // This is a placeholder since the original C code calls external functions
        unimplemented!("Median calculation needs separate implementation")
    } else {
        let size = sorted_data.len();
        let ilow = (trim * size as f64).floor() as usize;
        let ihigh = size - ilow - 1;
        
        let mut mean = 0.0;
        let mut k = 0.0;
        
        for i in (ilow..=ihigh).step_by(stride) {
            let value = sorted_data[i].to_f64().unwrap();
            let delta = value - mean;
            k += 1.0;
            mean += delta / k;
        }
        
        mean
    }
}

// Type-specific wrappers
pub fn gsl_stats_uchar_trmean_from_sorted_data(trim: f64, data: &[u8], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_char_trmean_from_sorted_data(trim: f64, data: &[i8], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_ushort_trmean_from_sorted_data(trim: f64, data: &[u16], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_short_trmean_from_sorted_data(trim: f64, data: &[i16], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_uint_trmean_from_sorted_data(trim: f64, data: &[u32], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_int_trmean_from_sorted_data(trim: f64, data: &[i32], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_ulong_trmean_from_sorted_data(trim: f64, data: &[u64], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_long_trmean_from_sorted_data(trim: f64, data: &[i64], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_float_trmean_from_sorted_data(trim: f64, data: &[f32], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

pub fn gsl_stats_double_trmean_from_sorted_data(trim: f64, data: &[f64], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}

// For f128 support, would need to use a crate that provides f128 type
#[cfg(feature = "f128")]
pub fn gsl_stats_long_double_trmean_from_sorted_data(trim: f64, data: &[f128], stride: usize) -> f64 {
    gsl_stats_trmean_from_sorted_data(trim, data, stride)
}
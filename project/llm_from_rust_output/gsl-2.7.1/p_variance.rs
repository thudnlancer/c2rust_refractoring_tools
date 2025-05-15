use libc::{c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort};
use f128;

pub type size_t = c_ulong;

pub trait Variance {
    fn variance(data: &[Self], stride: size_t) -> c_double where Self: Sized;
}

macro_rules! impl_variance {
    ($type:ty, $func:ident) => {
        impl Variance for $type {
            fn variance(data: &[Self], stride: size_t) -> c_double {
                unsafe { $func(data.as_ptr(), stride, data.len() as size_t) }
            }
        }
    };
}

impl_variance!(c_char, gsl_stats_char_variance);
impl_variance!(c_long, gsl_stats_long_variance);
impl_variance!(c_uint, gsl_stats_uint_variance);
impl_variance!(c_int, gsl_stats_int_variance);
impl_variance!(c_ushort, gsl_stats_ushort_variance);
impl_variance!(c_short, gsl_stats_short_variance);
impl_variance!(c_ulong, gsl_stats_ulong_variance);
impl_variance!(c_float, gsl_stats_float_variance);
impl_variance!(c_uchar, gsl_stats_uchar_variance);
impl_variance!(c_double, gsl_stats_variance);
impl_variance!(f128::f128, gsl_stats_long_double_variance);

pub fn pvariance<T: Variance>(data1: &[T], stride1: size_t, data2: &[T], stride2: size_t) -> c_double {
    let n1 = data1.len() as c_double;
    let n2 = data2.len() as c_double;
    
    let var1 = T::variance(data1, stride1);
    let var2 = T::variance(data2, stride2);
    
    ((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pvariance() {
        let data1 = [1.0, 2.0, 3.0];
        let data2 = [4.0, 5.0, 6.0];
        let result = pvariance(&data1, 1, &data2, 1);
        assert!(result > 0.0);
    }
}
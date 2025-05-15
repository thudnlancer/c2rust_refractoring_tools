use num_traits::{ToPrimitive, Num, Float};
use f128;

pub type size_t = usize;

pub trait Median<T> {
    fn median_from_sorted_data(sorted_data: &[T], stride: usize) -> f64;
    fn median(data: &mut [T], stride: usize) -> f64;
}

macro_rules! impl_median_for_integer {
    ($type:ty) => {
        impl Median<$type> for $type {
            fn median_from_sorted_data(sorted_data: &[$type], stride: usize) -> f64 {
                let n = sorted_data.len();
                if n == 0 {
                    return 0.0;
                }
                
                let lhs = (n - 1) / 2;
                let rhs = n / 2;
                
                if lhs == rhs {
                    sorted_data[lhs * stride] as f64
                } else {
                    (sorted_data[lhs * stride] as f64 + sorted_data[rhs * stride] as f64) / 2.0
                }
            }
            
            fn median(data: &mut [$type], stride: usize) -> f64 {
                let n = data.len();
                if n == 0 {
                    return 0.0;
                }
                
                let lhs = (n - 1) / 2;
                let rhs = n / 2;
                
                if lhs == rhs {
                    data[lhs * stride] as f64
                } else {
                    (data[lhs * stride] as f64 + data[rhs * stride] as f64) / 2.0
                }
            }
        }
    };
}

macro_rules! impl_median_for_float {
    ($type:ty) => {
        impl Median<$type> for $type {
            fn median_from_sorted_data(sorted_data: &[$type], stride: usize) -> f64 {
                let n = sorted_data.len();
                if n == 0 {
                    return 0.0;
                }
                
                let lhs = (n - 1) / 2;
                let rhs = n / 2;
                
                if lhs == rhs {
                    sorted_data[lhs * stride] as f64
                } else {
                    (sorted_data[lhs * stride] + sorted_data[rhs * stride]) as f64 / 2.0
                }
            }
            
            fn median(data: &mut [$type], stride: usize) -> f64 {
                let n = data.len();
                if n == 0 {
                    return 0.0;
                }
                
                let lhs = (n - 1) / 2;
                let rhs = n / 2;
                
                if lhs == rhs {
                    data[lhs * stride] as f64
                } else {
                    (data[lhs * stride] + data[rhs * stride]) as f64 / 2.0
                }
            }
        }
    };
}

impl Median<f128::f128> for f128::f128 {
    fn median_from_sorted_data(sorted_data: &[f128::f128], stride: usize) -> f64 {
        let n = sorted_data.len();
        if n == 0 {
            return 0.0;
        }
        
        let lhs = (n - 1) / 2;
        let rhs = n / 2;
        
        if lhs == rhs {
            sorted_data[lhs * stride].to_f64().unwrap()
        } else {
            ((sorted_data[lhs * stride] + sorted_data[rhs * stride]) / f128::f128::new(2.0)).to_f64().unwrap()
        }
    }
    
    fn median(data: &mut [f128::f128], stride: usize) -> f64 {
        let n = data.len();
        if n == 0 {
            return 0.0;
        }
        
        let lhs = (n - 1) / 2;
        let rhs = n / 2;
        
        if lhs == rhs {
            data[lhs * stride].to_f64().unwrap()
        } else {
            (f128::f128::new(0.5) * (data[lhs * stride] + data[rhs * stride])).to_f64().unwrap()
        }
    }
}

impl_median_for_integer!(i8);
impl_median_for_integer!(i16);
impl_median_for_integer!(i32);
impl_median_for_integer!(i64);
impl_median_for_integer!(u8);
impl_median_for_integer!(u16);
impl_median_for_integer!(u32);
impl_median_for_integer!(u64);
impl_median_for_float!(f32);
impl_median_for_float!(f64);
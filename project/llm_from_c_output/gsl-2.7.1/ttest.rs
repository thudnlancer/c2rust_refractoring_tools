// Note: Since the original C code is using template-based code generation via includes,
// and Rust doesn't have an exact equivalent of C's preprocessor-based templates,
// we'll use Rust's generics and traits to achieve similar functionality.

use std::ops::{Add, Sub, Mul, Div};
use std::iter::Sum;
use num_traits::{Float, Num, NumCast, Signed, Zero, One};

trait TTestBase: 
    Copy + 
    PartialOrd + 
    Num + 
    NumCast + 
    Sum + 
    Signed + 
    Float + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> {}

impl<T> TTestBase for T where 
    T: Copy + 
    PartialOrd + 
    Num + 
    NumCast + 
    Sum + 
    Signed + 
    Float + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> {}

fn t_test<T: TTestBase>(data1: &[T], data2: &[T]) -> Result<(T, T), &'static str> {
    if data1.is_empty() || data2.is_empty() {
        return Err("empty dataset");
    }

    let n1 = T::from(data1.len()).ok_or("conversion error")?;
    let n2 = T::from(data2.len()).ok_or("conversion error")?;

    let mean1 = data1.iter().cloned().sum::<T>() / n1;
    let mean2 = data2.iter().cloned().sum::<T>() / n2;

    let var1 = data1.iter().map(|x| (*x - mean1).powi(2)).sum::<T>() / (n1 - T::one());
    let var2 = data2.iter().map(|x| (*x - mean2).powi(2)).sum::<T>() / (n2 - T::one());

    let t = (mean1 - mean2) / ((var1 / n1 + var2 / n2).sqrt());
    let dof = (var1 / n1 + var2 / n2).powi(2) 
        / ((var1 / n1).powi(2) / (n1 - T::one()) + (var2 / n2).powi(2) / (n2 - T::one()));

    Ok((t, dof))
}

// Implementations for all required types
macro_rules! impl_t_test {
    ($($t:ty),*) => {
        $(
            impl TTestBase for $t {}
        )*
    }
}

impl_t_test!(f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
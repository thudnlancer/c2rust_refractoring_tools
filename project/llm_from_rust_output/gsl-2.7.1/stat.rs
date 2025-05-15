use num_traits::ToPrimitive;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct GslHistogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram {
    pub fn mean(&self) -> f64 {
        let mut wmean = 0.0f128;
        let mut w = 0.0f128;

        for i in 0..self.n {
            let xi = (self.range[i + 1] + self.range[i]) / 2.0;
            let wi = self.bin[i];
            if wi > 0.0 {
                w += wi as f128;
                wmean += (xi as f128 - wmean) * (wi as f128 / w);
            }
        }

        wmean.to_f64().unwrap()
    }

    pub fn sigma(&self) -> f64 {
        let mut wmean = 0.0f128;
        let mut w = 0.0f128;

        // First pass to compute mean
        for i in 0..self.n {
            let xi = (self.range[i + 1] + self.range[i]) / 2.0;
            let wi = self.bin[i];
            if wi > 0.0 {
                w += wi as f128;
                wmean += (xi as f128 - wmean) * (wi as f128 / w);
            }
        }

        // Second pass to compute variance
        let mut wvariance = 0.0f128;
        w = 0.0f128;
        
        for i in 0..self.n {
            let xi = (self.range[i + 1] + self.range[i]) / 2.0;
            let wi = self.bin[i];
            if wi > 0.0 {
                let delta = xi as f128 - wmean;
                w += wi as f128;
                wvariance += (delta * delta - wvariance) * (wi as f128 / w);
            }
        }

        wvariance.to_f64().unwrap().sqrt()
    }

    pub fn sum(&self) -> f64 {
        self.bin.iter().sum()
    }
}

// f128 implementation for basic operations
#[derive(Copy, Clone, Debug)]
struct f128(f64, f64);

impl f128 {
    fn new(val: f64) -> Self {
        f128(val, 0.0)
    }

    fn to_f64(self) -> Option<f64> {
        Some(self.0)
    }
}

impl Add for f128 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        f128(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for f128 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        f128(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for f128 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        f128(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for f128 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        f128(self.0 / rhs.0, self.1 / rhs.1)
    }
}
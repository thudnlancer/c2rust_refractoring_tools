// Note: Since the original C code is using template-based code generation for different
// floating-point types, we'll implement generic versions in Rust that can handle
// f32, f64, and f128 (if available) types.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StatsError {
    details: String,
}

impl StatsError {
    fn new(msg: &str) -> StatsError {
        StatsError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for StatsError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn weighted_variance<T>(
    data: &[T],
    weights: &[T],
    mean: T,
) -> Result<T, StatsError>
where
    T: num_traits::Float + std::iter::Sum,
{
    if data.len() != weights.len() {
        return Err(StatsError::new("data and weights must have same length"));
    }

    if data.is_empty() {
        return Err(StatsError::new("input data cannot be empty"));
    }

    let sum_weights: T = weights.iter().cloned().sum();
    if sum_weights <= T::zero() {
        return Err(StatsError::new("sum of weights must be positive"));
    }

    let wvariance = data
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| w * (d - mean).powi(2))
        .sum::<T>()
        / sum_weights;

    Ok(wvariance)
}

// Implementations for specific float types for compatibility
pub fn weighted_variance_f32(data: &[f32], weights: &[f32], mean: f32) -> Result<f32, StatsError> {
    weighted_variance(data, weights, mean)
}

pub fn weighted_variance_f64(data: &[f64], weights: &[f64], mean: f64) -> Result<f64, StatsError> {
    weighted_variance(data, weights, mean)
}

#[cfg(feature = "extended_float")]
pub fn weighted_variance_f128(
    data: &[f128],
    weights: &[f128],
    mean: f128,
) -> Result<f128, StatsError> {
    weighted_variance(data, weights, mean)
}
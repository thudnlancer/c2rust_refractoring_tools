//! Routines to find maximum and minimum content of a histogram.
//! 
//! Contains the routines:
//! - `histogram_max_val`: find max content value
//! - `histogram_min_val`: find min content value
//! - `histogram_max_bin`: find index of max contents bin
//! - `histogram_min_bin`: find index of min contents bin

/// Represents a histogram with bins
pub struct Histogram {
    bins: Vec<f64>,
}

impl Histogram {
    /// Creates a new histogram with given bins
    pub fn new(bins: Vec<f64>) -> Self {
        Histogram { bins }
    }

    /// Finds the maximum value in the histogram bins
    pub fn max_val(&self) -> f64 {
        self.bins.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    /// Finds the index of the bin with maximum value
    pub fn max_bin(&self) -> usize {
        self.bins.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index)
            .unwrap_or(0)
    }

    /// Finds the minimum value in the histogram bins
    pub fn min_val(&self) -> f64 {
        self.bins.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    /// Finds the index of the bin with minimum value
    pub fn min_bin(&self) -> usize {
        self.bins.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index)
            .unwrap_or(0)
    }
}
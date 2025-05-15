use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HistogramError {
    message: String,
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HistogramError {}

impl HistogramError {
    fn new(msg: &str) -> HistogramError {
        HistogramError {
            message: msg.to_string(),
        }
    }
}

pub struct Histogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram {
    pub fn equal_bins_p(&self, other: &Histogram) -> bool {
        if self.n != other.n {
            return false;
        }

        for i in 0..=self.n {
            if (self.range[i] - other.range[i]).abs() > f64::EPSILON {
                return false;
            }
        }

        true
    }

    pub fn add(&mut self, other: &Histogram) -> Result<(), HistogramError> {
        if !self.equal_bins_p(other) {
            return Err(HistogramError::new("histograms have different binning"));
        }

        for i in 0..self.n {
            self.bin[i] += other.bin[i];
        }

        Ok(())
    }

    pub fn sub(&mut self, other: &Histogram) -> Result<(), HistogramError> {
        if !self.equal_bins_p(other) {
            return Err(HistogramError::new("histograms have different binning"));
        }

        for i in 0..self.n {
            self.bin[i] -= other.bin[i];
        }

        Ok(())
    }

    pub fn mul(&mut self, other: &Histogram) -> Result<(), HistogramError> {
        if !self.equal_bins_p(other) {
            return Err(HistogramError::new("histograms have different binning"));
        }

        for i in 0..self.n {
            self.bin[i] *= other.bin[i];
        }

        Ok(())
    }

    pub fn div(&mut self, other: &Histogram) -> Result<(), HistogramError> {
        if !self.equal_bins_p(other) {
            return Err(HistogramError::new("histograms have different binning"));
        }

        for i in 0..self.n {
            if other.bin[i].abs() < f64::EPSILON {
                return Err(HistogramError::new("division by zero"));
            }
            self.bin[i] /= other.bin[i];
        }

        Ok(())
    }

    pub fn scale(&mut self, factor: f64) -> Result<(), HistogramError> {
        for i in 0..self.n {
            self.bin[i] *= factor;
        }

        Ok(())
    }

    pub fn shift(&mut self, offset: f64) -> Result<(), HistogramError> {
        for i in 0..self.n {
            self.bin[i] += offset;
        }

        Ok(())
    }
}
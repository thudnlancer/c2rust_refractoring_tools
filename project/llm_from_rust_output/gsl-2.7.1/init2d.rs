use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Domain,
    Range,
    Invalid,
    NoMemory,
    Failed,
    // ... other error variants
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            GslError::Domain => "input domain error",
            GslError::Range => "output range error",
            GslError::Invalid => "invalid argument",
            GslError::NoMemory => "memory allocation failed",
            GslError::Failed => "generic failure",
            // ... other error messages
        })
    }
}

impl Error for GslError {}

#[derive(Debug)]
pub struct Histogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram2D {
    pub fn new(nx: usize, ny: usize) -> Result<Self, GslError> {
        if nx == 0 {
            return Err(GslError::Domain);
        }
        if ny == 0 {
            return Err(GslError::Domain);
        }

        Ok(Self {
            nx,
            ny,
            xrange: vec![0.0; nx + 1],
            yrange: vec![0.0; ny + 1],
            bin: vec![0.0; nx * ny],
        })
    }

    pub fn new_uniform(
        nx: usize,
        ny: usize,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
    ) -> Result<Self, GslError> {
        if xmin >= xmax {
            return Err(GslError::Invalid);
        }
        if ymin >= ymax {
            return Err(GslError::Invalid);
        }

        let mut h = Self::new(nx, ny)?;
        h.set_ranges_uniform(xmin, xmax, ymin, ymax)?;
        Ok(h)
    }

    fn make_uniform(range: &mut [f64], xmin: f64, xmax: f64) {
        let n = range.len() - 1;
        for i in 0..=n {
            let f1 = (n - i) as f64 / n as f64;
            let f2 = i as f64 / n as f64;
            range[i] = f1 * xmin + f2 * xmax;
        }
    }

    pub fn set_ranges_uniform(
        &mut self,
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
    ) -> Result<(), GslError> {
        if xmin >= xmax {
            return Err(GslError::Invalid);
        }
        if ymin >= ymax {
            return Err(GslError::Invalid);
        }

        Self::make_uniform(&mut self.xrange, xmin, xmax);
        Self::make_uniform(&mut self.yrange, ymin, ymax);
        self.bin.fill(0.0);
        Ok(())
    }

    pub fn set_ranges(
        &mut self,
        xrange: &[f64],
        yrange: &[f64],
    ) -> Result<(), GslError> {
        if xrange.len() != self.nx + 1 {
            return Err(GslError::Invalid);
        }
        if yrange.len() != self.ny + 1 {
            return Err(GslError::Invalid);
        }

        self.xrange.copy_from_slice(xrange);
        self.yrange.copy_from_slice(yrange);
        self.bin.fill(0.0);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h = Histogram2D::new(10, 5).unwrap();
        assert_eq!(h.nx, 10);
        assert_eq!(h.ny, 5);
        assert_eq!(h.xrange.len(), 11);
        assert_eq!(h.yrange.len(), 6);
        assert_eq!(h.bin.len(), 50);
    }

    #[test]
    fn test_new_uniform() {
        let h = Histogram2D::new_uniform(10, 5, 0.0, 10.0, 0.0, 5.0).unwrap();
        assert_eq!(h.xrange[0], 0.0);
        assert_eq!(h.xrange[10], 10.0);
        assert_eq!(h.yrange[0], 0.0);
        assert_eq!(h.yrange[5], 5.0);
    }
}
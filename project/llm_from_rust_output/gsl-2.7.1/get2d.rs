use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Dom,
    Range,
    Failure,
    Eof,
    // ... other error variants
    Sanity,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug, Clone)]
pub struct Histogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram2D {
    pub fn new(nx: usize, ny: usize, xrange: Vec<f64>, yrange: Vec<f64>, bin: Vec<f64>) -> Self {
        Self {
            nx,
            ny,
            xrange,
            yrange,
            bin,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Result<f64, GslError> {
        if i >= self.nx {
            return Err(GslError::Dom);
        }
        if j >= self.ny {
            return Err(GslError::Dom);
        }
        Ok(self.bin[i * self.ny + j])
    }

    pub fn get_xrange(&self, i: usize) -> Result<(f64, f64), GslError> {
        if i >= self.nx {
            return Err(GslError::Dom);
        }
        Ok((self.xrange[i], self.xrange[i + 1]))
    }

    pub fn get_yrange(&self, j: usize) -> Result<(f64, f64), GslError> {
        if j >= self.ny {
            return Err(GslError::Dom);
        }
        Ok((self.yrange[j], self.yrange[j + 1]))
    }

    pub fn find(&self, x: f64, y: f64) -> Result<(usize, usize), GslError> {
        let i = self.find_in_range(&self.xrange, x, self.nx)?;
        let j = self.find_in_range(&self.yrange, y, self.ny)?;
        Ok((i, j))
    }

    fn find_in_range(&self, range: &[f64], x: f64, n: usize) -> Result<usize, GslError> {
        if x < range[0] {
            return Err(GslError::Dom);
        }
        if x >= range[n] {
            return Err(GslError::Dom);
        }

        let u = (x - range[0]) / (range[n] - range[0]);
        let i_linear = (u * n as f64) as usize;

        if x >= range[i_linear] && x < range[i_linear + 1] {
            return Ok(i_linear);
        }

        let mut lower = 0;
        let mut upper = n;

        while upper - lower > 1 {
            let mid = (upper + lower) / 2;
            if x >= range[mid] {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        if x < range[lower] || x >= range[lower + 1] {
            return Err(GslError::Sanity);
        }

        Ok(lower)
    }
}
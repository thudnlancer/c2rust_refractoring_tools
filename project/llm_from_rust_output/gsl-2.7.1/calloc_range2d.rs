use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Dom,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProgress,
    NoProgressJ,
    TolF,
    TolX,
    TolG,
    Eof,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug)]
pub struct GslHistogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram2D {
    pub fn calloc_range(
        nx: usize,
        ny: usize,
        xrange: &[f64],
        yrange: &[f64],
    ) -> Result<Self, GslError> {
        if nx == 0 {
            return Err(GslError::Dom);
        }
        if ny == 0 {
            return Err(GslError::Dom);
        }

        for i in 0..nx {
            if xrange[i] >= xrange[i + 1] {
                return Err(GslError::Dom);
            }
        }

        for j in 0..ny {
            if yrange[j] >= yrange[j + 1] {
                return Err(GslError::Dom);
            }
        }

        let xrange_vec = xrange[..=nx].to_vec();
        let yrange_vec = yrange[..=ny].to_vec();
        let bin = vec![0.0; nx * ny];

        Ok(Self {
            nx,
            ny,
            xrange: xrange_vec,
            yrange: yrange_vec,
            bin,
        })
    }
}
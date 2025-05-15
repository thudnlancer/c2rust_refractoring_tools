use thiserror::Error;

#[derive(Debug, Error)]
pub enum GslError {
    #[error("domain error")]
    EDOM,
    #[error("range error")]
    ERANGE,
    #[error("invalid argument")]
    EINVAL,
    #[error("generic failure")]
    EFAILED,
    #[error("factorization failed")]
    EFACTOR,
    #[error("sanity check failed")]
    ESANITY,
    #[error("out of memory")]
    ENOMEM,
    #[error("bad function")]
    EBADFUNC,
    #[error("runaway iteration")]
    ERUNAWAY,
    #[error("maximum iterations reached")]
    EMAXITER,
    #[error("division by zero")]
    EZERODIV,
    #[error("bad tolerance")]
    EBADTOL,
    #[error("tolerance reached")]
    ETOL,
    #[error("underflow")]
    EUNDRFLW,
    #[error("overflow")]
    EOVRFLW,
    #[error("loss of accuracy")]
    ELOSS,
    #[error("rounding error")]
    EROUND,
    #[error("bad length")]
    EBADLEN,
    #[error("matrix not square")]
    ENOTSQR,
    #[error("singularity")]
    ESING,
    #[error("integral or series divergent")]
    EDIVERGE,
    #[error("not supported")]
    EUNSUP,
    #[error("not implemented")]
    EUNIMPL,
    #[error("cache limit exceeded")]
    ECACHE,
    #[error("table limit exceeded")]
    ETABLE,
    #[error("iteration not progressing")]
    ENOPROG,
    #[error("jacobian not progressing")]
    ENOPROGJ,
    #[error("tolerance on f reached")]
    ETOLF,
    #[error("tolerance on x reached")]
    ETOLX,
    #[error("tolerance on gradient reached")]
    ETOLG,
    #[error("end of file")]
    EOF,
}

pub type GslResult<T> = Result<T, GslError>;

#[derive(Debug)]
pub struct Histogram {
    n: usize,
    range: Vec<f64>,
    bins: Vec<f64>,
}

impl Histogram {
    pub fn new(n: usize, range: Vec<f64>) -> GslResult<Self> {
        if range.len() != n + 1 {
            return Err(GslError::EBADLEN);
        }
        Ok(Self {
            n,
            range,
            bins: vec![0.0; n],
        })
    }

    fn find(&self, x: f64) -> GslResult<usize> {
        if x < self.range[0] {
            return Err(GslError::EDOM);
        }
        if x >= self.range[self.n] {
            return Err(GslError::EDOM);
        }

        let u = (x - self.range[0]) / (self.range[self.n] - self.range[0]);
        let i_linear = (u * self.n as f64) as usize;

        if x >= self.range[i_linear] && x < self.range[i_linear + 1] {
            return Ok(i_linear);
        }

        let mut lower = 0;
        let mut upper = self.n;

        while upper - lower > 1 {
            let mid = (upper + lower) / 2;
            if x >= self.range[mid] {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        if x < self.range[lower] || x >= self.range[lower + 1] {
            return Err(GslError::ESANITY);
        }

        Ok(lower)
    }

    pub fn increment(&mut self, x: f64) -> GslResult<()> {
        self.accumulate(x, 1.0)
    }

    pub fn accumulate(&mut self, x: f64, weight: f64) -> GslResult<()> {
        let index = self.find(x)?;
        if index >= self.n {
            return Err(GslError::ESANITY);
        }
        self.bins[index] += weight;
        Ok(())
    }
}
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    #[error("domain error")]
    EDOM,
    #[error("range error")]
    ERANGE,
    #[error("invalid pointer")]
    EFAULT,
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
    #[error("invalid tolerance")]
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
    #[error("divergence")]
    EDIVERGE,
    #[error("unsupported feature")]
    EUNSUP,
    #[error("unimplemented")]
    EUNIMPL,
    #[error("cache limit")]
    ECACHE,
    #[error("table limit")]
    ETABLE,
    #[error("no progress")]
    ENOPROG,
    #[error("no progress in jacobian")]
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

#[derive(Debug)]
pub struct Histogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram {
    pub fn new_with_range(n: usize, range: &[f64]) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::EDOM);
        }

        if range.len() != n + 1 {
            return Err(GslError::EBADLEN);
        }

        for i in 0..n {
            if range[i] >= range[i + 1] {
                return Err(GslError::EDOM);
            }
        }

        Ok(Histogram {
            n,
            range: range.to_vec(),
            bin: vec![0.0; n],
        })
    }
}

pub fn gsl_histogram_calloc_range(n: usize, range: &[f64]) -> Result<Histogram, GslError> {
    Histogram::new_with_range(n, range)
}
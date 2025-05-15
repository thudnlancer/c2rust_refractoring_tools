use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

#[derive(Copy, Clone, Debug)]
pub struct GslIntegrationFixedParams {
    pub alpha: f64,
    pub beta: f64,
    pub a: f64,
    pub b: f64,
    pub zemu: f64,
    pub shft: f64,
    pub slp: f64,
    pub al: f64,
    pub be: f64,
}

pub struct GslIntegrationFixedType {
    pub check: fn(usize, &GslIntegrationFixedParams) -> GslError,
    pub init: fn(usize, &mut [f64], &mut [f64], &mut GslIntegrationFixedParams) -> GslError,
}

fn chebyshev_check(n: usize, params: &GslIntegrationFixedParams) -> GslError {
    if (params.b - params.a).abs() <= 2.2204460492503131e-16 {
        // In real code, you'd want to handle this error properly
        GslError::EDOM
    } else if params.a >= params.b {
        GslError::EDOM
    } else {
        GslError::Success
    }
}

fn chebyshev_init(
    n: usize,
    diag: &mut [f64],
    subdiag: &mut [f64],
    params: &mut GslIntegrationFixedParams,
) -> GslError {
    if diag.len() < n || subdiag.len() < n {
        return GslError::EBADLEN;
    }

    diag[0] = 0.0;
    subdiag[0] = 0.70710678118654752440;

    for i in 1..n {
        diag[i] = 0.0;
        subdiag[i] = 0.5;
    }

    params.zemu = PI;
    params.shft = 0.5 * (params.b + params.a);
    params.slp = 0.5 * (params.b - params.a);
    params.al = -0.5;
    params.be = -0.5;

    GslError::Success
}

pub static GSL_INTEGRATION_FIXED_CHEBYSHEV: GslIntegrationFixedType = GslIntegrationFixedType {
    check: chebyshev_check,
    init: chebyshev_init,
};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Eof,
    Etolg,
    Etolx,
    Etolf,
    Enoprogj,
    Enoprog,
    Etable,
    Ecache,
    Eunimpl,
    Eunsup,
    Ediverge,
    Esing,
    Enotsqr,
    Ebadlen,
    Eround,
    Eloss,
    Eovrflw,
    Eundrflw,
    Etol,
    Ebadtol,
    Ezerodiv,
    Emaxiter,
    Erunaway,
    Ebadfunc,
    Enomem,
    Esanity,
    Efactor,
    Efailed,
    Einval,
    Efault,
    Erange,
    Edom,
    Continue,
    Failure,
    Success,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub type SizeT = usize;

pub fn gsl_poly_dd_init(
    dd: &mut [f64],
    xa: &[f64],
    ya: &[f64],
    size: SizeT,
) -> Result<(), GslError> {
    if dd.len() < size || xa.len() < size || ya.len() < size {
        return Err(GslError::Ebadlen);
    }

    dd[0] = ya[0];
    let mut j = size - 1;
    while j >= 1 {
        dd[j] = (ya[j] - ya[j - 1]) / (xa[j] - xa[j - 1]);
        j = j.wrapping_sub(1);
    }

    for i in 2..size {
        let mut j = size - 1;
        while j >= i {
            dd[j] = (dd[j] - dd[j - 1]) / (xa[j] - xa[j - i]);
            j = j.wrapping_sub(1);
        }
    }

    Ok(())
}

pub fn gsl_poly_dd_taylor(
    c: &mut [f64],
    xp: f64,
    dd: &[f64],
    xa: &[f64],
    size: SizeT,
    w: &mut [f64],
) -> Result<(), GslError> {
    if c.len() < size || dd.len() < size || xa.len() < size || w.len() < size {
        return Err(GslError::Ebadlen);
    }

    c[..size].fill(0.0);
    w[..size].fill(0.0);

    w[size - 1] = 1.0;
    c[0] = dd[0];

    let mut i = size - 1;
    while i > 0 {
        i -= 1;
        w[i] = -w[i + 1] * (xa[size - 2 - i] - xp);

        for j in (i + 1)..(size - 1) {
            w[j] -= w[j + 1] * (xa[size - 2 - i] - xp);
        }

        for j in i..size {
            c[j - i] += w[j] * dd[size - i - 1];
        }
    }

    Ok(())
}

pub fn gsl_poly_dd_hermite_init(
    dd: &mut [f64],
    za: &mut [f64],
    xa: &[f64],
    ya: &[f64],
    dya: &[f64],
    size: SizeT,
) -> Result<(), GslError> {
    let n = 2 * size;
    if dd.len() < n || za.len() < n || xa.len() < size || ya.len() < size || dya.len() < size {
        return Err(GslError::Ebadlen);
    }

    dd[0] = ya[0];
    for j in 0..size {
        za[2 * j] = xa[j];
        za[2 * j + 1] = xa[j];

        if j != 0 {
            dd[2 * j] = (ya[j] - ya[j - 1]) / (xa[j] - xa[j - 1]);
            dd[2 * j - 1] = dya[j - 1];
        }
    }
    dd[n - 1] = dya[size - 1];

    for i in 2..n {
        let mut j = n - 1;
        while j >= i {
            dd[j] = (dd[j] - dd[j - 1]) / (za[j] - za[j - i]);
            j = j.wrapping_sub(1);
        }
    }

    Ok(())
}
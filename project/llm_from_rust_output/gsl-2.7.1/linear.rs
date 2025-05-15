use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

pub fn gsl_fit_linear(
    x: &[f64],
    xstride: usize,
    y: &[f64],
    ystride: usize,
    n: usize,
) -> Result<(f64, f64, f64, f64, f64, f64), GslError> {
    if x.len() < n * xstride || y.len() < n * ystride {
        return Err(GslError::Invalid);
    }

    let mut m_x = 0.0;
    let mut m_y = 0.0;
    let mut m_dx2 = 0.0;
    let mut m_dxdy = 0.0;

    for i in 0..n {
        let xi = x[i * xstride];
        let yi = y[i * ystride];
        m_x += (xi - m_x) / (i as f64 + 1.0);
        m_y += (yi - m_y) / (i as f64 + 1.0);
    }

    for i in 0..n {
        let xi = x[i * xstride];
        let yi = y[i * ystride];
        let dx = xi - m_x;
        let dy = yi - m_y;
        m_dx2 += (dx * dx - m_dx2) / (i as f64 + 1.0);
        m_dxdy += (dx * dy - m_dxdy) / (i as f64 + 1.0);
    }

    let b = m_dxdy / m_dx2;
    let a = m_y - m_x * b;

    let mut d2 = 0.0;
    for i in 0..n {
        let xi = x[i * xstride];
        let yi = y[i * ystride];
        let dx = xi - m_x;
        let dy = yi - m_y;
        let d = dy - b * dx;
        d2 += d * d;
    }

    let s2 = d2 / (n as f64 - 2.0);
    let cov00 = s2 * (1.0 / n as f64) * (1.0 + m_x * m_x / m_dx2);
    let cov11 = s2 / (n as f64 * m_dx2);
    let cov01 = -s2 * m_x / (n as f64 * m_dx2);

    Ok((a, b, cov00, cov01, cov11, d2))
}

pub fn gsl_fit_wlinear(
    x: &[f64],
    xstride: usize,
    w: &[f64],
    wstride: usize,
    y: &[f64],
    ystride: usize,
    n: usize,
) -> Result<(f64, f64, f64, f64, f64, f64), GslError> {
    if x.len() < n * xstride || y.len() < n * ystride || w.len() < n * wstride {
        return Err(GslError::Invalid);
    }

    let mut w_total = 0.0;
    let mut wm_x = 0.0;
    let mut wm_y = 0.0;

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            w_total += wi;
            let xi = x[i * xstride];
            let yi = y[i * ystride];
            wm_x += (xi - wm_x) * (wi / w_total);
            wm_y += (yi - wm_y) * (wi / w_total);
        }
    }

    w_total = 0.0;
    let mut wm_dx2 = 0.0;
    let mut wm_dxdy = 0.0;

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let xi = x[i * xstride];
            let yi = y[i * ystride];
            let dx = xi - wm_x;
            let dy = yi - wm_y;
            w_total += wi;
            wm_dx2 += (dx * dx - wm_dx2) * (wi / w_total);
            wm_dxdy += (dx * dy - wm_dxdy) * (wi / w_total);
        }
    }

    let b = wm_dxdy / wm_dx2;
    let a = wm_y - wm_x * b;

    let cov00 = (1.0 / w_total) * (1.0 + wm_x * wm_x / wm_dx2);
    let cov11 = 1.0 / (w_total * wm_dx2);
    let cov01 = -wm_x / (w_total * wm_dx2);

    let mut d2 = 0.0;
    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let xi = x[i * xstride];
            let yi = y[i * ystride];
            let dx = xi - wm_x;
            let dy = yi - wm_y;
            let d = dy - b * dx;
            d2 += wi * d * d;
        }
    }

    Ok((a, b, cov00, cov01, cov11, d2))
}

pub fn gsl_fit_linear_est(
    x: f64,
    c0: f64,
    c1: f64,
    cov00: f64,
    cov01: f64,
    cov11: f64,
) -> (f64, f64) {
    let y = c0 + c1 * x;
    let y_err = (cov00 + x * (2.0 * cov01 + cov11 * x)).sqrt();
    (y, y_err)
}

pub fn gsl_fit_mul(
    x: &[f64],
    xstride: usize,
    y: &[f64],
    ystride: usize,
    n: usize,
) -> Result<(f64, f64, f64), GslError> {
    if x.len() < n * xstride || y.len() < n * ystride {
        return Err(GslError::Invalid);
    }

    let mut m_x = 0.0;
    let mut m_y = 0.0;
    let mut m_dx2 = 0.0;
    let mut m_dxdy = 0.0;

    for i in 0..n {
        let xi = x[i * xstride];
        let yi = y[i * ystride];
        m_x += (xi - m_x) / (i as f64 + 1.0);
        m_y += (yi - m_y) / (i as f64 + 1.0);
    }

    for i in 0..n {
        let xi = x[i * xstride];
        let yi = y[i * ystride];
        let dx = xi - m_x;
        let dy = yi - m_y;
        m_dx2 += (dx * dx - m_dx2) / (i as f64 + 1.0);
        m_dxdy += (dx * dy - m_dxdy) / (i as f64 + 1.0);
    }

    let b = (m_x * m_y + m_dxdy) / (m_x * m_x + m_dx2);

    let mut d2 = 0.0;
    for i in 0..n {
        let xi = x[i * xstride];
        let yi = y[i * ystride];
        let dx = xi - m_x;
        let dy = yi - m_y;
        let d = m_y - b * m_x + dy - b * dx;
        d2 += d * d;
    }

    let s2 = d2 / (n as f64 - 1.0);
    let cov11 = s2 / (n as f64 * (m_x * m_x + m_dx2));

    Ok((b, cov11, d2))
}

pub fn gsl_fit_wmul(
    x: &[f64],
    xstride: usize,
    w: &[f64],
    wstride: usize,
    y: &[f64],
    ystride: usize,
    n: usize,
) -> Result<(f64, f64, f64), GslError> {
    if x.len() < n * xstride || y.len() < n * ystride || w.len() < n * wstride {
        return Err(GslError::Invalid);
    }

    let mut w_total = 0.0;
    let mut wm_x = 0.0;
    let mut wm_y = 0.0;

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            w_total += wi;
            let xi = x[i * xstride];
            let yi = y[i * ystride];
            wm_x += (xi - wm_x) * (wi / w_total);
            wm_y += (yi - wm_y) * (wi / w_total);
        }
    }

    w_total = 0.0;
    let mut wm_dx2 = 0.0;
    let mut wm_dxdy = 0.0;

    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let xi = x[i * xstride];
            let yi = y[i * ystride];
            let dx = xi - wm_x;
            let dy = yi - wm_y;
            w_total += wi;
            wm_dx2 += (dx * dx - wm_dx2) * (wi / w_total);
            wm_dxdy += (dx * dy - wm_dxdy) * (wi / w_total);
        }
    }

    let b = (wm_x * wm_y + wm_dxdy) / (wm_x * wm_x + wm_dx2);
    let cov11 = 1.0 / (w_total * (wm_x * wm_x + wm_dx2));

    let mut d2 = 0.0;
    for i in 0..n {
        let wi = w[i * wstride];
        if wi > 0.0 {
            let xi = x[i * xstride];
            let yi = y[i * ystride];
            let dx = xi - wm_x;
            let dy = yi - wm_y;
            let d = wm_y - b * wm_x + (dy - b * dx);
            d2 += wi * d * d;
        }
    }

    Ok((b, cov11, d2))
}

pub fn gsl_fit_mul_est(x: f64, c1: f64, cov11: f64) -> (f64, f64) {
    let y = c1 * x;
    let y_err = cov11.sqrt() * x.abs();
    (y, y_err)
}
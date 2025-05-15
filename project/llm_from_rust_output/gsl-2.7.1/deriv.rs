use std::f64::consts::EPSILON;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success,
    Failure,
    Continue,
    Domain,
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

impl GslError {
    pub fn code(&self) -> i32 {
        match self {
            GslError::Success => 0,
            GslError::Failure => -1,
            GslError::Continue => -2,
            GslError::Domain => 1,
            GslError::Range => 2,
            GslError::Fault => 3,
            GslError::Invalid => 4,
            GslError::Failed => 5,
            GslError::Factor => 6,
            GslError::Sanity => 7,
            GslError::NoMem => 8,
            GslError::BadFunc => 9,
            GslError::Runaway => 10,
            GslError::MaxIter => 11,
            GslError::ZeroDiv => 12,
            GslError::BadTol => 13,
            GslError::Tol => 14,
            GslError::Underflow => 15,
            GslError::Overflow => 16,
            GslError::Loss => 17,
            GslError::Round => 18,
            GslError::BadLen => 19,
            GslError::NotSquare => 20,
            GslError::Singular => 21,
            GslError::Diverge => 22,
            GslError::Unsupported => 23,
            GslError::Unimplemented => 24,
            GslError::Cache => 25,
            GslError::Table => 26,
            GslError::NoProgress => 27,
            GslError::NoProgressJ => 28,
            GslError::TolF => 29,
            GslError::TolX => 30,
            GslError::TolG => 31,
            GslError::Eof => 32,
        }
    }
}

pub struct GslFunction<'a> {
    pub function: Box<dyn Fn(f64) -> f64 + 'a>,
}

impl<'a> GslFunction<'a> {
    pub fn new<F: Fn(f64) -> f64 + 'a>(f: F) -> Self {
        GslFunction {
            function: Box::new(f),
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

fn central_deriv(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, f64) {
    let fm1 = f.eval(x - h);
    let fp1 = f.eval(x + h);
    let fmh = f.eval(x - h / 2.0);
    let fph = f.eval(x + h / 2.0);

    let r3 = 0.5 * (fp1 - fm1);
    let r5 = (4.0 / 3.0) * (fph - fmh) - (1.0 / 3.0) * r3;

    let e3 = (fp1.abs() + fm1.abs()) * EPSILON;
    let e5 = 2.0 * (fph.abs() + fmh.abs()) * EPSILON + e3;

    let dy = (r3 / h).abs().max((r5 / h).abs()) * (x.abs() / h) * EPSILON;

    let result = r5 / h;
    let abserr_trunc = (r5 - r3).abs() / h;
    let abserr_round = e5 / h + dy;

    (result, abserr_round, abserr_trunc)
}

pub fn gsl_deriv_central(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> Result<(f64, f64), GslError> {
    let (r0, round, trunc) = central_deriv(f, x, h);
    let mut error = round + trunc;

    if round < trunc && (round > 0.0 && trunc > 0.0) {
        let h_opt = h * (round / (2.0 * trunc)).powf(1.0 / 3.0);
        let (r_opt, round_opt, trunc_opt) = central_deriv(f, x, h_opt);
        let error_opt = round_opt + trunc_opt;

        if error_opt < error && (r_opt - r0).abs() < 4.0 * error {
            return Ok((r_opt, error_opt));
        }
    }

    Ok((r0, error))
}

fn forward_deriv(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> (f64, f64, f64) {
    let f1 = f.eval(x + h / 4.0);
    let f2 = f.eval(x + h / 2.0);
    let f3 = f.eval(x + 3.0 / 4.0 * h);
    let f4 = f.eval(x + h);

    let r2 = 2.0 * (f4 - f2);
    let r4 = (22.0 / 3.0) * (f4 - f3) - (62.0 / 3.0) * (f3 - f2) + (52.0 / 3.0) * (f2 - f1);

    let e4 = 2.0 * 20.67 * (f4.abs() + f3.abs() + f2.abs() + f1.abs()) * EPSILON;
    let dy = (r2 / h).abs().max((r4 / h).abs()) * (x / h).abs() * EPSILON;

    let result = r4 / h;
    let abserr_trunc = (r4 - r2).abs() / h;
    let abserr_round = e4 / h + dy;

    (result, abserr_round, abserr_trunc)
}

pub fn gsl_deriv_forward(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> Result<(f64, f64), GslError> {
    let (r0, round, trunc) = forward_deriv(f, x, h);
    let mut error = round + trunc;

    if round < trunc && (round > 0.0 && trunc > 0.0) {
        let h_opt = h * (round / trunc).powf(1.0 / 2.0);
        let (r_opt, round_opt, trunc_opt) = forward_deriv(f, x, h_opt);
        let error_opt = round_opt + trunc_opt;

        if error_opt < error && (r_opt - r0).abs() < 4.0 * error {
            return Ok((r_opt, error_opt));
        }
    }

    Ok((r0, error))
}

pub fn gsl_deriv_backward(
    f: &GslFunction,
    x: f64,
    h: f64,
) -> Result<(f64, f64), GslError> {
    gsl_deriv_forward(f, x, -h)
}
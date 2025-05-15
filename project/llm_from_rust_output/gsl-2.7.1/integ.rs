use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

#[derive(Debug, Clone)]
pub struct ChebSeries {
    pub c: Vec<f64>,
    pub order: usize,
    pub a: f64,
    pub b: f64,
    pub order_sp: usize,
    pub f: Vec<f64>,
}

pub fn cheb_calc_integ(integ: &mut ChebSeries, f: &ChebSeries) -> Result<(), GslError> {
    let n = f.order + 1;
    let con = 0.25 * (f.b - f.a);

    if integ.order != f.order {
        return Err(GslError::Nomem);
    }

    integ.a = f.a;
    integ.b = f.b;

    match n {
        1 => {
            integ.c[0] = 0.0;
        }
        2 => {
            integ.c[1] = con * f.c[0];
            integ.c[0] = 2.0 * integ.c[1];
        }
        _ => {
            let mut sum = 0.0;
            let mut fac = 1.0;

            for i in 1..=n - 2 {
                integ.c[i] = con * (f.c[i - 1] - f.c[i + 1]) / i as f64;
                sum += fac * integ.c[i];
                fac = -fac;
            }

            integ.c[n - 1] = con * f.c[n - 2] / (n as f64 - 1.0);
            sum += fac * integ.c[n - 1];
            integ.c[0] = 2.0 * sum;
        }
    }

    Ok(())
}
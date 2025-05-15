use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GslError {
    NoMemory,
    InvalidOrder,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GslError::NoMemory => write!(f, "order of chebyshev series must be equal"),
            GslError::InvalidOrder => write!(f, "invalid order"),
        }
    }
}

impl Error for GslError {}

pub struct ChebSeries {
    pub order: usize,
    pub a: f64,
    pub b: f64,
    pub c: Vec<f64>,
}

pub fn gsl_cheb_calc_integ(integ: &mut ChebSeries, f: &ChebSeries) -> Result<(), GslError> {
    let n = f.order + 1;
    let con = 0.25 * (f.b - f.a);

    if integ.order != f.order {
        return Err(GslError::NoMemory);
    }

    integ.a = f.a;
    integ.b = f.b;

    if n == 1 {
        integ.c[0] = 0.0;
    } else if n == 2 {
        integ.c[1] = con * f.c[0];
        integ.c[0] = 2.0 * integ.c[1];
    } else {
        let mut sum = 0.0;
        let mut fac = 1.0;
        
        for i in 1..=n-2 {
            integ.c[i] = con * (f.c[i-1] - f.c[i+1]) / (i as f64);
            sum += fac * integ.c[i];
            fac = -fac;
        }
        
        integ.c[n-1] = con * f.c[n-2] / ((n-1) as f64);
        sum += fac * integ.c[n-1];
        integ.c[0] = 2.0 * sum;
    }

    Ok(())
}
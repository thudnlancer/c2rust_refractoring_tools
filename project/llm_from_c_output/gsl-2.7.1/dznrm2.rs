use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

#[derive(Debug)]
struct Nrm2Error {
    details: String,
}

impl Nrm2Error {
    fn new(msg: &str) -> Nrm2Error {
        Nrm2Error {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for Nrm2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for Nrm2Error {
    fn description(&self) -> &str {
        &self.details
    }
}

fn cblas_dznrm2(n: i32, x: &[Complex], inc_x: i32) -> Result<f64, Nrm2Error> {
    if n < 0 {
        return Err(Nrm2Error::new("n must be non-negative"));
    }
    if inc_x <= 0 {
        return Err(Nrm2Error::new("inc_x must be positive"));
    }
    if x.is_empty() {
        return Ok(0.0);
    }

    let mut scale = 0.0;
    let mut ssq = 1.0;

    for i in (0..n as usize).step_by(inc_x as usize) {
        if i >= x.len() {
            break;
        }
        let abs_re = x[i].re.abs();
        if abs_re != 0.0 {
            let temp = scale / abs_re;
            ssq = 1.0 + ssq * (temp * temp);
            scale = abs_re;
        } else {
            let temp = abs_re / scale;
            ssq += temp * temp;
        }

        let abs_im = x[i].im.abs();
        if abs_im != 0.0 {
            let temp = scale / abs_im;
            ssq = 1.0 + ssq * (temp * temp);
            scale = abs_im;
        } else {
            let temp = abs_im / scale;
            ssq += temp * temp;
        }
    }

    Ok(scale * ssq.sqrt())
}
/* integration/rational.rs

 * This code is based on IQPACK, specifically the LGPL
 * implementation found in HERMITE_RULE:
 * https://people.sc.fsu.edu/~jburkardt/c_src/hermite_rule/hermite_rule.html
 */

use std::f64;
use std::f64::consts;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IntegrationError {
    message: String,
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IntegrationError {}

impl IntegrationError {
    fn new(msg: &str) -> IntegrationError {
        IntegrationError {
            message: msg.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixedParams {
    pub a: f64,
    pub b: f64,
    pub alpha: f64,
    pub beta: f64,
    pub zemu: f64,
    pub shft: f64,
    pub slp: f64,
    pub al: f64,
    pub be: f64,
}

pub struct FixedType {
    pub check: fn(usize, &FixedParams) -> Result<(), IntegrationError>,
    pub init: fn(usize, &mut [f64], &mut [f64], &mut FixedParams) -> Result<(), IntegrationError>,
}

fn rational_check(n: usize, params: &FixedParams) -> Result<(), IntegrationError> {
    if (params.b - params.a).abs() <= f64::EPSILON {
        Err(IntegrationError::new("|b - a| too small"))
    } else if params.alpha <= -1.0 {
        Err(IntegrationError::new("alpha must be > -1"))
    } else if params.beta >= 0.0 
        || params.alpha + params.beta + 2.0 * (n as f64) >= 0.0 
        || 0.0 >= params.alpha + 2.0 * (n as f64) {
        Err(IntegrationError::new("beta < alpha + beta + 2n < 0 is required"))
    } else if params.a + params.b <= 0.0 {
        Err(IntegrationError::new("a + b <= 0 is not allowed"))
    } else {
        Ok(())
    }
}

fn gamma(x: f64) -> f64 {
    // Simple approximation of gamma function for positive arguments
    // For production code, consider using a proper gamma function implementation
    if x <= 0.0 {
        f64::NAN
    } else if x == 1.0 {
        1.0
    } else if x == 0.5 {
        consts::SQRT_PI
    } else {
        (x - 1.0) * gamma(x - 1.0)
    }
}

fn rational_init(
    n: usize,
    diag: &mut [f64],
    subdiag: &mut [f64],
    params: &mut FixedParams,
) -> Result<(), IntegrationError> {
    let absum = params.beta + params.alpha;
    let a1 = params.alpha + 1.0;
    let aba1 = absum * a1;
    let mut ab2i = absum + 2.0;

    // construct the diagonal and subdiagonal elements of Jacobi matrix
    diag[0] = -a1 / (absum + 2.0);
    subdiag[0] = (-diag[0] * (params.beta + 1.0) / ((absum + 2.0) * (absum + 3.0)))).sqrt();

    for i in 1..n-1 {
        ab2i += 2.0;
        diag[i] = (-aba1 - 2.0 * (i as f64) * (absum + (i as f64) + 1.0)) / (ab2i * (ab2i - 2.0));
        subdiag[i] = (
            ((i + 1) as f64) * (params.alpha + (i + 1) as f64) / (ab2i - 1.0) *
            (params.beta + (i + 1) as f64) / (ab2i * ab2i) *
            (absum + (i + 1) as f64) / (ab2i + 1.0)
        ).sqrt();
    }

    diag[n-1] = (-aba1 - 2.0 * ((n - 1) as f64) * (absum + (n as f64))) / 
                ((absum + 2.0 * (n as f64)) * (absum + 2.0 * (n as f64) - 2.0)));
    subdiag[n-1] = 0.0;

    params.zemu = gamma(params.alpha + 1.0) * gamma(-absum - 1.0) / gamma(-params.beta);
    params.shft = params.a;
    params.slp = params.b + params.a;
    params.al = params.alpha;
    params.be = params.beta;

    Ok(())
}

pub static RATIONAL_TYPE: FixedType = FixedType {
    check: rational_check,
    init: rational_init,
};
use std::f64;
use std::ptr;
use std::mem;

#[derive(Debug)]
pub struct IntegrationRombergWorkspace {
    n: usize,
    work1: Vec<f64>,
    work2: Vec<f64>,
}

pub type GslResult<T> = Result<T, GslError>;

#[derive(Debug)]
pub enum GslError {
    DomainError(String),
    NoMemory(String),
    MaxIteration,
}

pub trait GslFunction {
    fn evaluate(&self, x: f64) -> f64;
}

impl IntegrationRombergWorkspace {
    pub fn new(n: usize) -> GslResult<Self> {
        if n < 1 {
            return Err(GslError::DomainError(
                "workspace size n must be at least 1".to_string(),
            ));
        }

        let actual_n = n.min(30);
        let work1 = vec![0.0; actual_n];
        let work2 = vec![0.0; actual_n];

        Ok(Self {
            n: actual_n,
            work1,
            work2,
        })
    }
}

pub fn romberg_integrate<F: GslFunction>(
    f: &F,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    workspace: &mut IntegrationRombergWorkspace,
) -> GslResult<(f64, usize)> {
    if epsabs < 0.0 {
        return Err(GslError::DomainError("epsabs must be non-negative".to_string()));
    } else if epsrel < 0.0 {
        return Err(GslError::DomainError("epsrel must be non-negative".to_string()));
    }

    let n = workspace.n;
    let mut rp = &mut workspace.work1[..n]; // previous row
    let mut rc = &mut workspace.work2[..n]; // current row
    let mut h = 0.5 * (b - a); // step size
    let mut neval = 2;

    // R(0,0)
    rp[0] = h * (f.evaluate(a) + f.evaluate(b));

    for i in 1..n {
        let mut sum = 0.0;
        let two_i = 1 << i; // 2^i

        for j in (1..two_i).step_by(2) {
            sum += f.evaluate(a + j as f64 * h);
            neval += 1;
        }

        // R(i,0)
        rc[0] = sum * h + 0.5 * rp[0];

        let mut four_j = 4.0;
        for j in 1..=i {
            rc[j] = (four_j * rc[j - 1] - rp[j - 1]) / (four_j - 1.0);
            four_j *= 4.0;
        }

        // Check for convergence
        let err = (rc[i] - rp[i - 1]).abs();
        if err < epsabs || err < epsrel * rc[i].abs() {
            return Ok((rc[i], neval));
        }

        // Swap Rp and Rc
        mem::swap(&mut rp, &mut rc);
        h *= 0.5;
    }

    // Did not converge - return best guess
    Err(GslError::MaxIteration)
}
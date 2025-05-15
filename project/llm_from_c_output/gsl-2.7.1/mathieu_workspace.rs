use std::f64;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MathieuError {
    message: String,
    code: i32,
}

impl fmt::Display for MathieuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (code: {})", self.message, self.code)
    }
}

impl Error for MathieuError {}

impl MathieuError {
    fn new(message: &str, code: i32) -> Self {
        MathieuError {
            message: message.to_string(),
            code,
        }
    }
}

struct MathieuWorkspace {
    size: usize,
    even_order: usize,
    odd_order: usize,
    extra_values: usize,
    aa: Vec<f64>,
    bb: Vec<f64>,
    dd: Vec<f64>,
    ee: Vec<f64>,
    tt: Vec<f64>,
    e2: Vec<f64>,
    zz: Vec<f64>,
    eval: Vec<f64>,
    evec: Vec<Vec<f64>>,
    // Note: In Rust, we'd typically use nalgebra or similar for matrices,
    // but for direct translation we'll use Vec<Vec<f64>>
}

impl MathieuWorkspace {
    fn new(nn: usize, qq: f64) -> Result<Self, MathieuError> {
        if nn == 0 {
            return Err(MathieuError::new(
                "matrix dimension must be positive integer",
                gsl_sf_EINVAL,
            ));
        }

        let mut even_order = nn / 2 + 1;
        let mut odd_order = (nn + 1) / 2;

        // Compute extra values
        let mut extra_values = (2.1 * qq.abs().powf(0.37)) as usize + 9;
        extra_values += 20; // additional fudge

        // Extend matrices to ensure accuracy
        even_order += extra_values;
        odd_order += extra_values;

        // Allocate vectors
        let aa = vec![0.0; nn + 1];
        let bb = vec![0.0; nn + 1];
        let dd = vec![0.0; even_order];
        let ee = vec![0.0; even_order];
        let tt = vec![0.0; 3 * even_order];
        let e2 = vec![0.0; even_order];
        let zz = vec![0.0; even_order * even_order];
        let eval = vec![0.0; even_order];
        let evec = vec![vec![0.0; even_order]; even_order];

        Ok(MathieuWorkspace {
            size: nn,
            even_order,
            odd_order,
            extra_values,
            aa,
            bb,
            dd,
            ee,
            tt,
            e2,
            zz,
            eval,
            evec,
        })
    }
}

// Constants matching GSL error codes
const gsl_sf_EINVAL: i32 = 22;
const gsl_sf_ENOMEM: i32 = 12;

// Note: In a real Rust implementation, we'd use proper error types and
// probably a matrix library like nalgebra instead of Vec<Vec<f64>>.
// The free functionality is handled by Rust's Drop trait automatically.
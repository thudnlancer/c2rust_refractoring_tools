use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IntegrationError {
    message: String,
    code: i32,
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (code: {})", self.message, self.code)
    }
}

impl Error for IntegrationError {}

const GSL_EDOM: i32 = 1;
const GSL_ENOMEM: i32 = 2;

pub struct IntegrationWorkspace {
    alist: Vec<f64>,
    blist: Vec<f64>,
    rlist: Vec<f64>,
    elist: Vec<f64>,
    order: Vec<usize>,
    level: Vec<usize>,
    size: usize,
    limit: usize,
    maximum_level: usize,
}

impl IntegrationWorkspace {
    pub fn new(n: usize) -> Result<Self, IntegrationError> {
        if n == 0 {
            return Err(IntegrationError {
                message: "workspace length n must be positive integer".to_string(),
                code: GSL_EDOM,
            });
        }

        Ok(IntegrationWorkspace {
            alist: vec![0.0; n],
            blist: vec![0.0; n],
            rlist: vec![0.0; n],
            elist: vec![0.0; n],
            order: vec![0; n],
            level: vec![0; n],
            size: 0,
            limit: n,
            maximum_level: 0,
        })
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for IntegrationWorkspace {
    fn drop(&mut self) {
        // All memory is automatically managed by Vec
    }
}
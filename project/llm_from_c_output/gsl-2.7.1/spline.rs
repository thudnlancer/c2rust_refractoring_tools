use std::error::Error;
use std::fmt;
use std::ptr;
use std::mem;

#[derive(Debug)]
pub enum SplineError {
    InvalidSize,
    NoMemory,
    InterpError,
}

impl fmt::Display for SplineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SplineError::InvalidSize => write!(f, "data must match size of spline object"),
            SplineError::NoMemory => write!(f, "failed to allocate space"),
            SplineError::InterpError => write!(f, "interpolation error"),
        }
    }
}

impl Error for SplineError {}

pub struct Spline {
    interp: *mut gsl_interp,
    x: Vec<f64>,
    y: Vec<f64>,
    size: usize,
}

impl Spline {
    pub fn new(interp_type: *const gsl_interp_type, size: usize) -> Result<Self, SplineError> {
        let interp = unsafe { gsl_interp_alloc(interp_type, size) };
        if interp.is_null() {
            return Err(SplineError::NoMemory);
        }

        Ok(Self {
            interp,
            x: Vec::with_capacity(size),
            y: Vec::with_capacity(size),
            size,
        })
    }

    pub fn init(&mut self, x_array: &[f64], y_array: &[f64]) -> Result<(), SplineError> {
        if x_array.len() != self.size || y_array.len() != self.size {
            return Err(SplineError::InvalidSize);
        }

        self.x.clear();
        self.x.extend_from_slice(x_array);
        self.y.clear();
        self.y.extend_from_slice(y_array);

        let status = unsafe { gsl_interp_init(self.interp, x_array.as_ptr(), y_array.as_ptr(), self.size) };
        if status != 0 {
            return Err(SplineError::InterpError);
        }

        Ok(())
    }

    pub fn name(&self) -> &str {
        unsafe {
            let name_ptr = gsl_interp_name(self.interp);
            std::ffi::CStr::from_ptr(name_ptr).to_str().unwrap_or_default()
        }
    }

    pub fn min_size(&self) -> usize {
        unsafe { gsl_interp_min_size(self.interp) as usize }
    }

    pub fn eval(&self, x: f64, acc: *mut gsl_interp_accel) -> Result<f64, SplineError> {
        let mut result = 0.0;
        let status = unsafe {
            gsl_interp_eval_e(
                self.interp,
                self.x.as_ptr(),
                self.y.as_ptr(),
                x,
                acc,
                &mut result,
            )
        };
        if status != 0 {
            return Err(SplineError::InterpError);
        }
        Ok(result)
    }

    pub fn eval_deriv(&self, x: f64, acc: *mut gsl_interp_accel) -> Result<f64, SplineError> {
        let mut result = 0.0;
        let status = unsafe {
            gsl_interp_eval_deriv_e(
                self.interp,
                self.x.as_ptr(),
                self.y.as_ptr(),
                x,
                acc,
                &mut result,
            )
        };
        if status != 0 {
            return Err(SplineError::InterpError);
        }
        Ok(result)
    }

    pub fn eval_deriv2(&self, x: f64, acc: *mut gsl_interp_accel) -> Result<f64, SplineError> {
        let mut result = 0.0;
        let status = unsafe {
            gsl_interp_eval_deriv2_e(
                self.interp,
                self.x.as_ptr(),
                self.y.as_ptr(),
                x,
                acc,
                &mut result,
            )
        };
        if status != 0 {
            return Err(SplineError::InterpError);
        }
        Ok(result)
    }

    pub fn eval_integ(&self, a: f64, b: f64, acc: *mut gsl_interp_accel) -> Result<f64, SplineError> {
        let mut result = 0.0;
        let status = unsafe {
            gsl_interp_eval_integ_e(
                self.interp,
                self.x.as_ptr(),
                self.y.as_ptr(),
                a,
                b,
                acc,
                &mut result,
            )
        };
        if status != 0 {
            return Err(SplineError::InterpError);
        }
        Ok(result)
    }
}

impl Drop for Spline {
    fn drop(&mut self) {
        unsafe {
            gsl_interp_free(self.interp);
        }
    }
}
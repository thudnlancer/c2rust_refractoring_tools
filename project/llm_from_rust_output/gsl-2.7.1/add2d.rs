use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_ulong};

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

#[derive(Debug, Clone)]
pub struct GslHistogram2D {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

impl GslHistogram2D {
    pub fn increment(&mut self, x: f64, y: f64) -> Result<(), GslError> {
        self.accumulate(x, y, 1.0)
    }

    pub fn accumulate(&mut self, x: f64, y: f64, weight: f64) -> Result<(), GslError> {
        let (i, j) = self.find_2d(x, y)?;
        
        if i >= self.nx {
            gsl_error(
                "index lies outside valid range of 0 .. nx - 1",
                "add2d.c",
                54,
                GslError::ESANITY,
            );
            return Err(GslError::ESANITY);
        }
        
        if j >= self.ny {
            gsl_error(
                "index lies outside valid range of 0 .. ny - 1",
                "add2d.c",
                60,
                GslError::ESANITY,
            );
            return Err(GslError::ESANITY);
        }

        let index = i * self.ny + j;
        self.bin[index as usize] += weight;
        Ok(())
    }

    fn find_2d(&self, x: f64, y: f64) -> Result<(size_t, size_t), GslError> {
        let i = self.find(self.nx, &self.xrange, x)?;
        let j = self.find(self.ny, &self.yrange, y)?;
        Ok((i, j))
    }

    fn find(&self, n: size_t, range: &[f64], x: f64) -> Result<size_t, GslError> {
        if x < range[0] {
            return Err(GslError::EDOM);
        }
        if x >= range[n as usize] {
            return Err(GslError::EDOM);
        }

        let u = (x - range[0]) / (range[n as usize] - range[0]);
        let i_linear = (u * n as f64) as size_t;

        if x >= range[i_linear as usize] && x < range[(i_linear + 1) as usize] {
            return Ok(i_linear);
        }

        let mut upper = n;
        let mut lower = 0;
        while upper - lower > 1 {
            let mid = (upper + lower) / 2;
            if x >= range[mid as usize] {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        if x < range[lower as usize] || x >= range[(lower + 1) as usize] {
            gsl_error(
                "x not found in range",
                "./find.c",
                81,
                GslError::ESANITY,
            );
            return Err(GslError::ESANITY);
        }

        Ok(lower)
    }
}

fn gsl_error(reason: &str, file: &str, line: c_int, gsl_errno: GslError) {
    unsafe {
        let reason = CStr::from_bytes_with_nul_unchecked(reason.as_bytes());
        let file = CStr::from_bytes_with_nul_unchecked(file.as_bytes());
        extern "C" {
            fn gsl_error(
                reason: *const c_char,
                file: *const c_char,
                line: c_int,
                gsl_errno: c_int,
            );
        }
        gsl_error(
            reason.as_ptr(),
            file.as_ptr(),
            line,
            gsl_errno as c_int,
        );
    }
}
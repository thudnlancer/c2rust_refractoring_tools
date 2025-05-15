use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_ulong};

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDom = 1,
    ERange = 2,
    EDefault = 3,
    EInval = 4,
    EFailed = 5,
    EFactor = 6,
    ESanity = 7,
    ENoMem = 8,
    EBadFunc = 9,
    ERunaway = 10,
    EMaxIter = 11,
    EZeroDiv = 12,
    EBadTol = 13,
    ETol = 14,
    EUnderflow = 15,
    EOverflow = 16,
    ELoss = 17,
    ERound = 18,
    EBadLen = 19,
    ENotSqr = 20,
    ESing = 21,
    EDiverge = 22,
    EUnsup = 23,
    EUnimpl = 24,
    ECache = 25,
    ETable = 26,
    ENoProg = 27,
    ENoProgJ = 28,
    ETolF = 29,
    ETolX = 30,
    ETolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslHistogram {
    n: size_t,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram {
    pub fn new(n: size_t) -> Self {
        Self {
            n,
            range: vec![0.0; (n + 1) as usize],
            bin: vec![0.0; n as usize],
        }
    }

    pub fn get(&self, i: size_t) -> Result<f64, GslError> {
        if i >= self.n {
            gsl_error(
                "index lies outside valid range of 0 .. n - 1",
                "get.c",
                34,
                GslError::EDom,
            );
            return Err(GslError::EDom);
        }
        Ok(self.bin[i as usize])
    }

    pub fn get_range(&self, i: size_t) -> Result<(f64, f64), GslError> {
        if i >= self.n {
            gsl_error(
                "index lies outside valid range of 0 .. n - 1",
                "get.c",
                48,
                GslError::EDom,
            );
            return Err(GslError::EDom);
        }
        Ok((self.range[i as usize], self.range[(i + 1) as usize]))
    }

    pub fn find(&self, x: f64) -> Result<size_t, GslError> {
        let mut i = 0;
        let status = self.find_impl(x, &mut i);
        if status != GslError::Success {
            gsl_error(
                "x not found in range of h",
                "get.c",
                65,
                GslError::EDom,
            );
            return Err(GslError::EDom);
        }
        Ok(i)
    }

    fn find_impl(&self, x: f64, i: &mut size_t) -> GslError {
        if x < self.range[0] {
            return GslError::EDom;
        }
        if x >= self.range[self.n as usize] {
            return GslError::EDom;
        }

        let u = (x - self.range[0]) / (self.range[self.n as usize] - self.range[0]);
        let i_linear = (u * self.n as f64) as size_t;

        if x >= self.range[i_linear as usize] && x < self.range[(i_linear + 1) as usize] {
            *i = i_linear;
            return GslError::Success;
        }

        let mut upper = self.n;
        let mut lower = 0;

        while upper - lower > 1 {
            let mid = (upper + lower) / 2;
            if x >= self.range[mid as usize] {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        *i = lower;

        if x < self.range[lower as usize] || x >= self.range[(lower + 1) as usize] {
            gsl_error(
                "x not found in range",
                "./find.c",
                81,
                GslError::ESanity,
            );
            return GslError::ESanity;
        }

        GslError::Success
    }
}

fn gsl_error(reason: &str, file: &str, line: i32, gsl_errno: GslError) {
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
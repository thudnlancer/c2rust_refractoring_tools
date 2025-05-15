use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
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
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

fn gsl_error(reason: &str, file: &str, line: i32, errno: GslError) {
    eprintln!("GSL Error: {} ({}:{}) - {:?}", reason, file, line, errno);
}

#[derive(Debug)]
pub struct GslMultiset {
    n: usize,
    k: usize,
    data: Option<Box<[usize]>>,
}

impl GslMultiset {
    pub fn new(n: usize, k: usize) -> Result<Self, GslError> {
        if n == 0 {
            gsl_error(
                "multiset parameter n must be positive integer",
                "init.c",
                37,
                GslError::Dom,
            );
            return Err(GslError::Dom);
        }

        let data = if k > 0 {
            let layout = Layout::array::<usize>(k).map_err(|_| GslError::Nomem)?;
            let ptr = unsafe { alloc(layout) as *mut usize };
            if ptr.is_null() {
                gsl_error(
                    "failed to allocate space for multiset data",
                    "init.c",
                    56,
                    GslError::Nomem,
                );
                return Err(GslError::Nomem);
            }
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, k)) })
        } else {
            None
        };

        Ok(Self { n, k, data })
    }

    pub fn calloc(n: usize, k: usize) -> Result<Self, GslError> {
        let mut multiset = Self::new(n, k)?;
        if let Some(ref mut data) = multiset.data {
            for item in data.iter_mut() {
                *item = 0;
            }
        }
        Ok(multiset)
    }

    pub fn init_first(&mut self) {
        if let Some(ref mut data) = self.data {
            for item in data.iter_mut() {
                *item = 0;
            }
        }
    }

    pub fn init_last(&mut self) {
        if let Some(ref mut data) = self.data {
            let n = self.n;
            for item in data.iter_mut() {
                *item = n - 1;
            }
        }
    }
}

impl Drop for GslMultiset {
    fn drop(&mut self) {
        // Box will automatically deallocate when dropped
    }
}
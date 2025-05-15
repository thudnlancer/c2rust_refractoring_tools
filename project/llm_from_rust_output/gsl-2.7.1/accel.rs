use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct GslInterpAccel {
    cache: usize,
    miss_count: usize,
    hit_count: usize,
}

impl GslInterpAccel {
    pub fn new() -> Result<Box<Self>, GslError> {
        let layout = Layout::new::<Self>();
        unsafe {
            let ptr = alloc(layout) as *mut Self;
            if ptr.is_null() {
                return Err(GslError::Nomem);
            }
            let mut boxed = Box::from_raw(ptr);
            boxed.cache = 0;
            boxed.hit_count = 0;
            boxed.miss_count = 0;
            Ok(boxed)
        }
    }

    pub fn reset(&mut self) -> GslError {
        self.cache = 0;
        self.hit_count = 0;
        self.miss_count = 0;
        GslError::Success
    }
}

impl Drop for GslInterpAccel {
    fn drop(&mut self) {
        let layout = Layout::new::<Self>();
        unsafe {
            dealloc(self as *mut _ as *mut u8, layout);
        }
    }
}
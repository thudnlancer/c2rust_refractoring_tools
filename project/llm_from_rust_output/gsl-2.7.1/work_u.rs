use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

pub type SizeT = usize;

#[derive(Debug)]
pub struct GslSumLevinUWorkspace {
    size: SizeT,
    i: SizeT,
    terms_used: SizeT,
    sum_plain: f64,
    q_num: NonNull<f64>,
    q_den: NonNull<f64>,
    dq_num: NonNull<f64>,
    dq_den: NonNull<f64>,
    dsum: NonNull<f64>,
}

impl GslSumLevinUWorkspace {
    pub fn new(n: SizeT) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::Dom);
        }

        let layout = Layout::new::<Self>();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(GslError::Nomem);
        }

        let q_num = unsafe {
            let layout = Layout::array::<f64>(n).map_err(|_| GslError::Nomem)?;
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(GslError::Nomem);
            }
            NonNull::new(ptr as *mut f64).unwrap()
        };

        let q_den = unsafe {
            let layout = Layout::array::<f64>(n).map_err(|_| GslError::Nomem)?;
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(GslError::Nomem);
            }
            NonNull::new(ptr as *mut f64).unwrap()
        };

        let dq_num = unsafe {
            let layout = Layout::array::<f64>(n * n).map_err(|_| GslError::Nomem)?;
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(GslError::Nomem);
            }
            NonNull::new(ptr as *mut f64).unwrap()
        };

        let dq_den = unsafe {
            let layout = Layout::array::<f64>(n * n).map_err(|_| GslError::Nomem)?;
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(GslError::Nomem);
            }
            NonNull::new(ptr as *mut f64).unwrap()
        };

        let dsum = unsafe {
            let layout = Layout::array::<f64>(n).map_err(|_| GslError::Nomem)?;
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err(GslError::Nomem);
            }
            NonNull::new(ptr as *mut f64).unwrap()
        };

        Ok(Self {
            size: n,
            i: 0,
            terms_used: 0,
            sum_plain: 0.0,
            q_num,
            q_den,
            dq_num,
            dq_den,
            dsum,
        })
    }
}

impl Drop for GslSumLevinUWorkspace {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<f64>(self.size).unwrap();
            dealloc(self.q_num.as_ptr() as *mut u8, layout);
            
            let layout = Layout::array::<f64>(self.size).unwrap();
            dealloc(self.q_den.as_ptr() as *mut u8, layout);
            
            let layout = Layout::array::<f64>(self.size * self.size).unwrap();
            dealloc(self.dq_num.as_ptr() as *mut u8, layout);
            
            let layout = Layout::array::<f64>(self.size * self.size).unwrap();
            dealloc(self.dq_den.as_ptr() as *mut u8, layout);
            
            let layout = Layout::array::<f64>(self.size).unwrap();
            dealloc(self.dsum.as_ptr() as *mut u8, layout);
        }
    }
}
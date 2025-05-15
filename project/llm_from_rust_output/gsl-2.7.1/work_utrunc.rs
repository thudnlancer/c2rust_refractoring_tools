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
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Debug)]
pub struct GslSumLevinUtruncWorkspace {
    size: usize,
    i: usize,
    terms_used: usize,
    sum_plain: f64,
    q_num: NonNull<f64>,
    q_den: NonNull<f64>,
    dsum: NonNull<f64>,
}

impl GslSumLevinUtruncWorkspace {
    pub fn new(n: usize) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::Dom);
        }

        let layout = Layout::new::<Self>();
        let q_num_layout = Layout::array::<f64>(n).map_err(|_| GslError::NoMem)?;
        let q_den_layout = Layout::array::<f64>(n).map_err(|_| GslError::NoMem)?;
        let dsum_layout = Layout::array::<f64>(n).map_err(|_| GslError::NoMem)?;

        unsafe {
            let q_num_ptr = alloc(q_num_layout) as *mut f64;
            if q_num_ptr.is_null() {
                return Err(GslError::NoMem);
            }
            let q_num = NonNull::new(q_num_ptr).ok_or(GslError::NoMem)?;

            let q_den_ptr = alloc(q_den_layout) as *mut f64;
            if q_den_ptr.is_null() {
                dealloc(q_num_ptr as *mut u8, q_num_layout);
                return Err(GslError::NoMem);
            }
            let q_den = NonNull::new(q_den_ptr).ok_or(GslError::NoMem)?;

            let dsum_ptr = alloc(dsum_layout) as *mut f64;
            if dsum_ptr.is_null() {
                dealloc(q_num_ptr as *mut u8, q_num_layout);
                dealloc(q_den_ptr as *mut u8, q_den_layout);
                return Err(GslError::NoMem);
            }
            let dsum = NonNull::new(dsum_ptr).ok_or(GslError::NoMem)?;

            Ok(Self {
                size: n,
                i: 0,
                terms_used: 0,
                sum_plain: 0.0,
                q_num,
                q_den,
                dsum,
            })
        }
    }
}

impl Drop for GslSumLevinUtruncWorkspace {
    fn drop(&mut self) {
        unsafe {
            let q_num_layout = Layout::array::<f64>(self.size).unwrap();
            dealloc(self.q_num.as_ptr() as *mut u8, q_num_layout);

            let q_den_layout = Layout::array::<f64>(self.size).unwrap();
            dealloc(self.q_den.as_ptr() as *mut u8, q_den_layout);

            let dsum_layout = Layout::array::<f64>(self.size).unwrap();
            dealloc(self.dsum.as_ptr() as *mut u8, dsum_layout);
        }
    }
}
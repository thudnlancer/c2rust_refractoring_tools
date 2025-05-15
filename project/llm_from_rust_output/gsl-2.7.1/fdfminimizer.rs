use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GslBlock {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GslVector {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GslMultiminFunctionFdf {
    pub f: Option<unsafe extern "C" fn(*const GslVector, *mut c_void) -> c_double>,
    pub df: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut GslVector)>,
    pub fdf: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut c_double, *mut GslVector)>,
    pub n: usize,
    pub params: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GslMultiminFdfminimizerType {
    pub name: *const c_char,
    pub size: usize,
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, usize) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut c_void, *mut GslMultiminFunctionFdf, *const GslVector, *mut c_double, *mut GslVector, c_double, c_double) -> c_int>,
    pub iterate: Option<unsafe extern "C" fn(*mut c_void, *mut GslMultiminFunctionFdf, *mut GslVector, *mut c_double, *mut GslVector, *mut GslVector) -> c_int>,
    pub restart: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GslMultiminFdfminimizer {
    pub type_: *const GslMultiminFdfminimizerType,
    pub fdf: *mut GslMultiminFunctionFdf,
    pub f: c_double,
    pub x: *mut GslVector,
    pub gradient: *mut GslVector,
    pub dx: *mut GslVector,
    pub state: *mut c_void,
}

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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

pub struct MultiminFdfminimizer {
    inner: *mut GslMultiminFdfminimizer,
}

impl MultiminFdfminimizer {
    pub fn new(t: *const GslMultiminFdfminimizerType, n: usize) -> Option<Self> {
        unsafe {
            let s = libc::malloc(std::mem::size_of::<GslMultiminFdfminimizer>()) as *mut GslMultiminFdfminimizer;
            if s.is_null() {
                return None;
            }

            (*s).type_ = t;
            (*s).x = gsl_vector_calloc(n);
            if (*s).x.is_null() {
                libc::free(s as *mut c_void);
                return None;
            }

            (*s).gradient = gsl_vector_calloc(n);
            if (*s).gradient.is_null() {
                gsl_vector_free((*s).x);
                libc::free(s as *mut c_void);
                return None;
            }

            (*s).dx = gsl_vector_calloc(n);
            if (*s).dx.is_null() {
                gsl_vector_free((*s).x);
                gsl_vector_free((*s).gradient);
                libc::free(s as *mut c_void);
                return None;
            }

            (*s).state = libc::malloc((*t).size);
            if (*s).state.is_null() {
                gsl_vector_free((*s).x);
                gsl_vector_free((*s).gradient);
                gsl_vector_free((*s).dx);
                libc::free(s as *mut c_void);
                return None;
            }

            if let Some(alloc) = (*t).alloc {
                let status = alloc((*s).state, n);
                if status != GslError::Success as c_int {
                    libc::free((*s).state);
                    gsl_vector_free((*s).x);
                    gsl_vector_free((*s).gradient);
                    gsl_vector_free((*s).dx);
                    libc::free(s as *mut c_void);
                    return None;
                }
            }

            Some(Self { inner: s })
        }
    }

    pub fn set(&mut self, fdf: *mut GslMultiminFunctionFdf, x: *const GslVector, step_size: c_double, tol: c_double) -> Result<(), GslError> {
        unsafe {
            if (*(*self.inner).x).size != (*fdf).n {
                return Err(GslError::Badlen);
            }
            if (*x).size != (*fdf).n {
                return Err(GslError::Badlen);
            }

            (*self.inner).fdf = fdf;
            gsl_vector_memcpy((*self.inner).x, x);
            gsl_vector_set_zero((*self.inner).dx);

            if let Some(set) = (*(*self.inner).type_).set {
                let status = set(
                    (*self.inner).state,
                    (*self.inner).fdf,
                    (*self.inner).x,
                    &mut (*self.inner).f,
                    (*self.inner).gradient,
                    step_size,
                    tol,
                );
                if status != GslError::Success as c_int {
                    return Err(GslError::from(status));
                }
            }

            Ok(())
        }
    }

    pub fn iterate(&mut self) -> Result<(), GslError> {
        unsafe {
            if let Some(iterate) = (*(*self.inner).type_).iterate {
                let status = iterate(
                    (*self.inner).state,
                    (*self.inner).fdf,
                    (*self.inner).x,
                    &mut (*self.inner).f,
                    (*self.inner).gradient,
                    (*self.inner).dx,
                );
                if status != GslError::Success as c_int {
                    return Err(GslError::from(status));
                }
            }
            Ok(())
        }
    }

    pub fn restart(&mut self) -> Result<(), GslError> {
        unsafe {
            if let Some(restart) = (*(*self.inner).type_).restart {
                let status = restart((*self.inner).state);
                if status != GslError::Success as c_int {
                    return Err(GslError::from(status));
                }
            }
            Ok(())
        }
    }

    pub fn name(&self) -> Option<&str> {
        unsafe {
            if (*self.inner).type_.is_null() {
                return None;
            }
            let name_ptr = (*(*self.inner).type_).name;
            if name_ptr.is_null() {
                return None;
            }
            std::ffi::CStr::from_ptr(name_ptr).to_str().ok()
        }
    }

    pub fn x(&self) -> *mut GslVector {
        unsafe { (*self.inner).x }
    }

    pub fn dx(&self) -> *mut GslVector {
        unsafe { (*self.inner).dx }
    }

    pub fn gradient(&self) -> *mut GslVector {
        unsafe { (*self.inner).gradient }
    }

    pub fn minimum(&self) -> c_double {
        unsafe { (*self.inner).f }
    }
}

impl Drop for MultiminFdfminimizer {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                if let Some(free) = (*(*self.inner).type_).free {
                    free((*self.inner).state);
                }
                libc::free((*self.inner).state);
                gsl_vector_free((*self.inner).dx);
                gsl_vector_free((*self.inner).gradient);
                gsl_vector_free((*self.inner).x);
                libc::free(self.inner as *mut c_void);
            }
        }
    }
}

impl From<c_int> for GslError {
    fn from(value: c_int) -> Self {
        match value {
            0 => GslError::Success,
            -1 => GslError::Failure,
            -2 => GslError::Continue,
            1 => GslError::Dom,
            2 => GslError::Range,
            3 => GslError::Fault,
            4 => GslError::Inval,
            5 => GslError::Failed,
            6 => GslError::Factor,
            7 => GslError::Sanity,
            8 => GslError::Nomem,
            9 => GslError::Badfunc,
            10 => GslError::Runaway,
            11 => GslError::Maxiter,
            12 => GslError::Zerodiv,
            13 => GslError::Badtol,
            14 => GslError::Tol,
            15 => GslError::Undrflw,
            16 => GslError::Ovrflw,
            17 => GslError::Loss,
            18 => GslError::Round,
            19 => GslError::Badlen,
            20 => GslError::Notsqr,
            21 => GslError::Sing,
            22 => GslError::Diverge,
            23 => GslError::Unsup,
            24 => GslError::Unimpl,
            25 => GslError::Cache,
            26 => GslError::Table,
            27 => GslError::Noprog,
            28 => GslError::Noprogj,
            29 => GslError::Tolf,
            30 => GslError::Tolx,
            31 => GslError::Tolg,
            32 => GslError::Eof,
            _ => GslError::Failure,
        }
    }
}

extern "C" {
    fn gsl_vector_calloc(n: usize) -> *mut GslVector;
    fn gsl_vector_free(v: *mut GslVector);
    fn gsl_vector_set_zero(v: *mut GslVector);
    fn gsl_vector_memcpy(dest: *mut GslVector, src: *const GslVector) -> c_int;
}
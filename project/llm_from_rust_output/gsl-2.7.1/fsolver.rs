use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

pub type size_t = usize;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub struct GslBlock {
    size: size_t,
    data: *mut c_double,
}

#[derive(Debug)]
pub struct GslVector {
    size: size_t,
    stride: size_t,
    data: *mut c_double,
    block: *mut GslBlock,
    owner: c_int,
}

impl GslVector {
    pub fn new(n: size_t) -> Option<Self> {
        unsafe {
            let ptr = gsl_vector_calloc(n);
            if ptr.is_null() {
                None
            } else {
                Some(ptr::read(ptr))
            }
        }
    }
}

impl Drop for GslVector {
    fn drop(&mut self) {
        unsafe {
            gsl_vector_free(self);
        }
    }
}

#[derive(Debug)]
pub struct GslMultirootFunction {
    f: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut GslVector) -> c_int>,
    n: size_t,
    params: *mut c_void,
}

#[derive(Debug)]
pub struct GslMultirootFsolverType {
    name: *const c_char,
    size: size_t,
    alloc: Option<unsafe extern "C" fn(*mut c_void, size_t) -> c_int>,
    set: Option<unsafe extern "C" fn(*mut c_void, *mut GslMultirootFunction, *mut GslVector, *mut GslVector, *mut GslVector) -> c_int>,
    iterate: Option<unsafe extern "C" fn(*mut c_void, *mut GslMultirootFunction, *mut GslVector, *mut GslVector, *mut GslVector) -> c_int>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug)]
pub struct GslMultirootFsolver {
    solver_type: *const GslMultirootFsolverType,
    function: *mut GslMultirootFunction,
    x: *mut GslVector,
    f: *mut GslVector,
    dx: *mut GslVector,
    state: *mut c_void,
}

impl GslMultirootFsolver {
    pub fn new(solver_type: *const GslMultirootFsolverType, n: size_t) -> Option<Self> {
        unsafe {
            let state = ptr::null_mut();
            let x = GslVector::new(n)?;
            let f = GslVector::new(n)?;
            let dx = GslVector::new(n)?;
            
            let solver = Box::into_raw(Box::new(Self {
                solver_type,
                function: ptr::null_mut(),
                x: Box::into_raw(Box::new(x)),
                f: Box::into_raw(Box::new(f)),
                dx: Box::into_raw(Box::new(dx)),
                state,
            }));
            
            if let Some(alloc) = (*solver_type).alloc {
                if alloc((*solver).state, n) != GslError::Success as c_int {
                    return None;
                }
            }
            
            Some(*Box::from_raw(solver))
        }
    }

    pub fn set(&mut self, function: *mut GslMultirootFunction, x: *const GslVector) -> Result<(), GslError> {
        unsafe {
            if (*(*self).x).size != (*function).n || (*x).size != (*function).n {
                return Err(GslError::Badlen);
            }
            
            self.function = function;
            gsl_vector_memcpy(self.x, x);
            
            if let Some(set) = (*(*self).solver_type).set {
                let status = set(self.state, self.function, self.x, self.f, self.dx);
                if status != GslError::Success as c_int {
                    return Err(GslError::from(status));
                }
            }
            
            Ok(())
        }
    }

    pub fn iterate(&mut self) -> Result<(), GslError> {
        unsafe {
            if let Some(iterate) = (*(*self).solver_type).iterate {
                let status = iterate(self.state, self.function, self.x, self.f, self.dx);
                if status != GslError::Success as c_int {
                    return Err(GslError::from(status));
                }
            }
            Ok(())
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr((*self.solver_type).name).to_str().unwrap()
        }
    }

    pub fn root(&self) -> &GslVector {
        unsafe { &*self.x }
    }

    pub fn dx(&self) -> &GslVector {
        unsafe { &*self.dx }
    }

    pub fn f(&self) -> &GslVector {
        unsafe { &*self.f }
    }
}

impl Drop for GslMultirootFsolver {
    fn drop(&mut self) {
        unsafe {
            if let Some(free) = (*(*self).solver_type).free {
                free(self.state);
            }
            ptr::drop_in_place(self.x);
            ptr::drop_in_place(self.f);
            ptr::drop_in_place(self.dx);
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
            29 => GslError::Etolf,
            30 => GslError::Etolx,
            31 => GslError::Etolg,
            32 => GslError::Eof,
            _ => GslError::Failure,
        }
    }
}

extern "C" {
    fn gsl_vector_calloc(n: size_t) -> *mut GslVector;
    fn gsl_vector_free(v: *mut GslVector);
    fn gsl_vector_memcpy(dest: *mut GslVector, src: *const GslVector) -> c_int;
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
}
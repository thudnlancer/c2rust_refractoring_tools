use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_ulong};

pub type size_t = c_ulong;

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
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslVectorConstView {
    pub vector: GslVector,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

pub type GslVectorConstView = GslVectorConstView;

pub trait GslVectorOps {
    fn set_zero(&mut self) -> GslError;
    fn set_all(&mut self, x: c_double) -> GslError;
    fn set(&mut self, i: size_t, x: c_double) -> GslError;
    fn ptr(&mut self, i: size_t) -> *mut c_double;
}

impl GslVectorOps for GslVector {
    fn set_zero(&mut self) -> GslError {
        // Safe because we're initializing the vector to zero
        unsafe {
            gsl_vector_set_zero(self);
        }
        GslError::Success
    }

    fn set_all(&mut self, x: c_double) -> GslError {
        // Safe because we're setting all elements to a valid value
        unsafe {
            gsl_vector_set_all(self, x);
        }
        GslError::Success
    }

    fn set(&mut self, i: size_t, x: c_double) -> GslError {
        // Safe if i is within bounds
        unsafe {
            gsl_vector_set(self, i, x);
        }
        GslError::Success
    }

    fn ptr(&mut self, i: size_t) -> *mut c_double {
        // Safe if i is within bounds
        unsafe { gsl_vector_ptr(self, i) }
    }
}

pub trait GslMatrixOps {
    fn const_column(&self, j: size_t) -> GslVectorConstView;
}

impl GslMatrixOps for GslMatrix {
    fn const_column(&self, j: size_t) -> GslVectorConstView {
        // Safe if j is within bounds
        unsafe { gsl_matrix_const_column(self, j) }
    }
}

pub trait GslBlasOps {
    fn dnrm2(x: &GslVector) -> c_double;
}

impl GslBlasOps for GslVector {
    fn dnrm2(x: &GslVector) -> c_double {
        // Safe because we're just computing a norm
        unsafe { gsl_blas_dnrm2(x) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GslMultifitNlinearScale {
    pub name: &'static str,
    pub init: fn(&GslMatrix, &mut GslVector) -> GslError,
    pub update: fn(&GslMatrix, &mut GslVector) -> GslError,
}

fn init_diag_levenberg(_: &GslMatrix, diag: &mut GslVector) -> GslError {
    diag.set_all(1.0)
}

fn update_diag_levenberg(_: &GslMatrix, _: &mut GslVector) -> GslError {
    GslError::Success
}

fn init_diag_marquardt(j: &GslMatrix, diag: &mut GslVector) -> GslError {
    update_diag_marquardt(j, diag)
}

fn update_diag_marquardt(j: &GslMatrix, diag: &mut GslVector) -> GslError {
    let p = j.size2;
    for j in 0..p {
        let v = j.const_column(j);
        let mut norm = GslVector::dnrm2(&v.vector);
        if norm == 0.0 {
            norm = 1.0;
        }
        diag.set(j, norm)?;
    }
    GslError::Success
}

fn init_diag_more(j: &GslMatrix, diag: &mut GslVector) -> GslError {
    diag.set_zero()?;
    update_diag_more(j, diag)
}

fn update_diag_more(j: &GslMatrix, diag: &mut GslVector) -> GslError {
    let p = j.size2;
    for j in 0..p {
        let v = j.const_column(j);
        let mut norm = GslVector::dnrm2(&v.vector);
        let diagj = diag.ptr(j);
        if norm == 0.0 {
            norm = 1.0;
        }
        unsafe {
            *diagj = if *diagj > norm { *diagj } else { norm };
        }
    }
    GslError::Success
}

pub const LEVENBERG_SCALE: GslMultifitNlinearScale = GslMultifitNlinearScale {
    name: "levenberg",
    init: init_diag_levenberg,
    update: update_diag_levenberg,
};

pub const MARQUARDT_SCALE: GslMultifitNlinearScale = GslMultifitNlinearScale {
    name: "marquardt",
    init: init_diag_marquardt,
    update: update_diag_marquardt,
};

pub const MORE_SCALE: GslMultifitNlinearScale = GslMultifitNlinearScale {
    name: "more",
    init: init_diag_more,
    update: update_diag_more,
};

// External C functions - these would be linked to the actual GSL library
extern "C" {
    fn gsl_vector_set_zero(v: *mut GslVector);
    fn gsl_vector_set_all(v: *mut GslVector, x: c_double);
    fn gsl_matrix_const_column(m: *const GslMatrix, j: size_t) -> GslVectorConstView;
    fn gsl_blas_dnrm2(X: *const GslVector) -> c_double;
    fn gsl_vector_set(v: *mut GslVector, i: size_t, x: c_double);
    fn gsl_vector_ptr(v: *mut GslVector, i: size_t) -> *mut c_double;
}
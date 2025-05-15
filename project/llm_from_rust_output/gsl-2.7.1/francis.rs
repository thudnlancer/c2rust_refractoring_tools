use libc::{c_double, c_int, c_ulong, c_void};
use std::f64;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_complex {
    pub dat: [c_double; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block {
    pub size: c_ulong,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector {
    pub size: c_ulong,
    pub stride: c_ulong,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_view {
    pub vector: gsl_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_complex {
    pub size: c_ulong,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_complex {
    pub size: c_ulong,
    pub stride: c_ulong,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix {
    pub size1: c_ulong,
    pub size2: c_ulong,
    pub tda: c_ulong,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_view {
    pub matrix: gsl_matrix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_eigen_francis_workspace {
    pub size: c_ulong,
    pub max_iterations: c_ulong,
    pub n_iter: c_ulong,
    pub n_evals: c_ulong,
    pub compute_t: c_int,
    pub H: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
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
}

impl From<c_int> for GslError {
    fn from(code: c_int) -> Self {
        match code {
            0 => GslError::Success,
            -1 => GslError::Failure,
            -2 => GslError::Continue,
            1 => GslError::Domain,
            2 => GslError::Range,
            3 => GslError::Fault,
            4 => GslError::Invalid,
            5 => GslError::Failed,
            6 => GslError::Factor,
            7 => GslError::Sanity,
            8 => GslError::NoMem,
            9 => GslError::BadFunc,
            10 => GslError::Runaway,
            11 => GslError::MaxIter,
            12 => GslError::ZeroDiv,
            13 => GslError::BadTol,
            14 => GslError::Tol,
            15 => GslError::Underflow,
            16 => GslError::Overflow,
            17 => GslError::Loss,
            18 => GslError::Round,
            19 => GslError::BadLen,
            20 => GslError::NotSquare,
            21 => GslError::Singular,
            22 => GslError::Diverge,
            23 => GslError::Unsupported,
            24 => GslError::Unimplemented,
            25 => GslError::Cache,
            26 => GslError::Table,
            27 => GslError::NoProgress,
            28 => GslError::NoProgressJ,
            29 => GslError::TolF,
            30 => GslError::TolX,
            31 => GslError::TolG,
            32 => GslError::Eof,
            _ => GslError::Invalid,
        }
    }
}

pub fn gsl_eigen_francis_alloc() -> *mut gsl_eigen_francis_workspace {
    unsafe {
        let w = libc::calloc(
            1,
            std::mem::size_of::<gsl_eigen_francis_workspace>() as c_ulong,
        ) as *mut gsl_eigen_francis_workspace;

        if w.is_null() {
            gsl_error(
                b"failed to allocate space for workspace\0".as_ptr() as *const _,
                b"francis.c\0".as_ptr() as *const _,
                81,
                GslError::NoMem as c_int,
            );
            return ptr::null_mut();
        }

        (*w).size = 0;
        (*w).max_iterations = 0;
        (*w).n_iter = 0;
        (*w).n_evals = 0;
        (*w).compute_t = 0;
        (*w).Z = ptr::null_mut();
        (*w).H = ptr::null_mut();

        w
    }
}

pub fn gsl_eigen_francis_free(w: *mut gsl_eigen_francis_workspace) {
    if !w.is_null() {
        unsafe {
            libc::free(w as *mut c_void);
        }
    }
}

pub fn gsl_eigen_francis_T(compute_t: c_int, w: *mut gsl_eigen_francis_workspace) {
    if !w.is_null() {
        unsafe {
            (*w).compute_t = compute_t;
        }
    }
}

pub fn gsl_eigen_francis(
    H: *mut gsl_matrix,
    eval: *mut gsl_vector_complex,
    w: *mut gsl_eigen_francis_workspace,
) -> GslError {
    unsafe {
        if (*H).size1 != (*H).size2 {
            gsl_error(
                b"matrix must be square to compute eigenvalues\0".as_ptr() as *const _,
                b"francis.c\0".as_ptr() as *const _,
                158,
                GslError::NotSquare as c_int,
            );
            return GslError::NotSquare;
        }

        if (*eval).size != (*H).size1 {
            gsl_error(
                b"eigenvalue vector must match matrix size\0".as_ptr() as *const _,
                b"francis.c\0".as_ptr() as *const _,
                162,
                GslError::BadLen as c_int,
            );
            return GslError::BadLen;
        }

        let N = (*H).size1;
        (*w).size = N;
        (*w).max_iterations = 30 * N;
        (*w).H = H;
        (*w).n_iter = 0;
        (*w).n_evals = 0;

        for j in 0..(N as c_int - 3) {
            gsl_matrix_set(
                H,
                (j as c_ulong + 2) as c_ulong,
                j as c_ulong,
                0.0,
            );
            gsl_matrix_set(
                H,
                (j as c_ulong + 3) as c_ulong,
                j as c_ulong,
                0.0,
            );
        }

        if N > 2 {
            gsl_matrix_set(H, N - 1, N - 3, 0.0);
        }

        francis_schur_decomp(H, eval, w);

        if (*w).n_evals != N {
            gsl_error(
                b"maximum iterations reached without finding all eigenvalues\0"
                    .as_ptr() as *const _,
                b"francis.c\0".as_ptr() as *const _,
                209,
                GslError::MaxIter as c_int,
            );
            return GslError::MaxIter;
        }

        GslError::Success
    }
}

pub fn gsl_eigen_francis_Z(
    H: *mut gsl_matrix,
    eval: *mut gsl_vector_complex,
    Z: *mut gsl_matrix,
    w: *mut gsl_eigen_francis_workspace,
) -> GslError {
    unsafe {
        (*w).Z = Z;
        let s = gsl_eigen_francis(H, eval, w);
        (*w).Z = ptr::null_mut();
        s
    }
}

// Helper functions would follow here, implementing the same logic as the C code
// but with Rust safety features and error handling

fn gsl_error(reason: *const libc::c_char, file: *const libc::c_char, line: c_int, gsl_errno: c_int) {
    // Implementation would handle GSL errors appropriately
}

fn gsl_matrix_set(m: *mut gsl_matrix, i: c_ulong, j: c_ulong, x: c_double) {
    unsafe {
        *((*m).data).offset((i * (*m).tda + j) as isize) = x;
    }
}

fn gsl_matrix_get(m: *const gsl_matrix, i: c_ulong, j: c_ulong) -> c_double {
    unsafe { *((*m).data).offset((i * (*m).tda + j) as isize) }
}

// Additional helper functions would be implemented similarly
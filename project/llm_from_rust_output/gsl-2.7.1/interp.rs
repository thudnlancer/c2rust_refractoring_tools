use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_interp_accel {
    pub cache: usize,
    pub miss_count: usize,
    pub hit_count: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_interp_type {
    pub name: *const c_char,
    pub min_size: c_uint,
    pub alloc: Option<extern "C" fn(usize) -> *mut c_void>,
    pub init: Option<extern "C" fn(*mut c_void, *const c_double, *const c_double, usize) -> c_int>,
    pub eval: Option<extern "C" fn(*const c_void, *const c_double, *const c_double, usize, c_double, *mut gsl_interp_accel, *mut c_double) -> c_int>,
    pub eval_deriv: Option<extern "C" fn(*const c_void, *const c_double, *const c_double, usize, c_double, *mut gsl_interp_accel, *mut c_double) -> c_int>,
    pub eval_deriv2: Option<extern "C" fn(*const c_void, *const c_double, *const c_double, usize, c_double, *mut gsl_interp_accel, *mut c_double) -> c_int>,
    pub eval_integ: Option<extern "C" fn(*const c_void, *const c_double, *const c_double, usize, *mut gsl_interp_accel, c_double, c_double, *mut c_double) -> c_int>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_interp {
    pub type_: *const gsl_interp_type,
    pub xmin: c_double,
    pub xmax: c_double,
    pub size: usize,
    pub state: *mut c_void,
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
    Singularity = 21,
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
            21 => GslError::Singularity,
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

pub fn gsl_interp_alloc(T: &gsl_interp_type, size: usize) -> Option<Box<gsl_interp>> {
    if size < T.min_size as usize {
        unsafe {
            gsl_error(
                "insufficient number of points for interpolation type\0".as_ptr() as *const c_char,
                "interp.c\0".as_ptr() as *const c_char,
                38,
                GslError::Invalid as c_int,
            );
        }
        return None;
    }

    let mut interp = Box::new(gsl_interp {
        type_: T,
        xmin: 0.0,
        xmax: 0.0,
        size,
        state: ptr::null_mut(),
    });

    if let Some(alloc_fn) = T.alloc {
        interp.state = unsafe { alloc_fn(size) };
        if interp.state.is_null() {
            unsafe {
                gsl_error(
                    "failed to allocate space for interp state\0".as_ptr() as *const c_char,
                    "interp.c\0".as_ptr() as *const c_char,
                    63,
                    GslError::NoMem as c_int,
                );
            }
            return None;
        }
    }

    Some(interp)
}

pub fn gsl_interp_init(
    interp: &mut gsl_interp,
    x_array: &[c_double],
    y_array: &[c_double],
) -> Result<(), GslError> {
    if x_array.len() != interp.size || y_array.len() != interp.size {
        unsafe {
            gsl_error(
                "data must match size of interpolation object\0".as_ptr() as *const c_char,
                "interp.c\0".as_ptr() as *const c_char,
                76,
                GslError::Invalid as c_int,
            );
        }
        return Err(GslError::Invalid);
    }

    for i in 1..x_array.len() {
        if x_array[i - 1] >= x_array[i] {
            unsafe {
                gsl_error(
                    "x values must be strictly increasing\0".as_ptr() as *const c_char,
                    "interp.c\0".as_ptr() as *const c_char,
                    83,
                    GslError::Invalid as c_int,
                );
            }
            return Err(GslError::Invalid);
        }
    }

    interp.xmin = x_array[0];
    interp.xmax = x_array[x_array.len() - 1];

    if let Some(init_fn) = unsafe { (*interp.type_).init } {
        let status = unsafe { init_fn(interp.state, x_array.as_ptr(), y_array.as_ptr(), interp.size) };
        if status != GslError::Success as c_int {
            return Err(GslError::from(status));
        }
    }

    Ok(())
}

pub fn gsl_interp_name(interp: &gsl_interp) -> &'static str {
    unsafe {
        CStr::from_ptr((*interp.type_).name)
            .to_str()
            .unwrap_or_default()
    }
}

pub fn gsl_interp_min_size(interp: &gsl_interp) -> c_uint {
    unsafe { (*interp.type_).min_size }
}

pub fn gsl_interp_type_min_size(T: &gsl_interp_type) -> c_uint {
    T.min_size
}

pub fn gsl_interp_free(interp: Option<Box<gsl_interp>>) {
    if let Some(mut interp) = interp {
        if let Some(free_fn) = unsafe { (*interp.type_).free } {
            unsafe { free_fn(interp.state) };
        }
    }
}

// Additional safe wrappers for other functions would follow the same pattern
// as above, converting raw pointers to references/slices where possible
// and using Result types for error handling.

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
}
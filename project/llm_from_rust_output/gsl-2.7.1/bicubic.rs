use libc::{c_char, c_double, c_int, c_uint, c_ulong, c_void};
use std::ffi::CString;
use std::ptr;

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
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
    NoMemory = 8,
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

#[derive(Debug, Clone, Copy)]
pub struct GslInterpAccel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}

#[derive(Debug, Clone, Copy)]
pub struct GslInterpType {
    pub name: *const c_char,
    pub min_size: c_uint,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub init: Option<
        unsafe extern "C" fn(*mut c_void, *const c_double, *const c_double, size_t) -> c_int,
    >,
    pub eval: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            size_t,
            c_double,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            size_t,
            c_double,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv2: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            size_t,
            c_double,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_integ: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            size_t,
            *mut GslInterpAccel,
            c_double,
            c_double,
            *mut c_double,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
pub struct GslInterp {
    pub type_: *const GslInterpType,
    pub xmin: c_double,
    pub xmax: c_double,
    pub size: size_t,
    pub state: *mut c_void,
}

#[derive(Debug, Clone, Copy)]
pub struct GslSpline {
    pub interp: *mut GslInterp,
    pub x: *mut c_double,
    pub y: *mut c_double,
    pub size: size_t,
}

#[derive(Debug, Clone, Copy)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
pub struct GslInterp2dType {
    pub name: *const c_char,
    pub min_size: c_uint,
    pub alloc: Option<unsafe extern "C" fn(size_t, size_t) -> *mut c_void>,
    pub init: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
        ) -> c_int,
    >,
    pub eval: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
            c_double,
            c_double,
            *mut GslInterpAccel,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv_x: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
            c_double,
            c_double,
            *mut GslInterpAccel,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv_y: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
            c_double,
            c_double,
            *mut GslInterpAccel,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv_xx: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
            c_double,
            c_double,
            *mut GslInterpAccel,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv_xy: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
            c_double,
            c_double,
            *mut GslInterpAccel,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub eval_deriv_yy: Option<
        unsafe extern "C" fn(
            *const c_void,
            *const c_double,
            *const c_double,
            *const c_double,
            size_t,
            size_t,
            c_double,
            c_double,
            *mut GslInterpAccel,
            *mut GslInterpAccel,
            *mut c_double,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
pub struct BicubicState {
    pub zx: *mut c_double,
    pub zy: *mut c_double,
    pub zxy: *mut c_double,
    pub xsize: size_t,
    pub ysize: size_t,
}

pub static GSL_INTERP_CSPLINE: *const GslInterpType = ptr::null();
pub static mut GSL_INTERP2D_BICUBIC: *const GslInterp2dType = ptr::null();

fn gsl_error(reason: &str, file: &str, line: c_int, gsl_errno: GslError) {
    let reason = CString::new(reason).unwrap();
    let file = CString::new(file).unwrap();
    unsafe {
        libc::fprintf(
            libc::stderr,
            b"GSL error: %s at %s:%d\n\0" as *const u8 as *const c_char,
            reason.as_ptr(),
            file.as_ptr(),
            line,
        );
    }
}

fn bicubic_alloc(xsize: size_t, ysize: size_t) -> Option<Box<BicubicState>> {
    let state = Box::new(BicubicState {
        zx: ptr::null_mut(),
        zy: ptr::null_mut(),
        zxy: ptr::null_mut(),
        xsize,
        ysize,
    });

    unsafe {
        state.zx = libc::malloc(xsize * ysize * std::mem::size_of::<c_double>() as size_t) as *mut c_double;
        if state.zx.is_null() {
            gsl_error("failed to allocate space for zx", "bicubic.c", 56, GslError::NoMemory);
            return None;
        }

        state.zy = libc::malloc(xsize * ysize * std::mem::size_of::<c_double>() as size_t) as *mut c_double;
        if state.zy.is_null() {
            libc::free(state.zx as *mut c_void);
            gsl_error("failed to allocate space for zy", "bicubic.c", 63, GslError::NoMemory);
            return None;
        }

        state.zxy = libc::malloc(xsize * ysize * std::mem::size_of::<c_double>() as size_t) as *mut c_double;
        if state.zxy.is_null() {
            libc::free(state.zx as *mut c_void);
            libc::free(state.zy as *mut c_void);
            gsl_error("failed to allocate space for zxy", "bicubic.c", 70, GslError::NoMemory);
            return None;
        }
    }

    Some(state)
}

fn bicubic_free(state: Option<Box<BicubicState>>) {
    if let Some(state) = state {
        unsafe {
            if !state.zx.is_null() {
                libc::free(state.zx as *mut c_void);
            }
            if !state.zy.is_null() {
                libc::free(state.zy as *mut c_void);
            }
            if !state.zxy.is_null() {
                libc::free(state.zxy as *mut c_void);
            }
        }
    }
}

// ... (其他函数的类似转换)

static BICUBIC_TYPE: GslInterp2dType = GslInterp2dType {
    name: b"bicubic\0" as *const u8 as *const c_char,
    min_size: 4,
    alloc: Some(bicubic_alloc as unsafe extern "C" fn(size_t, size_t) -> *mut c_void),
    init: Some(
        bicubic_init
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
            ) -> c_int,
    ),
    eval: Some(
        bicubic_eval
            as unsafe extern "C" fn(
                *const c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
                c_double,
                c_double,
                *mut GslInterpAccel,
                *mut GslInterpAccel,
                *mut c_double,
            ) -> c_int,
    ),
    eval_deriv_x: Some(
        bicubic_deriv_x
            as unsafe extern "C" fn(
                *const c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
                c_double,
                c_double,
                *mut GslInterpAccel,
                *mut GslInterpAccel,
                *mut c_double,
            ) -> c_int,
    ),
    eval_deriv_y: Some(
        bicubic_deriv_y
            as unsafe extern "C" fn(
                *const c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
                c_double,
                c_double,
                *mut GslInterpAccel,
                *mut GslInterpAccel,
                *mut c_double,
            ) -> c_int,
    ),
    eval_deriv_xx: Some(
        bicubic_deriv_xx
            as unsafe extern "C" fn(
                *const c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
                c_double,
                c_double,
                *mut GslInterpAccel,
                *mut GslInterpAccel,
                *mut c_double,
            ) -> c_int,
    ),
    eval_deriv_xy: Some(
        bicubic_deriv_xy
            as unsafe extern "C" fn(
                *const c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
                c_double,
                c_double,
                *mut GslInterpAccel,
                *mut GslInterpAccel,
                *mut c_double,
            ) -> c_int,
    ),
    eval_deriv_yy: Some(
        bicubic_deriv_yy
            as unsafe extern "C" fn(
                *const c_void,
                *const c_double,
                *const c_double,
                *const c_double,
                size_t,
                size_t,
                c_double,
                c_double,
                *mut GslInterpAccel,
                *mut GslInterpAccel,
                *mut c_double,
            ) -> c_int,
    ),
    free: Some(bicubic_free as unsafe extern "C" fn(*mut c_void)),
};

pub unsafe fn gsl_interp2d_bicubic_init() {
    GSL_INTERP2D_BICUBIC = &BICUBIC_TYPE;
}
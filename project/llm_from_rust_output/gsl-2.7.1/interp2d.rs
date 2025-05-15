use std::ffi::CString;
use std::ptr;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
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
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Debug, Clone, Copy)]
pub struct GslInterpAccel {
    pub cache: usize,
    pub miss_count: usize,
    pub hit_count: usize,
}

pub type GslInterp2dAllocFn = fn(usize, usize) -> *mut std::ffi::c_void;
pub type GslInterp2dInitFn = fn(*mut std::ffi::c_void, *const f64, *const f64, *const f64, usize, usize) -> i32;
pub type GslInterp2dEvalFn = fn(*const std::ffi::c_void, *const f64, *const f64, *const f64, usize, usize, f64, f64, *mut GslInterpAccel, *mut GslInterpAccel, *mut f64) -> i32;
pub type GslInterp2dFreeFn = fn(*mut std::ffi::c_void);

#[derive(Debug, Clone, Copy)]
pub struct GslInterp2dType {
    pub name: &'static str,
    pub min_size: u32,
    pub alloc: Option<GslInterp2dAllocFn>,
    pub init: Option<GslInterp2dInitFn>,
    pub eval: Option<GslInterp2dEvalFn>,
    pub eval_deriv_x: Option<GslInterp2dEvalFn>,
    pub eval_deriv_y: Option<GslInterp2dEvalFn>,
    pub eval_deriv_xx: Option<GslInterp2dEvalFn>,
    pub eval_deriv_xy: Option<GslInterp2dEvalFn>,
    pub eval_deriv_yy: Option<GslInterp2dEvalFn>,
    pub free: Option<GslInterp2dFreeFn>,
}

#[derive(Debug)]
pub struct GslInterp2d {
    pub interp_type: &'static GslInterp2dType,
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub xsize: usize,
    pub ysize: usize,
    pub state: Option<Box<std::ffi::c_void>>,
}

impl GslInterp2d {
    pub fn new(interp_type: &'static GslInterp2dType, xsize: usize, ysize: usize) -> Option<Self> {
        if xsize < interp_type.min_size as usize || ysize < interp_type.min_size as usize {
            gsl_error("insufficient number of points for interpolation type", "interp2d.c", 44, GslError::Invalid);
            return None;
        }

        let state = if let Some(alloc_fn) = interp_type.alloc {
            let state_ptr = alloc_fn(xsize, ysize);
            if state_ptr.is_null() {
                gsl_error("failed to allocate space for gsl_interp2d state", "interp2d.c", 69, GslError::NoMem);
                return None;
            }
            Some(unsafe { Box::from_raw(state_ptr) })
        } else {
            None
        };

        Some(Self {
            interp_type,
            xmin: 0.0,
            xmax: 0.0,
            ymin: 0.0,
            ymax: 0.0,
            xsize,
            ysize,
            state,
        })
    }

    pub fn init(&mut self, xarr: &[f64], yarr: &[f64], zarr: &[f64]) -> Result<(), GslError> {
        if xarr.len() != self.xsize || yarr.len() != self.ysize {
            gsl_error("data must match size of interpolation object", "interp2d.c", 94, GslError::Invalid);
            return Err(GslError::Invalid);
        }

        for i in 1..xarr.len() {
            if xarr[i - 1] >= xarr[i] {
                gsl_error("x values must be strictly increasing", "interp2d.c", 101, GslError::Invalid);
                return Err(GslError::Invalid);
            }
        }

        for i in 1..yarr.len() {
            if yarr[i - 1] >= yarr[i] {
                gsl_error("y values must be strictly increasing", "interp2d.c", 109, GslError::Invalid);
                return Err(GslError::Invalid);
            }
        }

        self.xmin = xarr[0];
        self.xmax = xarr[xarr.len() - 1];
        self.ymin = yarr[0];
        self.ymax = yarr[yarr.len() - 1];

        if let Some(init_fn) = self.interp_type.init {
            let status = init_fn(
                self.state.as_ref().map_or(ptr::null_mut(), |s| &**s as *const _ as *mut _),
                xarr.as_ptr(),
                yarr.as_ptr(),
                zarr.as_ptr(),
                self.xsize,
                self.ysize,
            );
            if status != GslError::Success as i32 {
                return Err(GslError::from(status));
            }
        }

        Ok(())
    }

    pub fn eval(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut GslInterpAccel, ya: &mut GslInterpAccel) -> Result<f64, GslError> {
        self.eval_impl(self.interp_type.eval, xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_x(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut GslInterpAccel, ya: &mut GslInterpAccel) -> Result<f64, GslError> {
        self.eval_impl(self.interp_type.eval_deriv_x, xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_y(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut GslInterpAccel, ya: &mut GslInterpAccel) -> Result<f64, GslError> {
        self.eval_impl(self.interp_type.eval_deriv_y, xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_xx(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut GslInterpAccel, ya: &mut GslInterpAccel) -> Result<f64, GslError> {
        self.eval_impl(self.interp_type.eval_deriv_xx, xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_xy(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut GslInterpAccel, ya: &mut GslInterpAccel) -> Result<f64, GslError> {
        self.eval_impl(self.interp_type.eval_deriv_xy, xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_yy(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut GslInterpAccel, ya: &mut GslInterpAccel) -> Result<f64, GslError> {
        self.eval_impl(self.interp_type.eval_deriv_yy, xarr, yarr, zarr, x, y, xa, ya)
    }

    fn eval_impl(
        &self,
        eval_fn: Option<GslInterp2dEvalFn>,
        xarr: &[f64],
        yarr: &[f64],
        zarr: &[f64],
        x: f64,
        y: f64,
        xa: &mut GslInterpAccel,
        ya: &mut GslInterpAccel,
    ) -> Result<f64, GslError> {
        if x < self.xmin || x > self.xmax {
            gsl_error("interpolation x value out of range", "interp2d.c", 143, GslError::Domain);
            return Err(GslError::Domain);
        }
        if y < self.ymin || y > self.ymax {
            gsl_error("interpolation y value out of range", "interp2d.c", 147, GslError::Domain);
            return Err(GslError::Domain);
        }

        let mut result = 0.0;
        if let Some(eval_fn) = eval_fn {
            let status = eval_fn(
                self.state.as_ref().map_or(ptr::null(), |s| &**s as *const _),
                xarr.as_ptr(),
                yarr.as_ptr(),
                zarr.as_ptr(),
                self.xsize,
                self.ysize,
                x,
                y,
                xa,
                ya,
                &mut result,
            );
            if status != GslError::Success as i32 {
                return Err(GslError::from(status));
            }
        }
        Ok(result)
    }

    pub fn min_size(&self) -> usize {
        self.interp_type.min_size as usize
    }

    pub fn name(&self) -> &'static str {
        self.interp_type.name
    }

    pub fn index(&self, i: usize, j: usize) -> Result<usize, GslError> {
        if i >= self.xsize {
            gsl_error("x index out of range", "interp2d.c", 377, GslError::Range);
            return Err(GslError::Range);
        }
        if j >= self.ysize {
            gsl_error("y index out of range", "interp2d.c", 381, GslError::Range);
            return Err(GslError::Range);
        }
        Ok(j * self.xsize + i)
    }

    pub fn set(&self, zarr: &mut [f64], i: usize, j: usize, z: f64) -> Result<(), GslError> {
        let idx = self.index(i, j)?;
        zarr[idx] = z;
        Ok(())
    }

    pub fn get(&self, zarr: &[f64], i: usize, j: usize) -> Result<f64, GslError> {
        let idx = self.index(i, j)?;
        Ok(zarr[idx])
    }
}

impl Drop for GslInterp2d {
    fn drop(&mut self) {
        if let Some(free_fn) = self.interp_type.free {
            if let Some(state) = self.state.take() {
                free_fn(Box::into_raw(state));
            }
        }
    }
}

impl From<i32> for GslError {
    fn from(code: i32) -> Self {
        match code {
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
            -2 => GslError::Continue,
            -1 => GslError::Failure,
            _ => GslError::Success,
        }
    }
}

fn gsl_error(reason: &str, file: &str, line: i32, gsl_errno: GslError) {
    let reason_c = CString::new(reason).unwrap();
    let file_c = CString::new(file).unwrap();
    unsafe {
        // Assuming there's an external gsl_error function we need to call
        extern "C" {
            fn gsl_error(reason: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, gsl_errno: libc::c_int);
        }
        gsl_error(reason_c.as_ptr(), file_c.as_ptr(), line, gsl_errno as i32);
    }
}
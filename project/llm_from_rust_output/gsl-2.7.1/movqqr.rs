use libc::{c_char, c_double, c_int, c_uint, c_ulong, c_void};

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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GslMovstatEndType {
    PadZero = 0,
    PadValue = 1,
    Truncate = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMovstatAccum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut c_void) -> c_int>,
    pub insert: Option<unsafe extern "C" fn(c_double, *mut c_void) -> c_int>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut c_void, *mut c_double, *const c_void) -> c_int>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GslMovstatWorkspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut c_double,
    pub state: *mut c_void,
    pub state_size: size_t,
}

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
    fn gsl_movstat_apply_accum(
        endtype: GslMovstatEndType,
        x: *const GslVector,
        accum: *const GslMovstatAccum,
        accum_params: *mut c_void,
        y: *mut GslVector,
        z: *mut GslVector,
        w: *mut GslMovstatWorkspace,
    ) -> c_int;
    static gsl_movstat_accum_qqr: *const GslMovstatAccum;
}

pub fn gsl_movstat_qqr(
    endtype: GslMovstatEndType,
    x: &GslVector,
    q: c_double,
    xqqr: &mut GslVector,
    w: &mut GslMovstatWorkspace,
) -> Result<(), GslError> {
    if x.size != xqqr.size {
        unsafe {
            gsl_error(
                b"x and xqqr vectors must have same length\0".as_ptr() as *const c_char,
                b"movqqr.c\0".as_ptr() as *const c_char,
                50,
                GslError::Badlen as c_int,
            );
        }
        return Err(GslError::Badlen);
    }

    if q < 0.0 || q > 0.5 {
        unsafe {
            gsl_error(
                b"q must be between 0 and 0.5\0".as_ptr() as *const c_char,
                b"movqqr.c\0".as_ptr() as *const c_char,
                54,
                GslError::Dom as c_int,
            );
        }
        return Err(GslError::Dom);
    }

    let mut qq = q;
    let status = unsafe {
        gsl_movstat_apply_accum(
            endtype,
            x,
            gsl_movstat_accum_qqr,
            &mut qq as *mut c_double as *mut c_void,
            xqqr,
            std::ptr::null_mut(),
            w,
        )
    };

    match status {
        0 => Ok(()),
        e => Err(std::mem::transmute(e)),
    }
}
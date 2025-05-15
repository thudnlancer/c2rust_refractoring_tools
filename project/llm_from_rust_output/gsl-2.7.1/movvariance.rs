use libc::{c_double, c_int, c_uint, c_ulong, c_void};

pub type size_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

pub type GslMovstatEndT = c_uint;
pub const GSL_MOVSTAT_END_PADZERO: GslMovstatEndT = 0;
pub const GSL_MOVSTAT_END_PADVALUE: GslMovstatEndT = 1;
pub const GSL_MOVSTAT_END_TRUNCATE: GslMovstatEndT = 2;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslMovstatAccum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut c_void) -> c_int>,
    pub insert: Option<unsafe extern "C" fn(c_double, *mut c_void) -> c_int>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut c_void, *mut c_double, *const c_void) -> c_int>,
}

#[derive(Copy, Clone)]
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
    static gsl_movstat_accum_sd: *const GslMovstatAccum;
    static gsl_movstat_accum_variance: *const GslMovstatAccum;
    
    fn gsl_movstat_apply_accum(
        endtype: GslMovstatEndT,
        x: *const GslVector,
        accum: *const GslMovstatAccum,
        accum_params: *mut c_void,
        y: *mut GslVector,
        z: *mut GslVector,
        w: *mut GslMovstatWorkspace,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn gsl_movstat_variance(
    endtype: GslMovstatEndT,
    x: *const GslVector,
    y: *mut GslVector,
    w: *mut GslMovstatWorkspace,
) -> c_int {
    gsl_movstat_apply_accum(
        endtype,
        x,
        gsl_movstat_accum_variance,
        std::ptr::null_mut(),
        y,
        std::ptr::null_mut(),
        w,
    )
}

#[no_mangle]
pub unsafe extern "C" fn gsl_movstat_sd(
    endtype: GslMovstatEndT,
    x: *const GslVector,
    y: *mut GslVector,
    w: *mut GslMovstatWorkspace,
) -> c_int {
    gsl_movstat_apply_accum(
        endtype,
        x,
        gsl_movstat_accum_sd,
        std::ptr::null_mut(),
        y,
        std::ptr::null_mut(),
        w,
    )
}
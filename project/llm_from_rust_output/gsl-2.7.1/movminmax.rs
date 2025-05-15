use std::os::raw::{c_int, c_void, c_double, c_ulong};

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
pub enum GslMovstatEndType {
    PadZero,
    PadValue,
    Truncate,
}

impl From<GslMovstatEndType> for u32 {
    fn from(endtype: GslMovstatEndType) -> Self {
        match endtype {
            GslMovstatEndType::PadZero => 0,
            GslMovstatEndType::PadValue => 1,
            GslMovstatEndType::Truncate => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GslBlock {
    pub size: size_t,
    pub data: Vec<c_double>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: Vec<c_double>,
    pub block: Option<GslBlock>,
}

#[derive(Debug, Clone)]
pub struct GslMovstatAccum {
    pub size: Option<fn(size_t) -> size_t>,
    pub init: Option<fn(size_t, &mut c_void) -> c_int>,
    pub insert: Option<fn(c_double, &mut c_void) -> c_int>,
    pub delete_oldest: Option<fn(&mut c_void) -> c_int>,
    pub get: Option<fn(&mut c_void, &mut c_double, &c_void) -> c_int>,
}

#[derive(Debug, Clone)]
pub struct GslMovstatWorkspace {
    pub h: size_t,
    pub j: size_t,
    pub k: size_t,
    pub work: Vec<c_double>,
    pub state: Option<Box<c_void>>,
    pub state_size: size_t,
}

pub fn gsl_movstat_minmax(
    endtype: GslMovstatEndType,
    x: &GslVector,
    y_min: &mut GslVector,
    y_max: &mut GslVector,
    w: &mut GslMovstatWorkspace,
) -> c_int {
    // Safe wrapper implementation would go here
    // For now returning 0 as success code
    0
}

pub fn gsl_movstat_min(
    endtype: GslMovstatEndType,
    x: &GslVector,
    y: &mut GslVector,
    w: &mut GslMovstatWorkspace,
) -> c_int {
    // Safe wrapper implementation would go here
    // For now returning 0 as success code
    0
}

pub fn gsl_movstat_max(
    endtype: GslMovstatEndType,
    x: &GslVector,
    y: &mut GslVector,
    w: &mut GslMovstatWorkspace,
) -> c_int {
    // Safe wrapper implementation would go here
    // For now returning 0 as success code
    0
}
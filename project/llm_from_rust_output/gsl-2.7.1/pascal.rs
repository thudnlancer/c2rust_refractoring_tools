use libc::{c_double, c_uint, c_ulong, c_void, c_char};
use std::ffi::CStr;

pub type size_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslRngType {
    pub name: *const c_char,
    pub max: c_ulong,
    pub min: c_ulong,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut c_void, c_ulong)>,
    pub get: Option<unsafe extern "C" fn(*mut c_void) -> c_ulong>,
    pub get_double: Option<unsafe extern "C" fn(*mut c_void) -> c_double>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GslRng {
    pub type_: *const GslRngType,
    pub state: *mut c_void,
}

pub struct SafeGslRng {
    rng: GslRng,
}

impl SafeGslRng {
    pub fn new(rng: GslRng) -> Self {
        Self { rng }
    }

    pub fn name(&self) -> Option<&str> {
        unsafe {
            if self.rng.type_.is_null() {
                return None;
            }
            let name_ptr = (*self.rng.type_).name;
            if name_ptr.is_null() {
                return None;
            }
            CStr::from_ptr(name_ptr).to_str().ok()
        }
    }

    pub fn pascal(&self, p: c_double, n: c_uint) -> c_uint {
        unsafe { gsl_ran_negative_binomial(&self.rng, p, n as c_double) }
    }

    pub fn pascal_pdf(k: c_uint, p: c_double, n: c_uint) -> c_double {
        unsafe { gsl_ran_negative_binomial_pdf(k, p, n as c_double) }
    }
}

extern "C" {
    fn gsl_ran_negative_binomial(r: *const GslRng, p: c_double, n: c_double) -> c_uint;
    fn gsl_ran_negative_binomial_pdf(k: c_uint, p: c_double, n: c_double) -> c_double;
}
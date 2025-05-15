use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_short};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct gsl_complex {
    pub dat: [c_double; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct gsl_complex_long_double {
    pub dat: [f128::f128; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct gsl_complex_float {
    pub dat: [c_float; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_long_double {
    pub size: usize,
    pub data: *mut f128::f128,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_long_double {
    pub size: usize,
    pub stride: usize,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_complex_long_double {
    pub size: usize,
    pub data: *mut f128::f128,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_complex_long_double {
    pub size: usize,
    pub stride: usize,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_complex {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_complex {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_float {
    pub size: usize,
    pub data: *mut c_float,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_float {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_float,
    pub block: *mut gsl_block_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_complex_float {
    pub size: usize,
    pub data: *mut c_float,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_complex_float {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_ulong {
    pub size: usize,
    pub data: *mut c_ulong,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_ulong {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_long {
    pub size: usize,
    pub data: *mut c_long,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_long {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_long,
    pub block: *mut gsl_block_long,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_uint {
    pub size: usize,
    pub data: *mut c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_uint {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_int {
    pub size: usize,
    pub data: *mut c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_int {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_int,
    pub block: *mut gsl_block_int,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_ushort {
    pub size: usize,
    pub data: *mut c_ushort,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_ushort {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_short {
    pub size: usize,
    pub data: *mut c_short,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_short {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_short,
    pub block: *mut gsl_block_short,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_uchar {
    pub size: usize,
    pub data: *mut c_uchar,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_uchar {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_block_char {
    pub size: usize,
    pub data: *mut c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_vector_char {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_char,
    pub block: *mut gsl_block_char,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Edom = 1,
    Erange = 2,
    Efault = 3,
    Einval = 4,
    Efailed = 5,
    Efactor = 6,
    Esanity = 7,
    Enomem = 8,
    Ebadfunc = 9,
    Erunaway = 10,
    Emaxiter = 11,
    Ezerodiv = 12,
    Ebadtol = 13,
    Etol = 14,
    Eundrflw = 15,
    Eovrflw = 16,
    Eloss = 17,
    Eround = 18,
    Ebadlen = 19,
    Enotsqr = 20,
    Esing = 21,
    Ediverge = 22,
    Eunsup = 23,
    Eunimpl = 24,
    Ecache = 25,
    Etable = 26,
    Enoprog = 27,
    Enoprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

pub static mut GSL_CHECK_RANGE: c_int = 1;

fn gsl_error(reason: &str, file: &str, line: c_int, gsl_errno: GslError) {
    let reason = CString::new(reason).unwrap();
    let file = CString::new(file).unwrap();
    unsafe {
        extern "C" {
            fn gsl_error(
                reason: *const c_char,
                file: *const c_char,
                line: c_int,
                gsl_errno: c_int,
            );
        }
        gsl_error(
            reason.as_ptr(),
            file.as_ptr(),
            line,
            gsl_errno as c_int,
        );
    }
}

macro_rules! impl_vector_ops {
    ($vector_type:ty, $element_type:ty, $error_msg:expr, $header:expr) => {
        impl $vector_type {
            pub fn get(&self, i: usize) -> Result<$element_type, GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 182, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe { Ok(*self.data.offset((i * self.stride) as isize)) }
            }

            pub fn set(&mut self, i: usize, x: $element_type) -> Result<(), GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 195, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe {
                    *self.data.offset((i * self.stride) as isize) = x;
                }
                Ok(())
            }

            pub fn ptr(&mut self, i: usize) -> Result<*mut $element_type, GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 208, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe { Ok(self.data.offset((i * self.stride) as isize)) }
            }

            pub fn const_ptr(&self, i: usize) -> Result<*const $element_type, GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 221, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe { Ok(self.data.offset((i * self.stride) as isize) as *const _) }
            }
        }
    };
}

macro_rules! impl_complex_vector_ops {
    ($vector_type:ty, $complex_type:ty, $element_type:ty, $error_msg:expr, $header:expr) => {
        impl $vector_type {
            pub fn get(&self, i: usize) -> Result<$complex_type, GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 199, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe {
                    Ok(*(self.data
                        .offset((2 * i * self.stride) as isize) as *mut $complex_type))
                }
            }

            pub fn set(&mut self, i: usize, z: $complex_type) -> Result<(), GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 213, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe {
                    *(self.data
                        .offset((2 * i * self.stride) as isize) as *mut $complex_type) = z;
                }
                Ok(())
            }

            pub fn ptr(&mut self, i: usize) -> Result<*mut $complex_type, GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 227, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe {
                    Ok(self.data
                        .offset((2 * i * self.stride) as isize) as *mut $complex_type)
                }
            }

            pub fn const_ptr(&self, i: usize) -> Result<*const $complex_type, GslError> {
                if unsafe { GSL_CHECK_RANGE != 0 } && i >= self.size {
                    gsl_error($error_msg, $header, 241, GslError::Einval);
                    return Err(GslError::Einval);
                }
                unsafe {
                    Ok(self.data
                        .offset((2 * i * self.stride) as isize) as *const $complex_type)
                }
            }
        }
    };
}

impl_vector_ops!(
    gsl_vector_long_double,
    f128::f128,
    "index out of range",
    "../gsl/gsl_vector_long_double.h"
);

impl_complex_vector_ops!(
    gsl_vector_complex_long_double,
    gsl_complex_long_double,
    f128::f128,
    "index out of range",
    "../gsl/gsl_vector_complex_long_double.h"
);

impl_vector_ops!(
    gsl_vector,
    c_double,
    "index out of range",
    "../gsl/gsl_vector_double.h"
);

impl_complex_vector_ops!(
    gsl_vector_complex,
    gsl_complex,
    c_double,
    "index out of range",
    "../gsl/gsl_vector_complex_double.h"
);

impl_vector_ops!(
    gsl_vector_float,
    c_float,
    "index out of range",
    "../gsl/gsl_vector_float.h"
);

impl_complex_vector_ops!(
    gsl_vector_complex_float,
    gsl_complex_float,
    c_float,
    "index out of range",
    "../gsl/gsl_vector_complex_float.h"
);

impl_vector_ops!(
    gsl_vector_char,
    c_char,
    "index out of range",
    "../gsl/gsl_vector_char.h"
);

impl_vector_ops!(
    gsl_vector_ulong,
    c_ulong,
    "index out of range",
    "../gsl/gsl_vector_ulong.h"
);

impl_vector_ops!(
    gsl_vector_long,
    c_long,
    "index out of range",
    "../gsl/gsl_vector_long.h"
);

impl_vector_ops!(
    gsl_vector_uint,
    c_uint,
    "index out of range",
    "../gsl/gsl_vector_uint.h"
);

impl_vector_ops!(
    gsl_vector_int,
    c_int,
    "index out of range",
    "../gsl/gsl_vector_int.h"
);

impl_vector_ops!(
    gsl_vector_ushort,
    c_ushort,
    "index out of range",
    "../gsl/gsl_vector_ushort.h"
);

impl_vector_ops!(
    gsl_vector_short,
    c_short,
    "index out of range",
    "../gsl/gsl_vector_short.h"
);

impl_vector_ops!(
    gsl_vector_uchar,
    c_uchar,
    "index out of range",
    "../gsl/gsl_vector_uchar.h"
);
use libc::{c_char, c_int, c_uint, c_ulong, c_double, c_float, c_long, c_ulong, c_short, c_ushort, c_char, c_uchar};
use std::ptr;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_long_double {
    pub size: usize,
    pub data: *mut f128::f128,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_long_double {
    pub size: usize,
    pub stride: usize,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_complex_long_double {
    pub size: usize,
    pub data: *mut f128::f128,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_complex_long_double {
    pub size: usize,
    pub stride: usize,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_complex_long_double {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_complex {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_complex {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_complex {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_float {
    pub size: usize,
    pub data: *mut c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_float {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_float,
    pub block: *mut gsl_block_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_complex_float {
    pub size: usize,
    pub data: *mut c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_complex_float {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_complex_float {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_long_double {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_float {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_float,
    pub block: *mut gsl_block_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_ulong {
    pub size: usize,
    pub data: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_ulong {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_ulong {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_long {
    pub size: usize,
    pub data: *mut c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_long {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_long,
    pub block: *mut gsl_block_long,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_long {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_long,
    pub block: *mut gsl_block_long,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_uint {
    pub size: usize,
    pub data: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_uint {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_uint {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_int {
    pub size: usize,
    pub data: *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_int {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_int,
    pub block: *mut gsl_block_int,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_int {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_int,
    pub block: *mut gsl_block_int,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_ushort {
    pub size: usize,
    pub data: *mut c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_ushort {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_ushort {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_short {
    pub size: usize,
    pub data: *mut c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_short {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_short,
    pub block: *mut gsl_block_short,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_short {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_short,
    pub block: *mut gsl_block_short,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_uchar {
    pub size: usize,
    pub data: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_uchar {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_uchar {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_block_char {
    pub size: usize,
    pub data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_vector_char {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_char,
    pub block: *mut gsl_block_char,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsl_matrix_char {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
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

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
}

fn check_matrix_vector_dimensions<T>(matrix: &gsl_matrix, vector: &gsl_vector) -> Result<(), GslError> {
    if matrix.size1 != vector.size {
        unsafe {
            gsl_error(
                b"matrix row size and vector length are not equal\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                22,
                GslError::Ebadlen as c_int,
            );
        }
        Err(GslError::Ebadlen)
    } else {
        Ok(())
    }
}

fn check_matrix_index<T>(matrix: &gsl_matrix, index: usize) -> Result<(), GslError> {
    if index >= matrix.size1 {
        unsafe {
            gsl_error(
                b"row index is out of range\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                16,
                GslError::Einval as c_int,
            );
        }
        Err(GslError::Einval)
    } else {
        Ok(())
    }
}

pub fn gsl_matrix_get_row(
    v: &mut gsl_vector,
    m: &gsl_matrix,
    i: usize,
) -> Result<(), GslError> {
    check_matrix_index(m, i)?;
    check_matrix_vector_dimensions(m, v)?;

    unsafe {
        let v_data = v.data;
        let row_data = m.data.add(i * m.tda);
        let stride = v.stride;

        for j in 0..m.size2 {
            *v_data.add(j * stride) = *row_data.add(j);
        }
    }

    Ok(())
}

pub fn gsl_matrix_set_row(
    m: &mut gsl_matrix,
    i: usize,
    v: &gsl_vector,
) -> Result<(), GslError> {
    check_matrix_index(m, i)?;
    check_matrix_vector_dimensions(m, v)?;

    unsafe {
        let v_data = v.data;
        let row_data = m.data.add(i * m.tda);
        let stride = v.stride;

        for j in 0..m.size2 {
            *row_data.add(j) = *v_data.add(j * stride);
        }
    }

    Ok(())
}

pub fn gsl_matrix_get_col(
    v: &mut gsl_vector,
    m: &gsl_matrix,
    j: usize,
) -> Result<(), GslError> {
    if j >= m.size2 {
        unsafe {
            gsl_error(
                b"column index is out of range\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                57,
                GslError::Einval as c_int,
            );
        }
        return Err(GslError::Einval);
    }

    if v.size != m.size1 {
        unsafe {
            gsl_error(
                b"matrix column size and vector length are not equal\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                63,
                GslError::Ebadlen as c_int,
            );
        }
        return Err(GslError::Ebadlen);
    }

    unsafe {
        let v_data = v.data;
        let column_data = m.data.add(j);
        let stride = v.stride;
        let tda = m.tda;

        for i in 0..m.size1 {
            *v_data.add(i * stride) = *column_data.add(i * tda);
        }
    }

    Ok(())
}

pub fn gsl_matrix_set_col(
    m: &mut gsl_matrix,
    j: usize,
    v: &gsl_vector,
) -> Result<(), GslError> {
    if j >= m.size2 {
        unsafe {
            gsl_error(
                b"column index is out of range\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                140,
                GslError::Einval as c_int,
            );
        }
        return Err(GslError::Einval);
    }

    if v.size != m.size1 {
        unsafe {
            gsl_error(
                b"matrix column size and vector length are not equal\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                146,
                GslError::Ebadlen as c_int,
            );
        }
        return Err(GslError::Ebadlen);
    }

    unsafe {
        let v_data = v.data;
        let column_data = m.data.add(j);
        let stride = v.stride;
        let tda = m.tda;

        for i in 0..m.size1 {
            *column_data.add(i * tda) = *v_data.add(i * stride);
        }
    }

    Ok(())
}

pub fn gsl_vector_alloc_row_from_matrix(
    m: &gsl_matrix,
    i: usize,
) -> Result<gsl_vector, GslError> {
    if i >= m.size1 {
        unsafe {
            gsl_error(
                b"row index is out of range\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                181,
                GslError::Einval as c_int,
            );
        }
        return Err(GslError::Einval);
    }

    let v = unsafe {
        let ptr = libc::malloc(mem::size_of::<gsl_vector>()) as *mut gsl_vector;
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for vector struct\0".as_ptr() as *const c_char,
                b"./getset_source.c\0".as_ptr() as *const c_char,
                189,
                GslError::Enomem as c_int,
            );
            return Err(GslError::Enom
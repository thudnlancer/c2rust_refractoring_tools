#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn malloc(_: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut u64,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ulong {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u64,
    pub block: *mut gsl_block_ulong,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ulong {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut u64,
    pub block: *mut gsl_block_ulong,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut i64,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i64,
    pub block: *mut gsl_block_long,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut i64,
    pub block: *mut gsl_block_long,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut u32,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u32,
    pub block: *mut gsl_block_uint,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uint {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut u32,
    pub block: *mut gsl_block_uint,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut i32,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i32,
    pub block: *mut gsl_block_int,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_int {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut i32,
    pub block: *mut gsl_block_int,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ushort {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ushort {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_short {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_short {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut u8,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uchar {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u8,
    pub block: *mut gsl_block_uchar,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uchar {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut u8,
    pub block: *mut gsl_block_uchar,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut i8,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_char {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i8,
    pub block: *mut gsl_block_char,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut i8,
    pub block: *mut gsl_block_char,
    pub owner: i32,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_get_row(
    mut v: *mut gsl_vector_complex,
    mut m: *const gsl_matrix_complex,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_double = (*v).data;
    let mut row_data: *const libc::c_double = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (2 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_get_row(
    mut v: *mut gsl_vector_char,
    mut m: *const gsl_matrix_char,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut i8 = (*v).data;
    let mut row_data: *const i8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_get_row(
    mut v: *mut gsl_vector_ulong,
    mut m: *const gsl_matrix_ulong,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut u64 = (*v).data;
    let mut row_data: *const u64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_get_row(
    mut v: *mut gsl_vector_int,
    mut m: *const gsl_matrix_int,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut i32 = (*v).data;
    let mut row_data: *const i32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_get_row(
    mut v: *mut gsl_vector_complex_float,
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_float = (*v).data;
    let mut row_data: *const libc::c_float = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (2 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_get_row(
    mut v: *mut gsl_vector_long_double,
    mut m: *const gsl_matrix_long_double,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut f128::f128 = (*v).data;
    let mut row_data: *const f128::f128 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_get_row(
    mut v: *mut gsl_vector_uint,
    mut m: *const gsl_matrix_uint,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut u32 = (*v).data;
    let mut row_data: *const u32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_get_row(
    mut v: *mut gsl_vector_ushort,
    mut m: *const gsl_matrix_ushort,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_ushort = (*v).data;
    let mut row_data: *const libc::c_ushort = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_get_row(
    mut v: *mut gsl_vector_short,
    mut m: *const gsl_matrix_short,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_short = (*v).data;
    let mut row_data: *const libc::c_short = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_get_row(
    mut v: *mut gsl_vector_complex_long_double,
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut f128::f128 = (*v).data;
    let mut row_data: *const f128::f128 = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (2 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_get_row(
    mut v: *mut gsl_vector_float,
    mut m: *const gsl_matrix_float,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_float = (*v).data;
    let mut row_data: *const libc::c_float = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_get_row(
    mut v: *mut gsl_vector,
    mut m: *const gsl_matrix,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_double = (*v).data;
    let mut row_data: *const libc::c_double = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_get_row(
    mut v: *mut gsl_vector_long,
    mut m: *const gsl_matrix_long,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut i64 = (*v).data;
    let mut row_data: *const i64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_get_row(
    mut v: *mut gsl_vector_uchar,
    mut m: *const gsl_matrix_uchar,
    i: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            16 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            22 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut u8 = (*v).data;
    let mut row_data: *const u8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                ) = *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_get_col(
    mut v: *mut gsl_vector_ulong,
    mut m: *const gsl_matrix_ulong,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut u64 = (*v).data;
    let mut column_data: *const u64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_get_col(
    mut v: *mut gsl_vector_char,
    mut m: *const gsl_matrix_char,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut i8 = (*v).data;
    let mut column_data: *const i8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_get_col(
    mut v: *mut gsl_vector_complex_long_double,
    mut m: *const gsl_matrix_complex_long_double,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut f128::f128 = (*v).data;
    let mut column_data: *const f128::f128 = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_get_col(
    mut v: *mut gsl_vector_complex,
    mut m: *const gsl_matrix_complex,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_double = (*v).data;
    let mut column_data: *const libc::c_double = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_get_col(
    mut v: *mut gsl_vector_long,
    mut m: *const gsl_matrix_long,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut i64 = (*v).data;
    let mut column_data: *const i64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_get_col(
    mut v: *mut gsl_vector_uint,
    mut m: *const gsl_matrix_uint,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut u32 = (*v).data;
    let mut column_data: *const u32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_get_col(
    mut v: *mut gsl_vector_ushort,
    mut m: *const gsl_matrix_ushort,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_ushort = (*v).data;
    let mut column_data: *const libc::c_ushort = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_get_col(
    mut v: *mut gsl_vector_float,
    mut m: *const gsl_matrix_float,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_float = (*v).data;
    let mut column_data: *const libc::c_float = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_get_col(
    mut v: *mut gsl_vector_uchar,
    mut m: *const gsl_matrix_uchar,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut u8 = (*v).data;
    let mut column_data: *const u8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_get_col(
    mut v: *mut gsl_vector,
    mut m: *const gsl_matrix,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_double = (*v).data;
    let mut column_data: *const libc::c_double = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_get_col(
    mut v: *mut gsl_vector_int,
    mut m: *const gsl_matrix_int,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut i32 = (*v).data;
    let mut column_data: *const i32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_get_col(
    mut v: *mut gsl_vector_long_double,
    mut m: *const gsl_matrix_long_double,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut f128::f128 = (*v).data;
    let mut column_data: *const f128::f128 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_get_col(
    mut v: *mut gsl_vector_complex_float,
    mut m: *const gsl_matrix_complex_float,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_float = (*v).data;
    let mut column_data: *const libc::c_float = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_get_col(
    mut v: *mut gsl_vector_short,
    mut m: *const gsl_matrix_short,
    j: size_t,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            63 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *mut libc::c_short = (*v).data;
    let mut column_data: *const libc::c_short = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *v_data
                .offset(
                    stride
                        .wrapping_mul(1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                ) = *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_set_row(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    mut v: *const gsl_vector_uint,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const u32 = (*v).data;
    let mut row_data: *mut u32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_set_row(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    mut v: *const gsl_vector_float,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_float = (*v).data;
    let mut row_data: *mut libc::c_float = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_set_row(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    mut v: *const gsl_vector_long,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const i64 = (*v).data;
    let mut row_data: *mut i64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_set_row(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    mut v: *const gsl_vector_ushort,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_ushort = (*v).data;
    let mut row_data: *mut libc::c_ushort = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_set_row(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    mut v: *const gsl_vector_char,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const i8 = (*v).data;
    let mut row_data: *mut i8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_set_row(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    mut v: *const gsl_vector_complex_float,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_float = (*v).data;
    let mut row_data: *mut libc::c_float = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *row_data
                .offset(
                    (2 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_set_row(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    mut v: *const gsl_vector_complex_long_double,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const f128::f128 = (*v).data;
    let mut row_data: *mut f128::f128 = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *row_data
                .offset(
                    (2 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_set_row(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    mut v: *const gsl_vector_ulong,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const u64 = (*v).data;
    let mut row_data: *mut u64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_set_row(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    mut v: *const gsl_vector_uchar,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const u8 = (*v).data;
    let mut row_data: *mut u8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_set_row(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    mut v: *const gsl_vector_long_double,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const f128::f128 = (*v).data;
    let mut row_data: *mut f128::f128 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_set_row(
    mut m: *mut gsl_matrix,
    i: size_t,
    mut v: *const gsl_vector,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_double = (*v).data;
    let mut row_data: *mut libc::c_double = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_set_row(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    mut v: *const gsl_vector_int,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const i32 = (*v).data;
    let mut row_data: *mut i32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_set_row(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    mut v: *const gsl_vector_complex,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_double = (*v).data;
    let mut row_data: *mut libc::c_double = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *row_data
                .offset(
                    (2 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_set_row(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    mut v: *const gsl_vector_short,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != N {
        gsl_error(
            b"matrix row size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            105 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_short = (*v).data;
    let mut row_data: *mut libc::c_short = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul(tda) as isize);
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < N {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *row_data
                .offset(
                    (1 as i32 as u64).wrapping_mul(j).wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_set_col(
    mut m: *mut gsl_matrix_long_double,
    j: size_t,
    mut v: *const gsl_vector_long_double,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const f128::f128 = (*v).data;
    let mut column_data: *mut f128::f128 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_set_col(
    mut m: *mut gsl_matrix,
    j: size_t,
    mut v: *const gsl_vector,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_double = (*v).data;
    let mut column_data: *mut libc::c_double = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_set_col(
    mut m: *mut gsl_matrix_uint,
    j: size_t,
    mut v: *const gsl_vector_uint,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const u32 = (*v).data;
    let mut column_data: *mut u32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_set_col(
    mut m: *mut gsl_matrix_long,
    j: size_t,
    mut v: *const gsl_vector_long,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const i64 = (*v).data;
    let mut column_data: *mut i64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_set_col(
    mut m: *mut gsl_matrix_short,
    j: size_t,
    mut v: *const gsl_vector_short,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_short = (*v).data;
    let mut column_data: *mut libc::c_short = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_set_col(
    mut m: *mut gsl_matrix_complex,
    j: size_t,
    mut v: *const gsl_vector_complex,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_double = (*v).data;
    let mut column_data: *mut libc::c_double = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *column_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_set_col(
    mut m: *mut gsl_matrix_float,
    j: size_t,
    mut v: *const gsl_vector_float,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_float = (*v).data;
    let mut column_data: *mut libc::c_float = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_set_col(
    mut m: *mut gsl_matrix_complex_long_double,
    j: size_t,
    mut v: *const gsl_vector_complex_long_double,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const f128::f128 = (*v).data;
    let mut column_data: *mut f128::f128 = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *column_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_set_col(
    mut m: *mut gsl_matrix_char,
    j: size_t,
    mut v: *const gsl_vector_char,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const i8 = (*v).data;
    let mut column_data: *mut i8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_set_col(
    mut m: *mut gsl_matrix_uchar,
    j: size_t,
    mut v: *const gsl_vector_uchar,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const u8 = (*v).data;
    let mut column_data: *mut u8 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_set_col(
    mut m: *mut gsl_matrix_int,
    j: size_t,
    mut v: *const gsl_vector_int,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const i32 = (*v).data;
    let mut column_data: *mut i32 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_set_col(
    mut m: *mut gsl_matrix_ulong,
    j: size_t,
    mut v: *const gsl_vector_ulong,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const u64 = (*v).data;
    let mut column_data: *mut u64 = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_set_col(
    mut m: *mut gsl_matrix_complex_float,
    j: size_t,
    mut v: *const gsl_vector_complex_float,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_float = (*v).data;
    let mut column_data: *mut libc::c_float = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 2 as i32 as u32 {
            *column_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (2 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_set_col(
    mut m: *mut gsl_matrix_ushort,
    j: size_t,
    mut v: *const gsl_vector_ushort,
) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            140 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*v).size != M {
        gsl_error(
            b"matrix column size and vector length are not equal\0" as *const u8
                as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            146 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut v_data: *const libc::c_ushort = (*v).data;
    let mut column_data: *mut libc::c_ushort = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(j) as isize);
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        let mut k: u32 = 0;
        k = 0 as i32 as u32;
        while k < 1 as i32 as u32 {
            *column_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(i)
                        .wrapping_mul(tda)
                        .wrapping_add(k as u64) as isize,
                ) = *v_data
                .offset(
                    (1 as i32 as u64)
                        .wrapping_mul(stride)
                        .wrapping_mul(i)
                        .wrapping_add(k as u64) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_ulong_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
) -> *mut gsl_vector_ulong {
    let mut v: *mut gsl_vector_ulong = 0 as *mut gsl_vector_ulong;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_ulong>() as u64)
        as *mut gsl_vector_ulong;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_ulong;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_long_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_long,
    i: size_t,
) -> *mut gsl_vector_long {
    let mut v: *mut gsl_vector_long = 0 as *mut gsl_vector_long;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_long;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_long>() as u64) as *mut gsl_vector_long;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_long;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_long;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_uint_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
) -> *mut gsl_vector_uint {
    let mut v: *mut gsl_vector_uint = 0 as *mut gsl_vector_uint;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_uint;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_uint>() as u64) as *mut gsl_vector_uint;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_uint;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_uint;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_complex_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
) -> *mut gsl_vector_complex {
    let mut v: *mut gsl_vector_complex = 0 as *mut gsl_vector_complex;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_complex;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_complex>() as u64)
        as *mut gsl_vector_complex;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_complex;
    }
    (*v).data = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_complex;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_complex_long_double_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
) -> *mut gsl_vector_complex_long_double {
    let mut v: *mut gsl_vector_complex_long_double = 0
        as *mut gsl_vector_complex_long_double;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_complex_long_double>() as u64)
        as *mut gsl_vector_complex_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    (*v).data = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_complex_long_double;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_uchar_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
) -> *mut gsl_vector_uchar {
    let mut v: *mut gsl_vector_uchar = 0 as *mut gsl_vector_uchar;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_uchar>() as u64)
        as *mut gsl_vector_uchar;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_uchar;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_complex_float_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
) -> *mut gsl_vector_complex_float {
    let mut v: *mut gsl_vector_complex_float = 0 as *mut gsl_vector_complex_float;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_complex_float>() as u64)
        as *mut gsl_vector_complex_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    (*v).data = ((*m).data)
        .offset((2 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_complex_float;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_short_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_short,
    i: size_t,
) -> *mut gsl_vector_short {
    let mut v: *mut gsl_vector_short = 0 as *mut gsl_vector_short;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_short;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_short>() as u64)
        as *mut gsl_vector_short;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_short;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_short;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_char_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_char,
    i: size_t,
) -> *mut gsl_vector_char {
    let mut v: *mut gsl_vector_char = 0 as *mut gsl_vector_char;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_char;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_char>() as u64) as *mut gsl_vector_char;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_char;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_char;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_int_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_int,
    i: size_t,
) -> *mut gsl_vector_int {
    let mut v: *mut gsl_vector_int = 0 as *mut gsl_vector_int;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_int;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_int>() as u64) as *mut gsl_vector_int;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_int;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_int;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_long_double_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
) -> *mut gsl_vector_long_double {
    let mut v: *mut gsl_vector_long_double = 0 as *mut gsl_vector_long_double;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_long_double>() as u64)
        as *mut gsl_vector_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_long_double;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_float_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_float,
    i: size_t,
) -> *mut gsl_vector_float {
    let mut v: *mut gsl_vector_float = 0 as *mut gsl_vector_float;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_float;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_float>() as u64)
        as *mut gsl_vector_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_float;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_float;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_ushort_alloc_row_from_matrix(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
) -> *mut gsl_vector_ushort {
    let mut v: *mut gsl_vector_ushort = 0 as *mut gsl_vector_ushort;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_ushort>() as u64)
        as *mut gsl_vector_ushort;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block_ushort;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_alloc_row_from_matrix(
    mut m: *mut gsl_matrix,
    i: size_t,
) -> *mut gsl_vector {
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    let M: size_t = (*m).size1;
    if i >= M {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            181 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector;
    }
    v = malloc(::core::mem::size_of::<gsl_vector>() as u64) as *mut gsl_vector;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            189 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector;
    }
    (*v).data = ((*m).data)
        .offset((1 as i32 as u64).wrapping_mul(i).wrapping_mul((*m).tda) as isize);
    (*v).size = (*m).size2;
    (*v).stride = 1 as i32 as size_t;
    (*v).block = 0 as *mut gsl_block;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_complex_float_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_complex_float,
    j: size_t,
) -> *mut gsl_vector_complex_float {
    let mut v: *mut gsl_vector_complex_float = 0 as *mut gsl_vector_complex_float;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_complex_float>() as u64)
        as *mut gsl_vector_complex_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    (*v).data = ((*m).data).offset((2 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_complex_float;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_ushort_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_ushort,
    j: size_t,
) -> *mut gsl_vector_ushort {
    let mut v: *mut gsl_vector_ushort = 0 as *mut gsl_vector_ushort;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_ushort>() as u64)
        as *mut gsl_vector_ushort;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_ushort;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_long_double_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_long_double,
    j: size_t,
) -> *mut gsl_vector_long_double {
    let mut v: *mut gsl_vector_long_double = 0 as *mut gsl_vector_long_double;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_long_double>() as u64)
        as *mut gsl_vector_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_long_double;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_uint_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_uint,
    j: size_t,
) -> *mut gsl_vector_uint {
    let mut v: *mut gsl_vector_uint = 0 as *mut gsl_vector_uint;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_uint;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_uint>() as u64) as *mut gsl_vector_uint;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_uint;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_uint;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_long_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_long,
    j: size_t,
) -> *mut gsl_vector_long {
    let mut v: *mut gsl_vector_long = 0 as *mut gsl_vector_long;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_long;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_long>() as u64) as *mut gsl_vector_long;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_long;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_long;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_int_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_int,
    j: size_t,
) -> *mut gsl_vector_int {
    let mut v: *mut gsl_vector_int = 0 as *mut gsl_vector_int;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_int;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_int>() as u64) as *mut gsl_vector_int;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_int;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_int;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_short_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_short,
    j: size_t,
) -> *mut gsl_vector_short {
    let mut v: *mut gsl_vector_short = 0 as *mut gsl_vector_short;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_short;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_short>() as u64)
        as *mut gsl_vector_short;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_short;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_short;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_complex_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_complex,
    j: size_t,
) -> *mut gsl_vector_complex {
    let mut v: *mut gsl_vector_complex = 0 as *mut gsl_vector_complex;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_complex;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_complex>() as u64)
        as *mut gsl_vector_complex;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_complex;
    }
    (*v).data = ((*m).data).offset((2 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_complex;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_char_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_char,
    j: size_t,
) -> *mut gsl_vector_char {
    let mut v: *mut gsl_vector_char = 0 as *mut gsl_vector_char;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_char;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_char>() as u64) as *mut gsl_vector_char;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_char;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_char;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_complex_long_double_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_complex_long_double,
    j: size_t,
) -> *mut gsl_vector_complex_long_double {
    let mut v: *mut gsl_vector_complex_long_double = 0
        as *mut gsl_vector_complex_long_double;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_complex_long_double>() as u64)
        as *mut gsl_vector_complex_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    (*v).data = ((*m).data).offset((2 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_complex_long_double;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_float_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_float,
    j: size_t,
) -> *mut gsl_vector_float {
    let mut v: *mut gsl_vector_float = 0 as *mut gsl_vector_float;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_float;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_float>() as u64)
        as *mut gsl_vector_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_float;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_float;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_uchar_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_uchar,
    j: size_t,
) -> *mut gsl_vector_uchar {
    let mut v: *mut gsl_vector_uchar = 0 as *mut gsl_vector_uchar;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_uchar>() as u64)
        as *mut gsl_vector_uchar;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_uchar;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_ulong_alloc_col_from_matrix(
    mut m: *mut gsl_matrix_ulong,
    j: size_t,
) -> *mut gsl_vector_ulong {
    let mut v: *mut gsl_vector_ulong = 0 as *mut gsl_vector_ulong;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    v = malloc(::core::mem::size_of::<gsl_vector_ulong>() as u64)
        as *mut gsl_vector_ulong;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block_ulong;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_vector_alloc_col_from_matrix(
    mut m: *mut gsl_matrix,
    j: size_t,
) -> *mut gsl_vector {
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    let N: size_t = (*m).size2;
    if j >= N {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            210 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_vector;
    }
    v = malloc(::core::mem::size_of::<gsl_vector>() as u64) as *mut gsl_vector;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8 as *const i8,
            b"./getset_source.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_vector;
    }
    (*v).data = ((*m).data).offset((1 as i32 as u64).wrapping_mul(j) as isize);
    (*v).size = (*m).size1;
    (*v).stride = (*m).tda;
    (*v).block = 0 as *mut gsl_block;
    return v;
}
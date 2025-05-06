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
    fn gsl_matrix_float_const_column(
        m: *const gsl_matrix_float,
        j: size_t,
    ) -> _gsl_vector_float_const_view;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_matrix_long_double_const_column(
        m: *const gsl_matrix_long_double,
        j: size_t,
    ) -> _gsl_vector_long_double_const_view;
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_long_const_column(
        m: *const gsl_matrix_long,
        j: size_t,
    ) -> _gsl_vector_long_const_view;
    fn gsl_matrix_int_const_column(
        m: *const gsl_matrix_int,
        j: size_t,
    ) -> _gsl_vector_int_const_view;
    fn gsl_matrix_short_const_column(
        m: *const gsl_matrix_short,
        j: size_t,
    ) -> _gsl_vector_short_const_view;
    fn gsl_matrix_char_const_column(
        m: *const gsl_matrix_char,
        j: size_t,
    ) -> _gsl_vector_char_const_view;
    fn gsl_blas_sasum(X: *const gsl_vector_float) -> libc::c_float;
    fn gsl_blas_dasum(X: *const gsl_vector) -> libc::c_double;
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
pub struct _gsl_vector_long_double_const_view {
    pub vector: gsl_vector_long_double,
}
pub type gsl_vector_long_double_const_view = _gsl_vector_long_double_const_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
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
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
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
pub struct _gsl_vector_float_const_view {
    pub vector: gsl_vector_float,
}
pub type gsl_vector_float_const_view = _gsl_vector_float_const_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
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
pub struct _gsl_vector_long_const_view {
    pub vector: gsl_vector_long,
}
pub type gsl_vector_long_const_view = _gsl_vector_long_const_view;
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
pub struct _gsl_vector_int_const_view {
    pub vector: gsl_vector_int,
}
pub type gsl_vector_int_const_view = _gsl_vector_int_const_view;
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
pub struct _gsl_vector_short_const_view {
    pub vector: gsl_vector_short,
}
pub type gsl_vector_short_const_view = _gsl_vector_short_const_view;
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
pub struct _gsl_vector_char_const_view {
    pub vector: gsl_vector_char,
}
pub type gsl_vector_char_const_view = _gsl_vector_char_const_view;
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
#[inline]
unsafe extern "C" fn gsl_vector_long_double_get(
    mut v: *const gsl_vector_long_double,
    i: size_t,
) -> f128::f128 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_long_get(
    mut v: *const gsl_vector_long,
    i: size_t,
) -> i64 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_int_get(mut v: *const gsl_vector_int, i: size_t) -> i32 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_short_get(
    mut v: *const gsl_vector_short,
    i: size_t,
) -> libc::c_short {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_char_get(
    mut v: *const gsl_vector_char,
    i: size_t,
) -> i8 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_equal(
    mut a: *const gsl_matrix_long_double,
    mut b: *const gsl_matrix_long_double,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_equal(
    mut a: *const gsl_matrix_short,
    mut b: *const gsl_matrix_short,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        ) as i32
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            ) as i32
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_equal(
    mut a: *const gsl_matrix_complex_long_double,
    mut b: *const gsl_matrix_complex_long_double,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 2 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_equal(
    mut a: *const gsl_matrix_ulong,
    mut b: *const gsl_matrix_ulong,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_equal(
    mut a: *const gsl_matrix_float,
    mut b: *const gsl_matrix_float,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_equal(
    mut a: *const gsl_matrix_uchar,
    mut b: *const gsl_matrix_uchar,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        ) as i32
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            ) as i32
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_equal(
    mut a: *const gsl_matrix,
    mut b: *const gsl_matrix,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_equal(
    mut a: *const gsl_matrix_ushort,
    mut b: *const gsl_matrix_ushort,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        ) as i32
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            ) as i32
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_equal(
    mut a: *const gsl_matrix_long,
    mut b: *const gsl_matrix_long,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_equal(
    mut a: *const gsl_matrix_int,
    mut b: *const gsl_matrix_int,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_equal(
    mut a: *const gsl_matrix_uint,
    mut b: *const gsl_matrix_uint,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_equal(
    mut a: *const gsl_matrix_complex_float,
    mut b: *const gsl_matrix_complex_float,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 2 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_equal(
    mut a: *const gsl_matrix_complex,
    mut b: *const gsl_matrix_complex,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 2 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(k) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            )
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_equal(
    mut a: *const gsl_matrix_char,
    mut b: *const gsl_matrix_char,
) -> i32 {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const i8,
            b"./prop_source.c\0" as *const u8 as *const i8,
            28 as i32,
            GSL_EBADLEN as i32,
        );
        return 0 as i32;
    } else {
        let tda_a: size_t = (*a).tda;
        let tda_b: size_t = (*b).tda;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as i32 as size_t;
        while i < M {
            j = 0 as i32 as size_t;
            while j < N {
                k = 0 as i32 as size_t;
                while k < 1 as i32 as u64 {
                    if *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(j)
                                .wrapping_mul(1 as i32 as u64)
                                .wrapping_add(k) as isize,
                        ) as i32
                        != *((*b).data)
                            .offset(
                                i
                                    .wrapping_mul(tda_b)
                                    .wrapping_add(j)
                                    .wrapping_mul(1 as i32 as u64)
                                    .wrapping_add(k) as isize,
                            ) as i32
                    {
                        return 0 as i32;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_isnull(
    mut m: *const gsl_matrix_ushort,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_isnull(mut m: *const gsl_matrix_long) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_isnull(mut m: *const gsl_matrix_uchar) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_isnull(
    mut m: *const gsl_matrix_complex_float,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_isnull(mut m: *const gsl_matrix_int) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_isnull(
    mut m: *const gsl_matrix_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) != f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_isnull(
    mut m: *const gsl_matrix_complex_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) != f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_isnull(mut m: *const gsl_matrix_float) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_isnull(mut m: *const gsl_matrix_uint) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_isnull(mut m: *const gsl_matrix_char) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_isnull(
    mut m: *const gsl_matrix_complex,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_isnull(mut m: *const gsl_matrix_ulong) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_isnull(mut m: *const gsl_matrix) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_isnull(mut m: *const gsl_matrix_short) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double != 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_ispos(mut m: *const gsl_matrix_uint) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_ispos(mut m: *const gsl_matrix_int) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_ispos(mut m: *const gsl_matrix_short) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ispos(mut m: *const gsl_matrix) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_ispos(mut m: *const gsl_matrix_ulong) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_ispos(mut m: *const gsl_matrix_long) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_ispos(mut m: *const gsl_matrix_uchar) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_ispos(
    mut m: *const gsl_matrix_ushort,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_ispos(mut m: *const gsl_matrix_char) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_ispos(
    mut m: *const gsl_matrix_complex,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_ispos(mut m: *const gsl_matrix_float) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_ispos(
    mut m: *const gsl_matrix_complex_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) <= f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_ispos(
    mut m: *const gsl_matrix_complex_float,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double <= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_ispos(
    mut m: *const gsl_matrix_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) <= f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_isneg(
    mut m: *const gsl_matrix_complex_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) >= f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_isneg(mut m: *const gsl_matrix_short) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_isneg(
    mut m: *const gsl_matrix_complex,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_isneg(mut m: *const gsl_matrix_char) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_isneg(mut m: *const gsl_matrix_float) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_isneg(
    mut m: *const gsl_matrix_ushort,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_isneg(mut m: *const gsl_matrix) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_isneg(mut m: *const gsl_matrix_int) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_isneg(mut m: *const gsl_matrix_ulong) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_isneg(mut m: *const gsl_matrix_long) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_isneg(
    mut m: *const gsl_matrix_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) >= f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_isneg(
    mut m: *const gsl_matrix_complex_float,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_isneg(mut m: *const gsl_matrix_uint) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_isneg(mut m: *const gsl_matrix_uchar) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double >= 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_isnonneg(mut m: *const gsl_matrix_char) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_isnonneg(
    mut m: *const gsl_matrix_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) < f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_isnonneg(
    mut m: *const gsl_matrix_uchar,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_float_isnonneg(
    mut m: *const gsl_matrix_complex_float,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_isnonneg(mut m: *const gsl_matrix_uint) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_isnonneg(mut m: *const gsl_matrix) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_isnonneg(
    mut m: *const gsl_matrix_short,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_isnonneg(
    mut m: *const gsl_matrix_complex,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_isnonneg(
    mut m: *const gsl_matrix_ushort,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as i32 as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_isnonneg(
    mut m: *const gsl_matrix_float,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_isnonneg(
    mut m: *const gsl_matrix_ulong,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_complex_long_double_isnonneg(
    mut m: *const gsl_matrix_complex_long_double,
) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 2 as i32 as u64 {
                if *((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) < f128::f128::new(0.0f64)
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_isnonneg(mut m: *const gsl_matrix_int) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_isnonneg(mut m: *const gsl_matrix_long) -> i32 {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size1 {
        j = 0 as i32 as size_t;
        while j < size2 {
            k = 0 as i32 as size_t;
            while k < 1 as i32 as u64 {
                if (*((*m).data)
                    .offset(
                        i
                            .wrapping_mul(tda)
                            .wrapping_add(j)
                            .wrapping_mul(1 as i32 as u64)
                            .wrapping_add(k) as isize,
                    ) as libc::c_double) < 0.0f64
                {
                    return 0 as i32;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_norm1(
    mut m: *const gsl_matrix_short,
) -> libc::c_short {
    let mut value: libc::c_short = 0 as i32 as libc::c_short;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_short_const_view = gsl_matrix_short_const_column(m, j);
        let mut sum: libc::c_short = 0;
        let mut i: size_t = 0;
        sum = 0 as i32 as libc::c_short;
        i = 0 as i32 as size_t;
        while i < (*m).size1 {
            let mut mij: libc::c_short = gsl_vector_short_get(&mj.vector, i);
            if mij as i32 >= 0 as i32 as libc::c_short as i32 {
                sum = (sum as i32 + mij as i32) as libc::c_short;
            } else {
                sum = (sum as i32 + -(mij as i32)) as libc::c_short;
            }
            i = i.wrapping_add(1);
            i;
        }
        if sum as i32 > value as i32 {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_norm1(
    mut m: *const gsl_matrix_long_double,
) -> f128::f128 {
    let mut value: f128::f128 = f128::f128::new(0 as i32);
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_long_double_const_view = gsl_matrix_long_double_const_column(
            m,
            j,
        );
        let mut sum: f128::f128 = f128::f128::ZERO;
        let mut i: size_t = 0;
        sum = f128::f128::new(0 as i32);
        i = 0 as i32 as size_t;
        while i < (*m).size1 {
            let mut mij: f128::f128 = gsl_vector_long_double_get(&mj.vector, i);
            if mij >= f128::f128::new(0 as i32) {
                sum += mij;
            } else {
                sum += -mij;
            }
            i = i.wrapping_add(1);
            i;
        }
        if sum > value {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_norm1(mut m: *const gsl_matrix) -> libc::c_double {
    let mut value: libc::c_double = 0 as i32 as libc::c_double;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_const_view = gsl_matrix_const_column(m, j);
        let mut sum: libc::c_double = 0.;
        sum = gsl_blas_dasum(&mj.vector);
        if sum > value {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_norm1(
    mut m: *const gsl_matrix_float,
) -> libc::c_float {
    let mut value: libc::c_float = 0 as i32 as libc::c_float;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_float_const_view = gsl_matrix_float_const_column(m, j);
        let mut sum: libc::c_float = 0.;
        sum = gsl_blas_sasum(&mj.vector);
        if sum > value {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_norm1(mut m: *const gsl_matrix_int) -> i32 {
    let mut value: i32 = 0 as i32;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_int_const_view = gsl_matrix_int_const_column(m, j);
        let mut sum: i32 = 0;
        let mut i: size_t = 0;
        sum = 0 as i32;
        i = 0 as i32 as size_t;
        while i < (*m).size1 {
            let mut mij: i32 = gsl_vector_int_get(&mj.vector, i);
            if mij >= 0 as i32 {
                sum += mij;
            } else {
                sum += -mij;
            }
            i = i.wrapping_add(1);
            i;
        }
        if sum > value {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_norm1(mut m: *const gsl_matrix_char) -> i8 {
    let mut value: i8 = 0 as i32 as i8;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_char_const_view = gsl_matrix_char_const_column(m, j);
        let mut sum: i8 = 0;
        let mut i: size_t = 0;
        sum = 0 as i32 as i8;
        i = 0 as i32 as size_t;
        while i < (*m).size1 {
            let mut mij: i8 = gsl_vector_char_get(&mj.vector, i);
            if mij as i32 >= 0 as i32 as i8 as i32 {
                sum = (sum as i32 + mij as i32) as i8;
            } else {
                sum = (sum as i32 + -(mij as i32)) as i8;
            }
            i = i.wrapping_add(1);
            i;
        }
        if sum as i32 > value as i32 {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_norm1(mut m: *const gsl_matrix_long) -> i64 {
    let mut value: i64 = 0 as i32 as i64;
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < (*m).size2 {
        let mj: gsl_vector_long_const_view = gsl_matrix_long_const_column(m, j);
        let mut sum: i64 = 0;
        let mut i: size_t = 0;
        sum = 0 as i32 as i64;
        i = 0 as i32 as size_t;
        while i < (*m).size1 {
            let mut mij: i64 = gsl_vector_long_get(&mj.vector, i);
            if mij >= 0 as i32 as i64 {
                sum += mij;
            } else {
                sum += -mij;
            }
            i = i.wrapping_add(1);
            i;
        }
        if sum > value {
            value = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return value;
}
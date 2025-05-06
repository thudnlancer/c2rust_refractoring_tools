#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use num_traits::ToPrimitive;
extern "C" {
    fn __isnanl(__value: f128::f128) -> i32;
    fn __isnan(__value: libc::c_double) -> i32;
    fn __isnanf(__value: libc::c_float) -> i32;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
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
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut i8,
    pub block: *mut gsl_block_char,
    pub owner: i32,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_max(mut m: *const gsl_matrix_long) -> i64 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_max(
    mut m: *const gsl_matrix_long_double,
) -> f128::f128 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: f128::f128 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            if if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x.to_f32().unwrap())
            } else if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x.to_f64().unwrap())
            } else {
                __isnanl(x)
            } != 0
            {
                return x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_max(
    mut m: *const gsl_matrix_ushort,
) -> libc::c_ushort {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_ushort = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_max(mut m: *const gsl_matrix) -> libc::c_double {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_double = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            if if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x as libc::c_float)
            } else if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                return x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_max(mut m: *const gsl_matrix_int) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_max(
    mut m: *const gsl_matrix_short,
) -> libc::c_short {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_short = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_max(mut m: *const gsl_matrix_ulong) -> u64 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_max(
    mut m: *const gsl_matrix_float,
) -> libc::c_float {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_float = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            if if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x)
            } else if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x as libc::c_double)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                return x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_max(mut m: *const gsl_matrix_uint) -> u32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_max(mut m: *const gsl_matrix_char) -> i8 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_max(mut m: *const gsl_matrix_uchar) -> u8 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_min(mut m: *const gsl_matrix_uchar) -> u8 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_min(
    mut m: *const gsl_matrix_long_double,
) -> f128::f128 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: f128::f128 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x.to_f32().unwrap())
            } else if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x.to_f64().unwrap())
            } else {
                __isnanl(x)
            } != 0
            {
                return x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_min(
    mut m: *const gsl_matrix_ushort,
) -> libc::c_ushort {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_ushort = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_min(mut m: *const gsl_matrix_uint) -> u32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_min(mut m: *const gsl_matrix_long) -> i64 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_min(mut m: *const gsl_matrix_char) -> i8 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_min(
    mut m: *const gsl_matrix_float,
) -> libc::c_float {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_float = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x)
            } else if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x as libc::c_double)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                return x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_min(
    mut m: *const gsl_matrix_short,
) -> libc::c_short {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_short = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_min(mut m: *const gsl_matrix_ulong) -> u64 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_min(mut m: *const gsl_matrix_int) -> i32 {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_min(mut m: *const gsl_matrix) -> libc::c_double {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_double = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x as libc::c_float)
            } else if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                return x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_minmax(
    mut m: *const gsl_matrix_uint,
    mut min_out: *mut u32,
    mut max_out: *mut u32,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_minmax(
    mut m: *const gsl_matrix_short,
    mut min_out: *mut libc::c_short,
    mut max_out: *mut libc::c_short,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_short = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_minmax(
    mut m: *const gsl_matrix_long_double,
    mut min_out: *mut f128::f128,
    mut max_out: *mut f128::f128,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: f128::f128 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            if if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x.to_f32().unwrap())
            } else if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x.to_f64().unwrap())
            } else {
                __isnanl(x)
            } != 0
            {
                *min_out = x;
                *max_out = x;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_minmax(
    mut m: *const gsl_matrix_int,
    mut min_out: *mut i32,
    mut max_out: *mut i32,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_minmax(
    mut m: *const gsl_matrix_ulong,
    mut min_out: *mut u64,
    mut max_out: *mut u64,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_minmax(
    mut m: *const gsl_matrix_long,
    mut min_out: *mut i64,
    mut max_out: *mut i64,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_minmax(
    mut m: *const gsl_matrix,
    mut min_out: *mut libc::c_double,
    mut max_out: *mut libc::c_double,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_double = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            if if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x as libc::c_float)
            } else if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *min_out = x;
                *max_out = x;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_minmax(
    mut m: *const gsl_matrix_ushort,
    mut min_out: *mut libc::c_ushort,
    mut max_out: *mut libc::c_ushort,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_ushort = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_minmax(
    mut m: *const gsl_matrix_float,
    mut min_out: *mut libc::c_float,
    mut max_out: *mut libc::c_float,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_float = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            if if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x)
            } else if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x as libc::c_double)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *min_out = x;
                *max_out = x;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_minmax(
    mut m: *const gsl_matrix_uchar,
    mut min_out: *mut u8,
    mut max_out: *mut u8,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_minmax(
    mut m: *const gsl_matrix_char,
    mut min_out: *mut i8,
    mut max_out: *mut i8,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
            }
            if x as i32 > max as i32 {
                max = x;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_max_index(
    mut m: *const gsl_matrix_long,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_max_index(
    mut m: *const gsl_matrix,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_double = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            if if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x as libc::c_float)
            } else if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *imax_out = i;
                *jmax_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_max_index(
    mut m: *const gsl_matrix_short,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_short = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_max_index(
    mut m: *const gsl_matrix_float,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_float = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            if if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x)
            } else if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x as libc::c_double)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *imax_out = i;
                *jmax_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_max_index(
    mut m: *const gsl_matrix_ushort,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_ushort = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_max_index(
    mut m: *const gsl_matrix_char,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_max_index(
    mut m: *const gsl_matrix_ulong,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_max_index(
    mut m: *const gsl_matrix_uint,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_max_index(
    mut m: *const gsl_matrix_uchar,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_max_index(
    mut m: *const gsl_matrix_int,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_max_index(
    mut m: *const gsl_matrix_long_double,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut max: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: f128::f128 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            if if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x.to_f32().unwrap())
            } else if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x.to_f64().unwrap())
            } else {
                __isnanl(x)
            } != 0
            {
                *imax_out = i;
                *jmax_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_min_index(
    mut m: *const gsl_matrix_long_double,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: f128::f128 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x.to_f32().unwrap())
            } else if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x.to_f64().unwrap())
            } else {
                __isnanl(x)
            } != 0
            {
                *imin_out = i;
                *jmin_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_min_index(
    mut m: *const gsl_matrix_int,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_min_index(
    mut m: *const gsl_matrix_ulong,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_min_index(
    mut m: *const gsl_matrix_uchar,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_min_index(
    mut m: *const gsl_matrix_short,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_short = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_min_index(
    mut m: *const gsl_matrix_float,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_float = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x)
            } else if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x as libc::c_double)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *imin_out = i;
                *jmin_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_min_index(
    mut m: *const gsl_matrix_char,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_min_index(
    mut m: *const gsl_matrix_ushort,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_ushort = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_min_index(
    mut m: *const gsl_matrix_long,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_min_index(
    mut m: *const gsl_matrix_uint,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_min_index(
    mut m: *const gsl_matrix,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut min: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_double = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x as libc::c_float)
            } else if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *imin_out = i;
                *jmin_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_short_minmax_index(
    mut m: *const gsl_matrix_short,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_short = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_short = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_minmax_index(
    mut m: *const gsl_matrix,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_double = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_double = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            if if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x as libc::c_float)
            } else if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *imin_out = i;
                *jmin_out = j;
                *imax_out = i;
                *jmax_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_char_minmax_index(
    mut m: *const gsl_matrix_char,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: i8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_int_minmax_index(
    mut m: *const gsl_matrix_int,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: i32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_float_minmax_index(
    mut m: *const gsl_matrix_float,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_float = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_float = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            if if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x)
            } else if ::core::mem::size_of::<libc::c_float>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x as libc::c_double)
            } else {
                __isnanl(f128::f128::new(x))
            } != 0
            {
                *imin_out = i;
                *jmin_out = j;
                *imax_out = i;
                *jmax_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_double_minmax_index(
    mut m: *const gsl_matrix_long_double,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: f128::f128 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: f128::f128 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            if if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_float>() as u64
            {
                __isnanf(x.to_f32().unwrap())
            } else if ::core::mem::size_of::<f128::f128>() as u64
                == ::core::mem::size_of::<libc::c_double>() as u64
            {
                __isnan(x.to_f64().unwrap())
            } else {
                __isnanl(x)
            } != 0
            {
                *imin_out = i;
                *jmin_out = j;
                *imax_out = i;
                *jmax_out = j;
                return;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uint_minmax_index(
    mut m: *const gsl_matrix_uint,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: u32 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u32 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_long_minmax_index(
    mut m: *const gsl_matrix_long,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: i64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: i64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_uchar_minmax_index(
    mut m: *const gsl_matrix_uchar,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: u8 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u8 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ushort_minmax_index(
    mut m: *const gsl_matrix_ushort,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: libc::c_ushort = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: libc::c_ushort = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if (x as i32) < min as i32 {
                min = x;
                imin = i;
                jmin = j;
            }
            if x as i32 > max as i32 {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_matrix_ulong_minmax_index(
    mut m: *const gsl_matrix_ulong,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let M: size_t = (*m).size1;
    let N: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let mut imin: size_t = 0 as i32 as size_t;
    let mut jmin: size_t = 0 as i32 as size_t;
    let mut imax: size_t = 0 as i32 as size_t;
    let mut jmax: size_t = 0 as i32 as size_t;
    let mut max: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut min: u64 = *((*m).data)
        .offset(
            (0 as i32 as u64).wrapping_mul(tda).wrapping_add(0 as i32 as u64) as isize,
        );
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < M {
        j = 0 as i32 as size_t;
        while j < N {
            let mut x: u64 = *((*m).data)
                .offset(i.wrapping_mul(tda).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
    *imax_out = imax;
    *jmax_out = jmax;
}
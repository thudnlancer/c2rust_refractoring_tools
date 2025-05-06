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
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_mean(
    mut data: *const u64,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_mean(
    mut data: *const u8,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize) as i32)
                - mean) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_mean(
    mut data: *const u32,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_mean(
    mut data: *const f128::f128,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (*data.offset(i.wrapping_mul(stride) as isize) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_mean(
    mut data: *const libc::c_double,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_mean(
    mut data: *const libc::c_float,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_mean(
    mut data: *const i8,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize) as i32)
                - mean) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_mean(
    mut data: *const i64,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_mean(
    mut data: *const i32,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_mean(
    mut data: *const libc::c_ushort,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize) as i32)
                - mean) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_mean(
    mut data: *const libc::c_short,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize) as i32)
                - mean) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
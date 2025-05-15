/* This Rust translation provides equivalent functionality to the C math.h header,
   following Rust's safety practices and using standard Rust libraries. */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]

use std::f32;
use std::f64;

pub const HUGE_VALF: f32 = f32::INFINITY;
pub const HUGE_VAL: f64 = f64::INFINITY;
pub const HUGE_VALL: f64 = f64::INFINITY;

pub const FP_ILOGB0: i32 = i32::MIN;
pub const FP_ILOGBNAN: i32 = i32::MIN;

#[inline]
pub fn acosf(x: f32) -> f32 {
    x.acos()
}

#[inline]
pub fn acos(x: f64) -> f64 {
    x.acos()
}

#[inline]
pub fn asinf(x: f32) -> f32 {
    x.asin()
}

#[inline]
pub fn asin(x: f64) -> f64 {
    x.asin()
}

#[inline]
pub fn atanf(x: f32) -> f32 {
    x.atan()
}

#[inline]
pub fn atan(x: f64) -> f64 {
    x.atan()
}

#[inline]
pub fn atan2f(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

#[inline]
pub fn cbrtf(x: f32) -> f32 {
    x.cbrt()
}

#[inline]
pub fn cbrt(x: f64) -> f64 {
    x.cbrt()
}

#[inline]
pub fn ceilf(x: f32) -> f32 {
    x.ceil()
}

#[inline]
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

#[inline]
pub fn copysignf(x: f32, y: f32) -> f32 {
    x.copysign(y)
}

#[inline]
pub fn copysign(x: f64, y: f64) -> f64 {
    x.copysign(y)
}

#[inline]
pub fn cosf(x: f32) -> f32 {
    x.cos()
}

#[inline]
pub fn cos(x: f64) -> f64 {
    x.cos()
}

#[inline]
pub fn coshf(x: f32) -> f32 {
    x.cosh()
}

#[inline]
pub fn expf(x: f32) -> f32 {
    x.exp()
}

#[inline]
pub fn exp(x: f64) -> f64 {
    x.exp()
}

#[inline]
pub fn exp2f(x: f32) -> f32 {
    x.exp2()
}

#[inline]
pub fn exp2(x: f64) -> f64 {
    x.exp2()
}

#[inline]
pub fn expm1f(x: f32) -> f32 {
    x.exp_m1()
}

#[inline]
pub fn expm1(x: f64) -> f64 {
    x.exp_m1()
}

#[inline]
pub fn fabsf(x: f32) -> f32 {
    x.abs()
}

#[inline]
pub fn fabs(x: f64) -> f64 {
    x.abs()
}

#[inline]
pub fn floorf(x: f32) -> f32 {
    x.floor()
}

#[inline]
pub fn floor(x: f64) -> f64 {
    x.floor()
}

#[inline]
pub fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    x.mul_add(y, z)
}

#[inline]
pub fn fma(x: f64, y: f64, z: f64) -> f64 {
    x.mul_add(y, z)
}

#[inline]
pub fn fmodf(x: f32, y: f32) -> f32 {
    x % y
}

#[inline]
pub fn fmod(x: f64, y: f64) -> f64 {
    x % y
}

#[inline]
pub fn frexpf(x: f32) -> (f32, i32) {
    x.frexp()
}

#[inline]
pub fn frexp(x: f64) -> (f64, i32) {
    x.frexp()
}

#[inline]
pub fn hypotf(x: f32, y: f32) -> f32 {
    x.hypot(y)
}

#[inline]
pub fn hypot(x: f64, y: f64) -> f64 {
    x.hypot(y)
}

#[inline]
pub fn ilogbf(x: f32) -> i32 {
    x.log2() as i32
}

#[inline]
pub fn ilogb(x: f64) -> i32 {
    x.log2() as i32
}

#[inline]
pub fn ldexpf(x: f32, exp: i32) -> f32 {
    x.ldexp(exp)
}

#[inline]
pub fn ldexp(x: f64, exp: i32) -> f64 {
    x.ldexp(exp)
}

#[inline]
pub fn logf(x: f32) -> f32 {
    x.ln()
}

#[inline]
pub fn log(x: f64) -> f64 {
    x.ln()
}

#[inline]
pub fn log10f(x: f32) -> f32 {
    x.log10()
}

#[inline]
pub fn log10(x: f64) -> f64 {
    x.log10()
}

#[inline]
pub fn log1pf(x: f32) -> f32 {
    x.ln_1p()
}

#[inline]
pub fn log1p(x: f64) -> f64 {
    x.ln_1p()
}

#[inline]
pub fn log2f(x: f32) -> f32 {
    x.log2()
}

#[inline]
pub fn log2(x: f64) -> f64 {
    x.log2()
}

#[inline]
pub fn logbf(x: f32) -> f32 {
    x.log2()
}

#[inline]
pub fn logb(x: f64) -> f64 {
    x.log2()
}

#[inline]
pub fn modff(x: f32) -> (f32, f32) {
    x.fract()
}

#[inline]
pub fn modf(x: f64) -> (f64, f64) {
    x.fract()
}

#[inline]
pub fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[inline]
pub fn remainderf(x: f32, y: f32) -> f32 {
    x.rem_euclid(y)
}

#[inline]
pub fn remainder(x: f64, y: f64) -> f64 {
    x.rem_euclid(y)
}

#[inline]
pub fn rintf(x: f32) -> f32 {
    x.round()
}

#[inline]
pub fn rint(x: f64) -> f64 {
    x.round()
}

#[inline]
pub fn roundf(x: f32) -> f32 {
    x.round()
}

#[inline]
pub fn round(x: f64) -> f64 {
    x.round()
}

#[inline]
pub fn sinf(x: f32) -> f32 {
    x.sin()
}

#[inline]
pub fn sin(x: f64) -> f64 {
    x.sin()
}

#[inline]
pub fn sinhf(x: f32) -> f32 {
    x.sinh()
}

#[inline]
pub fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

#[inline]
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[inline]
pub fn tanf(x: f32) -> f32 {
    x.tan()
}

#[inline]
pub fn tan(x: f64) -> f64 {
    x.tan()
}

#[inline]
pub fn tanhf(x: f32) -> f32 {
    x.tanh()
}

#[inline]
pub fn truncf(x: f32) -> f32 {
    x.trunc()
}

#[inline]
pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

#[inline]
pub fn isfinite(x: f64) -> bool {
    x.is_finite()
}

#[inline]
pub fn isinf(x: f64) -> bool {
    x.is_infinite()
}

#[inline]
pub fn isnan(x: f64) -> bool {
    x.is_nan()
}

#[inline]
pub fn signbit(x: f64) -> bool {
    x.is_sign_negative()
}
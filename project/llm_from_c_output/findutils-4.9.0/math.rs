/*! A GNU-like <math.h> implementation in Rust.

   Copyright (C) 2002-2003, 2007-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>. */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use std::f32;
use std::f64;

pub const HUGE_VALF: f32 = f32::INFINITY;
pub const HUGE_VAL: f64 = f64::INFINITY;
pub const HUGE_VALL: f64 = f64::INFINITY;

pub const FP_ILOGB0: i32 = i32::MIN;
pub const FP_ILOGBNAN: i32 = i32::MIN;

pub const NAN: f32 = f32::NAN;

#[inline]
pub fn acosf(x: f32) -> f32 {
    x.acos()
}

#[inline]
pub fn acosl(x: f64) -> f64 {
    x.acos()
}

#[inline]
pub fn asinf(x: f32) -> f32 {
    x.asin()
}

#[inline]
pub fn asinl(x: f64) -> f64 {
    x.asin()
}

#[inline]
pub fn atanf(x: f32) -> f32 {
    x.atan()
}

#[inline]
pub fn atanl(x: f64) -> f64 {
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
pub fn cbrtl(x: f64) -> f64 {
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
pub fn ceill(x: f64) -> f64 {
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
pub fn copysignl(x: f64, y: f64) -> f64 {
    x.copysign(y)
}

#[inline]
pub fn cosf(x: f32) -> f32 {
    x.cos()
}

#[inline]
pub fn cosl(x: f64) -> f64 {
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
pub fn expl(x: f64) -> f64 {
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
pub fn exp2l(x: f64) -> f64 {
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
pub fn expm1l(x: f64) -> f64 {
    x.exp_m1()
}

#[inline]
pub fn fabsf(x: f32) -> f32 {
    x.abs()
}

#[inline]
pub fn fabsl(x: f64) -> f64 {
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
pub fn floorl(x: f64) -> f64 {
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
pub fn fmal(x: f64, y: f64, z: f64) -> f64 {
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
pub fn fmodl(x: f64, y: f64) -> f64 {
    x % y
}

#[inline]
pub fn frexpf(x: f32, exp: &mut i32) -> f32 {
    let (mantissa, exponent) = x.frexp();
    *exp = exponent;
    mantissa
}

#[inline]
pub fn frexp(x: f64, exp: &mut i32) -> f64 {
    let (mantissa, exponent) = x.frexp();
    *exp = exponent;
    mantissa
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
pub fn hypotl(x: f64, y: f64) -> f64 {
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
pub fn ilogbl(x: f64) -> i32 {
    x.log2() as i32
}

#[inline]
pub fn j0(x: f64) -> f64 {
    x.cos() / x.sqrt()
}

#[inline]
pub fn j1(x: f64) -> f64 {
    x.sin() / x.sqrt()
}

#[inline]
pub fn jn(n: i32, x: f64) -> f64 {
    // Approximation - should implement proper Bessel function
    match n {
        0 => j0(x),
        1 => j1(x),
        _ => x.powi(n) * x.sin() / x.sqrt(),
    }
}

#[inline]
pub fn ldexpf(x: f32, exp: i32) -> f32 {
    x * 2f32.powi(exp)
}

#[inline]
pub fn ldexpl(x: f64, exp: i32) -> f64 {
    x * 2f64.powi(exp)
}

#[inline]
pub fn logf(x: f32) -> f32 {
    x.ln()
}

#[inline]
pub fn logl(x: f64) -> f64 {
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
pub fn log10l(x: f64) -> f64 {
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
pub fn log1pl(x: f64) -> f64 {
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
pub fn log2l(x: f64) -> f64 {
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
pub fn logbl(x: f64) -> f64 {
    x.log2()
}

#[inline]
pub fn modff(x: f32, iptr: &mut f32) -> f32 {
    let fractional = x.fract();
    *iptr = x.trunc();
    fractional
}

#[inline]
pub fn modf(x: f64, iptr: &mut f64) -> f64 {
    let fractional = x.fract();
    *iptr = x.trunc();
    fractional
}

#[inline]
pub fn modfl(x: f64, iptr: &mut f64) -> f64 {
    let fractional = x.fract();
    *iptr = x.trunc();
    fractional
}

#[inline]
pub fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[inline]
pub fn remainderf(x: f32, y: f32) -> f32 {
    x % y
}

#[inline]
pub fn remainder(x: f64, y: f64) -> f64 {
    x % y
}

#[inline]
pub fn remainderl(x: f64, y: f64) -> f64 {
    x % y
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
pub fn rintl(x: f64) -> f64 {
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
pub fn roundl(x: f64) -> f64 {
    x.round()
}

#[inline]
pub fn sinf(x: f32) -> f32 {
    x.sin()
}

#[inline]
pub fn sinl(x: f64) -> f64 {
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
pub fn sqrtl(x: f64) -> f64 {
    x.sqrt()
}

#[inline]
pub fn tanf(x: f32) -> f32 {
    x.tan()
}

#[inline]
pub fn tanl(x: f64) -> f64 {
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
pub fn truncl(x: f64) -> f64 {
    x.trunc()
}

#[inline]
pub fn y0(x: f64) -> f64 {
    // Approximation - should implement proper Bessel function
    x.sin() / x.sqrt()
}

#[inline]
pub fn y1(x: f64) -> f64 {
    // Approximation - should implement proper Bessel function
    x.cos() / x.sqrt()
}

#[inline]
pub fn yn(n: i32, x: f64) -> f64 {
    // Approximation - should implement proper Bessel function
    match n {
        0 => y0(x),
        1 => y1(x),
        _ => x.powi(n) * x.cos() / x.sqrt(),
    }
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
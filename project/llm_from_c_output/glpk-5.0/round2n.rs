// round2n.rs (round floating-point number to nearest power of two)

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

/// Rounds a positive floating-point number to the nearest power of two.
///
/// Given a positive floating-point value `x`, returns `2^n` such that `|x - 2^n|` is minimal.
///
/// # Examples
///
/// ```
/// assert_eq!(round2n(10.1), 8.0);    // 2^3 = 8
/// assert_eq!(round2n(15.3), 16.0);   // 2^4 = 16
/// assert_eq!(round2n(0.01), 0.0078125); // 2^(-7) = 0.0078125
/// ```
///
/// # Panics
///
/// Panics if `x` is not positive.
///
/// # Background
///
/// Let `x = f * 2^e`, where `0.5 <= f < 1` is a normalized fractional part,
/// `e` is an integer exponent. Then, obviously, `0.5 * 2^e <= x < 2^e`, so
/// if `x - 0.5 * 2^e <= 2^e - x`, we choose `0.5 * 2^e = 2^(e-1)`, and `2^e`
/// otherwise. The latter condition can be written as `2 * x <= 1.5 * 2^e`
/// or `2 * f * 2^e <= 1.5 * 2^e` or, finally, `f <= 0.75`.
pub fn round2n(x: f64) -> f64 {
    assert!(x > 0.0, "x must be positive");
    let (f, e) = frexp(x);
    ldexp(1.0, if f <= 0.75 { e - 1 } else { e })
}

/// Splits a floating-point number into a normalized fraction and an exponent.
/// Returns `(fraction, exponent)` such that `x = fraction * 2^exponent`,
/// with the absolute value of `fraction` in the interval `[0.5, 1.0)`.
fn frexp(x: f64) -> (f64, i32) {
    if x == 0.0 {
        (0.0, 0)
    } else {
        let bits = x.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i32 - 1022;
        let fraction = f64::from_bits((bits & 0x800f_ffff_ffff_ffff) | 0x3fe0_0000_0000_0000);
        (fraction, exponent)
    }
}

/// Multiplies a floating-point number by an integral power of 2.
/// Returns `x * 2^exp`.
fn ldexp(x: f64, exp: i32) -> f64 {
    x * (2.0f64.powi(exp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round2n() {
        assert_eq!(round2n(10.1), 8.0);
        assert_eq!(round2n(15.3), 16.0);
        assert_eq!(round2n(0.01), 0.0078125);
    }

    #[test]
    #[should_panic(expected = "x must be positive")]
    fn test_round2n_non_positive() {
        round2n(0.0);
    }
}
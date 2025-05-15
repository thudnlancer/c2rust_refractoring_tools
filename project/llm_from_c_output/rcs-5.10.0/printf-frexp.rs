// Split a double into fraction and mantissa, for hexadecimal printf.
// Copyright (C) 2007, 2009-2020 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Write a finite, positive number x as
//   x = mantissa * 2^exp
// where exp >= DBL_MIN_EXP - 1,
//       mantissa < 2.0,
//       if x is not a denormalized number then mantissa >= 1.0.
// Store exp in *EXPPTR and return mantissa.

/// Splits a double into fraction and mantissa, similar to C's printf_frexp.
/// Returns a tuple of (mantissa, exponent).
pub fn printf_frexp(x: f64) -> (f64, i32) {
    let mut exponent = 0;
    let mut x = x;

    // Use frexp and ldexp if available (faster path)
    {
        let (mantissa, exp) = x.frexp();
        x = mantissa * 2.0;
        exponent = exp - 1;

        if exponent < f64::MIN_EXP - 1 {
            x = x.ldexp(exponent - (f64::MIN_EXP - 1));
            exponent = f64::MIN_EXP - 1;
        }
    }

    (x, exponent)
}

/// Splits a long double into fraction and mantissa, similar to C's printf_frexpl.
/// Returns a tuple of (mantissa, exponent).
#[cfg(feature = "long_double")]
pub fn printf_frexpl(x: f128) -> (f128, i32) {
    let mut exponent = 0;
    let mut x = x;

    // Use frexp and ldexp if available (faster path)
    {
        let (mantissa, exp) = x.frexp();
        x = mantissa * 2.0;
        exponent = exp - 1;

        if exponent < f128::MIN_EXP - 1 {
            x = x.ldexp(exponent - (f128::MIN_EXP - 1));
            exponent = f128::MIN_EXP - 1;
        }
    }

    (x, exponent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printf_frexp() {
        let (m, e) = printf_frexp(3.14);
        assert!(m >= 1.0 && m < 2.0);
        assert_eq!(3.14, m * 2.0f64.powi(e));

        let (m, e) = printf_frexp(0.1);
        assert!(m >= 0.5 && m < 1.0);
        assert_eq!(0.1, m * 2.0f64.powi(e));

        let (m, e) = printf_frexp(f64::MIN_POSITIVE);
        assert!(m >= 0.5 && m < 1.0);
        assert_eq!(f64::MIN_POSITIVE, m * 2.0f64.powi(e));
    }

    #[test]
    #[cfg(feature = "long_double")]
    fn test_printf_frexpl() {
        let (m, e) = printf_frexpl(3.14);
        assert!(m >= 1.0 && m < 2.0);
        assert_eq!(3.14, m * 2.0f128.powi(e));

        let (m, e) = printf_frexpl(0.1);
        assert!(m >= 0.5 && m < 1.0);
        assert_eq!(0.1, m * 2.0f128.powi(e));

        let (m, e) = printf_frexpl(f128::MIN_POSITIVE);
        assert!(m >= 0.5 && m < 1.0);
        assert_eq!(f128::MIN_POSITIVE, m * 2.0f128.powi(e));
    }
}
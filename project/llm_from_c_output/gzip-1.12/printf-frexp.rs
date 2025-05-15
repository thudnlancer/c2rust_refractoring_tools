//! Split a double into fraction and mantissa, for hexadecimal printf.
//!
//! This module provides functionality to write a finite, positive number x as:
//!     x = mantissa * 2^exp
//! where exp >= DBL_MIN_EXP - 1,
//!       mantissa < 2.0,
//!       if x is not a denormalized number then mantissa >= 1.0.
//! The exp is stored in the provided pointer and mantissa is returned.

/// Splits a double into fraction and mantissa.
///
/// # Arguments
/// * `x` - A finite, positive number to split
/// * `expptr` - Pointer to store the exponent value
///
/// # Returns
/// The mantissa component of the number.
pub fn printf_frexp(x: f64, expptr: &mut i32) -> f64 {
    let mut exponent = 0;
    let mut x = x;

    if x >= 1.0 {
        // A nonnegative exponent
        let mut pow2_i = 2.0;
        let mut powh_i = 0.5;
        let mut i = 0;

        loop {
            if x >= pow2_i {
                exponent += 1 << i;
                x *= powh_i;
                pow2_i *= pow2_i;
                powh_i *= powh_i;
                i += 1;
            } else {
                break;
            }
        }
    } else {
        // A negative exponent
        let mut pow2_i = 2.0;
        let mut powh_i = 0.5;
        let mut i = 0;
        let min_exp = f64::MIN_EXP as i32 - 1;

        loop {
            if exponent - (1 << i) < min_exp {
                break;
            }

            exponent -= 1 << i;
            x *= pow2_i;
            if x >= 1.0 {
                break;
            }

            pow2_i *= pow2_i;
            powh_i *= powh_i;
            i += 1;
        }

        if x < 1.0 {
            while i > 0 {
                i -= 1;
                if exponent - (1 << i) >= min_exp {
                    exponent -= 1 << i;
                    x *= pow2_i.powi(2);
                    if x >= 1.0 {
                        break;
                    }
                }
            }
        }
    }

    // Final adjustment
    let mut pow2_i = 2.0;
    let mut powh_i = 0.5;
    let mut i = 0;
    while x >= pow2_i {
        exponent += 1 << i;
        x *= powh_i;
        pow2_i *= pow2_i;
        powh_i *= powh_i;
        i += 1;
    }

    *expptr = exponent;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printf_frexp() {
        let mut exp = 0;
        let mantissa = printf_frexp(3.14, &mut exp);
        assert!(mantissa >= 1.0 && mantissa < 2.0);
        assert_eq!(mantissa * 2.0f64.powi(exp), 3.14);

        let mut exp = 0;
        let mantissa = printf_frexp(0.123, &mut exp);
        assert!(mantissa >= 0.5 && mantissa < 1.0);
        assert_eq!(mantissa * 2.0f64.powi(exp), 0.123);

        let mut exp = 0;
        let mantissa = printf_frexp(1.0, &mut exp);
        assert_eq!(mantissa, 1.0);
        assert_eq!(exp, 0);
    }
}
/// Convert floating-point number to rational number
///
/// Given a floating-point number 0 <= x < 1, finds its "best" rational approximation p / q,
/// where p >= 0 and q > 0 are integer numbers, such that |x - p / q| <= eps.
///
/// # Arguments
/// * `x` - The floating-point number to convert (0 <= x < 1)
/// * `eps` - The desired precision
///
/// # Returns
/// A tuple containing (p, q, iterations) where:
/// * p - The numerator of the rational approximation
/// * q - The denominator of the rational approximation
/// * iterations - The number of iterations used to achieve the specified precision
///
/// # Panics
/// Panics if x is not in the range [0, 1) or if the algorithm fails to converge within 100 iterations
pub fn fp2rat(x: f64, eps: f64) -> (f64, f64, i32) {
    assert!(0.0 <= x && x < 1.0, "x must be in range [0, 1)");

    let mut k = 0;
    let mut xk = x;
    let mut akm1 = 1.0;
    let mut ak = 0.0;
    let mut bkm1 = 0.0;
    let mut bk = 1.0;

    loop {
        assert!(k <= 100, "failed to converge within 100 iterations");

        let (new_ak, new_bk) = if k == 0 {
            // x[0] = x
            // A[-1] = 1
            // A[0] = b[0] = floor(x[0]) = 0
            // B[-1] = 0
            // B[0] = 1
            (ak, bk)
        } else {
            // x[k] = 1 / frac(x[k-1])
            let temp = xk - xk.floor();
            assert!(temp != 0.0, "division by zero in continued fraction");
            xk = 1.0 / temp;

            // a[k] = 1
            let ak_coeff = 1.0;
            // b[k] = floor(x[k])
            let bk_coeff = xk.floor();

            // A[k] = b[k] * A[k-1] + a[k] * A[k-2]
            let new_ak = bk_coeff * ak + ak_coeff * akm1;
            // B[k] = b[k] * B[k-1] + a[k] * B[k-2]
            let new_bk = bk_coeff * bk + ak_coeff * bkm1;

            akm1 = ak;
            bkm1 = bk;
            (new_ak, new_bk)
        };

        ak = new_ak;
        bk = new_bk;

        // f[k] = A[k] / B[k]
        let fk = ak / bk;

        if (x - fk).abs() <= eps {
            break;
        }

        k += 1;
    }

    (ak, bk, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp2rat() {
        // Example from the original C code
        let x = std::f64::consts::SQRT_2 - 1.0;
        let eps = 1e-6;
        let (p, q, _) = fp2rat(x, eps);
        assert_eq!(p, 408.0);
        assert_eq!(q, 985.0);
    }

    #[test]
    #[should_panic(expected = "x must be in range [0, 1)")]
    fn test_invalid_input() {
        fp2rat(-0.1, 1e-6);
    }

    #[test]
    #[should_panic(expected = "x must be in range [0, 1)")]
    fn test_input_ge_1() {
        fp2rat(1.0, 1e-6);
    }
}
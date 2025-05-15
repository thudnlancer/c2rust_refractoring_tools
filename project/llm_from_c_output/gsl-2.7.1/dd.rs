/* interpolation/interp_poly.rs

 * Translated from C to Rust while maintaining the same functionality
 * and following Rust's safety practices.
 */

/// Represents the success status for the polynomial operations
const GSL_SUCCESS: i32 = 0;

/// Computes Newton's divided differences for polynomial interpolation
pub fn gsl_poly_dd_init(
    dd: &mut [f64],
    xa: &[f64],
    ya: &[f64],
    size: usize,
) -> i32 {
    // Newton's divided differences

    if dd.is_empty() || xa.is_empty() || ya.is_empty() {
        return GSL_SUCCESS;
    }

    dd[0] = ya[0];

    for j in (1..size).rev() {
        dd[j] = (ya[j] - ya[j - 1]) / (xa[j] - xa[j - 1]);
    }

    for i in 2..size {
        for j in (i..size).rev() {
            dd[j] = (dd[j] - dd[j - 1]) / (xa[j] - xa[j - i]);
        }
    }

    GSL_SUCCESS
}

/// Computes Taylor expansion coefficients using divided differences
pub fn gsl_poly_dd_taylor(
    c: &mut [f64],
    xp: f64,
    dd: &[f64],
    xa: &[f64],
    size: usize,
    w: &mut [f64],
) -> i32 {
    for i in 0..size {
        c[i] = 0.0;
        w[i] = 0.0;
    }

    if size == 0 {
        return GSL_SUCCESS;
    }

    w[size - 1] = 1.0;
    c[0] = dd[0];

    for i in (0..size - 1).rev() {
        w[i] = -w[i + 1] * (xa[size - 2 - i] - xp);

        for j in i + 1..size - 1 {
            w[j] = w[j] - w[j + 1] * (xa[size - 2 - i] - xp);
        }

        for j in i..size {
            c[j - i] += w[j] * dd[size - i - 1];
        }
    }

    GSL_SUCCESS
}

/// Computes divided difference representation for Hermite polynomial interpolation
pub fn gsl_poly_dd_hermite_init(
    dd: &mut [f64],
    za: &mut [f64],
    xa: &[f64],
    ya: &[f64],
    dya: &[f64],
    size: usize,
) -> i32 {
    let n = 2 * size;

    if dd.is_empty() || za.is_empty() || xa.is_empty() || ya.is_empty() || dya.is_empty() {
        return GSL_SUCCESS;
    }

    // Hermite divided differences
    dd[0] = ya[0];

    // compute: dd[j] = f[z_{j-1},z_j] for j \in [1,N-1]
    for j in 0..size {
        za[2 * j] = xa[j];
        za[2 * j + 1] = xa[j];

        if j != 0 {
            dd[2 * j] = (ya[j] - ya[j - 1]) / (xa[j] - xa[j - 1]);
            dd[2 * j - 1] = dya[j - 1];
        }
    }

    if n == 0 {
        return GSL_SUCCESS;
    }

    dd[n - 1] = dya[size - 1];

    for i in 2..n {
        for j in (i..n).rev() {
            dd[j] = (dd[j] - dd[j - 1]) / (za[j] - za[j - i]);
        }
    }

    GSL_SUCCESS
}
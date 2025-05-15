use std::f64;

pub fn cblas_caxpy(
    n: i32,
    alpha: &[f32; 2],
    x: &[f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
) {
    if n <= 0 {
        return;
    }

    let alpha_real = alpha[0];
    let alpha_imag = alpha[1];

    if f64::abs(alpha_real as f64) == 0.0 && f64::abs(alpha_imag as f64) == 0.0 {
        return;
    }

    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        let x_real = x[(2 * ix) as usize];
        let x_imag = x[(2 * ix + 1) as usize];

        y[(2 * iy) as usize] += alpha_real * x_real - alpha_imag * x_imag;
        y[(2 * iy + 1) as usize] += alpha_real * x_imag + alpha_imag * x_real;

        ix += inc_x;
        iy += inc_y;
    }
}
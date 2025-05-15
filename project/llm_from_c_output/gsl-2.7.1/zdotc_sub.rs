use num_complex::Complex64;

pub fn cblas_zdotc_sub(
    n: i32,
    x: &[Complex64],
    inc_x: i32,
    y: &[Complex64],
    inc_y: i32,
    result: &mut Complex64,
) {
    if n <= 0 {
        *result = Complex64::new(0.0, 0.0);
        return;
    }

    let mut temp = Complex64::new(0.0, 0.0);
    let mut ix = 0;
    let mut iy = 0;

    if inc_x == 1 && inc_y == 1 {
        for i in 0..n as usize {
            temp += x[i].conj() * y[i];
        }
    } else {
        for _ in 0..n {
            temp += x[ix as usize].conj() * y[iy as usize];
            ix += inc_x;
            iy += inc_y;
        }
    }

    *result = temp;
}
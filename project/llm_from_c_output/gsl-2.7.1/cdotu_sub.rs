use num_complex::Complex32;

pub fn cblas_cdotu_sub(
    n: i32,
    x: &[Complex32],
    inc_x: i32,
    y: &[Complex32],
    inc_y: i32,
    result: &mut Complex32,
) {
    if n <= 0 {
        *result = Complex32::new(0.0, 0.0);
        return;
    }

    let mut temp = Complex32::new(0.0, 0.0);

    if inc_x == 1 && inc_y == 1 {
        for i in 0..n as usize {
            temp += x[i] * y[i];
        }
    } else {
        let mut ix = 0;
        let mut iy = 0;
        if inc_x < 0 {
            ix = (-n + 1) * inc_x;
        }
        if inc_y < 0 {
            iy = (-n + 1) * inc_y;
        }
        for _ in 0..n {
            temp += x[ix as usize] * y[iy as usize];
            ix += inc_x;
            iy += inc_y;
        }
    }

    *result = temp;
}
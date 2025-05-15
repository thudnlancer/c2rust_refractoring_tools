use num_complex::Complex32;

pub fn cblas_cdotu_sub(
    n: i32,
    x: &[Complex32],
    inc_x: i32,
    y: &[Complex32],
    inc_y: i32,
    result: &mut [Complex32],
) {
    let mut r_real = 0.0f32;
    let mut r_imag = 0.0f32;
    
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
    
    for _ in 0..n {
        let x_elem = x[(ix / 2) as usize];
        let y_elem = y[(iy / 2) as usize];
        
        r_real += x_elem.re * y_elem.re - x_elem.im * y_elem.im;
        r_imag += x_elem.re * y_elem.im + x_elem.im * y_elem.re;
        
        ix += inc_x;
        iy += inc_y;
    }
    
    result[0] = Complex32::new(r_real, r_imag);
}
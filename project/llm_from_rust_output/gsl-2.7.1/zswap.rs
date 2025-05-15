pub fn cblas_zswap(
    n: i32,
    x: &mut [f64],
    inc_x: i32,
    y: &mut [f64],
    inc_y: i32,
) {
    assert!(x.len() >= (2 * n as usize).max(1));
    assert!(y.len() >= (2 * n as usize).max(1));
    
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
    
    for _ in 0..n {
        let x_real = x[(2 * ix) as usize];
        let x_imag = x[(2 * ix + 1) as usize];
        
        x[(2 * ix) as usize] = y[(2 * iy) as usize];
        x[(2 * ix + 1) as usize] = y[(2 * iy + 1) as usize];
        
        y[(2 * iy) as usize] = x_real;
        y[(2 * iy + 1) as usize] = x_imag;
        
        ix += inc_x;
        iy += inc_y;
    }
}
pub fn cblas_dsdot(n: i32, x: &[f32], inc_x: i32, y: &[f32], inc_y: i32) -> f64 {
    if n <= 0 {
        return 0.0;
    }

    if inc_x == 1 && inc_y == 1 {
        x.iter().zip(y.iter()).take(n as usize).map(|(&a, &b)| a as f64 * b as f64).sum()
    } else {
        let mut sum = 0.0;
        let mut ix = 0;
        let mut iy = 0;
        
        for _ in 0..n {
            sum += x[ix as usize] as f64 * y[iy as usize] as f64;
            ix += inc_x;
            iy += inc_y;
            
            if ix as usize >= x.len() || iy as usize >= y.len() {
                break;
            }
        }
        
        sum
    }
}
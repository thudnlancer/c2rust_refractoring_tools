pub fn cblas_ddot(n: i32, x: &[f64], inc_x: i32, y: &[f64], inc_y: i32) -> f64 {
    if n <= 0 {
        return 0.0;
    }

    let mut result = 0.0;
    
    if inc_x == 1 && inc_y == 1 {
        let len = n as usize;
        for i in 0..len {
            result += x[i] * y[i];
        }
    } else {
        let mut ix = 0;
        let mut iy = 0;
        for _ in 0..n {
            result += x[ix as usize] * y[iy as usize];
            ix += inc_x;
            iy += inc_y;
        }
    }
    
    result
}
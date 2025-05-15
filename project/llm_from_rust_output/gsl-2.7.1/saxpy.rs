pub fn cblas_saxpy(
    n: i32,
    alpha: f32,
    x: &[f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
) {
    if alpha == 0.0 {
        return;
    }

    if inc_x == 1 && inc_y == 1 {
        let chunks = x.chunks_exact(4).zip(y.chunks_exact_mut(4));
        for (x_chunk, y_chunk) in chunks {
            y_chunk[0] += alpha * x_chunk[0];
            y_chunk[1] += alpha * x_chunk[1];
            y_chunk[2] += alpha * x_chunk[2];
            y_chunk[3] += alpha * x_chunk[3];
        }

        let remainder = x.len() % 4;
        for i in (x.len() - remainder)..x.len() {
            y[i] += alpha * x[i];
        }
    } else {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } as usize;
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
        
        for _ in 0..n {
            y[iy] += alpha * x[ix];
            ix = (ix as i32 + inc_x) as usize;
            iy = (iy as i32 + inc_y) as usize;
        }
    }
}
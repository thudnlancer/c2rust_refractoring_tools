pub fn cblas_csscal(
    n: i32,
    alpha: f32,
    x: &mut [f32],
    inc_x: i32,
) {
    if inc_x <= 0 || n <= 0 {
        return;
    }

    let mut ix = 0;
    for _ in 0..n {
        if ix + 1 >= x.len() {
            break;
        }
        
        x[ix] *= alpha;
        x[ix + 1] *= alpha;
        
        ix = (ix as i32 + inc_x * 2) as usize;
    }
}
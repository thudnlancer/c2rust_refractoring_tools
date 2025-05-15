use num_complex::Complex32;
use std::cmp::Ordering;

pub fn cblas_icamax(n: i32, x: &[Complex32], inc_x: i32) -> Option<usize> {
    if n <= 0 || inc_x <= 0 || x.is_empty() {
        return None;
    }

    let mut max_idx = 0;
    let mut max_val = 0.0f32;

    let mut ix = 0;
    for i in 0..n {
        let val = x[ix as usize].norm();
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
        ix += inc_x;
        if ix as usize >= x.len() {
            break;
        }
    }

    Some(max_idx as usize)
}
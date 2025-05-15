use num_complex::Complex64;
use std::cmp::Ordering;

pub fn cblas_izamax(n: i32, x: &[Complex64], inc_x: i32) -> usize {
    if n <= 0 || inc_x <= 0 {
        return 0;
    }

    let mut max_index = 0;
    let mut max_value = 0.0;

    let mut ix = 0;
    for i in 0..n {
        let current = x[ix as usize].norm();
        match current.partial_cmp(&max_value) {
            Some(Ordering::Greater) => {
                max_value = current;
                max_index = i;
            }
            _ => {}
        }
        ix += inc_x;
        if ix as usize >= x.len() {
            break;
        }
    }

    max_index as usize
}
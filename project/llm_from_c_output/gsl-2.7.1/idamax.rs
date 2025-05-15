pub fn cblas_idamax(n: usize, x: &[f64], inc_x: usize) -> Option<usize> {
    if n == 0 || inc_x == 0 {
        return None;
    }

    let mut max_idx = 0;
    let mut max_val = x[0].abs();
    let mut current_idx = inc_x;

    while current_idx < n {
        let val = x[current_idx].abs();
        if val > max_val {
            max_val = val;
            max_idx = current_idx;
        }
        current_idx += inc_x;
    }

    Some(max_idx)
}
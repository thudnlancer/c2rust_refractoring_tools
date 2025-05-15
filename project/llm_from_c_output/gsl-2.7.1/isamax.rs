pub fn cblas_isamax(n: i32, x: &[f32], inc_x: i32) -> usize {
    if n <= 0 || inc_x <= 0 {
        return 0;
    }

    let mut max_index = 0;
    let mut max_value = 0.0f32.abs();
    let mut current_index = 0;

    for i in 0..n {
        let value = x[current_index].abs();
        if value > max_value {
            max_value = value;
            max_index = i as usize;
        }
        current_index += inc_x as usize;
        if current_index >= x.len() {
            break;
        }
    }

    max_index
}
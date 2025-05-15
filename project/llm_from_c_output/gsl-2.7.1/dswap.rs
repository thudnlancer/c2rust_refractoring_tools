/// Swaps the elements of two f64 vectors with specified increments.
///
/// # Arguments
/// * `n` - The number of elements to swap.
/// * `x` - The first vector, with elements to be swapped.
/// * `inc_x` - The increment between elements of `x`.
/// * `y` - The second vector, with elements to be swapped.
/// * `inc_y` - The increment between elements of `y`.
///
/// # Panics
/// Panics if the vectors are not large enough for the specified increments and number of elements.
pub fn cblas_dswap(n: usize, x: &mut [f64], inc_x: usize, y: &mut [f64], inc_y: usize) {
    if n == 0 {
        return;
    }

    // Check that vectors are large enough
    let x_len_required = (n - 1) * inc_x + 1;
    let y_len_required = (n - 1) * inc_y + 1;
    assert!(x.len() >= x_len_required, "x vector too small");
    assert!(y.len() >= y_len_required, "y vector too small");

    // Perform the swap
    for i in 0..n {
        let x_idx = i * inc_x;
        let y_idx = i * inc_y;
        x.swap(x_idx, y_idx);
    }
}
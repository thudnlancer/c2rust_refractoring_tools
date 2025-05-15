pub fn gsl_spmatrix_cumsum(n: usize, c: &mut [i32]) {
    let mut sum = 0;
    for k in 0..n {
        let ck = c[k];
        c[k] = sum;
        sum += ck;
    }
    if n < c.len() {
        c[n] = sum;
    }
}
pub fn cblas_srotg(a: &mut f32, b: &mut f32, c: &mut f32, s: &mut f32) {
    let abs_a = a.abs();
    let abs_b = b.abs();
    
    let (cos_val, sin_val, norm) = if abs_b == 0.0 {
        (*a = 1.0, *b = 0.0, *s = 0.0);
        (1.0, 0.0, abs_a)
    } else if abs_a == 0.0 {
        (*a = *b, *b = 1.0, *s = 1.0);
        (0.0, 1.0, abs_b)
    } else if abs_b > abs_a {
        let tau = *a / *b;
        let norm = (*b * (1.0 + tau * tau).sqrt());
        let cos_val = 1.0 / (1.0 + tau * tau).sqrt();
        let sin_val = cos_val * tau;
        (cos_val, sin_val, norm)
    } else {
        let tau = *b / *a;
        let norm = (*a * (1.0 + tau * tau).sqrt());
        let sin_val = 1.0 / (1.0 + tau * tau).sqrt();
        let cos_val = sin_val * tau;
        (cos_val, sin_val, norm)
    };
    
    *c = cos_val;
    *s = sin_val;
    *a = norm;
    *b = tau;
}
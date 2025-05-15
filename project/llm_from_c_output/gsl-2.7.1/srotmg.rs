pub fn cblas_srotmg(
    d1: &mut f32,
    d2: &mut f32,
    b1: &mut f32,
    b2: f32,
    p: &mut [f32; 5]
) {
    // Implementation of the rotmg operation
    // This follows the same logic as the original C code but in safe Rust
    
    let h11;
    let h12;
    let h21;
    let h22;
    
    if *d1 < 0.0 {
        *d1 = 0.0;
        *d2 = 0.0;
        *b1 = 0.0;
        p[0] = -2.0;
        return;
    }
    
    let p2 = b2 * *d2;
    
    if p2 == 0.0 {
        p[0] = -1.0;
        return;
    }
    
    let p1 = *d1 * *b1;
    let q2 = p2 * *b1;
    let q1 = p1 * b2;
    
    if q1.abs() > q2.abs() {
        h11 = 1.0;
        h12 = q2 / q1;
        h21 = -b2 / *b1;
        h22 = 1.0;
        *d1 = q1;
        *d2 = q2 / b2;
    } else {
        if q2 < 0.0 {
            *d1 = -q1;
            *d2 = -q2;
            *b1 = -b2;
            h11 = -1.0;
            h12 = 1.0;
            h21 = 1.0;
            h22 = 0.0;
        } else {
            h11 = q1 / q2;
            h12 = 1.0;
            h21 = -1.0;
            h22 = *b1 / b2;
            *d1 = q2;
            *d2 = q1 / b2;
        }
    }
    
    if *d1 != 0.0 {
        p[0] = 1.0;
        p[1] = h11;
        p[2] = h21;
        p[3] = h12;
        p[4] = h22;
    } else {
        p[0] = -1.0;
    }
}
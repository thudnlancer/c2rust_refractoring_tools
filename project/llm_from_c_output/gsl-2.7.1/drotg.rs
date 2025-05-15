pub fn cblas_drotg(a: &mut f64, b: &mut f64, c: &mut f64, s: &mut f64) {
    let (new_c, new_s, new_r) = if *b == 0.0 {
        (1.0, 0.0, *a)
    } else if a.abs() > b.abs() {
        let t = *b / *a;
        let u = a.signum() * (1.0 + t * t).sqrt();
        let new_c = 1.0 / u;
        let new_s = t * new_c;
        let new_r = *a * u;
        (new_c, new_s, new_r)
    } else {
        let t = *a / *b;
        let u = b.signum() * (1.0 + t * t).sqrt();
        let new_s = 1.0 / u;
        let new_c = t * new_s;
        let new_r = *b * u;
        (new_c, new_s, new_r)
    };
    
    *a = new_r;
    *b = 0.0;
    *c = new_c;
    *s = new_s;
}
use std::f64;

pub fn cblas_drotg(
    a: &mut f64,
    b: &mut f64,
    c: &mut f64,
    s: &mut f64,
) {
    let roe = if f64::abs(*a) > f64::abs(*b) { *a } else { *b };
    let scale = f64::abs(*a) + f64::abs(*b);
    let (mut r, mut z) = (0.0, 0.0);
    
    if scale != 0.0 {
        let aos = *a / scale;
        let bos = *b / scale;
        r = scale * f64::sqrt(aos * aos + bos * bos);
        r *= if roe >= 0.0 { 1.0 } else { -1.0 };
        *c = *a / r;
        *s = *b / r;
        z = 1.0;
        if f64::abs(*a) > f64::abs(*b) {
            z = *s;
        }
        if f64::abs(*b) >= f64::abs(*a) && *c != 0.0 {
            z = 1.0 / *c;
        }
    } else {
        *c = 1.0;
        *s = 0.0;
        r = 0.0;
        z = 0.0;
    }
    *a = r;
    *b = z;
}
use std::f32;
use std::f64;

pub fn cblas_srotg(
    a: &mut f32,
    b: &mut f32,
    c: &mut f32,
    s: &mut f32,
) {
    let roe = if f64::from(a.abs()) > f64::from(b.abs()) {
        *a
    } else {
        *b
    };
    let scale = f64::from(a.abs()) + f64::from(b.abs());
    let (mut r, mut z) = (0.0, 0.0);
    
    if scale != 0.0 {
        let aos = *a / scale as f32;
        let bos = *b / scale as f32;
        r = (scale * f64::from(aos * aos + bos * bos).sqrt()) as f32;
        r *= if roe >= 0.0 { 1.0 } else { -1.0 };
        *c = *a / r;
        *s = *b / r;
        z = 1.0;
        if f64::from(a.abs()) > f64::from(b.abs()) {
            z = *s;
        }
        if f64::from(b.abs()) >= f64::from(a.abs()) && *c != 0.0 {
            z = (1.0 / *c as f64) as f32;
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
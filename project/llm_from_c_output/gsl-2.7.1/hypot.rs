use std::f64;

fn xhypot(x: f64, y: f64) -> f64 {
    let xabs = x.abs();
    let yabs = y.abs();
    let (min, max) = if xabs < yabs {
        (xabs, yabs)
    } else {
        (yabs, xabs)
    };

    if min == 0.0 {
        return max;
    }

    let u = min / max;
    max * (1.0 + u * u).sqrt()
}
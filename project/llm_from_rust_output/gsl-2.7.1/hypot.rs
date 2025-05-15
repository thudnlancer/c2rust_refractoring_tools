use std::f64;

#[no_mangle]
pub extern "C" fn gsl_hypot(x: f64, y: f64) -> f64 {
    let xabs = x.abs();
    let yabs = y.abs();
    
    if xabs.is_infinite() || yabs.is_infinite() {
        return f64::INFINITY;
    }
    
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

#[no_mangle]
pub extern "C" fn gsl_hypot3(x: f64, y: f64, z: f64) -> f64 {
    let xabs = x.abs();
    let yabs = y.abs();
    let zabs = z.abs();
    
    let w = if xabs > yabs && xabs > zabs {
        xabs
    } else if yabs > zabs {
        yabs
    } else {
        zabs
    };
    
    if w == 0.0 {
        0.0
    } else {
        let xnorm = xabs / w;
        let ynorm = yabs / w;
        let znorm = zabs / w;
        w * (xnorm * xnorm + ynorm * ynorm + znorm * znorm).sqrt()
    }
}
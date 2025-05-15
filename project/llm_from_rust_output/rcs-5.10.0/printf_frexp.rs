#[no_mangle]
pub extern "C" fn printf_frexp(x: f64, expptr: *mut i32) -> f64 {
    let (mut x, mut exponent) = x.frexp();
    x += x;
    exponent -= 1;
    
    let min_exponent = -1022;
    if exponent < min_exponent {
        x = x.ldexp(exponent - min_exponent);
        exponent = min_exponent;
    }
    
    unsafe {
        *expptr = exponent;
    }
    x
}
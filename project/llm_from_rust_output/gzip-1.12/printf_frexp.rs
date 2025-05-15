use std::mem::MaybeUninit;

#[no_mangle]
pub extern "C" fn printf_frexp(x: f64, expptr: &mut i32) -> f64 {
    let (mut x, mut exponent) = x.frexp();
    x += x;
    exponent -= 1;
    
    if exponent < -1022 {
        x = x.ldexp(exponent - (-1022));
        exponent = -1022;
    }
    
    *expptr = exponent;
    x
}

trait FloatExt {
    fn frexp(self) -> (Self, i32);
    fn ldexp(self, exp: i32) -> Self;
}

impl FloatExt for f64 {
    fn frexp(self) -> (f64, i32) {
        let mut exp = MaybeUninit::uninit();
        let mantissa = unsafe { libc::frexp(self, exp.as_mut_ptr()) };
        (mantissa, unsafe { exp.assume_init() })
    }

    fn ldexp(self, exp: i32) -> f64 {
        unsafe { libc::ldexp(self, exp) }
    }
}
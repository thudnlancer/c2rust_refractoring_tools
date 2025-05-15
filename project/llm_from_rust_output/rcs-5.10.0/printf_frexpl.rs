use std::arch::x86_64::{_fnstcw, _fldcw};
use f128::f128;

pub type Fpucw = u16;

#[no_mangle]
pub fn printf_frexpl(x: f128, expptr: &mut i32) -> f128 {
    let mut exponent = 0;
    let oldcw = unsafe {
        let mut cw: Fpucw = 0;
        _fnstcw(&mut cw);
        cw
    };

    let newcw = (oldcw as i32 & !0x300 | 0x300) as Fpucw;
    unsafe {
        _fldcw(&newcw);
    }

    let mut x = unsafe { frexpl(x, &mut exponent) };
    x = x + x;
    exponent -= 1;

    if exponent < -16382 {
        x = unsafe { ldexpl(x, exponent - (-16382)) };
        exponent = -16382;
    }

    unsafe {
        _fldcw(&oldcw);
    }

    *expptr = exponent;
    x
}

extern "C" {
    fn frexpl(x: f128, exp: &mut i32) -> f128;
    fn ldexpl(x: f128, exp: i32) -> f128;
}
use core::arch::asm;

pub type Fpucw = u16;

pub fn printf_frexpl(x: f128, expptr: &mut i32) -> f128 {
    let oldcw: Fpucw;
    unsafe {
        asm!("fnstcw [{}]", in(reg) &mut oldcw, options(preserves_flags));
    }

    let newcw = (oldcw as i32 & !0x300 | 0x300) as Fpucw;
    unsafe {
        asm!("fldcw [{}]", in(reg) &newcw, options(preserves_flags));
    }

    let mut exponent = 0;
    let mut x = unsafe { frexpl(x, &mut exponent) };
    x = x + x;
    exponent -= 1;

    if exponent < -16382 {
        x = unsafe { ldexpl(x, exponent - (-16382)) };
        exponent = -16382;
    }

    unsafe {
        asm!("fldcw [{}]", in(reg) &oldcw, options(preserves_flags));
    }

    *expptr = exponent;
    x
}

extern "C" {
    fn frexpl(x: f128, exp: &mut i32) -> f128;
    fn ldexpl(x: f128, exp: i32) -> f128;
}
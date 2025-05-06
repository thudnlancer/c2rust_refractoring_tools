#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, label_break_value)]
use core::arch::asm;
extern "C" {
    fn frexpl(_: f128::f128, _: *mut i32) -> f128::f128;
    fn ldexpl(_: f128::f128, _: i32) -> f128::f128;
}
pub type fpucw_t = libc::c_ushort;
#[no_mangle]
pub unsafe extern "C" fn printf_frexpl(
    mut x: f128::f128,
    mut expptr: *mut i32,
) -> f128::f128 {
    let mut exponent: i32 = 0;
    let mut oldcw: fpucw_t = 0;
    oldcw = ({
        let mut _cw: fpucw_t = 0;
        asm!("fnstcw [{0}]", in (reg) & mut _cw, options(preserves_flags));
        _cw
    });
    let mut _ncw: fpucw_t = (oldcw as i32 & !(0x300 as i32) | 0x300 as i32) as fpucw_t;
    asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    'c_149: {
        let mut _ncw: fpucw_t = (oldcw as i32 & !(0x300 as i32) | 0x300 as i32)
            as fpucw_t;
        asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    };
    x = frexpl(x, &mut exponent);
    x = x + x;
    exponent -= 1 as i32;
    if exponent < -(16381 as i32) - 1 as i32 {
        x = ldexpl(x, exponent - (-(16381 as i32) - 1 as i32));
        exponent = -(16381 as i32) - 1 as i32;
    }
    let mut _ncw: fpucw_t = oldcw;
    asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    'c_59: {
        let mut _ncw: fpucw_t = oldcw;
        asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    };
    *expptr = exponent;
    return x;
}
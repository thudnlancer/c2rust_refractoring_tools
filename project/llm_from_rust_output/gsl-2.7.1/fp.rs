use std::arch::asm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Eof = 32,
    Etolg = 31,
    Etolx = 30,
    Etolf = 29,
    Enoprogj = 28,
    Enoprog = 27,
    Etable = 26,
    Ecache = 25,
    Eunimpl = 24,
    Eunsup = 23,
    Ediverge = 22,
    Esing = 21,
    Enotsqr = 20,
    Ebadlen = 19,
    Eround = 18,
    Eloss = 17,
    Eovrflw = 16,
    Eundrflw = 15,
    Etol = 14,
    Ebadtol = 13,
    Ezerodiv = 12,
    Emaxiter = 11,
    Erunaway = 10,
    Ebadfunc = 9,
    Enomem = 8,
    Esanity = 7,
    Efactor = 6,
    Efailed = 5,
    Einval = 4,
    Efault = 3,
    Erange = 2,
    Edom = 1,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GslIeeePrecision {
    Single = 1,
    Double = 2,
    Extended = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GslIeeeRounding {
    Nearest = 1,
    Down = 2,
    Up = 3,
    Zero = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GslIeeeExceptionMask {
    Invalid = 1,
    Denormalized = 2,
    DivisionByZero = 4,
    Overflow = 8,
    Underflow = 16,
    All = 31,
    TrapInexact = 32,
}

pub fn gsl_ieee_set_mode(
    precision: GslIeeePrecision,
    rounding: GslIeeeRounding,
    exception_mask: u32,
) -> Result<(), GslError> {
    let mut mode: u16 = 0;

    match precision {
        GslIeeePrecision::Single => mode |= 0x000,
        GslIeeePrecision::Double => mode |= 0x200,
        GslIeeePrecision::Extended => mode |= 0x300,
    }

    match rounding {
        GslIeeeRounding::Nearest => mode |= 0x000,
        GslIeeeRounding::Down => mode |= 0x400,
        GslIeeeRounding::Up => mode |= 0x800,
        GslIeeeRounding::Zero => mode |= 0xC00,
    }

    if exception_mask & GslIeeeExceptionMask::Invalid as u32 != 0 {
        mode |= 0x1;
    }
    if exception_mask & GslIeeeExceptionMask::Denormalized as u32 != 0 {
        mode |= 0x2;
    }
    if exception_mask & GslIeeeExceptionMask::DivisionByZero as u32 != 0 {
        mode |= 0x4;
    }
    if exception_mask & GslIeeeExceptionMask::Overflow as u32 != 0 {
        mode |= 0x8;
    }
    if exception_mask & GslIeeeExceptionMask::Underflow as u32 != 0 {
        mode |= 0x10;
    }
    if exception_mask & GslIeeeExceptionMask::TrapInexact as u32 != 0 {
        mode &= !0x20;
    } else {
        mode |= 0x20;
    }

    unsafe {
        asm!("fldcw [{0}]", in(reg) &mode, options(preserves_flags));
        
        let mut mode_sse: u32 = 0;
        mode_sse |= ((mode as u32 & 0x3f) << 7);
        mode_sse |= ((mode as u32 & 0xc00) << 3);
        asm!("ldmxcsr [{0}]", in(reg) &mode_sse, options(preserves_flags));
    }

    Ok(())
}
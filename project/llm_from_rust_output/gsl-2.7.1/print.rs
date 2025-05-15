use std::io::{self, Write};
use std::ffi::CStr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GslIeeeFloatRep {
    pub sign: i32,
    pub mantissa: [i8; 24],
    pub exponent: i32,
    pub type_: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GslIeeeDoubleRep {
    pub sign: i32,
    pub mantissa: [i8; 53],
    pub exponent: i32,
    pub type_: i32,
}

#[derive(Debug, PartialEq)]
pub enum IeeeType {
    Nan,
    Inf,
    Normal,
    Denormal,
    Zero,
    NonStandard,
}

impl From<i32> for IeeeType {
    fn from(value: i32) -> Self {
        match value {
            1 => IeeeType::Nan,
            2 => IeeeType::Inf,
            3 => IeeeType::Normal,
            4 => IeeeType::Denormal,
            5 => IeeeType::Zero,
            _ => IeeeType::NonStandard,
        }
    }
}

const SIGNS: [char; 2] = [' ', '-'];

extern "C" {
    fn gsl_ieee_float_to_rep(x: *const f32, r: *mut GslIeeeFloatRep);
    fn gsl_ieee_double_to_rep(x: *const f64, r: *mut GslIeeeDoubleRep);
}

pub fn ieee_fprintf_float<W: Write>(stream: &mut W, x: f32) -> io::Result<()> {
    let mut r = GslIeeeFloatRep {
        sign: 0,
        mantissa: [0; 24],
        exponent: 0,
        type_: 0,
    };
    
    unsafe {
        gsl_ieee_float_to_rep(&x as *const f32, &mut r as *mut GslIeeeFloatRep);
    }
    
    let mantissa_str = unsafe {
        CStr::from_ptr(r.mantissa.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    
    match IeeeType::from(r.type_) {
        IeeeType::Nan => write!(stream, "NaN")?,
        IeeeType::Inf => write!(stream, "{}Inf", SIGNS[r.sign as usize])?,
        IeeeType::Normal => write!(
            stream,
            "{}1.{}*2^{}",
            SIGNS[r.sign as usize],
            mantissa_str,
            r.exponent
        )?,
        IeeeType::Denormal => write!(
            stream,
            "{}0.{}*2^{}",
            SIGNS[r.sign as usize],
            mantissa_str,
            r.exponent + 1
        )?,
        IeeeType::Zero => write!(stream, "{}0", SIGNS[r.sign as usize])?,
        IeeeType::NonStandard => write!(stream, "[non-standard IEEE float]")?,
    }
    
    Ok(())
}

pub fn ieee_printf_float(x: f32) -> io::Result<()> {
    ieee_fprintf_float(&mut io::stdout(), x)
}

pub fn ieee_fprintf_double<W: Write>(stream: &mut W, x: f64) -> io::Result<()> {
    let mut r = GslIeeeDoubleRep {
        sign: 0,
        mantissa: [0; 53],
        exponent: 0,
        type_: 0,
    };
    
    unsafe {
        gsl_ieee_double_to_rep(&x as *const f64, &mut r as *mut GslIeeeDoubleRep);
    }
    
    let mantissa_str = unsafe {
        CStr::from_ptr(r.mantissa.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    
    match IeeeType::from(r.type_) {
        IeeeType::Nan => write!(stream, "NaN")?,
        IeeeType::Inf => write!(stream, "{}Inf", SIGNS[r.sign as usize])?,
        IeeeType::Normal => write!(
            stream,
            "{}1.{}*2^{}",
            SIGNS[r.sign as usize],
            mantissa_str,
            r.exponent
        )?,
        IeeeType::Denormal => write!(
            stream,
            "{}0.{}*2^{}",
            SIGNS[r.sign as usize],
            mantissa_str,
            r.exponent + 1
        )?,
        IeeeType::Zero => write!(stream, "{}0", SIGNS[r.sign as usize])?,
        IeeeType::NonStandard => write!(stream, "[non-standard IEEE double]")?,
    }
    
    Ok(())
}

pub fn ieee_printf_double(x: f64) -> io::Result<()> {
    ieee_fprintf_double(&mut io::stdout(), x)
}
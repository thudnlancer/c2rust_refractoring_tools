//! IEEE floating-point printing utilities
//!
//! Ported from GNU Scientific Library's ieee-utils/print.c

use std::io::{self, Write};

// A table of sign characters, 0=positive, 1=negative. We print a space
// instead of a unary + sign for compatibility with bc
static SIGNS: [char; 2] = [' ', '-'];

/// Prints the IEEE representation of a float to the given stream
pub fn gsl_ieee_fprintf_float(stream: &mut dyn Write, x: &f32) -> io::Result<()> {
    let r = gsl_ieee_float_to_rep(x);

    match r.type_ {
        GslIeeeType::Nan => write!(stream, "NaN")?,
        GslIeeeType::Inf => write!(stream, "{}Inf", SIGNS[r.sign as usize])?,
        GslIeeeType::Normal => write!(
            stream,
            "{}1.{}*2^{}",
            SIGNS[r.sign as usize], r.mantissa, r.exponent
        )?,
        GslIeeeType::Denormal => write!(
            stream,
            "{}0.{}*2^{}",
            SIGNS[r.sign as usize],
            r.mantissa,
            r.exponent + 1
        )?,
        GslIeeeType::Zero => write!(stream, "{}0", SIGNS[r.sign as usize])?,
        _ => write!(stream, "[non-standard IEEE float]")?,
    }

    Ok(())
}

/// Prints the IEEE representation of a float to stdout
pub fn gsl_ieee_printf_float(x: &f32) -> io::Result<()> {
    gsl_ieee_fprintf_float(&mut io::stdout(), x)
}

/// Prints the IEEE representation of a double to the given stream
pub fn gsl_ieee_fprintf_double(stream: &mut dyn Write, x: &f64) -> io::Result<()> {
    let r = gsl_ieee_double_to_rep(x);

    match r.type_ {
        GslIeeeType::Nan => write!(stream, "NaN")?,
        GslIeeeType::Inf => write!(stream, "{}Inf", SIGNS[r.sign as usize])?,
        GslIeeeType::Normal => write!(
            stream,
            "{}1.{}*2^{}",
            SIGNS[r.sign as usize], r.mantissa, r.exponent
        )?,
        GslIeeeType::Denormal => write!(
            stream,
            "{}0.{}*2^{}",
            SIGNS[r.sign as usize],
            r.mantissa,
            r.exponent + 1
        )?,
        GslIeeeType::Zero => write!(stream, "{}0", SIGNS[r.sign as usize])?,
        _ => write!(stream, "[non-standard IEEE double]")?,
    }

    Ok(())
}

/// Prints the IEEE representation of a double to stdout
pub fn gsl_ieee_printf_double(x: &f64) -> io::Result<()> {
    gsl_ieee_fprintf_double(&mut io::stdout(), x)
}

// The following types and functions would be defined elsewhere in the Rust GSL bindings:

#[derive(Debug, PartialEq)]
enum GslIeeeType {
    Nan,
    Inf,
    Normal,
    Denormal,
    Zero,
    // Other variants as needed
}

struct GslIeeeFloatRep {
    sign: u32,
    mantissa: String,
    exponent: i32,
    type_: GslIeeeType,
}

struct GslIeeeDoubleRep {
    sign: u32,
    mantissa: String,
    exponent: i32,
    type_: GslIeeeType,
}

fn gsl_ieee_float_to_rep(x: &f32) -> GslIeeeFloatRep {
    // Implementation would convert float to its IEEE representation
    unimplemented!()
}

fn gsl_ieee_double_to_rep(x: &f64) -> GslIeeeDoubleRep {
    // Implementation would convert double to its IEEE representation
    unimplemented!()
}
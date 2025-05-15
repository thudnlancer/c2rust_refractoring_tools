use std::ffi::{CStr, CString};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeeePrecision {
    Single = 1,
    Double = 2,
    Extended = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeeeRounding {
    Nearest = 1,
    Down = 2,
    Up = 3,
    Zero = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeeeExceptionMask {
    Invalid = 1,
    Denormalized = 2,
    DivisionByZero = 4,
    Overflow = 8,
    Underflow = 16,
    All = 31,
    TrapInexact = 32,
}

pub fn gsl_ieee_read_mode_string(
    description: &str,
) -> Result<(IeeePrecision, IeeeRounding, IeeeExceptionMask), GslError> {
    let mut precision = None;
    let mut rounding = None;
    let mut exception_mask = IeeeExceptionMask::Invalid; // Initialize with default

    for token in description.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        match token {
            "single-precision" => {
                if precision.is_some() {
                    return Err(GslError::Inval);
                }
                precision = Some(IeeePrecision::Single);
            }
            "double-precision" => {
                if precision.is_some() {
                    return Err(GslError::Inval);
                }
                precision = Some(IeeePrecision::Double);
            }
            "extended-precision" => {
                if precision.is_some() {
                    return Err(GslError::Inval);
                }
                precision = Some(IeeePrecision::Extended);
            }
            "round-to-nearest" => {
                if rounding.is_some() {
                    return Err(GslError::Inval);
                }
                rounding = Some(IeeeRounding::Nearest);
            }
            "round-down" => {
                if rounding.is_some() {
                    return Err(GslError::Inval);
                }
                rounding = Some(IeeeRounding::Down);
            }
            "round-up" => {
                if rounding.is_some() {
                    return Err(GslError::Inval);
                }
                rounding = Some(IeeeRounding::Up);
            }
            "round-to-zero" => {
                if rounding.is_some() {
                    return Err(GslError::Inval);
                }
                rounding = Some(IeeeRounding::Zero);
            }
            "mask-all" => {
                exception_mask = IeeeExceptionMask::All;
            }
            "mask-invalid" => {
                exception_mask = IeeeExceptionMask::Invalid;
            }
            "mask-denormalized" => {
                exception_mask = IeeeExceptionMask::Denormalized;
            }
            "mask-division-by-zero" => {
                exception_mask = IeeeExceptionMask::DivisionByZero;
            }
            "mask-overflow" => {
                exception_mask = IeeeExceptionMask::Overflow;
            }
            "mask-underflow" => {
                exception_mask = IeeeExceptionMask::Underflow;
            }
            "trap-inexact" => {
                exception_mask = IeeeExceptionMask::TrapInexact;
            }
            "trap-common" => {}
            _ => {
                return Err(GslError::Inval);
            }
        }
    }

    let precision = precision.unwrap_or(IeeePrecision::Double);
    let rounding = rounding.unwrap_or(IeeeRounding::Nearest);

    Ok((precision, rounding, exception_mask))
}

fn gsl_error(reason: &str, file: &str, line: i32, errno: GslError) {
    eprintln!("GSL error: {} ({}:{}) - {:?}", reason, file, line, errno);
}
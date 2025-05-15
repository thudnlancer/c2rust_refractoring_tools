use std::ffi::CString;
use std::ptr;
use std::str;

#[derive(Debug, PartialEq)]
pub enum GslError {
    NoMemory,
    InvalidValue,
    Success,
}

#[derive(Debug, PartialEq)]
pub enum Precision {
    Single,
    Double,
    Extended,
}

#[derive(Debug, PartialEq)]
pub enum Rounding {
    Nearest,
    Down,
    Up,
    Zero,
}

#[derive(Debug, PartialEq)]
pub enum ExceptionMask {
    Invalid,
    Denormalized,
    DivisionByZero,
    Overflow,
    Underflow,
    All,
    TrapInexact,
}

pub fn gsl_ieee_read_mode_string(
    description: &str,
) -> Result<(Option<Precision>, Option<Rounding>, Vec<ExceptionMask>), GslError> {
    let mut precision = None;
    let mut rounding = None;
    let mut exception_mask = Vec::new();

    let mut precision_count = 0;
    let mut rounding_count = 0;

    for token in description.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        match lookup_string(token) {
            Ok((p, r, e)) => {
                if let Some(new_precision) = p {
                    if precision_count > 0 {
                        return Err(GslError::InvalidValue);
                    }
                    precision = Some(new_precision);
                    precision_count += 1;
                }

                if let Some(new_rounding) = r {
                    if rounding_count > 0 {
                        return Err(GslError::InvalidValue);
                    }
                    rounding = Some(new_rounding);
                    rounding_count += 1;
                }

                if let Some(new_exception) = e {
                    exception_mask.push(new_exception);
                }
            }
            Err(_) => {
                return Err(GslError::InvalidValue);
            }
        }
    }

    Ok((precision, rounding, exception_mask))
}

fn lookup_string(token: &str) -> Result<(Option<Precision>, Option<Rounding>, Option<ExceptionMask>), ()> {
    match token {
        "single-precision" => Ok((Some(Precision::Single), None, None)),
        "double-precision" => Ok((Some(Precision::Double), None, None)),
        "extended-precision" => Ok((Some(Precision::Extended), None, None)),
        "round-to-nearest" => Ok((None, Some(Rounding::Nearest), None)),
        "round-down" => Ok((None, Some(Rounding::Down), None)),
        "round-up" => Ok((None, Some(Rounding::Up), None)),
        "round-to-zero" => Ok((None, Some(Rounding::Zero), None)),
        "mask-all" => Ok((None, None, Some(ExceptionMask::All))),
        "mask-invalid" => Ok((None, None, Some(ExceptionMask::Invalid))),
        "mask-denormalized" => Ok((None, None, Some(ExceptionMask::Denormalized))),
        "mask-division-by-zero" => Ok((None, None, Some(ExceptionMask::DivisionByZero))),
        "mask-overflow" => Ok((None, None, Some(ExceptionMask::Overflow))),
        "mask-underflow" => Ok((None, None, Some(ExceptionMask::Underflow))),
        "trap-inexact" => Ok((None, None, Some(ExceptionMask::TrapInexact))),
        "trap-common" => Ok((None, None, None)),
        _ => Err(()),
    }
}
use std::env;
use std::ffi::CString;
use std::io::{stderr, Write};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IEEEPrecision {
    Single = 1,
    Double = 2,
    Extended = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IEEERounding {
    Nearest = 1,
    Down = 2,
    Up = 3,
    Zero = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IEEEMask {
    Invalid = 1,
    Denormalized = 2,
    DivisionByZero = 4,
    Overflow = 8,
    Underflow = 16,
    All = 31,
    TrapInexact = 32,
}

extern "C" {
    fn gsl_ieee_read_mode_string(
        description: *const libc::c_char,
        precision: *mut libc::c_int,
        rounding: *mut libc::c_int,
        exception_mask: *mut libc::c_int,
    ) -> libc::c_int;
    fn gsl_ieee_set_mode(
        precision: libc::c_int,
        rounding: libc::c_int,
        exception_mask: libc::c_int,
    ) -> libc::c_int;
}

pub fn gsl_ieee_env_setup() {
    let mode_var = match env::var("GSL_IEEE_MODE") {
        Ok(val) if !val.is_empty() => val,
        _ => return,
    };

    let c_str = CString::new(mode_var).expect("CString::new failed");
    let mut precision = 0;
    let mut rounding = 0;
    let mut exception_mask = 0;

    unsafe {
        gsl_ieee_read_mode_string(
            c_str.as_ptr(),
            &mut precision,
            &mut rounding,
            &mut exception_mask,
        );
        gsl_ieee_set_mode(precision, rounding, exception_mask);
    }

    let mut stderr = stderr();
    let mut comma = false;

    write!(stderr, "GSL_IEEE_MODE=\"").unwrap();

    if let Some(precision) = match precision {
        1 => Some("single-precision"),
        2 => Some("double-precision"),
        3 => Some("extended-precision"),
        _ => None,
    } {
        if comma {
            write!(stderr, ",").unwrap();
        }
        write!(stderr, "{}", precision).unwrap();
        comma = true;
    }

    if let Some(rounding) = match rounding {
        1 => Some("round-to-nearest"),
        2 => Some("round-down"),
        3 => Some("round-up"),
        4 => Some("round-to-zero"),
        _ => None,
    } {
        if comma {
            write!(stderr, ",").unwrap();
        }
        write!(stderr, "{}", rounding).unwrap();
        comma = true;
    }

    if exception_mask & IEEEMask::All as i32 == IEEEMask::All as i32 {
        if comma {
            write!(stderr, ",").unwrap();
        }
        write!(stderr, "mask-all").unwrap();
        comma = true;
    } else if exception_mask & IEEEMask::All as i32 == 0 {
        if comma {
            write!(stderr, ",").unwrap();
        }
        write!(stderr, "trap-common").unwrap();
        comma = true;
    } else {
        for (mask, name) in [
            (IEEEMask::Invalid, "mask-invalid"),
            (IEEEMask::Denormalized, "mask-denormalized"),
            (IEEEMask::DivisionByZero, "mask-division-by-zero"),
            (IEEEMask::Overflow, "mask-overflow"),
            (IEEEMask::Underflow, "mask-underflow"),
        ] {
            if exception_mask & mask as i32 != 0 {
                if comma {
                    write!(stderr, ",").unwrap();
                }
                write!(stderr, "{}", name).unwrap();
                comma = true;
            }
        }
    }

    if exception_mask & IEEEMask::TrapInexact as i32 != 0 {
        if comma {
            write!(stderr, ",").unwrap();
        }
        write!(stderr, "trap-inexact").unwrap();
    }

    writeln!(stderr, "\"").unwrap();
}
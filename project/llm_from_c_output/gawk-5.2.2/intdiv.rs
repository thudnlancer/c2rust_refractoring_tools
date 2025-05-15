/*
 * intdiv.rs - Provide integer div/mod for MPFR (Rust translation)
 */

/*
 * Copyright (C) 2017, 2018, 2021, 2022, the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use gawkapi::{GawkApi, AwkExtId, AwkValue, AwkArray, AwkNumberType, AwkExtFunc};
use std::ffi::CString;
use std::os::raw::c_int;
use std::mem;

#[cfg(feature = "mpfr")]
use rug::{Integer, Float};

static mut API: Option<&'static GawkApi> = None;
static mut EXT_ID: Option<AwkExtId> = None;
static EXT_VERSION: &str = "intdiv extension: version 1.0";
static PLUGIN_IS_GPL_COMPATIBLE: c_int = 1;

/// Convert a double to its integer part
fn double_to_int(d: f64) -> f64 {
    if d >= 0.0 {
        d.floor()
    } else {
        d.ceil()
    }
}

/// Set an array element to a numeric value
fn array_set_number(array: AwkArray, sub: &str, num: f64) -> Result<(), String> {
    let index = AwkValue::from_string(sub);
    let value = AwkValue::from_number(num);
    unsafe {
        API.unwrap().set_array_element(array, index, value)
            .map_err(|e| e.to_string())
    }
}

#[cfg(feature = "mpfr")]
/// Convert an AwkValue to an Integer
fn mpz_conv(arg: &AwkValue) -> Result<Integer, String> {
    match arg.num_type() {
        AwkNumberType::Mpz => {
            Ok(arg.as_mpz().clone())
        },
        AwkNumberType::Mpfr => {
            let mpfr = arg.as_mpfr();
            if !mpfr.is_finite() {
                return Err("value is not finite".to_string());
            }
            Ok(mpfr.to_integer_floor())
        },
        AwkNumberType::Double => {
            Ok(Integer::from(double_to_int(arg.as_number()) as i64))
        },
        _ => {
            Err(format!("intdiv: invalid numeric type `{:?}`", arg.num_type()))
        }
    }
}

#[cfg(feature = "mpfr")]
/// Set an array element to an MPZ value
fn array_set_mpz(array: AwkArray, sub: &str, num: &Integer) -> Result<(), String> {
    let index = AwkValue::from_string(sub);
    let value = AwkValue::from_mpz(num);
    unsafe {
        API.unwrap().set_array_element(array, index, value)
            .map_err(|e| e.to_string())
    }
}

/// Perform integer division, return quotient and remainder in dest array
fn do_intdiv(args: &[AwkValue]) -> Result<AwkValue, String> {
    if args.len() != 3 {
        return Err("intdiv: requires exactly 3 arguments".to_string());
    }

    let nv = &args[0];
    let dv = &args[1];
    let array_param = &args[2];

    if !nv.is_number() {
        return Err("intdiv: first argument must be numeric".to_string());
    }
    if !dv.is_number() {
        return Err("intdiv: second argument must be numeric".to_string());
    }
    if !array_param.is_array() {
        return Err("intdiv: third argument must be an array".to_string());
    }

    let array = array_param.as_array();
    unsafe {
        API.unwrap().clear_array(array)?;
    }

    #[cfg(not(feature = "mpfr"))]
    {
        if nv.num_type() != AwkNumberType::Double || dv.num_type() != AwkNumberType::Double {
            static mut WARNED: bool = false;
            unsafe {
                if !WARNED {
                    API.unwrap().warning(EXT_ID.unwrap(), "intdiv: MPFR arguments converted to IEEE because this extension was not compiled with MPFR support; loss of precision may occur");
                    WARNED = true;
                }
            }
        }
    }

    #[cfg(feature = "mpfr")]
    let is_double_case = nv.num_type() == AwkNumberType::Double && dv.num_type() == AwkNumberType::Double;
    #[cfg(not(feature = "mpfr"))]
    let is_double_case = true;

    if is_double_case {
        // Regular precision case
        let num = double_to_int(nv.as_number());
        let denom = double_to_int(dv.as_number());

        if denom == 0.0 {
            return Err("intdiv: division by zero attempted".to_string());
        }

        let quotient = double_to_int(num / denom);
        let remainder = num.rem_euclid(denom);
        let remainder = double_to_int(remainder);

        array_set_number(array, "quotient", quotient)?;
        array_set_number(array, "remainder", remainder)?;
    }
    #[cfg(feature = "mpfr")]
    else {
        // Extended precision case
        let numer = mpz_conv(nv)?;
        let denom = mpz_conv(dv)?;

        if denom == 0 {
            return Err("intdiv: division by zero attempted".to_string());
        }

        let (quotient, remainder) = numer.div_rem(denom);

        array_set_mpz(array, "quotient", &quotient)?;
        array_set_mpz(array, "remainder", &remainder)?;
    }

    Ok(AwkValue::from_number(0.0))
}

#[no_mangle]
pub extern "C" fn dl_load(api: &'static GawkApi, ext_id: AwkExtId) -> c_int {
    unsafe {
        API = Some(api);
        EXT_ID = Some(ext_id);
    }

    let func = AwkExtFunc {
        name: CString::new("intdiv").unwrap(),
        func: do_intdiv,
        num_expected_args: 3,
        min_required_args: 3,
        suppress_lint: false,
        namespace: None,
    };

    unsafe {
        match API.unwrap().register_extension_function(EXT_ID.unwrap(), func) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn plugin_is_GPL_compatible() -> c_int {
    PLUGIN_IS_GPL_COMPATIBLE
}
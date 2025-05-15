/*
 * time.rs - Builtin functions that provide time-related functions.
 */

/*
 * Copyright (C) 2012, 2013, 2014, 2018, 2022, 2023,
 * the Free Software Foundation, Inc.
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

use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono::format::ParseError;
use gawk::{GawkValue, GawkError, GawkResult, GawkExtFunc};

static EXT_VERSION: &str = "time extension: version 1.2";
static mut PLUGIN_IS_GPL_COMPATIBLE: bool = true;

/*
 * Returns time since 1/1/1970 UTC as a floating point value with sub-second precision
 */
fn do_gettimeofday(_nargs: usize, _result: &mut GawkValue, _unused: &GawkExtFunc) -> GawkResult<()> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs() as f64;
            let subsec = duration.subsec_nanos() as f64 / 1_000_000_000.0;
            _result.set_number(secs + subsec);
            Ok(())
        }
        Err(e) => {
            _result.set_number(-1.0);
            Err(GawkError::new(&format!("gettimeofday: system time error: {}", e)))
        }
    }
}

/*
 * Sleeps for the specified number of seconds
 */
fn do_sleep(nargs: usize, _result: &mut GawkValue, _unused: &GawkExtFunc) -> GawkResult<()> {
    if nargs < 1 {
        _result.set_number(-1.0);
        return Err(GawkError::new("sleep: missing required numeric argument"));
    }

    let arg = _unused.get_argument(0)?;
    let secs = match arg.as_number() {
        Some(n) => n,
        None => {
            _result.set_number(-1.0);
            return Err(GawkError::new("sleep: argument must be numeric"));
        }
    };

    if secs < 0.0 {
        _result.set_number(-1.0);
        return Err(GawkError::new("sleep: argument is negative"));
    }

    let duration = Duration::from_secs_f64(secs);
    thread::sleep(duration);
    _result.set_number(0.0);
    Ok(())
}

/*
 * Parses a time string according to format and returns seconds since epoch
 */
fn do_strptime(nargs: usize, _result: &mut GawkValue, _unused: &GawkExtFunc) -> GawkResult<()> {
    if nargs < 2 {
        _result.set_number(-1.0);
        return if nargs == 0 {
            Err(GawkError::new("strptime: called with no arguments"))
        } else {
            Err(GawkError::new("strptime: requires two arguments"))
        };
    }

    let string_arg = _unused.get_argument(0)?;
    let format_arg = _unused.get_argument(1)?;

    let string = match string_arg.as_string() {
        Some(s) => s,
        None => {
            _result.set_number(-1.0);
            return Err(GawkError::new("strptime: argument 1 must be a string"));
        }
    };

    let format = match format_arg.as_string() {
        Some(f) => f,
        None => {
            _result.set_number(-1.0);
            return Err(GawkError::new("strptime: argument 2 must be a string"));
        }
    };

    match NaiveDateTime::parse_from_str(string, format) {
        Ok(naive_dt) => {
            let dt = Utc.from_utc_datetime(&naive_dt);
            _result.set_number(dt.timestamp() as f64);
            Ok(())
        }
        Err(e) => {
            _result.set_number(-1.0);
            Err(GawkError::new(&format!("strptime: parse error: {}", e)))
        }
    }
}

#[no_mangle]
pub extern "C" fn gawk_plugin_init() -> *const gawk::GawkPlugin {
    static FUNC_TABLE: &[gawk::GawkExtFuncDef] = &[
        gawk::GawkExtFuncDef {
            name: "gettimeofday\0".as_ptr() as *const i8,
            func: do_gettimeofday,
            num_expected_args: 0,
            min_required_args: 0,
        },
        gawk::GawkExtFuncDef {
            name: "sleep\0".as_ptr() as *const i8,
            func: do_sleep,
            num_expected_args: 1,
            min_required_args: 1,
        },
        gawk::GawkExtFuncDef {
            name: "strptime\0".as_ptr() as *const i8,
            func: do_strptime,
            num_expected_args: 2,
            min_required_args: 2,
        },
    ];

    static PLUGIN: gawk::GawkPlugin = gawk::GawkPlugin {
        name: "time\0".as_ptr() as *const i8,
        version: EXT_VERSION.as_ptr() as *const i8,
        func_table: FUNC_TABLE.as_ptr(),
        init_func: None,
        dl_load_func: None,
    };

    &PLUGIN
}
/*
 * ordchr.rs - Builtin functions that provide ord() and chr() functions.
 *
 * Arnold Robbins
 * arnold@skeeve.com
 * 8/2001
 * Revised 6/2004
 * Revised 5/2012
 */

/*
 * Copyright (C) 2001, 2004, 2011, 2012, 2013, 2018, 2020, 2021,
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

use gawkapi::{awk_value_t, awk_ext_func_t, awk_ext_id_t, gawk_api_t};
use std::ffi::CString;

static mut API: *const gawk_api_t = std::ptr::null();
static mut EXT_ID: awk_ext_id_t = 0;
static EXT_VERSION: &str = "ordchr extension: version 1.0";
static mut INIT_FUNC: Option<fn() -> bool> = None;

static PLUGIN_IS_GPL_COMPATIBLE: bool = true;

/*  do_ord --- return numeric value of first char of string */

unsafe extern "C" fn do_ord(nargs: i32, result: *mut awk_value_t, unused: *mut awk_ext_func_t) -> *mut awk_value_t {
    let mut str = awk_value_t::default();
    let mut ret = -1.0;

    assert!(!result.is_null());

    if get_argument(0, AWK_STRING, &mut str) {
        let s = std::ffi::CStr::from_ptr(str.str_value.str);
        if let Some(first_char) = s.to_bytes().first() {
            ret = *first_char as f64;
        }
    } else if DO_LINT {
        lintwarn(EXT_ID, "ord: first argument is not a string");
    }

    make_number(ret, result)
}

/*  do_chr --- turn numeric value into a string */

unsafe extern "C" fn do_chr(nargs: i32, result: *mut awk_value_t, unused: *mut awk_ext_func_t) -> *mut awk_value_t {
    let mut num = awk_value_t::default();
    let mut ret: u8 = 0;
    let mut val = 0.0;
    let mut str = [0u8; 2];

    str[0] = 0;
    str[1] = 0;

    assert!(!result.is_null());

    if get_argument(0, AWK_NUMBER, &mut num) {
        val = num.num_value;
        ret = val as u8;
        ret &= 0xff;
        str[0] = ret;
        str[1] = 0;
    } else if DO_LINT {
        lintwarn(EXT_ID, "chr: first argument is not a number");
    }

    let c_str = CString::new(&str[..]).unwrap();
    make_const_string(c_str.as_ptr(), 1, result)
}

static FUNC_TABLE: [awk_ext_func_t; 2] = [
    awk_ext_func_t {
        name: CString::new("ord").unwrap().into_raw(),
        func: Some(do_ord),
        num_expected_args: 1,
        min_required_args: 1,
        suppress_lint: false,
        data: std::ptr::null_mut(),
    },
    awk_ext_func_t {
        name: CString::new("chr").unwrap().into_raw(),
        func: Some(do_chr),
        num_expected_args: 1,
        min_required_args: 1,
        suppress_lint: false,
        data: std::ptr::null_mut(),
    },
];

#[no_mangle]
pub unsafe extern "C" fn dl_load(
    api_ptr: *const gawk_api_t,
    ext_id: awk_ext_id_t,
) -> *mut awk_ext_func_t {
    API = api_ptr;
    EXT_ID = ext_id;
    FUNC_TABLE.as_ptr() as *mut awk_ext_func_t
}
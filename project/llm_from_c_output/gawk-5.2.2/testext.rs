/*
 * testext.rs - tests for the extension API.
 *
 * Copyright (C) 2012-2022, the Free Software Foundation, Inc.
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
 */

use gawkapi::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;
use std::fmt;

static mut API: *const gawk_api_t = ptr::null();
static mut EXT_ID: awk_ext_id_t = awk_ext_id_t { version: ptr::null() };
static EXT_VERSION: &str = "testext extension: version 1.0\0";

static mut PLUGIN_IS_GPL_COMPATIBLE: c_int = 1;

struct NVal {
    name: &'static str,
    val: f64,
}

unsafe extern "C" fn valrep2str(value: *const awk_value_t) -> *const c_char {
    static mut BUF: [u8; 1024] = [0; 1024]; // BUFSIZ
    let size = BUF.len() - 3;

    match (*value).val_type {
        AWK_UNDEFINED => {
            BUF[..11].copy_from_slice(b"<undefined>");
            BUF[11] = 0;
        }
        AWK_ARRAY => {
            BUF[..7].copy_from_slice(b"<array>");
            BUF[7] = 0;
        }
        AWK_SCALAR => {
            BUF[..8].copy_from_slice(b"<scalar>");
            BUF[8] = 0;
        }
        AWK_VALUE_COOKIE => {
            BUF[..15].copy_from_slice(b"<value-cookie>");
            BUF[15] = 0;
        }
        AWK_REGEX | AWK_STRNUM | AWK_STRING => {
            let str_len = (*value).str_value.len as usize;
            let copy_len = if str_len < size { str_len } else { size };
            let src = (*value).str_value.str;
            let dst = &mut BUF[..copy_len];
            ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), copy_len);
            BUF[copy_len] = 0;
        }
        AWK_BOOL => {
            let str_len = (*value).str_value.len as usize;
            let copy_len = if str_len + 8 < size { str_len } else { size };
            let src = (*value).str_value.str;
            let dst = &mut BUF[..copy_len];
            ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), copy_len);
            BUF[copy_len] = 0;
        }
        AWK_NUMBER => {
            let num = (*value).num_value;
            let s = format!("{}", num);
            BUF[..s.len()].copy_from_slice(s.as_bytes());
            BUF[s.len()] = 0;
        }
        _ => {
            BUF[0] = 0;
        }
    }

    BUF.as_ptr() as *const c_char
}

unsafe extern "C" fn dump_array_and_delete(
    nargs: c_int,
    result: *mut awk_value_t,
    _unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    assert!(!result.is_null());
    (*API).make_number.unwrap()(0.0, result);

    if nargs != 2 {
        println!("dump_array_and_delete: nargs not right ({} should be 2)", nargs);
        return result;
    }

    let mut value = mem::zeroed();
    let mut value2 = mem::zeroed();
    let mut value3 = mem::zeroed();

    if (*API).get_argument.unwrap()(0, AWK_STRING, &mut value) != 0 {
        let name = CStr::from_ptr(value.str_value.str).to_string_lossy();
        if (*API).sym_lookup.unwrap()(value.str_value.str, AWK_ARRAY, &mut value2) != 0 {
            println!("dump_array_and_delete: sym_lookup of {} passed", name);
        } else {
            println!("dump_array_and_delete: sym_lookup of {} failed", name);
            return result;
        }
    } else {
        println!("dump_array_and_delete: get_argument(0) failed");
        return result;
    }

    let mut count = 0;
    if (*API).get_element_count.unwrap()(value2.array_cookie, &mut count) == 0 {
        println!("dump_array_and_delete: get_element_count failed");
        return result;
    }

    println!(
        "dump_array_and_delete: incoming size is {}",
        count as u64
    );

    let mut flat_array: *mut awk_flat_array_t = ptr::null_mut();
    if (*API).flatten_array.unwrap()(value2.array_cookie, &mut flat_array) == 0 {
        println!("dump_array_and_delete: could not flatten array");
        return result;
    }

    if (*flat_array).count != count {
        println!(
            "dump_array_and_delete: flat_array->count ({}) != count ({})",
            (*flat_array).count as u64, count as u64
        );
        return result;
    }

    if (*API).get_argument.unwrap()(1, AWK_STRING, &mut value3) == 0 {
        println!("dump_array_and_delete: get_argument(1) failed");
        return result;
    }

    for i in 0..(*flat_array).count as isize {
        let elem = &mut *(*flat_array).elements.offset(i);
        let index_str = CStr::from_ptr(elem.index.str_value.str).to_string_lossy();
        let val_str = CStr::from_ptr(valrep2str(&elem.value)).to_string_lossy();

        println!(
            "\t{}[\"{}\"] = {}",
            CStr::from_ptr(value.str_value.str).to_string_lossy(),
            index_str,
            val_str
        );

        let delete_str = CStr::from_ptr(value3.str_value.str);
        if delete_str.to_bytes() == elem.index.str_value.str[..elem.index.str_value.len as usize] {
            elem.flags |= AWK_ELEMENT_DELETE;
            println!(
                "dump_array_and_delete: marking element \"{}\" for deletion",
                index_str
            );
        }
    }

    if (*API).release_flattened_array.unwrap()(value2.array_cookie, flat_array) == 0 {
        println!("dump_array_and_delete: could not release flattened array");
        return result;
    }

    (*API).make_number.unwrap()(1.0, result);
    result
}

// ... (其他函数实现类似)

static FUNC_TABLE: [awk_ext_func_t; 16] = [
    awk_ext_func_t {
        name: b"dump_array_and_delete\0".as_ptr() as *const c_char,
        func: Some(dump_array_and_delete),
        num_expected_args: 2,
        min_required_args: 2,
        suppress_lint: 0,
        data: ptr::null_mut(),
    },
    // ... (其他函数定义)
];

#[no_mangle]
pub unsafe extern "C" fn dl_load(
    api: *const gawk_api_t,
    ext_id: *mut awk_ext_id_t,
    _: *mut c_void,
) -> c_int {
    API = api;
    EXT_ID = *ext_id;

    (*ext_id).version = EXT_VERSION.as_ptr() as *const c_char;

    if (*api).register_extension.unwrap()(ext_id, FUNC_TABLE.as_ptr(), FUNC_TABLE.len() as size_t) == 0 {
        eprintln!("testext: could not register extension");
        return 0;
    }

    if init_testext() == 0 {
        return 0;
    }

    1
}

unsafe fn init_testext() -> c_int {
    let mut value = mem::zeroed();
    
    // ... (初始化逻辑)

    1
}
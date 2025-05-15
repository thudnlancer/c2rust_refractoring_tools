/*
 * revoutput.rs --- Provide an output wrapper that reverses lines.
 *
 * Arnold Robbins
 * arnold@skeeve.com
 * Written 8/2012
 */

/*
 * Copyright (C) 2012, 2013, 2015, 2018 the Free Software Foundation, Inc.
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

use gawkapi::{AwkBool, AwkExtId, AwkOutputBuf, AwkOutputWrapper, AwkValue, AwkValueType};
use std::io::Write;

static mut API: Option<&'static gawkapi::GawkApi> = None;
static mut EXT_ID: Option<AwkExtId> = None;
static EXT_VERSION: &str = "revoutput extension: version 1.1";

static mut INIT_FUNC: Option<fn() -> AwkBool> = Some(init_revoutput);

static PLUGIN_IS_GPL_COMPATIBLE: bool = true;

/* rev_fwrite --- write out characters in reverse order */
fn rev_fwrite(buf: &[u8], fp: &mut dyn Write, _opaque: Option<&mut dyn std::any::Any>) -> usize {
    let mut bytes_written = 0;
    for &byte in buf.iter().rev() {
        if fp.write_all(&[byte]).is_ok() {
            bytes_written += 1;
        }
    }
    bytes_written
}

/* revoutput_can_take_file --- return true if we want the file */
fn revoutput_can_take_file(outbuf: Option<&AwkOutputBuf>) -> AwkBool {
    let api = unsafe { API.expect("API not initialized") };
    let ext_id = unsafe { EXT_ID.expect("EXT_ID not initialized") };

    if outbuf.is_none() {
        return AwkBool::False;
    }

    let mut value = AwkValue::default();
    if api
        .sym_lookup("REVOUT", AwkValueType::Number, &mut value)
        .is_err()
    {
        return AwkBool::False;
    }

    if value.num_value != 0.0 {
        AwkBool::True
    } else {
        AwkBool::False
    }
}

/*
 * revoutput_take_control_of --- set up output wrapper.
 * We can assume that revoutput_can_take_file just returned true,
 * and no state has changed since then.
 */
fn revoutput_take_control_of(outbuf: &mut AwkOutputBuf) -> AwkBool {
    outbuf.gawk_fwrite = Some(rev_fwrite);
    outbuf.redirected = AwkBool::True;
    AwkBool::True
}

static OUTPUT_WRAPPER: AwkOutputWrapper = AwkOutputWrapper {
    name: "revoutput",
    can_take_file: Some(revoutput_can_take_file),
    take_control_of: Some(revoutput_take_control_of),
    close_func: None,
};

/* init_revoutput --- set things ups */
fn init_revoutput() -> AwkBool {
    let api = unsafe { API.expect("API not initialized") };
    let ext_id = unsafe { EXT_ID.expect("EXT_ID not initialized") };

    api.register_output_wrapper(&OUTPUT_WRAPPER);

    let mut value = AwkValue::default();
    if api
        .sym_lookup("REVOUT", AwkValueType::Scalar, &mut value)
        .is_err()
    {
        /* only install it if not there, e.g. -v REVOUT=1 */
        api.make_number(0.0, &mut value); /* init to false */
        if api.sym_update("REVOUT", &value).is_err() {
            api.warning(
                ext_id,
                &format!("revoutput: could not initialize REVOUT variable"),
            );
            return AwkBool::False;
        }
    }

    AwkBool::True
}

static FUNC_TABLE: [gawkapi::AwkExtFunc; 1] = [gawkapi::AwkExtFunc {
    name: None,
    func: None,
    num_expected_args: 0,
    min_required_args: 0,
    suppress_lint: AwkBool::False,
    namespace: None,
}];

#[no_mangle]
pub extern "C" fn dl_load(
    api_ptr: *const gawkapi::GawkApi,
    ext_id: AwkExtId,
    _argc: usize,
    _argv: *const *const std::os::raw::c_char,
) -> *const gawkapi::AwkExtFunc {
    unsafe {
        API = Some(&*api_ptr);
        EXT_ID = Some(ext_id);
    }
    &FUNC_TABLE[0] as *const gawkapi::AwkExtFunc
}
/*
 * inplace.rs - Provide support for in-place editing.
 */

/*
 * Copyright (C) 2013-2015, 2017, 2018, the Free Software Foundation, Inc.
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

use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions, rename, remove_file, metadata, set_permissions};
use std::io::{self, Seek, SeekFrom, Write};
use std::os::unix::fs::{PermissionsExt, chown};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::Mutex;
use libc::{mode_t, uid_t, gid_t};
use gawkapi::{GawkApi, AwkExtId, AwkValue, AwkExtFunc, AwkBool, AwkString};

static API: &'static GawkApi = unsafe { &*gawk_api };
static EXT_ID: AwkExtId = 0;
static EXT_VERSION: &'static str = "inplace extension: version 1.0";

static PLUGIN_IS_GPL_COMPATIBLE: bool = true;

struct State {
    tname: Option<PathBuf>,
    default_stdout: Option<File>,
    pos: Option<u64>,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        tname: None,
        default_stdout: None,
        pos: None,
    });
}

extern "C" fn at_exit(_data: *mut libc::c_void, _exit_status: libc::c_int) {
    let mut state = STATE.lock().unwrap();
    if let Some(ref tname) = state.tname {
        let _ = remove_file(tname);
        state.tname = None;
    }
}

fn invalid_filename(filename: &AwkString) -> bool {
    filename.len == 0 || (filename.len == 1 && unsafe { *filename.str == b'-' as libc::c_char })
}

extern "C" fn do_inplace_begin(
    nargs: libc::c_int,
    result: *mut AwkValue,
    _unused: *mut AwkExtFunc,
) -> *mut AwkValue {
    let mut filename = AwkValue::default();
    let mut suffix = AwkValue::default();
    let mut state = STATE.lock().unwrap();

    assert!(!result.is_null());
    io::stdout().flush().unwrap();

    if state.tname.is_some() {
        API.fatal(EXT_ID, "inplace::begin: in-place editing already active\0".as_ptr());
    }

    if nargs != 2 {
        API.fatal(
            EXT_ID,
            format!("inplace::begin: expects 2 arguments but called with {}\0", nargs).as_ptr(),
        );
    }

    if API.get_argument(0, gawkapi::AWK_STRING, &mut filename) == 0 {
        API.fatal(
            EXT_ID,
            "inplace::begin: cannot retrieve 1st argument as a string filename\0".as_ptr(),
        );
    }

    if invalid_filename(&filename.str_value) {
        API.warning(
            EXT_ID,
            format!(
                "inplace::begin: disabling in-place editing for invalid FILENAME `{}'\0",
                unsafe { CStr::from_ptr(filename.str_value.str) }.to_string_lossy()
            )
            .as_ptr(),
        );
        API.unset_ERRNO();
        API.make_number(-1, result);
        return result;
    }

    let filename_str = unsafe { CStr::from_ptr(filename.str_value.str) };
    let path = Path::new(filename_str.to_str().unwrap());
    let metadata = match metadata(path) {
        Ok(m) => m,
        Err(e) => {
            API.warning(
                EXT_ID,
                format!(
                    "inplace::begin: Cannot stat `{}' ({})\0",
                    path.display(),
                    e
                )
                .as_ptr(),
            );
            API.update_ERRNO_int(e.raw_os_error().unwrap());
            API.make_number(-1, result);
            return result;
        }
    };

    if !metadata.is_file() {
        API.warning(
            EXT_ID,
            format!("inplace::begin: `{}' is not a regular file\0", path.display()).as_ptr(),
        );
        API.unset_ERRNO();
        API.make_number(-1, result);
        return result;
    }

    let mut tname = path.to_path_buf();
    tname.set_extension("gawk.XXXXXX");
    let tname_str = tname.to_str().unwrap().to_owned() + "\0";

    let fd = match API.mkstemp(tname_str.as_ptr() as *mut libc::c_char) {
        -1 => {
            API.fatal(
                EXT_ID,
                format!(
                    "inplace::begin: mkstemp(`{}') failed ({})\0",
                    tname.display(),
                    io::Error::last_os_error()
                )
                .as_ptr(),
            );
            return result;
        }
        fd => fd,
    };

    if let Err(e) = chown(&tname, Some(metadata.uid()), Some(metadata.gid())) {
        let _ = chown(&tname, None, Some(metadata.gid()));
    }

    if let Err(e) = set_permissions(&tname, metadata.permissions()) {
        API.fatal(
            EXT_ID,
            format!(
                "inplace::begin: chmod failed ({})\0",
                e
            )
            .as_ptr(),
        );
    }

    io::stdout().flush().unwrap();
    state.pos = Some(io::stdout().seek(SeekFrom::Current(0)).unwrap());
    state.default_stdout = Some(File::try_clone(&io::stdout()).unwrap());
    
    let temp_file = unsafe { File::from_raw_fd(fd) };
    let stdout = io::stdout();
    unsafe {
        libc::dup2(temp_file.as_raw_fd(), libc::STDOUT_FILENO);
    }
    io::stdout().seek(SeekFrom::Start(0)).unwrap();

    state.tname = Some(tname);
    API.make_number(0, result);
    result
}

extern "C" fn do_inplace_end(
    nargs: libc::c_int,
    result: *mut AwkValue,
    _unused: *mut AwkExtFunc,
) -> *mut AwkValue {
    let mut filename = AwkValue::default();
    let mut suffix = AwkValue::default();
    let mut state = STATE.lock().unwrap();

    assert!(!result.is_null());

    if nargs != 2 {
        API.fatal(
            EXT_ID,
            format!("inplace::end: expects 2 arguments but called with {}\0", nargs).as_ptr(),
        );
    }

    if API.get_argument(0, gawkapi::AWK_STRING, &mut filename) == 0 {
        API.fatal(
            EXT_ID,
            "inplace::end: cannot retrieve 1st argument as a string filename\0".as_ptr(),
        );
    }

    if API.get_argument(1, gawkapi::AWK_STRING, &mut suffix) == 0 {
        suffix.str_value.str = ptr::null_mut();
    }

    if state.tname.is_none() {
        if !invalid_filename(&filename.str_value) {
            API.warning(
                EXT_ID,
                "inplace::end: in-place editing not active\0".as_ptr(),
            );
        }
        API.make_number(0, result);
        return result;
    }

    io::stdout().flush().unwrap();
    if let Some(stdout) = state.default_stdout.take() {
        unsafe {
            libc::dup2(stdout.as_raw_fd(), libc::STDOUT_FILENO);
        }
    }

    if let Some(pos) = state.pos.take() {
        io::stdout().seek(SeekFrom::Start(pos)).unwrap();
    }

    let filename_str = unsafe { CStr::from_ptr(filename.str_value.str) };
    let path = Path::new(filename_str.to_str().unwrap());

    if !suffix.str_value.str.is_null() && unsafe { *suffix.str_value.str } != 0 {
        let suffix_str = unsafe { CStr::from_ptr(suffix.str_value.str) };
        let bakname = path.with_extension(suffix_str.to_str().unwrap());
        let _ = remove_file(&bakname);
        if let Err(e) = std::fs::hard_link(path, &bakname) {
            API.fatal(
                EXT_ID,
                format!(
                    "inplace::end: link(`{}', `{}') failed ({})\0",
                    path.display(),
                    bakname.display(),
                    e
                )
                .as_ptr(),
            );
        }
    }

    if let Some(ref tname) = state.tname.take() {
        if let Err(e) = rename(tname, path) {
            API.fatal(
                EXT_ID,
                format!(
                    "inplace::end: rename(`{}', `{}') failed ({})\0",
                    tname.display(),
                    path.display(),
                    e
                )
                .as_ptr(),
            );
        }
    }

    API.make_number(0, result);
    result
}

static FUNC_TABLE: [AwkExtFunc; 2] = [
    AwkExtFunc {
        name: "begin\0".as_ptr(),
        func: Some(do_inplace_begin),
        num_expected_args: 2,
        min_required_args: 2,
        suppress_lint: 0,
        namespace: ptr::null(),
    },
    AwkExtFunc {
        name: "end\0".as_ptr(),
        func: Some(do_inplace_end),
        num_expected_args: 2,
        min_required_args: 2,
        suppress_lint: 0,
        namespace: ptr::null(),
    },
];

extern "C" fn init_inplace() -> AwkBool {
    API.atexit(at_exit, ptr::null_mut());
    1
}

#[no_mangle]
pub extern "C" fn dl_load(
    api: *const GawkApi,
    ext_id: AwkExtId,
) -> *mut AwkExtFunc {
    unsafe {
        gawk_api = api;
        EXT_ID = ext_id;
    }
    API.register_extension(EXT_ID, EXT_VERSION.as_ptr(), init_inplace);
    FUNC_TABLE.as_ptr() as *mut AwkExtFunc
}
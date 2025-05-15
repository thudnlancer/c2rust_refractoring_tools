/*
 * revtwoway.rs --- Provide a two-way processor that reverses lines.
 *
 * Translated from C to Rust by an AI assistant
 */

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;
use std::cmp;
use libc::{FILE, size_t};
use gawkapi::{GawkApi, AwkExtId, AwkBool, AwkInputBuf, AwkOutputBuf, AwkFieldWidthInfo, AwkTwoWayProcessor, AwkExtFunc, register_two_way_processor};

static mut API: *const GawkApi = ptr::null();
static mut EXT_ID: AwkExtId = 0;
static EXT_VERSION: &str = "revtwoway extension: version 1.0\0";

static mut MAX_FDS: size_t = 0;

struct TwoWayProcData {
    size: size_t,
    len: size_t,
    data: Option<Vec<u8>>,
    in_use: size_t,
}

impl TwoWayProcData {
    fn new() -> Self {
        TwoWayProcData {
            size: 0,
            len: 0,
            data: None,
            in_use: 0,
        }
    }
}

fn close_two_proc_data(proc_data: &mut TwoWayProcData) {
    if proc_data.in_use > 1 {
        proc_data.in_use -= 1;
        return;
    }

    proc_data.data = None;
}

fn rev2way_get_record(
    out: *mut *mut c_char,
    iobuf: *mut AwkInputBuf,
    errcode: *mut c_int,
    rt_start: *mut *mut c_char,
    rt_len: *mut size_t,
    unused: *mut *const AwkFieldWidthInfo,
) -> c_int {
    unsafe {
        if out.is_null() || iobuf.is_null() || (*iobuf).opaque.is_null() {
            return -1;
        }

        let proc_data = &mut *((*iobuf).opaque as *mut TwoWayProcData);
        if proc_data.len == 0 {
            return 0;
        }

        if let Some(ref data) = proc_data.data {
            *out = data.as_ptr() as *mut c_char;
            let mut len = proc_data.len as c_int;
            proc_data.len = 0;

            *rt_len = 0;
            if !data.is_empty() && data[data.len() - 1] == b'\n' {
                let mut pos = data.len() - 1;
                while pos > 0 && data[pos] == b'\n' {
                    pos -= 1;
                    *rt_len += 1;
                }
                *rt_start = data.as_ptr().add(pos + 1) as *mut c_char;
                len = (pos + 1) as c_int;
            }

            len
        } else {
            0
        }
    }
}

fn rev2way_close(iobuf: *mut AwkInputBuf) {
    unsafe {
        if iobuf.is_null() || (*iobuf).opaque.is_null() {
            return;
        }

        let proc_data = &mut *((*iobuf).opaque as *mut TwoWayProcData);
        close_two_proc_data(proc_data);
        (*iobuf).fd = -1;
    }
}

fn rev2way_fwrite(
    buf: *const c_void,
    size: size_t,
    count: size_t,
    fp: *mut FILE,
    opaque: *mut c_void,
) -> size_t {
    unsafe {
        if opaque.is_null() {
            return 0;
        }

        let proc_data = &mut *(opaque as *mut TwoWayProcData);
        let amount = size * count;
        let src = buf as *const u8;

        if amount > proc_data.size || proc_data.len > 0 {
            let new_size = proc_data.size + amount;
            let mut new_data = Vec::with_capacity(new_size);
            if let Some(ref data) = proc_data.data {
                new_data.extend_from_slice(data);
            }
            proc_data.data = Some(new_data);
            proc_data.size = new_size;
        }

        if let Some(ref mut data) = proc_data.data {
            for i in (0..amount).rev() {
                data.push(*src.add(i));
            }
            proc_data.len += amount;
        }

        amount
    }
}

fn rev2way_fflush(_fp: *mut FILE, _opaque: *mut c_void) -> c_int {
    0
}

fn rev2way_ferror(_fp: *mut FILE, _opaque: *mut c_void) -> c_int {
    0
}

fn rev2way_fclose(_fp: *mut FILE, opaque: *mut c_void) -> c_int {
    unsafe {
        if opaque.is_null() {
            return -1;
        }

        let proc_data = &mut *(opaque as *mut TwoWayProcData);
        close_two_proc_data(proc_data);
        0
    }
}

fn revtwoway_can_take_two_way(name: *const c_char) -> AwkBool {
    unsafe {
        if name.is_null() {
            return 0;
        }
        let name_str = CStr::from_ptr(name).to_string_lossy();
        if name_str == "/magic/mirror" {
            1
        } else {
            0
        }
    }
}

fn revtwoway_take_control_of(
    name: *const c_char,
    inbuf: *mut AwkInputBuf,
    outbuf: *mut AwkOutputBuf,
) -> AwkBool {
    unsafe {
        if inbuf.is_null() || outbuf.is_null() {
            return 0;
        }

        let mut proc_data = Box::new(TwoWayProcData::new());
        proc_data.in_use = 2;

        if MAX_FDS + 1 == 0 {
            MAX_FDS = libc::getdtablesize() as size_t;
        }

        (*inbuf).get_record = Some(rev2way_get_record);
        (*inbuf).close_func = Some(rev2way_close);
        (*inbuf).fd = MAX_FDS as c_int;
        (*inbuf).opaque = Box::into_raw(proc_data) as *mut c_void;

        (*outbuf).fp = MAX_FDS as *mut FILE;
        (*outbuf).opaque = (*inbuf).opaque;
        (*outbuf).gawk_fwrite = Some(rev2way_fwrite);
        (*outbuf).gawk_fflush = Some(rev2way_fflush);
        (*outbuf).gawk_ferror = Some(rev2way_ferror);
        (*outbuf).gawk_fclose = Some(rev2way_fclose);
        (*outbuf).redirected = 1;

        MAX_FDS += 1;
        1
    }
}

static TWO_WAY_PROCESSOR: AwkTwoWayProcessor = AwkTwoWayProcessor {
    name: b"revtwoway\0".as_ptr() as *const c_char,
    can_take_file: Some(revtwoway_can_take_two_way),
    take_control_of: Some(revtwoway_take_control_of),
    next: ptr::null(),
};

fn init_revtwoway() -> AwkBool {
    unsafe {
        register_two_way_processor(&TWO_WAY_PROCESSOR);
        MAX_FDS = libc::getdtablesize() as size_t;
        1
    }
}

static FUNC_TABLE: [AwkExtFunc; 1] = [AwkExtFunc {
    name: ptr::null(),
    func: None,
    num_expected_args: 0,
    min_required_args: 0,
    suppress_lint: 0,
    next: ptr::null(),
}];

#[no_mangle]
pub extern "C" fn dl_load(api: *const GawkApi, ext_id: AwkExtId) -> *const AwkExtFunc {
    unsafe {
        API = api;
        EXT_ID = ext_id;
        init_revtwoway();
        FUNC_TABLE.as_ptr()
    }
}
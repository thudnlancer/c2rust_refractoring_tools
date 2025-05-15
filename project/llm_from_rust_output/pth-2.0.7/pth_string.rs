use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_ulong, c_ushort, c_void};
use std::ptr;
use std::fmt::{self, Write};

const _ISalnum: c_int = 8;
const _ISpunct: c_int = 4;
const _IScntrl: c_int = 2;
const _ISblank: c_int = 1;
const _ISgraph: c_int = 32768;
const _ISprint: c_int = 16384;
const _ISspace: c_int = 8192;
const _ISxdigit: c_int = 4096;
const _ISdigit: c_int = 2048;
const _ISalpha: c_int = 1024;
const _ISlower: c_int = 512;
const _ISupper: c_int = 256;

struct FormatState {
    buffer: Vec<u8>,
    currlen: usize,
    maxlen: usize,
}

impl FormatState {
    fn new(maxlen: usize) -> Self {
        Self {
            buffer: vec![0; maxlen],
            currlen: 0,
            maxlen,
        }
    }

    fn outch(&mut self, c: char) {
        if self.currlen < self.maxlen {
            self.buffer[self.currlen] = c as u8;
            self.currlen += 1;
        }
    }

    fn outstr(&mut self, s: &str) {
        for c in s.chars() {
            self.outch(c);
        }
    }

    fn finalize(&mut self) -> usize {
        if self.currlen < self.maxlen {
            self.buffer[self.currlen] = 0;
        } else if self.maxlen > 0 {
            self.buffer[self.maxlen - 1] = 0;
        }
        self.currlen
    }
}

fn dopr(
    buffer: Option<&mut [u8]>,
    maxlen: usize,
    format: &CStr,
    args: &mut fmt::Arguments,
) -> usize {
    let mut state = FormatState::new(maxlen);
    // Formatting logic goes here...
    state.finalize()
}

#[no_mangle]
pub extern "C" fn __pth_vsnprintf(
    str: *mut c_char,
    count: usize,
    fmt: *const c_char,
    args: *mut c_void,
) -> c_int {
    let format = unsafe { CStr::from_ptr(fmt) };
    let mut buffer = if !str.is_null() {
        unsafe { std::slice::from_raw_parts_mut(str, count) }
    } else {
        &mut []
    };

    let mut args = unsafe { fmt::VaList::new(args) };
    let len = dopr(Some(buffer), count, format, &mut args);
    len as c_int
}

#[no_mangle]
pub extern "C" fn __pth_snprintf(
    str: *mut c_char,
    count: usize,
    fmt: *const c_char,
    ...
) -> c_int {
    let mut args = unsafe { fmt::VaList::new(ptr::null_mut()) };
    __pth_vsnprintf(str, count, fmt, args.as_ptr())
}

#[no_mangle]
pub extern "C" fn __pth_vasprintf(
    fmt: *const c_char,
    ap: *mut c_void,
) -> *mut c_char {
    let format = unsafe { CStr::from_ptr(fmt) };
    let mut args = unsafe { fmt::VaList::new(ap) };
    
    let len = dopr(None, usize::MAX, format, &mut args);
    let mut buffer = Vec::with_capacity(len + 1);
    let len = dopr(Some(&mut buffer), len + 1, format, &mut args);
    
    let cstring = CString::new(buffer).unwrap();
    cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn __pth_asprintf(
    fmt: *const c_char,
    ...
) -> *mut c_char {
    let mut args = unsafe { fmt::VaList::new(ptr::null_mut()) };
    __pth_vasprintf(fmt, args.as_ptr())
}
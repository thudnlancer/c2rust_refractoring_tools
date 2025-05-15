use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ushort};

pub type size_t = usize;
pub type error_t = c_int;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ArgpOption {
    pub name: *const c_char,
    pub key: c_int,
    pub arg: *const c_char,
    pub flags: c_int,
    pub doc: *const c_char,
    pub group: c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ArgpState {
    pub root_argp: *const Argp,
    pub argc: c_int,
    pub argv: *mut *mut c_char,
    pub next: c_int,
    pub flags: c_uint,
    pub arg_num: c_uint,
    pub quoted: c_int,
    pub input: *mut std::ffi::c_void,
    pub child_inputs: *mut *mut std::ffi::c_void,
    pub hook: *mut std::ffi::c_void,
    pub name: *mut c_char,
    pub err_stream: *mut std::io::FILE,
    pub out_stream: *mut std::io::FILE,
    pub pstate: *mut std::ffi::c_void,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Argp {
    pub options: *const ArgpOption,
    pub parser: Option<extern "C" fn(c_int, *mut c_char, *mut ArgpState) -> error_t>,
    pub args_doc: *const c_char,
    pub doc: *const c_char,
    pub children: *const ArgpChild,
    pub help_filter: Option<extern "C" fn(c_int, *const c_char, *mut std::ffi::c_void) -> *mut c_char>,
    pub argp_domain: *const c_char,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ArgpChild {
    pub argp: *const Argp,
    pub flags: c_int,
    pub header: *const c_char,
    pub group: c_int,
}

pub fn option_is_end(opt: &ArgpOption) -> bool {
    opt.key == 0 && opt.name.is_null() && opt.doc.is_null() && opt.group == 0
}

pub fn option_is_short(opt: &ArgpOption) -> bool {
    if opt.flags & 0x8 != 0 {
        false
    } else {
        let key = opt.key;
        key > 0 && key <= 127 * 2 + 1 && {
            unsafe {
                let ctype_b = *__ctype_b_loc();
                let mask = (*ctype_b.offset(key as isize) as c_int) & 0x00004000;
                mask != 0
            }
        }
    }
}

extern "C" {
    fn __ctype_b_loc() -> *mut *const c_ushort;
    fn argp_state_help(state: *const ArgpState, stream: *mut std::io::FILE, flags: c_uint);
}

pub fn argp_usage(state: &ArgpState) {
    unsafe {
        argp_state_help(
            state as *const ArgpState,
            std::io::stderr(),
            0x2 | 0x4 | 0x100,
        );
    }
}
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong, c_long, c_ushort, c_schar, c_uint};
use std::ptr;

pub type size_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type error_t = c_int;

#[derive(Debug, Clone)]
pub struct IoFile {
    pub _flags: c_int,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _offset: __off64_t,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}

#[derive(Debug, Clone)]
pub struct IoMarker {
    pub _pos: c_int,
}

#[derive(Debug, Clone)]
pub struct ArgpOption {
    pub name: Option<String>,
    pub key: c_int,
    pub arg: Option<String>,
    pub flags: c_int,
    pub doc: Option<String>,
    pub group: c_int,
}

#[derive(Debug, Clone)]
pub struct Argp {
    pub options: Vec<ArgpOption>,
    pub parser: Option<fn(c_int, Option<String>, &mut ArgpState) -> error_t>,
    pub args_doc: Option<String>,
    pub doc: Option<String>,
    pub children: Vec<ArgpChild>,
    pub help_filter: Option<fn(c_int, Option<String>, &mut ()) -> Option<String>>,
    pub argp_domain: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ArgpChild {
    pub argp: Box<Argp>,
    pub flags: c_int,
    pub header: Option<String>,
    pub group: c_int,
}

#[derive(Debug, Clone)]
pub struct ArgpState {
    pub root_argp: Box<Argp>,
    pub argc: c_int,
    pub argv: Vec<Option<String>>,
    pub next: c_int,
    pub flags: c_uint,
    pub arg_num: c_uint,
    pub quoted: c_int,
    pub input: Option<Box<dyn std::any::Any>>,
    pub child_inputs: Vec<Option<Box<dyn std::any::Any>>>,
    pub hook: Option<Box<dyn std::any::Any>>,
    pub name: Option<String>,
    pub err_stream: Option<Box<IoFile>>,
    pub out_stream: Option<Box<IoFile>>,
    pub pstate: Option<Box<dyn std::any::Any>>,
}

pub static mut ARGP_PROGRAM_VERSION_HOOK: Option<fn(&mut IoFile, &mut ArgpState)> = None;
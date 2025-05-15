use std::ffi::CStr;
use std::io::{self, Write};
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn __printf__(format: *const std::os::raw::c_char, ...) -> c_int {
    unsafe {
        let mut args: std::ffi::VaList = std::ffi::VaList::new();
        let retval = rpl_vfprintf(stdout, format, args.as_va_list());
        retval
    }
}

fn rpl_vfprintf(stream: *mut FILE, format: *const std::os::raw::c_char, args: std::ffi::VaList) -> c_int {
    unsafe {
        let format_str = CStr::from_ptr(format).to_string_lossy();
        let mut buffer = Vec::new();
        
        match std::fmt::write(&mut buffer, format_args!("{}", format_str)) {
            Ok(_) => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                match handle.write_all(&buffer) {
                    Ok(_) => buffer.len() as c_int,
                    Err(_) => -1,
                }
            }
            Err(_) => -1,
        }
    }
}

// Keep the original types for compatibility
pub type FILE = _IO_FILE;
pub type _IO_FILE = _IO_FILE;
pub type _IO_marker = _IO_marker;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: std::os::raw::c_long,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: std::os::raw::c_long,
    pub __pad1: *mut std::ffi::c_void,
    pub __pad2: *mut std::ffi::c_void,
    pub __pad3: *mut std::ffi::c_void,
    pub __pad4: *mut std::ffi::c_void,
    pub __pad5: std::os::raw::c_ulong,
    pub _mode: c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: c_int,
}

#[allow(non_upper_case_globals)]
pub static mut stdout: *mut FILE = std::ptr::null_mut();
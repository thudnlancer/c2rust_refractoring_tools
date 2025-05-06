#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type gsl_stream_handler_t = unsafe extern "C" fn(
    *const i8,
    *const i8,
    i32,
    *const i8,
) -> ();
#[no_mangle]
pub static mut gsl_stream: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut gsl_stream_handler: Option<gsl_stream_handler_t> = None;
#[no_mangle]
pub unsafe extern "C" fn gsl_stream_printf(
    mut label: *const i8,
    mut file: *const i8,
    mut line: i32,
    mut reason: *const i8,
) {
    if gsl_stream.is_null() {
        gsl_stream = stderr;
    }
    if gsl_stream_handler.is_some() {
        (Some(gsl_stream_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")(label, file, line, reason);
        return;
    }
    fprintf(
        gsl_stream,
        b"gsl: %s:%d: %s: %s\n\0" as *const u8 as *const i8,
        file,
        line,
        label,
        reason,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gsl_set_stream_handler(
    mut new_handler: Option<gsl_stream_handler_t>,
) -> Option<gsl_stream_handler_t> {
    let mut previous_handler: Option<gsl_stream_handler_t> = gsl_stream_handler;
    gsl_stream_handler = new_handler;
    return previous_handler;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_set_stream(mut new_stream: *mut FILE) -> *mut FILE {
    let mut previous_stream: *mut FILE = 0 as *mut FILE;
    if gsl_stream.is_null() {
        gsl_stream = stderr;
    }
    previous_stream = gsl_stream;
    gsl_stream = new_stream;
    return previous_stream;
}
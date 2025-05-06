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
    fn abort() -> !;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn gsl_stream_printf(
        label: *const i8,
        file: *const i8,
        line: i32,
        reason: *const i8,
    );
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
pub type gsl_error_handler_t = unsafe extern "C" fn(
    *const i8,
    *const i8,
    i32,
    i32,
) -> ();
#[no_mangle]
pub static mut gsl_error_handler: Option<gsl_error_handler_t> = None;
#[no_mangle]
pub unsafe extern "C" fn gsl_error(
    mut reason: *const i8,
    mut file: *const i8,
    mut line: i32,
    mut gsl_errno: i32,
) {
    if gsl_error_handler.is_some() {
        (Some(gsl_error_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")(reason, file, line, gsl_errno);
        return;
    }
    gsl_stream_printf(b"ERROR\0" as *const u8 as *const i8, file, line, reason);
    fflush(stdout);
    fprintf(stderr, b"Default GSL error handler invoked.\n\0" as *const u8 as *const i8);
    fflush(stderr);
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_set_error_handler(
    mut new_handler: Option<gsl_error_handler_t>,
) -> Option<gsl_error_handler_t> {
    let mut previous_handler: Option<gsl_error_handler_t> = gsl_error_handler;
    gsl_error_handler = new_handler;
    return previous_handler;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_set_error_handler_off() -> Option<gsl_error_handler_t> {
    let mut previous_handler: Option<gsl_error_handler_t> = gsl_error_handler;
    gsl_error_handler = Some(
        no_error_handler as unsafe extern "C" fn(*const i8, *const i8, i32, i32) -> (),
    );
    return previous_handler;
}
unsafe extern "C" fn no_error_handler(
    mut reason: *const i8,
    mut file: *const i8,
    mut line: i32,
    mut gsl_errno: i32,
) {
    reason = 0 as *const i8;
    file = 0 as *const i8;
    line = 0 as i32;
    gsl_errno = 0 as i32;
}
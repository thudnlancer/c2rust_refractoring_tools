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
    fn pclose(__stream: *mut FILE) -> i32;
    fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
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
#[no_mangle]
pub unsafe extern "C" fn printer_open(
    mut cmd: *mut i8,
    mut options: *mut i8,
    mut queue_param: *mut i8,
    mut printer_name: *mut i8,
    mut context_return: *mut *mut libc::c_void,
) -> *mut FILE {
    let mut pipe_cmd: [i8; 1024] = [0; 1024];
    let mut fp: *mut FILE = 0 as *mut FILE;
    sprintf(
        pipe_cmd.as_mut_ptr(),
        b"%s %s %s%s\0" as *const u8 as *const i8,
        cmd,
        if !options.is_null() { options } else { b"\0" as *const u8 as *const i8 },
        if !printer_name.is_null() {
            queue_param
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !printer_name.is_null() {
            printer_name
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    fp = popen(pipe_cmd.as_mut_ptr(), b"w\0" as *const u8 as *const i8);
    *context_return = fp as *mut libc::c_void;
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn printer_close(mut context: *mut libc::c_void) {
    let mut fp: *mut FILE = context as *mut FILE;
    pclose(fp);
}
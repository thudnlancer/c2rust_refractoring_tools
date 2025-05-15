use ::libc;
extern "C" {
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn printer_open(
    mut cmd: *mut libc::c_char,
    mut options: *mut libc::c_char,
    mut queue_param: *mut libc::c_char,
    mut printer_name: *mut libc::c_char,
    mut context_return: *mut *mut libc::c_void,
) -> *mut FILE {
    let mut pipe_cmd: [libc::c_char; 1024] = [0; 1024];
    let mut fp: *mut FILE = 0 as *mut FILE;
    sprintf(
        pipe_cmd.as_mut_ptr(),
        b"%s %s %s%s\0" as *const u8 as *const libc::c_char,
        cmd,
        if !options.is_null() {
            options
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !printer_name.is_null() {
            queue_param
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !printer_name.is_null() {
            printer_name
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    fp = popen(pipe_cmd.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    *context_return = fp as *mut libc::c_void;
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn printer_close(mut context: *mut libc::c_void) {
    let mut fp: *mut FILE = context as *mut FILE;
    pclose(fp);
}

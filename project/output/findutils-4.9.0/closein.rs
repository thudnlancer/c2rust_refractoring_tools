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
    fn __errno_location() -> *mut i32;
    static mut stdin: *mut _IO_FILE;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: i32) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn _exit(_: i32) -> !;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn close_stream(stream: *mut FILE) -> i32;
    fn close_stdout();
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    static mut exit_failure: i32;
    fn freadahead(stream: *mut FILE) -> size_t;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
}
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
pub type size_t = u64;
pub type __off64_t = i64;
pub type _IO_lock_t = ();
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
static mut file_name: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn close_stdin_set_file_name(mut file: *const i8) {
    file_name = file;
}
#[no_mangle]
pub unsafe extern "C" fn close_stdin() {
    let mut fail: bool = 0 as i32 != 0;
    if freadahead(stdin) > 0 as i32 as u64 {
        if rpl_fseeko(stdin, 0 as i32 as off_t, 1 as i32) == 0 as i32
            && rpl_fflush(stdin) != 0 as i32
        {
            fail = 1 as i32 != 0;
        }
    }
    if close_stream(stdin) != 0 as i32 {
        fail = 1 as i32 != 0;
    }
    if fail {
        let mut close_error: *const i8 = dcgettext(
            0 as *const i8,
            b"error closing file\0" as *const u8 as *const i8,
            5 as i32,
        );
        if !file_name.is_null() {
            error(
                0 as i32,
                *__errno_location(),
                b"%s: %s\0" as *const u8 as *const i8,
                quotearg_colon(file_name),
                close_error,
            );
        } else {
            error(
                0 as i32,
                *__errno_location(),
                b"%s\0" as *const u8 as *const i8,
                close_error,
            );
        }
    }
    close_stdout();
    if fail {
        _exit(exit_failure);
    }
}
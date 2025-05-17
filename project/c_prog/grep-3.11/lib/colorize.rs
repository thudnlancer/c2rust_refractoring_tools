use ::libc;
extern "C" {
    static mut stdout: *mut _IO_FILE;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn print_end_colorize(mut sgr_end: *const libc::c_char) {
    fputs(sgr_end, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn print_start_colorize(
    mut sgr_start: *const libc::c_char,
    mut sgr_seq: *const libc::c_char,
) {
    printf(sgr_start, sgr_seq);
}
#[no_mangle]
pub unsafe extern "C" fn init_colorize() {}
#[no_mangle]
pub unsafe extern "C" fn should_colorize() -> libc::c_int {
    let mut t: *const libc::c_char = getenv(
        b"TERM\0" as *const u8 as *const libc::c_char,
    );
    return (!t.is_null()
        && strcmp(t, b"dumb\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int)
        as libc::c_int;
}

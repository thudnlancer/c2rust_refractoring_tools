#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn zerror(_: *mut *const libc::c_char) -> zerror;
    static mut libzahl_error: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_INVALID_RADIX = 5,
    ZERROR_NEGATIVE = 4,
    ZERROR_DIV_0 = 3,
    ZERROR_0_DIV_0 = 2,
    ZERROR_0_POW_0 = 1,
    ZERROR_ERRNO_SET = 0,
}  // end of enum

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
pub unsafe extern "C" fn zperror(mut prefix: *const libc::c_char) {
    if libzahl_error >= 0 as libc::c_int {
        *__errno_location() = libzahl_error;
        perror(prefix);
    } else {
        let mut desc: *const libc::c_char = 0 as *const libc::c_char;
        zerror(&mut desc);
        if !prefix.is_null() && *prefix as libc::c_int != 0 {
            fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                prefix,
                desc,
            );
        } else {
            fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, desc);
        }
    };
}

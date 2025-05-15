use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn nettle_realloc(
    mut ctx: *mut libc::c_void,
    mut p: *mut libc::c_void,
    mut length: size_t,
) -> *mut libc::c_void {
    if length > 0 as libc::c_int as libc::c_ulong {
        return realloc(p, length);
    }
    free(p);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_xrealloc(
    mut ctx: *mut libc::c_void,
    mut p: *mut libc::c_void,
    mut length: size_t,
) -> *mut libc::c_void {
    if length > 0 as libc::c_int as libc::c_ulong {
        let mut n: *mut libc::c_void = realloc(p, length);
        if n.is_null() {
            fprintf(
                stderr,
                b"Virtual memory exhausted.\n\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
        return n;
    }
    free(p);
    return 0 as *mut libc::c_void;
}

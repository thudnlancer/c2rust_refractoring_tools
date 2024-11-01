#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type quoting_options;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn quotearg_buffer(
        buffer: *mut libc::c_char,
        buffersize: size_t,
        arg: *const libc::c_char,
        argsize: size_t,
        o: *const quoting_options,
    ) -> size_t;
    fn qmark_chars(buf: *mut libc::c_char, len: size_t) -> size_t;
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
pub unsafe extern "C" fn print_quoted(
    mut fp: *mut FILE,
    mut qopts: *const quoting_options,
    mut dest_is_tty: bool,
    mut format: *const libc::c_char,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if dest_is_tty {
        let mut smallbuf: [libc::c_char; 8192] = [0; 8192];
        let mut len: size_t = quotearg_buffer(
            smallbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            s,
            -(1 as libc::c_int) as size_t,
            qopts,
        );
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        if len < ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong {
            buf = smallbuf.as_mut_ptr();
        } else {
            buf = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            quotearg_buffer(
                buf,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                s,
                -(1 as libc::c_int) as size_t,
                qopts,
            );
        }
        len = qmark_chars(buf, len);
        *buf.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        rv = fprintf(fp, format, buf);
        if buf != smallbuf.as_mut_ptr() {
            rpl_free(buf as *mut libc::c_void);
            buf = 0 as *mut libc::c_char;
        }
    } else {
        rv = fprintf(fp, format, s);
    }
    return rv;
}

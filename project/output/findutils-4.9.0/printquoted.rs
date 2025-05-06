#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type quoting_options;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn quotearg_buffer(
        buffer: *mut i8,
        buffersize: size_t,
        arg: *const i8,
        argsize: size_t,
        o: *const quoting_options,
    ) -> size_t;
    fn qmark_chars(buf: *mut i8, len: size_t) -> size_t;
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
pub unsafe extern "C" fn print_quoted(
    mut fp: *mut FILE,
    mut qopts: *const quoting_options,
    mut dest_is_tty: bool,
    mut format: *const i8,
    mut s: *const i8,
) -> i32 {
    let mut rv: i32 = 0;
    if dest_is_tty {
        let mut smallbuf: [i8; 8192] = [0; 8192];
        let mut len: size_t = quotearg_buffer(
            smallbuf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 8192]>() as u64,
            s,
            -(1 as i32) as size_t,
            qopts,
        );
        let mut buf: *mut i8 = 0 as *mut i8;
        if len < ::core::mem::size_of::<[i8; 8192]>() as u64 {
            buf = smallbuf.as_mut_ptr();
        } else {
            buf = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
            quotearg_buffer(
                buf,
                len.wrapping_add(1 as i32 as u64),
                s,
                -(1 as i32) as size_t,
                qopts,
            );
        }
        len = qmark_chars(buf, len);
        *buf.offset(len as isize) = 0 as i32 as i8;
        rv = fprintf(fp, format, buf);
        if buf != smallbuf.as_mut_ptr() {
            rpl_free(buf as *mut libc::c_void);
            buf = 0 as *mut i8;
        }
    } else {
        rv = fprintf(fp, format, s);
    }
    return rv;
}